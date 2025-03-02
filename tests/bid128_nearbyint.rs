/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_nearbyint_001, bid128_nearbyint, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_002, bid128_nearbyint, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x30400000000000000000000000000001u128, 0x00); // -- 1-ulp
dec_test!(bid128_nearbyint_003, bid128_nearbyint, 0, 0x2FFE314DC6448D9338C15B0A00000001u128, 0x30400000000000000000000000000001u128, 0x00); // -- 1+ulp
dec_test!(bid128_nearbyint_004, bid128_nearbyint, 0, 0x2FFE49F4A966D45CD522088F00000000u128, 0x30400000000000000000000000000002u128, 0x00); // -- 1.5
dec_test!(bid128_nearbyint_005, bid128_nearbyint, 0, 0x2FFE7B426FAB61F00DE3639900000000u128, 0x30400000000000000000000000000002u128, 0x00); // -- 2.5
dec_test!(bid128_nearbyint_006, bid128_nearbyint, 0, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // -- Max
dec_test!(bid128_nearbyint_007, bid128_nearbyint, 0, 0x6003b75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_008, bid128_nearbyint, 0, 0x69dbb75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_009, bid128_nearbyint, 0, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_010, bid128_nearbyint, 0, 0x78000001000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_011, bid128_nearbyint, 0, 0x7c000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_012, bid128_nearbyint, 0, 0x7c000001000000000000000000000000u128, 0x7c000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_013, bid128_nearbyint, 0, 0x7cff3fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_014, bid128_nearbyint, 0, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_015, bid128_nearbyint, 0, 0x7e003fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_016, bid128_nearbyint, 0, 0x7ef00000000000000000000000000001u128, 0x7c000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nearbyint_017, bid128_nearbyint, 0, 0xAFFDED09BEAD87C0378D8E63FFFFFFFFu128, 0xB0400000000000000000000000000001u128, 0x00); // -- -(1-ulp)
dec_test!(bid128_nearbyint_018, bid128_nearbyint, 0, 0xAFFE314DC6448D9338C15B0A00000001u128, 0xB0400000000000000000000000000001u128, 0x00); // -- -(1+ulp)
dec_test!(bid128_nearbyint_019, bid128_nearbyint, 0, 0xAFFE49F4A966D45CD522088F00000000u128, 0xB0400000000000000000000000000002u128, 0x00); // -- -(1.5)
dec_test!(bid128_nearbyint_020, bid128_nearbyint, 0, 0xAFFE7B426FAB61F00DE3639900000000u128, 0xB0400000000000000000000000000002u128, 0x00); // -- -(2.5)
dec_test!(bid128_nearbyint_021, bid128_nearbyint, 0, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // -- -(Max)
dec_test!(bid128_nearbyint_022, bid128_nearbyint, 0, 0xe003b75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_023, bid128_nearbyint, 0, 0xe9dbb75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_024, bid128_nearbyint, 0, 0xf8000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_025, bid128_nearbyint, 0, 0xf8000001000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_026, bid128_nearbyint, 0, 0xfc000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_027, bid128_nearbyint, 0, 0xfc000001000000000000000000000000u128, 0xfc000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_028, bid128_nearbyint, 0, 0xfe000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_029, bid128_nearbyint, 0, 0xfef00000000000000000000000000001u128, 0xfc000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nearbyint_030, bid128_nearbyint, 1, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_031, bid128_nearbyint, 1, 0x2FFE314DC6448D9338C15B0A00000001u128, 0x30400000000000000000000000000001u128, 0x00); // -- 1+ulp
dec_test!(bid128_nearbyint_032, bid128_nearbyint, 1, 0x2FFE49F4A966D45CD522088F00000000u128, 0x30400000000000000000000000000001u128, 0x00); // -- 1.5
dec_test!(bid128_nearbyint_033, bid128_nearbyint, 1, 0x2FFE7B426FAB61F00DE3639900000000u128, 0x30400000000000000000000000000002u128, 0x00); // -- 2.5
dec_test!(bid128_nearbyint_034, bid128_nearbyint, 1, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // -- Max
dec_test!(bid128_nearbyint_035, bid128_nearbyint, 1, 0x6003b75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_036, bid128_nearbyint, 1, 0x69dbb75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_037, bid128_nearbyint, 1, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_038, bid128_nearbyint, 1, 0x78000001000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_039, bid128_nearbyint, 1, 0x7c000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_040, bid128_nearbyint, 1, 0x7c000001000000000000000000000000u128, 0x7c000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_041, bid128_nearbyint, 1, 0x7cff3fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_042, bid128_nearbyint, 1, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_043, bid128_nearbyint, 1, 0x7e003fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_044, bid128_nearbyint, 1, 0x7ef00000000000000000000000000001u128, 0x7c000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nearbyint_045, bid128_nearbyint, 1, 0x80000000000000000000000000000001u128, 0xB0400000000000000000000000000001u128, 0x00); // -- -(MinDen)
dec_test!(bid128_nearbyint_046, bid128_nearbyint, 1, 0x8000314DC6448D9338C15B0A00000000u128, 0xB0400000000000000000000000000001u128, 0x00); // -- -(MinNorm)
dec_test!(bid128_nearbyint_047, bid128_nearbyint, 1, 0xAFFDED09BEAD87C0378D8E63FFFFFFFFu128, 0xB0400000000000000000000000000001u128, 0x00); // -- -(1-ulp)
dec_test!(bid128_nearbyint_048, bid128_nearbyint, 1, 0xAFFE314DC6448D9338C15B0A00000001u128, 0xB0400000000000000000000000000002u128, 0x00); // -- -(1+ulp)
dec_test!(bid128_nearbyint_049, bid128_nearbyint, 1, 0xAFFE49F4A966D45CD522088F00000000u128, 0xB0400000000000000000000000000002u128, 0x00); // -- -(1.5)
dec_test!(bid128_nearbyint_050, bid128_nearbyint, 1, 0xAFFE7B426FAB61F00DE3639900000000u128, 0xB0400000000000000000000000000003u128, 0x00); // -- -(2.5)
dec_test!(bid128_nearbyint_051, bid128_nearbyint, 1, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // -- -(Max)
dec_test!(bid128_nearbyint_052, bid128_nearbyint, 1, 0xe003b75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_053, bid128_nearbyint, 1, 0xe9dbb75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_054, bid128_nearbyint, 1, 0xf8000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_055, bid128_nearbyint, 1, 0xf8000001000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_056, bid128_nearbyint, 1, 0xfc000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_057, bid128_nearbyint, 1, 0xfc000001000000000000000000000000u128, 0xfc000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_058, bid128_nearbyint, 1, 0xfe000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_059, bid128_nearbyint, 1, 0xfef00000000000000000000000000001u128, 0xfc000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nearbyint_060, bid128_nearbyint, 2, 0x00000000000000000000000000000001u128, 0x30400000000000000000000000000001u128, 0x00); // -- MinDen
dec_test!(bid128_nearbyint_061, bid128_nearbyint, 2, 0x0000314DC6448D9338C15B0A00000000u128, 0x30400000000000000000000000000001u128, 0x00); // -- MinNorm
dec_test!(bid128_nearbyint_062, bid128_nearbyint, 2, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_063, bid128_nearbyint, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x30400000000000000000000000000001u128, 0x00); // -- 1-ulp
dec_test!(bid128_nearbyint_064, bid128_nearbyint, 2, 0x2FFE314DC6448D9338C15B0A00000001u128, 0x30400000000000000000000000000002u128, 0x00); // -- 1+ulp
dec_test!(bid128_nearbyint_065, bid128_nearbyint, 2, 0x2FFE49F4A966D45CD522088F00000000u128, 0x30400000000000000000000000000002u128, 0x00); // -- 1.5
dec_test!(bid128_nearbyint_066, bid128_nearbyint, 2, 0x2FFE7B426FAB61F00DE3639900000000u128, 0x30400000000000000000000000000003u128, 0x00); // -- 2.5
dec_test!(bid128_nearbyint_067, bid128_nearbyint, 2, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // -- Max
dec_test!(bid128_nearbyint_068, bid128_nearbyint, 2, 0x6003b75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_069, bid128_nearbyint, 2, 0x69dbb75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_070, bid128_nearbyint, 2, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_071, bid128_nearbyint, 2, 0x78000001000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_072, bid128_nearbyint, 2, 0x7c000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_073, bid128_nearbyint, 2, 0x7c000001000000000000000000000000u128, 0x7c000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_074, bid128_nearbyint, 2, 0x7cff3fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_075, bid128_nearbyint, 2, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_076, bid128_nearbyint, 2, 0x7e003fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_077, bid128_nearbyint, 2, 0x7ef00000000000000000000000000001u128, 0x7c000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nearbyint_078, bid128_nearbyint, 2, 0xAFFE314DC6448D9338C15B0A00000001u128, 0xB0400000000000000000000000000001u128, 0x00); // -- -(1+ulp)
dec_test!(bid128_nearbyint_079, bid128_nearbyint, 2, 0xAFFE49F4A966D45CD522088F00000000u128, 0xB0400000000000000000000000000001u128, 0x00); // -- -(1.5)
dec_test!(bid128_nearbyint_080, bid128_nearbyint, 2, 0xAFFE7B426FAB61F00DE3639900000000u128, 0xB0400000000000000000000000000002u128, 0x00); // -- -(2.5)
dec_test!(bid128_nearbyint_081, bid128_nearbyint, 2, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // -- -(Max)
dec_test!(bid128_nearbyint_082, bid128_nearbyint, 2, 0xe003b75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_083, bid128_nearbyint, 2, 0xe9dbb75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_084, bid128_nearbyint, 2, 0xf8000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_085, bid128_nearbyint, 2, 0xf8000001000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_086, bid128_nearbyint, 2, 0xfc000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_087, bid128_nearbyint, 2, 0xfc000001000000000000000000000000u128, 0xfc000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_088, bid128_nearbyint, 2, 0xfe000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_089, bid128_nearbyint, 2, 0xfef00000000000000000000000000001u128, 0xfc000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nearbyint_090, bid128_nearbyint, 3, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_091, bid128_nearbyint, 3, 0x2FFE314DC6448D9338C15B0A00000001u128, 0x30400000000000000000000000000001u128, 0x00); // -- 1+ulp
dec_test!(bid128_nearbyint_092, bid128_nearbyint, 3, 0x2FFE49F4A966D45CD522088F00000000u128, 0x30400000000000000000000000000001u128, 0x00); // -- 1.5
dec_test!(bid128_nearbyint_093, bid128_nearbyint, 3, 0x2FFE7B426FAB61F00DE3639900000000u128, 0x30400000000000000000000000000002u128, 0x00); // -- 2.5
dec_test!(bid128_nearbyint_094, bid128_nearbyint, 3, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // -- Max
dec_test!(bid128_nearbyint_095, bid128_nearbyint, 3, 0x6003b75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_096, bid128_nearbyint, 3, 0x69dbb75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_097, bid128_nearbyint, 3, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_098, bid128_nearbyint, 3, 0x78000001000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_099, bid128_nearbyint, 3, 0x7c000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_100, bid128_nearbyint, 3, 0x7c000001000000000000000000000000u128, 0x7c000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_101, bid128_nearbyint, 3, 0x7cff3fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_102, bid128_nearbyint, 3, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_103, bid128_nearbyint, 3, 0x7e003fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_104, bid128_nearbyint, 3, 0x7ef00000000000000000000000000001u128, 0x7c000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nearbyint_105, bid128_nearbyint, 3, 0xAFFE314DC6448D9338C15B0A00000001u128, 0xB0400000000000000000000000000001u128, 0x00); // -- -(1+ulp)
dec_test!(bid128_nearbyint_106, bid128_nearbyint, 3, 0xAFFE49F4A966D45CD522088F00000000u128, 0xB0400000000000000000000000000001u128, 0x00); // -- -(1.5)
dec_test!(bid128_nearbyint_107, bid128_nearbyint, 3, 0xAFFE7B426FAB61F00DE3639900000000u128, 0xB0400000000000000000000000000002u128, 0x00); // -- -(2.5)
dec_test!(bid128_nearbyint_108, bid128_nearbyint, 3, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // -- -(Max)
dec_test!(bid128_nearbyint_109, bid128_nearbyint, 3, 0xe003b75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_110, bid128_nearbyint, 3, 0xe9dbb75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_111, bid128_nearbyint, 3, 0xf8000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_112, bid128_nearbyint, 3, 0xf8000001000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_113, bid128_nearbyint, 3, 0xfc000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_114, bid128_nearbyint, 3, 0xfc000001000000000000000000000000u128, 0xfc000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_115, bid128_nearbyint, 3, 0xfe000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_116, bid128_nearbyint, 3, 0xfef00000000000000000000000000001u128, 0xfc000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nearbyint_117, bid128_nearbyint, 4, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_118, bid128_nearbyint, 4, 0x6003b75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_119, bid128_nearbyint, 4, 0x69dbb75d7734cd9e1234567890123456u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_120, bid128_nearbyint, 4, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_121, bid128_nearbyint, 4, 0x78000001000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_122, bid128_nearbyint, 4, 0x7c000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_123, bid128_nearbyint, 4, 0x7c000001000000000000000000000000u128, 0x7c000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_124, bid128_nearbyint, 4, 0x7cff3fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_125, bid128_nearbyint, 4, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_126, bid128_nearbyint, 4, 0x7e003fffffffefffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_127, bid128_nearbyint, 4, 0x7ef00000000000000000000000000001u128, 0x7c000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nearbyint_128, bid128_nearbyint, 4, 0xe003b75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_129, bid128_nearbyint, 4, 0xe9dbb75d7734cd9e1234567890123456u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_130, bid128_nearbyint, 4, 0xf8000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_131, bid128_nearbyint, 4, 0xf8000001000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_132, bid128_nearbyint, 4, 0xfc000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_133, bid128_nearbyint, 4, 0xfc000001000000000000000000000000u128, 0xfc000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_134, bid128_nearbyint, 4, 0xfe000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nearbyint_135, bid128_nearbyint, 4, 0xfef00000000000000000000000000001u128, 0xfc000000000000000000000000000001u128, 0x01);

dec_test!(bid128_nearbyint_136, bid128_nearbyint, 0, 0x00000000000000000000000000000001u128, 0x30400000000000000000000000000000u128, 0x00); // -- MinDen
dec_test!(bid128_nearbyint_137, bid128_nearbyint, 0, 0x0000314DC6448D9338C15B0A00000000u128, 0x30400000000000000000000000000000u128, 0x00); // -- MinNorm
dec_test!(bid128_nearbyint_138, bid128_nearbyint, 0, 0x80000000000000000000000000000001u128, 0xb0400000000000000000000000000000u128, 0x00); // -- -(MinDen)
dec_test!(bid128_nearbyint_139, bid128_nearbyint, 0, 0x8000314DC6448D9338C15B0A00000000u128, 0xb0400000000000000000000000000000u128, 0x00); // -- -(MinNorm)
dec_test!(bid128_nearbyint_140, bid128_nearbyint, 1, 0x00000000000000000000000000000001u128, 0x30400000000000000000000000000000u128, 0x00); // -- MinDen
dec_test!(bid128_nearbyint_141, bid128_nearbyint, 1, 0x0000314DC6448D9338C15B0A00000000u128, 0x30400000000000000000000000000000u128, 0x00); // -- MinNorm
dec_test!(bid128_nearbyint_142, bid128_nearbyint, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x30400000000000000000000000000000u128, 0x00); // -- 1-ulp
dec_test!(bid128_nearbyint_143, bid128_nearbyint, 2, 0x80000000000000000000000000000001u128, 0xb0400000000000000000000000000000u128, 0x00); // -- -(MinDen)
dec_test!(bid128_nearbyint_144, bid128_nearbyint, 2, 0x8000314DC6448D9338C15B0A00000000u128, 0xb0400000000000000000000000000000u128, 0x00); // -- -(MinNorm)
dec_test!(bid128_nearbyint_145, bid128_nearbyint, 2, 0xAFFDED09BEAD87C0378D8E63FFFFFFFFu128, 0xb0400000000000000000000000000000u128, 0x00); // -- -(1-ulp)
dec_test!(bid128_nearbyint_146, bid128_nearbyint, 3, 0x00000000000000000000000000000001u128, 0x30400000000000000000000000000000u128, 0x00); // -- MinDen
dec_test!(bid128_nearbyint_147, bid128_nearbyint, 3, 0x0000314DC6448D9338C15B0A00000000u128, 0x30400000000000000000000000000000u128, 0x00); // -- MinNorm
dec_test!(bid128_nearbyint_148, bid128_nearbyint, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x30400000000000000000000000000000u128, 0x00); // -- 1-ulp
dec_test!(bid128_nearbyint_149, bid128_nearbyint, 3, 0x80000000000000000000000000000001u128, 0xb0400000000000000000000000000000u128, 0x00); // -- -(MinDen)
dec_test!(bid128_nearbyint_150, bid128_nearbyint, 3, 0x8000314DC6448D9338C15B0A00000000u128, 0xb0400000000000000000000000000000u128, 0x00); // -- -(MinNorm)
dec_test!(bid128_nearbyint_151, bid128_nearbyint, 3, 0xAFFDED09BEAD87C0378D8E63FFFFFFFFu128, 0xb0400000000000000000000000000000u128, 0x00); // -- -(1-ulp)
dec_test!(bid128_nearbyint_152, bid128_nearbyint, 0, 0xddfdeffd68b27fffd9c36ddd5f298961u128, 0xddfc0000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_153, bid128_nearbyint, 4, 0xb0111603720400e20008c00080000400u128, 0xb040000000000000000000015018ff78u128, 0x00);
dec_test!(bid128_nearbyint_154, bid128_nearbyint, 4, 0xb00080840700200864e10ba6e1649cb2u128, 0xb040000000000000000000000000001au128, 0x00);
dec_test!(bid128_nearbyint_155, bid128_nearbyint, 0, 0x301ca5142f3788c55c012bbc0837273eu128, 0x3040000000000000000be52ace9090b3u128, 0x00);
dec_test!(bid128_nearbyint_156, bid128_nearbyint, 4, 0x301ca5142f3788c55c012bbc0837273eu128, 0x3040000000000000000be52ace9090b3u128, 0x00);
dec_test!(bid128_nearbyint_157, bid128_nearbyint, 2, 0x301ca5142f3788c55c012bbc0837273eu128, 0x3040000000000000000be52ace9090b4u128, 0x00);
dec_test!(bid128_nearbyint_158, bid128_nearbyint, 1, 0x301ca5142f3788c55c012bbc0837273eu128, 0x3040000000000000000be52ace9090b3u128, 0x00);
dec_test!(bid128_nearbyint_159, bid128_nearbyint, 3, 0x301ca5142f3788c55c012bbc0837273eu128, 0x3040000000000000000be52ace9090b3u128, 0x00);
dec_test!(bid128_nearbyint_160, bid128_nearbyint, 1, 0xb0398edb5c5da0f043cdf3ab03350233u128, 0xb040000a35f2ccbd425caac03240cc85u128, 0x00);
dec_test!(bid128_nearbyint_161, bid128_nearbyint, 0, 0x40000000000000008840384031402320u128, 0x40000000000000008840384031402320u128, 0x00);
dec_test!(bid128_nearbyint_162, bid128_nearbyint, 2, 0x300000a00000000095d55e9bcd75413fu128, 0x30400000000000000000000000000001u128, 0x00);
dec_test!(bid128_nearbyint_163, bid128_nearbyint, 1, 0x300000a00000000095d55e9bcd75413fu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_164, bid128_nearbyint, 3, 0x300000a00000000095d55e9bcd75413fu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_165, bid128_nearbyint, 0, 0x40000000000000000004060068224000u128, 0x40000000000000000004060068224000u128, 0x00);
dec_test!(bid128_nearbyint_166, bid128_nearbyint, 0, 0x3000000948044a63ffffbfd75fdffffbu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_167, bid128_nearbyint, 4, 0x3000000948044a63ffffbfd75fdffffbu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_168, bid128_nearbyint, 2, 0xb00001509c040b040000000000000000u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_169, bid128_nearbyint, 1, 0xb00001509c040b040000000000000000u128, 0xb0400000000000000000000000000001u128, 0x00);
dec_test!(bid128_nearbyint_170, bid128_nearbyint, 0, 0x303c97921e7c48ee4c9c76f30b7f2f34u128, 0x30400184056cc34a102048de9727abfeu128, 0x00);
dec_test!(bid128_nearbyint_171, bid128_nearbyint, 4, 0x303c97921e7c48ee4c9c76f30b7f2f34u128, 0x30400184056cc34a102048de9727abfeu128, 0x00);
dec_test!(bid128_nearbyint_172, bid128_nearbyint, 2, 0x303c97921e7c48ee4c9c76f30b7f2f34u128, 0x30400184056cc34a102048de9727abfeu128, 0x00);
dec_test!(bid128_nearbyint_173, bid128_nearbyint, 1, 0x303c97921e7c48ee4c9c76f30b7f2f34u128, 0x30400184056cc34a102048de9727abfdu128, 0x00);
dec_test!(bid128_nearbyint_174, bid128_nearbyint, 3, 0x303c97921e7c48ee4c9c76f30b7f2f34u128, 0x30400184056cc34a102048de9727abfdu128, 0x00);
dec_test!(bid128_nearbyint_175, bid128_nearbyint, 1, 0xb03a5e58ed451e3d2c2bd145a13dfa0au128, 0xb04000182726368ee8bf8825308b94fdu128, 0x00);
dec_test!(bid128_nearbyint_176, bid128_nearbyint, 0, 0xb03f894d76f5d39f0161600c34130005u128, 0xb04027548be561f64cf023346b9b8000u128, 0x00);
dec_test!(bid128_nearbyint_177, bid128_nearbyint, 0, 0x7c00314dc6448d9338c15b1000000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nearbyint_178, bid128_nearbyint, 0, "18446744073709551616500E-3"          , "18446744073709551616E+0"             , 0x00);
dec_test!(bid128_nearbyint_179, bid128_nearbyint, 0, "91E-1"                               , 9                                     , 0x00);
dec_test!(bid128_nearbyint_180, bid128_nearbyint, 1, "91E-1"                               , 9                                     , 0x00);
dec_test!(bid128_nearbyint_181, bid128_nearbyint, 2, "91E-1"                               , 10                                    , 0x00);
dec_test!(bid128_nearbyint_182, bid128_nearbyint, 3, "91E-1"                               , 9                                     , 0x00);
dec_test!(bid128_nearbyint_183, bid128_nearbyint, 0, "90001E-4"                            , 9                                     , 0x00);
dec_test!(bid128_nearbyint_184, bid128_nearbyint, 1, "90001E-4"                            , 9                                     , 0x00);
dec_test!(bid128_nearbyint_185, bid128_nearbyint, 2, "90001E-4"                            , 10                                    , 0x00);
dec_test!(bid128_nearbyint_186, bid128_nearbyint, 3, "90001E-4"                            , 9                                     , 0x00);
dec_test!(bid128_nearbyint_187, bid128_nearbyint, 1, "18446744073709551616500E-3"          , "18446744073709551616E+0"             , 0x00);
dec_test!(bid128_nearbyint_188, bid128_nearbyint, 2, "18446744073709551616500E-3"          , "18446744073709551617E+0"             , 0x00);
dec_test!(bid128_nearbyint_189, bid128_nearbyint, 3, "18446744073709551616500E-3"          , "18446744073709551616E+0"             , 0x00);
dec_test!(bid128_nearbyint_190, bid128_nearbyint, 0, "205000E-4"                           , 20                                    , 0x00);
dec_test!(bid128_nearbyint_191, bid128_nearbyint, 4, "18446744073709551616500E-3"          , "18446744073709551617E+0"             , 0x00);
dec_test!(bid128_nearbyint_192, bid128_nearbyint, 0, "18446744073709551616500E-3"          , 0x30400000000000010000000000000000u128, 0x00);
