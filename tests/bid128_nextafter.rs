/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_nextafter_001, bid128_nextafter, 0x00000000000000000000002000000808u128     , 0xad7fd6f7effffdff100084200030000cu128, 0x00000000000000000000002000000807u128, 0x30);
dec_test!(bid128_nextafter_002, bid128_nextafter, 0x00000000000000000000010000000010u128     , 0x0201002004010102ffbffff7ffffffffu128, 0x00000000000000000000010000000011u128, 0x30);
dec_test!(bid128_nextafter_003, bid128_nextafter, 0x00000000000000000408040200802000u128     , 0x767be716da5ae6dd232316158bcd0fc7u128, 0x00000000000000000408040200801fffu128, 0x30);
dec_test!(bid128_nextafter_004, bid128_nextafter, 0x00000000000000007db397f74ae4514bu128     , 0x022088102528100cffffe9f8fbfddafeu128, 0x00000000000000007db397f74ae4514cu128, 0x30);
dec_test!(bid128_nextafter_005, bid128_nextafter, 0x00000044000000005c141b321f5c8224u128     , 0xe7fffffffeff8f7fcdd3816eee5fe6a9u128, 0x00000044000000005c141b321f5c8223u128, 0x30);
dec_test!(bid128_nextafter_006, bid128_nextafter, 0x000001830229a9846d8cb4a924c10429u128     , 0x0082a00400000200120a819472a52843u128, 0x000001830229a9846d8cb4a924c1042au128, 0x30);
dec_test!(bid128_nextafter_007, bid128_nextafter, 0x00000600902304417be5a93bfb781df5u128     , 0x9699586f987927350202040002000800u128, 0x00000600902304417be5a93bfb781df4u128, 0x30);
dec_test!(bid128_nextafter_008, bid128_nextafter, 0x0001ed09bead87c0378d8e62ffffffffu128     , 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_nextafter_009, bid128_nextafter, 0x0001ed09bead87c0378d8e62ffffffffu128     , 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62fffffffeu128, 0x00);
dec_test!(bid128_nextafter_010, bid128_nextafter, 0x0001ed09bead87c0378d8e62ffffffffu128     , 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_011, bid128_nextafter, 0x0001ed09bead87c0378d8e62ffffffffu128     , 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_012, bid128_nextafter, 0x0001ed09bead87c0378d8e64ffffffffu128     , 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nextafter_013, bid128_nextafter, 0x0001ed09bead87c0378d8e64ffffffffu128     , 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_014, bid128_nextafter, 0x0001ed09bead87c0378d8e64ffffffffu128     , 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_015, bid128_nextafter, 0x0001ed09bead87c0378d8e64ffffffffu128     , 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_016, bid128_nextafter, 0x03846610d0133b880000000000000000u128     , 0x95015f79939ac0e97840f3e565ac12d8u128, 0x03846610d0133b87ffffffffffffffffu128, 0x00);
dec_test!(bid128_nextafter_017, bid128_nextafter, 0x04000000000000002efeffdfbf1ffff9u128     , 0x6bebf9febbbfb7ff264c223ead69309au128, 0x03e2a6f6b24f36696f1553d97e927fffu128, 0x00);
dec_test!(bid128_nextafter_018, bid128_nextafter, 0x05020200888c010290445d5a4b0d334fu128     , 0xfffbff7fffffffffd7bfcfb1ffddf4d2u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextafter_019, bid128_nextafter, 0x0651080801591400ffffffffffffffffu128     , 0x1a405089930440d01180804220010020u128, 0x06510808015914010000000000000000u128, 0x00);
dec_test!(bid128_nextafter_020, bid128_nextafter, 0x08200000000000000000000001040000u128     , 0x4000000800000000fdffffff7fff7fffu128, 0x07ec5402b2283b2d062f900000000001u128, 0x00);
dec_test!(bid128_nextafter_021, bid128_nextafter, 0x0820000000000000000810018010058cu128     , 0x088c84a0000a0202ff7ffffffbffbffeu128, 0x07fc6fe3d5da65fc20c5e2c47eb00001u128, 0x00);
dec_test!(bid128_nextafter_022, bid128_nextafter, "-0"                                       , "-Infinity"                           , 0x80000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nextafter_023, bid128_nextafter, "-0"                                       , "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextafter_024, bid128_nextafter, 0x10200000000000000000000820101002u128     , 0xbffff7dfffb7fbf9edf9f7ffb3df3fedu128, 0x0ff2ac0f0abb2d85ab23d9fdecffffffu128, 0x00);
dec_test!(bid128_nextafter_025, bid128_nextafter, 0x165a0000000000000000000000000000u128     , 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextafter_026, bid128_nextafter, 0x16929b5f6e2d6e7ab35d2980d4c5c3e1u128     , 0x2528d9f9899dec722aecf98f2c6039a8u128, 0x16929b5f6e2d6e7ab35d2980d4c5c3e2u128, 0x00);
dec_test!(bid128_nextafter_027, bid128_nextafter, 0x18e7a9dd1e444c27a76671749d6830ddu128     , 0xa40c32187719e6b00103000460000008u128, 0x18e7a9dd1e444c27a76671749d6830dcu128, 0x00);
dec_test!(bid128_nextafter_028, bid128_nextafter, 0x1be8ce41eb48ec8ef1569627903b1345u128     , 0x51940000000000000000000000000000u128, 0x1be8ce41eb48ec8ef1569627903b1344u128, 0x00);
dec_test!(bid128_nextafter_029, bid128_nextafter, 0x202e4dde3350ef5ec227a873fe11ac69u128     , 0x78000000000000000000000000000000u128, 0x202e4dde3350ef5ec227a873fe11ac6au128, 0x00);
dec_test!(bid128_nextafter_030, bid128_nextafter, 0x2c7687422d4941341b69c409e5b18ee2u128     , 0xb15873f09ad2e2d0b056e763f9b15b3bu128, 0x2c7687422d4941341b69c409e5b18ee1u128, 0x00);
dec_test!(bid128_nextafter_031, bid128_nextafter, 0x451400a28b80000424f0beae56d23822u128     , 0x46a08f332bd3b5a6c8210f3889c3ddf0u128, 0x45103f7e7e00019e6e0a7c19ea1ded49u128, 0x00);
dec_test!(bid128_nextafter_032, bid128_nextafter, 0x7a43ce77edc76c4acf5995eb811f0c7cu128     , 0x200500048011a0940000002101101200u128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nextafter_033, bid128_nextafter, 0x7c003fffffffffff38c15b08ffffffffu128     , 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_034, bid128_nextafter, 0x7c003fffffffffff38c15b08ffffffffu128     , 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_035, bid128_nextafter, 0x7c003fffffffffff38c15b08ffffffffu128     , 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_036, bid128_nextafter, 0x7c003fffffffffff38c15b08ffffffffu128     , 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_037, bid128_nextafter, 0x7c003fffffffffff38c15b0affffffffu128     , 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_038, bid128_nextafter, 0x7c003fffffffffff38c15b0affffffffu128     , 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_039, bid128_nextafter, 0x7c003fffffffffff38c15b0affffffffu128     , 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_040, bid128_nextafter, 0x7c003fffffffffff38c15b0affffffffu128     , 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_041, bid128_nextafter, 0x7d73324ba7daf65f02c0800210400104u128     , 0x7fef66deffefdbbd0000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextafter_042, bid128_nextafter, 0x7e0007973e51357844b6a249f6ffd9afu128     , 0xd56e0000000000000000000000000000u128, 0x7c0007973e51357844b6a249f6ffd9afu128, 0x01);
dec_test!(bid128_nextafter_043, bid128_nextafter, 0x8000000000000000f7bcffeccfef7f4fu128     , 0x400065a1202064010000000000000000u128, 0x8000000000000000f7bcffeccfef7f4eu128, 0x30);
dec_test!(bid128_nextafter_044, bid128_nextafter, 0x849eb4ba5e57fb87ffffffffffffffffu128     , 0x9946928b75337654d9fcfdb1ff7fb3dcu128, 0x849eb4ba5e57fb880000000000000000u128, 0x00);
dec_test!(bid128_nextafter_045, bid128_nextafter, 0x88159de31f14d3536fbb15ebd60f4eb4u128     , 0xc4768c8469052e87e0b3b80c70bb2521u128, 0x88159de31f14d3536fbb15ebd60f4eb5u128, 0x00);
dec_test!(bid128_nextafter_046, bid128_nextafter, 0x8df60000000000000000000000000000u128     , 0x369a428628df8e7fe6854dc665fa6d6fu128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nextafter_047, bid128_nextafter, "9.999999999999999999999999999999999E+6144", "Infinity"                            , 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_nextafter_048, bid128_nextafter, 0x9d5f322b9686a8bdf68967ee00da47fdu128     , 0x17901c074773d313636af087d2ca8657u128, 0x9d5f322b9686a8bdf68967ee00da47fcu128, 0x00);
dec_test!(bid128_nextafter_049, bid128_nextafter, 0xab62e57117ac3f3e6b7a3f0e6e382743u128     , 0x5367a609f81ca1aac0998c8d62d888c6u128, 0xab62e57117ac3f3e6b7a3f0e6e382742u128, 0x00);
dec_test!(bid128_nextafter_050, bid128_nextafter, 0xb96a2793b2f392a4a57f74cfa9644f6eu128     , 0xbf6fdf87ce84bb0effffffffffffffffu128, 0xb9698bc4fd83ba6e76fa901c9deb1a4du128, 0x00);
dec_test!(bid128_nextafter_051, bid128_nextafter, 0xd1ebb9a6abc8f2350000000000000000u128     , 0xc65cb01a9af65f926a653098910d0f6eu128, 0xd1ebb9a6abc8f234ffffffffffffffffu128, 0x00);
dec_test!(bid128_nextafter_052, bid128_nextafter, 0xdfefffffffdffbf70000300008001048u128     , 0xce3dccfe2859a72390a735538b915444u128, 0x80000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nextafter_053, bid128_nextafter, 0xe11f457c6ae770aa9ef197d725f76b8du128     , 0xfdffbaffb3fbff9f0000000000000000u128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_054, bid128_nextafter, 0xe5404a0c1322810f221020a000440008u128     , 0xe62f6ecbc57bd337fffffffffffffffbu128, 0x95000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_055, bid128_nextafter, 0xf7fffbdfff7ffb5bb3f50fac6fc293bau128     , 0x4b093058ccfc6465b2fb54bad4fb5760u128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nextafter_056, bid128_nextafter, 0xfbffdfffffffffff71312e0cf7bed879u128     , 0x0b550f8c8c8966ea64ef0861c613e7ceu128, 0xdfffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nextafter_057, bid128_nextafter, 0xfccbb32bcf4c1805807bac54728bb584u128     , 0x8c2760e35110f500a7c8771ef7efefbfu128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_058, bid128_nextafter, 0xffffffffdfef7f7bfaf5bfbfffff2ffeu128     , 0xf95f67aee169437b06f49dc0e46f1c46u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextafter_059, bid128_nextafter, "-Infinity"                                , 0                                     , 0xdfffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nextafter_060, bid128_nextafter, "-Infinity"                                , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_061, bid128_nextafter, "Infinity"                                 , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_062, bid128_nextafter, "QNaN"                                     , "Infinity"                            , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_063, bid128_nextafter, "SNaN"                                     , "0"                                   , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextafter_064, bid128_nextafter, "SNaN"                                     , "-Infinity"                           , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextafter_065, bid128_nextafter, 1                                          , 0x7c00314dc6448d9338c15b0a00000001u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_066, bid128_nextafter, 0x7c00314dc6448d9338c15b0a00000001u128     , 1                                     , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_067, bid128_nextafter, 0x7c00314dc6448d9338c15b0a00000001u128     , 0x7c00314dc6448d9338c15b0a00000001u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_068, bid128_nextafter, 0x7c00314dc6448d9338c15b09ffffffffu128     , 0x7c00314dc6448d9338c15b0a00000001u128, 0x7c00314dc6448d9338c15b09ffffffffu128, 0x00);
dec_test!(bid128_nextafter_069, bid128_nextafter, 0x7c00314dc6448d9338c15b0a00000001u128     , 0x7c00314dc6448d9338c15b09ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_070, bid128_nextafter, 0x80000000000000000000000000000000u128     , 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextafter_071, bid128_nextafter, 0x00000000000000000000000000000000u128     , 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000000u128, 0x00);
