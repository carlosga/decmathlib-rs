/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

// dec_test!(bid128_is_normal_001, bid128_is_normal, -0i64                                  , false);
// dec_test!(bid128_is_normal_002, bid128_is_normal,  0u64                                  , false);
dec_test!(bid128_is_normal_003, bid128_is_normal,  0x00000000000000000000000000000000u128, false);
dec_test!(bid128_is_normal_004, bid128_is_normal,  0x0000000000000000ff7eff7efde9eb6fu128, false);
dec_test!(bid128_is_normal_005, bid128_is_normal,  0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_is_normal_006, bid128_is_normal,  0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_is_normal_007, bid128_is_normal,  0x00400000000000000010040010214040u128, true);
dec_test!(bid128_is_normal_008, bid128_is_normal,  0x03f40000000000000000000000000000u128, false);
dec_test!(bid128_is_normal_009, bid128_is_normal,  0x09780000000000000000000000000000u128, false);
dec_test!(bid128_is_normal_010, bid128_is_normal,  0x1a6a0000000000000000000000000000u128, false);
dec_test!(bid128_is_normal_011, bid128_is_normal,  0x25d9f797bbcaf773fffffddf7efcfff2u128, false);
dec_test!(bid128_is_normal_012, bid128_is_normal,  0x35ace066c705811c32d5b5ac007a79e4u128, true);
dec_test!(bid128_is_normal_013, bid128_is_normal,  0x42ec0000000000000000000000000000u128, false);
dec_test!(bid128_is_normal_014, bid128_is_normal,  0x49580caebdff5a9616f246996cb5d380u128, true);
dec_test!(bid128_is_normal_015, bid128_is_normal,  0x4f2e07ed4464f530f2b733c46e098dfau128, true);
dec_test!(bid128_is_normal_016, bid128_is_normal,  0x7c003fffffffffff38c15b08ffffffffu128, false);
dec_test!(bid128_is_normal_017, bid128_is_normal,  0x7c003fffffffffff38c15b0affffffffu128, false);
dec_test!(bid128_is_normal_018, bid128_is_normal,  0x818260b7152cc2d1e918ef83f6aaf170u128, true);
dec_test!(bid128_is_normal_019, bid128_is_normal,  0x83c9ce9c161201425481842016081200u128, true);
dec_test!(bid128_is_normal_020, bid128_is_normal,  0x9d083f98b8a9588416a840ca162eead0u128, true);
dec_test!(bid128_is_normal_021, bid128_is_normal,  0x9f777faa7f240d67872dedfcdfe2fa8eu128, true);
dec_test!(bid128_is_normal_022, bid128_is_normal,  0xa9634862e4a483809e678b2e978fb31cu128, true);
dec_test!(bid128_is_normal_023, bid128_is_normal,  0xfffdffffffbffff7fffbf7fcf7f5edf7u128, false);