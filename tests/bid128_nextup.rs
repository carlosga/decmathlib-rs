/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_nextup_001, bid128_nextup, -0                                         , 0x00000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextup_002, bid128_nextup, 0                                          , 0x00000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextup_003, bid128_nextup, 0x0001ed09bead87c0378d8e62ffffffffu128     , 0x0001ed09bead87c0378d8e6300000000u128, 0x00);
dec_test!(bid128_nextup_004, bid128_nextup, 0x0001ed09bead87c0378d8e64ffffffffu128     , 0x00000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextup_005, bid128_nextup, 0x00280000000000000000008000000000u128     , 0x000002b5e3af16b18800000000000001u128, 0x00);
dec_test!(bid128_nextup_006, bid128_nextup, 0x1dc2205f885182d05c4d7fcc63056120u128     , 0x1dc143bb532f1c239b06fdfbde35cb41u128, 0x00);
dec_test!(bid128_nextup_007, bid128_nextup, 0x2297188ee5995a16571813d4cb7ca060u128     , 0x2297188ee5995a16571813d4cb7ca061u128, 0x00);
dec_test!(bid128_nextup_008, bid128_nextup, 0x380a24e29845b5f260cd9cf82855437au128     , 0x380970d9f2b91b77c80821b19354a2c5u128, 0x00);
dec_test!(bid128_nextup_009, bid128_nextup, 0x380bcf94255508181807cc6de69fcee7u128     , 0x380bcf94255508181807cc6de69fcee8u128, 0x00);
dec_test!(bid128_nextup_010, bid128_nextup, 0x3aadf9ad6b4fdbf5f79bfcb6fdeb8fd8u128     , 0x00000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextup_011, bid128_nextup, 0x782d2f94d69006ec2196c0c64c5c5d60u128     , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextup_012, bid128_nextup, 0x7c003fffffffffff38c15b08ffffffffu128     , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextup_013, bid128_nextup, 0x7c003fffffffffff38c15b0affffffffu128     , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextup_014, bid128_nextup, 0x7e000000000000000000000000000000u128     , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextup_015, bid128_nextup, 0x7e000b4bdd5ec3e24d174387ac15f085u128     , 0x7c000b4bdd5ec3e24d174387ac15f085u128, 0x01);
dec_test!(bid128_nextup_016, bid128_nextup, 0x80000000000000000000000000000001u128     , 0x80000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextup_017, bid128_nextup, 0x80800000000000000000000000000001u128     , 0x803ded09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nextup_018, bid128_nextup, 0x8bae0000000000000000000000000000u128     , 0x00000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextup_019, bid128_nextup, "9.999999999999999999999999999999999E-6143", 0x0002314dc6448d9338c15b0a00000000u128, 0x00);
dec_test!(bid128_nextup_020, bid128_nextup, "9.999999999999999999999999999999999E+6144", 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextup_021, bid128_nextup, 0xfdffb5e7ffbbaf77a4dd7e5ad5fed731u128     , 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextup_022, bid128_nextup, 0xfe000000000000000000000000000000u128     , 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextup_023, bid128_nextup, 0xfe000574f7d3802ad93137567a500098u128     , 0xfc000574f7d3802ad93137567a500098u128, 0x01);
dec_test!(bid128_nextup_024, bid128_nextup, 0xfe002436840886490784a9055dbc2753u128     , 0xfc002436840886490784a9055dbc2753u128, 0x01);
dec_test!(bid128_nextup_025, bid128_nextup, 0xfffffffff77fffffffffffffdfffff7bu128     , 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextup_026, bid128_nextup, "Infinity"                                 , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextup_027, bid128_nextup, "-Infinity"                                , 0xdfffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nextup_028, bid128_nextup, "QNaN"                                     , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextup_029, bid128_nextup, "SNaN"                                     , 0x7c000000000000000000000000000000u128, 0x01);

// bid128_nextup 0 1 [7c00314dc6448d9338c15b0a00000001] [2ffe314dc6448d9338c15b0a00000001] 00
// bid128_nextup 0 [7c00314dc6448d9338c15b0a00000001] 1 [7c000000000000000000000000000000] 00
// bid128_nextup 0 [7c00314dc6448d9338c15b0a00000001] [7c00314dc6448d9338c15b0a00000001] [7c000000000000000000000000000000] 00
// bid128_nextup 0 [7c00314dc6448d9338c15b09ffffffff] [7c00314dc6448d9338c15b0a00000001] [7c00314dc6448d9338c15b09ffffffff] 00
// bid128_nextup 0 [7c00314dc6448d9338c15b0a00000001] [7c00314dc6448d9338c15b09ffffffff] [7c000000000000000000000000000000] 00
// bid128_nextup 0 4294967296 4294967297 [3010d3c21bcecceda100000000000001] 00
// bid128_nextup 0 4294967296 4294967295 [3010d3c21bcecceda100000000000001] 00
// bid128_nextup 0 4294967295 4294967297 [3010d3c21bcdf92b853133125f000001] 00
// bid128_nextup 0 4294967295 4294967294 [3010d3c21bcdf92b853133125f000001] 00
