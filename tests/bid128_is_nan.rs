/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

// dec_test!(bid128_is_nan_001, bid128_is_nan, -0i64                                 , false);
dec_test!(bid128_is_nan_002, bid128_is_nan, 0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_is_nan_003, bid128_is_nan, 0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_is_nan_004, bid128_is_nan, 0x21e40000000000000000000000000000u128, false);
dec_test!(bid128_is_nan_005, bid128_is_nan, 0x2a21676f21ef040108e2139097e4f45cu128, false);
dec_test!(bid128_is_nan_006, bid128_is_nan, 0x41940000000000000000000000000000u128, false);
dec_test!(bid128_is_nan_007, bid128_is_nan, 0x78000000000000000000000000000000u128, false);
dec_test!(bid128_is_nan_008, bid128_is_nan, 0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_is_nan_009, bid128_is_nan, 0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_is_nan_010, bid128_is_nan, 0x80ed2b9d1a45002732c4bfcbb8b15c1bu128, false);
dec_test!(bid128_is_nan_011, bid128_is_nan, 0x95165ea3a5f0caf66ba829819d59b6e2u128, false);
dec_test!(bid128_is_nan_012, bid128_is_nan, 0xb0a9c89c8400622c4764c72807e8a958u128, false);
dec_test!(bid128_is_nan_013, bid128_is_nan, 0xcacbd04a2ef781fd33f2c1bcab363a74u128, false);
dec_test!(bid128_is_nan_014, bid128_is_nan, 0xda34655b3c4627e7f91a483887abf433u128, false);
// dec_test!(bid128_is_nan_015, bid128_is_nan, -Infinity 0);
// dec_test!(bid128_is_nan_016, bid128_is_nan, Infinity 0);
// dec_test!(bid128_is_nan_017, bid128_is_nan, QNaN 1);
