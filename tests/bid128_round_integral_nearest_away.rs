/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_round_integral_nearest_away_001, bid128_round_integral_nearest_away, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_002, bid128_round_integral_nearest_away, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_003, bid128_round_integral_nearest_away, 0, "0"                                   , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_004, bid128_round_integral_nearest_away, 0, "-0"                                  , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_005, bid128_round_integral_nearest_away, 0, 0x2302c790151002970880000400000022u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_006, bid128_round_integral_nearest_away, 0, 0x3000000403046800001a0b3a1e13586cu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_007, bid128_round_integral_nearest_away, 0, 0x300a7b92c29201de583800049020c098u128, 0x30400000000000000000000000263e7cu128, 0x00);
dec_test!(bid128_round_integral_nearest_away_008, bid128_round_integral_nearest_away, 0, 0x303b546416c082af75718cb88378ed1eu128, 0x3040005723dcdd508f38b08654197705u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_009, bid128_round_integral_nearest_away, 0, 0x3085ED09BEAD87C0378D8E63ffffffffu128, 0x3085ed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_nearest_away_010, bid128_round_integral_nearest_away, 0, 0x3085ED09BEAD87C0378D8E6400000000u128, 0x30840000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_011, bid128_round_integral_nearest_away, 0, 0x3CB2314DC6448D9338C15B09ffffffffu128, 0x3cb2314dc6448d9338c15b09ffffffffu128, 0x00);
dec_test!(bid128_round_integral_nearest_away_012, bid128_round_integral_nearest_away, 0, 0x3CB2314DC6448D9338C15B0A00000000u128, 0x3cb2314dc6448d9338c15b0a00000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_013, bid128_round_integral_nearest_away, 0, 0x40000000000000000000000000000000u128, 0x40000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_014, bid128_round_integral_nearest_away, 0, 0x404000018020880070348ac055972310u128, 0x404000018020880070348ac055972310u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_015, bid128_round_integral_nearest_away, 0, 0x40800000000000000040903404480000u128, 0x40800000000000000040903404480000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_016, bid128_round_integral_nearest_away, 0, 0x44000000000000000010800000400000u128, 0x44000000000000000010800000400000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_017, bid128_round_integral_nearest_away, 0, 0x78e7382cdde6bfe91d9aa291ad44b335u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_018, bid128_round_integral_nearest_away, 0, 0x7c00314dc6448d9338c15b0a00000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_019, bid128_round_integral_nearest_away, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_020, bid128_round_integral_nearest_away, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_021, bid128_round_integral_nearest_away, 0, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_nearest_away_022, bid128_round_integral_nearest_away, 0, 0x7e001413f293344497bf81501d92e628u128, 0x7c001413f293344497bf81501d92e628u128, 0x01);
dec_test!(bid128_round_integral_nearest_away_023, bid128_round_integral_nearest_away, 0, 0x7e001e7001298e7e45efdd84925ef8fau128, 0x7c001e7001298e7e45efdd84925ef8fau128, 0x01);
dec_test!(bid128_round_integral_nearest_away_024, bid128_round_integral_nearest_away, 0, 0x7e00217e424424d0276223680476b578u128, 0x7c00217e424424d0276223680476b578u128, 0x01);
dec_test!(bid128_round_integral_nearest_away_025, bid128_round_integral_nearest_away, 0, 0x7e002bcf6d09c0b6cd4a95361d2d757du128, 0x7c002bcf6d09c0b6cd4a95361d2d757du128, 0x01);
dec_test!(bid128_round_integral_nearest_away_026, bid128_round_integral_nearest_away, 0, 0xafff7fbfdffff3eb6d8e1e34f5e0bbe4u128, 0xb0400000000000000000000000000008u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_027, bid128_round_integral_nearest_away, 0, 0xb025683257a4c0b2e968c6dafbb4a1acu128, 0xb040000000000003f5dd0a28364c1ca2u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_028, bid128_round_integral_nearest_away, 0, 0xc82ed9104de961108746987bd570811cu128, 0xc82ed9104de961108746987bd570811cu128, 0x00);
dec_test!(bid128_round_integral_nearest_away_029, bid128_round_integral_nearest_away, 0, 0xdb9b13d408c7fee1a1e472f7fc304ae6u128, 0xdb9b13d408c7fee1a1e472f7fc304ae6u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_030, bid128_round_integral_nearest_away, 0, 0xDF7FED09BEAD87C0378D8E63ffffffffu128, 0xdf7fed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_nearest_away_031, bid128_round_integral_nearest_away, 0, 0xDF7FED09BEAD87C0378D8E6400000000u128, 0xdf7e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_032, bid128_round_integral_nearest_away, 0, 0xdfa5f9b538f9441d4f03792cbde10b60u128, 0xdfa40000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_033, bid128_round_integral_nearest_away, 0, 0xe002a2f5a825ddc4a1063108080c8c28u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_034, bid128_round_integral_nearest_away, 0, 0xfbe86f74adfb5fad088284ea23902252u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_035, bid128_round_integral_nearest_away, 0, 0xfddff79fc5bcbffdfffeffefdff7ffffu128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_036, bid128_round_integral_nearest_away, 0, 0xfe00133f591b479bc7c6516e558528d1u128, 0xfc00133f591b479bc7c6516e558528d1u128, 0x01);
dec_test!(bid128_round_integral_nearest_away_037, bid128_round_integral_nearest_away, 0, 0xfe002371a21bfd42548f4f6b11c2cebau128, 0xfc002371a21bfd42548f4f6b11c2cebau128, 0x01);
dec_test!(bid128_round_integral_nearest_away_038, bid128_round_integral_nearest_away, 0, 0xfe002e75147f78530af4f9fc56a3e25fu128, 0xfc002e75147f78530af4f9fc56a3e25fu128, 0x01);
dec_test!(bid128_round_integral_nearest_away_039, bid128_round_integral_nearest_away, 0, 0xfe002ff8ab0c91742bd0b3760bb4d665u128, 0xfc002ff8ab0c91742bd0b3760bb4d665u128, 0x01);
dec_test!(bid128_round_integral_nearest_away_040, bid128_round_integral_nearest_away, 0, 0xffeffbffffe6ffef2000000000040000u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_nearest_away_041, bid128_round_integral_nearest_away, 0, "-Infinity"                           , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_away_042, bid128_round_integral_nearest_away, 0, "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);

dec_test!(bid128_round_integral_nearest_away_043, bid128_round_integral_nearest_away, 0, "18446744073709551616500E-3"          , "18446744073709551617E+0"             , 0x00);
dec_test!(bid128_round_integral_nearest_away_044, bid128_round_integral_nearest_away, 0, "900000000000000000000001E-23"        , "9"                                   , 0x00);
