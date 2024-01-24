/* ----------------------------------------------------------------------------- */
/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_is_signed_001, bid128_is_signed, "0"                                   , false);
dec_test!(bid128_is_signed_002, bid128_is_signed, 0x0d1b4a11dd2b2c2d4d69c430aae5a351u128, false);
dec_test!(bid128_is_signed_003, bid128_is_signed, 0x1010080201c404080200000808020000u128, false);
dec_test!(bid128_is_signed_004, bid128_is_signed, 0x1da7727000019412043034285964b3e5u128, false);
dec_test!(bid128_is_signed_005, bid128_is_signed, 0x291faf7540e31c98a37ad09f66370b5cu128, false);
dec_test!(bid128_is_signed_006, bid128_is_signed, 0x32ed04cf2a6dd7973b5689f82fd08e7bu128, false);
dec_test!(bid128_is_signed_007, bid128_is_signed, 0x4f33bb3daf404b482895c9fcdd30262fu128, false);
dec_test!(bid128_is_signed_008, bid128_is_signed, 0x5bd983e54f226f4e447f2d30f6d2853du128, false);
dec_test!(bid128_is_signed_009, bid128_is_signed, 0x5d037a9f76ec79f3169e495eb8705c03u128, false);
dec_test!(bid128_is_signed_010, bid128_is_signed, 0x78000000000000000000000000000000u128, false);
dec_test!(bid128_is_signed_011, bid128_is_signed, 0xd4aa0000000000000000000000000000u128, true);
dec_test!(bid128_is_signed_012, bid128_is_signed, 0xdabc266156593af84fd97dc923bc7064u128, true);
dec_test!(bid128_is_signed_013, bid128_is_signed,  "Infinity"                           , false);
dec_test!(bid128_is_signed_014, bid128_is_signed, "-Infinity"                           , true);
dec_test!(bid128_is_signed_015, bid128_is_signed,      "QNaN"                           , false);
