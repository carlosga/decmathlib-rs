/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

mod common;

dec_test!(bid128_is_signaling_001, bid128_is_signaling, -0i64                                  , false);
dec_test!(bid128_is_signaling_002, bid128_is_signaling,  0u64                                  , false);
dec_test!(bid128_is_signaling_003, bid128_is_signaling,  0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_is_signaling_004, bid128_is_signaling,  0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_is_signaling_005, bid128_is_signaling,  0x041044f2703fe2d8fd85989865ebe87bu128, false);
dec_test!(bid128_is_signaling_006, bid128_is_signaling,  0x13aa0000000000000000000000000000u128, false);
dec_test!(bid128_is_signaling_007, bid128_is_signaling,  0x3664db3ee13f5c050db8edd219ca964cu128, false);
dec_test!(bid128_is_signaling_008, bid128_is_signaling,  0x7c00061ccb0a291e65f549f4dc5e31c4u128, false);
dec_test!(bid128_is_signaling_009, bid128_is_signaling,  0x7c003fffffffffff38c15b08ffffffffu128, false);
dec_test!(bid128_is_signaling_010, bid128_is_signaling,  0x7c003fffffffffff38c15b0affffffffu128, false);
dec_test!(bid128_is_signaling_011, bid128_is_signaling,  0x82b17468499ab71919d2bb2c0fb23df0u128, false);
dec_test!(bid128_is_signaling_012, bid128_is_signaling,  0x858a2ae1be255ac2e5c96a599c35a1e2u128, false);
dec_test!(bid128_is_signaling_013, bid128_is_signaling,  0x86d4c3261d14ad3241fa2f426682032au128, false);
dec_test!(bid128_is_signaling_014, bid128_is_signaling,  0xa294a762989a43543e8759e80229fe58u128, false);
dec_test!(bid128_is_signaling_015, bid128_is_signaling,  0xa3ea377d806481be43653c96237e2c57u128, false);
dec_test!(bid128_is_signaling_016, bid128_is_signaling,  0xfe000000000000000000000000000000u128, true);
// dec_test!(bid128_is_signaling_017, bid128_is_signaling, -Infinity 0 00);
// dec_test!(bid128_is_signaling_018, bid128_is_signaling, Infinity 0 00);
// dec_test!(bid128_is_signaling_019, bid128_is_signaling, SNaN 1 00);
