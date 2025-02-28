use super::BigInteger256;
use super::Felt;
use super::{NUM_COLUMNS, NUM_HASH_ROUNDS};

/// Additive round constants C for Anemoi.
pub(crate) const C: [[Felt; NUM_COLUMNS]; NUM_HASH_ROUNDS] = [
    [
        Felt::new(BigInteger256([
            0x7346fc0cffffff6d,
            0x517627467fa308c8,
            0xffffffffffffffec,
            0x3fffffffffffffff,
        ])),
        Felt::new(BigInteger256([
            0xdd1178d05cddbbb9,
            0xbd8c24862fe7c4d1,
            0x04dfdb230c1bf060,
            0x3659b14ca19b3891,
        ])),
        Felt::new(BigInteger256([
            0x08477b06d8a2814b,
            0x473de0d00ab9801a,
            0x628e228a698d0be1,
            0x3abeee38a9c048d1,
        ])),
        Felt::new(BigInteger256([
            0x23607714739cdab8,
            0x12c98127fec5f1e2,
            0xa691d3cb21d425ec,
            0x04cf9d6d056792bd,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xba0d6b9b3fa3633d,
            0xf99eef39c529d723,
            0x7af28749f7395589,
            0x02ef61c6b77a510f,
        ])),
        Felt::new(BigInteger256([
            0xe0b3151465440acd,
            0x3b48888c6c5565be,
            0x5f2d2585f096804e,
            0x291429a9ea04cb3f,
        ])),
        Felt::new(BigInteger256([
            0xdfe8c6ea5f55ea7b,
            0xd305adf43f0860fc,
            0x6ca9d07a537a21e8,
            0x3a611982a6a07bc8,
        ])),
        Felt::new(BigInteger256([
            0xe1b9593b9828cbf6,
            0x3987dfc03cba481f,
            0xae1d501336053ee7,
            0x3a79a431f211dca0,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x698d33f778f7c4fa,
            0x870adf3fd9e4ec90,
            0xa084add0a525fa2d,
            0x1ac8048f2e389485,
        ])),
        Felt::new(BigInteger256([
            0x6056828de708b943,
            0x825dbbe3f5424de2,
            0xf6baed51e54a76a3,
            0x19f87a536cf65d5a,
        ])),
        Felt::new(BigInteger256([
            0xb15f4ad816487620,
            0xad0a9482b9bdd407,
            0x43cb8fb2a0f11766,
            0x185580e3f6a76e43,
        ])),
        Felt::new(BigInteger256([
            0x9c74a919e5a15860,
            0xfe8983de372f0bfe,
            0xf0cbd366ee541870,
            0x19119ff99857ff88,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x79b834feef2c3839,
            0xaae19cadc9407e29,
            0x66d36f5b19289453,
            0x34440a184a180800,
        ])),
        Felt::new(BigInteger256([
            0xdcfec7f06a770db9,
            0x19fc3b99f8731eb8,
            0x573b5ec5a02fb0a5,
            0x3211ce60c0dde888,
        ])),
        Felt::new(BigInteger256([
            0xca5354698da00c54,
            0x3c6df6bc29314d7b,
            0x5e7c064337d6039a,
            0x083ab13af660d29a,
        ])),
        Felt::new(BigInteger256([
            0xbd193394681e5285,
            0xdb1d5a1af1327b9b,
            0x6646a52e7527ca4b,
            0x1319b15bc6053aa2,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x8f9d974c0fddcdaa,
            0x451b85da76eaa3bb,
            0x4f05589eb4cd7c68,
            0x14e18498bd951684,
        ])),
        Felt::new(BigInteger256([
            0xcfae98be18a0c4e7,
            0x4fed38d71eb4bfff,
            0xf0725685381c83aa,
            0x092a2f60cb5606b4,
        ])),
        Felt::new(BigInteger256([
            0x9d59fa0777b886ee,
            0x28aad522a9cc9907,
            0x584602e05ad7d57f,
            0x2276384a64f2527d,
        ])),
        Felt::new(BigInteger256([
            0xc42f61616b7e3c00,
            0x94c1676f01076e0f,
            0xed213d73747dc0f5,
            0x318e45fa0e3e1ef8,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x32e5e8b5e721710e,
            0x783d8096d132985c,
            0x428ca9de5e3fe006,
            0x384cfb58bd05691c,
        ])),
        Felt::new(BigInteger256([
            0x1d55ea153e3948c1,
            0x402e3d750f834353,
            0x0760b8cf40c32b3c,
            0x29d57ef2fd77eb59,
        ])),
        Felt::new(BigInteger256([
            0x502e59fb047ba362,
            0xcc832d20b6e22557,
            0x557e622c2557482a,
            0x3547b8a64d6d1c5d,
        ])),
        Felt::new(BigInteger256([
            0x73104ece449bafbb,
            0x0f9dcc56c400015d,
            0x2c3a468a2ebd1850,
            0x25d8547526d6c295,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x271faad71772fc51,
            0xe95b52fe5732e618,
            0xd0504b779f895c73,
            0x3cf7dffba61d8a86,
        ])),
        Felt::new(BigInteger256([
            0xf8159070f5824aa5,
            0xcb8739dac5ec94dc,
            0x426cf3f16f092a30,
            0x28c71fceb13f06a1,
        ])),
        Felt::new(BigInteger256([
            0x0c959cbcf8fe7a10,
            0xc31ae0efc8f4327d,
            0x912c2244eaf20964,
            0x040c708a594ba02a,
        ])),
        Felt::new(BigInteger256([
            0xae3371cf16e439dc,
            0x67fe57286f9c7ab0,
            0xafe5b1547964eae5,
            0x1de08548ca77ed11,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xebfc726afe288f17,
            0x105c9600c55d5461,
            0x6e45121d3212bccb,
            0x277db81baa443223,
        ])),
        Felt::new(BigInteger256([
            0x79c027999f48d390,
            0xabc4f9e711570b5d,
            0xd9b640bcb4a40845,
            0x3abd153ea3d7be1e,
        ])),
        Felt::new(BigInteger256([
            0x7b3f589edf574f4b,
            0xb3db1cccb8ce3b60,
            0xbdabe4d72a5089e8,
            0x079dfcf2355af322,
        ])),
        Felt::new(BigInteger256([
            0x4fafdeeb648773b8,
            0x22b5bda5842fa68c,
            0x15a3a74e5ba7c50f,
            0x264ce4200414cc12,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xca19b06b4e6df2a3,
            0xcce22050a4ad7858,
            0xc644747d2cae1918,
            0x177eaf84e1d0f4c9,
        ])),
        Felt::new(BigInteger256([
            0xb89f7e263bff609c,
            0x48305c88fa7bbe57,
            0xee816831d64ba437,
            0x337af74a01eda4c5,
        ])),
        Felt::new(BigInteger256([
            0x776ffe23eb8bc53d,
            0xa2f7c286455fcb56,
            0xf67e7b9b52331114,
            0x0b4748bf395dc210,
        ])),
        Felt::new(BigInteger256([
            0x3ce2f98fc7ce9df0,
            0x0d7e5210d2952d99,
            0x4990898542209aa1,
            0x1f12112863eea40d,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x4cd177b08d9633fb,
            0x77467f1fb1ef8f62,
            0x4bf0ddb4360a9750,
            0x27a64389729f63b6,
        ])),
        Felt::new(BigInteger256([
            0xe49efb7d804e4de7,
            0xd6886c7479f7090a,
            0x612144415c31d7b3,
            0x3f53c94988bb2f1e,
        ])),
        Felt::new(BigInteger256([
            0xbbb6a176fcba2dfc,
            0xa58747258ad709f4,
            0x71a860fbfcb9a8e4,
            0x3d7794dc86344b8d,
        ])),
        Felt::new(BigInteger256([
            0x494aa051a0b60431,
            0x860dde909de7a60d,
            0xcde7a917c840b21b,
            0x3fc298ff14536c5c,
        ])),
    ],
];

