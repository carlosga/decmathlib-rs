/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

#[test]
fn rem_binop() {
    let a= decmathlib_rs::d128::d128::from(12);
    let b= decmathlib_rs::d128::d128::from(10);
    let e= decmathlib_rs::d128::d128::from(2);

    assert_eq!(a % b  , e);
    assert_eq!(&a % b , e);
    assert_eq!(a % &b , e);
    assert_eq!(&a % &b, e);
}

#[test]
fn rem_assign_op() {
    let mut a1 = decmathlib_rs::d128::d128::from(12);
    let mut a2 = decmathlib_rs::d128::d128::from(12);
    let mut a3 = decmathlib_rs::d128::d128::from(12);
    let mut a4 = decmathlib_rs::d128::d128::from(12);
    let b = decmathlib_rs::d128::d128::from(10);
    let e = decmathlib_rs::d128::d128::from(2);

    a1 %= b;
    assert_eq!(e, a1);
    a2 %= b;
    assert_eq!(e, a2);
    a3 %= b;
    assert_eq!(e, a3);
    a4 %= b;
    assert_eq!(e, a4);
}

dec_test!(bid128_rem_001, bid128_rem, 0, 0x00000080024520145e7ff56d67eb67a5u128, 0x000000c100000020d277330430a0ee20u128, 0x80000040fdbae00c73f73d96c8b5867bu128, 0x00);
dec_test!(bid128_rem_002, bid128_rem, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_003, bid128_rem, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_004, bid128_rem, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_005, bid128_rem, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_006, bid128_rem, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_007, bid128_rem, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_008, bid128_rem, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_009, bid128_rem, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_010, bid128_rem, 0, 0x0065420a01b95380dfae38016de45fb4u128, 0x96b1104022ac3cca72c2b81d8fec995eu128, 0x0065420a01b95380dfae38016de45fb4u128, 0x00);
dec_test!(bid128_rem_011, bid128_rem, 0, "-0"                                  , "-0.E0"                               , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_012, bid128_rem, 0, 0x010009020c4200025ff98fbff95edf7eu128, 0x0104102400200020f7ffffdef5efffffu128, 0x010009020c4200025ff98fbff95edf7eu128, 0x00);
dec_test!(bid128_rem_013, bid128_rem, 0, 0x020000001320124c79a07e3c04b31fb0u128, 0xffffffffffffffff9056a2027e3103c0u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_014, bid128_rem, 0,  "0"                                  , "-Infinity"                           , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_015, bid128_rem, 0, "-0"                                  , "-Infinity"                           , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_016, bid128_rem, 0, "-0"                                  ,      "QNaN"                           , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_017, bid128_rem, 0, "-0"                                  ,      "SNaN"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_018, bid128_rem, 0, 0x2000020002083001af1b166bffda5ddeu128, 0x20000004042240000000000000000000u128, 0x20000001f50a7001af1b166bffda5ddeu128, 0x00);
dec_test!(bid128_rem_019, bid128_rem, 0, 0x2000040000002200f7fffffffd7fffffu128, 0x01000000000000000000000000000001u128, 0x01000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_020, bid128_rem, 0, 0x25765e1113026459e37ff77afe7ffdffu128, 0xfbffdff7bfb7ffa7201a7081325d1c12u128, 0x25765e1113026459e37ff77afe7ffdffu128, 0x00);
dec_test!(bid128_rem_021, bid128_rem, 0, 0x26741c0590811072ffffffff6ef7ffffu128, 0x00000000000000000300040220100840u128, 0x8000000000000000015515ab6c6033c0u128, 0x00);
dec_test!(bid128_rem_022, bid128_rem, 0, 0x397cdc9a254ff56299bc2af7b316f57cu128, 0x268d807293a0dcacafddbd631e831cbbu128, 0xa68c1d3c3991cbb19eee601c8bdcb12cu128, 0x00);
dec_test!(bid128_rem_023, bid128_rem, 0, 0x3a1db514f8af9733fefeffefffffffffu128, 0xc48ac2bddb9295e80110484000234290u128, 0x3a1db514f8af9733fefeffefffffffffu128, 0x00);
dec_test!(bid128_rem_024, bid128_rem, 0, 0x3a46a1388afe6e505418c8e8d3d4bd4bu128, 0x7e0016f259f8f0aaf6426a11cf634d38u128, 0x7c0016f259f8f0aaf6426a11cf634d38u128, 0x01);
dec_test!(bid128_rem_025, bid128_rem, 0, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0xc6626c7f2d14a3a10d507aea3bae04b8u128, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0x00);
dec_test!(bid128_rem_026, bid128_rem, 0, 0x40820002150104001f7dbeffb7fdff7du128, 0xbfffbfff7ffa7fbf240c30b8380ad037u128, 0xbffe0708356252ac4b444f585a0c6d93u128, 0x00);
dec_test!(bid128_rem_027, bid128_rem, 0, 0x4dbc0000000000000000000000000000u128, 0x4d138600f520137d2001e957c4b7cc57u128, 0x4d120000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_028, bid128_rem, 0, 0x55401804998161f16b735f43ecd46f95u128, 0x78000000000000000000000000000000u128, 0x55401804998161f16b735f43ecd46f95u128, 0x00);
dec_test!(bid128_rem_029, bid128_rem, 0, 0x623df7ae457861bf0e300840402b221cu128, 0x632cddb83f415035db7e6ff75558f103u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_030, bid128_rem, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_031, bid128_rem, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_032, bid128_rem, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_033, bid128_rem, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_034, bid128_rem, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_035, bid128_rem, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_036, bid128_rem, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_037, bid128_rem, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_038, bid128_rem, 0, 0x8000000000000000f8ba388cd7bc1c1bu128, 0x8040280400c45401fefd71fd65bbeffdu128, 0x8000000000000000f8ba388cd7bc1c1bu128, 0x00);
dec_test!(bid128_rem_039, bid128_rem, 0, 0x80060803004a1208bde3f7effba3efbdu128, 0x3691feee73b72bd9614cbbcb3aadc4f3u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_040, bid128_rem, 0, 0x97f3ca2bca3dcd34321ba8822721f58fu128, 0x8000000000000000bae106a1402505d6u128, 0x000000000000000025bf9a7041265ffeu128, 0x00);
dec_test!(bid128_rem_041, bid128_rem, 0, 0x9a000000000000000000000000000000u128, 0xbc580000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_042, bid128_rem, 0, 0x9c5f9589d6c5fa51caa6dcb43a12852eu128, 0x00800000000000000001000000000000u128, 0x80800000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_043, bid128_rem, 0, 0xa1a32dc578f676e45bfb68dece45b7d8u128, 0xbc460000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_044, bid128_rem, 0, 0xa7f734b7f7e7a516229ddcc454df042cu128, 0x00000000000000029fc80ebfc7fed6d1u128, 0x800000000000000118eb45cf8f8cc645u128, 0x00);
dec_test!(bid128_rem_045, bid128_rem, 0, 0xa9f80000000000000000000000000000u128, 0x419ecd52cafad77c1c15d4445e2c728bu128, 0xa9f80000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_046, bid128_rem, 0, 0xd75fff7bffffffff084410100a41019du128, 0xfe37efefde7bf7d7c000906283812080u128, 0xfc002fefde7bf7d7c000906283812080u128, 0x01);
dec_test!(bid128_rem_047, bid128_rem, 0, 0xf2c66c7b3c127bf90210014041804890u128, 0x838123dae08520910000000000000008u128, 0x83800000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_048, bid128_rem, 0, 0xf363427e815677401012e9212556b005u128, 0x02000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_049, bid128_rem, 0, 0xf8000000000000000000000000000000u128, 0xdabe0000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_050, bid128_rem, 0, 0xfade065e7bff6fbf805415821d030f92u128, 0xa8c595b58b10f056fdbeaf79d6fbbf9fu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_051, bid128_rem, 0, 0xffae1f7bbbdfac9fa1d2e802c5278780u128, 0xd0434d8a6060e8140302011024001300u128, 0xfc001f7bbbdfac9fa1d2e802c5278780u128, 0x01);
dec_test!(bid128_rem_052, bid128_rem, 0, "-Infinity"                           ,   "-0"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_053, bid128_rem, 0, "-Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_054, bid128_rem, 0,  "Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_055, bid128_rem, 0,      "QNaN"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_056, bid128_rem, 1, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_057, bid128_rem, 1, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_058, bid128_rem, 1, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_059, bid128_rem, 1, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_060, bid128_rem, 1, 0x0065420a01b95380dfae38016de45fb4u128, 0x96b1104022ac3cca72c2b81d8fec995eu128, 0x0065420a01b95380dfae38016de45fb4u128, 0x00);
dec_test!(bid128_rem_061, bid128_rem, 1, "-0"                                  ,     "-0.E0"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_062, bid128_rem, 1,  "0"                                  , "-Infinity"                           , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_063, bid128_rem, 1, "-0"                                  , "-Infinity"                           , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_064, bid128_rem, 1, "-0"                                  ,      "QNaN"                           , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_065, bid128_rem, 1, "-0"                                  ,      "SNaN"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_066, bid128_rem, 1, 0x397cdc9a254ff56299bc2af7b316f57cu128, 0x268d807293a0dcacafddbd631e831cbbu128, 0xa68c1d3c3991cbb19eee601c8bdcb12cu128, 0x00);
dec_test!(bid128_rem_067, bid128_rem, 1, 0x3a46a1388afe6e505418c8e8d3d4bd4bu128, 0x7e0016f259f8f0aaf6426a11cf634d38u128, 0x7c0016f259f8f0aaf6426a11cf634d38u128, 0x01);
dec_test!(bid128_rem_068, bid128_rem, 1, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0xc6626c7f2d14a3a10d507aea3bae04b8u128, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0x00);
dec_test!(bid128_rem_069, bid128_rem, 1, 0x4dbc0000000000000000000000000000u128, 0x4d138600f520137d2001e957c4b7cc57u128, 0x4d120000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_070, bid128_rem, 1, 0x55401804998161f16b735f43ecd46f95u128, 0x78000000000000000000000000000000u128, 0x55401804998161f16b735f43ecd46f95u128, 0x00);
dec_test!(bid128_rem_071, bid128_rem, 1, 0x9a000000000000000000000000000000u128, 0xbc580000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_072, bid128_rem, 1, 0xa1a32dc578f676e45bfb68dece45b7d8u128, 0xbc460000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_073, bid128_rem, 1, 0xa9f80000000000000000000000000000u128, 0x419ecd52cafad77c1c15d4445e2c728bu128, 0xa9f80000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_074, bid128_rem, 1, 0xf8000000000000000000000000000000u128, 0xdabe0000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_075, bid128_rem, 1, "-Infinity"                           ,   "-0"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_076, bid128_rem, 1, "-Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_077, bid128_rem, 1,  "Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_078, bid128_rem, 1,      "QNaN"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_079, bid128_rem, 2, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_080, bid128_rem, 2, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_081, bid128_rem, 2, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_082, bid128_rem, 2, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_083, bid128_rem, 2, 0x0065420a01b95380dfae38016de45fb4u128, 0x96b1104022ac3cca72c2b81d8fec995eu128, 0x0065420a01b95380dfae38016de45fb4u128, 0x00);
dec_test!(bid128_rem_084, bid128_rem, 2, "-0"                                  ,     "-0.E0"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_085, bid128_rem, 2,  "0"                                  , "-Infinity"                           , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_086, bid128_rem, 2, "-0"                                  , "-Infinity"                           , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_087, bid128_rem, 2, "-0"                                  ,      "QNaN"                           , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_088, bid128_rem, 2, "-0"                                  ,      "SNaN"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_089, bid128_rem, 2, 0x397cdc9a254ff56299bc2af7b316f57cu128, 0x268d807293a0dcacafddbd631e831cbbu128, 0xa68c1d3c3991cbb19eee601c8bdcb12cu128, 0x00);
dec_test!(bid128_rem_090, bid128_rem, 2, 0x3a46a1388afe6e505418c8e8d3d4bd4bu128, 0x7e0016f259f8f0aaf6426a11cf634d38u128, 0x7c0016f259f8f0aaf6426a11cf634d38u128, 0x01);
dec_test!(bid128_rem_091, bid128_rem, 2, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0xc6626c7f2d14a3a10d507aea3bae04b8u128, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0x00);
dec_test!(bid128_rem_092, bid128_rem, 2, 0x4dbc0000000000000000000000000000u128, 0x4d138600f520137d2001e957c4b7cc57u128, 0x4d120000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_093, bid128_rem, 2, 0x55401804998161f16b735f43ecd46f95u128, 0x78000000000000000000000000000000u128, 0x55401804998161f16b735f43ecd46f95u128, 0x00);
dec_test!(bid128_rem_094, bid128_rem, 2, 0x9a000000000000000000000000000000u128, 0xbc580000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_095, bid128_rem, 2, 0xa1a32dc578f676e45bfb68dece45b7d8u128, 0xbc460000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_096, bid128_rem, 2, 0xa9f80000000000000000000000000000u128, 0x419ecd52cafad77c1c15d4445e2c728bu128, 0xa9f80000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_097, bid128_rem, 2, 0xf8000000000000000000000000000000u128, 0xdabe0000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_098, bid128_rem, 2, "-Infinity"                           ,   "-0"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_099, bid128_rem, 2, "-Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_100, bid128_rem, 2,  "Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_101, bid128_rem, 2,      "QNaN"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_102, bid128_rem, 3, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_103, bid128_rem, 3, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_104, bid128_rem, 3, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_105, bid128_rem, 3, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_106, bid128_rem, 3, 0x0065420a01b95380dfae38016de45fb4u128, 0x96b1104022ac3cca72c2b81d8fec995eu128, 0x0065420a01b95380dfae38016de45fb4u128, 0x00);
dec_test!(bid128_rem_107, bid128_rem, 3, "-0"                                  ,     "-0.E0"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_108, bid128_rem, 3,  "0"                                  , "-Infinity"                           , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_109, bid128_rem, 3, "-0"                                  , "-Infinity"                           , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_110, bid128_rem, 3, "-0"                                  ,      "QNaN"                           , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_111, bid128_rem, 3, "-0"                                  ,      "SNaN"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_112, bid128_rem, 3, 0x397cdc9a254ff56299bc2af7b316f57cu128, 0x268d807293a0dcacafddbd631e831cbbu128, 0xa68c1d3c3991cbb19eee601c8bdcb12cu128, 0x00);
dec_test!(bid128_rem_113, bid128_rem, 3, 0x3a46a1388afe6e505418c8e8d3d4bd4bu128, 0x7e0016f259f8f0aaf6426a11cf634d38u128, 0x7c0016f259f8f0aaf6426a11cf634d38u128, 0x01);
dec_test!(bid128_rem_114, bid128_rem, 3, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0xc6626c7f2d14a3a10d507aea3bae04b8u128, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0x00);
dec_test!(bid128_rem_115, bid128_rem, 3, 0x4dbc0000000000000000000000000000u128, 0x4d138600f520137d2001e957c4b7cc57u128, 0x4d120000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_116, bid128_rem, 3, 0x55401804998161f16b735f43ecd46f95u128, 0x78000000000000000000000000000000u128, 0x55401804998161f16b735f43ecd46f95u128, 0x00);
dec_test!(bid128_rem_117, bid128_rem, 3, 0x9a000000000000000000000000000000u128, 0xbc580000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_118, bid128_rem, 3, 0xa1a32dc578f676e45bfb68dece45b7d8u128, 0xbc460000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_119, bid128_rem, 3, 0xa9f80000000000000000000000000000u128, 0x419ecd52cafad77c1c15d4445e2c728bu128, 0xa9f80000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_120, bid128_rem, 3, 0xf8000000000000000000000000000000u128, 0xdabe0000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_121, bid128_rem, 3, "-Infinity"                           ,   "-0"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_122, bid128_rem, 3, "-Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_123, bid128_rem, 3,  "Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_124, bid128_rem, 3,      "QNaN"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_125, bid128_rem, 4, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_126, bid128_rem, 4, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_127, bid128_rem, 4, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_128, bid128_rem, 4, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_129, bid128_rem, 4, 0x0065420a01b95380dfae38016de45fb4u128, 0x96b1104022ac3cca72c2b81d8fec995eu128, 0x0065420a01b95380dfae38016de45fb4u128, 0x00);
dec_test!(bid128_rem_130, bid128_rem, 4, "-0"                                  ,     "-0.E0"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_131, bid128_rem, 4,  "0"                                  , "-Infinity"                           , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_132, bid128_rem, 4, "-0"                                  , "-Infinity"                           , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_133, bid128_rem, 4, "-0"                                  ,      "QNaN"                           , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_134, bid128_rem, 4, "-0"                                  ,      "SNaN"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_135, bid128_rem, 4, 0x397cdc9a254ff56299bc2af7b316f57cu128, 0x268d807293a0dcacafddbd631e831cbbu128, 0xa68c1d3c3991cbb19eee601c8bdcb12cu128, 0x00);
dec_test!(bid128_rem_136, bid128_rem, 4, 0x3a46a1388afe6e505418c8e8d3d4bd4bu128, 0x7e0016f259f8f0aaf6426a11cf634d38u128, 0x7c0016f259f8f0aaf6426a11cf634d38u128, 0x01);
dec_test!(bid128_rem_137, bid128_rem, 4, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0xc6626c7f2d14a3a10d507aea3bae04b8u128, 0x3aaf3f4db47ae4f23b2705d283e3d160u128, 0x00);
dec_test!(bid128_rem_138, bid128_rem, 4, 0x4dbc0000000000000000000000000000u128, 0x4d138600f520137d2001e957c4b7cc57u128, 0x4d120000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_139, bid128_rem, 4, 0x55401804998161f16b735f43ecd46f95u128, 0x78000000000000000000000000000000u128, 0x55401804998161f16b735f43ecd46f95u128, 0x00);
dec_test!(bid128_rem_140, bid128_rem, 4, 0x9a000000000000000000000000000000u128, 0xbc580000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_141, bid128_rem, 4, 0xa1a32dc578f676e45bfb68dece45b7d8u128, 0xbc460000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_142, bid128_rem, 4, 0xa9f80000000000000000000000000000u128, 0x419ecd52cafad77c1c15d4445e2c728bu128, 0xa9f80000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_143, bid128_rem, 4, 0xf8000000000000000000000000000000u128, 0xdabe0000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_144, bid128_rem, 4, "-Infinity"                           ,   "-0"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_rem_145, bid128_rem, 4, "-Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_146, bid128_rem, 4,  "Infinity"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_rem_147, bid128_rem, 4,      "QNaN"                           , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);

dec_test!(bid128_rem_148, bid128_rem, 0, "99"                                  , "66"                                  , "-33"                                 , 0x00);