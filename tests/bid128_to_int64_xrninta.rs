/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int64_xrninta_001, bid128_to_int64_xrninta, "-0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_002, bid128_to_int64_xrninta,  "0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_003, bid128_to_int64_xrninta, 0x00000000000000000001200400000002u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_004, bid128_to_int64_xrninta, "-0.0001010000E0"                     , 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_005, bid128_to_int64_xrninta, 0x0001ed09bead87c0378d8e62ffffffffu128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_006, bid128_to_int64_xrninta, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_007, bid128_to_int64_xrninta, 0x002f399f999eb33debcd47df968376efu128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_008, bid128_to_int64_xrninta, 0x007b548d56b3979d848cbbbf3b390341u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_009, bid128_to_int64_xrninta, 0x00800000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_010, bid128_to_int64_xrninta, "0.1"                                 , 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_011, bid128_to_int64_xrninta, "0.5"                                 , 1                   , 0x20);
dec_test!(bid128_to_int64_xrninta_012, bid128_to_int64_xrninta, 0x1000000000000000005097c06c820047u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_013, bid128_to_int64_xrninta, "1.0"                                 , 1                   , 0x00);
dec_test!(bid128_to_int64_xrninta_014, bid128_to_int64_xrninta, "+101111101111101.0011E0"             , 101111101111101     , 0x20);
dec_test!(bid128_to_int64_xrninta_015, bid128_to_int64_xrninta, 0x17de0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_016, bid128_to_int64_xrninta, 0x2534ce63fee8f8e0bb2302e33ba43105u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_017, bid128_to_int64_xrninta, 0x27ea035e2b210508312f19446b65c04cu128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_018, bid128_to_int64_xrninta, 0x282d36816cb01a31cb4b1171e32e5697u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_019, bid128_to_int64_xrninta, 0x2bfb770df0f82a9d5b8f03ebf8a156e8u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_020, bid128_to_int64_xrninta, 0x2fc60000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_021, bid128_to_int64_xrninta, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_int64_xrninta_022, bid128_to_int64_xrninta, 0x2FFCF684DF56C3E01BC6C73200000000u128, 1                   , 0x20); // -- 0.5
dec_test!(bid128_to_int64_xrninta_023, bid128_to_int64_xrninta, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1                   , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_int64_xrninta_024, bid128_to_int64_xrninta, 0x2ffd7ffdfdf6fff7baa252d9774ffdb2u128, 1                   , 0x20);
dec_test!(bid128_to_int64_xrninta_025, bid128_to_int64_xrninta, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1                   , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_int64_xrninta_026, bid128_to_int64_xrninta, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1                   , 0x20); // -- 0.999
dec_test!(bid128_to_int64_xrninta_027, bid128_to_int64_xrninta, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1                   , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_int64_xrninta_028, bid128_to_int64_xrninta, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1                   , 0x20); // -- 1-ulp
dec_test!(bid128_to_int64_xrninta_029, bid128_to_int64_xrninta, 0x2FFE314DC6448D9338C15B0A00000000u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_xrninta_030, bid128_to_int64_xrninta, 0x2FFE314DC6448D9338C15B0A00000001u128, 1                   , 0x20); // -- 1+ulp
dec_test!(bid128_to_int64_xrninta_031, bid128_to_int64_xrninta, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1                   , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_int64_xrninta_032, bid128_to_int64_xrninta, 0x2FFE49F4A966D45CD522088F00000000u128, 2                   , 0x20); // -- 1.5
dec_test!(bid128_to_int64_xrninta_033, bid128_to_int64_xrninta, 0x2FFE49F4A966D45CD522088F00000001u128, 2                   , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_int64_xrninta_034, bid128_to_int64_xrninta, 0x2fffbd6e53c43e9f0000000000000000u128, 9                   , 0x20);
dec_test!(bid128_to_int64_xrninta_035, bid128_to_int64_xrninta, 0x300001220100010805f01e135bdfd488u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_036, bid128_to_int64_xrninta, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_037, bid128_to_int64_xrninta, 0x300293E952CDA8B9AA44111E00000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_038, bid128_to_int64_xrninta, 0x300293E952CDA8B9AA44111E00000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_039, bid128_to_int64_xrninta, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xrninta_040, bid128_to_int64_xrninta, 0x300294286EACB8CB0A8CB6B140000000u128, 301                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xrninta_041, bid128_to_int64_xrninta, 0x300294286EACB8CB0A8CB6B140000001u128, 301                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xrninta_042, bid128_to_int64_xrninta, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_043, bid128_to_int64_xrninta, 0x30040ECA8847C4129106CE8300000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_044, bid128_to_int64_xrninta, 0x30040ECA8847C4129106CE8300000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_045, bid128_to_int64_xrninta, 0x3004112540b825280c939a4f444290c0u128, 348                 , 0x20);
dec_test!(bid128_to_int64_xrninta_046, bid128_to_int64_xrninta, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_047, bid128_to_int64_xrninta, 0x300A0003C95A2F0B4856475FE0000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_048, bid128_to_int64_xrninta, 0x300A0003C95A2F0B4856475FE0000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_049, bid128_to_int64_xrninta, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_050, bid128_to_int64_xrninta, 0x300C000060EF6B1ABA6F072330000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_051, bid128_to_int64_xrninta, 0x300C000060EF6B1ABA6F072330000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_052, bid128_to_int64_xrninta, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000          , 0x20); // -- 4e9-ulp
dec_test!(bid128_to_int64_xrninta_053, bid128_to_int64_xrninta, 0x3010C5371912364CE3056C2800000000u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_xrninta_054, bid128_to_int64_xrninta, 0x3010C5371912364CE3056C2800000001u128, 4000000000          , 0x20); // -- 4e9+ulp
dec_test!(bid128_to_int64_xrninta_055, bid128_to_int64_xrninta, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 5000000000          , 0x20); // -- 5e9-ulp
dec_test!(bid128_to_int64_xrninta_056, bid128_to_int64_xrninta, 0x3010F684DF56C3E01BC6C73200000000u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_xrninta_057, bid128_to_int64_xrninta, 0x3010F684DF56C3E01BC6C73200000001u128, 5000000000          , 0x20); // -- 5e9+ulp
dec_test!(bid128_to_int64_xrninta_058, bid128_to_int64_xrninta, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 19999999998         , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_xrninta_059, bid128_to_int64_xrninta, 0x3012629B8C88FB62ED56E4238E400000u128, 19999999999         , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_int64_xrninta_060, bid128_to_int64_xrninta, 0x3012629B8C88FB62ED56E4238E400001u128, 19999999999         , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_xrninta_061, bid128_to_int64_xrninta, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 19999999999         , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_xrninta_062, bid128_to_int64_xrninta, 0x3012629B8C8905F96EBAD4C909800000u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_xrninta_063, bid128_to_int64_xrninta, 0x3012629B8C8905F96EBAD4C909800001u128, 19999999999         , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_xrninta_064, bid128_to_int64_xrninta, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 19999999999         , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_xrninta_065, bid128_to_int64_xrninta, 0x3012629B8C89108FF01EC56E84C00000u128, 20000000000         , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_int64_xrninta_066, bid128_to_int64_xrninta, 0x3012629B8C89108FF01EC56E84C00001u128, 20000000000         , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_xrninta_067, bid128_to_int64_xrninta, 0x3012629B8C891B267182B613FFFFFFFFu128, 20000000000         , 0x20); // -- 2e10-ulp
dec_test!(bid128_to_int64_xrninta_068, bid128_to_int64_xrninta, 0x3012629B8C891B267182B61400000000u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_xrninta_069, bid128_to_int64_xrninta, 0x3012629B8C891B267182B61400000001u128, 20000000000         , 0x20); // -- 2e10+ulp
dec_test!(bid128_to_int64_xrninta_070, bid128_to_int64_xrninta, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 20000000000         , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_xrninta_071, bid128_to_int64_xrninta, 0x3012629B8C8925BCF2E6A6B97B400000u128, 20000000001         , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_int64_xrninta_072, bid128_to_int64_xrninta, 0x3012629B8C8925BCF2E6A6B97B400001u128, 20000000001         , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_xrninta_073, bid128_to_int64_xrninta, 0x3012629B8C893053744A975EF67FFFFFu128, 20000000001         , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_xrninta_074, bid128_to_int64_xrninta, 0x3012629B8C893053744A975EF6800000u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_xrninta_075, bid128_to_int64_xrninta, 0x3012629B8C893053744A975EF6800001u128, 20000000001         , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_xrninta_076, bid128_to_int64_xrninta, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 20000000001         , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_xrninta_077, bid128_to_int64_xrninta, 0x3012629B8C893AE9F5AE880471C00000u128, 20000000002         , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_int64_xrninta_078, bid128_to_int64_xrninta, 0x3012629B8C893AE9F5AE880471C00001u128, 20000000002         , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_xrninta_079, bid128_to_int64_xrninta, 0x30143ac566efa440fbfbefddeef3dc7fu128, 119201956221        , 0x20);
dec_test!(bid128_to_int64_xrninta_080, bid128_to_int64_xrninta, 0x3018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 35184372088832      , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_int64_xrninta_081, bid128_to_int64_xrninta, 0x3018AD78EBC5AC620000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_xrninta_082, bid128_to_int64_xrninta, 0x3018AD78EBC5AC620000000000000001u128, 35184372088832      , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_int64_xrninta_083, bid128_to_int64_xrninta, 0x3018AD78EBC5AC64B5E3AF16B187FFFFu128, 35184372088832      , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_xrninta_084, bid128_to_int64_xrninta, 0x3018AD78EBC5AC64B5E3AF16B1880000u128, 35184372088833      , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_int64_xrninta_085, bid128_to_int64_xrninta, 0x3018AD78EBC5AC64B5E3AF16B1880001u128, 35184372088833      , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_xrninta_086, bid128_to_int64_xrninta, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xrninta_087, bid128_to_int64_xrninta, 0x301A0000000000A2E6C09AD3E0D40000u128, 301                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xrninta_088, bid128_to_int64_xrninta, 0x301A0000000000A2E6C09AD3E0D40001u128, 301                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xrninta_089, bid128_to_int64_xrninta, 0x301C629B8C891B265CB1A40684E9FFFFu128, 1999999999999998    , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_xrninta_090, bid128_to_int64_xrninta, 0x301C629B8C891B265CB1A40684EA0000u128, 1999999999999999    , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_int64_xrninta_091, bid128_to_int64_xrninta, 0x301C629B8C891B265CB1A40684EA0001u128, 1999999999999999    , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_xrninta_092, bid128_to_int64_xrninta, 0x301C629B8C891B2663A1FF60589BFFFFu128, 1999999999999999    , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_xrninta_093, bid128_to_int64_xrninta, 0x301C629B8C891B2663A1FF60589C0000u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_xrninta_094, bid128_to_int64_xrninta, 0x301C629B8C891B2663A1FF60589C0001u128, 1999999999999999    , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_xrninta_095, bid128_to_int64_xrninta, 0x301C629B8C891B266A925ABA2C4DFFFFu128, 1999999999999999    , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_xrninta_096, bid128_to_int64_xrninta, 0x301C629B8C891B266A925ABA2C4E0000u128, 2000000000000000    , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_int64_xrninta_097, bid128_to_int64_xrninta, 0x301C629B8C891B266A925ABA2C4E0001u128, 2000000000000000    , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_xrninta_098, bid128_to_int64_xrninta, 0x301C629B8C891B267182B613FFFFFFFFu128, 2000000000000000    , 0x20); // -- 2e15-ulp
dec_test!(bid128_to_int64_xrninta_099, bid128_to_int64_xrninta, 0x301C629B8C891B267182B61400000000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_xrninta_100, bid128_to_int64_xrninta, 0x301C629B8C891B267182B61400000001u128, 2000000000000000    , 0x20); // -- 2e15+ulp
dec_test!(bid128_to_int64_xrninta_101, bid128_to_int64_xrninta, 0x301C629B8C891B267873116DD3B1FFFFu128, 2000000000000000    , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_xrninta_102, bid128_to_int64_xrninta, 0x301C629B8C891B267873116DD3B20000u128, 2000000000000001    , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_int64_xrninta_103, bid128_to_int64_xrninta, 0x301C629B8C891B267873116DD3B20001u128, 2000000000000001    , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_xrninta_104, bid128_to_int64_xrninta, 0x301C629B8C891B267F636CC7A763FFFFu128, 2000000000000001    , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_xrninta_105, bid128_to_int64_xrninta, 0x301C629B8C891B267F636CC7A7640000u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_xrninta_106, bid128_to_int64_xrninta, 0x301C629B8C891B267F636CC7A7640001u128, 2000000000000001    , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_xrninta_107, bid128_to_int64_xrninta, 0x301C629B8C891B268653C8217B15FFFFu128, 2000000000000001    , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_xrninta_108, bid128_to_int64_xrninta, 0x301C629B8C891B268653C8217B160000u128, 2000000000000002    , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_int64_xrninta_109, bid128_to_int64_xrninta, 0x301C629B8C891B268653C8217B160001u128, 2000000000000002    , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_xrninta_110, bid128_to_int64_xrninta, 0x301E000000000001A055690D9DB7FFFFu128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_111, bid128_to_int64_xrninta, 0x301E000000000001A055690D9DB80000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_112, bid128_to_int64_xrninta, 0x301E000000000001A055690D9DB80001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_113, bid128_to_int64_xrninta, 0x301E002C68AF0BB140B1A2BC2EC4FFFFu128, 35184372088832      , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_xrninta_114, bid128_to_int64_xrninta, 0x301E002C68AF0BB140B1A2BC2EC50000u128, 35184372088833      , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_int64_xrninta_115, bid128_to_int64_xrninta, 0x301E002C68AF0BB140B1A2BC2EC50001u128, 35184372088833      , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_xrninta_116, bid128_to_int64_xrninta, 0x302000000000000029A2241AF62BFFFFu128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_117, bid128_to_int64_xrninta, 0x302000000000000029A2241AF62C0000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_118, bid128_to_int64_xrninta, 0x302000000000000029A2241AF62C0001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_119, bid128_to_int64_xrninta, 0x3020000470DE4DF81FFFFFFFFFFFFFFFu128, 35184372088832      , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_int64_xrninta_120, bid128_to_int64_xrninta, 0x3020000470DE4DF82000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_xrninta_121, bid128_to_int64_xrninta, 0x3020000470DE4DF82000000000000001u128, 35184372088832      , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_int64_xrninta_122, bid128_to_int64_xrninta, 0x302000FC6F7C4045813459C637E07FFFu128, 2000000000000000    , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_xrninta_123, bid128_to_int64_xrninta, 0x302000FC6F7C4045813459C637E08000u128, 2000000000000001    , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_int64_xrninta_124, bid128_to_int64_xrninta, 0x302000FC6F7C4045813459C637E08001u128, 2000000000000001    , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_xrninta_125, bid128_to_int64_xrninta, 0x302000FC6F7C40458157E0B8A7A17FFFu128, 2000000000000001    , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_xrninta_126, bid128_to_int64_xrninta, 0x302000FC6F7C40458157E0B8A7A18000u128, 2000000000000002    , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_int64_xrninta_127, bid128_to_int64_xrninta, 0x302000FC6F7C40458157E0B8A7A18001u128, 2000000000000002    , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_xrninta_128, bid128_to_int64_xrninta, 0x30208b8004980410fffdffffffffffffu128, 282939756142537495  , 0x20);
dec_test!(bid128_to_int64_xrninta_129, bid128_to_int64_xrninta, 0x302200193E5939A08CE4879688D63FFFu128, 1999999999999998    , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_xrninta_130, bid128_to_int64_xrninta, 0x302200193E5939A08CE4879688D64000u128, 1999999999999999    , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_int64_xrninta_131, bid128_to_int64_xrninta, 0x302200193E5939A08CE4879688D64001u128, 1999999999999999    , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_xrninta_132, bid128_to_int64_xrninta, 0x302200193E5939A08CE815152D9CBFFFu128, 1999999999999999    , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_xrninta_133, bid128_to_int64_xrninta, 0x302200193E5939A08CE815152D9CC000u128, 2000000000000000    , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_int64_xrninta_134, bid128_to_int64_xrninta, 0x302200193E5939A08CE815152D9CC001u128, 2000000000000000    , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_xrninta_135, bid128_to_int64_xrninta, 0x302313fd4fd4fdd5e6ce98f8a48a5c97u128, 5597732072927072397 , 0x20);
dec_test!(bid128_to_int64_xrninta_136, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775806 , 0x20); // -- 2^63-1.5-ulp
dec_test!(bid128_to_int64_xrninta_137, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775807 , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_int64_xrninta_138, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775807 , 0x20); // -- 2^63-1.5+ulp
dec_test!(bid128_to_int64_xrninta_139, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775807 , 0x20); // -- 2^63-1-ulp
dec_test!(bid128_to_int64_xrninta_140, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFC72815B398000u128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_xrninta_141, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFC72815B398001u128, 9223372036854775807 , 0x20); // -- 2^63-1+ulp
dec_test!(bid128_to_int64_xrninta_142, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775807 , 0x20); // -- 2^63-0.5-ulp
dec_test!(bid128_to_int64_xrninta_143, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFE3940AD9CC000u128, -9223372036854775808, 0x01); // -- 2^63-0.5
dec_test!(bid128_to_int64_xrninta_144, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFE3940AD9CC001u128, -9223372036854775808, 0x01); // -- 2^63-0.5+ulp
dec_test!(bid128_to_int64_xrninta_145, bid128_to_int64_xrninta, 0x3023C6BF52633FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^63-ulp
dec_test!(bid128_to_int64_xrninta_146, bid128_to_int64_xrninta, 0x3023C6BF526340000000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_xrninta_147, bid128_to_int64_xrninta, 0x3023C6BF526340000000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+ulp
dec_test!(bid128_to_int64_xrninta_148, bid128_to_int64_xrninta, 0x3023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x01); // -- 2^63+0.5-ulp
dec_test!(bid128_to_int64_xrninta_149, bid128_to_int64_xrninta, 0x3023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_xrninta_150, bid128_to_int64_xrninta, 0x3023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- 2^63+0.5+ulp
dec_test!(bid128_to_int64_xrninta_151, bid128_to_int64_xrninta, 0x3023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- 2^63+1-ulp
dec_test!(bid128_to_int64_xrninta_152, bid128_to_int64_xrninta, 0x3023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_xrninta_153, bid128_to_int64_xrninta, 0x3023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- 2^63+1+ulp
dec_test!(bid128_to_int64_xrninta_154, bid128_to_int64_xrninta, 0x3024000000000000006A94D74F42FFFFu128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_155, bid128_to_int64_xrninta, 0x3024000000000000006A94D74F430000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_156, bid128_to_int64_xrninta, 0x3024000000000000006A94D74F430001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_157, bid128_to_int64_xrninta, 0x30242e6403d866e37ffeffffffd3feffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_158, bid128_to_int64_xrninta, 0x3024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e19-ulp
dec_test!(bid128_to_int64_xrninta_159, bid128_to_int64_xrninta, 0x3024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_xrninta_160, bid128_to_int64_xrninta, 0x3024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e19+ulp
dec_test!(bid128_to_int64_xrninta_161, bid128_to_int64_xrninta, 0x3024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- 1e19+0.5-ulp
dec_test!(bid128_to_int64_xrninta_162, bid128_to_int64_xrninta, 0x3024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_xrninta_163, bid128_to_int64_xrninta, 0x3024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- 1e19+0.5+ulp
dec_test!(bid128_to_int64_xrninta_164, bid128_to_int64_xrninta, 0x302449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- 1.5e19-ulp
dec_test!(bid128_to_int64_xrninta_165, bid128_to_int64_xrninta, 0x302449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_xrninta_166, bid128_to_int64_xrninta, 0x302449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- 1.5e19+ulp
dec_test!(bid128_to_int64_xrninta_167, bid128_to_int64_xrninta, 0x30245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- 2^64-1-ulp
dec_test!(bid128_to_int64_xrninta_168, bid128_to_int64_xrninta, 0x30245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_xrninta_169, bid128_to_int64_xrninta, 0x30245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- 2^64-1+ulp
dec_test!(bid128_to_int64_xrninta_170, bid128_to_int64_xrninta, 0x30245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- 2^64-0.5-ulp
dec_test!(bid128_to_int64_xrninta_171, bid128_to_int64_xrninta, 0x30245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_xrninta_172, bid128_to_int64_xrninta, 0x30245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- 2^64-0.5+ulp
dec_test!(bid128_to_int64_xrninta_173, bid128_to_int64_xrninta, 0x30245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-ulp
dec_test!(bid128_to_int64_xrninta_174, bid128_to_int64_xrninta, 0x30245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_xrninta_175, bid128_to_int64_xrninta, 0x30245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+ulp
dec_test!(bid128_to_int64_xrninta_176, bid128_to_int64_xrninta, 0x30245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- 2^64+0.5-ulp
dec_test!(bid128_to_int64_xrninta_177, bid128_to_int64_xrninta, 0x30245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_xrninta_178, bid128_to_int64_xrninta, 0x30245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- 2^64+0.5+ulp
dec_test!(bid128_to_int64_xrninta_179, bid128_to_int64_xrninta, 0x30245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- 2^64+1-ulp
dec_test!(bid128_to_int64_xrninta_180, bid128_to_int64_xrninta, 0x30245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_xrninta_181, bid128_to_int64_xrninta, 0x30245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- 2^64+1+ulp
dec_test!(bid128_to_int64_xrninta_182, bid128_to_int64_xrninta, 0x3024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2e19-ulp
dec_test!(bid128_to_int64_xrninta_183, bid128_to_int64_xrninta, 0x3024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_xrninta_184, bid128_to_int64_xrninta, 0x3024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- 2e19+ulp
dec_test!(bid128_to_int64_xrninta_185, bid128_to_int64_xrninta, 0x30247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2.5e19-ulp
dec_test!(bid128_to_int64_xrninta_186, bid128_to_int64_xrninta, 0x30247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_xrninta_187, bid128_to_int64_xrninta, 0x30247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- 2.5e19+ulp
dec_test!(bid128_to_int64_xrninta_188, bid128_to_int64_xrninta, 0x3026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e20-ulp
dec_test!(bid128_to_int64_xrninta_189, bid128_to_int64_xrninta, 0x3026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_xrninta_190, bid128_to_int64_xrninta, 0x3026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e20+ulp
dec_test!(bid128_to_int64_xrninta_191, bid128_to_int64_xrninta, 0x302A00000000006C6B935B68D08DA3FFu128, 19999999998         , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_xrninta_192, bid128_to_int64_xrninta, 0x302A00000000006C6B935B68D08DA400u128, 19999999999         , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_int64_xrninta_193, bid128_to_int64_xrninta, 0x302A00000000006C6B935B68D08DA401u128, 19999999999         , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_xrninta_194, bid128_to_int64_xrninta, 0x302A00000000006C6B935B8019048BFFu128, 19999999999         , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_xrninta_195, bid128_to_int64_xrninta, 0x302A00000000006C6B935B8019048C00u128, 20000000000         , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_int64_xrninta_196, bid128_to_int64_xrninta, 0x302A00000000006C6B935B8019048C01u128, 20000000000         , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_xrninta_197, bid128_to_int64_xrninta, 0x302C000000000000000002BBA7F521FFu128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xrninta_198, bid128_to_int64_xrninta, 0x302C000000000000000002BBA7F52200u128, 301                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xrninta_199, bid128_to_int64_xrninta, 0x302C000000000000000002BBA7F52201u128, 301                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xrninta_200, bid128_to_int64_xrninta, 0x302C00000000000AD78EBC5872141BFFu128, 19999999999         , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_xrninta_201, bid128_to_int64_xrninta, 0x302C00000000000AD78EBC5872141C00u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_xrninta_202, bid128_to_int64_xrninta, 0x302C00000000000AD78EBC5872141C01u128, 19999999999         , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_xrninta_203, bid128_to_int64_xrninta, 0x302C00000000000AD78EBC5BF025F1FFu128, 20000000000         , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_xrninta_204, bid128_to_int64_xrninta, 0x302C00000000000AD78EBC5BF025F200u128, 20000000001         , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_int64_xrninta_205, bid128_to_int64_xrninta, 0x302C00000000000AD78EBC5BF025F201u128, 20000000001         , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_xrninta_206, bid128_to_int64_xrninta, 0x302C00000000000AD78EBC5E4431D5FFu128, 20000000001         , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_xrninta_207, bid128_to_int64_xrninta, 0x302C00000000000AD78EBC5E4431D600u128, 20000000002         , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_int64_xrninta_208, bid128_to_int64_xrninta, 0x302C00000000000AD78EBC5E4431D601u128, 20000000002         , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_xrninta_209, bid128_to_int64_xrninta, 0x302C000000108B2A2C28028E3FF41BFFu128, 1999999999999999    , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_xrninta_210, bid128_to_int64_xrninta, 0x302C000000108B2A2C28028E3FF41C00u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_xrninta_211, bid128_to_int64_xrninta, 0x302C000000108B2A2C28028E3FF41C01u128, 1999999999999999    , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_xrninta_212, bid128_to_int64_xrninta, 0x302E000000000001158E46094F6AC9FFu128, 20000000001         , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_xrninta_213, bid128_to_int64_xrninta, 0x302E000000000001158E46094F6ACA00u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_xrninta_214, bid128_to_int64_xrninta, 0x302E000000000001158E46094F6ACA01u128, 20000000001         , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_xrninta_215, bid128_to_int64_xrninta, 0x302E00000001A784379D99DB7D9AC9FFu128, 2000000000000001    , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_xrninta_216, bid128_to_int64_xrninta, 0x302E00000001A784379D99DB7D9ACA00u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_xrninta_217, bid128_to_int64_xrninta, 0x302E00000001A784379D99DB7D9ACA01u128, 2000000000000001    , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_xrninta_218, bid128_to_int64_xrninta, 0x303000000000000000000006FC23ABFFu128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_219, bid128_to_int64_xrninta, 0x303000000000000000000006FC23AC00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_220, bid128_to_int64_xrninta, 0x303000000000000000000006FC23AC01u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_221, bid128_to_int64_xrninta, 0x303200000000000000000000B2D05DFFu128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_222, bid128_to_int64_xrninta, 0x303200000000000000000000B2D05E00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_223, bid128_to_int64_xrninta, 0x303200000000000000000000B2D05E01u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_224, bid128_to_int64_xrninta, 0x303800000000000000000000002DDA47u128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xrninta_225, bid128_to_int64_xrninta, 0x303800000000000000000000002DDA48u128, 301                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xrninta_226, bid128_to_int64_xrninta, 0x303800000000000000000000002DDA49u128, 301                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xrninta_227, bid128_to_int64_xrninta, 0x303A00000000000000000000000003E7u128, 1                   , 0x20); // -- 0.999
dec_test!(bid128_to_int64_xrninta_228, bid128_to_int64_xrninta, 0x303A00000000000000000000000495D3u128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xrninta_229, bid128_to_int64_xrninta, 0x303A00000000000000000000000495D4u128, 301                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xrninta_230, bid128_to_int64_xrninta, 0x303A00000000000000000000000495D5u128, 301                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xrninta_231, bid128_to_int64_xrninta, 0x303C0000000000000000000000007561u128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xrninta_232, bid128_to_int64_xrninta, 0x303C0000000000000000000000007562u128, 301                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xrninta_233, bid128_to_int64_xrninta, 0x303C0000000000000000000000007563u128, 301                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xrninta_234, bid128_to_int64_xrninta, 0x303E0000000000000000000000000005u128, 1                   , 0x20); // -- 0.5
dec_test!(bid128_to_int64_xrninta_235, bid128_to_int64_xrninta, 0x303E000000000000000000000000000Fu128, 2                   , 0x20); // -- 1.5
dec_test!(bid128_to_int64_xrninta_236, bid128_to_int64_xrninta, 0x303E0000000000000000000000000BB7u128, 300                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_237, bid128_to_int64_xrninta, 0x303E0000000000000000000000000BB8u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_238, bid128_to_int64_xrninta, 0x303E0000000000000000000000000BB9u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_239, bid128_to_int64_xrninta, 0x303E0000000000000000000000000BBDu128, 301                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xrninta_240, bid128_to_int64_xrninta, 0x303E0000000000000000002E90EDCFF1u128, 19999999999         , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_int64_xrninta_241, bid128_to_int64_xrninta, 0x303E0000000000000000002E90EDCFFBu128, 20000000000         , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_int64_xrninta_242, bid128_to_int64_xrninta, 0x303E0000000000000000002E90EDD005u128, 20000000001         , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_int64_xrninta_243, bid128_to_int64_xrninta, 0x303E0000000000000000002E90EDD00Fu128, 20000000002         , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_int64_xrninta_244, bid128_to_int64_xrninta, 0x303E0000000000000001400000000005u128, 35184372088833      , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_int64_xrninta_245, bid128_to_int64_xrninta, 0x303E00000000000000470DE4DF81FFF1u128, 1999999999999999    , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_int64_xrninta_246, bid128_to_int64_xrninta, 0x303E00000000000000470DE4DF81FFFBu128, 2000000000000000    , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_int64_xrninta_247, bid128_to_int64_xrninta, 0x303E00000000000000470DE4DF820005u128, 2000000000000001    , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_int64_xrninta_248, bid128_to_int64_xrninta, 0x303E00000000000000470DE4DF82000Fu128, 2000000000000002    , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_int64_xrninta_249, bid128_to_int64_xrninta, 0x303E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775807 , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_int64_xrninta_250, bid128_to_int64_xrninta, 0x303E000000000004FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- 2^63-0.5
dec_test!(bid128_to_int64_xrninta_251, bid128_to_int64_xrninta, 0x303E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_xrninta_252, bid128_to_int64_xrninta, 0x303E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_xrninta_253, bid128_to_int64_xrninta, 0x303E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_xrninta_254, bid128_to_int64_xrninta, 0x303E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_xrninta_255, bid128_to_int64_xrninta, 0x30400000000000000000000000000001u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_xrninta_256, bid128_to_int64_xrninta, 0x3040000000000000000000000000012Bu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_257, bid128_to_int64_xrninta, 0x3040000000000000000000000000012Cu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_258, bid128_to_int64_xrninta, 0x3040000000000000000000000000012Du128, 301                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_259, bid128_to_int64_xrninta, 0x304000000000000000000004A817C7FFu128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_xrninta_260, bid128_to_int64_xrninta, 0x304000000000000000000004A817C801u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_xrninta_261, bid128_to_int64_xrninta, 0x30400000000000000000200000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_xrninta_262, bid128_to_int64_xrninta, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_xrninta_263, bid128_to_int64_xrninta, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_int64_xrninta_264, bid128_to_int64_xrninta, 0x304000000000000000071AFD498D0000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_xrninta_265, bid128_to_int64_xrninta, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_xrninta_266, bid128_to_int64_xrninta, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_int64_xrninta_267, bid128_to_int64_xrninta, 0x30400000000000002e00008000203012u128, 3314649875502608402 , 0x00);
dec_test!(bid128_to_int64_xrninta_268, bid128_to_int64_xrninta, 0x30400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_xrninta_269, bid128_to_int64_xrninta, 0x30400000000000008000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_xrninta_270, bid128_to_int64_xrninta, 0x30400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_xrninta_271, bid128_to_int64_xrninta, 0x3040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_xrninta_272, bid128_to_int64_xrninta, 0x30400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_xrninta_273, bid128_to_int64_xrninta, 0x30400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_xrninta_274, bid128_to_int64_xrninta, 0x3041ED09BEAD87C0378D8E6400000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_275, bid128_to_int64_xrninta, 0x3042000000000000000000000000001Du128, 290                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_xrninta_276, bid128_to_int64_xrninta, 0x3042000000000000000000000000001Eu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_277, bid128_to_int64_xrninta, 0x3042000000000000000000000000001Fu128, 310                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_xrninta_278, bid128_to_int64_xrninta, 0x304200000000000000000000773593FFu128, 19999999990         , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_int64_xrninta_279, bid128_to_int64_xrninta, 0x30420000000000000000000077359400u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_xrninta_280, bid128_to_int64_xrninta, 0x30420000000000000000000077359401u128, 20000000010         , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_int64_xrninta_281, bid128_to_int64_xrninta, 0x30440000000000000000000000000003u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xrninta_282, bid128_to_int64_xrninta, 0x30520000000000000000000000000004u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_xrninta_283, bid128_to_int64_xrninta, 0x30520000000000000000000000000005u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_xrninta_284, bid128_to_int64_xrninta, 0x30540000000000000000000000000002u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_xrninta_285, bid128_to_int64_xrninta, 0x305E0000000000000000000000000002u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_xrninta_286, bid128_to_int64_xrninta, 0x3064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_xrninta_287, bid128_to_int64_xrninta, 0x30640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_xrninta_288, bid128_to_int64_xrninta, 0x30660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_xrninta_289, bid128_to_int64_xrninta, 0x30660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_xrninta_290, bid128_to_int64_xrninta, 0x30680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_xrninta_291, bid128_to_int64_xrninta, 0x3b480b9490aaa16859b6f18efd8c2354u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_292, bid128_to_int64_xrninta, 0x426f41fe4fb2293ea0c22babe987cccbu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_293, bid128_to_int64_xrninta, 0x4d6a0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_294, bid128_to_int64_xrninta, "5.05"                                , 5                   , 0x20);
dec_test!(bid128_to_int64_xrninta_295, bid128_to_int64_xrninta, "+688776.985E0"                       , 688777              , 0x20);
dec_test!(bid128_to_int64_xrninta_296, bid128_to_int64_xrninta, 0x78000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_297, bid128_to_int64_xrninta, 0x7bdffa7f75bfbfee098814cb755eb6b4u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_298, bid128_to_int64_xrninta, 0x7c000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_299, bid128_to_int64_xrninta, 0x7c003fffffffffff38c15b08ffffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_300, bid128_to_int64_xrninta, 0x7c003fffffffffff38c15b0affffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_301, bid128_to_int64_xrninta, 0x7ddbeffefffff6cfb0def7cbf64f0fafu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_302, bid128_to_int64_xrninta, 0x7e000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_303, bid128_to_int64_xrninta, 0x865d00d951484763300e79b5c5737df3u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_304, bid128_to_int64_xrninta, "+89988.88998E0"                      , 89989               , 0x20);
dec_test!(bid128_to_int64_xrninta_305, bid128_to_int64_xrninta, 0x910400002c1000183156d6ffdd062af0u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xrninta_306, bid128_to_int64_xrninta, 0x98740000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_307, bid128_to_int64_xrninta, "-9"                                  , -9                  , 0x00);
dec_test!(bid128_to_int64_xrninta_308, bid128_to_int64_xrninta, 0x9ef00000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_309, bid128_to_int64_xrninta, 0xa8620000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_310, bid128_to_int64_xrninta, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_int64_xrninta_311, bid128_to_int64_xrninta, 0xAFFCF684DF56C3E01BC6C73200000000u128, -1                  , 0x20); // -- -(0.5)
dec_test!(bid128_to_int64_xrninta_312, bid128_to_int64_xrninta, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1                  , 0x20); // -- -(0.5+ulp)
dec_test!(bid128_to_int64_xrninta_313, bid128_to_int64_xrninta, 0xaffd94a520a07f9c03fbcb8f8f0ad9d6u128, -1                  , 0x20);
dec_test!(bid128_to_int64_xrninta_314, bid128_to_int64_xrninta, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1                  , 0x20); // -- -(0.999-ulp)
dec_test!(bid128_to_int64_xrninta_315, bid128_to_int64_xrninta, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1                  , 0x20); // -- -(0.999)
dec_test!(bid128_to_int64_xrninta_316, bid128_to_int64_xrninta, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1                  , 0x20); // -- -(0.999+ulp)
dec_test!(bid128_to_int64_xrninta_317, bid128_to_int64_xrninta, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1                  , 0x20); // -- -(1-ulp)
dec_test!(bid128_to_int64_xrninta_318, bid128_to_int64_xrninta, 0xAFFE314DC6448D9338C15B0A00000000u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_xrninta_319, bid128_to_int64_xrninta, 0xAFFE314DC6448D9338C15B0A00000001u128, -1                  , 0x20); // -- -(1+ulp)
dec_test!(bid128_to_int64_xrninta_320, bid128_to_int64_xrninta, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1                  , 0x20); // -- -(1.5-ulp)
dec_test!(bid128_to_int64_xrninta_321, bid128_to_int64_xrninta, 0xAFFE49F4A966D45CD522088F00000000u128, -2                  , 0x20); // -- -(1.5)
dec_test!(bid128_to_int64_xrninta_322, bid128_to_int64_xrninta, 0xAFFE49F4A966D45CD522088F00000001u128, -2                  , 0x20); // -- -(1.5+ulp)
dec_test!(bid128_to_int64_xrninta_323, bid128_to_int64_xrninta, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_324, bid128_to_int64_xrninta, 0xB00293E952CDA8B9AA44111E00000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_325, bid128_to_int64_xrninta, 0xB00293E952CDA8B9AA44111E00000001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_326, bid128_to_int64_xrninta, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xrninta_327, bid128_to_int64_xrninta, 0xB00294286EACB8CB0A8CB6B140000000u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xrninta_328, bid128_to_int64_xrninta, 0xB00294286EACB8CB0A8CB6B140000001u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xrninta_329, bid128_to_int64_xrninta, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_330, bid128_to_int64_xrninta, 0xB0040ECA8847C4129106CE8300000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_331, bid128_to_int64_xrninta, 0xB0040ECA8847C4129106CE8300000001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_332, bid128_to_int64_xrninta, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_333, bid128_to_int64_xrninta, 0xB00A0003C95A2F0B4856475FE0000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_334, bid128_to_int64_xrninta, 0xB00A0003C95A2F0B4856475FE0000001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_335, bid128_to_int64_xrninta, 0xb00a6a83cfdf9da6c7a021f1a0100013u128, -2160379            , 0x20);
dec_test!(bid128_to_int64_xrninta_336, bid128_to_int64_xrninta, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_337, bid128_to_int64_xrninta, 0xB00C000060EF6B1ABA6F072330000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_338, bid128_to_int64_xrninta, 0xB00C000060EF6B1ABA6F072330000001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_339, bid128_to_int64_xrninta, 0xB010C5371912364CE3056C27FFFFFFFFu128, -4000000000         , 0x20); // -- -(4e9-ulp)
dec_test!(bid128_to_int64_xrninta_340, bid128_to_int64_xrninta, 0xB010C5371912364CE3056C2800000000u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_xrninta_341, bid128_to_int64_xrninta, 0xB010C5371912364CE3056C2800000001u128, -4000000000         , 0x20); // -- -(4e9+ulp)
dec_test!(bid128_to_int64_xrninta_342, bid128_to_int64_xrninta, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -5000000000         , 0x20); // -- -(5e9-ulp)
dec_test!(bid128_to_int64_xrninta_343, bid128_to_int64_xrninta, 0xB010F684DF56C3E01BC6C73200000000u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_xrninta_344, bid128_to_int64_xrninta, 0xB010F684DF56C3E01BC6C73200000001u128, -5000000000         , 0x20); // -- -(5e9+ulp)
dec_test!(bid128_to_int64_xrninta_345, bid128_to_int64_xrninta, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -19999999998        , 0x20); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_xrninta_346, bid128_to_int64_xrninta, 0xB012629B8C88FB62ED56E4238E400000u128, -19999999999        , 0x20); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_xrninta_347, bid128_to_int64_xrninta, 0xB012629B8C88FB62ED56E4238E400001u128, -19999999999        , 0x20); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_xrninta_348, bid128_to_int64_xrninta, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -19999999999        , 0x20); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_xrninta_349, bid128_to_int64_xrninta, 0xB012629B8C8905F96EBAD4C909800000u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_xrninta_350, bid128_to_int64_xrninta, 0xB012629B8C8905F96EBAD4C909800001u128, -19999999999        , 0x20); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_xrninta_351, bid128_to_int64_xrninta, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -19999999999        , 0x20); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_xrninta_352, bid128_to_int64_xrninta, 0xB012629B8C89108FF01EC56E84C00000u128, -20000000000        , 0x20); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_xrninta_353, bid128_to_int64_xrninta, 0xB012629B8C89108FF01EC56E84C00001u128, -20000000000        , 0x20); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_xrninta_354, bid128_to_int64_xrninta, 0xB012629B8C891B267182B613FFFFFFFFu128, -20000000000        , 0x20); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_xrninta_355, bid128_to_int64_xrninta, 0xB012629B8C891B267182B61400000000u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_xrninta_356, bid128_to_int64_xrninta, 0xB012629B8C891B267182B61400000001u128, -20000000000        , 0x20); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_xrninta_357, bid128_to_int64_xrninta, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -20000000000        , 0x20); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_xrninta_358, bid128_to_int64_xrninta, 0xB012629B8C8925BCF2E6A6B97B400000u128, -20000000001        , 0x20); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_xrninta_359, bid128_to_int64_xrninta, 0xB012629B8C8925BCF2E6A6B97B400001u128, -20000000001        , 0x20); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_xrninta_360, bid128_to_int64_xrninta, 0xB012629B8C893053744A975EF67FFFFFu128, -20000000001        , 0x20); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_xrninta_361, bid128_to_int64_xrninta, 0xB012629B8C893053744A975EF6800000u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_xrninta_362, bid128_to_int64_xrninta, 0xB012629B8C893053744A975EF6800001u128, -20000000001        , 0x20); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_xrninta_363, bid128_to_int64_xrninta, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -20000000001        , 0x20); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_xrninta_364, bid128_to_int64_xrninta, 0xB012629B8C893AE9F5AE880471C00000u128, -20000000002        , 0x20); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_xrninta_365, bid128_to_int64_xrninta, 0xB012629B8C893AE9F5AE880471C00001u128, -20000000002        , 0x20); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_xrninta_366, bid128_to_int64_xrninta, 0xB018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, -35184372088832     , 0x20); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_xrninta_367, bid128_to_int64_xrninta, 0xB018AD78EBC5AC620000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_xrninta_368, bid128_to_int64_xrninta, 0xB018AD78EBC5AC620000000000000001u128, -35184372088832     , 0x20); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_xrninta_369, bid128_to_int64_xrninta, 0xB018AD78EBC5AC64B5E3AF16B187FFFFu128, -35184372088832     , 0x20); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_xrninta_370, bid128_to_int64_xrninta, 0xB018AD78EBC5AC64B5E3AF16B1880000u128, -35184372088833     , 0x20); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_xrninta_371, bid128_to_int64_xrninta, 0xB018AD78EBC5AC64B5E3AF16B1880001u128, -35184372088833     , 0x20); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_xrninta_372, bid128_to_int64_xrninta, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xrninta_373, bid128_to_int64_xrninta, 0xB01A0000000000A2E6C09AD3E0D40000u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xrninta_374, bid128_to_int64_xrninta, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xrninta_375, bid128_to_int64_xrninta, 0xB01C629B8C891B265CB1A40684E9FFFFu128, -1999999999999998   , 0x20); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_xrninta_376, bid128_to_int64_xrninta, 0xB01C629B8C891B265CB1A40684EA0000u128, -1999999999999999   , 0x20); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_xrninta_377, bid128_to_int64_xrninta, 0xB01C629B8C891B265CB1A40684EA0001u128, -1999999999999999   , 0x20); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_xrninta_378, bid128_to_int64_xrninta, 0xB01C629B8C891B2663A1FF60589BFFFFu128, -1999999999999999   , 0x20); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_xrninta_379, bid128_to_int64_xrninta, 0xB01C629B8C891B2663A1FF60589C0000u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_xrninta_380, bid128_to_int64_xrninta, 0xB01C629B8C891B2663A1FF60589C0001u128, -1999999999999999   , 0x20); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_xrninta_381, bid128_to_int64_xrninta, 0xB01C629B8C891B266A925ABA2C4DFFFFu128, -1999999999999999   , 0x20); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_xrninta_382, bid128_to_int64_xrninta, 0xB01C629B8C891B266A925ABA2C4E0000u128, -2000000000000000   , 0x20); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_xrninta_383, bid128_to_int64_xrninta, 0xB01C629B8C891B266A925ABA2C4E0001u128, -2000000000000000   , 0x20); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_xrninta_384, bid128_to_int64_xrninta, 0xB01C629B8C891B267182B613FFFFFFFFu128, -2000000000000000   , 0x20); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_xrninta_385, bid128_to_int64_xrninta, 0xB01C629B8C891B267182B61400000000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_xrninta_386, bid128_to_int64_xrninta, 0xB01C629B8C891B267182B61400000001u128, -2000000000000000   , 0x20); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_xrninta_387, bid128_to_int64_xrninta, 0xB01C629B8C891B267873116DD3B1FFFFu128, -2000000000000000   , 0x20); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_xrninta_388, bid128_to_int64_xrninta, 0xB01C629B8C891B267873116DD3B20000u128, -2000000000000001   , 0x20); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_xrninta_389, bid128_to_int64_xrninta, 0xB01C629B8C891B267873116DD3B20001u128, -2000000000000001   , 0x20); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_xrninta_390, bid128_to_int64_xrninta, 0xB01C629B8C891B267F636CC7A763FFFFu128, -2000000000000001   , 0x20); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_xrninta_391, bid128_to_int64_xrninta, 0xB01C629B8C891B267F636CC7A7640000u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_xrninta_392, bid128_to_int64_xrninta, 0xB01C629B8C891B267F636CC7A7640001u128, -2000000000000001   , 0x20); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_xrninta_393, bid128_to_int64_xrninta, 0xB01C629B8C891B268653C8217B15FFFFu128, -2000000000000001   , 0x20); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_xrninta_394, bid128_to_int64_xrninta, 0xB01C629B8C891B268653C8217B160000u128, -2000000000000002   , 0x20); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_xrninta_395, bid128_to_int64_xrninta, 0xB01C629B8C891B268653C8217B160001u128, -2000000000000002   , 0x20); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_xrninta_396, bid128_to_int64_xrninta, 0xB01E000000000001A055690D9DB7FFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_397, bid128_to_int64_xrninta, 0xB01E000000000001A055690D9DB80000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_398, bid128_to_int64_xrninta, 0xB01E000000000001A055690D9DB80001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_399, bid128_to_int64_xrninta, 0xB01E002C68AF0BB140B1A2BC2EC4FFFFu128, -35184372088832     , 0x20); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_xrninta_400, bid128_to_int64_xrninta, 0xB01E002C68AF0BB140B1A2BC2EC50000u128, -35184372088833     , 0x20); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_xrninta_401, bid128_to_int64_xrninta, 0xB01E002C68AF0BB140B1A2BC2EC50001u128, -35184372088833     , 0x20); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_xrninta_402, bid128_to_int64_xrninta, 0xB02000000000000029A2241AF62BFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_403, bid128_to_int64_xrninta, 0xB02000000000000029A2241AF62C0000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_404, bid128_to_int64_xrninta, 0xB02000000000000029A2241AF62C0001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_405, bid128_to_int64_xrninta, 0xB020000470DE4DF81FFFFFFFFFFFFFFFu128, -35184372088832     , 0x20); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_xrninta_406, bid128_to_int64_xrninta, 0xB020000470DE4DF82000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_xrninta_407, bid128_to_int64_xrninta, 0xB020000470DE4DF82000000000000001u128, -35184372088832     , 0x20); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_xrninta_408, bid128_to_int64_xrninta, 0xB02000FC6F7C4045813459C637E07FFFu128, -2000000000000000   , 0x20); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_xrninta_409, bid128_to_int64_xrninta, 0xB02000FC6F7C4045813459C637E08000u128, -2000000000000001   , 0x20); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_xrninta_410, bid128_to_int64_xrninta, 0xB02000FC6F7C4045813459C637E08001u128, -2000000000000001   , 0x20); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_xrninta_411, bid128_to_int64_xrninta, 0xB02000FC6F7C40458157E0B8A7A17FFFu128, -2000000000000001   , 0x20); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_xrninta_412, bid128_to_int64_xrninta, 0xB02000FC6F7C40458157E0B8A7A18000u128, -2000000000000002   , 0x20); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_xrninta_413, bid128_to_int64_xrninta, 0xB02000FC6F7C40458157E0B8A7A18001u128, -2000000000000002   , 0x20); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_xrninta_414, bid128_to_int64_xrninta, 0xB02200193E5939A08CE4879688D63FFFu128, -1999999999999998   , 0x20); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_xrninta_415, bid128_to_int64_xrninta, 0xB02200193E5939A08CE4879688D64000u128, -1999999999999999   , 0x20); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_xrninta_416, bid128_to_int64_xrninta, 0xB02200193E5939A08CE4879688D64001u128, -1999999999999999   , 0x20); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_xrninta_417, bid128_to_int64_xrninta, 0xB02200193E5939A08CE815152D9CBFFFu128, -1999999999999999   , 0x20); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_xrninta_418, bid128_to_int64_xrninta, 0xB02200193E5939A08CE815152D9CC000u128, -2000000000000000   , 0x20); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_xrninta_419, bid128_to_int64_xrninta, 0xB02200193E5939A08CE815152D9CC001u128, -2000000000000000   , 0x20); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_xrninta_420, bid128_to_int64_xrninta, 0xb023be33aaee97a4784b5edf879d5f3au128, -9050048220408998460, 0x20);
dec_test!(bid128_to_int64_xrninta_421, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFAABC208D63FFFu128, -9223372036854775806, 0x20); // -- -(2^63-1.5-ulp)
dec_test!(bid128_to_int64_xrninta_422, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFAABC208D64000u128, -9223372036854775807, 0x20); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_xrninta_423, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFAABC208D64001u128, -9223372036854775807, 0x20); // -- -(2^63-1.5+ulp)
dec_test!(bid128_to_int64_xrninta_424, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFC72815B397FFFu128, -9223372036854775807, 0x20); // -- -(2^63-1-ulp)
dec_test!(bid128_to_int64_xrninta_425, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFC72815B398000u128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_xrninta_426, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFC72815B398001u128, -9223372036854775807, 0x20); // -- -(2^63-1+ulp)
dec_test!(bid128_to_int64_xrninta_427, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFE3940AD9CBFFFu128, -9223372036854775807, 0x20); // -- -(2^63-0.5-ulp)
dec_test!(bid128_to_int64_xrninta_428, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFE3940AD9CC000u128, -9223372036854775808, 0x20); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_xrninta_429, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFE3940AD9CC001u128, -9223372036854775808, 0x20); // -- -(2^63-0.5+ulp)
dec_test!(bid128_to_int64_xrninta_430, bid128_to_int64_xrninta, 0xB023C6BF52633FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x20); // -- -(2^63-ulp)
dec_test!(bid128_to_int64_xrninta_431, bid128_to_int64_xrninta, 0xB023C6BF526340000000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_xrninta_432, bid128_to_int64_xrninta, 0xB023C6BF526340000000000000000001u128, -9223372036854775808, 0x20); // -- -(2^63+ulp)
dec_test!(bid128_to_int64_xrninta_433, bid128_to_int64_xrninta, 0xB023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x20); // -- -(2^63+0.5-ulp)
dec_test!(bid128_to_int64_xrninta_434, bid128_to_int64_xrninta, 0xB023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_xrninta_435, bid128_to_int64_xrninta, 0xB023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- -(2^63+0.5+ulp)
dec_test!(bid128_to_int64_xrninta_436, bid128_to_int64_xrninta, 0xB023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- -(2^63+1-ulp)
dec_test!(bid128_to_int64_xrninta_437, bid128_to_int64_xrninta, 0xB023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_xrninta_438, bid128_to_int64_xrninta, 0xB023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- -(2^63+1+ulp)
dec_test!(bid128_to_int64_xrninta_439, bid128_to_int64_xrninta, 0xB024000000000000006A94D74F42FFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_440, bid128_to_int64_xrninta, 0xB024000000000000006A94D74F430000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_441, bid128_to_int64_xrninta, 0xB024000000000000006A94D74F430001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_442, bid128_to_int64_xrninta, 0xb024308fac71eb5a79c77e013f467164u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_443, bid128_to_int64_xrninta, 0xB024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e19-ulp)
dec_test!(bid128_to_int64_xrninta_444, bid128_to_int64_xrninta, 0xB024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_xrninta_445, bid128_to_int64_xrninta, 0xB024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e19+ulp)
dec_test!(bid128_to_int64_xrninta_446, bid128_to_int64_xrninta, 0xB024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- -(1e19+0.5-ulp)
dec_test!(bid128_to_int64_xrninta_447, bid128_to_int64_xrninta, 0xB024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_xrninta_448, bid128_to_int64_xrninta, 0xB024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- -(1e19+0.5+ulp)
dec_test!(bid128_to_int64_xrninta_449, bid128_to_int64_xrninta, 0xB02449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1.5e19-ulp)
dec_test!(bid128_to_int64_xrninta_450, bid128_to_int64_xrninta, 0xB02449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_xrninta_451, bid128_to_int64_xrninta, 0xB02449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- -(1.5e19+ulp)
dec_test!(bid128_to_int64_xrninta_452, bid128_to_int64_xrninta, 0xB0245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1-ulp)
dec_test!(bid128_to_int64_xrninta_453, bid128_to_int64_xrninta, 0xB0245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_xrninta_454, bid128_to_int64_xrninta, 0xB0245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- -(2^64-1+ulp)
dec_test!(bid128_to_int64_xrninta_455, bid128_to_int64_xrninta, 0xB0245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- -(2^64-0.5-ulp)
dec_test!(bid128_to_int64_xrninta_456, bid128_to_int64_xrninta, 0xB0245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_xrninta_457, bid128_to_int64_xrninta, 0xB0245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- -(2^64-0.5+ulp)
dec_test!(bid128_to_int64_xrninta_458, bid128_to_int64_xrninta, 0xB0245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-ulp)
dec_test!(bid128_to_int64_xrninta_459, bid128_to_int64_xrninta, 0xB0245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_xrninta_460, bid128_to_int64_xrninta, 0xB0245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+ulp)
dec_test!(bid128_to_int64_xrninta_461, bid128_to_int64_xrninta, 0xB0245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- -(2^64+0.5-ulp)
dec_test!(bid128_to_int64_xrninta_462, bid128_to_int64_xrninta, 0xB0245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_xrninta_463, bid128_to_int64_xrninta, 0xB0245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- -(2^64+0.5+ulp)
dec_test!(bid128_to_int64_xrninta_464, bid128_to_int64_xrninta, 0xB0245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- -(2^64+1-ulp)
dec_test!(bid128_to_int64_xrninta_465, bid128_to_int64_xrninta, 0xB0245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_xrninta_466, bid128_to_int64_xrninta, 0xB0245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- -(2^64+1+ulp)
dec_test!(bid128_to_int64_xrninta_467, bid128_to_int64_xrninta, 0xB024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2e19-ulp)
dec_test!(bid128_to_int64_xrninta_468, bid128_to_int64_xrninta, 0xB024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_xrninta_469, bid128_to_int64_xrninta, 0xB024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- -(2e19+ulp)
dec_test!(bid128_to_int64_xrninta_470, bid128_to_int64_xrninta, 0xB0247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2.5e19-ulp)
dec_test!(bid128_to_int64_xrninta_471, bid128_to_int64_xrninta, 0xB0247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_xrninta_472, bid128_to_int64_xrninta, 0xB0247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- -(2.5e19+ulp)
dec_test!(bid128_to_int64_xrninta_473, bid128_to_int64_xrninta, 0xB026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e20-ulp)
dec_test!(bid128_to_int64_xrninta_474, bid128_to_int64_xrninta, 0xB026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_xrninta_475, bid128_to_int64_xrninta, 0xB026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e20+ulp)
dec_test!(bid128_to_int64_xrninta_476, bid128_to_int64_xrninta, 0xB02A00000000006C6B935B68D08DA3FFu128, -19999999998        , 0x20); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_xrninta_477, bid128_to_int64_xrninta, 0xB02A00000000006C6B935B68D08DA400u128, -19999999999        , 0x20); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_xrninta_478, bid128_to_int64_xrninta, 0xB02A00000000006C6B935B68D08DA401u128, -19999999999        , 0x20); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_xrninta_479, bid128_to_int64_xrninta, 0xB02A00000000006C6B935B8019048BFFu128, -19999999999        , 0x20); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_xrninta_480, bid128_to_int64_xrninta, 0xB02A00000000006C6B935B8019048C00u128, -20000000000        , 0x20); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_xrninta_481, bid128_to_int64_xrninta, 0xB02A00000000006C6B935B8019048C01u128, -20000000000        , 0x20); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_xrninta_482, bid128_to_int64_xrninta, 0xB02C000000000000000002BBA7F521FFu128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xrninta_483, bid128_to_int64_xrninta, 0xB02C000000000000000002BBA7F52200u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xrninta_484, bid128_to_int64_xrninta, 0xB02C000000000000000002BBA7F52201u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xrninta_485, bid128_to_int64_xrninta, 0xB02C00000000000AD78EBC5872141BFFu128, -19999999999        , 0x20); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_xrninta_486, bid128_to_int64_xrninta, 0xB02C00000000000AD78EBC5872141C00u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_xrninta_487, bid128_to_int64_xrninta, 0xB02C00000000000AD78EBC5872141C01u128, -19999999999        , 0x20); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_xrninta_488, bid128_to_int64_xrninta, 0xB02C00000000000AD78EBC5BF025F1FFu128, -20000000000        , 0x20); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_xrninta_489, bid128_to_int64_xrninta, 0xB02C00000000000AD78EBC5BF025F200u128, -20000000001        , 0x20); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_xrninta_490, bid128_to_int64_xrninta, 0xB02C00000000000AD78EBC5BF025F201u128, -20000000001        , 0x20); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_xrninta_491, bid128_to_int64_xrninta, 0xB02C00000000000AD78EBC5E4431D5FFu128, -20000000001        , 0x20); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_xrninta_492, bid128_to_int64_xrninta, 0xB02C00000000000AD78EBC5E4431D600u128, -20000000002        , 0x20); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_xrninta_493, bid128_to_int64_xrninta, 0xB02C00000000000AD78EBC5E4431D601u128, -20000000002        , 0x20); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_xrninta_494, bid128_to_int64_xrninta, 0xB02C000000108B2A2C28028E3FF41BFFu128, -1999999999999999   , 0x20); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_xrninta_495, bid128_to_int64_xrninta, 0xB02C000000108B2A2C28028E3FF41C00u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_xrninta_496, bid128_to_int64_xrninta, 0xB02C000000108B2A2C28028E3FF41C01u128, -1999999999999999   , 0x20); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_xrninta_497, bid128_to_int64_xrninta, 0xB02E000000000001158E46094F6AC9FFu128, -20000000001        , 0x20); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_xrninta_498, bid128_to_int64_xrninta, 0xB02E000000000001158E46094F6ACA00u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_xrninta_499, bid128_to_int64_xrninta, 0xB02E000000000001158E46094F6ACA01u128, -20000000001        , 0x20); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_xrninta_500, bid128_to_int64_xrninta, 0xB02E00000001A784379D99DB7D9AC9FFu128, -2000000000000001   , 0x20); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_xrninta_501, bid128_to_int64_xrninta, 0xB02E00000001A784379D99DB7D9ACA00u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_xrninta_502, bid128_to_int64_xrninta, 0xB02E00000001A784379D99DB7D9ACA01u128, -2000000000000001   , 0x20); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_xrninta_503, bid128_to_int64_xrninta, 0xB03000000000000000000006FC23ABFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_504, bid128_to_int64_xrninta, 0xB03000000000000000000006FC23AC00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_505, bid128_to_int64_xrninta, 0xB03000000000000000000006FC23AC01u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_506, bid128_to_int64_xrninta, 0xB03200000000000000000000B2D05DFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_507, bid128_to_int64_xrninta, 0xB03200000000000000000000B2D05E00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_508, bid128_to_int64_xrninta, 0xB03200000000000000000000B2D05E01u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_509, bid128_to_int64_xrninta, 0xB03800000000000000000000002DDA47u128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xrninta_510, bid128_to_int64_xrninta, 0xB03800000000000000000000002DDA48u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xrninta_511, bid128_to_int64_xrninta, 0xB03800000000000000000000002DDA49u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xrninta_512, bid128_to_int64_xrninta, 0xB03A00000000000000000000000003E7u128, -1                  , 0x20); // -- -(0.999)
dec_test!(bid128_to_int64_xrninta_513, bid128_to_int64_xrninta, 0xB03A00000000000000000000000495D3u128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xrninta_514, bid128_to_int64_xrninta, 0xB03A00000000000000000000000495D4u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xrninta_515, bid128_to_int64_xrninta, 0xB03A00000000000000000000000495D5u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xrninta_516, bid128_to_int64_xrninta, 0xB03C0000000000000000000000007561u128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xrninta_517, bid128_to_int64_xrninta, 0xB03C0000000000000000000000007562u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xrninta_518, bid128_to_int64_xrninta, 0xB03C0000000000000000000000007563u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xrninta_519, bid128_to_int64_xrninta, 0xB03E0000000000000000000000000005u128, -1                  , 0x20); // -- -(0.5)
dec_test!(bid128_to_int64_xrninta_520, bid128_to_int64_xrninta, 0xB03E000000000000000000000000000Fu128, -2                  , 0x20); // -- -(1.5)
dec_test!(bid128_to_int64_xrninta_521, bid128_to_int64_xrninta, 0xB03E0000000000000000000000000BB7u128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_522, bid128_to_int64_xrninta, 0xB03E0000000000000000000000000BB8u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_523, bid128_to_int64_xrninta, 0xB03E0000000000000000000000000BB9u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_524, bid128_to_int64_xrninta, 0xB03E0000000000000000000000000BBDu128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xrninta_525, bid128_to_int64_xrninta, 0xB03E0000000000000000002E90EDCFF1u128, -19999999999        , 0x20); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_xrninta_526, bid128_to_int64_xrninta, 0xB03E0000000000000000002E90EDCFFBu128, -20000000000        , 0x20); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_xrninta_527, bid128_to_int64_xrninta, 0xB03E0000000000000000002E90EDD005u128, -20000000001        , 0x20); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_xrninta_528, bid128_to_int64_xrninta, 0xB03E0000000000000000002E90EDD00Fu128, -20000000002        , 0x20); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_xrninta_529, bid128_to_int64_xrninta, 0xB03E0000000000000001400000000005u128, -35184372088833     , 0x20); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_xrninta_530, bid128_to_int64_xrninta, 0xB03E00000000000000470DE4DF81FFF1u128, -1999999999999999   , 0x20); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_xrninta_531, bid128_to_int64_xrninta, 0xB03E00000000000000470DE4DF81FFFBu128, -2000000000000000   , 0x20); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_xrninta_532, bid128_to_int64_xrninta, 0xB03E00000000000000470DE4DF820005u128, -2000000000000001   , 0x20); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_xrninta_533, bid128_to_int64_xrninta, 0xB03E00000000000000470DE4DF82000Fu128, -2000000000000002   , 0x20); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_xrninta_534, bid128_to_int64_xrninta, 0xB03E000000000004FFFFFFFFFFFFFFF1u128, -9223372036854775807, 0x20); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_xrninta_535, bid128_to_int64_xrninta, 0xB03E000000000004FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x20); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_xrninta_536, bid128_to_int64_xrninta, 0xB03E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_xrninta_537, bid128_to_int64_xrninta, 0xB03E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_xrninta_538, bid128_to_int64_xrninta, 0xB03E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_xrninta_539, bid128_to_int64_xrninta, 0xB03E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_xrninta_540, bid128_to_int64_xrninta, 0xB0400000000000000000000000000001u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_xrninta_541, bid128_to_int64_xrninta, 0xB040000000000000000000000000012Bu128, -299                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_542, bid128_to_int64_xrninta, 0xB040000000000000000000000000012Cu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_543, bid128_to_int64_xrninta, 0xB040000000000000000000000000012Du128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_544, bid128_to_int64_xrninta, 0xB04000000000000000000004A817C7FFu128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_xrninta_545, bid128_to_int64_xrninta, 0xB04000000000000000000004A817C801u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_xrninta_546, bid128_to_int64_xrninta, 0xB0400000000000000000200000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_xrninta_547, bid128_to_int64_xrninta, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_xrninta_548, bid128_to_int64_xrninta, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_xrninta_549, bid128_to_int64_xrninta, 0xB04000000000000000071AFD498D0000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_xrninta_550, bid128_to_int64_xrninta, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_xrninta_551, bid128_to_int64_xrninta, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_xrninta_552, bid128_to_int64_xrninta, 0xB0400000000000007FFFFFFFFFFFFFFFu128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_xrninta_553, bid128_to_int64_xrninta, 0xB0400000000000008000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_xrninta_554, bid128_to_int64_xrninta, 0xB0400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_xrninta_555, bid128_to_int64_xrninta, 0xB040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_xrninta_556, bid128_to_int64_xrninta, 0xB0400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_xrninta_557, bid128_to_int64_xrninta, 0xB0400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_xrninta_558, bid128_to_int64_xrninta, 0xB042000000000000000000000000001Du128, -290                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_xrninta_559, bid128_to_int64_xrninta, 0xB042000000000000000000000000001Eu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_560, bid128_to_int64_xrninta, 0xB042000000000000000000000000001Fu128, -310                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_xrninta_561, bid128_to_int64_xrninta, 0xB04200000000000000000000773593FFu128, -19999999990        , 0x00); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_xrninta_562, bid128_to_int64_xrninta, 0xB0420000000000000000000077359400u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_xrninta_563, bid128_to_int64_xrninta, 0xB0420000000000000000000077359401u128, -20000000010        , 0x00); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_xrninta_564, bid128_to_int64_xrninta, 0xB0440000000000000000000000000003u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xrninta_565, bid128_to_int64_xrninta, 0xB0520000000000000000000000000004u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_xrninta_566, bid128_to_int64_xrninta, 0xB0520000000000000000000000000005u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_xrninta_567, bid128_to_int64_xrninta, 0xB0540000000000000000000000000002u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_xrninta_568, bid128_to_int64_xrninta, 0xB05E0000000000000000000000000002u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_xrninta_569, bid128_to_int64_xrninta, 0xB064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_xrninta_570, bid128_to_int64_xrninta, 0xB0640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_xrninta_571, bid128_to_int64_xrninta, 0xB0660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_xrninta_572, bid128_to_int64_xrninta, 0xB0660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_xrninta_573, bid128_to_int64_xrninta, 0xB0680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_xrninta_574, bid128_to_int64_xrninta, 0xb11a0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_575, bid128_to_int64_xrninta, 0xb52c0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_576, bid128_to_int64_xrninta, 0xb8000000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_577, bid128_to_int64_xrninta, 0xbfd2912ade848dbee8f57deca7c2a82du128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_578, bid128_to_int64_xrninta, 0xc7c48934d84c9cada046b9cff153505cu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_579, bid128_to_int64_xrninta, 0xda3cd151b9d193bb0a1cbb21ba04d44fu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_580, bid128_to_int64_xrninta, 0xe084442273e0205337fd7ada99ef7e2bu128, 0                   , 0x00);
dec_test!(bid128_to_int64_xrninta_581, bid128_to_int64_xrninta, 0xf8000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_582, bid128_to_int64_xrninta, 0xfa4e3c7b527f3fe77fefffef6fff2dfbu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_583, bid128_to_int64_xrninta, 0xfefffdfddffdff6f1100029502004780u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_584, bid128_to_int64_xrninta, "-Infinity"                           , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_585, bid128_to_int64_xrninta, "QNaN"                                , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xrninta_586, bid128_to_int64_xrninta, "SNaN"                                , -9223372036854775808, 0x01);
