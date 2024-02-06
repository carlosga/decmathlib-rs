/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_is_zero_001, bid128_is_zero,  0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_is_zero_002, bid128_is_zero,  0x0001ed09bead87c0378d8e64ffffffffu128, true);
dec_test!(bid128_is_zero_003, bid128_is_zero,  "-0"                                  , true);
dec_test!(bid128_is_zero_004, bid128_is_zero,   "0"                                  , true);
dec_test!(bid128_is_zero_005, bid128_is_zero,  0x0ee80000000000000000000000000000u128, true);
dec_test!(bid128_is_zero_006, bid128_is_zero,  0x13ee9ca2e80fd3a807c0d8414c535392u128, false);
dec_test!(bid128_is_zero_007, bid128_is_zero,  0x31de9b1749a9038c04b0b67e429838c2u128, false);
dec_test!(bid128_is_zero_008, bid128_is_zero,  0x3b6ffefffd79ddfd3304651402b7cb82u128, true);
dec_test!(bid128_is_zero_009, bid128_is_zero,  0x789b88be70d10384ffffffffffffffffu128, false);
dec_test!(bid128_is_zero_010, bid128_is_zero,  0x7c003fffffffffff38c15b08ffffffffu128, false);
dec_test!(bid128_is_zero_011, bid128_is_zero,  0x7c003fffffffffff38c15b0affffffffu128, false);
dec_test!(bid128_is_zero_012, bid128_is_zero,  0x7e0028d5f55d1b90cd0683a16d4f6440u128, false);
dec_test!(bid128_is_zero_013, bid128_is_zero,  0x892747418097592c11a5167c09ca2055u128, false);
dec_test!(bid128_is_zero_014, bid128_is_zero,  0x8aa0c8dafc695d0242fb6071b7550296u128, false);
dec_test!(bid128_is_zero_015, bid128_is_zero,  0x9ee35adc537f299321571042d581776au128, false);
dec_test!(bid128_is_zero_016, bid128_is_zero,  0xaf520000000000000000000000000000u128, true);
dec_test!(bid128_is_zero_017, bid128_is_zero,  0xb37ef809e2d1f6a62badece51a0eddd9u128, false);
dec_test!(bid128_is_zero_018, bid128_is_zero,  0xb8249c80a0002a5e9fc635c5912fb958u128, false);
dec_test!(bid128_is_zero_019, bid128_is_zero,  0xd8e96a50ff859c401b0d91d7b39d89c8u128, false);
dec_test!(bid128_is_zero_020, bid128_is_zero, "-Infinity"                            , false);
dec_test!(bid128_is_zero_021, bid128_is_zero,  "Infinity"                            , false);
dec_test!(bid128_is_zero_022, bid128_is_zero,      "QNaN"                            , false);
dec_test!(bid128_is_zero_023, bid128_is_zero,      "SNaN"                            , false);
