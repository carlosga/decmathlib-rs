/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_from_string_001, bid128_from_string, 0u32, "0e6176"                                , "+0E+6111"                              , 0x00);
dec_test!(bid128_from_string_002, bid128_from_string, 0u32, "12345678901234567890123456789012345"   , "+1234567890123456789012345678901234E+1", 0x20);
dec_test!(bid128_from_string_003, bid128_from_string, 1u32, "12345678901234567890123456789012345"   , "+1234567890123456789012345678901234E+1", 0x20);
dec_test!(bid128_from_string_004, bid128_from_string, 2u32, "12345678901234567890123456789012345"   , "+1234567890123456789012345678901235E+1", 0x20);
dec_test!(bid128_from_string_005, bid128_from_string, 3u32, "12345678901234567890123456789012345"   , "+1234567890123456789012345678901234E+1", 0x20);
dec_test!(bid128_from_string_006, bid128_from_string, 4u32, "12345678901234567890123456789012345"   , "+1234567890123456789012345678901235E+1", 0x20);
dec_test!(bid128_from_string_007, bid128_from_string, 0u32, "1.0"                                   , "+10E-1"                                , 0x00);

dec_test!(bid128_from_string_008, bid128_from_string, 2u32, "-9.9999999999999999999999999999999995" , 0xafffed09bead87c0378d8e63ffffffffu128  , 0x20);
dec_test!(bid128_from_string_009, bid128_from_string, 1u32, "-9.9999999999999999999999999999999995" , 0xb000314dc6448d9338c15b0a00000000u128  , 0x20);
dec_test!(bid128_from_string_010, bid128_from_string, 0u32,  "9.9999999999999999999999999999999995" , 0x3000314dc6448d9338c15b0a00000000u128  , 0x20);
dec_test!(bid128_from_string_011, bid128_from_string, 2u32,  "9.9999999999999999999999999999999995" , 0x3000314dc6448d9338c15b0a00000000u128  , 0x20);
dec_test!(bid128_from_string_012, bid128_from_string, 4u32,  "9.9999999999999999999999999999999995" , 0x3000314dc6448d9338c15b0a00000000u128  , 0x20);
dec_test!(bid128_from_string_013, bid128_from_string, 3u32,  "9.9999999999999999999999999999999995" , 0x2fffed09bead87c0378d8e63ffffffffu128  , 0x20);
dec_test!(bid128_from_string_014, bid128_from_string, 1u32,  "9.9999999999999999999999999999999995" , 0x2fffed09bead87c0378d8e63ffffffffu128  , 0x20);
dec_test!(bid128_from_string_015, bid128_from_string, 0u32,  "1.0000000000000000000000000000000015" , 0x2ffe314dc6448d9338c15b0a00000002u128  , 0x20);
dec_test!(bid128_from_string_016, bid128_from_string, 2u32,  "1.0000000000000000000000000000000015" , 0x2ffe314dc6448d9338c15b0a00000002u128  , 0x20);
dec_test!(bid128_from_string_017, bid128_from_string, 4u32,  "1.0000000000000000000000000000000015" , 0x2ffe314dc6448d9338c15b0a00000002u128  , 0x20);
dec_test!(bid128_from_string_018, bid128_from_string, 3u32,  "1.0000000000000000000000000000000015" , 0x2ffe314dc6448d9338c15b0a00000001u128  , 0x20);
dec_test!(bid128_from_string_019, bid128_from_string, 1u32,  "1.0000000000000000000000000000000015" , 0x2ffe314dc6448d9338c15b0a00000001u128  , 0x20);
dec_test!(bid128_from_string_020, bid128_from_string, 1u32,  "000.0"                                , 0x303e0000000000000000000000000000u128  , 0x00);
dec_test!(bid128_from_string_021, bid128_from_string, 0u32,  "0."                                   , 0x30400000000000000000000000000000u128  , 0x00);
dec_test!(bid128_from_string_022, bid128_from_string, 0u32,  "1."                                   , 0x30400000000000000000000000000001u128  , 0x00);
dec_test!(bid128_from_string_023, bid128_from_string, 0u32,  "1.."                                  , 0x7c000000000000000000000000000000u128  , 0x00);
dec_test!(bid128_from_string_024, bid128_from_string, 0u32,  "1.0000000000000000000000000000000005" , 0x2ffe314dc6448d9338c15b0a00000000u128  , 0x20);
dec_test!(bid128_from_string_025, bid128_from_string, 2u32,  "1.0000000000000000000000000000000005" , 0x2ffe314dc6448d9338c15b0a00000001u128  , 0x20);
dec_test!(bid128_from_string_026, bid128_from_string, 4u32,  "1.0000000000000000000000000000000005" , 0x2ffe314dc6448d9338c15b0a00000001u128  , 0x20);
dec_test!(bid128_from_string_027, bid128_from_string, 3u32,  "1.0000000000000000000000000000000005" , 0x2ffe314dc6448d9338c15b0a00000000u128  , 0x20);
dec_test!(bid128_from_string_028, bid128_from_string, 1u32,  "1.0000000000000000000000000000000005" , 0x2ffe314dc6448d9338c15b0a00000000u128  , 0x20);
dec_test!(bid128_from_string_029, bid128_from_string, 0u32,  "1.00000000000000000000000000000000051", 0x2ffe314dc6448d9338c15b0a00000001u128  , 0x20);
dec_test!(bid128_from_string_030, bid128_from_string, 2u32,  "1.00000000000000000000000000000000051", 0x2ffe314dc6448d9338c15b0a00000001u128  , 0x20);
dec_test!(bid128_from_string_031, bid128_from_string, 4u32,  "1.00000000000000000000000000000000051", 0x2ffe314dc6448d9338c15b0a00000001u128  , 0x20);
dec_test!(bid128_from_string_032, bid128_from_string, 3u32,  "1.00000000000000000000000000000000051", 0x2ffe314dc6448d9338c15b0a00000000u128  , 0x20);
dec_test!(bid128_from_string_033, bid128_from_string, 1u32,  "1.00000000000000000000000000000000051", 0x2ffe314dc6448d9338c15b0a00000000u128  , 0x20);
dec_test!(bid128_from_string_034, bid128_from_string, 0u32,  "1.1E2"                                , 0x3042000000000000000000000000000bu128  , 0x00);
dec_test!(bid128_from_string_035, bid128_from_string, 0u32,  "1.1P2"                                , 0x7c000000000000000000000000000000u128  , 0x00);
dec_test!(bid128_from_string_036, bid128_from_string, 0u32,  "1.1EE"                                , 0x7c000000000000000000000000000000u128  , 0x00);
dec_test!(bid128_from_string_037, bid128_from_string, 0u32,  "1.1P-2"                               , 0x7c000000000000000000000000000000u128  , 0x00);
dec_test!(bid128_from_string_038, bid128_from_string, 0u32,  "1.1E-2E"                              , 0x303a000000000000000000000000000bu128  , 0x00);

