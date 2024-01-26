/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

use std::convert::From;
use decmathlib_rs::core::ClassTypes;

mod common;

dec_test!(bid128class_001, bid128_class, 0x60000000000000000000000000000000u128, ClassTypes::positiveZero);
dec_test!(bid128class_002, bid128_class, 0x6003b75d7734cd9e1234567890123456u128, ClassTypes::positiveZero);
dec_test!(bid128class_003, bid128_class, 0x69dbb75d7734cd9e1234567890123456u128, ClassTypes::positiveZero);
dec_test!(bid128class_004, bid128_class, 0x78000000000000000000000000000000u128, ClassTypes::positiveInfinity);
dec_test!(bid128class_005, bid128_class, 0x78000000000000000000000000000001u128, ClassTypes::positiveInfinity);
dec_test!(bid128class_006, bid128_class, 0x7c000000000000000000000000000000u128, ClassTypes::quietNaN);
dec_test!(bid128class_007, bid128_class, 0x7c000000000000000000000000000001u128, ClassTypes::quietNaN);
dec_test!(bid128class_008, bid128_class, 0x7c003fffffffffff38c15b08ffffffffu128, ClassTypes::quietNaN);
dec_test!(bid128class_009, bid128_class, 0x7c003fffffffffff38c15b0affffffffu128, ClassTypes::quietNaN);
dec_test!(bid128class_010, bid128_class, 0x7e000000000000000000000000000000u128, ClassTypes::signalingNaN);
dec_test!(bid128class_011, bid128_class, 0x7e000000000000000000000000000001u128, ClassTypes::signalingNaN);
dec_test!(bid128class_012, bid128_class, 0xe0000000000000000000000000000001u128, ClassTypes::negativeZero);
dec_test!(bid128class_013, bid128_class, 0xe003b75d7734cd9e1234567890123456u128, ClassTypes::negativeZero);
dec_test!(bid128class_014, bid128_class, 0xe9dbb75d7734cd9e1234567890123456u128, ClassTypes::negativeZero);
dec_test!(bid128class_015, bid128_class, 0xf8000000000000000000000000000000u128, ClassTypes::negativeInfinity);
dec_test!(bid128class_016, bid128_class, 0xfc000000000000000000000000000000u128, ClassTypes::quietNaN);
dec_test!(bid128class_017, bid128_class, 0xfc000000000000000000000000000001u128, ClassTypes::quietNaN);
dec_test!(bid128class_018, bid128_class, 0xfe000000000000000000000000000000u128, ClassTypes::signalingNaN);
dec_test!(bid128class_019, bid128_class, 0xfe000000000000000000000000000001u128, ClassTypes::signalingNaN);
dec_test!(bid128class_020, bid128_class, "-Infinity"                           , ClassTypes::negativeInfinity);
dec_test!(bid128class_021, bid128_class,  "Infinity"                           , ClassTypes::positiveInfinity);
