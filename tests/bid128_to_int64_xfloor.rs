/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int64_xfloor_001, bid128_to_int64_xfloor, "0"                                   , 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_002, bid128_to_int64_xfloor, 0x00000000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_003, bid128_to_int64_xfloor, 0x00000000000000000000000000080002u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xfloor_004, bid128_to_int64_xfloor, 0x000000000000000010800d030a028014u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xfloor_005, bid128_to_int64_xfloor, 0x0001ed09bead87c0378d8e62ffffffffu128, 0                   , 0x20);
dec_test!(bid128_to_int64_xfloor_006, bid128_to_int64_xfloor, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_007, bid128_to_int64_xfloor, 0x03ce0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_008, bid128_to_int64_xfloor, 0x0dc00000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_009, bid128_to_int64_xfloor, 0x10d88b92c2c9e3e67f595982d66d2542u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xfloor_010, bid128_to_int64_xfloor, "+110100.11111E0"                     , 110100              , 0x20);
dec_test!(bid128_to_int64_xfloor_011, bid128_to_int64_xfloor, "+111.010011011111001E0"              , 111                 , 0x20);
dec_test!(bid128_to_int64_xfloor_012, bid128_to_int64_xfloor, 0x12980000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_013, bid128_to_int64_xfloor, "+2477887763728253.5E0"               , 2477887763728253    , 0x20);
dec_test!(bid128_to_int64_xfloor_014, bid128_to_int64_xfloor, 0x280000000000010007b797cef69ba6ffu128, 0                   , 0x20);
dec_test!(bid128_to_int64_xfloor_015, bid128_to_int64_xfloor, 0x2f160000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_016, bid128_to_int64_xfloor, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_int64_xfloor_017, bid128_to_int64_xfloor, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0                   , 0x20); // -- 0.5
dec_test!(bid128_to_int64_xfloor_018, bid128_to_int64_xfloor, 0x2FFCF684DF56C3E01BC6C73200000001u128, 0                   , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_int64_xfloor_019, bid128_to_int64_xfloor, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 0                   , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_int64_xfloor_020, bid128_to_int64_xfloor, 0x2FFDEC8B86EF679D76FC433D80000000u128, 0                   , 0x20); // -- 0.999
dec_test!(bid128_to_int64_xfloor_021, bid128_to_int64_xfloor, 0x2FFDEC8B86EF679D76FC433D80000001u128, 0                   , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_int64_xfloor_022, bid128_to_int64_xfloor, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 0                   , 0x20); // -- 1-ulp
dec_test!(bid128_to_int64_xfloor_023, bid128_to_int64_xfloor, 0x2FFE314DC6448D9338C15B0A00000000u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_xfloor_024, bid128_to_int64_xfloor, 0x2FFE314DC6448D9338C15B0A00000001u128, 1                   , 0x20); // -- 1+ulp
dec_test!(bid128_to_int64_xfloor_025, bid128_to_int64_xfloor, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1                   , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_int64_xfloor_026, bid128_to_int64_xfloor, 0x2FFE49F4A966D45CD522088F00000000u128, 1                   , 0x20); // -- 1.5
dec_test!(bid128_to_int64_xfloor_027, bid128_to_int64_xfloor, 0x2FFE49F4A966D45CD522088F00000001u128, 1                   , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_int64_xfloor_028, bid128_to_int64_xfloor, 0x300008020080000100b4500003a00205u128, 1                   , 0x20);
dec_test!(bid128_to_int64_xfloor_029, bid128_to_int64_xfloor, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_030, bid128_to_int64_xfloor, 0x300293E952CDA8B9AA44111E00000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_031, bid128_to_int64_xfloor, 0x300293E952CDA8B9AA44111E00000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_032, bid128_to_int64_xfloor, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xfloor_033, bid128_to_int64_xfloor, 0x300294286EACB8CB0A8CB6B140000000u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xfloor_034, bid128_to_int64_xfloor, 0x300294286EACB8CB0A8CB6B140000001u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xfloor_035, bid128_to_int64_xfloor, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_036, bid128_to_int64_xfloor, 0x30040ECA8847C4129106CE8300000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_037, bid128_to_int64_xfloor, 0x30040ECA8847C4129106CE8300000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_038, bid128_to_int64_xfloor, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_039, bid128_to_int64_xfloor, 0x300A0003C95A2F0B4856475FE0000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_040, bid128_to_int64_xfloor, 0x300A0003C95A2F0B4856475FE0000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_041, bid128_to_int64_xfloor, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_042, bid128_to_int64_xfloor, 0x300C000060EF6B1ABA6F072330000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_043, bid128_to_int64_xfloor, 0x300C000060EF6B1ABA6F072330000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_044, bid128_to_int64_xfloor, 0x3010C5371912364CE3056C27FFFFFFFFu128, 3999999999          , 0x20); // -- 4e9-ulp
dec_test!(bid128_to_int64_xfloor_045, bid128_to_int64_xfloor, 0x3010C5371912364CE3056C2800000000u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_xfloor_046, bid128_to_int64_xfloor, 0x3010C5371912364CE3056C2800000001u128, 4000000000          , 0x20); // -- 4e9+ulp
dec_test!(bid128_to_int64_xfloor_047, bid128_to_int64_xfloor, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 4999999999          , 0x20); // -- 5e9-ulp
dec_test!(bid128_to_int64_xfloor_048, bid128_to_int64_xfloor, 0x3010F684DF56C3E01BC6C73200000000u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_xfloor_049, bid128_to_int64_xfloor, 0x3010F684DF56C3E01BC6C73200000001u128, 5000000000          , 0x20); // -- 5e9+ulp
dec_test!(bid128_to_int64_xfloor_050, bid128_to_int64_xfloor, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 19999999998         , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_xfloor_051, bid128_to_int64_xfloor, 0x3012629B8C88FB62ED56E4238E400000u128, 19999999998         , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_int64_xfloor_052, bid128_to_int64_xfloor, 0x3012629B8C88FB62ED56E4238E400001u128, 19999999998         , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_xfloor_053, bid128_to_int64_xfloor, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 19999999998         , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_xfloor_054, bid128_to_int64_xfloor, 0x3012629B8C8905F96EBAD4C909800000u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_xfloor_055, bid128_to_int64_xfloor, 0x3012629B8C8905F96EBAD4C909800001u128, 19999999999         , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_xfloor_056, bid128_to_int64_xfloor, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 19999999999         , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_xfloor_057, bid128_to_int64_xfloor, 0x3012629B8C89108FF01EC56E84C00000u128, 19999999999         , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_int64_xfloor_058, bid128_to_int64_xfloor, 0x3012629B8C89108FF01EC56E84C00001u128, 19999999999         , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_xfloor_059, bid128_to_int64_xfloor, 0x3012629B8C891B267182B613FFFFFFFFu128, 19999999999         , 0x20); // -- 2e10-ulp
dec_test!(bid128_to_int64_xfloor_060, bid128_to_int64_xfloor, 0x3012629B8C891B267182B61400000000u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_xfloor_061, bid128_to_int64_xfloor, 0x3012629B8C891B267182B61400000001u128, 20000000000         , 0x20); // -- 2e10+ulp
dec_test!(bid128_to_int64_xfloor_062, bid128_to_int64_xfloor, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 20000000000         , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_xfloor_063, bid128_to_int64_xfloor, 0x3012629B8C8925BCF2E6A6B97B400000u128, 20000000000         , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_int64_xfloor_064, bid128_to_int64_xfloor, 0x3012629B8C8925BCF2E6A6B97B400001u128, 20000000000         , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_xfloor_065, bid128_to_int64_xfloor, 0x3012629B8C893053744A975EF67FFFFFu128, 20000000000         , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_xfloor_066, bid128_to_int64_xfloor, 0x3012629B8C893053744A975EF6800000u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_xfloor_067, bid128_to_int64_xfloor, 0x3012629B8C893053744A975EF6800001u128, 20000000001         , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_xfloor_068, bid128_to_int64_xfloor, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 20000000001         , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_xfloor_069, bid128_to_int64_xfloor, 0x3012629B8C893AE9F5AE880471C00000u128, 20000000001         , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_int64_xfloor_070, bid128_to_int64_xfloor, 0x3012629B8C893AE9F5AE880471C00001u128, 20000000001         , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_xfloor_071, bid128_to_int64_xfloor, 0x3018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 35184372088831      , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_int64_xfloor_072, bid128_to_int64_xfloor, 0x3018AD78EBC5AC620000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_xfloor_073, bid128_to_int64_xfloor, 0x3018AD78EBC5AC620000000000000001u128, 35184372088832      , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_int64_xfloor_074, bid128_to_int64_xfloor, 0x3018AD78EBC5AC64B5E3AF16B187FFFFu128, 35184372088832      , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_xfloor_075, bid128_to_int64_xfloor, 0x3018AD78EBC5AC64B5E3AF16B1880000u128, 35184372088832      , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_int64_xfloor_076, bid128_to_int64_xfloor, 0x3018AD78EBC5AC64B5E3AF16B1880001u128, 35184372088832      , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_xfloor_077, bid128_to_int64_xfloor, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xfloor_078, bid128_to_int64_xfloor, 0x301A0000000000A2E6C09AD3E0D40000u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xfloor_079, bid128_to_int64_xfloor, 0x301A0000000000A2E6C09AD3E0D40001u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xfloor_080, bid128_to_int64_xfloor, 0x301C629B8C891B265CB1A40684E9FFFFu128, 1999999999999998    , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_xfloor_081, bid128_to_int64_xfloor, 0x301C629B8C891B265CB1A40684EA0000u128, 1999999999999998    , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_int64_xfloor_082, bid128_to_int64_xfloor, 0x301C629B8C891B265CB1A40684EA0001u128, 1999999999999998    , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_xfloor_083, bid128_to_int64_xfloor, 0x301C629B8C891B2663A1FF60589BFFFFu128, 1999999999999998    , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_xfloor_084, bid128_to_int64_xfloor, 0x301C629B8C891B2663A1FF60589C0000u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_xfloor_085, bid128_to_int64_xfloor, 0x301C629B8C891B2663A1FF60589C0001u128, 1999999999999999    , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_xfloor_086, bid128_to_int64_xfloor, 0x301C629B8C891B266A925ABA2C4DFFFFu128, 1999999999999999    , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_xfloor_087, bid128_to_int64_xfloor, 0x301C629B8C891B266A925ABA2C4E0000u128, 1999999999999999    , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_int64_xfloor_088, bid128_to_int64_xfloor, 0x301C629B8C891B266A925ABA2C4E0001u128, 1999999999999999    , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_xfloor_089, bid128_to_int64_xfloor, 0x301C629B8C891B267182B613FFFFFFFFu128, 1999999999999999    , 0x20); // -- 2e15-ulp
dec_test!(bid128_to_int64_xfloor_090, bid128_to_int64_xfloor, 0x301C629B8C891B267182B61400000000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_xfloor_091, bid128_to_int64_xfloor, 0x301C629B8C891B267182B61400000001u128, 2000000000000000    , 0x20); // -- 2e15+ulp
dec_test!(bid128_to_int64_xfloor_092, bid128_to_int64_xfloor, 0x301C629B8C891B267873116DD3B1FFFFu128, 2000000000000000    , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_xfloor_093, bid128_to_int64_xfloor, 0x301C629B8C891B267873116DD3B20000u128, 2000000000000000    , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_int64_xfloor_094, bid128_to_int64_xfloor, 0x301C629B8C891B267873116DD3B20001u128, 2000000000000000    , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_xfloor_095, bid128_to_int64_xfloor, 0x301C629B8C891B267F636CC7A763FFFFu128, 2000000000000000    , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_xfloor_096, bid128_to_int64_xfloor, 0x301C629B8C891B267F636CC7A7640000u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_xfloor_097, bid128_to_int64_xfloor, 0x301C629B8C891B267F636CC7A7640001u128, 2000000000000001    , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_xfloor_098, bid128_to_int64_xfloor, 0x301C629B8C891B268653C8217B15FFFFu128, 2000000000000001    , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_xfloor_099, bid128_to_int64_xfloor, 0x301C629B8C891B268653C8217B160000u128, 2000000000000001    , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_int64_xfloor_100, bid128_to_int64_xfloor, 0x301C629B8C891B268653C8217B160001u128, 2000000000000001    , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_xfloor_101, bid128_to_int64_xfloor, 0x301E000000000001A055690D9DB7FFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_102, bid128_to_int64_xfloor, 0x301E000000000001A055690D9DB80000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_103, bid128_to_int64_xfloor, 0x301E000000000001A055690D9DB80001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_104, bid128_to_int64_xfloor, 0x301E002C68AF0BB140B1A2BC2EC4FFFFu128, 35184372088832      , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_xfloor_105, bid128_to_int64_xfloor, 0x301E002C68AF0BB140B1A2BC2EC50000u128, 35184372088832      , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_int64_xfloor_106, bid128_to_int64_xfloor, 0x301E002C68AF0BB140B1A2BC2EC50001u128, 35184372088832      , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_xfloor_107, bid128_to_int64_xfloor, 0x302000000000000029A2241AF62BFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_108, bid128_to_int64_xfloor, 0x302000000000000029A2241AF62C0000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_109, bid128_to_int64_xfloor, 0x302000000000000029A2241AF62C0001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_110, bid128_to_int64_xfloor, 0x3020000470DE4DF81FFFFFFFFFFFFFFFu128, 35184372088831      , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_int64_xfloor_111, bid128_to_int64_xfloor, 0x3020000470DE4DF82000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_xfloor_112, bid128_to_int64_xfloor, 0x3020000470DE4DF82000000000000001u128, 35184372088832      , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_int64_xfloor_113, bid128_to_int64_xfloor, 0x302000FC6F7C4045813459C637E07FFFu128, 2000000000000000    , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_xfloor_114, bid128_to_int64_xfloor, 0x302000FC6F7C4045813459C637E08000u128, 2000000000000000    , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_int64_xfloor_115, bid128_to_int64_xfloor, 0x302000FC6F7C4045813459C637E08001u128, 2000000000000000    , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_xfloor_116, bid128_to_int64_xfloor, 0x302000FC6F7C40458157E0B8A7A17FFFu128, 2000000000000001    , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_xfloor_117, bid128_to_int64_xfloor, 0x302000FC6F7C40458157E0B8A7A18000u128, 2000000000000001    , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_int64_xfloor_118, bid128_to_int64_xfloor, 0x302000FC6F7C40458157E0B8A7A18001u128, 2000000000000001    , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_xfloor_119, bid128_to_int64_xfloor, 0x302200193E5939A08CE4879688D63FFFu128, 1999999999999998    , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_xfloor_120, bid128_to_int64_xfloor, 0x302200193E5939A08CE4879688D64000u128, 1999999999999998    , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_int64_xfloor_121, bid128_to_int64_xfloor, 0x302200193E5939A08CE4879688D64001u128, 1999999999999998    , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_xfloor_122, bid128_to_int64_xfloor, 0x302200193E5939A08CE815152D9CBFFFu128, 1999999999999999    , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_xfloor_123, bid128_to_int64_xfloor, 0x302200193E5939A08CE815152D9CC000u128, 1999999999999999    , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_int64_xfloor_124, bid128_to_int64_xfloor, 0x302200193E5939A08CE815152D9CC001u128, 1999999999999999    , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_xfloor_125, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775806 , 0x20); // -- 2^63-1.5-ulp
dec_test!(bid128_to_int64_xfloor_126, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775806 , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_int64_xfloor_127, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775806 , 0x20); // -- 2^63-1.5+ulp
dec_test!(bid128_to_int64_xfloor_128, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775806 , 0x20); // -- 2^63-1-ulp
dec_test!(bid128_to_int64_xfloor_129, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFC72815B398000u128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_xfloor_130, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFC72815B398001u128, 9223372036854775807 , 0x20); // -- 2^63-1+ulp
dec_test!(bid128_to_int64_xfloor_131, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775807 , 0x20); // -- 2^63-0.5-ulp
dec_test!(bid128_to_int64_xfloor_132, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFE3940AD9CC000u128, 9223372036854775807 , 0x20); // -- 2^63-0.5
dec_test!(bid128_to_int64_xfloor_133, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFE3940AD9CC001u128, 9223372036854775807 , 0x20); // -- 2^63-0.5+ulp
dec_test!(bid128_to_int64_xfloor_134, bid128_to_int64_xfloor, 0x3023C6BF52633FFFFFFFFFFFFFFFFFFFu128, 9223372036854775807 , 0x20); // -- 2^63-ulp
dec_test!(bid128_to_int64_xfloor_135, bid128_to_int64_xfloor, 0x3023C6BF526340000000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_xfloor_136, bid128_to_int64_xfloor, 0x3023C6BF526340000000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+ulp
dec_test!(bid128_to_int64_xfloor_137, bid128_to_int64_xfloor, 0x3023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x01); // -- 2^63+0.5-ulp
dec_test!(bid128_to_int64_xfloor_138, bid128_to_int64_xfloor, 0x3023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_xfloor_139, bid128_to_int64_xfloor, 0x3023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- 2^63+0.5+ulp
dec_test!(bid128_to_int64_xfloor_140, bid128_to_int64_xfloor, 0x3023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- 2^63+1-ulp
dec_test!(bid128_to_int64_xfloor_141, bid128_to_int64_xfloor, 0x3023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_xfloor_142, bid128_to_int64_xfloor, 0x3023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- 2^63+1+ulp
dec_test!(bid128_to_int64_xfloor_143, bid128_to_int64_xfloor, 0x3023e54996869f353e2dd83188183c77u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_144, bid128_to_int64_xfloor, 0x3024000000000000006A94D74F42FFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_145, bid128_to_int64_xfloor, 0x3024000000000000006A94D74F430000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_146, bid128_to_int64_xfloor, 0x3024000000000000006A94D74F430001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_147, bid128_to_int64_xfloor, 0x3024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e19-ulp
dec_test!(bid128_to_int64_xfloor_148, bid128_to_int64_xfloor, 0x3024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_xfloor_149, bid128_to_int64_xfloor, 0x3024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e19+ulp
dec_test!(bid128_to_int64_xfloor_150, bid128_to_int64_xfloor, 0x3024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- 1e19+0.5-ulp
dec_test!(bid128_to_int64_xfloor_151, bid128_to_int64_xfloor, 0x3024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_xfloor_152, bid128_to_int64_xfloor, 0x3024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- 1e19+0.5+ulp
dec_test!(bid128_to_int64_xfloor_153, bid128_to_int64_xfloor, 0x302449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- 1.5e19-ulp
dec_test!(bid128_to_int64_xfloor_154, bid128_to_int64_xfloor, 0x302449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_xfloor_155, bid128_to_int64_xfloor, 0x302449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- 1.5e19+ulp
dec_test!(bid128_to_int64_xfloor_156, bid128_to_int64_xfloor, 0x30245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- 2^64-1-ulp
dec_test!(bid128_to_int64_xfloor_157, bid128_to_int64_xfloor, 0x30245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_xfloor_158, bid128_to_int64_xfloor, 0x30245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- 2^64-1+ulp
dec_test!(bid128_to_int64_xfloor_159, bid128_to_int64_xfloor, 0x30245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- 2^64-0.5-ulp
dec_test!(bid128_to_int64_xfloor_160, bid128_to_int64_xfloor, 0x30245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_xfloor_161, bid128_to_int64_xfloor, 0x30245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- 2^64-0.5+ulp
dec_test!(bid128_to_int64_xfloor_162, bid128_to_int64_xfloor, 0x30245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-ulp
dec_test!(bid128_to_int64_xfloor_163, bid128_to_int64_xfloor, 0x30245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_xfloor_164, bid128_to_int64_xfloor, 0x30245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+ulp
dec_test!(bid128_to_int64_xfloor_165, bid128_to_int64_xfloor, 0x30245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- 2^64+0.5-ulp
dec_test!(bid128_to_int64_xfloor_166, bid128_to_int64_xfloor, 0x30245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_xfloor_167, bid128_to_int64_xfloor, 0x30245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- 2^64+0.5+ulp
dec_test!(bid128_to_int64_xfloor_168, bid128_to_int64_xfloor, 0x30245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- 2^64+1-ulp
dec_test!(bid128_to_int64_xfloor_169, bid128_to_int64_xfloor, 0x30245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_xfloor_170, bid128_to_int64_xfloor, 0x30245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- 2^64+1+ulp
dec_test!(bid128_to_int64_xfloor_171, bid128_to_int64_xfloor, 0x3024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2e19-ulp
dec_test!(bid128_to_int64_xfloor_172, bid128_to_int64_xfloor, 0x3024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_xfloor_173, bid128_to_int64_xfloor, 0x3024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- 2e19+ulp
dec_test!(bid128_to_int64_xfloor_174, bid128_to_int64_xfloor, 0x30247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2.5e19-ulp
dec_test!(bid128_to_int64_xfloor_175, bid128_to_int64_xfloor, 0x30247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_xfloor_176, bid128_to_int64_xfloor, 0x30247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- 2.5e19+ulp
dec_test!(bid128_to_int64_xfloor_177, bid128_to_int64_xfloor, 0x3026024011101263b9febff3eeff5bf6u128, 4564070228303856186 , 0x20);
dec_test!(bid128_to_int64_xfloor_178, bid128_to_int64_xfloor, 0x3026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e20-ulp
dec_test!(bid128_to_int64_xfloor_179, bid128_to_int64_xfloor, 0x3026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_xfloor_180, bid128_to_int64_xfloor, 0x3026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e20+ulp
dec_test!(bid128_to_int64_xfloor_181, bid128_to_int64_xfloor, 0x302A00000000006C6B935B68D08DA3FFu128, 19999999998         , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_xfloor_182, bid128_to_int64_xfloor, 0x302A00000000006C6B935B68D08DA400u128, 19999999998         , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_int64_xfloor_183, bid128_to_int64_xfloor, 0x302A00000000006C6B935B68D08DA401u128, 19999999998         , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_xfloor_184, bid128_to_int64_xfloor, 0x302A00000000006C6B935B8019048BFFu128, 19999999999         , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_xfloor_185, bid128_to_int64_xfloor, 0x302A00000000006C6B935B8019048C00u128, 19999999999         , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_int64_xfloor_186, bid128_to_int64_xfloor, 0x302A00000000006C6B935B8019048C01u128, 19999999999         , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_xfloor_187, bid128_to_int64_xfloor, 0x302C000000000000000002BBA7F521FFu128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xfloor_188, bid128_to_int64_xfloor, 0x302C000000000000000002BBA7F52200u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xfloor_189, bid128_to_int64_xfloor, 0x302C000000000000000002BBA7F52201u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xfloor_190, bid128_to_int64_xfloor, 0x302C00000000000AD78EBC5872141BFFu128, 19999999998         , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_xfloor_191, bid128_to_int64_xfloor, 0x302C00000000000AD78EBC5872141C00u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_xfloor_192, bid128_to_int64_xfloor, 0x302C00000000000AD78EBC5872141C01u128, 19999999999         , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_xfloor_193, bid128_to_int64_xfloor, 0x302C00000000000AD78EBC5BF025F1FFu128, 20000000000         , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_xfloor_194, bid128_to_int64_xfloor, 0x302C00000000000AD78EBC5BF025F200u128, 20000000000         , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_int64_xfloor_195, bid128_to_int64_xfloor, 0x302C00000000000AD78EBC5BF025F201u128, 20000000000         , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_xfloor_196, bid128_to_int64_xfloor, 0x302C00000000000AD78EBC5E4431D5FFu128, 20000000001         , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_xfloor_197, bid128_to_int64_xfloor, 0x302C00000000000AD78EBC5E4431D600u128, 20000000001         , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_int64_xfloor_198, bid128_to_int64_xfloor, 0x302C00000000000AD78EBC5E4431D601u128, 20000000001         , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_xfloor_199, bid128_to_int64_xfloor, 0x302C000000108B2A2C28028E3FF41BFFu128, 1999999999999998    , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_xfloor_200, bid128_to_int64_xfloor, 0x302C000000108B2A2C28028E3FF41C00u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_xfloor_201, bid128_to_int64_xfloor, 0x302C000000108B2A2C28028E3FF41C01u128, 1999999999999999    , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_xfloor_202, bid128_to_int64_xfloor, 0x302E000000000001158E46094F6AC9FFu128, 20000000000         , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_xfloor_203, bid128_to_int64_xfloor, 0x302E000000000001158E46094F6ACA00u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_xfloor_204, bid128_to_int64_xfloor, 0x302E000000000001158E46094F6ACA01u128, 20000000001         , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_xfloor_205, bid128_to_int64_xfloor, 0x302E00000001A784379D99DB7D9AC9FFu128, 2000000000000000    , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_xfloor_206, bid128_to_int64_xfloor, 0x302E00000001A784379D99DB7D9ACA00u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_xfloor_207, bid128_to_int64_xfloor, 0x302E00000001A784379D99DB7D9ACA01u128, 2000000000000001    , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_xfloor_208, bid128_to_int64_xfloor, 0x303000000000000000000006FC23ABFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_209, bid128_to_int64_xfloor, 0x303000000000000000000006FC23AC00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_210, bid128_to_int64_xfloor, 0x303000000000000000000006FC23AC01u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_211, bid128_to_int64_xfloor, 0x303200000000000000000000B2D05DFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_212, bid128_to_int64_xfloor, 0x303200000000000000000000B2D05E00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_213, bid128_to_int64_xfloor, 0x303200000000000000000000B2D05E01u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_214, bid128_to_int64_xfloor, 0x303800000000000000000000002DDA47u128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xfloor_215, bid128_to_int64_xfloor, 0x303800000000000000000000002DDA48u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xfloor_216, bid128_to_int64_xfloor, 0x303800000000000000000000002DDA49u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xfloor_217, bid128_to_int64_xfloor, 0x303A00000000000000000000000003E7u128, 0                   , 0x20); // -- 0.999
dec_test!(bid128_to_int64_xfloor_218, bid128_to_int64_xfloor, 0x303A00000000000000000000000495D3u128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xfloor_219, bid128_to_int64_xfloor, 0x303A00000000000000000000000495D4u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xfloor_220, bid128_to_int64_xfloor, 0x303A00000000000000000000000495D5u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xfloor_221, bid128_to_int64_xfloor, 0x303C0000000000000000000000007561u128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xfloor_222, bid128_to_int64_xfloor, 0x303C0000000000000000000000007562u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xfloor_223, bid128_to_int64_xfloor, 0x303C0000000000000000000000007563u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xfloor_224, bid128_to_int64_xfloor, 0x303E0000000000000000000000000005u128, 0                   , 0x20); // -- 0.5
dec_test!(bid128_to_int64_xfloor_225, bid128_to_int64_xfloor, 0x303E000000000000000000000000000Fu128, 1                   , 0x20); // -- 1.5
dec_test!(bid128_to_int64_xfloor_226, bid128_to_int64_xfloor, 0x303E0000000000000000000000000BB7u128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_227, bid128_to_int64_xfloor, 0x303E0000000000000000000000000BB8u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_228, bid128_to_int64_xfloor, 0x303E0000000000000000000000000BB9u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_229, bid128_to_int64_xfloor, 0x303E0000000000000000000000000BBDu128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xfloor_230, bid128_to_int64_xfloor, 0x303E0000000000000000002E90EDCFF1u128, 19999999998         , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_int64_xfloor_231, bid128_to_int64_xfloor, 0x303E0000000000000000002E90EDCFFBu128, 19999999999         , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_int64_xfloor_232, bid128_to_int64_xfloor, 0x303E0000000000000000002E90EDD005u128, 20000000000         , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_int64_xfloor_233, bid128_to_int64_xfloor, 0x303E0000000000000000002E90EDD00Fu128, 20000000001         , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_int64_xfloor_234, bid128_to_int64_xfloor, 0x303E0000000000000001400000000005u128, 35184372088832      , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_int64_xfloor_235, bid128_to_int64_xfloor, 0x303E00000000000000470DE4DF81FFF1u128, 1999999999999998    , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_int64_xfloor_236, bid128_to_int64_xfloor, 0x303E00000000000000470DE4DF81FFFBu128, 1999999999999999    , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_int64_xfloor_237, bid128_to_int64_xfloor, 0x303E00000000000000470DE4DF820005u128, 2000000000000000    , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_int64_xfloor_238, bid128_to_int64_xfloor, 0x303E00000000000000470DE4DF82000Fu128, 2000000000000001    , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_int64_xfloor_239, bid128_to_int64_xfloor, 0x303E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775806 , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_int64_xfloor_240, bid128_to_int64_xfloor, 0x303E000000000004FFFFFFFFFFFFFFFBu128, 9223372036854775807 , 0x20); // -- 2^63-0.5
dec_test!(bid128_to_int64_xfloor_241, bid128_to_int64_xfloor, 0x303E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_xfloor_242, bid128_to_int64_xfloor, 0x303E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_xfloor_243, bid128_to_int64_xfloor, 0x303E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_xfloor_244, bid128_to_int64_xfloor, 0x303E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_xfloor_245, bid128_to_int64_xfloor, 0x30400000000000000000000000000001u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_xfloor_246, bid128_to_int64_xfloor, 0x3040000000000000000000000000012Bu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_247, bid128_to_int64_xfloor, 0x3040000000000000000000000000012Cu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_248, bid128_to_int64_xfloor, 0x3040000000000000000000000000012Du128, 301                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_249, bid128_to_int64_xfloor, 0x304000000000000000000004A817C7FFu128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_xfloor_250, bid128_to_int64_xfloor, 0x304000000000000000000004A817C801u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_xfloor_251, bid128_to_int64_xfloor, 0x30400000000000000000200000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_xfloor_252, bid128_to_int64_xfloor, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_xfloor_253, bid128_to_int64_xfloor, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_int64_xfloor_254, bid128_to_int64_xfloor, 0x304000000000000000071AFD498D0000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_xfloor_255, bid128_to_int64_xfloor, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_xfloor_256, bid128_to_int64_xfloor, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_int64_xfloor_257, bid128_to_int64_xfloor, 0x30400000000000000038256890002400u128, 15803729718551552   , 0x00);
dec_test!(bid128_to_int64_xfloor_258, bid128_to_int64_xfloor, 0x30400000000000006d4d47cdd1ff5e6eu128, 7876030272657907310 , 0x00);
dec_test!(bid128_to_int64_xfloor_259, bid128_to_int64_xfloor, 0x30400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_xfloor_260, bid128_to_int64_xfloor, 0x30400000000000008000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_xfloor_261, bid128_to_int64_xfloor, 0x30400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_xfloor_262, bid128_to_int64_xfloor, 0x3040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_xfloor_263, bid128_to_int64_xfloor, 0x30400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_xfloor_264, bid128_to_int64_xfloor, 0x30400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_xfloor_265, bid128_to_int64_xfloor, 0x3041ED09BEAD87C0378D8E6400000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_266, bid128_to_int64_xfloor, 0x3042000000000000000000000000001Du128, 290                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_xfloor_267, bid128_to_int64_xfloor, 0x3042000000000000000000000000001Eu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_268, bid128_to_int64_xfloor, 0x3042000000000000000000000000001Fu128, 310                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_xfloor_269, bid128_to_int64_xfloor, 0x304200000000000000000000773593FFu128, 19999999990         , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_int64_xfloor_270, bid128_to_int64_xfloor, 0x30420000000000000000000077359400u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_xfloor_271, bid128_to_int64_xfloor, 0x30420000000000000000000077359401u128, 20000000010         , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_int64_xfloor_272, bid128_to_int64_xfloor, 0x30440000000000000000000000000003u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xfloor_273, bid128_to_int64_xfloor, 0x30520000000000000000000000000004u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_xfloor_274, bid128_to_int64_xfloor, 0x30520000000000000000000000000005u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_xfloor_275, bid128_to_int64_xfloor, 0x30540000000000000000000000000002u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_xfloor_276, bid128_to_int64_xfloor, 0x305E0000000000000000000000000002u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_xfloor_277, bid128_to_int64_xfloor, 0x3064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_xfloor_278, bid128_to_int64_xfloor, 0x30640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_xfloor_279, bid128_to_int64_xfloor, 0x30660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_xfloor_280, bid128_to_int64_xfloor, 0x30660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_xfloor_281, bid128_to_int64_xfloor, 0x30680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_xfloor_282, bid128_to_int64_xfloor, 0x3163e9edfebd7826840217081a055491u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_283, bid128_to_int64_xfloor, 0x324036b0037ea5b42e8a6c20bae27d60u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_284, bid128_to_int64_xfloor, "+39.22432222297684E0"                , 39                  , 0x20);
dec_test!(bid128_to_int64_xfloor_285, bid128_to_int64_xfloor, 0x49f34800f0ec0606ef26e3a8cca1ab70u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_286, bid128_to_int64_xfloor, 0x528c125bdaefc3f76f91fee2dcd106cdu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_287, bid128_to_int64_xfloor, 0x5b36a99f0765fa416f076bbf7cecad60u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_288, bid128_to_int64_xfloor, 0x69e8cb5d7a50df87d20a87829725635eu128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_289, bid128_to_int64_xfloor, "+7658.997E0"                         , 7658                , 0x20);
dec_test!(bid128_to_int64_xfloor_290, bid128_to_int64_xfloor, 0x78000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_291, bid128_to_int64_xfloor, 0x78b2bb953a9831e733b50c7a9c29f191u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_292, bid128_to_int64_xfloor, 0x7c000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_293, bid128_to_int64_xfloor, 0x7c003fffffffffff38c15b08ffffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_294, bid128_to_int64_xfloor, 0x7c003fffffffffff38c15b0affffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_295, bid128_to_int64_xfloor, 0x7e000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_296, bid128_to_int64_xfloor, 0x87f60000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_297, bid128_to_int64_xfloor, "-887598695.897E0"                    , -887598696          , 0x20);
dec_test!(bid128_to_int64_xfloor_298, bid128_to_int64_xfloor, "-898888988998.88889E0"               , -898888988999       , 0x20);
dec_test!(bid128_to_int64_xfloor_299, bid128_to_int64_xfloor, "-8.9E0"                              , -9                  , 0x20);
dec_test!(bid128_to_int64_xfloor_300, bid128_to_int64_xfloor, 0x91a80000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_301, bid128_to_int64_xfloor, 0x9573a6e484871911e7f754295aeb9a73u128, -1                  , 0x20);
dec_test!(bid128_to_int64_xfloor_302, bid128_to_int64_xfloor, "-9"                                  , -9                  , 0x00);
dec_test!(bid128_to_int64_xfloor_303, bid128_to_int64_xfloor, "-995755778967.56677E0"               , -995755778968       , 0x20);
dec_test!(bid128_to_int64_xfloor_304, bid128_to_int64_xfloor, 0xabe20000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_305, bid128_to_int64_xfloor, 0xade40000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_306, bid128_to_int64_xfloor, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, -1                  , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_int64_xfloor_307, bid128_to_int64_xfloor, 0xAFFCF684DF56C3E01BC6C73200000000u128, -1                  , 0x20); // -- -(0.5)
dec_test!(bid128_to_int64_xfloor_308, bid128_to_int64_xfloor, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1                  , 0x20); // -- -(0.5+ulp)
dec_test!(bid128_to_int64_xfloor_309, bid128_to_int64_xfloor, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1                  , 0x20); // -- -(0.999-ulp)
dec_test!(bid128_to_int64_xfloor_310, bid128_to_int64_xfloor, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1                  , 0x20); // -- -(0.999)
dec_test!(bid128_to_int64_xfloor_311, bid128_to_int64_xfloor, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1                  , 0x20); // -- -(0.999+ulp)
dec_test!(bid128_to_int64_xfloor_312, bid128_to_int64_xfloor, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1                  , 0x20); // -- -(1-ulp)
dec_test!(bid128_to_int64_xfloor_313, bid128_to_int64_xfloor, 0xAFFE314DC6448D9338C15B0A00000000u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_xfloor_314, bid128_to_int64_xfloor, 0xAFFE314DC6448D9338C15B0A00000001u128, -2                  , 0x20); // -- -(1+ulp)
dec_test!(bid128_to_int64_xfloor_315, bid128_to_int64_xfloor, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -2                  , 0x20); // -- -(1.5-ulp)
dec_test!(bid128_to_int64_xfloor_316, bid128_to_int64_xfloor, 0xAFFE49F4A966D45CD522088F00000000u128, -2                  , 0x20); // -- -(1.5)
dec_test!(bid128_to_int64_xfloor_317, bid128_to_int64_xfloor, 0xAFFE49F4A966D45CD522088F00000001u128, -2                  , 0x20); // -- -(1.5+ulp)
dec_test!(bid128_to_int64_xfloor_318, bid128_to_int64_xfloor, 0xafff55b2cc4fb712d9fbcd3da3ca53d1u128, -7                  , 0x20);
dec_test!(bid128_to_int64_xfloor_319, bid128_to_int64_xfloor, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_320, bid128_to_int64_xfloor, 0xB00293E952CDA8B9AA44111E00000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_321, bid128_to_int64_xfloor, 0xB00293E952CDA8B9AA44111E00000001u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_322, bid128_to_int64_xfloor, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -301                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xfloor_323, bid128_to_int64_xfloor, 0xB00294286EACB8CB0A8CB6B140000000u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xfloor_324, bid128_to_int64_xfloor, 0xB00294286EACB8CB0A8CB6B140000001u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xfloor_325, bid128_to_int64_xfloor, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_326, bid128_to_int64_xfloor, 0xB0040ECA8847C4129106CE8300000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_327, bid128_to_int64_xfloor, 0xB0040ECA8847C4129106CE8300000001u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_328, bid128_to_int64_xfloor, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_329, bid128_to_int64_xfloor, 0xB00A0003C95A2F0B4856475FE0000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_330, bid128_to_int64_xfloor, 0xB00A0003C95A2F0B4856475FE0000001u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_331, bid128_to_int64_xfloor, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_332, bid128_to_int64_xfloor, 0xB00C000060EF6B1ABA6F072330000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_333, bid128_to_int64_xfloor, 0xB00C000060EF6B1ABA6F072330000001u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_334, bid128_to_int64_xfloor, 0xB010C5371912364CE3056C27FFFFFFFFu128, -4000000000         , 0x20); // -- -(4e9-ulp)
dec_test!(bid128_to_int64_xfloor_335, bid128_to_int64_xfloor, 0xB010C5371912364CE3056C2800000000u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_xfloor_336, bid128_to_int64_xfloor, 0xB010C5371912364CE3056C2800000001u128, -4000000001         , 0x20); // -- -(4e9+ulp)
dec_test!(bid128_to_int64_xfloor_337, bid128_to_int64_xfloor, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -5000000000         , 0x20); // -- -(5e9-ulp)
dec_test!(bid128_to_int64_xfloor_338, bid128_to_int64_xfloor, 0xB010F684DF56C3E01BC6C73200000000u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_xfloor_339, bid128_to_int64_xfloor, 0xB010F684DF56C3E01BC6C73200000001u128, -5000000001         , 0x20); // -- -(5e9+ulp)
dec_test!(bid128_to_int64_xfloor_340, bid128_to_int64_xfloor, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -19999999999        , 0x20); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_xfloor_341, bid128_to_int64_xfloor, 0xB012629B8C88FB62ED56E4238E400000u128, -19999999999        , 0x20); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_xfloor_342, bid128_to_int64_xfloor, 0xB012629B8C88FB62ED56E4238E400001u128, -19999999999        , 0x20); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_xfloor_343, bid128_to_int64_xfloor, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -19999999999        , 0x20); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_xfloor_344, bid128_to_int64_xfloor, 0xB012629B8C8905F96EBAD4C909800000u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_xfloor_345, bid128_to_int64_xfloor, 0xB012629B8C8905F96EBAD4C909800001u128, -20000000000        , 0x20); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_xfloor_346, bid128_to_int64_xfloor, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -20000000000        , 0x20); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_xfloor_347, bid128_to_int64_xfloor, 0xB012629B8C89108FF01EC56E84C00000u128, -20000000000        , 0x20); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_xfloor_348, bid128_to_int64_xfloor, 0xB012629B8C89108FF01EC56E84C00001u128, -20000000000        , 0x20); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_xfloor_349, bid128_to_int64_xfloor, 0xB012629B8C891B267182B613FFFFFFFFu128, -20000000000        , 0x20); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_xfloor_350, bid128_to_int64_xfloor, 0xB012629B8C891B267182B61400000000u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_xfloor_351, bid128_to_int64_xfloor, 0xB012629B8C891B267182B61400000001u128, -20000000001        , 0x20); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_xfloor_352, bid128_to_int64_xfloor, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -20000000001        , 0x20); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_xfloor_353, bid128_to_int64_xfloor, 0xB012629B8C8925BCF2E6A6B97B400000u128, -20000000001        , 0x20); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_xfloor_354, bid128_to_int64_xfloor, 0xB012629B8C8925BCF2E6A6B97B400001u128, -20000000001        , 0x20); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_xfloor_355, bid128_to_int64_xfloor, 0xB012629B8C893053744A975EF67FFFFFu128, -20000000001        , 0x20); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_xfloor_356, bid128_to_int64_xfloor, 0xB012629B8C893053744A975EF6800000u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_xfloor_357, bid128_to_int64_xfloor, 0xB012629B8C893053744A975EF6800001u128, -20000000002        , 0x20); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_xfloor_358, bid128_to_int64_xfloor, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -20000000002        , 0x20); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_xfloor_359, bid128_to_int64_xfloor, 0xB012629B8C893AE9F5AE880471C00000u128, -20000000002        , 0x20); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_xfloor_360, bid128_to_int64_xfloor, 0xB012629B8C893AE9F5AE880471C00001u128, -20000000002        , 0x20); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_xfloor_361, bid128_to_int64_xfloor, 0xb017786ee66597f5d6cdc9959ca3a35au128, -7634972413221      , 0x20);
dec_test!(bid128_to_int64_xfloor_362, bid128_to_int64_xfloor, 0xB018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, -35184372088832     , 0x20); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_xfloor_363, bid128_to_int64_xfloor, 0xB018AD78EBC5AC620000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_xfloor_364, bid128_to_int64_xfloor, 0xB018AD78EBC5AC620000000000000001u128, -35184372088833     , 0x20); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_xfloor_365, bid128_to_int64_xfloor, 0xB018AD78EBC5AC64B5E3AF16B187FFFFu128, -35184372088833     , 0x20); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_xfloor_366, bid128_to_int64_xfloor, 0xB018AD78EBC5AC64B5E3AF16B1880000u128, -35184372088833     , 0x20); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_xfloor_367, bid128_to_int64_xfloor, 0xB018AD78EBC5AC64B5E3AF16B1880001u128, -35184372088833     , 0x20); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_xfloor_368, bid128_to_int64_xfloor, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -301                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xfloor_369, bid128_to_int64_xfloor, 0xB01A0000000000A2E6C09AD3E0D40000u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xfloor_370, bid128_to_int64_xfloor, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xfloor_371, bid128_to_int64_xfloor, 0xB01C629B8C891B265CB1A40684E9FFFFu128, -1999999999999999   , 0x20); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_xfloor_372, bid128_to_int64_xfloor, 0xB01C629B8C891B265CB1A40684EA0000u128, -1999999999999999   , 0x20); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_xfloor_373, bid128_to_int64_xfloor, 0xB01C629B8C891B265CB1A40684EA0001u128, -1999999999999999   , 0x20); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_xfloor_374, bid128_to_int64_xfloor, 0xB01C629B8C891B2663A1FF60589BFFFFu128, -1999999999999999   , 0x20); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_xfloor_375, bid128_to_int64_xfloor, 0xB01C629B8C891B2663A1FF60589C0000u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_xfloor_376, bid128_to_int64_xfloor, 0xB01C629B8C891B2663A1FF60589C0001u128, -2000000000000000   , 0x20); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_xfloor_377, bid128_to_int64_xfloor, 0xB01C629B8C891B266A925ABA2C4DFFFFu128, -2000000000000000   , 0x20); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_xfloor_378, bid128_to_int64_xfloor, 0xB01C629B8C891B266A925ABA2C4E0000u128, -2000000000000000   , 0x20); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_xfloor_379, bid128_to_int64_xfloor, 0xB01C629B8C891B266A925ABA2C4E0001u128, -2000000000000000   , 0x20); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_xfloor_380, bid128_to_int64_xfloor, 0xB01C629B8C891B267182B613FFFFFFFFu128, -2000000000000000   , 0x20); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_xfloor_381, bid128_to_int64_xfloor, 0xB01C629B8C891B267182B61400000000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_xfloor_382, bid128_to_int64_xfloor, 0xB01C629B8C891B267182B61400000001u128, -2000000000000001   , 0x20); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_xfloor_383, bid128_to_int64_xfloor, 0xB01C629B8C891B267873116DD3B1FFFFu128, -2000000000000001   , 0x20); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_xfloor_384, bid128_to_int64_xfloor, 0xB01C629B8C891B267873116DD3B20000u128, -2000000000000001   , 0x20); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_xfloor_385, bid128_to_int64_xfloor, 0xB01C629B8C891B267873116DD3B20001u128, -2000000000000001   , 0x20); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_xfloor_386, bid128_to_int64_xfloor, 0xB01C629B8C891B267F636CC7A763FFFFu128, -2000000000000001   , 0x20); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_xfloor_387, bid128_to_int64_xfloor, 0xB01C629B8C891B267F636CC7A7640000u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_xfloor_388, bid128_to_int64_xfloor, 0xB01C629B8C891B267F636CC7A7640001u128, -2000000000000002   , 0x20); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_xfloor_389, bid128_to_int64_xfloor, 0xB01C629B8C891B268653C8217B15FFFFu128, -2000000000000002   , 0x20); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_xfloor_390, bid128_to_int64_xfloor, 0xB01C629B8C891B268653C8217B160000u128, -2000000000000002   , 0x20); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_xfloor_391, bid128_to_int64_xfloor, 0xB01C629B8C891B268653C8217B160001u128, -2000000000000002   , 0x20); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_xfloor_392, bid128_to_int64_xfloor, 0xB01E000000000001A055690D9DB7FFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_393, bid128_to_int64_xfloor, 0xB01E000000000001A055690D9DB80000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_394, bid128_to_int64_xfloor, 0xB01E000000000001A055690D9DB80001u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_395, bid128_to_int64_xfloor, 0xB01E002C68AF0BB140B1A2BC2EC4FFFFu128, -35184372088833     , 0x20); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_xfloor_396, bid128_to_int64_xfloor, 0xB01E002C68AF0BB140B1A2BC2EC50000u128, -35184372088833     , 0x20); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_xfloor_397, bid128_to_int64_xfloor, 0xB01E002C68AF0BB140B1A2BC2EC50001u128, -35184372088833     , 0x20); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_xfloor_398, bid128_to_int64_xfloor, 0xB02000000000000029A2241AF62BFFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_399, bid128_to_int64_xfloor, 0xB02000000000000029A2241AF62C0000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_400, bid128_to_int64_xfloor, 0xB02000000000000029A2241AF62C0001u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_401, bid128_to_int64_xfloor, 0xB020000470DE4DF81FFFFFFFFFFFFFFFu128, -35184372088832     , 0x20); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_xfloor_402, bid128_to_int64_xfloor, 0xB020000470DE4DF82000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_xfloor_403, bid128_to_int64_xfloor, 0xB020000470DE4DF82000000000000001u128, -35184372088833     , 0x20); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_xfloor_404, bid128_to_int64_xfloor, 0xB02000FC6F7C4045813459C637E07FFFu128, -2000000000000001   , 0x20); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_xfloor_405, bid128_to_int64_xfloor, 0xB02000FC6F7C4045813459C637E08000u128, -2000000000000001   , 0x20); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_xfloor_406, bid128_to_int64_xfloor, 0xB02000FC6F7C4045813459C637E08001u128, -2000000000000001   , 0x20); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_xfloor_407, bid128_to_int64_xfloor, 0xB02000FC6F7C40458157E0B8A7A17FFFu128, -2000000000000002   , 0x20); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_xfloor_408, bid128_to_int64_xfloor, 0xB02000FC6F7C40458157E0B8A7A18000u128, -2000000000000002   , 0x20); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_xfloor_409, bid128_to_int64_xfloor, 0xB02000FC6F7C40458157E0B8A7A18001u128, -2000000000000002   , 0x20); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_xfloor_410, bid128_to_int64_xfloor, 0xB02200193E5939A08CE4879688D63FFFu128, -1999999999999999   , 0x20); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_xfloor_411, bid128_to_int64_xfloor, 0xB02200193E5939A08CE4879688D64000u128, -1999999999999999   , 0x20); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_xfloor_412, bid128_to_int64_xfloor, 0xB02200193E5939A08CE4879688D64001u128, -1999999999999999   , 0x20); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_xfloor_413, bid128_to_int64_xfloor, 0xB02200193E5939A08CE815152D9CBFFFu128, -2000000000000000   , 0x20); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_xfloor_414, bid128_to_int64_xfloor, 0xB02200193E5939A08CE815152D9CC000u128, -2000000000000000   , 0x20); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_xfloor_415, bid128_to_int64_xfloor, 0xB02200193E5939A08CE815152D9CC001u128, -2000000000000000   , 0x20); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_xfloor_416, bid128_to_int64_xfloor, 0xb0230ee265068413d73fff9c0cebc8bdu128, -5494187423577440003, 0x20);
dec_test!(bid128_to_int64_xfloor_417, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFAABC208D63FFFu128, -9223372036854775807, 0x20); // -- -(2^63-1.5-ulp)
dec_test!(bid128_to_int64_xfloor_418, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFAABC208D64000u128, -9223372036854775807, 0x20); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_xfloor_419, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFAABC208D64001u128, -9223372036854775807, 0x20); // -- -(2^63-1.5+ulp)
dec_test!(bid128_to_int64_xfloor_420, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFC72815B397FFFu128, -9223372036854775807, 0x20); // -- -(2^63-1-ulp)
dec_test!(bid128_to_int64_xfloor_421, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFC72815B398000u128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_xfloor_422, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFC72815B398001u128, -9223372036854775808, 0x20); // -- -(2^63-1+ulp)
dec_test!(bid128_to_int64_xfloor_423, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFE3940AD9CBFFFu128, -9223372036854775808, 0x20); // -- -(2^63-0.5-ulp)
dec_test!(bid128_to_int64_xfloor_424, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFE3940AD9CC000u128, -9223372036854775808, 0x20); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_xfloor_425, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFE3940AD9CC001u128, -9223372036854775808, 0x20); // -- -(2^63-0.5+ulp)
dec_test!(bid128_to_int64_xfloor_426, bid128_to_int64_xfloor, 0xB023C6BF52633FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x20); // -- -(2^63-ulp)
dec_test!(bid128_to_int64_xfloor_427, bid128_to_int64_xfloor, 0xB023C6BF526340000000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_xfloor_428, bid128_to_int64_xfloor, 0xB023C6BF526340000000000000000001u128, -9223372036854775808, 0x01); // -- -(2^63+ulp)
dec_test!(bid128_to_int64_xfloor_429, bid128_to_int64_xfloor, 0xB023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x01); // -- -(2^63+0.5-ulp)
dec_test!(bid128_to_int64_xfloor_430, bid128_to_int64_xfloor, 0xB023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_xfloor_431, bid128_to_int64_xfloor, 0xB023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- -(2^63+0.5+ulp)
dec_test!(bid128_to_int64_xfloor_432, bid128_to_int64_xfloor, 0xB023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- -(2^63+1-ulp)
dec_test!(bid128_to_int64_xfloor_433, bid128_to_int64_xfloor, 0xB023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_xfloor_434, bid128_to_int64_xfloor, 0xB023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- -(2^63+1+ulp)
dec_test!(bid128_to_int64_xfloor_435, bid128_to_int64_xfloor, 0xb023c6f3c7ae9278301c914978d26d33u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_436, bid128_to_int64_xfloor, 0xB024000000000000006A94D74F42FFFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_437, bid128_to_int64_xfloor, 0xB024000000000000006A94D74F430000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_438, bid128_to_int64_xfloor, 0xB024000000000000006A94D74F430001u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_439, bid128_to_int64_xfloor, 0xB024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e19-ulp)
dec_test!(bid128_to_int64_xfloor_440, bid128_to_int64_xfloor, 0xB024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_xfloor_441, bid128_to_int64_xfloor, 0xB024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e19+ulp)
dec_test!(bid128_to_int64_xfloor_442, bid128_to_int64_xfloor, 0xB024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- -(1e19+0.5-ulp)
dec_test!(bid128_to_int64_xfloor_443, bid128_to_int64_xfloor, 0xB024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_xfloor_444, bid128_to_int64_xfloor, 0xB024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- -(1e19+0.5+ulp)
dec_test!(bid128_to_int64_xfloor_445, bid128_to_int64_xfloor, 0xB02449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1.5e19-ulp)
dec_test!(bid128_to_int64_xfloor_446, bid128_to_int64_xfloor, 0xB02449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_xfloor_447, bid128_to_int64_xfloor, 0xB02449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- -(1.5e19+ulp)
dec_test!(bid128_to_int64_xfloor_448, bid128_to_int64_xfloor, 0xB0245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1-ulp)
dec_test!(bid128_to_int64_xfloor_449, bid128_to_int64_xfloor, 0xB0245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_xfloor_450, bid128_to_int64_xfloor, 0xB0245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- -(2^64-1+ulp)
dec_test!(bid128_to_int64_xfloor_451, bid128_to_int64_xfloor, 0xB0245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- -(2^64-0.5-ulp)
dec_test!(bid128_to_int64_xfloor_452, bid128_to_int64_xfloor, 0xB0245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_xfloor_453, bid128_to_int64_xfloor, 0xB0245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- -(2^64-0.5+ulp)
dec_test!(bid128_to_int64_xfloor_454, bid128_to_int64_xfloor, 0xB0245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-ulp)
dec_test!(bid128_to_int64_xfloor_455, bid128_to_int64_xfloor, 0xB0245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_xfloor_456, bid128_to_int64_xfloor, 0xB0245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+ulp)
dec_test!(bid128_to_int64_xfloor_457, bid128_to_int64_xfloor, 0xB0245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- -(2^64+0.5-ulp)
dec_test!(bid128_to_int64_xfloor_458, bid128_to_int64_xfloor, 0xB0245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_xfloor_459, bid128_to_int64_xfloor, 0xB0245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- -(2^64+0.5+ulp)
dec_test!(bid128_to_int64_xfloor_460, bid128_to_int64_xfloor, 0xB0245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- -(2^64+1-ulp)
dec_test!(bid128_to_int64_xfloor_461, bid128_to_int64_xfloor, 0xB0245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_xfloor_462, bid128_to_int64_xfloor, 0xB0245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- -(2^64+1+ulp)
dec_test!(bid128_to_int64_xfloor_463, bid128_to_int64_xfloor, 0xB024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2e19-ulp)
dec_test!(bid128_to_int64_xfloor_464, bid128_to_int64_xfloor, 0xB024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_xfloor_465, bid128_to_int64_xfloor, 0xB024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- -(2e19+ulp)
dec_test!(bid128_to_int64_xfloor_466, bid128_to_int64_xfloor, 0xB0247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2.5e19-ulp)
dec_test!(bid128_to_int64_xfloor_467, bid128_to_int64_xfloor, 0xB0247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_xfloor_468, bid128_to_int64_xfloor, 0xB0247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- -(2.5e19+ulp)
dec_test!(bid128_to_int64_xfloor_469, bid128_to_int64_xfloor, 0xB026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e20-ulp)
dec_test!(bid128_to_int64_xfloor_470, bid128_to_int64_xfloor, 0xB026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_xfloor_471, bid128_to_int64_xfloor, 0xB026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e20+ulp)
dec_test!(bid128_to_int64_xfloor_472, bid128_to_int64_xfloor, 0xB02A00000000006C6B935B68D08DA3FFu128, -19999999999        , 0x20); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_xfloor_473, bid128_to_int64_xfloor, 0xB02A00000000006C6B935B68D08DA400u128, -19999999999        , 0x20); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_xfloor_474, bid128_to_int64_xfloor, 0xB02A00000000006C6B935B68D08DA401u128, -19999999999        , 0x20); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_xfloor_475, bid128_to_int64_xfloor, 0xB02A00000000006C6B935B8019048BFFu128, -20000000000        , 0x20); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_xfloor_476, bid128_to_int64_xfloor, 0xB02A00000000006C6B935B8019048C00u128, -20000000000        , 0x20); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_xfloor_477, bid128_to_int64_xfloor, 0xB02A00000000006C6B935B8019048C01u128, -20000000000        , 0x20); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_xfloor_478, bid128_to_int64_xfloor, 0xB02C000000000000000002BBA7F521FFu128, -301                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xfloor_479, bid128_to_int64_xfloor, 0xB02C000000000000000002BBA7F52200u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xfloor_480, bid128_to_int64_xfloor, 0xB02C000000000000000002BBA7F52201u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xfloor_481, bid128_to_int64_xfloor, 0xB02C00000000000AD78EBC5872141BFFu128, -19999999999        , 0x20); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_xfloor_482, bid128_to_int64_xfloor, 0xB02C00000000000AD78EBC5872141C00u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_xfloor_483, bid128_to_int64_xfloor, 0xB02C00000000000AD78EBC5872141C01u128, -20000000000        , 0x20); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_xfloor_484, bid128_to_int64_xfloor, 0xB02C00000000000AD78EBC5BF025F1FFu128, -20000000001        , 0x20); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_xfloor_485, bid128_to_int64_xfloor, 0xB02C00000000000AD78EBC5BF025F200u128, -20000000001        , 0x20); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_xfloor_486, bid128_to_int64_xfloor, 0xB02C00000000000AD78EBC5BF025F201u128, -20000000001        , 0x20); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_xfloor_487, bid128_to_int64_xfloor, 0xB02C00000000000AD78EBC5E4431D5FFu128, -20000000002        , 0x20); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_xfloor_488, bid128_to_int64_xfloor, 0xB02C00000000000AD78EBC5E4431D600u128, -20000000002        , 0x20); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_xfloor_489, bid128_to_int64_xfloor, 0xB02C00000000000AD78EBC5E4431D601u128, -20000000002        , 0x20); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_xfloor_490, bid128_to_int64_xfloor, 0xB02C000000108B2A2C28028E3FF41BFFu128, -1999999999999999   , 0x20); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_xfloor_491, bid128_to_int64_xfloor, 0xB02C000000108B2A2C28028E3FF41C00u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_xfloor_492, bid128_to_int64_xfloor, 0xB02C000000108B2A2C28028E3FF41C01u128, -2000000000000000   , 0x20); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_xfloor_493, bid128_to_int64_xfloor, 0xB02E000000000001158E46094F6AC9FFu128, -20000000001        , 0x20); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_xfloor_494, bid128_to_int64_xfloor, 0xB02E000000000001158E46094F6ACA00u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_xfloor_495, bid128_to_int64_xfloor, 0xB02E000000000001158E46094F6ACA01u128, -20000000002        , 0x20); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_xfloor_496, bid128_to_int64_xfloor, 0xB02E00000001A784379D99DB7D9AC9FFu128, -2000000000000001   , 0x20); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_xfloor_497, bid128_to_int64_xfloor, 0xB02E00000001A784379D99DB7D9ACA00u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_xfloor_498, bid128_to_int64_xfloor, 0xB02E00000001A784379D99DB7D9ACA01u128, -2000000000000002   , 0x20); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_xfloor_499, bid128_to_int64_xfloor, 0xB03000000000000000000006FC23ABFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_500, bid128_to_int64_xfloor, 0xB03000000000000000000006FC23AC00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_501, bid128_to_int64_xfloor, 0xB03000000000000000000006FC23AC01u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_502, bid128_to_int64_xfloor, 0xB03200000000000000000000B2D05DFFu128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_503, bid128_to_int64_xfloor, 0xB03200000000000000000000B2D05E00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_504, bid128_to_int64_xfloor, 0xB03200000000000000000000B2D05E01u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_505, bid128_to_int64_xfloor, 0xB03800000000000000000000002DDA47u128, -301                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xfloor_506, bid128_to_int64_xfloor, 0xB03800000000000000000000002DDA48u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xfloor_507, bid128_to_int64_xfloor, 0xB03800000000000000000000002DDA49u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xfloor_508, bid128_to_int64_xfloor, 0xB03A00000000000000000000000003E7u128, -1                  , 0x20); // -- -(0.999)
dec_test!(bid128_to_int64_xfloor_509, bid128_to_int64_xfloor, 0xB03A00000000000000000000000495D3u128, -301                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xfloor_510, bid128_to_int64_xfloor, 0xB03A00000000000000000000000495D4u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xfloor_511, bid128_to_int64_xfloor, 0xB03A00000000000000000000000495D5u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xfloor_512, bid128_to_int64_xfloor, 0xB03C0000000000000000000000007561u128, -301                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xfloor_513, bid128_to_int64_xfloor, 0xB03C0000000000000000000000007562u128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xfloor_514, bid128_to_int64_xfloor, 0xB03C0000000000000000000000007563u128, -301                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xfloor_515, bid128_to_int64_xfloor, 0xB03E0000000000000000000000000005u128, -1                  , 0x20); // -- -(0.5)
dec_test!(bid128_to_int64_xfloor_516, bid128_to_int64_xfloor, 0xB03E000000000000000000000000000Fu128, -2                  , 0x20); // -- -(1.5)
dec_test!(bid128_to_int64_xfloor_517, bid128_to_int64_xfloor, 0xB03E0000000000000000000000000BB7u128, -300                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_518, bid128_to_int64_xfloor, 0xB03E0000000000000000000000000BB8u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_519, bid128_to_int64_xfloor, 0xB03E0000000000000000000000000BB9u128, -301                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_520, bid128_to_int64_xfloor, 0xB03E0000000000000000000000000BBDu128, -301                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xfloor_521, bid128_to_int64_xfloor, 0xB03E0000000000000000002E90EDCFF1u128, -19999999999        , 0x20); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_xfloor_522, bid128_to_int64_xfloor, 0xB03E0000000000000000002E90EDCFFBu128, -20000000000        , 0x20); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_xfloor_523, bid128_to_int64_xfloor, 0xB03E0000000000000000002E90EDD005u128, -20000000001        , 0x20); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_xfloor_524, bid128_to_int64_xfloor, 0xB03E0000000000000000002E90EDD00Fu128, -20000000002        , 0x20); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_xfloor_525, bid128_to_int64_xfloor, 0xB03E0000000000000001400000000005u128, -35184372088833     , 0x20); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_xfloor_526, bid128_to_int64_xfloor, 0xB03E00000000000000470DE4DF81FFF1u128, -1999999999999999   , 0x20); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_xfloor_527, bid128_to_int64_xfloor, 0xB03E00000000000000470DE4DF81FFFBu128, -2000000000000000   , 0x20); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_xfloor_528, bid128_to_int64_xfloor, 0xB03E00000000000000470DE4DF820005u128, -2000000000000001   , 0x20); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_xfloor_529, bid128_to_int64_xfloor, 0xB03E00000000000000470DE4DF82000Fu128, -2000000000000002   , 0x20); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_xfloor_530, bid128_to_int64_xfloor, 0xB03E000000000004FFFFFFFFFFFFFFF1u128, -9223372036854775807, 0x20); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_xfloor_531, bid128_to_int64_xfloor, 0xB03E000000000004FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x20); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_xfloor_532, bid128_to_int64_xfloor, 0xB03E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_xfloor_533, bid128_to_int64_xfloor, 0xB03E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_xfloor_534, bid128_to_int64_xfloor, 0xB03E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_xfloor_535, bid128_to_int64_xfloor, 0xB03E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_xfloor_536, bid128_to_int64_xfloor, 0xB0400000000000000000000000000001u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_xfloor_537, bid128_to_int64_xfloor, 0xB040000000000000000000000000012Bu128, -299                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_538, bid128_to_int64_xfloor, 0xB040000000000000000000000000012Cu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_539, bid128_to_int64_xfloor, 0xB040000000000000000000000000012Du128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_540, bid128_to_int64_xfloor, 0xB04000000000000000000004A817C7FFu128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_xfloor_541, bid128_to_int64_xfloor, 0xB04000000000000000000004A817C801u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_xfloor_542, bid128_to_int64_xfloor, 0xB0400000000000000000200000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_xfloor_543, bid128_to_int64_xfloor, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_xfloor_544, bid128_to_int64_xfloor, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_xfloor_545, bid128_to_int64_xfloor, 0xB04000000000000000071AFD498D0000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_xfloor_546, bid128_to_int64_xfloor, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_xfloor_547, bid128_to_int64_xfloor, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_xfloor_548, bid128_to_int64_xfloor, 0xB0400000000000007FFFFFFFFFFFFFFFu128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_xfloor_549, bid128_to_int64_xfloor, 0xB0400000000000008000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_xfloor_550, bid128_to_int64_xfloor, 0xB0400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_xfloor_551, bid128_to_int64_xfloor, 0xB040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_xfloor_552, bid128_to_int64_xfloor, 0xB0400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_xfloor_553, bid128_to_int64_xfloor, 0xB0400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_xfloor_554, bid128_to_int64_xfloor, 0xB042000000000000000000000000001Du128, -290                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_xfloor_555, bid128_to_int64_xfloor, 0xB042000000000000000000000000001Eu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_556, bid128_to_int64_xfloor, 0xB042000000000000000000000000001Fu128, -310                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_xfloor_557, bid128_to_int64_xfloor, 0xB04200000000000000000000773593FFu128, -19999999990        , 0x00); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_xfloor_558, bid128_to_int64_xfloor, 0xB0420000000000000000000077359400u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_xfloor_559, bid128_to_int64_xfloor, 0xB0420000000000000000000077359401u128, -20000000010        , 0x00); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_xfloor_560, bid128_to_int64_xfloor, 0xB0440000000000000000000000000003u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xfloor_561, bid128_to_int64_xfloor, 0xB0520000000000000000000000000004u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_xfloor_562, bid128_to_int64_xfloor, 0xB0520000000000000000000000000005u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_xfloor_563, bid128_to_int64_xfloor, 0xB0540000000000000000000000000002u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_xfloor_564, bid128_to_int64_xfloor, 0xB05E0000000000000000000000000002u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_xfloor_565, bid128_to_int64_xfloor, 0xB064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_xfloor_566, bid128_to_int64_xfloor, 0xB0640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_xfloor_567, bid128_to_int64_xfloor, 0xB0660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_xfloor_568, bid128_to_int64_xfloor, 0xB0660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_xfloor_569, bid128_to_int64_xfloor, 0xB0680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_xfloor_570, bid128_to_int64_xfloor, 0xc4620000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_571, bid128_to_int64_xfloor, 0xc50daac0c347558764b7dda1eb14db7eu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_572, bid128_to_int64_xfloor, 0xc6898d466cd0d68921eb5accfdab2966u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_573, bid128_to_int64_xfloor, 0xc7080724873b51594109c436d38e560du128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_574, bid128_to_int64_xfloor, 0xd0da0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xfloor_575, bid128_to_int64_xfloor, 0xd4aaef72ae947901c776ad6a27f3ee25u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_576, bid128_to_int64_xfloor, 0xd5cb93fedb80368c32dde6a3b69b5bd4u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_577, bid128_to_int64_xfloor, 0xdf4ea61f4cf8f3317b2982d60e3742dcu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_578, bid128_to_int64_xfloor, 0xfab9d9eeffd59ffd56b6ec177cc2cce2u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_579, bid128_to_int64_xfloor, 0xfdffb77ff5bffffbb7d2019c15fb1077u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_580, bid128_to_int64_xfloor, 0xffffbfcef7ffefdb0820002000000800u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_581, bid128_to_int64_xfloor, "-Infinity"                           , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_582, bid128_to_int64_xfloor, "Infinity"                            , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_583, bid128_to_int64_xfloor, "QNaN"                                , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xfloor_584, bid128_to_int64_xfloor, "SNaN"                                , -9223372036854775808, 0x01);