dec_test!(bid128_from_string_039, bid128_from_string, 0u32,  "1.9999999999999999999999999990000004999999999999999", 0x2ffe629b8c891b267182b613fff0bdc0u128, 0x20);
dec_test!(bid128_from_string_040, bid128_from_string, 2u32,  "1.9999999999999999999999999990000004999999999999999", 0x2ffe629b8c891b267182b613fff0bdc1u128, 0x20);
dec_test!(bid128_from_string_041, bid128_from_string, 1u32,  "1.9999999999999999999999999990000004999999999999999", 0x2ffe629b8c891b267182b613fff0bdc0u128, 0x20);
dec_test!(bid128_from_string_042, bid128_from_string, 4u32,  "1.9999999999999999999999999990000004999999999999999", 0x2ffe629b8c891b267182b613fff0bdc0u128, 0x20);
dec_test!(bid128_from_string_043, bid128_from_string, 3u32,  "1.9999999999999999999999999990000004999999999999999", 0x2ffe629b8c891b267182b613fff0bdc0u128, 0x20);

dec_test!(bid128_from_string_044, bid128_from_string, 1u32,  "0.0000000000000000000000000000000000000000000000000000000000000001001", 0x2fba00000000000000000000000003e9u128, 0x00);
dec_test!(bid128_from_string_045, bid128_from_string, 1u32,  "0.0000000000000000000000000000000000000000000000000000000000000001001", 0x2fba00000000000000000000000003e9u128, 0x00);

dec_test!(bid128_from_string_046, bid128_from_string, 0u32, "+INF"      , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_047, bid128_from_string, 0u32, "+INFi"     , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_048, bid128_from_string, 0u32, "+NAN"      , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_049, bid128_from_string, 0u32, "+SNAN"     , 0x7e000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_050, bid128_from_string, 0u32, "+SNANi"    , 0x7e000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_051, bid128_from_string, 0u32, "+inf"      , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_052, bid128_from_string, 0u32, "+nan"      , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_053, bid128_from_string, 0u32, "+snan"     , 0x7e000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_054, bid128_from_string, 0u32, "+INFINITY" , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_055, bid128_from_string, 0u32, "+infinity" , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_056, bid128_from_string, 0u32, "+INFiNITY" , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_057, bid128_from_string, 0u32, "+INFINITYi", 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_058, bid128_from_string, 0u32, "INF"       , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_059, bid128_from_string, 0u32, "INFi"      , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_060, bid128_from_string, 0u32, "NAN"       , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_061, bid128_from_string, 0u32, "SNAN"      , 0x7e000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_062, bid128_from_string, 0u32, "SNANi"     , 0x7e000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_063, bid128_from_string, 0u32, "inf"       , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_064, bid128_from_string, 0u32, "nan"       , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_065, bid128_from_string, 0u32, "snan"      , 0x7e000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_066, bid128_from_string, 0u32, "INFINITY"  , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_067, bid128_from_string, 0u32, "infinity"  , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_068, bid128_from_string, 0u32, "INFiNITY"  , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_from_string_069, bid128_from_string, 0u32, "INFINITYi" , 0x7c000000000000000000000000000000u128, 0x00);