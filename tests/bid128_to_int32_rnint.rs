/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int32_rnint_001, bid128_to_int32_rnint, "-0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_rnint_002, bid128_to_int32_rnint,  "0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_rnint_003, bid128_to_int32_rnint, 0x00000000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_004, bid128_to_int32_rnint, 0x00000000000000000000403108001224u128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_005, bid128_to_int32_rnint, 0x000000000000000018d42b41099d28ecu128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_006, bid128_to_int32_rnint, 0x0001ed09bead87c0378d8e62ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_007, bid128_to_int32_rnint, 0x0001ed09bead87c0378d8e64ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_008, bid128_to_int32_rnint, "0.5"                                 , 0          , 0x00);
dec_test!(bid128_to_int32_rnint_009, bid128_to_int32_rnint, 0x052e40ff260f66e9e6b58e5d6cb3e000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_010, bid128_to_int32_rnint, 0x0e000500000582809e0ee0bab6221c03u128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_011, bid128_to_int32_rnint, "1073741824"                          , 1073741824 , 0x00);
dec_test!(bid128_to_int32_rnint_012, bid128_to_int32_rnint, 0x187a973175f6e8709d57b091f2e10220u128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_013, bid128_to_int32_rnint, "2147483648"                          , -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_014, bid128_to_int32_rnint, 0x2f28819715e6d00c9eb183d3ee2d6035u128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_015, bid128_to_int32_rnint, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_int32_rnint_016, bid128_to_int32_rnint, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_rnint_017, bid128_to_int32_rnint, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1          , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_int32_rnint_018, bid128_to_int32_rnint, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1          , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_int32_rnint_019, bid128_to_int32_rnint, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_rnint_020, bid128_to_int32_rnint, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1          , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_int32_rnint_021, bid128_to_int32_rnint, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1          , 0x00); // -- 1-ulp
dec_test!(bid128_to_int32_rnint_022, bid128_to_int32_rnint, 0x2FFE314DC6448D9338C15B0A00000000u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_rnint_023, bid128_to_int32_rnint, 0x2FFE314DC6448D9338C15B0A00000001u128, 1          , 0x00); // -- 1+ulp
dec_test!(bid128_to_int32_rnint_024, bid128_to_int32_rnint, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1          , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_int32_rnint_025, bid128_to_int32_rnint, 0x2FFE49F4A966D45CD522088F00000000u128, 2          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_rnint_026, bid128_to_int32_rnint, 0x2FFE49F4A966D45CD522088F00000001u128, 2          , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_int32_rnint_027, bid128_to_int32_rnint, 0x300000841000004018ec9ec0c2803301u128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_028, bid128_to_int32_rnint, 0x30003940000a2150001812a801093a5au128, 12         , 0x00);
dec_test!(bid128_to_int32_rnint_029, bid128_to_int32_rnint, 0x3002004030410400ffffdfffffffffefu128, 1          , 0x00);
dec_test!(bid128_to_int32_rnint_030, bid128_to_int32_rnint, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_031, bid128_to_int32_rnint, 0x300293E952CDA8B9AA44111E00000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_032, bid128_to_int32_rnint, 0x300293E952CDA8B9AA44111E00000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_033, bid128_to_int32_rnint, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rnint_034, bid128_to_int32_rnint, 0x300294286EACB8CB0A8CB6B140000000u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rnint_035, bid128_to_int32_rnint, 0x300294286EACB8CB0A8CB6B140000001u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rnint_036, bid128_to_int32_rnint, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_037, bid128_to_int32_rnint, 0x30040ECA8847C4129106CE8300000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_038, bid128_to_int32_rnint, 0x30040ECA8847C4129106CE8300000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_039, bid128_to_int32_rnint, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_040, bid128_to_int32_rnint, 0x300A0003C95A2F0B4856475FE0000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_041, bid128_to_int32_rnint, 0x300A0003C95A2F0B4856475FE0000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_042, bid128_to_int32_rnint, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_043, bid128_to_int32_rnint, 0x300C000060EF6B1ABA6F072330000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_044, bid128_to_int32_rnint, 0x300C000060EF6B1ABA6F072330000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_045, bid128_to_int32_rnint, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_rnint_046, bid128_to_int32_rnint, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483646 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_rnint_047, bid128_to_int32_rnint, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483647 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_rnint_048, bid128_to_int32_rnint, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_rnint_049, bid128_to_int32_rnint, 0x301069E10DE692B4B4B133125F000000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_rnint_050, bid128_to_int32_rnint, 0x301069E10DE692B4B4B133125F000001u128, 2147483647 , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_rnint_051, bid128_to_int32_rnint, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647 , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_rnint_052, bid128_to_int32_rnint, 0x301069E10DE6FC95C29899892F800000u128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_rnint_053, bid128_to_int32_rnint, 0x301069E10DE6FC95C29899892F800001u128, -2147483648, 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_rnint_054, bid128_to_int32_rnint, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_rnint_055, bid128_to_int32_rnint, 0x301069E10DE76676D080000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_rnint_056, bid128_to_int32_rnint, 0x301069E10DE76676D080000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_rnint_057, bid128_to_int32_rnint, 0x301069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_rnint_058, bid128_to_int32_rnint, 0x301069E10DE7D057DE676676D0800000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_rnint_059, bid128_to_int32_rnint, 0x301069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_rnint_060, bid128_to_int32_rnint, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_rnint_061, bid128_to_int32_rnint, 0x301069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_rnint_062, bid128_to_int32_rnint, 0x301069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_rnint_063, bid128_to_int32_rnint, 0x3010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- 4e9-ulp
dec_test!(bid128_to_int32_rnint_064, bid128_to_int32_rnint, 0x3010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_rnint_065, bid128_to_int32_rnint, 0x3010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- 4e9+ulp
dec_test!(bid128_to_int32_rnint_066, bid128_to_int32_rnint, 0x3010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_rnint_067, bid128_to_int32_rnint, 0x3010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_rnint_068, bid128_to_int32_rnint, 0x3010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_rnint_069, bid128_to_int32_rnint, 0x3010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_rnint_070, bid128_to_int32_rnint, 0x3010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_rnint_071, bid128_to_int32_rnint, 0x3010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_rnint_072, bid128_to_int32_rnint, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_rnint_073, bid128_to_int32_rnint, 0x3010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_rnint_074, bid128_to_int32_rnint, 0x3010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_rnint_075, bid128_to_int32_rnint, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_rnint_076, bid128_to_int32_rnint, 0x3010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_rnint_077, bid128_to_int32_rnint, 0x3010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_rnint_078, bid128_to_int32_rnint, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_rnint_079, bid128_to_int32_rnint, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_rnint_080, bid128_to_int32_rnint, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_rnint_081, bid128_to_int32_rnint, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- 5e9-ulp
dec_test!(bid128_to_int32_rnint_082, bid128_to_int32_rnint, 0x3010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_rnint_083, bid128_to_int32_rnint, 0x3010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- 5e9+ulp
dec_test!(bid128_to_int32_rnint_084, bid128_to_int32_rnint, 0x30121e40033104a0820d6c201d881900u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_085, bid128_to_int32_rnint, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_rnint_086, bid128_to_int32_rnint, 0x3012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_rnint_087, bid128_to_int32_rnint, 0x3012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_rnint_088, bid128_to_int32_rnint, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_rnint_089, bid128_to_int32_rnint, 0x3012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_rnint_090, bid128_to_int32_rnint, 0x3012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_rnint_091, bid128_to_int32_rnint, 0x3012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_rnint_092, bid128_to_int32_rnint, 0x3012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_rnint_093, bid128_to_int32_rnint, 0x3012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_rnint_094, bid128_to_int32_rnint, 0x3012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_rnint_095, bid128_to_int32_rnint, 0x3012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_rnint_096, bid128_to_int32_rnint, 0x3012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_rnint_097, bid128_to_int32_rnint, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_rnint_098, bid128_to_int32_rnint, 0x3012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_rnint_099, bid128_to_int32_rnint, 0x3012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_rnint_100, bid128_to_int32_rnint, 0x3012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_rnint_101, bid128_to_int32_rnint, 0x3012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_rnint_102, bid128_to_int32_rnint, 0x3012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_rnint_103, bid128_to_int32_rnint, 0x3012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_rnint_104, bid128_to_int32_rnint, 0x3012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_rnint_105, bid128_to_int32_rnint, 0x3012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_rnint_106, bid128_to_int32_rnint, 0x3018000200000001c086254821615311u128, 1584563251 , 0x00);
dec_test!(bid128_to_int32_rnint_107, bid128_to_int32_rnint, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_rnint_108, bid128_to_int32_rnint, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483646 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_rnint_109, bid128_to_int32_rnint, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483647 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_rnint_110, bid128_to_int32_rnint, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647 , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_rnint_111, bid128_to_int32_rnint, 0x30180002B5E3AF13FBA450E94E780000u128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_rnint_112, bid128_to_int32_rnint, 0x30180002B5E3AF13FBA450E94E780001u128, -2147483648, 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_rnint_113, bid128_to_int32_rnint, 0x30180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_rnint_114, bid128_to_int32_rnint, 0x30180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_rnint_115, bid128_to_int32_rnint, 0x30180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_rnint_116, bid128_to_int32_rnint, 0x301800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_rnint_117, bid128_to_int32_rnint, 0x301800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_rnint_118, bid128_to_int32_rnint, 0x301800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_rnint_119, bid128_to_int32_rnint, 0x301800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_rnint_120, bid128_to_int32_rnint, 0x301800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_rnint_121, bid128_to_int32_rnint, 0x301800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_rnint_122, bid128_to_int32_rnint, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rnint_123, bid128_to_int32_rnint, 0x301A0000000000A2E6C09AD3E0D40000u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rnint_124, bid128_to_int32_rnint, 0x301A0000000000A2E6C09AD3E0D40001u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rnint_125, bid128_to_int32_rnint, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483647 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_rnint_126, bid128_to_int32_rnint, 0x301A000045639181BA2CDCFB76180000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_rnint_127, bid128_to_int32_rnint, 0x301A000045639181BA2CDCFB76180001u128, 2147483647 , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_rnint_128, bid128_to_int32_rnint, 0x301A00004563918244F3FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_rnint_129, bid128_to_int32_rnint, 0x301A00004563918244F4000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_rnint_130, bid128_to_int32_rnint, 0x301A00004563918244F4000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_rnint_131, bid128_to_int32_rnint, 0x301A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_rnint_132, bid128_to_int32_rnint, 0x301A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_rnint_133, bid128_to_int32_rnint, 0x301A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_rnint_134, bid128_to_int32_rnint, 0x301A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_rnint_135, bid128_to_int32_rnint, 0x301A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_rnint_136, bid128_to_int32_rnint, 0x301A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_rnint_137, bid128_to_int32_rnint, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_rnint_138, bid128_to_int32_rnint, 0x301A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_rnint_139, bid128_to_int32_rnint, 0x301A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_rnint_140, bid128_to_int32_rnint, 0x301A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_rnint_141, bid128_to_int32_rnint, 0x301A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_rnint_142, bid128_to_int32_rnint, 0x301A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_rnint_143, bid128_to_int32_rnint, 0x301E000000000001A055690D9DB7FFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_144, bid128_to_int32_rnint, 0x301E000000000001A055690D9DB80000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_145, bid128_to_int32_rnint, 0x301E000000000001A055690D9DB80001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_146, bid128_to_int32_rnint, 0x3020000000000000211c4d300bcaadd6u128, 239        , 0x00);
dec_test!(bid128_to_int32_rnint_147, bid128_to_int32_rnint, 0x302000000000000029A2241AF62BFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_148, bid128_to_int32_rnint, 0x302000000000000029A2241AF62C0000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_149, bid128_to_int32_rnint, 0x302000000000000029A2241AF62C0001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_150, bid128_to_int32_rnint, 0x3024000000000000006A94D74F42FFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_151, bid128_to_int32_rnint, 0x3024000000000000006A94D74F430000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_152, bid128_to_int32_rnint, 0x3024000000000000006A94D74F430001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_153, bid128_to_int32_rnint, 0x302A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_rnint_154, bid128_to_int32_rnint, 0x302A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_rnint_155, bid128_to_int32_rnint, 0x302A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_rnint_156, bid128_to_int32_rnint, 0x302A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_rnint_157, bid128_to_int32_rnint, 0x302A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_rnint_158, bid128_to_int32_rnint, 0x302A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_rnint_159, bid128_to_int32_rnint, 0x302C000000000000000002BBA7F521FFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rnint_160, bid128_to_int32_rnint, 0x302C000000000000000002BBA7F52200u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rnint_161, bid128_to_int32_rnint, 0x302C000000000000000002BBA7F52201u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rnint_162, bid128_to_int32_rnint, 0x302C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_rnint_163, bid128_to_int32_rnint, 0x302C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_rnint_164, bid128_to_int32_rnint, 0x302C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_rnint_165, bid128_to_int32_rnint, 0x302C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_rnint_166, bid128_to_int32_rnint, 0x302C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_rnint_167, bid128_to_int32_rnint, 0x302C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_rnint_168, bid128_to_int32_rnint, 0x302C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_rnint_169, bid128_to_int32_rnint, 0x302C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_rnint_170, bid128_to_int32_rnint, 0x302C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_rnint_171, bid128_to_int32_rnint, 0x302E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_rnint_172, bid128_to_int32_rnint, 0x302E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_rnint_173, bid128_to_int32_rnint, 0x302E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_rnint_174, bid128_to_int32_rnint, 0x303000000000000000000006FC23ABFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_175, bid128_to_int32_rnint, 0x303000000000000000000006FC23AC00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_176, bid128_to_int32_rnint, 0x303000000000000000000006FC23AC01u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_177, bid128_to_int32_rnint, 0x303200000000000000000000B2D05DFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_178, bid128_to_int32_rnint, 0x303200000000000000000000B2D05E00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_179, bid128_to_int32_rnint, 0x303200000000000000000000B2D05E01u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_180, bid128_to_int32_rnint, 0x303800000000000000000000002DDA47u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rnint_181, bid128_to_int32_rnint, 0x303800000000000000000000002DDA48u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rnint_182, bid128_to_int32_rnint, 0x303800000000000000000000002DDA49u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rnint_183, bid128_to_int32_rnint, 0x303A00000000000000000000000003E7u128, 1          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_rnint_184, bid128_to_int32_rnint, 0x303A00000000000000000000000495D3u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rnint_185, bid128_to_int32_rnint, 0x303A00000000000000000000000495D4u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rnint_186, bid128_to_int32_rnint, 0x303A00000000000000000000000495D5u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rnint_187, bid128_to_int32_rnint, 0x303C0000000000000000000000007561u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rnint_188, bid128_to_int32_rnint, 0x303C0000000000000000000000007562u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rnint_189, bid128_to_int32_rnint, 0x303C0000000000000000000000007563u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rnint_190, bid128_to_int32_rnint, 0x303E0000000000000000000000000005u128, 0          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_rnint_191, bid128_to_int32_rnint, 0x303E000000000000000000000000000Fu128, 2          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_rnint_192, bid128_to_int32_rnint, 0x303E0000000000000000000000000BB7u128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_193, bid128_to_int32_rnint, 0x303E0000000000000000000000000BB8u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_194, bid128_to_int32_rnint, 0x303E0000000000000000000000000BB9u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_195, bid128_to_int32_rnint, 0x303E0000000000000000000000000BBDu128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rnint_196, bid128_to_int32_rnint, 0x303E00000000000000000004FFFFFFF1u128, 2147483646 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_rnint_197, bid128_to_int32_rnint, 0x303E00000000000000000004FFFFFFFBu128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_rnint_198, bid128_to_int32_rnint, 0x303E0000000000000000000500000005u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_rnint_199, bid128_to_int32_rnint, 0x303E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_rnint_200, bid128_to_int32_rnint, 0x303E0000000000000000000A00000005u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_rnint_201, bid128_to_int32_rnint, 0x303E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_rnint_202, bid128_to_int32_rnint, 0x303E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_rnint_203, bid128_to_int32_rnint, 0x303E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_rnint_204, bid128_to_int32_rnint, 0x303E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_rnint_205, bid128_to_int32_rnint, 0x30400000000000000000000000000001u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_rnint_206, bid128_to_int32_rnint, 0x3040000000000000000000000000012Bu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_207, bid128_to_int32_rnint, 0x3040000000000000000000000000012Cu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_208, bid128_to_int32_rnint, 0x3040000000000000000000000000012Du128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_209, bid128_to_int32_rnint, 0x30400000000000000000000020000000u128, 536870912  , 0x00);
dec_test!(bid128_to_int32_rnint_210, bid128_to_int32_rnint, 0x3040000000000000000000007FFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_rnint_211, bid128_to_int32_rnint, 0x30400000000000000000000080000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_rnint_212, bid128_to_int32_rnint, 0x30400000000000000000000080000001u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_rnint_213, bid128_to_int32_rnint, 0x304000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_rnint_214, bid128_to_int32_rnint, 0x30400000000000000000000100000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_rnint_215, bid128_to_int32_rnint, 0x30400000000000000000000100000001u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_rnint_216, bid128_to_int32_rnint, 0x304000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_rnint_217, bid128_to_int32_rnint, 0x304000000000000000000004A817C801u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_rnint_218, bid128_to_int32_rnint, 0x3041ED09BEAD87C0378D8E6400000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_219, bid128_to_int32_rnint, 0x3042000000000000000000000000001Du128, 290        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rnint_220, bid128_to_int32_rnint, 0x3042000000000000000000000000001Eu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_221, bid128_to_int32_rnint, 0x3042000000000000000000000000001Fu128, 310        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rnint_222, bid128_to_int32_rnint, 0x304200000000000000000000773593FFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_rnint_223, bid128_to_int32_rnint, 0x30420000000000000000000077359400u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_rnint_224, bid128_to_int32_rnint, 0x30420000000000000000000077359401u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_rnint_225, bid128_to_int32_rnint, 0x30440000000000000000000000000003u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rnint_226, bid128_to_int32_rnint, 0x30520000000000000000000000000004u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_rnint_227, bid128_to_int32_rnint, 0x30520000000000000000000000000005u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_rnint_228, bid128_to_int32_rnint, 0x30540000000000000000000000000002u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_rnint_229, bid128_to_int32_rnint, 0x34b183f22b6458ec947127aa1a36b4feu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_230, bid128_to_int32_rnint, 0x356931477136c2277ddff5ca27e6308eu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_231, bid128_to_int32_rnint, 0x35a68eda63486a6c1ad6bdd146c4a8a2u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_232, bid128_to_int32_rnint, "5.5"                                 , 6          , 0x00);
dec_test!(bid128_to_int32_rnint_233, bid128_to_int32_rnint, 0x6afb6edffdefffbddaf99f997f8a4cf7u128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_234, bid128_to_int32_rnint, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_235, bid128_to_int32_rnint, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_236, bid128_to_int32_rnint, 0x7bdb353dfe7b3b27e13f4dfe6be7f6fau128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_237, bid128_to_int32_rnint, 0x7c000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_238, bid128_to_int32_rnint, 0x7c003fffffffffff38c15b08ffffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_239, bid128_to_int32_rnint, 0x7c003fffffffffff38c15b0affffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_240, bid128_to_int32_rnint, 0x7dfbefdffffffedf0820080120000010u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_241, bid128_to_int32_rnint, 0x7e000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_242, bid128_to_int32_rnint, "-9"                                  , -9         , 0x00);
dec_test!(bid128_to_int32_rnint_243, bid128_to_int32_rnint, 0xa23d6f56f5010273ec362545a53f594au128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_244, bid128_to_int32_rnint, 0xa703ccb60ae9f3277a2d4589530817ddu128, 0          , 0x00);
dec_test!(bid128_to_int32_rnint_245, bid128_to_int32_rnint, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_int32_rnint_246, bid128_to_int32_rnint, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0          , 0x00); // -- -(0.5)
dec_test!(bid128_to_int32_rnint_247, bid128_to_int32_rnint, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1         , 0x00); // -- -(0.5+ulp)
dec_test!(bid128_to_int32_rnint_248, bid128_to_int32_rnint, 0xaffd5fbeffbffffdfdfffffdeb7bffffu128, -1         , 0x00);
dec_test!(bid128_to_int32_rnint_249, bid128_to_int32_rnint, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1         , 0x00); // -- -(0.999-ulp)
dec_test!(bid128_to_int32_rnint_250, bid128_to_int32_rnint, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1         , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_rnint_251, bid128_to_int32_rnint, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1         , 0x00); // -- -(0.999+ulp)
dec_test!(bid128_to_int32_rnint_252, bid128_to_int32_rnint, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1         , 0x00); // -- -(1-ulp)
dec_test!(bid128_to_int32_rnint_253, bid128_to_int32_rnint, 0xAFFE314DC6448D9338C15B0A00000000u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_rnint_254, bid128_to_int32_rnint, 0xAFFE314DC6448D9338C15B0A00000001u128, -1         , 0x00); // -- -(1+ulp)
dec_test!(bid128_to_int32_rnint_255, bid128_to_int32_rnint, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1         , 0x00); // -- -(1.5-ulp)
dec_test!(bid128_to_int32_rnint_256, bid128_to_int32_rnint, 0xAFFE49F4A966D45CD522088F00000000u128, -2         , 0x00); // -- -(1.5)
dec_test!(bid128_to_int32_rnint_257, bid128_to_int32_rnint, 0xAFFE49F4A966D45CD522088F00000001u128, -2         , 0x00); // -- -(1.5+ulp)
dec_test!(bid128_to_int32_rnint_258, bid128_to_int32_rnint, 0xaffe7ffffbafffffffffffbffffffbffu128, -3         , 0x00);
dec_test!(bid128_to_int32_rnint_259, bid128_to_int32_rnint, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_260, bid128_to_int32_rnint, 0xB00293E952CDA8B9AA44111E00000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_261, bid128_to_int32_rnint, 0xB00293E952CDA8B9AA44111E00000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_262, bid128_to_int32_rnint, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rnint_263, bid128_to_int32_rnint, 0xB00294286EACB8CB0A8CB6B140000000u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rnint_264, bid128_to_int32_rnint, 0xB00294286EACB8CB0A8CB6B140000001u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rnint_265, bid128_to_int32_rnint, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_266, bid128_to_int32_rnint, 0xB0040ECA8847C4129106CE8300000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_267, bid128_to_int32_rnint, 0xB0040ECA8847C4129106CE8300000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_268, bid128_to_int32_rnint, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_269, bid128_to_int32_rnint, 0xB00A0003C95A2F0B4856475FE0000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_270, bid128_to_int32_rnint, 0xB00A0003C95A2F0B4856475FE0000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_271, bid128_to_int32_rnint, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_272, bid128_to_int32_rnint, 0xB00C000060EF6B1ABA6F072330000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_273, bid128_to_int32_rnint, 0xB00C000060EF6B1ABA6F072330000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_274, bid128_to_int32_rnint, 0xb0104bf489bddad081f22ddf453e3069u128, -1540555021, 0x00);
dec_test!(bid128_to_int32_rnint_275, bid128_to_int32_rnint, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, -2147483646, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_rnint_276, bid128_to_int32_rnint, 0xB01069E10DE628D3A6C9CC9B8E800000u128, -2147483646, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_rnint_277, bid128_to_int32_rnint, 0xB01069E10DE628D3A6C9CC9B8E800001u128, -2147483647, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_rnint_278, bid128_to_int32_rnint, 0xB01069E10DE692B4B4B133125EFFFFFFu128, -2147483647, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_rnint_279, bid128_to_int32_rnint, 0xB01069E10DE692B4B4B133125F000000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_rnint_280, bid128_to_int32_rnint, 0xB01069E10DE692B4B4B133125F000001u128, -2147483647, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_rnint_281, bid128_to_int32_rnint, 0xB01069E10DE6FC95C29899892F7FFFFFu128, -2147483647, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_rnint_282, bid128_to_int32_rnint, 0xB01069E10DE6FC95C29899892F800000u128, -2147483648, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_rnint_283, bid128_to_int32_rnint, 0xB01069E10DE6FC95C29899892F800001u128, -2147483648, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_rnint_284, bid128_to_int32_rnint, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, -2147483648, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_rnint_285, bid128_to_int32_rnint, 0xB01069E10DE76676D080000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_rnint_286, bid128_to_int32_rnint, 0xB01069E10DE76676D080000000000001u128, -2147483648, 0x00); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_rnint_287, bid128_to_int32_rnint, 0xB01069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x00); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_rnint_288, bid128_to_int32_rnint, 0xB01069E10DE7D057DE676676D0800000u128, -2147483648, 0x00); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_rnint_289, bid128_to_int32_rnint, 0xB01069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_rnint_290, bid128_to_int32_rnint, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_rnint_291, bid128_to_int32_rnint, 0xB01069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_rnint_292, bid128_to_int32_rnint, 0xB01069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_rnint_293, bid128_to_int32_rnint, 0xB010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_int32_rnint_294, bid128_to_int32_rnint, 0xB010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_rnint_295, bid128_to_int32_rnint, 0xB010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_int32_rnint_296, bid128_to_int32_rnint, 0xB010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_rnint_297, bid128_to_int32_rnint, 0xB010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_rnint_298, bid128_to_int32_rnint, 0xB010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_rnint_299, bid128_to_int32_rnint, 0xB010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_rnint_300, bid128_to_int32_rnint, 0xB010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_rnint_301, bid128_to_int32_rnint, 0xB010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_rnint_302, bid128_to_int32_rnint, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_rnint_303, bid128_to_int32_rnint, 0xB010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_rnint_304, bid128_to_int32_rnint, 0xB010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_rnint_305, bid128_to_int32_rnint, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_rnint_306, bid128_to_int32_rnint, 0xB010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_rnint_307, bid128_to_int32_rnint, 0xB010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_rnint_308, bid128_to_int32_rnint, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_rnint_309, bid128_to_int32_rnint, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_rnint_310, bid128_to_int32_rnint, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_rnint_311, bid128_to_int32_rnint, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_int32_rnint_312, bid128_to_int32_rnint, 0xB010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_rnint_313, bid128_to_int32_rnint, 0xB010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_int32_rnint_314, bid128_to_int32_rnint, 0xb0115204083042acb049a005c5004083u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_315, bid128_to_int32_rnint, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_rnint_316, bid128_to_int32_rnint, 0xB012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_rnint_317, bid128_to_int32_rnint, 0xB012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_rnint_318, bid128_to_int32_rnint, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_rnint_319, bid128_to_int32_rnint, 0xB012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_rnint_320, bid128_to_int32_rnint, 0xB012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_rnint_321, bid128_to_int32_rnint, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_rnint_322, bid128_to_int32_rnint, 0xB012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_rnint_323, bid128_to_int32_rnint, 0xB012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_rnint_324, bid128_to_int32_rnint, 0xB012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_rnint_325, bid128_to_int32_rnint, 0xB012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_rnint_326, bid128_to_int32_rnint, 0xB012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_rnint_327, bid128_to_int32_rnint, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_rnint_328, bid128_to_int32_rnint, 0xB012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_rnint_329, bid128_to_int32_rnint, 0xB012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_rnint_330, bid128_to_int32_rnint, 0xB012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_rnint_331, bid128_to_int32_rnint, 0xB012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_rnint_332, bid128_to_int32_rnint, 0xB012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_rnint_333, bid128_to_int32_rnint, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_rnint_334, bid128_to_int32_rnint, 0xB012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_rnint_335, bid128_to_int32_rnint, 0xB012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_rnint_336, bid128_to_int32_rnint, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, -2147483646, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_rnint_337, bid128_to_int32_rnint, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, -2147483646, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_rnint_338, bid128_to_int32_rnint, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, -2147483647, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_rnint_339, bid128_to_int32_rnint, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, -2147483647, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_rnint_340, bid128_to_int32_rnint, 0xB0180002B5E3AF13FBA450E94E780000u128, -2147483648, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_rnint_341, bid128_to_int32_rnint, 0xB0180002B5E3AF13FBA450E94E780001u128, -2147483648, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_rnint_342, bid128_to_int32_rnint, 0xB0180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x00); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_rnint_343, bid128_to_int32_rnint, 0xB0180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x00); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_rnint_344, bid128_to_int32_rnint, 0xB0180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_rnint_345, bid128_to_int32_rnint, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_rnint_346, bid128_to_int32_rnint, 0xB01800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_rnint_347, bid128_to_int32_rnint, 0xB01800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_rnint_348, bid128_to_int32_rnint, 0xB01800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_rnint_349, bid128_to_int32_rnint, 0xB01800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_rnint_350, bid128_to_int32_rnint, 0xB01800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_rnint_351, bid128_to_int32_rnint, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rnint_352, bid128_to_int32_rnint, 0xB01A0000000000A2E6C09AD3E0D40000u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rnint_353, bid128_to_int32_rnint, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rnint_354, bid128_to_int32_rnint, 0xB01A000045639181BA2CDCFB7617FFFFu128, -2147483647, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_rnint_355, bid128_to_int32_rnint, 0xB01A000045639181BA2CDCFB76180000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_rnint_356, bid128_to_int32_rnint, 0xB01A000045639181BA2CDCFB76180001u128, -2147483647, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_rnint_357, bid128_to_int32_rnint, 0xB01A00004563918244F3FFFFFFFFFFFFu128, -2147483648, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_rnint_358, bid128_to_int32_rnint, 0xB01A00004563918244F4000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_rnint_359, bid128_to_int32_rnint, 0xB01A00004563918244F4000000000001u128, -2147483648, 0x00); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_rnint_360, bid128_to_int32_rnint, 0xB01A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_rnint_361, bid128_to_int32_rnint, 0xB01A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_rnint_362, bid128_to_int32_rnint, 0xB01A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_rnint_363, bid128_to_int32_rnint, 0xB01A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_rnint_364, bid128_to_int32_rnint, 0xB01A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_rnint_365, bid128_to_int32_rnint, 0xB01A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_rnint_366, bid128_to_int32_rnint, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_rnint_367, bid128_to_int32_rnint, 0xB01A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_rnint_368, bid128_to_int32_rnint, 0xB01A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_rnint_369, bid128_to_int32_rnint, 0xB01A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_rnint_370, bid128_to_int32_rnint, 0xB01A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_rnint_371, bid128_to_int32_rnint, 0xB01A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_rnint_372, bid128_to_int32_rnint, 0xB01E000000000001A055690D9DB7FFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_373, bid128_to_int32_rnint, 0xB01E000000000001A055690D9DB80000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_374, bid128_to_int32_rnint, 0xB01E000000000001A055690D9DB80001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_375, bid128_to_int32_rnint, 0xB02000000000000029A2241AF62BFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_376, bid128_to_int32_rnint, 0xB02000000000000029A2241AF62C0000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_377, bid128_to_int32_rnint, 0xB02000000000000029A2241AF62C0001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_378, bid128_to_int32_rnint, 0xB024000000000000006A94D74F42FFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_379, bid128_to_int32_rnint, 0xB024000000000000006A94D74F430000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_380, bid128_to_int32_rnint, 0xB024000000000000006A94D74F430001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_381, bid128_to_int32_rnint, 0xB02A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_rnint_382, bid128_to_int32_rnint, 0xB02A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_rnint_383, bid128_to_int32_rnint, 0xB02A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_rnint_384, bid128_to_int32_rnint, 0xB02A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_rnint_385, bid128_to_int32_rnint, 0xB02A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_rnint_386, bid128_to_int32_rnint, 0xB02A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_rnint_387, bid128_to_int32_rnint, 0xB02C000000000000000002BBA7F521FFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rnint_388, bid128_to_int32_rnint, 0xB02C000000000000000002BBA7F52200u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rnint_389, bid128_to_int32_rnint, 0xB02C000000000000000002BBA7F52201u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rnint_390, bid128_to_int32_rnint, 0xB02C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_rnint_391, bid128_to_int32_rnint, 0xB02C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_rnint_392, bid128_to_int32_rnint, 0xB02C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_rnint_393, bid128_to_int32_rnint, 0xB02C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_rnint_394, bid128_to_int32_rnint, 0xB02C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_rnint_395, bid128_to_int32_rnint, 0xB02C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_rnint_396, bid128_to_int32_rnint, 0xB02C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_rnint_397, bid128_to_int32_rnint, 0xB02C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_rnint_398, bid128_to_int32_rnint, 0xB02C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_rnint_399, bid128_to_int32_rnint, 0xB02E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_rnint_400, bid128_to_int32_rnint, 0xB02E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_rnint_401, bid128_to_int32_rnint, 0xB02E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_rnint_402, bid128_to_int32_rnint, 0xB03000000000000000000006FC23ABFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_403, bid128_to_int32_rnint, 0xB03000000000000000000006FC23AC00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_404, bid128_to_int32_rnint, 0xB03000000000000000000006FC23AC01u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_405, bid128_to_int32_rnint, 0xB03200000000000000000000B2D05DFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_406, bid128_to_int32_rnint, 0xB03200000000000000000000B2D05E00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_407, bid128_to_int32_rnint, 0xB03200000000000000000000B2D05E01u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_408, bid128_to_int32_rnint, 0xB03800000000000000000000002DDA47u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rnint_409, bid128_to_int32_rnint, 0xB03800000000000000000000002DDA48u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rnint_410, bid128_to_int32_rnint, 0xB03800000000000000000000002DDA49u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rnint_411, bid128_to_int32_rnint, 0xB03A00000000000000000000000003E7u128, -1         , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_rnint_412, bid128_to_int32_rnint, 0xB03A00000000000000000000000495D3u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rnint_413, bid128_to_int32_rnint, 0xB03A00000000000000000000000495D4u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rnint_414, bid128_to_int32_rnint, 0xB03A00000000000000000000000495D5u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rnint_415, bid128_to_int32_rnint, 0xB03C0000000000000000000000007561u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rnint_416, bid128_to_int32_rnint, 0xB03C0000000000000000000000007562u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rnint_417, bid128_to_int32_rnint, 0xB03C0000000000000000000000007563u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rnint_418, bid128_to_int32_rnint, 0xB03E0000000000000000000000000005u128, 0          , 0x00); // -- -(0.5)
dec_test!(bid128_to_int32_rnint_419, bid128_to_int32_rnint, 0xB03E000000000000000000000000000Fu128, -2         , 0x00); // -- -(1.5)
dec_test!(bid128_to_int32_rnint_420, bid128_to_int32_rnint, 0xB03E0000000000000000000000000BB7u128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_421, bid128_to_int32_rnint, 0xB03E0000000000000000000000000BB8u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_422, bid128_to_int32_rnint, 0xB03E0000000000000000000000000BB9u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_423, bid128_to_int32_rnint, 0xB03E0000000000000000000000000BBDu128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rnint_424, bid128_to_int32_rnint, 0xB03E00000000000000000004FFFFFFF1u128, -2147483646, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_rnint_425, bid128_to_int32_rnint, 0xB03E00000000000000000004FFFFFFFBu128, -2147483648, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_rnint_426, bid128_to_int32_rnint, 0xB03E0000000000000000000500000005u128, -2147483648, 0x00); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_rnint_427, bid128_to_int32_rnint, 0xB03E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_rnint_428, bid128_to_int32_rnint, 0xB03E0000000000000000000A00000005u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_rnint_429, bid128_to_int32_rnint, 0xB03E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_rnint_430, bid128_to_int32_rnint, 0xB03E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_rnint_431, bid128_to_int32_rnint, 0xB03E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_rnint_432, bid128_to_int32_rnint, 0xB03E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_rnint_433, bid128_to_int32_rnint, 0xB0400000000000000000000000000001u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_rnint_434, bid128_to_int32_rnint, 0xB040000000000000000000000000012Bu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_435, bid128_to_int32_rnint, 0xB040000000000000000000000000012Cu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_436, bid128_to_int32_rnint, 0xB040000000000000000000000000012Du128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_437, bid128_to_int32_rnint, 0xB040000000000000000000007FFFFFFFu128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_rnint_438, bid128_to_int32_rnint, 0xB0400000000000000000000080000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_rnint_439, bid128_to_int32_rnint, 0xB0400000000000000000000080000001u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_rnint_440, bid128_to_int32_rnint, 0xB04000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_rnint_441, bid128_to_int32_rnint, 0xB0400000000000000000000100000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_rnint_442, bid128_to_int32_rnint, 0xB0400000000000000000000100000001u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_rnint_443, bid128_to_int32_rnint, 0xB04000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_rnint_444, bid128_to_int32_rnint, 0xB04000000000000000000004A817C801u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_rnint_445, bid128_to_int32_rnint, 0xB042000000000000000000000000001Du128, -290       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rnint_446, bid128_to_int32_rnint, 0xB042000000000000000000000000001Eu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_447, bid128_to_int32_rnint, 0xB042000000000000000000000000001Fu128, -310       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rnint_448, bid128_to_int32_rnint, 0xB04200000000000000000000773593FFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_rnint_449, bid128_to_int32_rnint, 0xB0420000000000000000000077359400u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_rnint_450, bid128_to_int32_rnint, 0xB0420000000000000000000077359401u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_rnint_451, bid128_to_int32_rnint, 0xB0440000000000000000000000000003u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rnint_452, bid128_to_int32_rnint, 0xB0520000000000000000000000000004u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_rnint_453, bid128_to_int32_rnint, 0xB0520000000000000000000000000005u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_rnint_454, bid128_to_int32_rnint, 0xB0540000000000000000000000000002u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_rnint_455, bid128_to_int32_rnint, 0xb4218d946c3e7369d5c286da008b6e03u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_456, bid128_to_int32_rnint, 0xbb2fc9485875b3861884b06a1c3f63b9u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_457, bid128_to_int32_rnint, 0xcb8b7a2b4c448560d8200385efe3ffc4u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_458, bid128_to_int32_rnint, 0xfa5e3c7b527f3fffea3cc8189049fb54u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_459, bid128_to_int32_rnint, 0xffe7fffffffffbff99fd09212978d07au128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_460, bid128_to_int32_rnint, "Infinity"                            , -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_461, bid128_to_int32_rnint, "QNaN"                                , -2147483648, 0x01);
dec_test!(bid128_to_int32_rnint_462, bid128_to_int32_rnint, "SNaN"                                , -2147483648, 0x01);