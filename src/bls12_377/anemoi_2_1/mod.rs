use super::{mul_by_generator, sbox, BigInteger384, Felt};
use crate::{Jive, Sponge};
use ark_ff::{Field, One, Zero};
use unroll::unroll_for_loops;

/// Digest for Anemoi
mod digest;
/// Sponge for Anemoi
mod hasher;
/// MDS matrix for Anemoi
mod mds;
/// Round constants for Anemoi
mod round_constants;

pub use digest::AnemoiDigest;
pub use hasher::AnemoiHash;

// ANEMOI CONSTANTS
// ================================================================================================

/// Function state is set to 2 field elements or 96 bytes.
/// 1 element of the state is reserved for capacity.
pub const STATE_WIDTH: usize = 2;
/// 1 element of the state is reserved for rate.
pub const RATE_WIDTH: usize = 1;

/// The state is divided into two even-length rows.
pub const NUM_COLUMNS: usize = 1;

/// One element (48-bytes) is returned as digest.
pub const DIGEST_SIZE: usize = RATE_WIDTH;

/// The number of rounds is set to 19 to provide 128-bit security level.
pub const NUM_HASH_ROUNDS: usize = 19;

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
/// Applies exponentiation of the current hash
/// state elements with the Anemoi S-Box.
pub(crate) fn apply_sbox(state: &mut [Felt; STATE_WIDTH]) {
    let mut x: [Felt; NUM_COLUMNS] = state[..NUM_COLUMNS].try_into().unwrap();
    let mut y: [Felt; NUM_COLUMNS] = state[NUM_COLUMNS..].try_into().unwrap();

    x.iter_mut().enumerate().for_each(|(i, t)| {
        let y2 = y[i].square();
        *t -= mul_by_generator(&y2);
    });

    let mut x_alpha_inv = x;
    x_alpha_inv
        .iter_mut()
        .for_each(|t| *t = sbox::exp_inv_alpha(t));

    y.iter_mut()
        .enumerate()
        .for_each(|(i, t)| *t -= x_alpha_inv[i]);

    x.iter_mut().enumerate().for_each(|(i, t)| {
        let y2 = y[i].square();
        *t += mul_by_generator(&y2) + sbox::DELTA;
    });

    state[..NUM_COLUMNS].copy_from_slice(&x);
    state[NUM_COLUMNS..].copy_from_slice(&y);
}

#[inline(always)]
/// Applies matrix-vector multiplication of the current
/// hash state with the Anemoi MDS matrix.
pub(crate) fn apply_mds(state: &mut [Felt; STATE_WIDTH]) {
    state[0] += mul_by_generator(&state[1]);
    state[1] += mul_by_generator(&state[0]);
}

// ANEMOI PERMUTATION
// ================================================================================================

/// Applies an Anemoi permutation to the provided state
#[inline(always)]
#[unroll_for_loops]
pub(crate) fn apply_permutation(state: &mut [Felt; STATE_WIDTH]) {
    for i in 0..NUM_HASH_ROUNDS {
        apply_round(state, i);
    }

    apply_mds(state)
}

