/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_round_integral_zero_001, bid128_round_integral_zero, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_002, bid128_round_integral_zero, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_003, bid128_round_integral_zero, 0, 0x00fe0000000000000000000000000000u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_004, bid128_round_integral_zero, 0, 0x2492beeb9d818053fadd8970151c961au128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_005, bid128_round_integral_zero, 0, 0x27a38809decba00ccc0598806ece1d9au128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_006, bid128_round_integral_zero, 0, 0x2dc3a5281c1849a24f461b5ebca8dd1eu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_007, bid128_round_integral_zero, 0, 0x300000060168010c9dffdc62bffcfdffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_008, bid128_round_integral_zero, 0, 0x303000003002000005c8c405674035c0u128, 0x30400000000000080deb7b0eb5c64be5u128, 0x00);
dec_test!(bid128_round_integral_zero_009, bid128_round_integral_zero, 0, 0x3085ED09BEAD87C0378D8E63ffffffffu128, 0x3085ed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_zero_010, bid128_round_integral_zero, 0, 0x3085ED09BEAD87C0378D8E6400000000u128, 0x30840000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_011, bid128_round_integral_zero, 0, 0x3bce6aaf63364afd867250e924da783cu128, 0x3bce6aaf63364afd867250e924da783cu128, 0x00);
dec_test!(bid128_round_integral_zero_012, bid128_round_integral_zero, 0, 0x3CB2314DC6448D9338C15B09ffffffffu128, 0x3cb2314dc6448d9338c15b09ffffffffu128, 0x00);
dec_test!(bid128_round_integral_zero_013, bid128_round_integral_zero, 0, 0x3CB2314DC6448D9338C15B0A00000000u128, 0x3cb2314dc6448d9338c15b0a00000000u128, 0x00);
dec_test!(bid128_round_integral_zero_014, bid128_round_integral_zero, 0, 0x404000000000000066220a61d0b92ae1u128, 0x404000000000000066220a61d0b92ae1u128, 0x00);
dec_test!(bid128_round_integral_zero_015, bid128_round_integral_zero, 0, 0x40e0e8defd166f7d713a2065cb6900c1u128, 0x40e0e8defd166f7d713a2065cb6900c1u128, 0x00);
dec_test!(bid128_round_integral_zero_016, bid128_round_integral_zero, 0, 0x48000000000000000008020380402048u128, 0x48000000000000000008020380402048u128, 0x00);
dec_test!(bid128_round_integral_zero_017, bid128_round_integral_zero, 0, 0x4bc1ad7fe049e6d7a36c954ebdadd082u128, 0x4bc1ad7fe049e6d7a36c954ebdadd082u128, 0x00);
dec_test!(bid128_round_integral_zero_018, bid128_round_integral_zero, 0, 0x50080000000000000000000000000000u128, 0x50080000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_019, bid128_round_integral_zero, 0, 0x5090051000a024405dadbe8e7ff9a4deu128, 0x5090051000a024405dadbe8e7ff9a4deu128, 0x00);
dec_test!(bid128_round_integral_zero_020, bid128_round_integral_zero, 0, 0x54baf41602df4c7d5181ede05fd46419u128, 0x54baf41602df4c7d5181ede05fd46419u128, 0x00);
dec_test!(bid128_round_integral_zero_021, bid128_round_integral_zero, 0, 0x690468555ed4f0f7d9922fd589adef8fu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_022, bid128_round_integral_zero, 0, 0x793f338fb569875d9bcfc4a588776b15u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_025, bid128_round_integral_zero, 0, 0x7c00314dc6448d9338c15b0a00000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_023, bid128_round_integral_zero, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_024, bid128_round_integral_zero, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_026, bid128_round_integral_zero, 0, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_zero_027, bid128_round_integral_zero, 0, 0x7e00058b1d247b3c0aea9b0ae91383b4u128, 0x7c00058b1d247b3c0aea9b0ae91383b4u128, 0x01);
dec_test!(bid128_round_integral_zero_028, bid128_round_integral_zero, 0, 0x7e0009cfe015573e0113c46b265de9cdu128, 0x7c0009cfe015573e0113c46b265de9cdu128, 0x01);
dec_test!(bid128_round_integral_zero_029, bid128_round_integral_zero, 0, 0x7e000b1e271ef7e0ca5a6e6fa8843815u128, 0x7c000b1e271ef7e0ca5a6e6fa8843815u128, 0x01);
dec_test!(bid128_round_integral_zero_030, bid128_round_integral_zero, 0, 0x7e00116e883f67b1a6ad2561d94e47abu128, 0x7c00116e883f67b1a6ad2561d94e47abu128, 0x01);
dec_test!(bid128_round_integral_zero_031, bid128_round_integral_zero, 0, 0x7e0024f56b1a83cf2f341fcb73df6e9cu128, 0x7c0024f56b1a83cf2f341fcb73df6e9cu128, 0x01);
dec_test!(bid128_round_integral_zero_032, bid128_round_integral_zero, 0, 0xafff767edef7bdffbafcb11eee5ecff2u128, 0xb0400000000000000000000000000007u128, 0x00);
dec_test!(bid128_round_integral_zero_033, bid128_round_integral_zero, 0, 0xb03c503938e36cdb6a313861c9b75003u128, 0xb04000cd5f49f449dfc82c15753797d7u128, 0x00);
dec_test!(bid128_round_integral_zero_034, bid128_round_integral_zero, 0, 0xb6ccc46c69e989c2359f45f752afa94cu128, 0xb6ccc46c69e989c2359f45f752afa94cu128, 0x00);
dec_test!(bid128_round_integral_zero_035, bid128_round_integral_zero, 0, 0xb96dfc6073fc763dfd73e6c85c99363fu128, 0xb96c0000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_036, bid128_round_integral_zero, 0, 0xc2769a11a18238f07684855ad8d00fdcu128, 0xc2769a11a18238f07684855ad8d00fdcu128, 0x00);
dec_test!(bid128_round_integral_zero_037, bid128_round_integral_zero, 0, 0xc8d61646db2d3907eef4c54a43c62ebdu128, 0xc8d61646db2d3907eef4c54a43c62ebdu128, 0x00);
dec_test!(bid128_round_integral_zero_038, bid128_round_integral_zero, 0, 0xd307801862a1a51b67fdd7704d99d231u128, 0xd307801862a1a51b67fdd7704d99d231u128, 0x00);
dec_test!(bid128_round_integral_zero_039, bid128_round_integral_zero, 0, 0xdd03e68cc977e82c3eb804abab546268u128, 0xdd03e68cc977e82c3eb804abab546268u128, 0x00);
dec_test!(bid128_round_integral_zero_040, bid128_round_integral_zero, 0, 0xDF7FED09BEAD87C0378D8E63ffffffffu128, 0xdf7fed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_zero_041, bid128_round_integral_zero, 0, 0xDF7FED09BEAD87C0378D8E6400000000u128, 0xdf7e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_042, bid128_round_integral_zero, 0, 0xf7fcfc7dfffefff9fffffeffffffffeeu128, 0xdff20000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_043, bid128_round_integral_zero, 0, 0xfb7fbf5dfffdeafc3fad1f7bbadf27f2u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_044, bid128_round_integral_zero, 0, 0xfdbecffbffebf53f0905800000008000u128, 0xfc000ffbffebf53f0905800000008000u128, 0x00);
dec_test!(bid128_round_integral_zero_045, bid128_round_integral_zero, 0, 0xfe000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_zero_046, bid128_round_integral_zero, 0, 0xfe001c1551ff726e85f8d5226cf76148u128, 0xfc001c1551ff726e85f8d5226cf76148u128, 0x01);
dec_test!(bid128_round_integral_zero_047, bid128_round_integral_zero, 0, 0xfff77ff9f7fcffbdfdffffffffffffffu128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_zero_048, bid128_round_integral_zero, 0, "-Infinity"                           , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_049, bid128_round_integral_zero, 0, "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_zero_050, bid128_round_integral_zero, 0, "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_zero_051, bid128_round_integral_zero, 0, "18446744073709551616500E-3"          , "18446744073709551616E+0"             , 0x00);
