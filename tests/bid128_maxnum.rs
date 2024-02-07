/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_maxnum_001, bid128_maxnum, 0x0000000000000000ffffffffffffffffu128, 0x0000000000000000ffffffffffffffffu128, 0x0000000000000000ffffffffffffffffu128, 0x00);
dec_test!(bid128_maxnum_002, bid128_maxnum, 0x00000000000020829420040221e302a2u128, 0x0000040000002000e67ffb6edeecc9f9u128, 0x0000040000002000e67ffb6edeecc9f9u128, 0x00);
dec_test!(bid128_maxnum_003, bid128_maxnum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_004, bid128_maxnum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_005, bid128_maxnum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_006, bid128_maxnum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_007, bid128_maxnum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_008, bid128_maxnum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_009, bid128_maxnum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_010, bid128_maxnum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_011, bid128_maxnum, "0"                                   , "-0"                                  , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_012, bid128_maxnum, 0x0040020300400080ab5be031c4007c10u128, 0x00163e6d0cc03c41fffffff7fffafff1u128, 0x0040020300400080ab5be031c4007c10u128, 0x00);
dec_test!(bid128_maxnum_013, bid128_maxnum, "-0"                                  , "-0"                                  , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_014, bid128_maxnum, 0x027f6444824c75d3de55c70f25ad47abu128, 0x0000040030001802bb3ade90d6a702afu128, 0x027f6444824c75d3de55c70f25ad47abu128, 0x00);
dec_test!(bid128_maxnum_015, bid128_maxnum, "0"                                   , "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_016, bid128_maxnum, 0x1b6ede4dc1b5e81a733ddf9a6de69324u128, 0x257d26a6df028622c403c0276e44511du128, 0x257d26a6df028622c403c0276e44511du128, 0x00);
dec_test!(bid128_maxnum_017, bid128_maxnum, 0x215019291413a80cee1a640242280804u128, 0x2138c03624008097445a9e7a579c27c0u128, 0x215019291413a80cee1a640242280804u128, 0x00);
dec_test!(bid128_maxnum_018, bid128_maxnum, 0x25f60000000000000000000000000000u128, 0x80180000000000000000000000000000u128, 0x25f60000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_019, bid128_maxnum, 0x3b2a1ed8bc22e8ae38a99edede5d22eeu128, 0xffd71d5dfd76b593519e76769ef19ecbu128, 0xfc001d5dfd76b593519e76769ef19ecbu128, 0x01);
dec_test!(bid128_maxnum_020, bid128_maxnum, 0x3ff99c3bcad0177fed783d3dc068938fu128, 0x4000400204000100a4c0281a10100c8au128, 0x4000400204000100a4c0281a10100c8au128, 0x00);
dec_test!(bid128_maxnum_021, bid128_maxnum, 0x405a0000000000000000000000000000u128, 0xae537b161246a75e8f1ba5a2fd65fce8u128, 0x405a0000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_022, bid128_maxnum, 0x53e82c96831cd96a8108f41a069e4e4fu128, 0x542000008001a0001124318113641030u128, 0x542000008001a0001124318113641030u128, 0x00);
dec_test!(bid128_maxnum_023, bid128_maxnum, 0x5f0912d3d8af5cf7431cde8f1e967a76u128, 0x1b9826915241fe6db7fb5f5b7b25d766u128, 0x5f0912d3d8af5cf7431cde8f1e967a76u128, 0x00);
dec_test!(bid128_maxnum_024, bid128_maxnum, 0x78000000000000000000000000000000u128, 0xa1a2bcedb041a13b2aece9ce6b8f7cebu128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_025, bid128_maxnum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_026, bid128_maxnum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_027, bid128_maxnum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_028, bid128_maxnum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_029, bid128_maxnum, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_030, bid128_maxnum, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_031, bid128_maxnum, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_032, bid128_maxnum, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_033, bid128_maxnum, 0x82406741d5d9af36391d155de3af7feeu128, 0xc7146777abc6a6cc01cd589112372927u128, 0x82406741d5d9af36391d155de3af7feeu128, 0x00);
dec_test!(bid128_maxnum_034, bid128_maxnum, 0x865fa9ff29f693748c76a9e2c56435a2u128, 0xcea0611bf6cc6e2269370457b658a21fu128, 0x865fa9ff29f693748c76a9e2c56435a2u128, 0x00);
dec_test!(bid128_maxnum_035, bid128_maxnum, 0x99ef6fd089ffacc44e22505adb104c92u128, 0xc7dbe3240b4d20353530230f89cd799au128, 0x99ef6fd089ffacc44e22505adb104c92u128, 0x00);
dec_test!(bid128_maxnum_036, bid128_maxnum, 0x9bdfd9430a17fe3c0000000000000020u128, 0x5fda1838be350be07c1fe63201a8ad03u128, 0x5fda1838be350be07c1fe63201a8ad03u128, 0x00);
dec_test!(bid128_maxnum_037, bid128_maxnum, 0x9fffedfbdbdff5ff7fade4e377eb3a1fu128, 0x30e75607ff5597e3e2a629ee50397f1bu128, 0x30e75607ff5597e3e2a629ee50397f1bu128, 0x00);
dec_test!(bid128_maxnum_038, bid128_maxnum, 0xaba4b9598f96ca54bf5e83af6fde2bb7u128, 0xffffff9dfffdfff700d7a59508e03740u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_039, bid128_maxnum, 0xbfacea0a6a0da1177298837e0a6750a2u128, 0xd1a2a0ee075d6de38968fc6d27031281u128, 0xbfacea0a6a0da1177298837e0a6750a2u128, 0x00);
dec_test!(bid128_maxnum_040, bid128_maxnum, 0xcaf9ce3f6fe65ecf7c421830890c8c4cu128, 0xed5c9ffd57ce63ff2048100280002000u128, 0xb5720000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_041, bid128_maxnum, 0xcff874f326ef65b3de41e92cabb02763u128, 0xcd01519cca19f40a20c1449a43cd5854u128, 0xcd01519cca19f40a20c1449a43cd5854u128, 0x00);
dec_test!(bid128_maxnum_042, bid128_maxnum, 0xdbf20000000000000000000000000000u128, 0x9e060000000000000000000000000000u128, 0xdbf20000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_043, bid128_maxnum, 0xef9e6eea5c3fd81df6b70f9d0a2dbfffu128, 0xdbdfffff7ffefffb759ffd3c7efeabfdu128, 0xbe780000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_044, bid128_maxnum, 0xf265d1006ac5a9d30a1cc11ad4580129u128, 0xf9f7feff7ffffcfd1061d8014f438130u128, 0xc9960000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_045, bid128_maxnum, 0xf8000000000000000000000000000000u128, 0xa85e3f33929cbe169fba91212c46fa36u128, 0xa85e3f33929cbe169fba91212c46fa36u128, 0x00);
dec_test!(bid128_maxnum_046, bid128_maxnum, 0xfbffebfe6eff7fff5482556b2a27b09cu128, 0xd68ecc41811c43d70000000000000000u128, 0xd68ecc41811c43d70000000000000000u128, 0x00);
dec_test!(bid128_maxnum_047, bid128_maxnum, 0xfc0007f4f1066ee942f47fcbdf9ce7d2u128, 0xab220000000000000000000000000000u128, 0xab220000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_048, bid128_maxnum, 0xfdbfb44dff6faffb7fffdfdbfddbffffu128, 0xff999ebef9fa7dad7fe1ae9ff6d7afc7u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_049, bid128_maxnum, 0xfde7bf7f3d7bafd8ffffbefffff7fffeu128, 0xa8cdbc16ba0a4d593374576f6aeb0a73u128, 0xa8cdbc16ba0a4d593374576f6aeb0a73u128, 0x00);
dec_test!(bid128_maxnum_050, bid128_maxnum, "-Infinity"                           , "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_051, bid128_maxnum, "-Infinity"                           , "-Infinity"                           , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_052, bid128_maxnum, "-Infinity"                           , "QNaN"                                , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_053, bid128_maxnum, "SNaN"                                , "-0"                                  , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_054, bid128_maxnum, "SNaN"                                , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);
