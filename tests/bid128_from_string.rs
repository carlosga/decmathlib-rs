/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_from_string_001, bid128_from_string, 0, "0e6176"                             , "+0E+6111"                              , 0x00);
dec_test!(bid128_from_string_002, bid128_from_string, 0, "12345678901234567890123456789012345", "+1234567890123456789012345678901234E+1", 0x20);
dec_test!(bid128_from_string_003, bid128_from_string, 1, "12345678901234567890123456789012345", "+1234567890123456789012345678901234E+1", 0x20);
dec_test!(bid128_from_string_004, bid128_from_string, 2, "12345678901234567890123456789012345", "+1234567890123456789012345678901235E+1", 0x20);
dec_test!(bid128_from_string_005, bid128_from_string, 3, "12345678901234567890123456789012345", "+1234567890123456789012345678901234E+1", 0x20);
dec_test!(bid128_from_string_006, bid128_from_string, 4, "12345678901234567890123456789012345", "+1234567890123456789012345678901235E+1", 0x20);
dec_test!(bid128_from_string_007, bid128_from_string, 0, "1.0"                                , "+10E-1"                                , 0x00);
