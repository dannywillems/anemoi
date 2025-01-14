use super::BigInteger256;
use super::Felt;
use super::NUM_HASH_ROUNDS;

/// Additive round constants C for Anemoi.
pub(crate) const C: [Felt; NUM_HASH_ROUNDS] = [
    Felt::new(BigInteger256([
        0xabafffffffff9120,
        0x4bb663a9ffff8cc3,
        0xe91979cf61a56a4a,
        0x0478a6f9e9e436dd,
    ])),
    Felt::new(BigInteger256([
        0xbfdc959fc7b4b4f6,
        0xa1645fd4ff9605ce,
        0xd5691b0520c7782e,
        0x0a14dedd9894572a,
    ])),
    Felt::new(BigInteger256([
        0xa8710aeddc00c31f,
        0x48d555e0b98bbcdc,
        0x90c713f2837ff521,
        0x0a1f01b2d09a9963,
    ])),
    Felt::new(BigInteger256([
        0xa747c810b42443fb,
        0xf6d421a4dfe10249,
        0xc87bd48e64d5d249,
        0x0b6b19608e0494d1,
    ])),
    Felt::new(BigInteger256([
        0x4b84564b4ed4cc40,
        0x614fb3d8ba6fa258,
        0x6d21dc5b9cbc24a9,
        0x0d7804aeee114147,
    ])),
    Felt::new(BigInteger256([
        0x094d0862efc8d4b2,
        0x518ff99b3760e6b7,
        0x14b68a84040da5fe,
        0x0b4b897ead371aa6,
    ])),
    Felt::new(BigInteger256([
        0xb65d03158e8d330a,
        0x062c677ab9fb38fa,
        0x395365290dc341a3,
        0x075e6fec1dbc6a05,
    ])),
    Felt::new(BigInteger256([
        0x405cc568b090bbf5,
        0x90216c2a4a1335e3,
        0xdabc18b784bdda4e,
        0x0c3faca6b6e1fb47,
    ])),
    Felt::new(BigInteger256([
        0x1ce64c4fbff3bf46,
        0x9ef195190fc39b99,
        0x4f7f6c077b257685,
        0x0af1b0422db4e68e,
    ])),
    Felt::new(BigInteger256([
        0xd81c4503f4186b9b,
        0xf3e86355d2c3a343,
        0xa8655929d372f300,
        0x0bf60b0e67b9f46c,
    ])),
    Felt::new(BigInteger256([
        0x22235cc554950665,
        0x363b64395e607ef2,
        0x1f59bbca2a70ec44,
        0x0ae482c5e3408c20,
    ])),
    Felt::new(BigInteger256([
        0x0094ddf87b73c62e,
        0x3b57d5da6184528d,
        0x29c964eae02a0341,
        0x1195c5cb1bb5e57c,
    ])),
    Felt::new(BigInteger256([
        0x3aea95da21aad6b7,
        0xb05d537cda9edf7a,
        0x7c96f97b91ff07f2,
        0x084cd270a8033917,
    ])),
    Felt::new(BigInteger256([
        0xbf6ac3357d5d3ee5,
        0x7ec50f143ce32fac,
        0xec483d394b227566,
        0x04aa7447819c2e65,
    ])),
    Felt::new(BigInteger256([
        0xa894002e33c4357f,
        0x383c0137fefacf63,
        0x24af7606f48aa710,
        0x0097eda5ce18dd64,
    ])),
    Felt::new(BigInteger256([
        0xb2eec9a28e2eba20,
        0xf607d183f5f84668,
        0x56a7d5102c268c50,
        0x010f5901b594d4f1,
    ])),
    Felt::new(BigInteger256([
        0x366fc14bedfaf150,
        0xd3346a63f63e005f,
        0xe4b5fc9ceb75bcec,
        0x0c23180cc5314c76,
    ])),
    Felt::new(BigInteger256([
        0x71c44f0b812c42c5,
        0x55fb5c584f7fa7fd,
        0xbd2ca413a3038998,
        0x0a82de612f0e3d31,
    ])),
];

