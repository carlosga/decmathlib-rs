/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

mod common;

// dec_test!(bid128_is_infinity_001, bid128_is_infinity, -0i64                                 , false);
// dec_test!(bid128_is_infinity_002, bid128_is_infinity, 0u64                                  , false);
dec_test!(bid128_is_infinity_003, bid128_is_infinity, 0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_is_infinity_004, bid128_is_infinity, 0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_is_infinity_005, bid128_is_infinity, 0x0a57e401d7f2f4bf1a0275159b396b2au128, false);
dec_test!(bid128_is_infinity_006, bid128_is_infinity, 0x3737d5460658b65502508fbbc30ff543u128, false);
dec_test!(bid128_is_infinity_007, bid128_is_infinity, 0x5902fa4ce87119640b0601200002a041u128, false);
dec_test!(bid128_is_infinity_008, bid128_is_infinity, 0x7c003fffffffffff38c15b08ffffffffu128, false);
dec_test!(bid128_is_infinity_009, bid128_is_infinity, 0x7c003fffffffffff38c15b0affffffffu128, false);
dec_test!(bid128_is_infinity_010, bid128_is_infinity, 0x8cd4ffbce0ecd8e27cba342f141e4231u128, false);
dec_test!(bid128_is_infinity_011, bid128_is_infinity, 0x8dd84ed7938863291f6560ffe1332f11u128, false);
dec_test!(bid128_is_infinity_012, bid128_is_infinity, 0x935d9c0d11b20b309b8116fd41f5394fu128, false);
dec_test!(bid128_is_infinity_013, bid128_is_infinity, 0x94bc0000000000000000000000000000u128, false);
dec_test!(bid128_is_infinity_014, bid128_is_infinity, 0x9b6e0000000000000000000000000000u128, false);
dec_test!(bid128_is_infinity_015, bid128_is_infinity, 0xa9a7b8e122afeaccd57a83774becaedbu128, false);
dec_test!(bid128_is_infinity_016, bid128_is_infinity, 0xb05fafc0064a7b3137c23276a8be7ed3u128, false);
dec_test!(bid128_is_infinity_017, bid128_is_infinity, 0xbe42bd49b8cd1d759da62cb45a360a56u128, false);
// dec_test!(bid128_is_inf, bid128_isinf, Infinity 1 00);
// dec_test!(bid128_is_inf, bid128_isinf, QNaN 0 00);
// dec_test!(bid128_is_inf, bid128_isinf, SNaN 0 00);