/// Applies an Anemoi round to the provided state
#[inline(always)]
#[unroll_for_loops]
pub(crate) fn apply_round(state: &mut [Felt; STATE_WIDTH], step: usize) {
    state[0] += round_constants::C[step % NUM_HASH_ROUNDS];
    state[1] += round_constants::D[step % NUM_HASH_ROUNDS];

    apply_mds(state);
    apply_sbox(state);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apply_naive_mds(state: &mut [Felt; STATE_WIDTH]) {
        let mut result = [Felt::zero(); STATE_WIDTH];
        for (i, r) in result.iter_mut().enumerate().take(STATE_WIDTH) {
            for (j, s) in state.iter().enumerate().take(STATE_WIDTH) {
                *r += *s * mds::MDS[i * STATE_WIDTH + j]
            }
        }

        state.copy_from_slice(&result);
    }

    #[test]
    fn test_sbox() {
        // Generated from https://github.com/Nashtare/anemoi-hash/
        let mut input = [
            [Felt::zero(), Felt::zero()],
            [Felt::one(), Felt::one()],
            [Felt::zero(), Felt::one()],
            [Felt::one(), Felt::zero()],
            [
                Felt::new(BigInteger384([
                    0x1586e378d7065336,
                    0xe51668bf90dcbd4f,
                    0xa02ddcb1ff06f3ec,
                    0x1d3c2c19d524e0e5,
                    0x27686446d1d62e31,
                    0x00adb6131abd4d08,
                ])),
                Felt::new(BigInteger384([
                    0x0aff9898091ffb98,
                    0xfc3d2d2d7cb7b51b,
                    0x5e33e51f9a396d65,
                    0x6c9650a24f348f61,
                    0x54bdf0fed321cf51,
                    0x000e2c88457a7e21,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0xb843088859f4f528,
                    0x067fc1482b55b34b,
                    0x60b3c8fa17b96350,
                    0x28a64f0f9d0b918b,
                    0xc0d69b46f9de6eab,
                    0x00301a2dda213a66,
                ])),
                Felt::new(BigInteger384([
                    0xaaf9ea9673e7312c,
                    0xabbedf8a634f2a49,
                    0xbb2f53353a87d185,
                    0xad2a7fa7ad5a7c2b,
                    0xb2fc67788fb8ffb9,
                    0x014d35b6b40505dd,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x0e4fc8701baab5b0,
                    0x58076fce98bb1dc4,
                    0x3904a3ea9295354f,
                    0xb83a3450aa5f7a44,
                    0xa361dfe34839fcaa,
                    0x0120287ad2fcba13,
                ])),
                Felt::new(BigInteger384([
                    0xd238be36c0278d77,
                    0xfd3ebf209170dfcd,
                    0x539f971d669ead60,
                    0x2a126f427161a070,
                    0xbca8c4ec6523e290,
                    0x002dae4a872b5c06,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x1c01c546a2633bea,
                    0x42ccd67104a4e169,
                    0xa73dc50f638908ec,
                    0x6083a997022c0345,
                    0x076485047d945817,
                    0x004d10729697ca92,
                ])),
                Felt::new(BigInteger384([
                    0x674aa98f943f77c4,
                    0xc0ce63abf275854d,
                    0x27afa36a641e045e,
                    0x8bedbdf5b6b45bcc,
                    0x7d27a2beb54d13ab,
                    0x00e30e5310ac86a7,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x2d8470cb4ee45f4c,
                    0x384522a61c4573a0,
                    0xfb69f3e384e7c1b6,
                    0x3e5bb6c37a524409,
                    0x730cb9c4a4992fda,
                    0x003c20087d555c03,
                ])),
                Felt::new(BigInteger384([
                    0xbb47fc9e2d129ad4,
                    0xdcd3edbce2b63a50,
                    0x214192ad1b78253d,
                    0x2899464c0f5aff60,
                    0x4abffc9403edb774,
                    0x01343420801028ba,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x3477ebc57026e5b5,
                    0xc3eedaa2570dd199,
                    0x29f6a6a3ab302298,
                    0xf023298dfb260449,
                    0x214aaff215ad7c6d,
                    0x00c57098ea9d2136,
                ])),
                Felt::new(BigInteger384([
                    0x84f1f9be9be83817,
                    0xfb33cbe181149adf,
                    0x7f1f26bc64d205f4,
                    0x126191e4f60c135f,
                    0xffb20dfae7ed09b7,
                    0x0167a09be59b728c,
                ])),
            ],
        ];

        let output = [
            [
                Felt::new(BigInteger384([
                    0x56dcddddddddddd4,
                    0x2db2015f37777772,
                    0x8a5a595c4be8b110,
                    0x2041bbb36e056126,
                    0x7e422da67ad9b5fd,
                    0x007c276e8cf025e2,
                ])),
                Felt::zero(),
            ],
            [
                Felt::new(BigInteger384([
                    0x59b3fa50b6287770,
                    0x8d8cc2314dc632d6,
                    0x29b1a14daf4e27f8,
                    0xedb8547b775eb818,
                    0xd8e661a68b195234,
                    0x0105b860faad5b24,
                ])),
                Felt::new(BigInteger384([
                    0xa578190a443f6866,
                    0xba8dc4fbbf9b29b6,
                    0x7eb0b8ad3adf4bee,
                    0x878939a768b75022,
                    0x5a0c806ddc201082,
                    0x00b44682633de315,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x3f83be968f6712f4,
                    0x7a14ed2c157a3e9f,
                    0x13379f76f22125ab,
                    0xd2b0fd505a8223b8,
                    0x90580d58fcb34c3f,
                    0x0073a0b34fa4fca9,
                ])),
                Felt::new(BigInteger384([
                    0x6af47e2abade4fc8,
                    0x1a826133f52d710b,
                    0x6c3926ec49ea6ba8,
                    0xbfd567223db99dfb,
                    0xca056d88e9e47a82,
                    0x0036eb7a315a0db0,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0xea911dddddddd44f,
                    0xce8327424777727f,
                    0xe774a906518e4834,
                    0x527cf56b51022fb4,
                    0x6e646cdc5f7b965d,
                    0x00eb6a2e45f61af1,
                ])),
                Felt::new(BigInteger384([
                    0x823ac00000000099,
                    0xc5cabdc0b000004f,
                    0x7f75ae862f8c080d,
                    0x9ed4423b9278b089,
                    0x79467000ec64c452,
                    0x0120d3e434c71c50,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x84e9bdfd865e5089,
                    0x0eab1d3400458b74,
                    0xa74a78f9cebc5750,
                    0xa0987466f33c0d1a,
                    0xa1d77beffb04e10c,
                    0x00f619ef9ddfb857,
                ])),
                Felt::new(BigInteger384([
                    0xa52332c502e8f2b2,
                    0xb00092cd1a81d674,
                    0x02bd99a257d157cd,
                    0x4e4e66224eb50d34,
                    0x490a41d0a8f0af0a,
                    0x00c9d0b091fc41e7,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x786ff5f457d0ecd9,
                    0xfae4abe3c01bec25,
                    0xadd9107f0b00bb59,
                    0x68ecd0edbce2e9b4,
                    0xe3e7418bcfe6df03,
                    0x0198bc0843aa45d0,
                ])),
                Felt::new(BigInteger384([
                    0x5a9edfe74f1a3a3e,
                    0x7de5b62af61bc2f8,
                    0xce5dbd562ad9d9af,
                    0xab34a2a4ea35516d,
                    0xb3232c61f92db5af,
                    0x01819a8d709cd99c,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x7ee8d8d054391c63,
                    0xe6fa70920f4a23ae,
                    0xd6f3c78e60216b15,
                    0x46081f64c6e55ccf,
                    0x52d22d34971c69b2,
                    0x0067234b2c5992ed,
                ])),
                Felt::new(BigInteger384([
                    0xc3d853c81dd63cfd,
                    0x0d8c51e01fa15b65,
                    0xb8ac74113761a05c,
                    0x6947fd7c6b38841b,
                    0xafa252ad90fad31c,
                    0x0157b6b8e79d80f0,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x40210e03efbcf044,
                    0xdc4d424546f8105f,
                    0x9c12d2fe2e8df23c,
                    0x7fc44d6b14cad81c,
                    0x79032682da343e51,
                    0x0005aa61f13d1f92,
                ])),
                Felt::new(BigInteger384([
                    0x40f45d409063a560,
                    0x59e94df05e1d0af0,
                    0xef2acd01cb46f018,
                    0xe9d078b7c38e4c21,
                    0x5b2bf7f316fe692d,
                    0x00dcd42aba8cb31f,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0xaf134b54892dd66a,
                    0x4d252476e4d3d808,
                    0xe1dcc1b00b2b3c5f,
                    0x85883dec47566551,
                    0xa55551b061956f17,
                    0x003b53b315e5bd4b,
                ])),
                Felt::new(BigInteger384([
                    0xe1de592d0226e734,
                    0xe7a29bf432a04950,
                    0xd297274c627a8516,
                    0x6c2b4308b6c9997a,
                    0xa7e6c2e743ba9f27,
                    0x013b20924a407f22,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x9017a550ad04789c,
                    0xd275e01065eea50b,
                    0x55ca9d4daf222657,
                    0x3e8837e81c8ce31d,
                    0xb07932859e50503e,
                    0x0108d3ad354bde61,
                ])),
                Felt::new(BigInteger384([
                    0x435f90274d805a56,
                    0xeb7ac832e8ba8b8e,
                    0xa808155693c484e8,
                    0x5f46e62878fee442,
                    0xf3b050434ca89302,
                    0x006508ff23925784,
                ])),
            ],
        ];

        for i in input.iter_mut() {
            apply_sbox(i);
        }

        for (&i, o) in input.iter().zip(output) {
            assert_eq!(i, o);
        }
    }

    #[test]
    fn test_mds() {
        // Generated from https://github.com/Nashtare/anemoi-hash/
        let mut input = [
            [Felt::zero(), Felt::zero()],
            [Felt::one(), Felt::one()],
            [Felt::zero(), Felt::one()],
            [Felt::one(), Felt::zero()],
            [
                Felt::new(BigInteger384([
                    0x7e24061934810e7e,
                    0xf7dce3fff8817c92,
                    0x7a6f997f3f5fbda1,
                    0xba407a6aa5d031b4,
                    0x9c75025061c7cedb,
                    0x014adaa628dd6345,
                ])),
                Felt::new(BigInteger384([
                    0xf799aa1c461da4e5,
                    0x8519fbd3813a0834,
                    0x6df0b0eb0f296e27,
                    0x259af58caf9be120,
                    0x32bab058411a7fb9,
                    0x0173e87fd956b8e9,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x9de2f32d1f792ef2,
                    0x33f3cc2b5983ea94,
                    0xcbff2ee8dbc1e6a1,
                    0xf88bf9a2fa7749d9,
                    0x34d7c4505136877e,
                    0x00be477c879a502b,
                ])),
                Felt::new(BigInteger384([
                    0x87de1cbf8db62777,
                    0x2dabe565c71141f6,
                    0xca98338a5bd39ccc,
                    0x2eb8b48c8f396c40,
                    0x3e2cda929c640165,
                    0x010885bdadf6bdf3,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x06bc9f36ec823bba,
                    0xaa015fc922fdf8ac,
                    0x9b43fdba31dfebd8,
                    0x84e054c1ad0c1c7e,
                    0x9ac47b6376a85cbd,
                    0x010e2fe0ac1f38b0,
                ])),
                Felt::new(BigInteger384([
                    0xd92bd34cf668009d,
                    0xf1de54335c82c58d,
                    0xac500cb145693da0,
                    0xfe1e79152a258866,
                    0xea226e7d7e0a50ef,
                    0x0070022883edbc36,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0xa3c8157ea4bfca60,
                    0xe1e5190e809ca226,
                    0x69a91f67e2c50a43,
                    0x4482e64ed1f04d98,
                    0x76bb965c731580de,
                    0x00e47527ff85e6b1,
                ])),
                Felt::new(BigInteger384([
                    0x93897205e6068b86,
                    0xfdd93372b13f8fa0,
                    0x72cffccd953de4c5,
                    0xbf05ea7bf6806171,
                    0xfaee94bb53ce9c93,
                    0x013c14d2b98d5e7b,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0xb4106ba1260296b0,
                    0xe51dfbf5f007101a,
                    0xc5a290411dd0f2ab,
                    0x099b4510c0ef5054,
                    0x467cf9cbc5e1141b,
                    0x017ecdfcdb91ab67,
                ])),
                Felt::new(BigInteger384([
                    0xe1cd7510d752476e,
                    0x57b3828a4ed2afd9,
                    0x52712cec9bce1b07,
                    0x8366bf50e518a556,
                    0x68e9649fe60c102f,
                    0x00f9483b8a4784bd,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x77aa5404c3b16087,
                    0x5106ef965896dd2f,
                    0x85f7864e56fab2ba,
                    0x76ffa8a48cae3b6b,
                    0xb40261b918cdf7dd,
                    0x001eb610676d2d25,
                ])),
                Felt::new(BigInteger384([
                    0x46fb04fc4cdd4dfb,
                    0xaf74abe2a658e49f,
                    0x752d30bdc934c129,
                    0x35e376982147f43b,
                    0xc28ef25963b802f8,
                    0x019c6d0335b66109,
                ])),
            ],
        ];

        let mut input2 = input;

        let output = [
            [Felt::zero(), Felt::zero()],
            [
                Felt::new(BigInteger384([
                    0x93b43ffffffff67b,
                    0xa0d125e30ffffb0d,
                    0x5d1a4faa05a59724,
                    0x323b39b7e2fcce8e,
                    0xf0223f35e4a1e060,
                    0x006f42bfb905f50e,
                ])),
                Felt::new(BigInteger384([
                    0x963abfffffff7099,
                    0x615462c0afffb57a,
                    0x983ad5e0f70bfb17,
                    0x043b91b1b7782f20,
                    0x460a32e63333859f,
                    0x005966855b430ccf,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x15eefffffffff714,
                    0x669be3a3bffffb5d,
                    0xdc8ffe3035319f32,
                    0xd10f7bf375757f17,
                    0x6968af36d106a4b2,
                    0x019016a3edcd115f,
                ])),
                Felt::new(BigInteger384([
                    0x05547fffffff7986,
                    0x11c3dc611fffba1e,
                    0xda9e39e07be3a3e5,
                    0x4d4eefb142f7c397,
                    0xa2dc896fcece2a27,
                    0x00778a27853b0c5a,
                ])),
            ],
            [
                Felt::one(),
                Felt::new(BigInteger384([
                    0x15eefffffffff714,
                    0x669be3a3bffffb5d,
                    0xdc8ffe3035319f32,
                    0xd10f7bf375757f17,
                    0x6968af36d106a4b2,
                    0x019016a3edcd115f,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x3eb33dc1503db7dc,
                    0x98cee8ee1ae7f7a6,
                    0x592dfad8b05489f1,
                    0x9a8fcc51e3806456,
                    0x84660cb6ae2492b4,
                    0x013c8494aff05d03,
                ])),
                Felt::new(BigInteger384([
                    0xecba086ff9bb6abe,
                    0x7bbc9fd904d18aec,
                    0x532d2b9065b76b4e,
                    0x148890e7f999eb09,
                    0x702c2fc7ca50f2c4,
                    0x01832c3322f47207,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0xe697e2666d257ee2,
                    0x119ff5bc5386c801,
                    0x945bc056b2d48e97,
                    0xca24e4545636f19d,
                    0xe1655f23a9670958,
                    0x011e1122e321d92a,
                ])),
                Felt::new(BigInteger384([
                    0xd86ce0bff2e896ab,
                    0x4f99a6c4cbf6fa0d,
                    0x4676a2c191eb27a5,
                    0x018591ff92ded0e5,
                    0xb4cf34254b20b14a,
                    0x00fd400c0f3fce4a,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0xac2b00b95c9a44e9,
                    0x79dad9bacea78bf9,
                    0x3827335e5ae56846,
                    0x001e0533216acc46,
                    0x39dcddbc26bdf5e1,
                    0x00e5672807f8fc3d,
                ])),
                Felt::new(BigInteger384([
                    0xc76ade2963720a3c,
                    0x5d562c03f853f92a,
                    0xff00fdbac89019c1,
                    0x2ec9f77c17bee40a,
                    0x1c3b3e805e216f46,
                    0x006f3b503d5bfc77,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x917583d71f21f72f,
                    0xc4221ad8d3560c89,
                    0xcf63b766a2ff59dc,
                    0x565c462138ee2c17,
                    0xa62d0e11b14287fc,
                    0x00ed2c7dd956b5de,
                ])),
                Felt::new(BigInteger384([
                    0x6c1d6ba0b904073e,
                    0xac727ec1634a4bab,
                    0x8319482397e0a0b4,
                    0xe3345ce343d746ce,
                    0xbf7f3400e5090148,
                    0x0001a3bb9fb46e46,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x41cb879dc3d4c619,
                    0x393c5aaade5f5dda,
                    0x83b5be70b4910019,
                    0xd166d14324c3505b,
                    0x7416ab666eea73cf,
                    0x00f9fd031fd4da3f,
                ])),
                Felt::new(BigInteger384([
                    0x0f6ba74f50c9e2dc,
                    0xe2d68b27a6692f9f,
                    0xf388e1d9a5f99480,
                    0xdd3358b503ecaaaa,
                    0x3e2a3bdc941d4648,
                    0x007f0ef391d2b634,
                ])),
            ],
            [
                Felt::new(BigInteger384([
                    0x59e51ecd44a8f22e,
                    0x563de82377cc427d,
                    0xb24e02d0f490142a,
                    0x316bb044727e7913,
                    0x4328446fffc4232d,
                    0x00c1e96b4055efe1,
                ])),
                Felt::new(BigInteger384([
                    0xe82a930352c37ea6,
                    0x1bc5b9195b50c9f3,
                    0x0f17abae0764f7a4,
                    0x643ed4f5cffd8379,
                    0x464dcca667cd1200,
                    0x01358261545bf8d1,
                ])),
            ],
        ];

        for i in input.iter_mut() {
            apply_mds(i);
        }
        for i in input2.iter_mut() {
            apply_naive_mds(i);
        }

        for (index, (&i_1, i_2)) in input.iter().zip(input2).enumerate() {
            assert_eq!(output[index], i_1);
            assert_eq!(output[index], i_2);
        }
    }
}
