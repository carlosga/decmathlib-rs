/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_is_finite_001, bid128_is_finite, 0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_is_finite_002, bid128_is_finite, 0x0001ed09bead87c0378d8e64ffffffffu128, true);
dec_test!(bid128_is_finite_003, bid128_is_finite, "-0"                                  , true);
dec_test!(bid128_is_finite_004, bid128_is_finite,  "0"                                  , true);
dec_test!(bid128_is_finite_005, bid128_is_finite, 0x069ebbde47cecaf646be7beaba7b59b2u128, true);
dec_test!(bid128_is_finite_006, bid128_is_finite, 0x0e3676b062a501d10b09be374b9584afu128, true);
dec_test!(bid128_is_finite_007, bid128_is_finite, 0x0e89a019d51dfb526d3d4f5cec95045eu128, true);
dec_test!(bid128_is_finite_008, bid128_is_finite, 0x2fc2e6f5db986999e05bf17d978fe530u128, true);
dec_test!(bid128_is_finite_009, bid128_is_finite, 0x38f8c0be302ea60357e8f20e5933296fu128, true);
// TODO: Review
//dec_test!(bid128_is_finite_010, bid128_is_finite, "+5296735256349.E0"                   , true);
dec_test!(bid128_is_finite_011, bid128_is_finite, 0x587840394b1e40281e9750988e309c82u128, true);
dec_test!(bid128_is_finite_012, bid128_is_finite, 0x7c0013fea80d776e4da30049fb4205a6u128, false);
dec_test!(bid128_is_finite_013, bid128_is_finite, 0x7c003fffffffffff38c15b08ffffffffu128, false);
dec_test!(bid128_is_finite_014, bid128_is_finite, 0x7c003fffffffffff38c15b0affffffffu128, false);
dec_test!(bid128_is_finite_015, bid128_is_finite, 0xa34647a87b5d409d0b81a865d0b81418u128, true);
dec_test!(bid128_is_finite_016, bid128_is_finite, 0xb1a03c62cd15d1e3984fecd726ec37f2u128, true);
dec_test!(bid128_is_finite_017, bid128_is_finite, 0xb1cc5b6aee8fe1c3d7ba66922fd8e8d7u128, true);
dec_test!(bid128_is_finite_018, bid128_is_finite, 0xb5960000000000000000000000000000u128, true);
dec_test!(bid128_is_finite_019, bid128_is_finite, "-Infinity"                           , false);
dec_test!(bid128_is_finite_020, bid128_is_finite,  "Infinity"                           , false);
dec_test!(bid128_is_finite_021, bid128_is_finite,      "QNaN"                           , false);
dec_test!(bid128_is_finite_022, bid128_is_finite,      "SNaN"                           , false);
