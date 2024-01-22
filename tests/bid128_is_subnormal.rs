/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

// dec_test!(bid128_is_subnormal_001, bid128_is_subnormal, -0i64                                  , false);
dec_test!(bid128_is_subnormal_002, bid128_is_subnormal,  0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_is_subnormal_003, bid128_is_subnormal,  0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_is_subnormal_004, bid128_is_subnormal,  0x000200000000000007a63158fbd6b32fu128, true);
dec_test!(bid128_is_subnormal_005, bid128_is_subnormal,  0x00080000000000000002000000000000u128, true);
dec_test!(bid128_is_subnormal_006, bid128_is_subnormal,  0x001020104460001d09012838901080cdu128, false);
dec_test!(bid128_is_subnormal_007, bid128_is_subnormal,  0x08200000000000000000000000000000u128, false);
dec_test!(bid128_is_subnormal_008, bid128_is_subnormal,  0x1b0b59df37c84a5a5ca7168a8feedcb1u128, false);
dec_test!(bid128_is_subnormal_009, bid128_is_subnormal,  0x1b4cc22b9c3e09bd8a09c2efa4609a41u128, false);
dec_test!(bid128_is_subnormal_010, bid128_is_subnormal,  0x238f2cfa1d644f695cf1e547007227bdu128, false);
dec_test!(bid128_is_subnormal_011, bid128_is_subnormal,  0x247e394a27efd967381010001400081cu128, false);
dec_test!(bid128_is_subnormal_012, bid128_is_subnormal,  0x264a0000000000000000000000000000u128, false);
dec_test!(bid128_is_subnormal_013, bid128_is_subnormal,  0x402afbe261e6cd8a65a1629af55f94beu128, false);
dec_test!(bid128_is_subnormal_014, bid128_is_subnormal,  0x46360000000000000000000000000000u128, false);
dec_test!(bid128_is_subnormal_015, bid128_is_subnormal,  0x7c003fffffffffff38c15b08ffffffffu128, false);
dec_test!(bid128_is_subnormal_016, bid128_is_subnormal,  0x7c003fffffffffff38c15b0affffffffu128, false);
dec_test!(bid128_is_subnormal_017, bid128_is_subnormal,  0x8000001400000004ffffffffffffffefu128, true);
dec_test!(bid128_is_subnormal_018, bid128_is_subnormal,  0x92a9700a58e271c45f52b6c080f12c07u128, false);
dec_test!(bid128_is_subnormal_019, bid128_is_subnormal,  0xa83e0000000000000000000000000000u128, false);
dec_test!(bid128_is_subnormal_020, bid128_is_subnormal,  0xbefbefbfaf5bdfddaddfe727ecde7d3bu128, false);
dec_test!(bid128_is_subnormal_021, bid128_is_subnormal,  0xd754ce3bc22f555f79c8815335535001u128, false);
dec_test!(bid128_is_subnormal_022, bid128_is_subnormal,  0xd9000000000000000000000000000000u128, false);
dec_test!(bid128_is_subnormal_023, bid128_is_subnormal,  0xfdfdf7ff7ffdf7bfffffefffffffffafu128, false);
// dec_test!(bid128_is_subnormal_024, bid128_is_subnormal, -Infinity 0 00);
// dec_test!(bid128_is_subnormal_025, bid128_is_subnormal, Infinity 0 00);
// dec_test!(bid128_is_subnormal_026, bid128_is_subnormal, QNaN 0 00);
// dec_test!(bid128_is_subnormal_027, bid128_is_subnormal, SNaN 0 00);