/// Additive round constants D for Anemoi.
pub(crate) const D: [[Felt; NUM_COLUMNS]; NUM_HASH_ROUNDS] = [
    [
        Felt::new(BigInteger256([
            0xf93bea3e99999905,
            0xe76b98e6704ec765,
            0xccccccccccccccb8,
            0x0ccccccccccccccc,
        ])),
        Felt::new(BigInteger256([
            0x54b77d06498c2839,
            0x615c40f05b9d31ea,
            0x2c76c6a290adc30e,
            0x2421f328b24eb6ab,
        ])),
        Felt::new(BigInteger256([
            0x15af6ff7865a0ac7,
            0xe37b9d7f555c9ce7,
            0xb52040fcd4f7b7c6,
            0x276763a03cea022b,
        ])),
        Felt::new(BigInteger256([
            0x9ad7d259366e01bc,
            0x7655fd4aa538db09,
            0x908737b36e6afe7b,
            0x2c25faaa472b3260,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x2b5942a60725fb0e,
            0x82fa22a8633a158e,
            0x7222c0fd00a61c88,
            0x3d0f46fc36cb6075,
        ])),
        Felt::new(BigInteger256([
            0x2b222be17fdb7584,
            0x8df134cd324600e9,
            0xb1277debb1c84d2d,
            0x042f83eead3c8bf2,
        ])),
        Felt::new(BigInteger256([
            0xc019ce723af6722e,
            0x1e1bfa7a23e6abdb,
            0xe99f5bd2fb84c800,
            0x145ca752ec4e77bb,
        ])),
        Felt::new(BigInteger256([
            0x9fb2dbf688e2f130,
            0x29a652bd73d3b67b,
            0xc27620e1bf3c11a8,
            0x0f2319d7e659bedc,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xe978ee1860aee0be,
            0x3109d849e842304e,
            0x9e6a224077aac46f,
            0x2ad600d144e2d50c,
        ])),
        Felt::new(BigInteger256([
            0x45ac679221d4a7ee,
            0x17f0c6bc3514973e,
            0x4f6a80746f9446c6,
            0x0b01eba4c7874f2f,
        ])),
        Felt::new(BigInteger256([
            0x2c772097121d81c7,
            0x3b0b3fa0187dcd18,
            0xc77655c81213c0c1,
            0x083f25c0d3ae9b57,
        ])),
        Felt::new(BigInteger256([
            0xf554fa0bf690018e,
            0x31925572e82a288b,
            0x0bd9def240a2ee75,
            0x03a92cac23f912e6,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xb57e8ab7feaec69e,
            0x2f7a9fdbb5f3e6e1,
            0xf227f11a3b85b6a4,
            0x10e8ae3876dd9e99,
        ])),
        Felt::new(BigInteger256([
            0x0a7633adcd0e6f06,
            0xac6fe992203035ec,
            0x3d59ff377a51d8d6,
            0x2fb1e790318a306f,
        ])),
        Felt::new(BigInteger256([
            0x8d8cb0e1b1408a9d,
            0xc74f44f96fdc1463,
            0x6f95d9a7f8d10503,
            0x04bafdf5e98355c1,
        ])),
        Felt::new(BigInteger256([
            0x5e1b0b3fa0d86e55,
            0x0b06cecf8a186600,
            0x0ec3be09174ef85f,
            0x0a47e5ec67c1a412,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x828b305803f5fd96,
            0x862a411d717cf719,
            0x0f9c5b2bcc9d5b90,
            0x356fb258c8b075f6,
        ])),
        Felt::new(BigInteger256([
            0x28065cad5fcdc7ba,
            0x7c9005e84abc18fb,
            0x0bd377c507b168b3,
            0x0ab3d2301a581774,
        ])),
        Felt::new(BigInteger256([
            0x8b73aeb17feea6bd,
            0x4dbb4278f4c1a1b7,
            0x9ea25713114593c0,
            0x22e00ea5366a9e7c,
        ])),
        Felt::new(BigInteger256([
            0x9011913e88cdf956,
            0x5ed9fb3c9e379a3c,
            0xcae0d71c0c17abe0,
            0x2ca6042a8e505140,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x6ce057804b0a17ad,
            0x916a08e0fd6a7e29,
            0x041f00529db459c9,
            0x2a8b216cdae77b2e,
        ])),
        Felt::new(BigInteger256([
            0x49016ee3f536c248,
            0x6735708976c4d79b,
            0x23bd2df637fcaae0,
            0x3d0f1a165f40aeb8,
        ])),
        Felt::new(BigInteger256([
            0x8554e4637c8239e4,
            0xc9b1677e337cc076,
            0x9cd60a460369a106,
            0x0761875531ac1afc,
        ])),
        Felt::new(BigInteger256([
            0x12463f8ad1bbe3c5,
            0xd41ac6279c6a68d7,
            0x0af53419edfb9dd5,
            0x32a00af9b9afa77d,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xfc82d052d6df88cf,
            0xe94aae1c2867a290,
            0xef69272bbe5d7e80,
            0x05a2f82d7ef1affb,
        ])),
        Felt::new(BigInteger256([
            0xbf29cbf10803aa0b,
            0xd9513fc2d22affd0,
            0xbc4fee5845a2521e,
            0x126dad0fcdf9dd63,
        ])),
        Felt::new(BigInteger256([
            0xf5b2b418cc88f673,
            0xeb992018fdb4f602,
            0x360a4f9ea8640a8a,
            0x2c933156f87cb22d,
        ])),
        Felt::new(BigInteger256([
            0xe8d2193cff8853c5,
            0x133e23cced03b8d5,
            0xec272424180318b5,
            0x01152deb1842e55c,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xc848163e0d44e6d8,
            0xe3d056079f9b7269,
            0x9803ca32145c41bf,
            0x053c63e6cdf2161b,
        ])),
        Felt::new(BigInteger256([
            0x47bce1710179fe39,
            0x8d1364b8269ed7e0,
            0x5e3f17844eb2931b,
            0x397736190b6c5364,
        ])),
        Felt::new(BigInteger256([
            0xdefe0331029196f0,
            0x8d9727e2ed03b797,
            0x6d2fee91ab37edf6,
            0x053851581f65c3a8,
        ])),
        Felt::new(BigInteger256([
            0x913704b09cdb58e4,
            0xa179ef330aa04640,
            0x5c8af67ebdbb55c6,
            0x1e95205b9cb982e0,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x105609aeb533f374,
            0x2bf1e08032a32853,
            0x4a62c0966a99a8eb,
            0x142f1db7015fc17b,
        ])),
        Felt::new(BigInteger256([
            0x6446024cf5da3454,
            0x92d42e86b9e673ef,
            0xcd69d2fdcbfc39ea,
            0x1126da8b656322c4,
        ])),
        Felt::new(BigInteger256([
            0x451f5e26666fb5f2,
            0x084fcdc52d4cd980,
            0x0062195a2ebc8000,
            0x27d35f8c1f497b50,
        ])),
        Felt::new(BigInteger256([
            0xe85ad4c557cc2c2c,
            0x17de83c70cbd5f3f,
            0xead76cb9ffd63636,
            0x364c0fcaf8744394,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x05b85a24ac81294a,
            0xb4ce7d0a14651042,
            0xa2fa7a5c6a57a856,
            0x312eda050b089406,
        ])),
        Felt::new(BigInteger256([
            0x02f008d4f24e161d,
            0xffa47c2d0de18f88,
            0x12f4ff9c4843ee9a,
            0x29d7d4d4650b10bc,
        ])),
        Felt::new(BigInteger256([
            0x6fc99f892fc3132e,
            0xc710f7233daf4026,
            0x4e774f49cfa49903,
            0x26dbd3f2e4fa686b,
        ])),
        Felt::new(BigInteger256([
            0xdb261996e8d886ea,
            0x4c9fb505a2faffbb,
            0x4219dcdb7c57cee4,
            0x23d4bfeb21b36f83,
        ])),
    ],
];
