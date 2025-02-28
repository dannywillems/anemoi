//! Sponge trait implementation for Anemoi

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use super::digest::AnemoiDigest;
use super::{apply_permutation, DIGEST_SIZE, NUM_COLUMNS, RATE_WIDTH, STATE_WIDTH};
use super::{Jive, Sponge};

use super::Felt;
use super::{One, Zero};

use ark_ff::FromBytes;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// An Anemoi hash instantiation
pub struct AnemoiHash {
    state: [Felt; STATE_WIDTH],
    idx: usize,
}

impl Default for AnemoiHash {
    fn default() -> Self {
        Self {
            state: [Felt::zero(); STATE_WIDTH],
            idx: 0,
        }
    }
}

impl Sponge<Felt> for AnemoiHash {
    type Digest = AnemoiDigest;

    fn hash(bytes: &[u8]) -> Self::Digest {
        // Compute the number of field elements required to represent this
        // sequence of bytes.
        let num_elements = if bytes.len() % 31 == 0 {
            bytes.len() / 31
        } else {
            bytes.len() / 31 + 1
        };

        let sigma = if num_elements % RATE_WIDTH == 0 {
            Felt::one()
        } else {
            Felt::zero()
        };

        // Initialize the internal hash state to all zeroes.
        let mut state = [Felt::zero(); STATE_WIDTH];

        // Absorption phase

        // Break the string into 31-byte chunks, then convert each chunk into a field element,
        // and absorb the element into the rate portion of the state. The conversion is
        // guaranteed to succeed as we spare one last byte to ensure this can represent a valid
        // element encoding.
        let mut i = 0;
        let mut num_hashed = 0;
        let mut buf = [0u8; 32];
        for chunk in bytes.chunks(31) {
            if num_hashed + i < num_elements - 1 {
                buf[..31].copy_from_slice(chunk);
            } else {
                // The last chunk may be smaller than the others, which requires a special handling.
                // In this case, we also append a byte set to 1 to the end of the string, padding the
                // sequence in a way that adding additional trailing zeros will yield a different hash.
                let chunk_len = chunk.len();
                buf = [0u8; 32];
                buf[..chunk_len].copy_from_slice(chunk);
                // [Different to paper]: We pad the last chunk with 1 to prevent length extension attack.
                if chunk_len < 31 {
                    buf[chunk_len] = 1;
                }
            }

            // Convert the bytes into a field element and absorb it into the rate portion of the
            // state. An Anemoi permutation is applied to the internal state if all the the rate
            // registers have been filled with additional values. We then reset the insertion index.
            state[i] += Felt::read(&buf[..]).unwrap();
            i += 1;
            if i % RATE_WIDTH == 0 {
                apply_permutation(&mut state);
                i = 0;
                num_hashed += RATE_WIDTH;
            }
        }

        // We then add sigma to the last register of the capacity.
        state[STATE_WIDTH - 1] += sigma;

        // If the message length is not a multiple of RATE_WIDTH, we append 1 to the rate cell
        // next to the one where we previously appended the last message element. This is
        // guaranted to be in the rate registers (i.e. to not require an extra permutation before
        // adding this constant) if sigma is equal to zero. We then apply a final Anemoi permutation
        // to the whole state.
        if sigma.is_zero() {
            state[i] += Felt::one();
            apply_permutation(&mut state);
        }

        // Squeezing phase

        // Finally, return the first DIGEST_SIZE elements of the state.
        Self::Digest::new(state[..DIGEST_SIZE].try_into().unwrap())
    }

    fn hash_field(elems: &[Felt]) -> Self::Digest {
        // initialize state to all zeros
        let mut state = [Felt::zero(); STATE_WIDTH];

        let sigma = if elems.len() % RATE_WIDTH == 0 {
            Felt::one()
        } else {
            Felt::zero()
        };

        let mut i = 0;
        for &element in elems.iter() {
            state[i] += element;
            i += 1;
            if i % RATE_WIDTH == 0 {
                apply_permutation(&mut state);
                i = 0;
            }
        }

        // We then add sigma to the last register of the capacity.
        state[STATE_WIDTH - 1] += sigma;

        // If the message length is not a multiple of RATE_WIDTH, we append 1 to the rate cell
        // next to the one where we previously appended the last message element. This is
        // guaranted to be in the rate registers (i.e. to not require an extra permutation before
        // adding this constant) if sigma is equal to zero. We then apply a final Anemoi permutation
        // to the whole state.
        if sigma.is_zero() {
            state[i] += Felt::one();
            apply_permutation(&mut state);
        }

        // Squeezing phase

        Self::Digest::new(state[..DIGEST_SIZE].try_into().unwrap())
    }

    fn merge(digests: &[Self::Digest; 2]) -> Self::Digest {
        // initialize state to all zeros
        let mut state = [Felt::zero(); STATE_WIDTH];

        // 2*DIGEST_SIZE < RATE_SIZE so we can safely store
        // the digests into the rate registers at once
        state[0..DIGEST_SIZE].copy_from_slice(digests[0].as_elements());
        state[DIGEST_SIZE..2 * DIGEST_SIZE].copy_from_slice(digests[0].as_elements());

        // Apply internal Anemoi permutation
        apply_permutation(&mut state);

        Self::Digest::new(state[..DIGEST_SIZE].try_into().unwrap())
    }
}

impl Jive<Felt> for AnemoiHash {
    fn compress(elems: &[Felt]) -> Vec<Felt> {
        assert!(elems.len() == STATE_WIDTH);

        let mut state = elems.try_into().unwrap();
        apply_permutation(&mut state);

        let mut result = [Felt::zero(); NUM_COLUMNS];
        for (i, r) in result.iter_mut().enumerate() {
            *r = elems[i] + elems[i + NUM_COLUMNS] + state[i] + state[i + NUM_COLUMNS];
        }

        result.to_vec()
    }

