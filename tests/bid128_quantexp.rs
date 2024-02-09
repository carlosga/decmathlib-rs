/* // ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* // -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* // -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* // -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_quantexp_001, bid128_quantexp, 0x00000000000000000000000000000001u128, -6176      , 0x00); // -- MinDen
dec_test!(bid128_quantexp_002, bid128_quantexp, 0x0000314DC6448D9338C15B0A00000000u128, -6176      , 0x00); // -- MinNorm
dec_test!(bid128_quantexp_003, bid128_quantexp, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -34        , 0x00); // -- 1-ulp
dec_test!(bid128_quantexp_004, bid128_quantexp, 0x2FFE314DC6448D9338C15B0A00000000u128, -33        , 0x00); // -- Max
dec_test!(bid128_quantexp_005, bid128_quantexp, 0x30400000000000000000000000000001u128, 0          , 0x00); // -- Max
dec_test!(bid128_quantexp_006, bid128_quantexp, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 6111       , 0x00); // -- Max
dec_test!(bid128_quantexp_007, bid128_quantexp, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_008, bid128_quantexp, 0x78000001000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_009, bid128_quantexp, 0x7c000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_010, bid128_quantexp, 0x7c000001000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_011, bid128_quantexp, 0x7cff3fffffffefffffffffffffffffffu128, -2147483648, 0x01);
dec_test!(bid128_quantexp_012, bid128_quantexp, 0x7e000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_013, bid128_quantexp, 0x7e003fffffffefffffffffffffffffffu128, -2147483648, 0x01);
dec_test!(bid128_quantexp_014, bid128_quantexp, 0x7ef00000000000000000000000000001u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_015, bid128_quantexp, 0x80000000000000000000000000000001u128, -6176      , 0x00); // -- -(MinDen)
dec_test!(bid128_quantexp_016, bid128_quantexp, 0x8000314DC6448D9338C15B0A00000000u128, -6176      , 0x00); // -- -(MinNorm)
dec_test!(bid128_quantexp_017, bid128_quantexp, 0xAFFDED09BEAD87C0378D8E63FFFFFFFFu128, -34        , 0x00); // -- -(1-ulp)
dec_test!(bid128_quantexp_018, bid128_quantexp, 0xAFFE314DC6448D9338C15B0A00000000u128, -33        , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_019, bid128_quantexp, 0xB0400000000000000000000000000001u128, 0          , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_020, bid128_quantexp, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 6111       , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_021, bid128_quantexp, 0xf8000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_022, bid128_quantexp, 0xf8000001000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_023, bid128_quantexp, 0xfc000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_024, bid128_quantexp, 0xfc000001000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_025, bid128_quantexp, 0xfe000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_026, bid128_quantexp, 0xfef00000000000000000000000000001u128, -2147483648, 0x01);
dec_test!(bid128_quantexp_027, bid128_quantexp, 0x00000000000000000000000000000001u128, -6176      , 0x00); // -- MinDen
dec_test!(bid128_quantexp_028, bid128_quantexp, 0x0000314DC6448D9338C15B0A00000000u128, -6176      , 0x00); // -- MinNorm
dec_test!(bid128_quantexp_029, bid128_quantexp, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -34        , 0x00); // -- 1-ulp
dec_test!(bid128_quantexp_030, bid128_quantexp, 0x2FFE314DC6448D9338C15B0A00000000u128, -33        , 0x00); // -- Max
dec_test!(bid128_quantexp_031, bid128_quantexp, 0x30400000000000000000000000000001u128, 0          , 0x00); // -- Max
dec_test!(bid128_quantexp_032, bid128_quantexp, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 6111       , 0x00); // -- Max
dec_test!(bid128_quantexp_033, bid128_quantexp, 0x80000000000000000000000000000001u128, -6176      , 0x00); // -- -(MinDen)
dec_test!(bid128_quantexp_034, bid128_quantexp, 0x8000314DC6448D9338C15B0A00000000u128, -6176      , 0x00); // -- -(MinNorm)
dec_test!(bid128_quantexp_035, bid128_quantexp, 0xAFFDED09BEAD87C0378D8E63FFFFFFFFu128, -34        , 0x00); // -- -(1-ulp)
dec_test!(bid128_quantexp_036, bid128_quantexp, 0xAFFE314DC6448D9338C15B0A00000000u128, -33        , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_037, bid128_quantexp, 0xB0400000000000000000000000000001u128, 0          , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_038, bid128_quantexp, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 6111       , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_039, bid128_quantexp, 0x00000000000000000000000000000001u128, -6176      , 0x00); // -- MinDen
dec_test!(bid128_quantexp_040, bid128_quantexp, 0x0000314DC6448D9338C15B0A00000000u128, -6176      , 0x00); // -- MinNorm
dec_test!(bid128_quantexp_041, bid128_quantexp, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -34        , 0x00); // -- 1-ulp
dec_test!(bid128_quantexp_042, bid128_quantexp, 0x2FFE314DC6448D9338C15B0A00000000u128, -33        , 0x00); // -- Max
dec_test!(bid128_quantexp_043, bid128_quantexp, 0x30400000000000000000000000000001u128, 0          , 0x00); // -- Max
dec_test!(bid128_quantexp_044, bid128_quantexp, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 6111       , 0x00); // -- Max
dec_test!(bid128_quantexp_045, bid128_quantexp, 0x80000000000000000000000000000001u128, -6176      , 0x00); // -- -(MinDen)
dec_test!(bid128_quantexp_046, bid128_quantexp, 0x8000314DC6448D9338C15B0A00000000u128, -6176      , 0x00); // -- -(MinNorm)
dec_test!(bid128_quantexp_047, bid128_quantexp, 0xAFFDED09BEAD87C0378D8E63FFFFFFFFu128, -34        , 0x00); // -- -(1-ulp)
dec_test!(bid128_quantexp_048, bid128_quantexp, 0xAFFE314DC6448D9338C15B0A00000000u128, -33        , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_049, bid128_quantexp, 0xB0400000000000000000000000000001u128, 0          , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_050, bid128_quantexp, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 6111       , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_051, bid128_quantexp, 0x00000000000000000000000000000001u128, -6176      , 0x00); // -- MinDen
dec_test!(bid128_quantexp_052, bid128_quantexp, 0x0000314DC6448D9338C15B0A00000000u128, -6176      , 0x00); // -- MinNorm
dec_test!(bid128_quantexp_053, bid128_quantexp, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -34        , 0x00); // -- 1-ulp
dec_test!(bid128_quantexp_054, bid128_quantexp, 0x2FFE314DC6448D9338C15B0A00000000u128, -33        , 0x00); // -- Max
dec_test!(bid128_quantexp_055, bid128_quantexp, 0x30400000000000000000000000000001u128, 0          , 0x00); // -- Max
dec_test!(bid128_quantexp_056, bid128_quantexp, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 6111       , 0x00); // -- Max
dec_test!(bid128_quantexp_057, bid128_quantexp, 0x80000000000000000000000000000001u128, -6176      , 0x00); // -- -(MinDen)
dec_test!(bid128_quantexp_058, bid128_quantexp, 0x8000314DC6448D9338C15B0A00000000u128, -6176      , 0x00); // -- -(MinNorm)
dec_test!(bid128_quantexp_059, bid128_quantexp, 0xAFFDED09BEAD87C0378D8E63FFFFFFFFu128, -34        , 0x00); // -- -(1-ulp)
dec_test!(bid128_quantexp_060, bid128_quantexp, 0xAFFE314DC6448D9338C15B0A00000000u128, -33        , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_061, bid128_quantexp, 0xB0400000000000000000000000000001u128, 0          , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_062, bid128_quantexp, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 6111       , 0x00); // -- -(Max)
dec_test!(bid128_quantexp_063, bid128_quantexp, 0x0001ed09bead87c0378d8e64ffffffffu128, -6176      , 0x00);
dec_test!(bid128_quantexp_064, bid128_quantexp, 0x6003b75d7734cd9e1234567890123456u128, -6169      , 0x00);
dec_test!(bid128_quantexp_065, bid128_quantexp, 0x69dbb75d7734cd9e1234567890123456u128, -1129      , 0x00);
dec_test!(bid128_quantexp_066, bid128_quantexp, 0xe003b75d7734cd9e1234567890123456u128, -6169      , 0x00);
dec_test!(bid128_quantexp_067, bid128_quantexp, 0xe9dbb75d7734cd9e1234567890123456u128, -1129      , 0x00);
