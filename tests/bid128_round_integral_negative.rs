/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_round_integral_negative_001, bid128_round_integral_negative, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_002, bid128_round_integral_negative, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_003, bid128_round_integral_negative, 0, "0"                                   , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_004, bid128_round_integral_negative, 0, "-0"                                  , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_005, bid128_round_integral_negative, 0, 0x2e356032ef374a1ef5c18b58d399db18u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_006, bid128_round_integral_negative, 0, 0x30000000620200007ffffcfffff7bfcdu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_007, bid128_round_integral_negative, 0, 0x303c50a05d300f59044b8170c010c4e1u128, 0x304000ce6754f5e9d9a498559b8549a6u128, 0x00);
dec_test!(bid128_round_integral_negative_008, bid128_round_integral_negative, 0, 0x3085ED09BEAD87C0378D8E63ffffffffu128, 0x3085ed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_negative_009, bid128_round_integral_negative, 0, 0x3085ED09BEAD87C0378D8E6400000000u128, 0x30840000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_010, bid128_round_integral_negative, 0, 0x3CB2314DC6448D9338C15B09ffffffffu128, 0x3cb2314dc6448d9338c15b09ffffffffu128, 0x00);
dec_test!(bid128_round_integral_negative_011, bid128_round_integral_negative, 0, 0x3CB2314DC6448D9338C15B0A00000000u128, 0x3cb2314dc6448d9338c15b0a00000000u128, 0x00);
dec_test!(bid128_round_integral_negative_012, bid128_round_integral_negative, 0, 0x40000000000000000008202000000010u128, 0x40000000000000000008202000000010u128, 0x00);
dec_test!(bid128_round_integral_negative_013, bid128_round_integral_negative, 0, 0x40000000000000007842d7d33f5ceefau128, 0x40000000000000007842d7d33f5ceefau128, 0x00);
dec_test!(bid128_round_integral_negative_014, bid128_round_integral_negative, 0, 0x40400000000000000000000000000000u128, 0x40400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_015, bid128_round_integral_negative, 0, 0x4b97bef1c61c8d7d62beb8870c3e5a1eu128, 0x4b97bef1c61c8d7d62beb8870c3e5a1eu128, 0x00);
dec_test!(bid128_round_integral_negative_016, bid128_round_integral_negative, 0, 0x50803f0e5867db6ae10832b0bce6cf06u128, 0x50803f0e5867db6ae10832b0bce6cf06u128, 0x00);
dec_test!(bid128_round_integral_negative_017, bid128_round_integral_negative, 0, 0x7a1d7af6782abebace4a15e54f6dfed8u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_018, bid128_round_integral_negative, 0, 0x7c00314dc6448d9338c15b0a00000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_019, bid128_round_integral_negative, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_020, bid128_round_integral_negative, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_021, bid128_round_integral_negative, 0, 0x7c0565465f07724616e83dcb6ad99200u128, 0x7c0025465f07724616e83dcb6ad99200u128, 0x00);
dec_test!(bid128_round_integral_negative_022, bid128_round_integral_negative, 0, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_negative_023, bid128_round_integral_negative, 0, 0x7e00241b05c9feb6d734042769d02c5bu128, 0x7c00241b05c9feb6d734042769d02c5bu128, 0x01);
dec_test!(bid128_round_integral_negative_024, bid128_round_integral_negative, 0, 0xa16faece37bfee32636cd40894ffd68cu128, 0xb0400000000000000000000000000001u128, 0x00);
dec_test!(bid128_round_integral_negative_025, bid128_round_integral_negative, 0, 0xb000040305101002a7cdedb5e6cdb867u128, 0xb0400000000000000000000000000001u128, 0x00);
dec_test!(bid128_round_integral_negative_026, bid128_round_integral_negative, 0, 0xb011563badf876dc84bb2c08fc0eb1c3u128, 0xb040000000000000000000019dbc0584u128, 0x00);
dec_test!(bid128_round_integral_negative_027, bid128_round_integral_negative, 0, 0xb0165915736885acd0e5bd967af7e273u128, 0xb040000000000000000001a4afb2510au128, 0x00);
dec_test!(bid128_round_integral_negative_028, bid128_round_integral_negative, 0, 0xb03e647f5c4e49b45b5269f8727019ecu128, 0xb0400a0cbc6e3a920921d765a50b35cbu128, 0x00);
dec_test!(bid128_round_integral_negative_029, bid128_round_integral_negative, 0, 0xbea827b3ca5b5a1d8134b2004751a05fu128, 0xbea827b3ca5b5a1d8134b2004751a05fu128, 0x00);
dec_test!(bid128_round_integral_negative_030, bid128_round_integral_negative, 0, 0xd47a91fd09312878789b88bf70d14784u128, 0xd47a91fd09312878789b88bf70d14784u128, 0x00);
dec_test!(bid128_round_integral_negative_031, bid128_round_integral_negative, 0, 0xd4c64d1faca949b1856d026daa624159u128, 0xd4c64d1faca949b1856d026daa624159u128, 0x00);
dec_test!(bid128_round_integral_negative_032, bid128_round_integral_negative, 0, 0xDF7FED09BEAD87C0378D8E63ffffffffu128, 0xdf7fed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_negative_033, bid128_round_integral_negative, 0, 0xDF7FED09BEAD87C0378D8E6400000000u128, 0xdf7e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_034, bid128_round_integral_negative, 0, 0xdfc7ee97ff8effefffbfdff5bfffff7fu128, 0xdfc60000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_035, bid128_round_integral_negative, 0, 0xeab3a9be174202d23bafe9fb9b748a30u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_036, bid128_round_integral_negative, 0, 0xf2f5d1ebb9b6abc8030df3ffb920c07bu128, 0xcbd60000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_037, bid128_round_integral_negative, 0, 0xf9fde572f9d4e39dffff3dae53c71fadu128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_038, bid128_round_integral_negative, 0, 0xfe000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_negative_039, bid128_round_integral_negative, 0, 0xfe000ff3d59a5d91e4d0467d3a915507u128, 0xfc000ff3d59a5d91e4d0467d3a915507u128, 0x01);
dec_test!(bid128_round_integral_negative_040, bid128_round_integral_negative, 0, 0xfe001698682e86aa57b2178fa9c3d382u128, 0xfc001698682e86aa57b2178fa9c3d382u128, 0x01);
dec_test!(bid128_round_integral_negative_041, bid128_round_integral_negative, 0, 0xff6ffffffdffdfff3f7fffffff79fdfdu128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_negative_042, bid128_round_integral_negative, 0, "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_043, bid128_round_integral_negative, 0, "-Infinity"                           , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_044, bid128_round_integral_negative, 0, "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_negative_045, bid128_round_integral_negative, 0, "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);

dec_test!(bid128_round_integral_negative_046, bid128_round_integral_negative, 0, "-91E-1"                              , -10                                   , 0x00);
dec_test!(bid128_round_integral_negative_047, bid128_round_integral_negative, 0, "-90001E-4"                           , -10                                   , 0x00);
dec_test!(bid128_round_integral_negative_048, bid128_round_integral_negative, 0, "900000000000000000000001E-23"        , 9                                     , 0x00);
dec_test!(bid128_round_integral_negative_049, bid128_round_integral_negative, 0, "-9600000000000000000000000E-23"      , -96                                   , 0x00);
dec_test!(bid128_round_integral_negative_050, bid128_round_integral_negative, 0, "-900000000000000000000001E-23"       , -10                                   , 0x00);