    fn compress_k(elems: &[Felt], k: usize) -> Vec<Felt> {
        assert!(elems.len() == STATE_WIDTH);
        assert!(STATE_WIDTH % k == 0);
        // We can output as few as 1 element while
        // maintaining the targeted security level.
        assert!(k <= STATE_WIDTH);

        let mut state = elems.try_into().unwrap();
        apply_permutation(&mut state);

        let mut result = vec![Felt::zero(); STATE_WIDTH / k];
        let c = result.len();
        for (i, r) in result.iter_mut().enumerate() {
            for j in 0..k {
                *r += elems[i + c * j] + state[i + c * j];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::BigInteger256;
    use super::*;

    #[test]
    fn test_anemoi_hash() {
        // Generated from https://github.com/Nashtare/anemoi-hash/
        let input_data = [
            vec![Felt::zero(); 12],
            vec![Felt::one(); 12],
            vec![
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
            ],
            vec![
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
            ],
            vec![Felt::new(BigInteger256([
                0xdcf5b540d9547f97,
                0x57b9228f4a8d52b4,
                0x4870bbe22508dcb0,
                0x0881e2ddb1d83e6a,
            ]))],
            vec![
                Felt::new(BigInteger256([
                    0x5c19f7a984232e9a,
                    0x501e3dd97d23f2a9,
                    0xeb5d3fcc23693644,
                    0x133b449057000688,
                ])),
                Felt::new(BigInteger256([
                    0x8ce7079ad1b353b8,
                    0xd93b215c26a55ae3,
                    0x2692fe0df132959f,
                    0x26a6ee81a54054fb,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0xb89790a30133f218,
                    0x510f8f8454d18f42,
                    0x3f2ec497c5137d4c,
                    0x2feb6cbd3f3c4514,
                ])),
                Felt::new(BigInteger256([
                    0xe92373ca97678ce8,
                    0xddeb39befb047ff3,
                    0x4d52660ce000ac12,
                    0x105101d2ca80482d,
                ])),
                Felt::new(BigInteger256([
                    0x731ad72bdc250de4,
                    0xf96bb8416ba39718,
                    0xb6e1bc5e3cc38abc,
                    0x0c4b2aeb9838b119,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0x6a087c50c18b8634,
                    0x8186b6201a4cd193,
                    0xa6a3e27db80edae1,
                    0x11abebb0cf8f98db,
                ])),
                Felt::new(BigInteger256([
                    0xdf7729b6b3797fb5,
                    0x1e96ccfdff3cc319,
                    0x7079b41350f04216,
                    0x2c246ef812d77a71,
                ])),
                Felt::new(BigInteger256([
                    0x21b691a504e974ee,
                    0x7955b1daffa9fbf4,
                    0xff41f79e5a612d03,
                    0x1923a59060315e41,
                ])),
                Felt::new(BigInteger256([
                    0x3d86f953f9724a2e,
                    0x71cec81bf25692c8,
                    0x7a5f6813f7e9d304,
                    0x0362d76ff5654917,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0xb33cf8f62117c2a3,
                    0xbee4db5c0257e332,
                    0x487c0be4ad7aae47,
                    0x2d96e5eee82bc520,
                ])),
                Felt::new(BigInteger256([
                    0x90b727b900615ea0,
                    0xae8ad94a1682d27d,
                    0x5da7941372771e4d,
                    0x21de9745b309eb91,
                ])),
                Felt::new(BigInteger256([
                    0xca162fa24a98660f,
                    0xa101aebe8ef1b60d,
                    0x2f348c1603481142,
                    0x16c7979ef36e92e1,
                ])),
                Felt::new(BigInteger256([
                    0x282226e5b6a2c687,
                    0xce7d7dc4e5273ef7,
                    0x25e75bb378b067cd,
                    0x2cddd03f42426549,
                ])),
                Felt::new(BigInteger256([
                    0x3c546b0a6b587fb7,
                    0x48722c8f79e28da2,
                    0x0b7826379d576a5b,
                    0x277b3524c03cde13,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0x2fd90b372d27f092,
                    0xefa18851bbbcff4b,
                    0xe99fb4480524ffc5,
                    0x16a23443415febdd,
                ])),
                Felt::new(BigInteger256([
                    0xb2a79518a201d293,
                    0xe25b067a23328972,
                    0x673ee2e2fe72cc80,
                    0x29a5de0ac8a20239,
                ])),
                Felt::new(BigInteger256([
                    0x97c174fe238b3409,
                    0xa8e57a207382c6d5,
                    0x52f3a2505c1af67c,
                    0x0c2a699126de4b77,
                ])),
                Felt::new(BigInteger256([
                    0xebd6eb80e8ccccd6,
                    0x54c35fcb24c53e02,
                    0x838bb437a427b930,
                    0x0b9566af2c18d4b4,
                ])),
                Felt::new(BigInteger256([
                    0x15eb69d338e768bf,
                    0x7e4143e79e545f4e,
                    0x23028ac4c926bece,
                    0x01f9edb385a08edc,
                ])),
                Felt::new(BigInteger256([
                    0x8e28254c8d5ee716,
                    0x77d23c14c939aca0,
                    0xe1113ec0611450e3,
                    0x16c35125640e4f32,
                ])),
            ],
        ];

        let output_data = [
            [Felt::new(BigInteger256([
                0x8bc26946024011fe,
                0x5a0a30efbc359b88,
                0x85a5fb7aff568714,
                0x09b01d4bd2b15ece,
            ]))],
            [Felt::new(BigInteger256([
                0xab1d21daa0ac4b46,
                0x1d0fec995ffd32b1,
                0x6e76b04acf7eecfa,
                0x0ab12e71d5cb7a92,
            ]))],
            [Felt::new(BigInteger256([
                0xcf34acc2b72b05f0,
                0xe86750388bee1a70,
                0x7d32e81bd277e6f5,
                0x1053728a6c199f36,
            ]))],
            [Felt::new(BigInteger256([
                0xc44b973ef22c5377,
                0x77a55eabe99c37e4,
                0x3281b845164b4a30,
                0x2d91ee5ea68d57b5,
            ]))],
            [Felt::new(BigInteger256([
                0x66781e4d2da4a0cb,
                0x1626eca410a30293,
                0x5651caa9801fd81e,
                0x0c3abb1d12e69f6a,
            ]))],
            [Felt::new(BigInteger256([
                0xc88e906275887199,
                0x34e2ae7618ccfe05,
                0xe107cd8f4ba65576,
                0x2dd585be5c6f3e5f,
            ]))],
            [Felt::new(BigInteger256([
                0xa688612c2bbf4533,
                0xf43bfc50dba8c312,
                0xd52a66fbbd7ea89f,
                0x026cf0c1020e6963,
            ]))],
            [Felt::new(BigInteger256([
                0x5c88079f42f52faf,
                0x9b9a6640cc07f4b4,
                0x9c29e68b44d70414,
                0x0f364269a9e7ce77,
            ]))],
            [Felt::new(BigInteger256([
                0x0b2499bd209a7584,
                0x70166829c749669a,
                0x3590b923d0c714e3,
                0x12453b97b19f4899,
            ]))],
            [Felt::new(BigInteger256([
                0x79d8da420689daad,
                0xad156fc0c8c72c8c,
                0x4a11ed23a63ac7f2,
                0x2b4b418838855342,
            ]))],
        ];

        for (input, expected) in input_data.iter().zip(output_data) {
            assert_eq!(expected, AnemoiHash::hash_field(input).to_elements());
        }
    }

    #[test]
    fn test_anemoi_jive() {
        // Generated from https://github.com/Nashtare/anemoi-hash/
        let input_data = [
            vec![Felt::zero(); 12],
            vec![Felt::one(); 12],
            vec![
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
            ],
            vec![
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
            ],
            vec![
                Felt::new(BigInteger256([
                    0xd9d2bd097334eb0e,
                    0x3f29f3a31c23ac77,
                    0x8672d707993bf691,
                    0x15e23d5fc0ebc7ca,
                ])),
                Felt::new(BigInteger256([
                    0x814a26b5fc8a7f97,
                    0x4f6537d6dc85831b,
                    0x9be1c627b10ac241,
                    0x02465eb7e3d354ed,
                ])),
                Felt::new(BigInteger256([
                    0xccd205ee52775941,
                    0x9b2a8025e5cb44d7,
                    0xb0630340021d6bcb,
                    0x162781a0e4d4d7df,
                ])),
                Felt::new(BigInteger256([
                    0x4a0523b7eb7bb201,
                    0x68e1e375788e31dd,
                    0x6b9ee3307614e4f5,
                    0x2f06242115b6471c,
                ])),
                Felt::new(BigInteger256([
                    0x6853c380c8240335,
                    0x6c93ae81e597ce76,
                    0x8c51ac44c65791ff,
                    0x12f7165aec4070d6,
                ])),
                Felt::new(BigInteger256([
                    0x8ddd400bd1c3c4b4,
                    0xf043113143358bcb,
                    0xba0fc6988d853c1b,
                    0x1b30227d26ab3687,
                ])),
                Felt::new(BigInteger256([
                    0x39069c6f55694794,
                    0x961d9a8483bb519c,
                    0xe029891f8f2a4412,
                    0x13d7675395d9d3fa,
                ])),
                Felt::new(BigInteger256([
                    0xfd5dbd32bc05e8ed,
                    0x93de78fc02bb8da1,
                    0xdd7500517fe95106,
                    0x19ac3b4538983e61,
                ])),
                Felt::new(BigInteger256([
                    0x29d7ac41d1e977db,
                    0x896ed57accda4130,
                    0xb44b8c36ebc0be55,
                    0x27df2aa90412b567,
                ])),
                Felt::new(BigInteger256([
                    0x3c08577d3d084f92,
                    0x8c7f53c99626ce6b,
                    0x1fba8c17c928f34c,
                    0x096d37404d5c6429,
                ])),
                Felt::new(BigInteger256([
                    0x5dfefcf4a3b51d56,
                    0xd960e63124e7607d,
                    0xa4616608ee1887fd,
                    0x2e8aa20abc0218c6,
                ])),
                Felt::new(BigInteger256([
                    0x173934b7263d7776,
                    0x0f8ba20fcd6fe641,
                    0xc7844f86475bed8b,
                    0x0f6cc368c54dbe0a,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0xf165818a92450dad,
                    0x6e8b04ecdd158aa8,
                    0xf0aeb802b898b07b,
                    0x1429c061d2a2a461,
                ])),
                Felt::new(BigInteger256([
                    0xc65272ea70a3baaa,
                    0x3cd6d156ceeea925,
                    0xe4d463716e69b243,
                    0x131c6ed1fb788385,
                ])),
                Felt::new(BigInteger256([
                    0x5b4331cd93d3a023,
                    0x5aaf6561eb7e16f6,
                    0xa6f3aa490d345754,
                    0x151d992573704e17,
                ])),
                Felt::new(BigInteger256([
                    0x8765bc0a032ed6cf,
                    0xc0816553dd09127d,
                    0x4e769e06796c5eef,
                    0x18c0c702a105f0a8,
                ])),
                Felt::new(BigInteger256([
                    0x743e6f2349f46ca8,
                    0xb1ad3fec05a7bbf9,
                    0x858a79b9c99e7271,
                    0x0c701811404b9769,
                ])),
                Felt::new(BigInteger256([
                    0x6c4640d64ab1eddc,
                    0x7707666930b3643a,
                    0x855555f53655f84d,
                    0x093c69220656cb21,
                ])),
                Felt::new(BigInteger256([
                    0x7148c86d4cdd4a4e,
                    0x8526652f198ea296,
                    0xe1dfb2ad035c06e2,
                    0x163a5ae1e1536217,
                ])),
                Felt::new(BigInteger256([
                    0xe7b706c45894fbad,
                    0x979665c2c5c99c65,
                    0xa0d55e16dd65ebd9,
                    0x21c67abef881b331,
                ])),
                Felt::new(BigInteger256([
                    0xf22c7bdca4335706,
                    0x5623189caa6be6c9,
                    0x59e332c4a618f8a5,
                    0x265c0b62a9e16211,
                ])),
                Felt::new(BigInteger256([
                    0xf3e70ab8c524441e,
                    0x6f0d4e69d1ce04e5,
                    0x7d996f0370729d0c,
                    0x0fe8319cf1257dc1,
                ])),
                Felt::new(BigInteger256([
                    0x826bde1c1adb34b9,
                    0xc94722388c7c8692,
                    0xe4474877ae47e2fb,
                    0x16b5f89b3f13f7a3,
                ])),
                Felt::new(BigInteger256([
                    0xc2ea341d9482682c,
                    0x4e4fc681af3c1907,
                    0xf8ca6a5fdfa26be3,
                    0x0cf88815c6871e8d,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0x6480ed726f59beae,
                    0x589fb4a37f6e974a,
                    0xc9b4978bdcc43fe2,
                    0x288d7bb89279f8f2,
                ])),
                Felt::new(BigInteger256([
                    0x6536928017569d2b,
                    0x52e4b79842014553,
                    0xd87da3b47fb2263c,
                    0x173cb1df28c31fc5,
                ])),
                Felt::new(BigInteger256([
                    0x952d0cbae3654e82,
                    0x61298d8b0ec7973f,
                    0x78eae1ebc7df0d08,
                    0x2a850fcb745ff282,
                ])),
                Felt::new(BigInteger256([
                    0x5a28ae2c047a651f,
                    0xb675d8f9b1c0a864,
                    0xa48f37461dfb9873,
                    0x28a89babedc213e5,
                ])),
                Felt::new(BigInteger256([
                    0x1471e7cb415dca90,
                    0x0f71228649da1728,
                    0x7efdef1161b3d874,
                    0x276dc613aec42cb4,
                ])),
                Felt::new(BigInteger256([
                    0x0111dad1192d6046,
                    0x5cd16fd0badaaaa1,
                    0xc169487b47d499ae,
                    0x16fe34f314bd5451,
                ])),
                Felt::new(BigInteger256([
                    0x8f40b8011c2289eb,
                    0xfffe26eafe43a88d,
                    0x6c0af1359b8dda57,
                    0x1d7c6e7a2f56386c,
                ])),
                Felt::new(BigInteger256([
                    0x6e9c5e9b20a18ccf,
                    0x28268e8eb8b50789,
                    0xc0a45433547e04a3,
                    0x299db3b8aacebed8,
                ])),
                Felt::new(BigInteger256([
                    0x5bf80d1a78be3e19,
                    0x0524c239abd9ae9d,
                    0x85bd08f59a669e10,
                    0x19600196064a6d3c,
                ])),
                Felt::new(BigInteger256([
                    0x65e2f4ea600fda5d,
                    0x9b08366fc165420c,
                    0x85ac013007ecd996,
                    0x0f7b8bcfa9ad7cf4,
                ])),
                Felt::new(BigInteger256([
                    0x8a3c6c8dcd92fb96,
                    0x1893cc2e82472727,
                    0x832923caca993665,
                    0x15c17a04f44258c1,
                ])),
                Felt::new(BigInteger256([
                    0xaeadf13786cd20b0,
                    0xf0ebbc4f388e5139,
                    0x492ba1a815db6e1e,
                    0x244485a8aa6157b4,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0xdaa8ed5741c17c07,
                    0xe0b4705492d78ee1,
                    0x233ec1e4b2d1055b,
                    0x22052f791f0f9b01,
                ])),
                Felt::new(BigInteger256([
                    0xbebf0a7195182862,
                    0x5b1f674c89f07d15,
                    0xee1c910c4967e106,
                    0x08119e3da219a969,
                ])),
                Felt::new(BigInteger256([
                    0xbde497046b1f2b56,
                    0x2cbc06629f3fa796,
                    0xe6d2243d1b44cb06,
                    0x1973a2a9d52e2b7f,
                ])),
                Felt::new(BigInteger256([
                    0x6bdd0c809a9fd358,
                    0xf849ba0391ee20ab,
                    0x6792e60210df25e5,
                    0x0f3d76179a79b0fc,
                ])),
                Felt::new(BigInteger256([
                    0xcc21e3b3b9e2c73d,
                    0xb5fe8d0c1dfed97f,
                    0x1b4d0925e8eaf27c,
                    0x0663ca17b9efbc4a,
                ])),
                Felt::new(BigInteger256([
                    0x6dd4f847077cbe1c,
                    0xdbf8b1baadda1107,
                    0xf204f5c208c3df00,
                    0x2da42ce498a29c05,
                ])),
                Felt::new(BigInteger256([
                    0x6c82d0e74786cb3a,
                    0x7ef1f0f3abad2856,
                    0x16f6080d6d26c1c3,
                    0x01da54c0c08fa80b,
                ])),
                Felt::new(BigInteger256([
                    0xe308bd4f21f359c3,
                    0x6813aa75f06b632d,
                    0xfcf2b99d7b133d1d,
                    0x0e3b441c5cd55572,
                ])),
                Felt::new(BigInteger256([
                    0x02b53ee5be7429f9,
                    0xb3ed5721de132a3e,
                    0x29d9838d52532d7d,
                    0x2c9b818540a24fc7,
                ])),
                Felt::new(BigInteger256([
                    0x0f5e3e2864786cd0,
                    0xbbe562dfc2d4bfea,
                    0x5546996c90ea80d7,
                    0x1943d30f47c92447,
                ])),
                Felt::new(BigInteger256([
                    0x60d10d304ce43c2f,
                    0x931819f2083213f8,
                    0x2bea06d74068aa6d,
                    0x2fdfeb5bbae49f8b,
                ])),
                Felt::new(BigInteger256([
                    0x8702a16f53479b29,
                    0xc6e6dac5ed3c9502,
                    0x5b6cf8f8587c3ff0,
                    0x17abb095abaf770d,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0x69bdc7ee313e9e33,
                    0xee88f2588837f3ba,
                    0x7455636d804f546e,
                    0x2078b917c560ddd3,
                ])),
                Felt::new(BigInteger256([
                    0x60088686d71d1802,
                    0x979bb9f0d0203fab,
                    0x7e4a64c8574d3c2f,
                    0x004c7ad2350ce33e,
                ])),
                Felt::new(BigInteger256([
                    0x3f7ad07af1500178,
                    0x731c49d2e783e567,
                    0xcc436ff1fbe9da2d,
                    0x04dc965c2cfe38e0,
                ])),
                Felt::new(BigInteger256([
                    0xc0bf18e4e5ae678c,
                    0xe67736d73f55aa57,
                    0x7217c3dcebb628b8,
                    0x1e086e5c6c98f391,
                ])),
                Felt::new(BigInteger256([
                    0xfbb0781da16b56ca,
                    0xa2ac59628a978e58,
                    0x1298d4f2b363095f,
                    0x2f3e588fb1039f70,
                ])),
                Felt::new(BigInteger256([
                    0x4a2e93dc887d6b9d,
                    0xecedbf6e0599f6c3,
                    0xb9db8794758f8da6,
                    0x0fa7dcbf837a8461,
                ])),
                Felt::new(BigInteger256([
                    0x338b9d6e9f8f6264,
                    0xd7d72b1051bfde13,
                    0x978389b2cbc13b91,
                    0x0fe200a57e16e076,
                ])),
                Felt::new(BigInteger256([
                    0x0d4b2425d1587ce9,
                    0x183d215d99544dac,
                    0x5bbcf356f6c82825,
                    0x16488e8d0bb02e17,
                ])),
                Felt::new(BigInteger256([
                    0x0d92d4c21edf982f,
                    0xcab3b95af95d80aa,
                    0xdcbf77c7609ffa64,
                    0x2a06e0f20db17ce0,
                ])),
                Felt::new(BigInteger256([
                    0xf10c9b8daacd506f,
                    0x0017e1af3cc150eb,
                    0x46483644c2cfe605,
                    0x23c8b690da8c09d3,
                ])),
                Felt::new(BigInteger256([
                    0xa2e3dab3780146a5,
                    0x17ff63c30d628034,
                    0xcc2d540432d7ca98,
                    0x01b8405878a6c4aa,
                ])),
                Felt::new(BigInteger256([
                    0x1f2a1d7c45f72f07,
                    0x0588edaa62d5d3cb,
                    0x74a670de03da0529,
                    0x19e96f49757b41ea,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0xc40ceb8fd0a1916c,
                    0xee9335d8deaad7f7,
                    0x70facb20ca467c2c,
                    0x09a41ca22c02f129,
                ])),
                Felt::new(BigInteger256([
                    0x9f6c6b6c3fbcc1f5,
                    0x9c1086f6537ebc03,
                    0xce240a1dd8bd06e7,
                    0x1916f01947ead645,
                ])),
                Felt::new(BigInteger256([
                    0x874d779f86d3d234,
                    0x3b6e89dd90b150d2,
                    0x9f7cf32c3df6509f,
                    0x1c70e1c206955414,
                ])),
                Felt::new(BigInteger256([
                    0xf92e2659d0e52d87,
                    0x98d7cd9fec172725,
                    0x5611a71f685970b2,
                    0x200f5f7b84089930,
                ])),
                Felt::new(BigInteger256([
                    0x6a1f7b467f4e0fbb,
                    0xc859717839622e56,
                    0x076ef9cfdab58109,
                    0x2bde9c727c312422,
                ])),
                Felt::new(BigInteger256([
                    0xe400388d0220644b,
                    0x89a5b4b4ac12647d,
                    0x7314639460749cde,
                    0x19dbb5e154f118cf,
                ])),
                Felt::new(BigInteger256([
                    0xadd219d3716ac627,
                    0x87d0139767efc7a1,
                    0x96c0ac3b55e0aa2a,
                    0x0df1fdcad0b835f4,
                ])),
                Felt::new(BigInteger256([
                    0x39a90bb233157e18,
                    0x0350215477e75e87,
                    0xdd39296b0670fd1d,
                    0x258255700c663e23,
                ])),
                Felt::new(BigInteger256([
                    0xbdd634f67263aa08,
                    0x2a71a362c31aa626,
                    0x5901b3b32e6c8c7f,
                    0x2dad3a5ce42fe9cd,
                ])),
                Felt::new(BigInteger256([
                    0xa1be756c85b0aaf2,
                    0x15436b5c303bca5a,
                    0xe8c3db135f4c0d29,
                    0x19a4266c3433a849,
                ])),
                Felt::new(BigInteger256([
                    0xdd053e96d1c9a599,
                    0xfbf072bbbf97ce03,
                    0xdc4b3774dab84d10,
                    0x18261b1249a29304,
                ])),
                Felt::new(BigInteger256([
                    0x1281ba6225ca30ba,
                    0xefb1f3bbc93950c7,
                    0x7260b9e7e8fb2084,
                    0x1118778e63bfee29,
                ])),
            ],
        ];

        let output_data = [
            [
                Felt::new(BigInteger256([
                    0xc86ad8928e449577,
                    0x6c0a4371c4b06721,
                    0x1bd4ee785105f189,
                    0x2f272cda10af5649,
                ])),
                Felt::new(BigInteger256([
                    0xa983e8e8cd2010f2,
                    0xc57d984c9e69c153,
                    0x8d5761bb9710b35f,
                    0x16dfb40bc0b3c9df,
                ])),
                Felt::new(BigInteger256([
                    0x3a1c9d47e865f027,
                    0x5236ff03c44ec26f,
                    0x2e3136ae75caba84,
                    0x1a11866cdd8037ca,
                ])),
                Felt::new(BigInteger256([
                    0x2a94d35de84159ba,
                    0xdd30c9c8f81a44ab,
                    0x62bba779992c77bf,
                    0x2be4242b545c3a53,
                ])),
                Felt::new(BigInteger256([
                    0x25335eaa5dd544d8,
                    0x3509fc57bb5eb616,
                    0xd581bdfad27fec40,
                    0x135d358396b7ded9,
                ])),
                Felt::new(BigInteger256([
                    0x2353bd9aed8016c1,
                    0x8bb0207fcfcb25ee,
                    0xc50f211faca51240,
                    0x295398723a3d285a,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0xada5eb1c1a52b67e,
                    0xb865b4361b488daa,
                    0x415ef520d849f63c,
                    0x2887474b416c0143,
                ])),
                Felt::new(BigInteger256([
                    0x96bb8f6ab8eaa86d,
                    0x5db6a734e546d0fc,
                    0x3a0714221534862c,
                    0x1f9ed2de64e67a21,
                ])),
                Felt::new(BigInteger256([
                    0xc5b5cc4ee40744a2,
                    0xba16b84ead87135d,
                    0x4dd6e7637f6b948c,
                    0x1909b4e0fafb69d6,
                ])),
                Felt::new(BigInteger256([
                    0x0faad487bfcbd331,
                    0x12cabb30587b1d64,
                    0xa88e1e31b79eb6a0,
                    0x045c99937ea06c06,
                ])),
                Felt::new(BigInteger256([
                    0x286f3e37ac666418,
                    0xb94a98a8933b7d6f,
                    0x465adb9e55f6a70a,
                    0x23ee032eec13bbce,
                ])),
                Felt::new(BigInteger256([
                    0xe88712ade0559419,
                    0x3729c60e280e4f09,
                    0xee31a6e51b82151e,
                    0x25fbac09d50acef9,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x381df8a26fe0c663,
                    0x8e98976de82a1c44,
                    0x389b04e98128cd84,
                    0x1f15ecb1ff641932,
                ])),
                Felt::new(BigInteger256([
                    0xabb4959a67bd654b,
                    0x2efa97282d9e3544,
                    0x65b90470e17e3f84,
                    0x2c831fe7ca23339b,
                ])),
                Felt::new(BigInteger256([
                    0x7c187865da3aeba2,
                    0x5aad37444a4aa417,
                    0x6306196b5a32ebb3,
                    0x0164437450ccc7af,
                ])),
                Felt::new(BigInteger256([
                    0xca95965be66fec69,
                    0x0576d6a4978a47fb,
                    0xb7431d63c27e618c,
                    0x0776523cbd068a9b,
                ])),
                Felt::new(BigInteger256([
                    0x4dfbbed6261964aa,
                    0xfff4874b69594833,
                    0x46cf848c36463b22,
                    0x2b52007ad33e78fa,
                ])),
                Felt::new(BigInteger256([
                    0xe5f068fa7c71bdd7,
                    0x2436536e0e4dede6,
                    0x116422d13a261890,
                    0x2275e50752da260e,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x6a743d222fff7d0b,
                    0xbd0837c63817a06f,
                    0x93f9216deec61685,
                    0x2e2116c7d77e946f,
                ])),
                Felt::new(BigInteger256([
                    0x03a479a9ec6293d0,
                    0x29a503bab544227c,
                    0x623afda97480c277,
                    0x0a58fd17abb881ab,
                ])),
                Felt::new(BigInteger256([
                    0x2ae1b4cebb254327,
                    0x0085af2550d60078,
                    0x0f8f04b4ef3e7c07,
                    0x0faef77351c9d5ec,
                ])),
                Felt::new(BigInteger256([
                    0x18edd668f9cb8b3a,
                    0x4dbaaefee0263d08,
                    0x3f1c7a396b54a2b9,
                    0x03e7d8b5a5e8431e,
                ])),
                Felt::new(BigInteger256([
                    0xa1249c9e0fd747fa,
                    0x8f9198fc45369ce6,
                    0x286da004feac7882,
                    0x117399aae1600d24,
                ])),
                Felt::new(BigInteger256([
                    0x56c59d9f53557d1f,
                    0x37df92ef9e05cf0d,
                    0x47b6da60d93a0d8f,
                    0x0084ca7931ce7357,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x6aa1f4b53efcf821,
                    0xe67d7b6d959ebc85,
                    0x1981d650c335ab41,
                    0x068fc5c020bc4442,
                ])),
                Felt::new(BigInteger256([
                    0xd2572fe2c5753f01,
                    0x0b8b722cd6225bc6,
                    0xb2e5e434e1b743cf,
                    0x0cea3e7ea633fc47,
                ])),
                Felt::new(BigInteger256([
                    0xa7170e9afe7f139b,
                    0xd1a57362bb6e5e37,
                    0xafee29a68093fdba,
                    0x0edaee70234c0af8,
                ])),
                Felt::new(BigInteger256([
                    0x7874fbb0b3f827c5,
                    0xbd752a344d185fea,
                    0x06c73547eee03650,
                    0x2e5e7259f79a8a8e,
                ])),
                Felt::new(BigInteger256([
                    0xc41c28a1864ca120,
                    0x92227b46e6fbcfff,
                    0xb5dd05f18decd990,
                    0x13795929bab523ab,
                ])),
                Felt::new(BigInteger256([
                    0xb55f7773c62efbbf,
                    0xa86e4c41942065b3,
                    0xd9380fbefc8bea79,
                    0x2d2733e6b4a3cdce,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0xb5e1addd3ef6e4e3,
                    0xcbeb83506939800c,
                    0xcb2e4386be9639d6,
                    0x121c8a5ae9f96678,
                ])),
                Felt::new(BigInteger256([
                    0xb38a0974b842c93b,
                    0x034e6c9c76b7e44c,
                    0xa616c13c7ce4d239,
                    0x0648e0e3c7d8a93c,
                ])),
                Felt::new(BigInteger256([
                    0xa0a29c2d7db73eeb,
                    0x263576d6d2cb4428,
                    0x00b119881824073b,
                    0x08a1c81f06d52eed,
                ])),
                Felt::new(BigInteger256([
                    0xefa8754784f4365f,
                    0x5cc63ec40647c7a9,
                    0xb6913201c7b02f95,
                    0x15034816d5cd72d3,
                ])),
                Felt::new(BigInteger256([
                    0xaa103a4e1316c3a6,
                    0x2bd546bbdc78d4d9,
                    0xe621f204241fea40,
                    0x0fbbaa89cf65aeca,
                ])),
                Felt::new(BigInteger256([
                    0xf68c4c3cd4e5563a,
                    0x810c1464f2115181,
                    0x603b962eb7553ace,
                    0x1499a986d6f603d0,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x8622db2d817f1005,
                    0x5a9fe37b76ae93d3,
                    0x58d316818e86e417,
                    0x24aee20b519be3e0,
                ])),
                Felt::new(BigInteger256([
                    0x75b89d96bf79d8d8,
                    0x093dddae4fb26c30,
                    0xc790327bff8621d8,
                    0x02785c8cf608f67a,
                ])),
                Felt::new(BigInteger256([
                    0x5f06ff9e9d38eb19,
                    0x79cae4219b5248d5,
                    0x1c276670cc550d94,
                    0x0fdcafa68f06dfc1,
                ])),
                Felt::new(BigInteger256([
                    0x02280e8337a42234,
                    0x213142083753afbb,
                    0x08a7f6da8656cd6c,
                    0x00c5d1a43fa50357,
                ])),
                Felt::new(BigInteger256([
                    0xed05263c74b624e2,
                    0xe96659618dd841f8,
                    0x642b96ad1701c7e3,
                    0x0eb09456a6c35c11,
                ])),
                Felt::new(BigInteger256([
                    0x55e1079b2d8f1b24,
                    0x05d5e8099d4bd8fa,
                    0xd2fc920e1798748a,
                    0x04677565e4b6736a,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x91bd9fa50dcc5bbb,
                    0x581cf3bc3ad80aa8,
                    0x1d528c9e74d0997c,
                    0x24531fe236813c9b,
                ])),
                Felt::new(BigInteger256([
                    0xba22bcbd97ec096a,
                    0xe847bf549a64d7f7,
                    0xc8b27efd9ec03a8d,
                    0x26b612d09feccd21,
                ])),
                Felt::new(BigInteger256([
                    0x439c9c88583f4bc3,
                    0x28f194abcfc6db70,
                    0x20375ebc5f038b33,
                    0x16692a76229e53f1,
                ])),
                Felt::new(BigInteger256([
                    0xe88b72e17572ed7f,
                    0x8a5dffcc54b9582a,
                    0xdd33cbc567075738,
                    0x2f7bf0e0356e02e0,
                ])),
                Felt::new(BigInteger256([
                    0xdda27786a80326aa,
                    0x0aa93a7862d137f6,
                    0xa7834d64350a2fce,
                    0x0eafcf531038314c,
                ])),
                Felt::new(BigInteger256([
                    0x8491692fab036078,
                    0x11f5beaa51c10afa,
                    0x4b1d9e03be5bdfd5,
                    0x008853d3fdbb445b,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0xea265193a18ec743,
                    0xfad05003682b0636,
                    0xc2c05f6723ac2aa9,
                    0x2c9fb761476f4f2a,
                ])),
                Felt::new(BigInteger256([
                    0x3966e6a3f6e08096,
                    0x9bb1f52475e60901,
                    0x366681b4edc36413,
                    0x19cdba8e1f962f94,
                ])),
                Felt::new(BigInteger256([
                    0xe9d5611b2ec3b70e,
                    0x54778a3235567dfc,
                    0xd87b63c857032d6e,
                    0x0f05e2d0b61d1466,
                ])),
                Felt::new(BigInteger256([
                    0x1a5e3ce61b76f019,
                    0xc742875e2d90c9dc,
                    0x49edef5fa0f11c03,
                    0x17c119e1ea0a3664,
                ])),
                Felt::new(BigInteger256([
                    0x6dba3627d3c184ac,
                    0xbdb6c48d53788ad7,
                    0xbaabf3219cae4ae2,
                    0x261afe8c21ff1525,
                ])),
                Felt::new(BigInteger256([
                    0x4eb621a861fa23a9,
                    0xa03f53b51fb9509e,
                    0x661ccd0eccc460a5,
                    0x2e265a628c5b36c1,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x30deaf92491fc681,
                    0x6638fd37489cb190,
                    0xb5644af7cf4674f0,
                    0x05ad9c00693c3370,
                ])),
                Felt::new(BigInteger256([
                    0x0bf605bdb7decfed,
                    0x1f12a9c1d35dfbb5,
                    0x623abe23c7e1abb7,
                    0x0f6cebd1fd200b79,
                ])),
                Felt::new(BigInteger256([
                    0x7bba4bc2fdc5eea2,
                    0x415111cb05b747af,
                    0x5a02d636587a4127,
                    0x1e182e6e2d56ff8d,
                ])),
                Felt::new(BigInteger256([
                    0xccfdeeeef58991a8,
                    0x025472d8efb2a07e,
                    0xe3b66ad6636bd67b,
                    0x1012b517bf681af6,
                ])),
                Felt::new(BigInteger256([
                    0x5fe8918ddc290216,
                    0xdda9fe3ac894521e,
                    0xdfd5731277a5e7cb,
                    0x1a84a25472fa8010,
                ])),
                Felt::new(BigInteger256([
                    0x12ef0697b62aa349,
                    0x10d3cdd65550baf4,
                    0x88b612b487dacdd3,
                    0x1e01982d1bf67f05,
                ])),
            ],
        ];

        for (input, expected) in input_data.iter().zip(output_data) {
            assert_eq!(expected.to_vec(), AnemoiHash::compress(input));
        }

        for (input, expected) in input_data.iter().zip(output_data) {
            assert_eq!(expected.to_vec(), AnemoiHash::compress_k(input, 2));
        }

        let input_data = [
            vec![Felt::zero(); 12],
            vec![Felt::one(); 12],
            vec![
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
            ],
            vec![
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::one(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
                Felt::zero(),
            ],
            vec![
                Felt::new(BigInteger256([
                    0x8a11fdbc13b898b0,
                    0xfe76da990731b734,
                    0xce0217879f750a26,
                    0x08c1b7ec136d7e05,
                ])),
                Felt::new(BigInteger256([
                    0x22844800104f6baf,
                    0x44944f8b11c875d7,
                    0x2e5e705f72e735b4,
                    0x205d611ab983a94c,
                ])),
                Felt::new(BigInteger256([
                    0x4fd3329479df70b1,
                    0x49eb0486d88f0012,
                    0x9badec50d8c8db17,
                    0x1d1e745024be42ae,
                ])),
                Felt::new(BigInteger256([
                    0x763462d72dd73daa,
                    0xf9f6ea4e18f9cedf,
                    0xd8c9dfefc14367e9,
                    0x060c69e5b82556b2,
                ])),
                Felt::new(BigInteger256([
                    0x71e17eb96e0e706e,
                    0x24a279acf01b76cc,
                    0x0a99ff473b187f8a,
                    0x2d923c3964f23074,
                ])),
                Felt::new(BigInteger256([
                    0x0b96511633cb60a5,
                    0x43464576269ff5d1,
                    0x0489b3740d9f4935,
                    0x1e4b62c51e9ee4db,
                ])),
                Felt::new(BigInteger256([
                    0x8ce57d588c436dd8,
                    0x1dc70cdd9f3a5afb,
                    0xee2f42d6acf836c9,
                    0x1619fcb42b2045fa,
                ])),
                Felt::new(BigInteger256([
                    0x58a8098201babc8b,
                    0x1376ca8fdf851346,
                    0x17d838db0d619a40,
                    0x0c0905105e808dd8,
                ])),
                Felt::new(BigInteger256([
                    0x231cfca5de6843a2,
                    0x7700f52e7a11124d,
                    0x805bdcf1d326265b,
                    0x1b5004154b9bd3cf,
                ])),
                Felt::new(BigInteger256([
                    0xea51542d74b3b662,
                    0x86eda9b3c41768af,
                    0x81065ebab085f570,
                    0x29c7e93c2e4db684,
                ])),
                Felt::new(BigInteger256([
                    0x549e6099790c912d,
                    0x78df3dcbb575d186,
                    0xf73da4123882d7ad,
                    0x27f58b4465829b30,
                ])),
                Felt::new(BigInteger256([
                    0x12bc6c397faecae2,
                    0xb3998f7f24a6f88a,
                    0x3769bdd4c9ad8eca,
                    0x043a9a56c5c75450,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0x3049012d69aa685a,
                    0xb6cca84a35930cdd,
                    0x8660b8638e5c2fc3,
                    0x0006700168ea98e0,
                ])),
                Felt::new(BigInteger256([
                    0x99eedf55a39cf298,
                    0x4ac43ef10313fb5b,
                    0x6a45386575ae50ac,
                    0x22e3b85a443e7286,
                ])),
                Felt::new(BigInteger256([
                    0x72f3c2ef85506e71,
                    0x5d8da95bc1613731,
                    0x33a2b6c5d7119259,
                    0x1caab0a0f7ae3a6e,
                ])),
                Felt::new(BigInteger256([
                    0x291a45d1d149fb16,
                    0x549a26e22579dd0d,
                    0xf935e97a0e0a7d89,
                    0x1b45e07a80765eeb,
                ])),
                Felt::new(BigInteger256([
                    0x5b32da3b639403eb,
                    0x0625e359e18561f0,
                    0x94ad99d477c0a5bc,
                    0x19f8731ffdfdc27e,
                ])),
                Felt::new(BigInteger256([
                    0x1b8fb3eb2689ca3d,
                    0x5d43e2787ff8bdf9,
                    0x8ad70acce35628ff,
                    0x012e06693c1d0ac9,
                ])),
                Felt::new(BigInteger256([
                    0xb320ca508fc95b47,
                    0x724e710433d67820,
                    0x1ba5fc30d75d628d,
                    0x1f268cbb73131b2f,
                ])),
                Felt::new(BigInteger256([
                    0xa6a9250329670b7f,
                    0x747e619c292fc03b,
                    0x0854625b4e3873a0,
                    0x09551da95b98e162,
                ])),
                Felt::new(BigInteger256([
                    0x8631b3f29e5121ea,
                    0xf10c70b85beab7d1,
                    0x5afd3567ba45e9c8,
                    0x0830d6da5b18c162,
                ])),
                Felt::new(BigInteger256([
                    0x68c080b12c0a7b12,
                    0x4abef9fabfca9ef6,
                    0x92ce51e9de141009,
                    0x0b667011219ecc6d,
                ])),
                Felt::new(BigInteger256([
                    0xb92063839cdc415e,
                    0xf6acc4cf275bb04b,
                    0x6e14cfb596e4582e,
                    0x1bf6c8bca251f5e0,
                ])),
                Felt::new(BigInteger256([
                    0x0d6557da3113cf38,
                    0x6c7ca15f2a9128c7,
                    0x87321803626d7df7,
                    0x1930bcec15eb6482,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0x44bceb70087bc481,
                    0xdfc6566ed493bf59,
                    0xb3eab080ba59d17d,
                    0x18e4fdb8041bbecd,
                ])),
                Felt::new(BigInteger256([
                    0x22fc4796d2c31cd5,
                    0x9bb8fc162c6aeab9,
                    0x861753b404cd540f,
                    0x0aba13992e73d2dc,
                ])),
                Felt::new(BigInteger256([
                    0x5cd34c0ceb099f46,
                    0x1d2547061c44a0c2,
                    0x09b968c3725b85aa,
                    0x040a057fd53fec08,
                ])),
                Felt::new(BigInteger256([
                    0xbf261256b9451801,
                    0xd94e26e684aeb308,
                    0xf5c520d632cedf63,
                    0x1ceb89d30c84fbc5,
                ])),
                Felt::new(BigInteger256([
                    0x675347258d431e37,
                    0x7a5d9dbf4a400795,
                    0x52380a0e3eb43020,
                    0x30242c1ace7523b7,
                ])),
                Felt::new(BigInteger256([
                    0x332e28e44f8aea55,
                    0x3c7daa02962ba662,
                    0x7a0331a8d221ba39,
                    0x168b475013155f2c,
                ])),
                Felt::new(BigInteger256([
                    0x6b7093d89c3a4ba7,
                    0x1578d5fb50c20edd,
                    0x1d0989fb97caad9f,
                    0x1f11345ae8ff8eb6,
                ])),
                Felt::new(BigInteger256([
                    0xd60211d54278365d,
                    0xdb7c9746bd83f592,
                    0x61fe9633a3a0d6cc,
                    0x0210e89c394b91f3,
                ])),
                Felt::new(BigInteger256([
                    0x0c9d6472e6d30e5c,
                    0x0c6398ae99d87872,
                    0x7700a55f0e58fa5c,
                    0x2b80e5dfff905f69,
                ])),
                Felt::new(BigInteger256([
                    0x9c83a5fac6c08208,
                    0x11c599611fbaee73,
                    0x84f9be839f576e22,
                    0x1430483e27075628,
                ])),
                Felt::new(BigInteger256([
                    0x11ec875c04d5569a,
                    0x021bced40b4441e6,
                    0x15e12106ac118639,
                    0x2ee0384547e3cdfb,
                ])),
                Felt::new(BigInteger256([
                    0xd1a80f2bc7e03a97,
                    0xa0b22bb8591a681b,
                    0x5192412f3bbc6ed1,
                    0x27e4059680a63e7d,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0xc8a3ed3d366c5e1c,
                    0x39a7733f0d5b64a7,
                    0x39c7920d08cbbe69,
                    0x25f4ae9fa932f24c,
                ])),
                Felt::new(BigInteger256([
                    0x5a7e24b20e319072,
                    0x659b26c6347c620f,
                    0x1a3686aeb6f5eea4,
                    0x0bdfef1be4c95eff,
                ])),
                Felt::new(BigInteger256([
                    0xe5412a0752b6b9ca,
                    0x7ec2b0b60f33f695,
                    0xe3eafec53317b617,
                    0x2400b2f93ddff5b9,
                ])),
                Felt::new(BigInteger256([
                    0xa72eb5abf710238d,
                    0x0d6890ef2b31f047,
                    0x26562f20deaa02d5,
                    0x24de825a07bddfad,
                ])),
                Felt::new(BigInteger256([
                    0xc6e1502a3802cecc,
                    0x487dd21d8cd40ab1,
                    0xf695e60c5d21433c,
                    0x2da3a12a1e54b439,
                ])),
                Felt::new(BigInteger256([
                    0xafe8c63dda6d463c,
                    0x94a1bf3eec7a8cb2,
                    0x5a93ecf69d86fa69,
                    0x2e614b7b47eea7ce,
                ])),
                Felt::new(BigInteger256([
                    0xfc34ab69f5066e9f,
                    0x1cb8690d1a576f2d,
                    0xd5fbcc8b8d1e690c,
                    0x2e007171d9fd5379,
                ])),
                Felt::new(BigInteger256([
                    0x28867d2a4f9f674d,
                    0xee63217a8aad05f8,
                    0xfd439a644e8dc889,
                    0x1b94bc70a77617f1,
                ])),
                Felt::new(BigInteger256([
                    0x29e5651474b122cc,
                    0x19c15f17618d6649,
                    0x72cac45e45ba0e58,
                    0x22309f3094dbadef,
                ])),
                Felt::new(BigInteger256([
                    0xd6251e45cae84403,
                    0xa3be5020c365a0bc,
                    0x0c9661600de18264,
                    0x122723c362f5ae97,
                ])),
                Felt::new(BigInteger256([
                    0x2893ff78e8171392,
                    0x483cd9117e83e243,
                    0xd8f8402382ba6d32,
                    0x09a25c834024d1b7,
                ])),
                Felt::new(BigInteger256([
                    0xa3c6f3f8af973b69,
                    0x53d7920828e3b4fa,
                    0x141c5da3f40baec2,
                    0x27790515575752c2,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0x2ed4d888d4f0b4fa,
                    0xce618e405aeac7ff,
                    0x90066047f18c9476,
                    0x16f37d19d8bb3b70,
                ])),
                Felt::new(BigInteger256([
                    0xdd1b01f5835f801c,
                    0xf383d6bf3362cc3b,
                    0x34c012298b0492f1,
                    0x07c15078049f51c9,
                ])),
                Felt::new(BigInteger256([
                    0x3f377b844edcdfe0,
                    0xe7caa5715efb8197,
                    0x5d7d1a9f9827561d,
                    0x12c73103f5834a0a,
                ])),
                Felt::new(BigInteger256([
                    0x6d3057f2d163a085,
                    0xc97ba4c8a3e2ec16,
                    0xb456f852c3926fa0,
                    0x098ad1abdabb440c,
                ])),
                Felt::new(BigInteger256([
                    0x43d0c42bbc9a8c43,
                    0x05cba40b83404fab,
                    0xd88b78e09d5619b0,
                    0x0d7f33eb39ef70c9,
                ])),
                Felt::new(BigInteger256([
                    0xb058028a70f78d17,
                    0xb280b1afc6486827,
                    0xfca06513264ac174,
                    0x25475568a92757b8,
                ])),
                Felt::new(BigInteger256([
                    0xf522073ed67e8ae4,
                    0x20e24ee76af28521,
                    0xea6c295f6b38f3b2,
                    0x0fd77dda13098f2d,
                ])),
                Felt::new(BigInteger256([
                    0x279e8c7677797c87,
                    0xb00abdd7e6baaffd,
                    0xe6d77f2b842ebf6b,
                    0x1b4ead2d39e47fe5,
                ])),
                Felt::new(BigInteger256([
                    0x3520f2fb821d652f,
                    0x2f51947bf406f7c5,
                    0x57c33f62cca4375b,
                    0x10f63c1bac558336,
                ])),
                Felt::new(BigInteger256([
                    0x522f92ff09142960,
                    0x9d249f9e3e4d324f,
                    0xde90f709b959ce2b,
                    0x0857f97f7e99b896,
                ])),
                Felt::new(BigInteger256([
                    0x3dab161c6a2d5bbe,
                    0x509fa9c7823d8feb,
                    0x453bad132ce74252,
                    0x0ee89e7520e655da,
                ])),
                Felt::new(BigInteger256([
                    0x92dbb9c6174d7a2b,
                    0x2e7c391ac7c9bdd9,
                    0x911edfc7d1c97747,
                    0x174e4033c363ab22,
                ])),
            ],
            vec![
                Felt::new(BigInteger256([
                    0x8ae721b2ca186f14,
                    0xa464f8f93eed13cb,
                    0x6fc5b2f820fcd1d2,
                    0x137666bdb8ac02a8,
                ])),
                Felt::new(BigInteger256([
                    0x7d8f4a728a497ce4,
                    0x58f6f89ca794e2c4,
                    0xb52e6c03b23f9572,
                    0x196c1956828b5e25,
                ])),
                Felt::new(BigInteger256([
                    0xf2299d5e92f05c78,
                    0x8631dc3535ae9753,
                    0x86f34016830a3997,
                    0x2451918fb86a9e76,
                ])),
                Felt::new(BigInteger256([
                    0xabc43f1abee0b215,
                    0xf329492bedb746dc,
                    0xe262d5d75577894f,
                    0x07704fbb816f612e,
                ])),
                Felt::new(BigInteger256([
                    0x142fc783a9d9050c,
                    0x9edee62a01a1b43d,
                    0x141f732afe7ee8e2,
                    0x14d1ad0a0569ca9c,
                ])),
                Felt::new(BigInteger256([
                    0x538eddde327de532,
                    0x6e66c13c64ff70e9,
                    0x76fe4bdd822b32c3,
                    0x04e2f564b26768cc,
                ])),
                Felt::new(BigInteger256([
                    0xc88408438c56eab8,
                    0x15b9b4dbfac73341,
                    0x1a01e1e0a5faaf1c,
                    0x10ce5c4f4d7528d0,
                ])),
                Felt::new(BigInteger256([
                    0x281fd705f863aef3,
                    0x8cc70f2181a86d35,
                    0xd2b20b02c3b7735c,
                    0x188ad16f9ae6f80f,
                ])),
                Felt::new(BigInteger256([
                    0xfb8bcc59bcc22d18,
                    0xf03dd86298a44e39,
                    0x9c5258bc14a5afec,
                    0x10882ad9029fae19,
                ])),
                Felt::new(BigInteger256([
                    0xbc066e3b291c2a4b,
                    0xfb301e870a0ebf8f,
                    0xa8a79c634897b39b,
                    0x10d14732a8cbde24,
                ])),
                Felt::new(BigInteger256([
                    0xb257116291cfde4f,
                    0x55ba54e1211eed8a,
                    0x06da002c0c375969,
                    0x2acedd1a4515b090,
                ])),
                Felt::new(BigInteger256([
                    0x81ec63c2c178351d,
                    0xf35a74bb7d5ff8d6,
                    0x7e7d5f5809f6aeb2,
                    0x10ed52c04f9d254d,
                ])),
            ],
        ];

        let output_data = [
            [
                Felt::new(BigInteger256([
                    0xeb9a486dfc02cd2f,
                    0x5bc9d43bdbec1519,
                    0x67379d6b17cf3ff0,
                    0x2c319a57a3b5ccc3,
                ])),
                Felt::new(BigInteger256([
                    0x7f2b61b3f1e786df,
                    0xff5bad72956b96d2,
                    0x44819ee7d9df8ca4,
                    0x0b4ed3c38ce9ec3a,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x2389dd74f9c664aa,
                    0xfcc4300a8b27895d,
                    0x64f02cb5aaa98118,
                    0x04b662756617e694,
                ])),
                Felt::new(BigInteger256([
                    0x52ccea89808f1270,
                    0x1029bde1fd5e72dd,
                    0x1876938266d3f98d,
                    0x1992ca08d76014f8,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0xc611a3c797b81968,
                    0x51b8eb6c335c3e01,
                    0x2a205d2a90209bfd,
                    0x1b67e22e423db9b2,
                ])),
                Felt::new(BigInteger256([
                    0x201a08d9f2221244,
                    0xc12656a96b04a09a,
                    0x760ffeef5ca16142,
                    0x260b08b8f8d2441b,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0xfa5a0278227f0ae5,
                    0xb59e155665b27340,
                    0x13a580715b2fb2b1,
                    0x1edf59732976d756,
                ])),
                Felt::new(BigInteger256([
                    0x7357edb239839c29,
                    0xaf3f45a933702e91,
                    0xe90e5243b90f72bf,
                    0x0ec5a046836f3820,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x5c04c9a9d43d2f88,
                    0x20fc3b339544eaa4,
                    0x720bb2721b7e8f34,
                    0x12a05ff3ddc54272,
                ])),
                Felt::new(BigInteger256([
                    0x69486d7411ac1e3a,
                    0xf6406b86f738017c,
                    0x8241603c268285bb,
                    0x0880d58a7b69fad8,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x482ea8da3e855ebe,
                    0xaa21bb9f47251e02,
                    0xfae1ca06bb2d3ebb,
                    0x03e2952a286458c3,
                ])),
                Felt::new(BigInteger256([
                    0x1929f4a4a9586370,
                    0x76337c6157466e0d,
                    0x3b391d0b95ee55ae,
                    0x22e6f730ec031b4d,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0xf97a06f9144e1a74,
                    0x12c2162944a041e2,
                    0x9cc78ccba545c6a9,
                    0x3010466e75ff23b9,
                ])),
                Felt::new(BigInteger256([
                    0xff0282bbc0d6878b,
                    0x6f5bdaac319d9706,
                    0x78489dec1a4e6351,
                    0x2fd107cbcaaa178e,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0xdcaf7b9a6b3743b7,
                    0xd8275fd6ff10068e,
                    0x72925f068c7f28a2,
                    0x1dec0e3238b51bf1,
                ])),
                Felt::new(BigInteger256([
                    0x405178bb48d34c30,
                    0xf932236c4409ce3c,
                    0xfbd1e39353cedd4b,
                    0x1a28250ca82f033b,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x5fff69d1f646e706,
                    0x8a93bcb98e11a640,
                    0x10825b281fe4bc1c,
                    0x0e7d3721afe16816,
                ])),
                Felt::new(BigInteger256([
                    0x7eb0c220ca49665f,
                    0x7b934f3bedb185ce,
                    0x2072ea8fadcb9cb8,
                    0x253844963af570dd,
                ])),
            ],
            [
                Felt::new(BigInteger256([
                    0x2edeb633108c4a1b,
                    0x6d44c7a1f7310acd,
                    0xe70b30e055901c55,
                    0x04555b85645aea1a,
                ])),
                Felt::new(BigInteger256([
                    0x4bdc3bf66ad2e2c4,
                    0x91b096912e4ca247,
                    0xa6073bb21760f576,
                    0x1d2481466bef208a,
                ])),
            ],
        ];

        for (input, expected) in input_data.iter().zip(output_data) {
            assert_eq!(expected.to_vec(), AnemoiHash::compress_k(input, 6));
        }
    }
}
