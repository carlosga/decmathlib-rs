/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_llrint_001, bid128_llrint, 0, 0x00000000000000000000000000000001u128, 0                    , 0x20); // -- MinDen
dec_test!(bid128_llrint_002, bid128_llrint, 0, 0x0000314DC6448D9338C15B09FFFFFFFFu128, 0                    , 0x20); // -- MinNorm-ulp
dec_test!(bid128_llrint_003, bid128_llrint, 0, 0x0000314DC6448D9338C15B0A00000000u128, 0                    , 0x20); // -- MinNorm
dec_test!(bid128_llrint_004, bid128_llrint, 0, 0x0000314DC6448D9338C15B0A00000001u128, 0                    , 0x20); // -- MinNorm+ulp
dec_test!(bid128_llrint_005, bid128_llrint, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                    , 0x00);
dec_test!(bid128_llrint_006, bid128_llrint, 0, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808 , 0x01); // -- Max
dec_test!(bid128_llrint_007, bid128_llrint, 0, 0x6003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_008, bid128_llrint, 0, 0x69dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_009, bid128_llrint, 0, 0x78000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_010, bid128_llrint, 0, 0x78000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_011, bid128_llrint, 0, 0x7c000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_012, bid128_llrint, 0, 0x7c000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_013, bid128_llrint, 0, 0x7cff3fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_014, bid128_llrint, 0, 0x7e000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_015, bid128_llrint, 0, 0x7e003fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_016, bid128_llrint, 0, 0x7ef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_017, bid128_llrint, 0, 0x80000000000000000000000000000001u128, 0                    , 0x20); // -- -(MinDen)
dec_test!(bid128_llrint_018, bid128_llrint, 0, 0x8000314DC6448D9338C15B09FFFFFFFFu128, 0                    , 0x20); // -- -(MinNorm-ulp)
dec_test!(bid128_llrint_019, bid128_llrint, 0, 0x8000314DC6448D9338C15B0A00000000u128, 0                    , 0x20); // -- -(MinNorm)
dec_test!(bid128_llrint_020, bid128_llrint, 0, 0x8000314DC6448D9338C15B0A00000001u128, 0                    , 0x20); // -- -(MinNorm+ulp)
dec_test!(bid128_llrint_021, bid128_llrint, 0, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808 , 0x01); // -- -(Max)
dec_test!(bid128_llrint_022, bid128_llrint, 0, 0xe003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_023, bid128_llrint, 0, 0xe9dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_024, bid128_llrint, 0, 0xf8000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_025, bid128_llrint, 0, 0xf8000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_026, bid128_llrint, 0, 0xfc000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_027, bid128_llrint, 0, 0xfc000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_028, bid128_llrint, 0, 0xfe000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_029, bid128_llrint, 0, 0xfef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_030, bid128_llrint, 1, 0x00000000000000000000000000000001u128, 0                    , 0x20); // -- MinDen
dec_test!(bid128_llrint_031, bid128_llrint, 1, 0x0000314DC6448D9338C15B09FFFFFFFFu128, 0                    , 0x20); // -- MinNorm-ulp
dec_test!(bid128_llrint_032, bid128_llrint, 1, 0x0000314DC6448D9338C15B0A00000000u128, 0                    , 0x20); // -- MinNorm
dec_test!(bid128_llrint_033, bid128_llrint, 1, 0x0000314DC6448D9338C15B0A00000001u128, 0                    , 0x20); // -- MinNorm+ulp
dec_test!(bid128_llrint_034, bid128_llrint, 1, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                    , 0x00);
dec_test!(bid128_llrint_035, bid128_llrint, 1, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808 , 0x01); // -- Max
dec_test!(bid128_llrint_036, bid128_llrint, 1, 0x6003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_037, bid128_llrint, 1, 0x69dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_038, bid128_llrint, 1, 0x78000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_039, bid128_llrint, 1, 0x78000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_040, bid128_llrint, 1, 0x7c000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_041, bid128_llrint, 1, 0x7c000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_042, bid128_llrint, 1, 0x7cff3fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_043, bid128_llrint, 1, 0x7e000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_044, bid128_llrint, 1, 0x7e003fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_045, bid128_llrint, 1, 0x7ef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_046, bid128_llrint, 1, 0x80000000000000000000000000000001u128, -1                   , 0x20); // -- -(MinDen)
dec_test!(bid128_llrint_047, bid128_llrint, 1, 0x8000314DC6448D9338C15B09FFFFFFFFu128, -1                   , 0x20); // -- -(MinNorm-ulp)
dec_test!(bid128_llrint_048, bid128_llrint, 1, 0x8000314DC6448D9338C15B0A00000000u128, -1                   , 0x20); // -- -(MinNorm)
dec_test!(bid128_llrint_049, bid128_llrint, 1, 0x8000314DC6448D9338C15B0A00000001u128, -1                   , 0x20); // -- -(MinNorm+ulp)
dec_test!(bid128_llrint_050, bid128_llrint, 1, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808 , 0x01); // -- -(Max)
dec_test!(bid128_llrint_051, bid128_llrint, 1, 0xe003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_052, bid128_llrint, 1, 0xe9dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_053, bid128_llrint, 1, 0xf8000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_054, bid128_llrint, 1, 0xf8000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_055, bid128_llrint, 1, 0xfc000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_056, bid128_llrint, 1, 0xfc000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_057, bid128_llrint, 1, 0xfe000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_058, bid128_llrint, 1, 0xfef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_059, bid128_llrint, 2, 0x00000000000000000000000000000001u128, 1                    , 0x20); // -- MinDen
dec_test!(bid128_llrint_060, bid128_llrint, 2, 0x0000314DC6448D9338C15B09FFFFFFFFu128, 1                    , 0x20); // -- MinNorm-ulp
dec_test!(bid128_llrint_061, bid128_llrint, 2, 0x0000314DC6448D9338C15B0A00000000u128, 1                    , 0x20); // -- MinNorm
dec_test!(bid128_llrint_062, bid128_llrint, 2, 0x0000314DC6448D9338C15B0A00000001u128, 1                    , 0x20); // -- MinNorm+ulp
dec_test!(bid128_llrint_063, bid128_llrint, 2, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                    , 0x00);
dec_test!(bid128_llrint_064, bid128_llrint, 2, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808 , 0x01); //-- Max
dec_test!(bid128_llrint_065, bid128_llrint, 2, 0x6003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_066, bid128_llrint, 2, 0x69dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_067, bid128_llrint, 2, 0x78000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_068, bid128_llrint, 2, 0x78000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_069, bid128_llrint, 2, 0x7c000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_070, bid128_llrint, 2, 0x7c000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_071, bid128_llrint, 2, 0x7cff3fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_072, bid128_llrint, 2, 0x7e000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_073, bid128_llrint, 2, 0x7e003fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_074, bid128_llrint, 2, 0x7ef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_075, bid128_llrint, 2, 0x80000000000000000000000000000001u128, 0                    , 0x20); // -- -(MinDen)
dec_test!(bid128_llrint_076, bid128_llrint, 2, 0x8000314DC6448D9338C15B09FFFFFFFFu128, 0                    , 0x20); // -- -(MinNorm-ulp)
dec_test!(bid128_llrint_077, bid128_llrint, 2, 0x8000314DC6448D9338C15B0A00000000u128, 0                    , 0x20); // -- -(MinNorm)
dec_test!(bid128_llrint_078, bid128_llrint, 2, 0x8000314DC6448D9338C15B0A00000001u128, 0                    , 0x20); // -- -(MinNorm+ulp)
dec_test!(bid128_llrint_079, bid128_llrint, 2, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808 , 0x01); // -- -(Max)
dec_test!(bid128_llrint_080, bid128_llrint, 2, 0xe003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_081, bid128_llrint, 2, 0xe9dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_082, bid128_llrint, 2, 0xf8000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_083, bid128_llrint, 2, 0xf8000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_084, bid128_llrint, 2, 0xfc000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_085, bid128_llrint, 2, 0xfc000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_086, bid128_llrint, 2, 0xfe000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_087, bid128_llrint, 2, 0xfef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_088, bid128_llrint, 3, 0x00000000000000000000000000000001u128, 0                    , 0x20); // -- MinDen
dec_test!(bid128_llrint_089, bid128_llrint, 3, 0x0000314DC6448D9338C15B09FFFFFFFFu128, 0                    , 0x20); // -- MinNorm-ulp
dec_test!(bid128_llrint_090, bid128_llrint, 3, 0x0000314DC6448D9338C15B0A00000000u128, 0                    , 0x20); // -- MinNorm
dec_test!(bid128_llrint_091, bid128_llrint, 3, 0x0000314DC6448D9338C15B0A00000001u128, 0                    , 0x20); // -- MinNorm+ulp
dec_test!(bid128_llrint_092, bid128_llrint, 3, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                    , 0x00);
dec_test!(bid128_llrint_093, bid128_llrint, 3, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808 , 0x01); // -- Max
dec_test!(bid128_llrint_094, bid128_llrint, 3, 0x6003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_095, bid128_llrint, 3, 0x69dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_096, bid128_llrint, 3, 0x78000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_097, bid128_llrint, 3, 0x78000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_098, bid128_llrint, 3, 0x7c000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_099, bid128_llrint, 3, 0x7c000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_100, bid128_llrint, 3, 0x7cff3fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_101, bid128_llrint, 3, 0x7e000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_102, bid128_llrint, 3, 0x7e003fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_103, bid128_llrint, 3, 0x7ef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_104, bid128_llrint, 3, 0x80000000000000000000000000000001u128, 0                    , 0x20); // -- -(MinDen)
dec_test!(bid128_llrint_105, bid128_llrint, 3, 0x8000314DC6448D9338C15B09FFFFFFFFu128, 0                    , 0x20); // -- -(MinNorm-ulp)
dec_test!(bid128_llrint_106, bid128_llrint, 3, 0x8000314DC6448D9338C15B0A00000000u128, 0                    , 0x20); // -- -(MinNorm)
dec_test!(bid128_llrint_107, bid128_llrint, 3, 0x8000314DC6448D9338C15B0A00000001u128, 0                    , 0x20); // -- -(MinNorm+ulp)
dec_test!(bid128_llrint_108, bid128_llrint, 3, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808 , 0x01); //-- -(Max)
dec_test!(bid128_llrint_109, bid128_llrint, 3, 0xe003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_110, bid128_llrint, 3, 0xe9dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_111, bid128_llrint, 3, 0xf8000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_112, bid128_llrint, 3, 0xf8000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_113, bid128_llrint, 3, 0xfc000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_114, bid128_llrint, 3, 0xfc000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_115, bid128_llrint, 3, 0xfe000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_116, bid128_llrint, 3, 0xfef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_117, bid128_llrint, 4, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                    , 0x00);
dec_test!(bid128_llrint_118, bid128_llrint, 4, 0x6003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_119, bid128_llrint, 4, 0x69dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_120, bid128_llrint, 4, 0x78000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_121, bid128_llrint, 4, 0x78000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_122, bid128_llrint, 4, 0x7c000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_123, bid128_llrint, 4, 0x7c000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_124, bid128_llrint, 4, 0x7cff3fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_125, bid128_llrint, 4, 0x7e000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_126, bid128_llrint, 4, 0x7e003fffffffefffffffffffffffffffu128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_127, bid128_llrint, 4, 0x7ef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_128, bid128_llrint, 4, 0xe003b75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_129, bid128_llrint, 4, 0xe9dbb75d7734cd9e1234567890123456u128, 0                    , 0x00);
dec_test!(bid128_llrint_130, bid128_llrint, 4, 0xf8000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_131, bid128_llrint, 4, 0xf8000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_132, bid128_llrint, 4, 0xfc000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_133, bid128_llrint, 4, 0xfc000001000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_134, bid128_llrint, 4, 0xfe000000000000000000000000000000u128, -9223372036854775808 , 0x01);
dec_test!(bid128_llrint_135, bid128_llrint, 4, 0xfef00000000000000000000000000001u128, -9223372036854775808 , 0x01);
