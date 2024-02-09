/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_nextdown_001, bid128_nextdown, 0x00000000000000000000000000000001u128      , 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextdown_002, bid128_nextdown, 0x0001ed09bead87c0378d8e62ffffffffu128      , 0x0001ed09bead87c0378d8e62fffffffeu128, 0x00);
dec_test!(bid128_nextdown_003, bid128_nextdown, 0x0001ed09bead87c0378d8e64ffffffffu128      , 0x80000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextdown_004, bid128_nextdown, 0x00400000000000000000000000000002u128      , 0x000009dc5ada82b70b59df01ffffffffu128, 0x00);
dec_test!(bid128_nextdown_005, bid128_nextdown, 0x00800000000000000000000000000001u128      , 0x003ded09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nextdown_006, bid128_nextdown, "-0"                                        , 0x80000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextdown_007, bid128_nextdown, 0                                           , 0x80000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextdown_008, bid128_nextdown, 0x1dc2205f885182d05c4d7fcc63056120u128      , 0x1dc143bb532f1c239b06fdfbde35cb3fu128, 0x00);
dec_test!(bid128_nextdown_009, bid128_nextdown, 0x2297188ee5995a16571813d4cb7ca060u128      , 0x2297188ee5995a16571813d4cb7ca05fu128, 0x00);
dec_test!(bid128_nextdown_010, bid128_nextdown, 0x380a24e29845b5f260cd9cf82855437au128      , 0x380970d9f2b91b77c80821b19354a2c3u128, 0x00);
dec_test!(bid128_nextdown_011, bid128_nextdown, 0x380bcf94255508181807cc6de69fcee7u128      , 0x380bcf94255508181807cc6de69fcee6u128, 0x00);
dec_test!(bid128_nextdown_012, bid128_nextdown, 0x6ffff2fd7eefff9f5801404100000401u128      , 0x80000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextdown_013, bid128_nextdown, 0x7c003fffffffffff38c15b08ffffffffu128      , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextdown_014, bid128_nextdown, 0x7c003fffffffffff38c15b0affffffffu128      , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextdown_015, bid128_nextdown, 0x7e000000000000000000000000000000u128      , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextdown_016, bid128_nextdown, 0x7e000b4bdd5ec3e24d174387ac15f085u128      , 0x7c000b4bdd5ec3e24d174387ac15f085u128, 0x01);
dec_test!(bid128_nextdown_017, bid128_nextdown, 0x8bae0000000000000000000000000000u128      , 0x80000000000000000000000000000001u128, 0x00);
dec_test!(bid128_nextdown_018, bid128_nextdown, "-9.999999999999999999999999999999999E+6144", 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextdown_019, bid128_nextdown, 0xf8fa90e5dcfb4727540dba68bc2a5f7du128      , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextdown_020, bid128_nextdown, 0xfdbffffd9ff7c3ff7ff3be5e87fe7747u128      , 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextdown_021, bid128_nextdown, 0xfe000000000000000000000000000000u128      , 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextdown_022, bid128_nextdown, 0xfe000574f7d3802ad93137567a500098u128      , 0xfc000574f7d3802ad93137567a500098u128, 0x01);
dec_test!(bid128_nextdown_023, bid128_nextdown, 0xfe002436840886490784a9055dbc2753u128      , 0xfc002436840886490784a9055dbc2753u128, 0x01);
dec_test!(bid128_nextdown_024, bid128_nextdown, 0xffffbffffffffffffa8cdfac7ffefdf7u128      , 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nextdown_025, bid128_nextdown, "Infinity"                                  , 0x5fffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nextdown_026, bid128_nextdown, "QNaN"                                      , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nextdown_027, bid128_nextdown, "SNaN"                                      , 0x7c000000000000000000000000000000u128, 0x01);

// bid128_nextdown 0 1 0x7c00314dc6448d9338c15b0a00000001u128 0x2ffded09bead87c0378d8e63ffffffffu128 00
// bid128_nextdown 0 0x7c00314dc6448d9338c15b0a00000001u128 1 0x7c000000000000000000000000000000u128 00
// bid128_nextdown 0 0x7c00314dc6448d9338c15b0a00000001u128 0x7c00314dc6448d9338c15b0a00000001u128 0x7c000000000000000000000000000000u128 00
// bid128_nextdown 0 0x7c00314dc6448d9338c15b09ffffffffu128 0x7c00314dc6448d9338c15b0a00000001u128 0x7c00314dc6448d9338c15b09ffffffffu128 00
// bid128_nextdown 0 0x7c00314dc6448d9338c15b0a00000001u128 0x7c00314dc6448d9338c15b09ffffffffu128 0x7c000000000000000000000000000000u128 00
// bid128_nextdown 0 4294967296 4294967297 0x3010d3c21bcecceda0ffffffffffffffu128 00
// bid128_nextdown 0 4294967296 4294967295 0x3010d3c21bcecceda0ffffffffffffffu128 00
// bid128_nextdown 0 4294967295 4294967297 0x3010d3c21bcdf92b853133125effffffu128 00
// bid128_nextdown 0 4294967295 4294967294 0x3010d3c21bcdf92b853133125effffffu128 00
