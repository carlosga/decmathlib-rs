/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(unused)]

use std::convert::From;
use decmathlib_rs::d128::ClassTypes;

mod common;

dec_test!(bid128class_001, bid128_class, 0x60000000000000000000000000000000u128, ClassTypes::PositiveZero);
dec_test!(bid128class_002, bid128_class, 0x6003b75d7734cd9e1234567890123456u128, ClassTypes::PositiveZero);
dec_test!(bid128class_003, bid128_class, 0x69dbb75d7734cd9e1234567890123456u128, ClassTypes::PositiveZero);
dec_test!(bid128class_004, bid128_class, 0x78000000000000000000000000000000u128, ClassTypes::PositiveInfinity);
dec_test!(bid128class_005, bid128_class, 0x78000000000000000000000000000001u128, ClassTypes::PositiveInfinity);
dec_test!(bid128class_006, bid128_class, 0x7c000000000000000000000000000000u128, ClassTypes::QuietNaN);
dec_test!(bid128class_007, bid128_class, 0x7c000000000000000000000000000001u128, ClassTypes::QuietNaN);
dec_test!(bid128class_008, bid128_class, 0x7c003fffffffffff38c15b08ffffffffu128, ClassTypes::QuietNaN);
dec_test!(bid128class_009, bid128_class, 0x7c003fffffffffff38c15b0affffffffu128, ClassTypes::QuietNaN);
dec_test!(bid128class_010, bid128_class, 0x7e000000000000000000000000000000u128, ClassTypes::SignalingNaN);
dec_test!(bid128class_011, bid128_class, 0x7e000000000000000000000000000001u128, ClassTypes::SignalingNaN);
dec_test!(bid128class_012, bid128_class, 0xe0000000000000000000000000000001u128, ClassTypes::NegativeZero);
dec_test!(bid128class_013, bid128_class, 0xe003b75d7734cd9e1234567890123456u128, ClassTypes::NegativeZero);
dec_test!(bid128class_014, bid128_class, 0xe9dbb75d7734cd9e1234567890123456u128, ClassTypes::NegativeZero);
dec_test!(bid128class_015, bid128_class, 0xf8000000000000000000000000000000u128, ClassTypes::NegativeInfinity);
dec_test!(bid128class_016, bid128_class, 0xfc000000000000000000000000000000u128, ClassTypes::QuietNaN);
dec_test!(bid128class_017, bid128_class, 0xfc000000000000000000000000000001u128, ClassTypes::QuietNaN);
dec_test!(bid128class_018, bid128_class, 0xfe000000000000000000000000000000u128, ClassTypes::SignalingNaN);
dec_test!(bid128class_019, bid128_class, 0xfe000000000000000000000000000001u128, ClassTypes::SignalingNaN);
dec_test!(bid128class_020, bid128_class, "-Infinity"                           , ClassTypes::NegativeInfinity);
dec_test!(bid128class_021, bid128_class,  "Infinity"                           , ClassTypes::PositiveInfinity);

dec_test!(bid128class_022, bid128_class, 0x00400000000000000000000000000001u128, ClassTypes::PositiveSubnormal);
dec_test!(bid128class_023, bid128_class, 0x0000314DC6448D93FFFFFFFFFFFFFFFFu128, ClassTypes::PositiveNormal);
dec_test!(bid128class_024, bid128_class, 0x0000314DC6448D93FFFFFFFF00000000u128, ClassTypes::PositiveNormal);
dec_test!(bid128class_025, bid128_class, 0x002A000000000000FFFFFFFFFFFFFFFFu128, ClassTypes::PositiveNormal);
dec_test!(bid128class_026, bid128_class, 0x0028000000000000FFFFFFFFFFFFFFFFu128, ClassTypes::PositiveNormal);
dec_test!(bid128class_027, bid128_class, 0x002A000000000000000000E8D4A51000u128, ClassTypes::PositiveNormal);
dec_test!(bid128class_028, bid128_class, 0x002A000000000000000000E8D4A51001u128, ClassTypes::PositiveNormal);
dec_test!(bid128class_029, bid128_class, 0x002A000000000000000000E8D4A50FFFu128, ClassTypes::PositiveSubnormal);
dec_test!(bid128class_030, bid128_class, 0x0000314DC6448D9338C15B0A00000001u128, ClassTypes::PositiveNormal);
dec_test!(bid128class_031, bid128_class, 0x0000314DC6448D9338C15B0A00000000u128, ClassTypes::PositiveNormal);
dec_test!(bid128class_032, bid128_class, 0x0000314DC6448D9338C15B09FFFFFFFFu128, ClassTypes::PositiveSubnormal);
dec_test!(bid128class_033, bid128_class, 0x802A000000000000000000E8D4A51000u128, ClassTypes::NegativeNormal);
dec_test!(bid128class_034, bid128_class, 0x802A000000000000000000E8D4A51001u128, ClassTypes::NegativeNormal);
dec_test!(bid128class_035, bid128_class, 0x802A000000000000000000E8D4A50FFFu128, ClassTypes::NegativeSubnormal);
dec_test!(bid128class_036, bid128_class, 0x8000314DC6448D9338C15B0A00000001u128, ClassTypes::NegativeNormal);
dec_test!(bid128class_037, bid128_class, 0x8000314DC6448D9338C15B0A00000000u128, ClassTypes::NegativeNormal);
dec_test!(bid128class_038, bid128_class, 0x8000314DC6448D9338C15B09FFFFFFFFu128, ClassTypes::NegativeSubnormal);