/// Additive round constants D for Anemoi.
pub(crate) const D: [Felt; NUM_HASH_ROUNDS] = [
    Felt::new(BigInteger256([
        0x631f9745d173ee37,
        0x4a87e61eafff8cc3,
        0xe5ff9352bcdc0bbe,
        0x0b0416f762c0c3f4,
    ])),
    Felt::new(BigInteger256([
        0x87f729b70dd07c39,
        0x834a0803a0820bb8,
        0x317e1fc7d2e63bf4,
        0x0d71b42dcbdf1706,
    ])),
    Felt::new(BigInteger256([
        0xf4c0bcf4a6c3cc60,
        0xd8d96837dbe528c6,
        0x69b35452c2620234,
        0x0badb273ea671d56,
    ])),
    Felt::new(BigInteger256([
        0x900b244fb0904625,
        0xb096a2690fc1e758,
        0x6a6bf60727070908,
        0x041ee3f01f042972,
    ])),
    Felt::new(BigInteger256([
        0x9ac5ecc7cbcdbdce,
        0x1c3798121f107e06,
        0xd370b8d18d209d77,
        0x0ee4a9c71adbe1a8,
    ])),
    Felt::new(BigInteger256([
        0xd43665b6caf64b48,
        0xbbdc285be039533f,
        0x0327decf2d91be11,
        0x1054f5b00a8be98e,
    ])),
    Felt::new(BigInteger256([
        0xcc9541c055d23450,
        0x0b1903f37e920489,
        0xe9f35fa9483e30d9,
        0x123e2e636f1b0391,
    ])),
    Felt::new(BigInteger256([
        0xc2b22d9af007839b,
        0xa6095ae2c11500d7,
        0xea02a77c314478d6,
        0x024c9ce75cfe9049,
    ])),
    Felt::new(BigInteger256([
        0x23ddc3fffe4a2579,
        0xa7fd28b2360523b3,
        0xae0096e00d9ab955,
        0x087cc0ab19d909c3,
    ])),
    Felt::new(BigInteger256([
        0xd3c34969c9626da6,
        0x5f45bdcc3741afc8,
        0x31dd3ecec0046cb3,
        0x0ace99c7ee0eaafa,
    ])),
    Felt::new(BigInteger256([
        0x55d670d1af2e2598,
        0xd4a98a5e3da47bd8,
        0x67dae10f5e058bbc,
        0x05fbf7abbc9d040c,
    ])),
    Felt::new(BigInteger256([
        0xf9306160bb9162fa,
        0x6ff472054edb827e,
        0x5efd875b858c043d,
        0x0bd2af30dd22ec33,
    ])),
    Felt::new(BigInteger256([
        0x56903eb82dcff70b,
        0x8d917e507232a489,
        0xf8c19538899bd53f,
        0x03c695341f2f4d94,
    ])),
    Felt::new(BigInteger256([
        0xbbea813202cbfa14,
        0x78c9d753c2be8f5f,
        0x9f233fd053259001,
        0x0acb100b846b46cc,
    ])),
    Felt::new(BigInteger256([
        0xb008e589a1511380,
        0x5574b1151044d103,
        0x29239469e348a5d4,
        0x09c957cfec948c63,
    ])),
    Felt::new(BigInteger256([
        0x9dde36649c6a8a6d,
        0xf623c0aa32ec2b22,
        0xd51a20f1efd4b082,
        0x08dae778d9298911,
    ])),
    Felt::new(BigInteger256([
        0x1379fa8a75ed676b,
        0xc86da850a859f618,
        0x80bbe64735fe7597,
        0x11ef58ac36c36a21,
    ])),
    Felt::new(BigInteger256([
        0xd83279612d1ba95d,
        0x39489cbfa3975bf2,
        0x55906677b1509f0b,
        0x0c36fa18ade3312a,
    ])),
];
