/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_string_001, bid128_to_string, /*0,*/ 0x0001ed09bead87c0378d8e62ffffffffu128, "+9999999999999999999999995705032703E-6176");
dec_test!(bid128_to_string_002, bid128_to_string, /*0,*/ 0x0001ed09bead87c0378d8e64ffffffffu128, "+0E-6176");
dec_test!(bid128_to_string_003, bid128_to_string, /*0,*/ 0x30400000000000000000000000000010u128, "+16E+0");
dec_test!(bid128_to_string_004, bid128_to_string, /*0,*/ 0x60000000000000000000000000000000u128, "+0E-6176");
dec_test!(bid128_to_string_005, bid128_to_string, /*0,*/ 0x69dbb75d7734cd9e1234567890123456u128, "+0E-1129");
dec_test!(bid128_to_string_006, bid128_to_string, /*0,*/ 0x79003fffffffffff38c15b08ffffffffu128, "+Inf");
dec_test!(bid128_to_string_007, bid128_to_string, /*0,*/ 0x79100000000000000000000000000000u128, "+Inf");
dec_test!(bid128_to_string_008, bid128_to_string, /*0,*/ 0x7c003fffffffffff38c15b08ffffffffu128, "+NaN");
dec_test!(bid128_to_string_009, bid128_to_string, /*0,*/ 0x7c003fffffffffff38c15b0affffffffu128, "+NaN");
dec_test!(bid128_to_string_010, bid128_to_string, /*0,*/ 0x7e000000000000000000000000000000u128, "+SNaN");
dec_test!(bid128_to_string_011, bid128_to_string, /*0,*/ 0x7eff3fffffffffffffffffffffffffffu128, "+SNaN");
dec_test!(bid128_to_string_012, bid128_to_string, /*0,*/ 0xb0fa0000000000000000000001312d00u128, "-20000000E+93");
dec_test!(bid128_to_string_013, bid128_to_string, /*0,*/ 0xe0000000000000000000000000000001u128, "-0E-6176");
dec_test!(bid128_to_string_014, bid128_to_string, /*0,*/ 0xf9003fffffffffff38c15b08ffffffffu128, "-Inf");
dec_test!(bid128_to_string_015, bid128_to_string, /*1,*/ 0x0001ed09bead87c0378d8e62ffffffffu128, "+9999999999999999999999995705032703E-6176");
dec_test!(bid128_to_string_016, bid128_to_string, /*1,*/ 0x0001ed09bead87c0378d8e64ffffffffu128, "+0E-6176");
dec_test!(bid128_to_string_017, bid128_to_string, /*1,*/ 0x7c003fffffffffff38c15b08ffffffffu128, "+NaN");
dec_test!(bid128_to_string_018, bid128_to_string, /*1,*/ 0x7c003fffffffffff38c15b0affffffffu128, "+NaN");
dec_test!(bid128_to_string_019, bid128_to_string, /*2,*/ 0x0001ed09bead87c0378d8e62ffffffffu128, "+9999999999999999999999995705032703E-6176");
dec_test!(bid128_to_string_020, bid128_to_string, /*2,*/ 0x0001ed09bead87c0378d8e64ffffffffu128, "+0E-6176");
dec_test!(bid128_to_string_021, bid128_to_string, /*2,*/ 0x7c003fffffffffff38c15b08ffffffffu128, "+NaN");
dec_test!(bid128_to_string_022, bid128_to_string, /*2,*/ 0x7c003fffffffffff38c15b0affffffffu128, "+NaN");
dec_test!(bid128_to_string_023, bid128_to_string, /*3,*/ 0x0001ed09bead87c0378d8e62ffffffffu128, "+9999999999999999999999995705032703E-6176");
dec_test!(bid128_to_string_024, bid128_to_string, /*3,*/ 0x0001ed09bead87c0378d8e64ffffffffu128, "+0E-6176");
dec_test!(bid128_to_string_025, bid128_to_string, /*3,*/ 0x7c003fffffffffff38c15b08ffffffffu128, "+NaN");
dec_test!(bid128_to_string_026, bid128_to_string, /*3,*/ 0x7c003fffffffffff38c15b0affffffffu128, "+NaN");
dec_test!(bid128_to_string_027, bid128_to_string, /*3,*/ 0xfe003fffffffffffffffffffffffffffu128, "-SNaN");
dec_test!(bid128_to_string_028, bid128_to_string, /*4,*/ 0x0001ed09bead87c0378d8e62ffffffffu128, "+9999999999999999999999995705032703E-6176");
dec_test!(bid128_to_string_029, bid128_to_string, /*4,*/ 0x0001ed09bead87c0378d8e64ffffffffu128, "+0E-6176");
dec_test!(bid128_to_string_030, bid128_to_string, /*4,*/ 0x7c003fffffffffff38c15b08ffffffffu128, "+NaN");
dec_test!(bid128_to_string_031, bid128_to_string, /*4,*/ 0x7c003fffffffffff38c15b0affffffffu128, "+NaN");