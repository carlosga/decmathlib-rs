/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_ilogb_001, bid128_ilogb, 0x00000000000000000000000000000000u128, -2147483648         , 0x01); //=0.0000000000
dec_test!(bid128_ilogb_002, bid128_ilogb, 0x00000000000000000000000000000001u128, -6176               , 0x00); //=0.0000000000 -- MinDen
dec_test!(bid128_ilogb_003, bid128_ilogb, 0x0001ed09bead87c0378d8e62ffffffffu128, -6143               , 0x00);
dec_test!(bid128_ilogb_004, bid128_ilogb, 0x0001ed09bead87c0378d8e64ffffffffu128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_005, bid128_ilogb, 0x00420000000000000000000000000001u128, -6143               , 0x00); //=0.0000000000 -- MinNorm
dec_test!(bid128_ilogb_006, bid128_ilogb, 0x04920000000000000000000000000000u128, 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_007, bid128_ilogb, "-0"                                  , 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_008, bid128_ilogb,  "0"                                  , 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_009, bid128_ilogb, 0x15ae0000000000000000000000000000u128, 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_010, bid128_ilogb, 0x1a380000000000000000000000000000u128, 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_011, bid128_ilogb, 0x2fd00000000000000000000000000000u128, 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_012, bid128_ilogb, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x00); //=0.0000000000 -- 1-ulp
dec_test!(bid128_ilogb_013, bid128_ilogb, 0x4024471a24b9c094eb33e24e51e6e393u128, 0x0000813i32        , 0x00);
dec_test!(bid128_ilogb_014, bid128_ilogb, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 6144                , 0x00); //=0.0000000000 -- Max
dec_test!(bid128_ilogb_015, bid128_ilogb, 0x6003b75d7734cd9e1234567890123456u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_016, bid128_ilogb, 0x69dbb75d7734cd9e1234567890123456u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_017, bid128_ilogb, 0x78000000000000000000000000000000u128, 2147483647          , 0x01);
dec_test!(bid128_ilogb_018, bid128_ilogb, 0x78000000000000000000000000000000u128, 0x7fffffffi32       , 0x01);
dec_test!(bid128_ilogb_019, bid128_ilogb, 0x78000001000000000000000000000000u128, 2147483647          , 0x01);
dec_test!(bid128_ilogb_020, bid128_ilogb, 0x7c000000000000000000000000000000u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_021, bid128_ilogb, 0x7c000001000000000000000000000000u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_022, bid128_ilogb, 0x7c003fffffffffff38c15b08ffffffffu128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_023, bid128_ilogb, 0x7c003fffffffffff38c15b0affffffffu128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_024, bid128_ilogb, 0x7cff3fffffffefffffffffffffffffffu128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_025, bid128_ilogb, 0x7e000000000000000000000000000000u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_026, bid128_ilogb, 0x7e003fffffffefffffffffffffffffffu128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_027, bid128_ilogb, 0x7ef00000000000000000000000000001u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_028, bid128_ilogb, 0x80000000000000000000000000000001u128, -6176               , 0x00); //=0.0000000000 -- -MinDen
dec_test!(bid128_ilogb_029, bid128_ilogb, 0x80420000000000000000000000000001u128, -6143               , 0x00); //=0.0000000000 -- -MinNorm
dec_test!(bid128_ilogb_030, bid128_ilogb, 0x86211d476ec3ab4de58a9dd24de85615u128, 0xFFFFEB11u32 as i32, 0x00);
dec_test!(bid128_ilogb_031, bid128_ilogb, 0x9fbfff2ffbffef5e80e117d6820566ebu128, 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_032, bid128_ilogb, 0xAFFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x00); //=0.0000000000 -- -(1-ulp)
dec_test!(bid128_ilogb_033, bid128_ilogb, 0xbb37c0485fa81f153392bf3090a40397u128, 0x0059ci32          , 0x00);
dec_test!(bid128_ilogb_034, bid128_ilogb, 0xc9920000000000000000000000000000u128, 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_035, bid128_ilogb, 0xd5ab94f9589fa6cc7008a05cc89061b5u128, 0x0012d6i32         , 0x00);
dec_test!(bid128_ilogb_036, bid128_ilogb, 0xdc2e0000000000000000000000000000u128, 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_037, bid128_ilogb, 0xdcca37edb647a616cafcb0077b828448u128, 0x0001666i32        , 0x00);
dec_test!(bid128_ilogb_038, bid128_ilogb, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 6144                , 0x00); //=0.0000000000 -- -Max
dec_test!(bid128_ilogb_039, bid128_ilogb, 0xe003b75d7734cd9e1234567890123456u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_040, bid128_ilogb, 0xe9dbb75d7734cd9e1234567890123456u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_041, bid128_ilogb, 0xf4c89c03d9863bc34e53a7c4bb2d8399u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_042, bid128_ilogb, 0xf8000000000000000000000000000000u128, 2147483647          , 0x01);
dec_test!(bid128_ilogb_043, bid128_ilogb, 0xf8000001000000000000000000000000u128, 2147483647          , 0x01);
dec_test!(bid128_ilogb_044, bid128_ilogb, 0xfbff3fff6f997ffe8680117040495b03u128, 0x7fffffffi32       , 0x01);
dec_test!(bid128_ilogb_045, bid128_ilogb, 0xfc000000000000000000000000000000u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_046, bid128_ilogb, 0xfc000001000000000000000000000000u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_047, bid128_ilogb, 0xfe000000000000000000000000000000u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_048, bid128_ilogb, 0xfef00000000000000000000000000001u128, -2147483648         , 0x01);
dec_test!(bid128_ilogb_049, bid128_ilogb, 0xffffffffffffffff2c9a250808583608u128, 0x80000000u32 as i32, 0x01);
dec_test!(bid128_ilogb_050, bid128_ilogb, "-Infinity"                           , 0x7fffffffi32       , 0x01);
dec_test!(bid128_ilogb_051, bid128_ilogb,  "Infinity"                           , 0x7fffffffi32       , 0x01);
