/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_uint32_001, bid128_to_uint32_rnint, "-0"                                  , 0            , 0x00);
dec_test!(bid128_to_uint32_002, bid128_to_uint32_rnint, 0x00000000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_003, bid128_to_uint32_rnint, 0x00000000000000000000000800001200u128, 0            , 0x00);
dec_test!(bid128_to_uint32_004, bid128_to_uint32_rnint, 0x0000000000000000a138746e31c01009u128, 0            , 0x00);
dec_test!(bid128_to_uint32_005, bid128_to_uint32_rnint, 0x0001ed09bead87c0378d8e62ffffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_006, bid128_to_uint32_rnint, 0x0001ed09bead87c0378d8e64ffffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_007, bid128_to_uint32_rnint, 0x01200000000000007af626f7bf37ce9bu128, 0            , 0x00);
dec_test!(bid128_to_uint32_008, bid128_to_uint32_rnint, 0x081fdfac17793ad8ffffffff7fffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_009, bid128_to_uint32_rnint, 0x0f998fb207514890eb3cb78b20dd7e7fu128, 0            , 0x00);
dec_test!(bid128_to_uint32_010, bid128_to_uint32_rnint, 1073741824                            , 1073741824   , 0x00);
dec_test!(bid128_to_uint32_011, bid128_to_uint32_rnint, 1                                     , 1            , 0x00);
dec_test!(bid128_to_uint32_012, bid128_to_uint32_rnint, 0x173a3b47d6f18423fffff73ff7aeefffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_013, bid128_to_uint32_rnint, 0x1c460000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_014, bid128_to_uint32_rnint, 0x1ccc212c0128566ddafd7e52499bf035u128, 0            , 0x00);
dec_test!(bid128_to_uint32_015, bid128_to_uint32_rnint, 0x1ef00000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_016, bid128_to_uint32_rnint, 0x26bcf5e801c36421f1725e5a5b636742u128, 0            , 0x00);
dec_test!(bid128_to_uint32_017, bid128_to_uint32_rnint, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_uint32_018, bid128_to_uint32_rnint, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0            , 0x00); // -- 0.5
dec_test!(bid128_to_uint32_019, bid128_to_uint32_rnint, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1            , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_uint32_020, bid128_to_uint32_rnint, 0x2ffdb4e937c8d3990000040000000000u128, 1            , 0x00);
dec_test!(bid128_to_uint32_021, bid128_to_uint32_rnint, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1            , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_uint32_022, bid128_to_uint32_rnint, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1            , 0x00); // -- 0.999
dec_test!(bid128_to_uint32_023, bid128_to_uint32_rnint, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1            , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_uint32_024, bid128_to_uint32_rnint, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1            , 0x00); // -- 1-ulp
dec_test!(bid128_to_uint32_025, bid128_to_uint32_rnint, 0x2FFE314DC6448D9338C15B0A00000000u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_026, bid128_to_uint32_rnint, 0x2FFE314DC6448D9338C15B0A00000001u128, 1            , 0x00); // -- 1+ulp
dec_test!(bid128_to_uint32_027, bid128_to_uint32_rnint, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_028, bid128_to_uint32_rnint, 0x2FFE49F4A966D45CD522088F00000000u128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_029, bid128_to_uint32_rnint, 0x2FFE49F4A966D45CD522088F00000001u128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_030, bid128_to_uint32_rnint, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_031, bid128_to_uint32_rnint, 0x300293E952CDA8B9AA44111E00000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_032, bid128_to_uint32_rnint, 0x300293E952CDA8B9AA44111E00000001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_033, bid128_to_uint32_rnint, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_034, bid128_to_uint32_rnint, 0x300294286EACB8CB0A8CB6B140000000u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_035, bid128_to_uint32_rnint, 0x300294286EACB8CB0A8CB6B140000001u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_036, bid128_to_uint32_rnint, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_037, bid128_to_uint32_rnint, 0x30040ECA8847C4129106CE8300000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_038, bid128_to_uint32_rnint, 0x30040ECA8847C4129106CE8300000001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_039, bid128_to_uint32_rnint, 0x3006082200890122dc2fb197063dd1b5u128, 1650         , 0x00);
dec_test!(bid128_to_uint32_040, bid128_to_uint32_rnint, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_041, bid128_to_uint32_rnint, 0x300A0003C95A2F0B4856475FE0000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_042, bid128_to_uint32_rnint, 0x300A0003C95A2F0B4856475FE0000001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_043, bid128_to_uint32_rnint, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_044, bid128_to_uint32_rnint, 0x300C000060EF6B1ABA6F072330000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_045, bid128_to_uint32_rnint, 0x300C000060EF6B1ABA6F072330000001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_046, bid128_to_uint32_rnint, 0x3010421006b8c60253380e68050fefa2u128, 1339908765   , 0x00);
dec_test!(bid128_to_uint32_047, bid128_to_uint32_rnint, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646   , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_048, bid128_to_uint32_rnint, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483646   , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_uint32_049, bid128_to_uint32_rnint, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483647   , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_050, bid128_to_uint32_rnint, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483647   , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_051, bid128_to_uint32_rnint, 0x301069E10DE692B4B4B133125F000000u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_052, bid128_to_uint32_rnint, 0x301069E10DE692B4B4B133125F000001u128, 2147483647   , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_053, bid128_to_uint32_rnint, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647   , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_054, bid128_to_uint32_rnint, 0x301069E10DE6FC95C29899892F800000u128, 2147483648   , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_uint32_055, bid128_to_uint32_rnint, 0x301069E10DE6FC95C29899892F800001u128, 2147483648   , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_056, bid128_to_uint32_rnint, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_uint32_057, bid128_to_uint32_rnint, 0x301069E10DE76676D080000000000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_058, bid128_to_uint32_rnint, 0x301069E10DE76676D080000000000001u128, 2147483648   , 0x00); // -- 2^31+ulp
dec_test!(bid128_to_uint32_059, bid128_to_uint32_rnint, 0x301069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x00); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_060, bid128_to_uint32_rnint, 0x301069E10DE7D057DE676676D0800000u128, 2147483648   , 0x00); // -- 2^31+0.5
dec_test!(bid128_to_uint32_061, bid128_to_uint32_rnint, 0x301069E10DE7D057DE676676D0800001u128, 2147483649   , 0x00); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_062, bid128_to_uint32_rnint, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483649   , 0x00); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_063, bid128_to_uint32_rnint, 0x301069E10DE83A38EC4ECCEDA1000000u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_064, bid128_to_uint32_rnint, 0x301069E10DE83A38EC4ECCEDA1000001u128, 2147483649   , 0x00); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_065, bid128_to_uint32_rnint, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000   , 0x00); // -- 4e9-ulp
dec_test!(bid128_to_uint32_066, bid128_to_uint32_rnint, 0x3010C5371912364CE3056C2800000000u128, 4000000000   , 0x00); // -- 4e9
dec_test!(bid128_to_uint32_067, bid128_to_uint32_rnint, 0x3010C5371912364CE3056C2800000001u128, 4000000000   , 0x00); // -- 4e9+ulp
dec_test!(bid128_to_uint32_068, bid128_to_uint32_rnint, 0x3010D3C21BCDF92B853133125EFFFFFFu128, 4294967295   , 0x00); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_069, bid128_to_uint32_rnint, 0x3010D3C21BCDF92B853133125F000000u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_070, bid128_to_uint32_rnint, 0x3010D3C21BCDF92B853133125F000001u128, 4294967295   , 0x00); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_071, bid128_to_uint32_rnint, 0x3010D3C21BCE630C931899892F7FFFFFu128, 4294967295   , 0x00); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_072, bid128_to_uint32_rnint, 0x3010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_073, bid128_to_uint32_rnint, 0x3010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_074, bid128_to_uint32_rnint, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_uint32_075, bid128_to_uint32_rnint, 0x3010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_076, bid128_to_uint32_rnint, 0x3010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_077, bid128_to_uint32_rnint, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_078, bid128_to_uint32_rnint, 0x3010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_079, bid128_to_uint32_rnint, 0x3010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_080, bid128_to_uint32_rnint, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_081, bid128_to_uint32_rnint, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_082, bid128_to_uint32_rnint, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_083, bid128_to_uint32_rnint, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); // -- 5e9-ulp
dec_test!(bid128_to_uint32_084, bid128_to_uint32_rnint, 0x3010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- 5e9
dec_test!(bid128_to_uint32_085, bid128_to_uint32_rnint, 0x3010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- 5e9+ulp
dec_test!(bid128_to_uint32_086, bid128_to_uint32_rnint, 0x3011028102430001ffffffffffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_087, bid128_to_uint32_rnint, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_088, bid128_to_uint32_rnint, 0x3012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_089, bid128_to_uint32_rnint, 0x3012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_090, bid128_to_uint32_rnint, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_091, bid128_to_uint32_rnint, 0x3012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_092, bid128_to_uint32_rnint, 0x3012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_093, bid128_to_uint32_rnint, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_094, bid128_to_uint32_rnint, 0x3012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_095, bid128_to_uint32_rnint, 0x3012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_096, bid128_to_uint32_rnint, 0x3012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_uint32_097, bid128_to_uint32_rnint, 0x3012629B8C891B267182B61400000000u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_098, bid128_to_uint32_rnint, 0x3012629B8C891B267182B61400000001u128, 2147483648   , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_uint32_099, bid128_to_uint32_rnint, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_100, bid128_to_uint32_rnint, 0x3012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_101, bid128_to_uint32_rnint, 0x3012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_102, bid128_to_uint32_rnint, 0x3012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_103, bid128_to_uint32_rnint, 0x3012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_104, bid128_to_uint32_rnint, 0x3012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_105, bid128_to_uint32_rnint, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_106, bid128_to_uint32_rnint, 0x3012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_107, bid128_to_uint32_rnint, 0x3012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_108, bid128_to_uint32_rnint, 0x301600000000003627E8F712373BFFFFu128, 1            , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_uint32_109, bid128_to_uint32_rnint, 0x301600000000003627E8F712373C0000u128, 1            , 0x00); // -- 0.999
dec_test!(bid128_to_uint32_110, bid128_to_uint32_rnint, 0x301600000000003627E8F712373C0001u128, 1            , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_uint32_111, bid128_to_uint32_rnint, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646   , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_112, bid128_to_uint32_rnint, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483646   , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_uint32_113, bid128_to_uint32_rnint, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483647   , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_114, bid128_to_uint32_rnint, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647   , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_115, bid128_to_uint32_rnint, 0x30180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_uint32_116, bid128_to_uint32_rnint, 0x30180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_117, bid128_to_uint32_rnint, 0x30180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x00); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_118, bid128_to_uint32_rnint, 0x30180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x00); // -- 2^31+0.5
dec_test!(bid128_to_uint32_119, bid128_to_uint32_rnint, 0x30180002B5E3AF19676BAF16B1880001u128, 2147483649   , 0x00); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_120, bid128_to_uint32_rnint, 0x301800056BC75E2AAD2C50E94E77FFFFu128, 4294967295   , 0x00); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_121, bid128_to_uint32_rnint, 0x301800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_122, bid128_to_uint32_rnint, 0x301800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_123, bid128_to_uint32_rnint, 0x301800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_124, bid128_to_uint32_rnint, 0x301800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_125, bid128_to_uint32_rnint, 0x301800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_126, bid128_to_uint32_rnint, 0x301A0000000000004563918244F3FFFFu128, 0            , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_uint32_127, bid128_to_uint32_rnint, 0x301A0000000000004563918244F40000u128, 0            , 0x00); // -- 0.5
dec_test!(bid128_to_uint32_128, bid128_to_uint32_rnint, 0x301A0000000000004563918244F40001u128, 1            , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_uint32_129, bid128_to_uint32_rnint, 0x301A0000000000008AC7230489E7FFFFu128, 1            , 0x00); // -- 1-ulp
dec_test!(bid128_to_uint32_130, bid128_to_uint32_rnint, 0x301A0000000000008AC7230489E80000u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_131, bid128_to_uint32_rnint, 0x301A0000000000008AC7230489E80001u128, 1            , 0x00); // -- 1+ulp
dec_test!(bid128_to_uint32_132, bid128_to_uint32_rnint, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_133, bid128_to_uint32_rnint, 0x301A0000000000A2E6C09AD3E0D40000u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_134, bid128_to_uint32_rnint, 0x301A0000000000A2E6C09AD3E0D40001u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_135, bid128_to_uint32_rnint, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483647   , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_136, bid128_to_uint32_rnint, 0x301A000045639181BA2CDCFB76180000u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_137, bid128_to_uint32_rnint, 0x301A000045639181BA2CDCFB76180001u128, 2147483647   , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_138, bid128_to_uint32_rnint, 0x301A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_uint32_139, bid128_to_uint32_rnint, 0x301A00004563918244F4000000000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_140, bid128_to_uint32_rnint, 0x301A00004563918244F4000000000001u128, 2147483648   , 0x00); // -- 2^31+ulp
dec_test!(bid128_to_uint32_141, bid128_to_uint32_rnint, 0x301A000045639182CFBB230489E7FFFFu128, 2147483649   , 0x00); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_142, bid128_to_uint32_rnint, 0x301A000045639182CFBB230489E80000u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_143, bid128_to_uint32_rnint, 0x301A000045639182CFBB230489E80001u128, 2147483649   , 0x00); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_144, bid128_to_uint32_rnint, 0x301A00008AC72303FF20DCFB7617FFFFu128, 4294967295   , 0x00); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_145, bid128_to_uint32_rnint, 0x301A00008AC72303FF20DCFB76180000u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_146, bid128_to_uint32_rnint, 0x301A00008AC72303FF20DCFB76180001u128, 4294967295   , 0x00); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_147, bid128_to_uint32_rnint, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_uint32_148, bid128_to_uint32_rnint, 0x301A00008AC7230489E8000000000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_149, bid128_to_uint32_rnint, 0x301A00008AC7230489E8000000000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_150, bid128_to_uint32_rnint, 0x301A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_151, bid128_to_uint32_rnint, 0x301A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_152, bid128_to_uint32_rnint, 0x301A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_153, bid128_to_uint32_rnint, 0x301C00000000000014D1120D7B15FFFFu128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_154, bid128_to_uint32_rnint, 0x301C00000000000014D1120D7B160000u128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_155, bid128_to_uint32_rnint, 0x301C00000000000014D1120D7B160001u128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_156, bid128_to_uint32_rnint, 0x301E000000000001A055690D9DB7FFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_157, bid128_to_uint32_rnint, 0x301E000000000001A055690D9DB80000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_158, bid128_to_uint32_rnint, 0x301E000000000001A055690D9DB80001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_159, bid128_to_uint32_rnint, 0x302000000000000029A2241AF62BFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_160, bid128_to_uint32_rnint, 0x302000000000000029A2241AF62C0000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_161, bid128_to_uint32_rnint, 0x302000000000000029A2241AF62C0001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_162, bid128_to_uint32_rnint, 0x302000000021400028004460c2808304u128, 0xEF976DAEu32, 0x00);
dec_test!(bid128_to_uint32_163, bid128_to_uint32_rnint, 0x3024000000000000006A94D74F42FFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_164, bid128_to_uint32_rnint, 0x3024000000000000006A94D74F430000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_165, bid128_to_uint32_rnint, 0x3024000000000000006A94D74F430001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_166, bid128_to_uint32_rnint, 0x302A00000000000000000017428106FFu128, 1            , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_uint32_167, bid128_to_uint32_rnint, 0x302A0000000000000000001742810700u128, 1            , 0x00); // -- 0.999
dec_test!(bid128_to_uint32_168, bid128_to_uint32_rnint, 0x302A0000000000000000001742810701u128, 1            , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_uint32_169, bid128_to_uint32_rnint, 0x302A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_170, bid128_to_uint32_rnint, 0x302A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_171, bid128_to_uint32_rnint, 0x302A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_172, bid128_to_uint32_rnint, 0x302A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_173, bid128_to_uint32_rnint, 0x302A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_174, bid128_to_uint32_rnint, 0x302A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_175, bid128_to_uint32_rnint, 0x302C000000000000000002BBA7F521FFu128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_176, bid128_to_uint32_rnint, 0x302C000000000000000002BBA7F52200u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_177, bid128_to_uint32_rnint, 0x302C000000000000000002BBA7F52201u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_178, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_179, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_180, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_181, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_182, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_183, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_184, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_185, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_186, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_187, bid128_to_uint32_rnint, 0x302E000000000000000000001DCD64FFu128, 0            , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_uint32_188, bid128_to_uint32_rnint, 0x302E000000000000000000001DCD6500u128, 0            , 0x00); // -- 0.5
dec_test!(bid128_to_uint32_189, bid128_to_uint32_rnint, 0x302E000000000000000000001DCD6501u128, 1            , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_uint32_190, bid128_to_uint32_rnint, 0x302E000000000000000000003B9AC9FFu128, 1            , 0x00); // -- 1-ulp
dec_test!(bid128_to_uint32_191, bid128_to_uint32_rnint, 0x302E000000000000000000003B9ACA00u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_192, bid128_to_uint32_rnint, 0x302E000000000000000000003B9ACA01u128, 1            , 0x00); // -- 1+ulp
dec_test!(bid128_to_uint32_193, bid128_to_uint32_rnint, 0x302E0000000000000000000059682EFFu128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_194, bid128_to_uint32_rnint, 0x302E0000000000000000000059682F00u128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_195, bid128_to_uint32_rnint, 0x302E0000000000000000000059682F01u128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_196, bid128_to_uint32_rnint, 0x302E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_197, bid128_to_uint32_rnint, 0x302E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_198, bid128_to_uint32_rnint, 0x302E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_199, bid128_to_uint32_rnint, 0x303000000000000000000006FC23ABFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_200, bid128_to_uint32_rnint, 0x303000000000000000000006FC23AC00u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_201, bid128_to_uint32_rnint, 0x303000000000000000000006FC23AC01u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_202, bid128_to_uint32_rnint, 0x303200000000000000000000B2D05DFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_203, bid128_to_uint32_rnint, 0x303200000000000000000000B2D05E00u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_204, bid128_to_uint32_rnint, 0x303200000000000000000000B2D05E01u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_205, bid128_to_uint32_rnint, 0x303800000000000000000000002DDA47u128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_206, bid128_to_uint32_rnint, 0x303800000000000000000000002DDA48u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_207, bid128_to_uint32_rnint, 0x303800000000000000000000002DDA49u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_208, bid128_to_uint32_rnint, 0x303A00000000000000000000000003E7u128, 1            , 0x00); // -- 0.999
dec_test!(bid128_to_uint32_209, bid128_to_uint32_rnint, 0x303A00000000000000000000000005DBu128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_210, bid128_to_uint32_rnint, 0x303A00000000000000000000000005DCu128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_211, bid128_to_uint32_rnint, 0x303A00000000000000000000000005DDu128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_212, bid128_to_uint32_rnint, 0x303A00000000000000000000000495D3u128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_213, bid128_to_uint32_rnint, 0x303A00000000000000000000000495D4u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_214, bid128_to_uint32_rnint, 0x303A00000000000000000000000495D5u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_215, bid128_to_uint32_rnint, 0x303C0000000000000000000000000095u128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_216, bid128_to_uint32_rnint, 0x303C0000000000000000000000000096u128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_217, bid128_to_uint32_rnint, 0x303C0000000000000000000000000097u128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_218, bid128_to_uint32_rnint, 0x303C0000000000000000000000007561u128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_219, bid128_to_uint32_rnint, 0x303C0000000000000000000000007562u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_220, bid128_to_uint32_rnint, 0x303C0000000000000000000000007563u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_221, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFF69u128, 2147483646   , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_222, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFF6Au128, 2147483646   , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_uint32_223, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFF6Bu128, 2147483647   , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_224, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFFCDu128, 2147483647   , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_225, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFFCEu128, 2147483648   , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_uint32_226, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFFCFu128, 2147483648   , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_227, bid128_to_uint32_rnint, 0x303C0000000000000000003200000031u128, 2147483648   , 0x00); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_228, bid128_to_uint32_rnint, 0x303C0000000000000000003200000032u128, 2147483648   , 0x00); // -- 2^31+0.5
dec_test!(bid128_to_uint32_229, bid128_to_uint32_rnint, 0x303C0000000000000000003200000033u128, 2147483649   , 0x00); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_230, bid128_to_uint32_rnint, 0x303C00000000000000000063FFFFFFCDu128, 4294967295   , 0x00); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_231, bid128_to_uint32_rnint, 0x303C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_232, bid128_to_uint32_rnint, 0x303C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_233, bid128_to_uint32_rnint, 0x303C0000000000000000006400000031u128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_234, bid128_to_uint32_rnint, 0x303C0000000000000000006400000032u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_235, bid128_to_uint32_rnint, 0x303C0000000000000000006400000033u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_236, bid128_to_uint32_rnint, 0x303E0000000000000000000000000005u128, 0            , 0x00); // -- 0.5
dec_test!(bid128_to_uint32_237, bid128_to_uint32_rnint, 0x303E000000000000000000000000000Fu128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_238, bid128_to_uint32_rnint, 0x303E0000000000000000000000000BB7u128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_239, bid128_to_uint32_rnint, 0x303E0000000000000000000000000BB8u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_240, bid128_to_uint32_rnint, 0x303E0000000000000000000000000BB9u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_241, bid128_to_uint32_rnint, 0x303E0000000000000000000000000BBDu128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_242, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFF1u128, 2147483646   , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_uint32_243, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFF5u128, 2147483647   , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_244, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFF6u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_245, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFF7u128, 2147483647   , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_246, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFFBu128, 2147483648   , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_uint32_247, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFFFu128, 2147483648   , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_uint32_248, bid128_to_uint32_rnint, 0x303E0000000000000000000500000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_249, bid128_to_uint32_rnint, 0x303E0000000000000000000500000001u128, 2147483648   , 0x00); // -- 2^31+ulp
dec_test!(bid128_to_uint32_250, bid128_to_uint32_rnint, 0x303E0000000000000000000500000005u128, 2147483648   , 0x00); // -- 2^31+0.5
dec_test!(bid128_to_uint32_251, bid128_to_uint32_rnint, 0x303E0000000000000000000500000009u128, 2147483649   , 0x00); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_252, bid128_to_uint32_rnint, 0x303E000000000000000000050000000Au128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_253, bid128_to_uint32_rnint, 0x303E000000000000000000050000000Bu128, 2147483649   , 0x00); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_254, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFF5u128, 4294967295   , 0x00); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_255, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFF6u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_256, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFF7u128, 4294967295   , 0x00); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_257, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_258, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_uint32_259, bid128_to_uint32_rnint, 0x303E0000000000000000000A00000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_260, bid128_to_uint32_rnint, 0x303E0000000000000000000A00000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_261, bid128_to_uint32_rnint, 0x303E0000000000000000000A00000005u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_262, bid128_to_uint32_rnint, 0x303E0000000000000000000A00000009u128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_263, bid128_to_uint32_rnint, 0x303E0000000000000000000A0000000Au128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_264, bid128_to_uint32_rnint, 0x303E0000000000000000000A0000000Bu128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_265, bid128_to_uint32_rnint, 0x303E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_266, bid128_to_uint32_rnint, 0x303E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_267, bid128_to_uint32_rnint, 0x303E0000000000000000002E90EDD005u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_268, bid128_to_uint32_rnint, 0x303E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_269, bid128_to_uint32_rnint, 0x30400000000000000000000000000001u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_270, bid128_to_uint32_rnint, 0x3040000000000000000000000000012Bu128, 299          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_271, bid128_to_uint32_rnint, 0x3040000000000000000000000000012Cu128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_272, bid128_to_uint32_rnint, 0x3040000000000000000000000000012Du128, 301          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_273, bid128_to_uint32_rnint, 0x3040000000000000000000007FFFFFFFu128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_274, bid128_to_uint32_rnint, 0x30400000000000000000000080000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_275, bid128_to_uint32_rnint, 0x30400000000000000000000080000001u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_276, bid128_to_uint32_rnint, 0x304000000000000000000000FFFFFFFFu128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_277, bid128_to_uint32_rnint, 0x30400000000000000000000100000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_278, bid128_to_uint32_rnint, 0x30400000000000000000000100000001u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_279, bid128_to_uint32_rnint, 0x304000000000000000000004A817C7FFu128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_280, bid128_to_uint32_rnint, 0x304000000000000000000004A817C801u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_281, bid128_to_uint32_rnint, 0x3041ED09BEAD87C0378D8E6400000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_282, bid128_to_uint32_rnint, 0x3042000000000000000000000000001Du128, 290          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_283, bid128_to_uint32_rnint, 0x3042000000000000000000000000001Eu128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_284, bid128_to_uint32_rnint, 0x3042000000000000000000000000001Fu128, 310          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_285, bid128_to_uint32_rnint, 0x304200000000000000000000773593FFu128, 2147483648   , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_uint32_286, bid128_to_uint32_rnint, 0x30420000000000000000000077359400u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_287, bid128_to_uint32_rnint, 0x30420000000000000000000077359401u128, 2147483648   , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_uint32_288, bid128_to_uint32_rnint, 0x30440000000000000000000000000003u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_289, bid128_to_uint32_rnint, 0x30520000000000000000000000000004u128, 4000000000   , 0x00); // -- 4e9
dec_test!(bid128_to_uint32_290, bid128_to_uint32_rnint, 0x30520000000000000000000000000005u128, 2147483648   , 0x01); // -- 5e9
dec_test!(bid128_to_uint32_291, bid128_to_uint32_rnint, 0x30540000000000000000000000000002u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_292, bid128_to_uint32_rnint, 0x38a80127243d5e393f8d670de6e9ff87u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_293, bid128_to_uint32_rnint, "4294967296"                          , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_294, bid128_to_uint32_rnint, 0x49f83dd5788fbf5876ebde8bb9a40e82u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_295, bid128_to_uint32_rnint, 0x53f20000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_296, bid128_to_uint32_rnint, 0x544ba795b9bb9926658c2582b222c055u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_297, bid128_to_uint32_rnint, "5.5"                                 , 6            , 0x00);
dec_test!(bid128_to_uint32_298, bid128_to_uint32_rnint, 0x57e91fc2692729d68cdb9420d7383914u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_299, bid128_to_uint32_rnint, 0x620a256801220ee0eeee5e779eefd79bu128, 0            , 0x00);
dec_test!(bid128_to_uint32_300, bid128_to_uint32_rnint, 0x78000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_301, bid128_to_uint32_rnint, 0x7ae75c2ec2fbf9f7fffffdbdfe7fff5fu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_302, bid128_to_uint32_rnint, 0x7bff7fffaff7fffcfffffeffffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_303, bid128_to_uint32_rnint, 0x7c000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_304, bid128_to_uint32_rnint, 0x7c003fffffffffff38c15b08ffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_305, bid128_to_uint32_rnint, 0x7c003fffffffffff38c15b0affffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_306, bid128_to_uint32_rnint, 0x7e000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_307, bid128_to_uint32_rnint, 0x923a2ad67a9ea6348010000001010401u128, 0            , 0x00);
dec_test!(bid128_to_uint32_308, bid128_to_uint32_rnint, 0xa06bb3c35abf22030f3897599156be4eu128, 0            , 0x00);
dec_test!(bid128_to_uint32_309, bid128_to_uint32_rnint, 0xabb60000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_310, bid128_to_uint32_rnint, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_311, bid128_to_uint32_rnint, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0            , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint32_312, bid128_to_uint32_rnint, 0xAFFCF684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_313, bid128_to_uint32_rnint, 0xaffddfbcffff9dfc0000412000000020u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_314, bid128_to_uint32_rnint, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_315, bid128_to_uint32_rnint, 0xAFFDEC8B86EF679D76FC433D80000000u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_316, bid128_to_uint32_rnint, 0xAFFDEC8B86EF679D76FC433D80000001u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_317, bid128_to_uint32_rnint, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_318, bid128_to_uint32_rnint, 0xAFFE314DC6448D9338C15B0A00000000u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_319, bid128_to_uint32_rnint, 0xAFFE314DC6448D9338C15B0A00000001u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_320, bid128_to_uint32_rnint, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_321, bid128_to_uint32_rnint, 0xAFFE49F4A966D45CD522088F00000000u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_322, bid128_to_uint32_rnint, 0xAFFE49F4A966D45CD522088F00000001u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_323, bid128_to_uint32_rnint, 0xb0000112018a008164c08d66a22003cau128, 0            , 0x00);
dec_test!(bid128_to_uint32_324, bid128_to_uint32_rnint, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_325, bid128_to_uint32_rnint, 0xB00293E952CDA8B9AA44111E00000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_326, bid128_to_uint32_rnint, 0xB00293E952CDA8B9AA44111E00000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_327, bid128_to_uint32_rnint, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_328, bid128_to_uint32_rnint, 0xB00294286EACB8CB0A8CB6B140000000u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_329, bid128_to_uint32_rnint, 0xB00294286EACB8CB0A8CB6B140000001u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_330, bid128_to_uint32_rnint, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_331, bid128_to_uint32_rnint, 0xB0040ECA8847C4129106CE8300000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_332, bid128_to_uint32_rnint, 0xB0040ECA8847C4129106CE8300000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_333, bid128_to_uint32_rnint, 0xb008c84384c1b86e60d63da585053b7fu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_334, bid128_to_uint32_rnint, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_335, bid128_to_uint32_rnint, 0xB00A0003C95A2F0B4856475FE0000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_336, bid128_to_uint32_rnint, 0xB00A0003C95A2F0B4856475FE0000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_337, bid128_to_uint32_rnint, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_338, bid128_to_uint32_rnint, 0xB00C000060EF6B1ABA6F072330000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_339, bid128_to_uint32_rnint, 0xB00C000060EF6B1ABA6F072330000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_340, bid128_to_uint32_rnint, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_341, bid128_to_uint32_rnint, 0xB01069E10DE628D3A6C9CC9B8E800000u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_342, bid128_to_uint32_rnint, 0xB01069E10DE628D3A6C9CC9B8E800001u128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_343, bid128_to_uint32_rnint, 0xB01069E10DE692B4B4B133125EFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_344, bid128_to_uint32_rnint, 0xB01069E10DE692B4B4B133125F000000u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_345, bid128_to_uint32_rnint, 0xB01069E10DE692B4B4B133125F000001u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_346, bid128_to_uint32_rnint, 0xB01069E10DE6FC95C29899892F7FFFFFu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_347, bid128_to_uint32_rnint, 0xB01069E10DE6FC95C29899892F800000u128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_348, bid128_to_uint32_rnint, 0xB01069E10DE6FC95C29899892F800001u128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_349, bid128_to_uint32_rnint, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_350, bid128_to_uint32_rnint, 0xB01069E10DE76676D080000000000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_351, bid128_to_uint32_rnint, 0xB01069E10DE76676D080000000000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_352, bid128_to_uint32_rnint, 0xB01069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_353, bid128_to_uint32_rnint, 0xB01069E10DE7D057DE676676D0800000u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_354, bid128_to_uint32_rnint, 0xB01069E10DE7D057DE676676D0800001u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_355, bid128_to_uint32_rnint, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_356, bid128_to_uint32_rnint, 0xB01069E10DE83A38EC4ECCEDA1000000u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_357, bid128_to_uint32_rnint, 0xB01069E10DE83A38EC4ECCEDA1000001u128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_358, bid128_to_uint32_rnint, 0xb010a58000004846be48dd32fd8eebb6u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_359, bid128_to_uint32_rnint, 0xB010C5371912364CE3056C27FFFFFFFFu128, 2147483648   , 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_uint32_360, bid128_to_uint32_rnint, 0xB010C5371912364CE3056C2800000000u128, 2147483648   , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint32_361, bid128_to_uint32_rnint, 0xB010C5371912364CE3056C2800000001u128, 2147483648   , 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_uint32_362, bid128_to_uint32_rnint, 0xB010D3C21BCDF92B853133125EFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_363, bid128_to_uint32_rnint, 0xB010D3C21BCDF92B853133125F000000u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_364, bid128_to_uint32_rnint, 0xB010D3C21BCDF92B853133125F000001u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_365, bid128_to_uint32_rnint, 0xB010D3C21BCE630C931899892F7FFFFFu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_366, bid128_to_uint32_rnint, 0xB010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_367, bid128_to_uint32_rnint, 0xB010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_368, bid128_to_uint32_rnint, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_369, bid128_to_uint32_rnint, 0xB010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_370, bid128_to_uint32_rnint, 0xB010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_371, bid128_to_uint32_rnint, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_372, bid128_to_uint32_rnint, 0xB010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_373, bid128_to_uint32_rnint, 0xB010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_374, bid128_to_uint32_rnint, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_375, bid128_to_uint32_rnint, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_376, bid128_to_uint32_rnint, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_377, bid128_to_uint32_rnint, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_uint32_378, bid128_to_uint32_rnint, 0xB010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint32_379, bid128_to_uint32_rnint, 0xB010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_uint32_380, bid128_to_uint32_rnint, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_381, bid128_to_uint32_rnint, 0xB012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_382, bid128_to_uint32_rnint, 0xB012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_383, bid128_to_uint32_rnint, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_384, bid128_to_uint32_rnint, 0xB012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_385, bid128_to_uint32_rnint, 0xB012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_386, bid128_to_uint32_rnint, 0xB012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_387, bid128_to_uint32_rnint, 0xB012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_388, bid128_to_uint32_rnint, 0xB012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_389, bid128_to_uint32_rnint, 0xB012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_390, bid128_to_uint32_rnint, 0xB012629B8C891B267182B61400000000u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_391, bid128_to_uint32_rnint, 0xB012629B8C891B267182B61400000001u128, 2147483648   , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_392, bid128_to_uint32_rnint, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_393, bid128_to_uint32_rnint, 0xB012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_394, bid128_to_uint32_rnint, 0xB012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_395, bid128_to_uint32_rnint, 0xB012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_396, bid128_to_uint32_rnint, 0xB012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_397, bid128_to_uint32_rnint, 0xB012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_398, bid128_to_uint32_rnint, 0xB012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_399, bid128_to_uint32_rnint, 0xB012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_400, bid128_to_uint32_rnint, 0xB012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_401, bid128_to_uint32_rnint, 0xB01600000000003627E8F712373BFFFFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_402, bid128_to_uint32_rnint, 0xB01600000000003627E8F712373C0000u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_403, bid128_to_uint32_rnint, 0xB01600000000003627E8F712373C0001u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_404, bid128_to_uint32_rnint, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_405, bid128_to_uint32_rnint, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_406, bid128_to_uint32_rnint, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_407, bid128_to_uint32_rnint, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_408, bid128_to_uint32_rnint, 0xB0180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_409, bid128_to_uint32_rnint, 0xB0180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_410, bid128_to_uint32_rnint, 0xB0180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_411, bid128_to_uint32_rnint, 0xB0180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_412, bid128_to_uint32_rnint, 0xB0180002B5E3AF19676BAF16B1880001u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_413, bid128_to_uint32_rnint, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_414, bid128_to_uint32_rnint, 0xB01800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_415, bid128_to_uint32_rnint, 0xB01800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_416, bid128_to_uint32_rnint, 0xB01800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_417, bid128_to_uint32_rnint, 0xB01800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_418, bid128_to_uint32_rnint, 0xB01800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_419, bid128_to_uint32_rnint, 0xb018000c00000000deffffbfddfdfbffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_420, bid128_to_uint32_rnint, 0xB01A0000000000004563918244F3FFFFu128, 0            , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_421, bid128_to_uint32_rnint, 0xB01A0000000000004563918244F40000u128, 0            , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint32_422, bid128_to_uint32_rnint, 0xB01A0000000000004563918244F40001u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_423, bid128_to_uint32_rnint, 0xB01A0000000000008AC7230489E7FFFFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_424, bid128_to_uint32_rnint, 0xB01A0000000000008AC7230489E80000u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_425, bid128_to_uint32_rnint, 0xB01A0000000000008AC7230489E80001u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_426, bid128_to_uint32_rnint, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_427, bid128_to_uint32_rnint, 0xB01A0000000000A2E6C09AD3E0D40000u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_428, bid128_to_uint32_rnint, 0xB01A0000000000A2E6C09AD3E0D40001u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_429, bid128_to_uint32_rnint, 0xB01A000045639181BA2CDCFB7617FFFFu128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_430, bid128_to_uint32_rnint, 0xB01A000045639181BA2CDCFB76180000u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_431, bid128_to_uint32_rnint, 0xB01A000045639181BA2CDCFB76180001u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_432, bid128_to_uint32_rnint, 0xB01A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_433, bid128_to_uint32_rnint, 0xB01A00004563918244F4000000000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_434, bid128_to_uint32_rnint, 0xB01A00004563918244F4000000000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_435, bid128_to_uint32_rnint, 0xB01A000045639182CFBB230489E7FFFFu128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_436, bid128_to_uint32_rnint, 0xB01A000045639182CFBB230489E80000u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_437, bid128_to_uint32_rnint, 0xB01A000045639182CFBB230489E80001u128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_438, bid128_to_uint32_rnint, 0xB01A00008AC72303FF20DCFB7617FFFFu128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_439, bid128_to_uint32_rnint, 0xB01A00008AC72303FF20DCFB76180000u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_440, bid128_to_uint32_rnint, 0xB01A00008AC72303FF20DCFB76180001u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_441, bid128_to_uint32_rnint, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_442, bid128_to_uint32_rnint, 0xB01A00008AC7230489E8000000000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_443, bid128_to_uint32_rnint, 0xB01A00008AC7230489E8000000000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_444, bid128_to_uint32_rnint, 0xB01A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_445, bid128_to_uint32_rnint, 0xB01A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_446, bid128_to_uint32_rnint, 0xB01A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_447, bid128_to_uint32_rnint, 0xB01C00000000000014D1120D7B15FFFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_448, bid128_to_uint32_rnint, 0xB01C00000000000014D1120D7B160000u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_449, bid128_to_uint32_rnint, 0xB01C00000000000014D1120D7B160001u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_450, bid128_to_uint32_rnint, 0xB01E000000000001A055690D9DB7FFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_451, bid128_to_uint32_rnint, 0xB01E000000000001A055690D9DB80000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_452, bid128_to_uint32_rnint, 0xB01E000000000001A055690D9DB80001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_453, bid128_to_uint32_rnint, 0xB02000000000000029A2241AF62BFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_454, bid128_to_uint32_rnint, 0xB02000000000000029A2241AF62C0000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_455, bid128_to_uint32_rnint, 0xB02000000000000029A2241AF62C0001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_456, bid128_to_uint32_rnint, 0xB024000000000000006A94D74F42FFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_457, bid128_to_uint32_rnint, 0xB024000000000000006A94D74F430000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_458, bid128_to_uint32_rnint, 0xB024000000000000006A94D74F430001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_459, bid128_to_uint32_rnint, 0xB02A00000000000000000017428106FFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_460, bid128_to_uint32_rnint, 0xB02A0000000000000000001742810700u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_461, bid128_to_uint32_rnint, 0xB02A0000000000000000001742810701u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_462, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_463, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_464, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_465, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_466, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_467, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_468, bid128_to_uint32_rnint, 0xB02C000000000000000002BBA7F521FFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_469, bid128_to_uint32_rnint, 0xB02C000000000000000002BBA7F52200u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_470, bid128_to_uint32_rnint, 0xB02C000000000000000002BBA7F52201u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_471, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_472, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_473, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_474, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_475, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_476, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_477, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_478, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_479, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_480, bid128_to_uint32_rnint, 0xB02E000000000000000000001DCD64FFu128, 0            , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_481, bid128_to_uint32_rnint, 0xB02E000000000000000000001DCD6500u128, 0            , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint32_482, bid128_to_uint32_rnint, 0xB02E000000000000000000001DCD6501u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_483, bid128_to_uint32_rnint, 0xB02E000000000000000000003B9AC9FFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_484, bid128_to_uint32_rnint, 0xB02E000000000000000000003B9ACA00u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_485, bid128_to_uint32_rnint, 0xB02E000000000000000000003B9ACA01u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_486, bid128_to_uint32_rnint, 0xB02E0000000000000000000059682EFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_487, bid128_to_uint32_rnint, 0xB02E0000000000000000000059682F00u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_488, bid128_to_uint32_rnint, 0xB02E0000000000000000000059682F01u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_489, bid128_to_uint32_rnint, 0xB02E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_490, bid128_to_uint32_rnint, 0xB02E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_491, bid128_to_uint32_rnint, 0xB02E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_492, bid128_to_uint32_rnint, 0xB03000000000000000000006FC23ABFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_493, bid128_to_uint32_rnint, 0xB03000000000000000000006FC23AC00u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_494, bid128_to_uint32_rnint, 0xB03000000000000000000006FC23AC01u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_495, bid128_to_uint32_rnint, 0xB03200000000000000000000B2D05DFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_496, bid128_to_uint32_rnint, 0xB03200000000000000000000B2D05E00u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_497, bid128_to_uint32_rnint, 0xB03200000000000000000000B2D05E01u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_498, bid128_to_uint32_rnint, 0xB03800000000000000000000002DDA47u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_499, bid128_to_uint32_rnint, 0xB03800000000000000000000002DDA48u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_500, bid128_to_uint32_rnint, 0xB03800000000000000000000002DDA49u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_501, bid128_to_uint32_rnint, 0xB03A00000000000000000000000003E7u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_502, bid128_to_uint32_rnint, 0xB03A00000000000000000000000005DBu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_503, bid128_to_uint32_rnint, 0xB03A00000000000000000000000005DCu128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_504, bid128_to_uint32_rnint, 0xB03A00000000000000000000000005DDu128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_505, bid128_to_uint32_rnint, 0xB03A00000000000000000000000495D3u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_506, bid128_to_uint32_rnint, 0xB03A00000000000000000000000495D4u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_507, bid128_to_uint32_rnint, 0xB03A00000000000000000000000495D5u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_508, bid128_to_uint32_rnint, 0xB03C0000000000000000000000000095u128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_509, bid128_to_uint32_rnint, 0xB03C0000000000000000000000000096u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_510, bid128_to_uint32_rnint, 0xB03C0000000000000000000000000097u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_511, bid128_to_uint32_rnint, 0xB03C0000000000000000000000007561u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_512, bid128_to_uint32_rnint, 0xB03C0000000000000000000000007562u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_513, bid128_to_uint32_rnint, 0xB03C0000000000000000000000007563u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_514, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFF69u128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_515, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFF6Au128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_516, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFF6Bu128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_517, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFFCDu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_518, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFFCEu128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_519, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFFCFu128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_520, bid128_to_uint32_rnint, 0xB03C0000000000000000003200000031u128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_521, bid128_to_uint32_rnint, 0xB03C0000000000000000003200000032u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_522, bid128_to_uint32_rnint, 0xB03C0000000000000000003200000033u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_523, bid128_to_uint32_rnint, 0xB03C00000000000000000063FFFFFFCDu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_524, bid128_to_uint32_rnint, 0xB03C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_525, bid128_to_uint32_rnint, 0xB03C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_526, bid128_to_uint32_rnint, 0xB03C0000000000000000006400000031u128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_527, bid128_to_uint32_rnint, 0xB03C0000000000000000006400000032u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_528, bid128_to_uint32_rnint, 0xB03C0000000000000000006400000033u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_529, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000005u128, 0            , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint32_530, bid128_to_uint32_rnint, 0xB03E000000000000000000000000000Fu128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_531, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000BB7u128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_532, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000BB8u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_533, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000BB9u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_534, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000BBDu128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_535, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFF1u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_536, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFF5u128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_537, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFF6u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_538, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFF7u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_539, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFFBu128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_540, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_541, bid128_to_uint32_rnint, 0xB03E0000000000000000000500000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_542, bid128_to_uint32_rnint, 0xB03E0000000000000000000500000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_543, bid128_to_uint32_rnint, 0xB03E0000000000000000000500000005u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_544, bid128_to_uint32_rnint, 0xB03E0000000000000000000500000009u128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_545, bid128_to_uint32_rnint, 0xB03E000000000000000000050000000Au128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_546, bid128_to_uint32_rnint, 0xB03E000000000000000000050000000Bu128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_547, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFF5u128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_548, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFF6u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_549, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFF7u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_550, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_551, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_552, bid128_to_uint32_rnint, 0xB03E0000000000000000000A00000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_553, bid128_to_uint32_rnint, 0xB03E0000000000000000000A00000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_554, bid128_to_uint32_rnint, 0xB03E0000000000000000000A00000005u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_555, bid128_to_uint32_rnint, 0xB03E0000000000000000000A00000009u128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_556, bid128_to_uint32_rnint, 0xB03E0000000000000000000A0000000Au128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_557, bid128_to_uint32_rnint, 0xB03E0000000000000000000A0000000Bu128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_558, bid128_to_uint32_rnint, 0xB03E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_559, bid128_to_uint32_rnint, 0xB03E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_560, bid128_to_uint32_rnint, 0xB03E0000000000000000002E90EDD005u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_561, bid128_to_uint32_rnint, 0xB03E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_562, bid128_to_uint32_rnint, 0xB0400000000000000000000000000001u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_563, bid128_to_uint32_rnint, 0xB040000000000000000000000000012Bu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_564, bid128_to_uint32_rnint, 0xB040000000000000000000000000012Cu128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_565, bid128_to_uint32_rnint, 0xB040000000000000000000000000012Du128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_566, bid128_to_uint32_rnint, 0xB040000000000000000000007FFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_567, bid128_to_uint32_rnint, 0xB0400000000000000000000080000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_568, bid128_to_uint32_rnint, 0xB0400000000000000000000080000001u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_569, bid128_to_uint32_rnint, 0xB04000000000000000000000FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_570, bid128_to_uint32_rnint, 0xB0400000000000000000000100000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_571, bid128_to_uint32_rnint, 0xB0400000000000000000000100000001u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_572, bid128_to_uint32_rnint, 0xB04000000000000000000004A817C7FFu128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_573, bid128_to_uint32_rnint, 0xB04000000000000000000004A817C801u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_574, bid128_to_uint32_rnint, 0xB042000000000000000000000000001Du128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_575, bid128_to_uint32_rnint, 0xB042000000000000000000000000001Eu128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_576, bid128_to_uint32_rnint, 0xB042000000000000000000000000001Fu128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_577, bid128_to_uint32_rnint, 0xB04200000000000000000000773593FFu128, 2147483648   , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_578, bid128_to_uint32_rnint, 0xB0420000000000000000000077359400u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_579, bid128_to_uint32_rnint, 0xB0420000000000000000000077359401u128, 2147483648   , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_580, bid128_to_uint32_rnint, 0xB0440000000000000000000000000003u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_581, bid128_to_uint32_rnint, 0xB0520000000000000000000000000004u128, 2147483648   , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint32_582, bid128_to_uint32_rnint, 0xB0520000000000000000000000000005u128, 2147483648   , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint32_583, bid128_to_uint32_rnint, 0xB0540000000000000000000000000002u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_584, bid128_to_uint32_rnint, 0xb6c25dfe6fec677de578aa8aa40f4b1du128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_585, bid128_to_uint32_rnint, 0xc0240000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_586, bid128_to_uint32_rnint, 0xd8749bc1592c005392cb8ac564abffa5u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_587, bid128_to_uint32_rnint, 0xe453ad749a5880dbff893a32625017f1u128, 0            , 0x00);
dec_test!(bid128_to_uint32_588, bid128_to_uint32_rnint, 0xfa2152399330f7fd42afeb0123ae5dc1u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_589, bid128_to_uint32_rnint, 0xfbfbef1e1fbffe2f0000422200400210u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_590, bid128_to_uint32_rnint, 0xfddfffffffffffff3bfebbef5c91ea5bu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_591, bid128_to_uint32_rnint, 0xfdfddffd7f6ffefffd48f47fb8fbeaefu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_592, bid128_to_uint32_rnint, 0xfeffefbfffffeff72a97bf5e030d67c8u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_593, bid128_to_uint32_rnint, 0xfffbffffffffffffffffffffffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_594, bid128_to_uint32_rnint, "-Infinity"                           , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_595, bid128_to_uint32_rnint, "Infinity"                            , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_596, bid128_to_uint32_rnint, "QNaN"                                , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_597, bid128_to_uint32_rnint, "SNaN"                                , 0x80000000u32, 0x01);
