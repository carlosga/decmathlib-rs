/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_uint32_rnint_001, bid128_to_uint32_rnint, "-0"                                  , 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_002, bid128_to_uint32_rnint, 0x00000000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_003, bid128_to_uint32_rnint, 0x00000000000000000000000800001200u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_004, bid128_to_uint32_rnint, 0x0000000000000000a138746e31c01009u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_005, bid128_to_uint32_rnint, 0x0001ed09bead87c0378d8e62ffffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_006, bid128_to_uint32_rnint, 0x0001ed09bead87c0378d8e64ffffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_007, bid128_to_uint32_rnint, 0x01200000000000007af626f7bf37ce9bu128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_008, bid128_to_uint32_rnint, 0x081fdfac17793ad8ffffffff7fffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_009, bid128_to_uint32_rnint, 0x0f998fb207514890eb3cb78b20dd7e7fu128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_010, bid128_to_uint32_rnint, 1073741824                            , 1073741824   , 0x00);
dec_test!(bid128_to_uint32_rnint_011, bid128_to_uint32_rnint, 1                                     , 1            , 0x00);
dec_test!(bid128_to_uint32_rnint_012, bid128_to_uint32_rnint, 0x173a3b47d6f18423fffff73ff7aeefffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_013, bid128_to_uint32_rnint, 0x1c460000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_014, bid128_to_uint32_rnint, 0x1ccc212c0128566ddafd7e52499bf035u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_015, bid128_to_uint32_rnint, 0x1ef00000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_016, bid128_to_uint32_rnint, 0x26bcf5e801c36421f1725e5a5b636742u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_017, bid128_to_uint32_rnint, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_uint32_rnint_018, bid128_to_uint32_rnint, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0            , 0x00); // -- 0.5
dec_test!(bid128_to_uint32_rnint_019, bid128_to_uint32_rnint, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1            , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_uint32_rnint_020, bid128_to_uint32_rnint, 0x2ffdb4e937c8d3990000040000000000u128, 1            , 0x00);
dec_test!(bid128_to_uint32_rnint_021, bid128_to_uint32_rnint, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1            , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_uint32_rnint_022, bid128_to_uint32_rnint, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1            , 0x00); // -- 0.999
dec_test!(bid128_to_uint32_rnint_023, bid128_to_uint32_rnint, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1            , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_uint32_rnint_024, bid128_to_uint32_rnint, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1            , 0x00); // -- 1-ulp
dec_test!(bid128_to_uint32_rnint_025, bid128_to_uint32_rnint, 0x2FFE314DC6448D9338C15B0A00000000u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_rnint_026, bid128_to_uint32_rnint, 0x2FFE314DC6448D9338C15B0A00000001u128, 1            , 0x00); // -- 1+ulp
dec_test!(bid128_to_uint32_rnint_027, bid128_to_uint32_rnint, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_rnint_028, bid128_to_uint32_rnint, 0x2FFE49F4A966D45CD522088F00000000u128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_rnint_029, bid128_to_uint32_rnint, 0x2FFE49F4A966D45CD522088F00000001u128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_rnint_030, bid128_to_uint32_rnint, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_031, bid128_to_uint32_rnint, 0x300293E952CDA8B9AA44111E00000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_032, bid128_to_uint32_rnint, 0x300293E952CDA8B9AA44111E00000001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_033, bid128_to_uint32_rnint, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_rnint_034, bid128_to_uint32_rnint, 0x300294286EACB8CB0A8CB6B140000000u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_rnint_035, bid128_to_uint32_rnint, 0x300294286EACB8CB0A8CB6B140000001u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_rnint_036, bid128_to_uint32_rnint, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_037, bid128_to_uint32_rnint, 0x30040ECA8847C4129106CE8300000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_038, bid128_to_uint32_rnint, 0x30040ECA8847C4129106CE8300000001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_039, bid128_to_uint32_rnint, 0x3006082200890122dc2fb197063dd1b5u128, 1650         , 0x00);
dec_test!(bid128_to_uint32_rnint_040, bid128_to_uint32_rnint, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_041, bid128_to_uint32_rnint, 0x300A0003C95A2F0B4856475FE0000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_042, bid128_to_uint32_rnint, 0x300A0003C95A2F0B4856475FE0000001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_043, bid128_to_uint32_rnint, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_044, bid128_to_uint32_rnint, 0x300C000060EF6B1ABA6F072330000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_045, bid128_to_uint32_rnint, 0x300C000060EF6B1ABA6F072330000001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_046, bid128_to_uint32_rnint, 0x3010421006b8c60253380e68050fefa2u128, 1339908765   , 0x00);
dec_test!(bid128_to_uint32_rnint_047, bid128_to_uint32_rnint, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646   , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_rnint_048, bid128_to_uint32_rnint, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483646   , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_uint32_rnint_049, bid128_to_uint32_rnint, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483647   , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_rnint_050, bid128_to_uint32_rnint, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483647   , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_rnint_051, bid128_to_uint32_rnint, 0x301069E10DE692B4B4B133125F000000u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_rnint_052, bid128_to_uint32_rnint, 0x301069E10DE692B4B4B133125F000001u128, 2147483647   , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_rnint_053, bid128_to_uint32_rnint, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647   , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_rnint_054, bid128_to_uint32_rnint, 0x301069E10DE6FC95C29899892F800000u128, 2147483648   , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_uint32_rnint_055, bid128_to_uint32_rnint, 0x301069E10DE6FC95C29899892F800001u128, 2147483648   , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_rnint_056, bid128_to_uint32_rnint, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_uint32_rnint_057, bid128_to_uint32_rnint, 0x301069E10DE76676D080000000000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_rnint_058, bid128_to_uint32_rnint, 0x301069E10DE76676D080000000000001u128, 2147483648   , 0x00); // -- 2^31+ulp
dec_test!(bid128_to_uint32_rnint_059, bid128_to_uint32_rnint, 0x301069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x00); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_rnint_060, bid128_to_uint32_rnint, 0x301069E10DE7D057DE676676D0800000u128, 2147483648   , 0x00); // -- 2^31+0.5
dec_test!(bid128_to_uint32_rnint_061, bid128_to_uint32_rnint, 0x301069E10DE7D057DE676676D0800001u128, 2147483649   , 0x00); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_rnint_062, bid128_to_uint32_rnint, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483649   , 0x00); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_rnint_063, bid128_to_uint32_rnint, 0x301069E10DE83A38EC4ECCEDA1000000u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_rnint_064, bid128_to_uint32_rnint, 0x301069E10DE83A38EC4ECCEDA1000001u128, 2147483649   , 0x00); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_rnint_065, bid128_to_uint32_rnint, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000   , 0x00); // -- 4e9-ulp
dec_test!(bid128_to_uint32_rnint_066, bid128_to_uint32_rnint, 0x3010C5371912364CE3056C2800000000u128, 4000000000   , 0x00); // -- 4e9
dec_test!(bid128_to_uint32_rnint_067, bid128_to_uint32_rnint, 0x3010C5371912364CE3056C2800000001u128, 4000000000   , 0x00); // -- 4e9+ulp
dec_test!(bid128_to_uint32_rnint_068, bid128_to_uint32_rnint, 0x3010D3C21BCDF92B853133125EFFFFFFu128, 4294967295   , 0x00); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_rnint_069, bid128_to_uint32_rnint, 0x3010D3C21BCDF92B853133125F000000u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_rnint_070, bid128_to_uint32_rnint, 0x3010D3C21BCDF92B853133125F000001u128, 4294967295   , 0x00); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_rnint_071, bid128_to_uint32_rnint, 0x3010D3C21BCE630C931899892F7FFFFFu128, 4294967295   , 0x00); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_rnint_072, bid128_to_uint32_rnint, 0x3010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_rnint_073, bid128_to_uint32_rnint, 0x3010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_rnint_074, bid128_to_uint32_rnint, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_uint32_rnint_075, bid128_to_uint32_rnint, 0x3010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_rnint_076, bid128_to_uint32_rnint, 0x3010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_rnint_077, bid128_to_uint32_rnint, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_rnint_078, bid128_to_uint32_rnint, 0x3010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_rnint_079, bid128_to_uint32_rnint, 0x3010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_rnint_080, bid128_to_uint32_rnint, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_rnint_081, bid128_to_uint32_rnint, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_rnint_082, bid128_to_uint32_rnint, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_rnint_083, bid128_to_uint32_rnint, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); // -- 5e9-ulp
dec_test!(bid128_to_uint32_rnint_084, bid128_to_uint32_rnint, 0x3010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- 5e9
dec_test!(bid128_to_uint32_rnint_085, bid128_to_uint32_rnint, 0x3010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- 5e9+ulp
dec_test!(bid128_to_uint32_rnint_086, bid128_to_uint32_rnint, 0x3011028102430001ffffffffffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_087, bid128_to_uint32_rnint, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_rnint_088, bid128_to_uint32_rnint, 0x3012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_rnint_089, bid128_to_uint32_rnint, 0x3012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_rnint_090, bid128_to_uint32_rnint, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_rnint_091, bid128_to_uint32_rnint, 0x3012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_rnint_092, bid128_to_uint32_rnint, 0x3012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_rnint_093, bid128_to_uint32_rnint, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_rnint_094, bid128_to_uint32_rnint, 0x3012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_rnint_095, bid128_to_uint32_rnint, 0x3012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_rnint_096, bid128_to_uint32_rnint, 0x3012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_uint32_rnint_097, bid128_to_uint32_rnint, 0x3012629B8C891B267182B61400000000u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_rnint_098, bid128_to_uint32_rnint, 0x3012629B8C891B267182B61400000001u128, 2147483648   , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_uint32_rnint_099, bid128_to_uint32_rnint, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_rnint_100, bid128_to_uint32_rnint, 0x3012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_rnint_101, bid128_to_uint32_rnint, 0x3012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_rnint_102, bid128_to_uint32_rnint, 0x3012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_rnint_103, bid128_to_uint32_rnint, 0x3012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_rnint_104, bid128_to_uint32_rnint, 0x3012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_rnint_105, bid128_to_uint32_rnint, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_rnint_106, bid128_to_uint32_rnint, 0x3012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_rnint_107, bid128_to_uint32_rnint, 0x3012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_rnint_108, bid128_to_uint32_rnint, 0x301600000000003627E8F712373BFFFFu128, 1            , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_uint32_rnint_109, bid128_to_uint32_rnint, 0x301600000000003627E8F712373C0000u128, 1            , 0x00); // -- 0.999
dec_test!(bid128_to_uint32_rnint_110, bid128_to_uint32_rnint, 0x301600000000003627E8F712373C0001u128, 1            , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_uint32_rnint_111, bid128_to_uint32_rnint, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646   , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_rnint_112, bid128_to_uint32_rnint, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483646   , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_uint32_rnint_113, bid128_to_uint32_rnint, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483647   , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_rnint_114, bid128_to_uint32_rnint, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647   , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_rnint_115, bid128_to_uint32_rnint, 0x30180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_uint32_rnint_116, bid128_to_uint32_rnint, 0x30180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_rnint_117, bid128_to_uint32_rnint, 0x30180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x00); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_rnint_118, bid128_to_uint32_rnint, 0x30180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x00); // -- 2^31+0.5
dec_test!(bid128_to_uint32_rnint_119, bid128_to_uint32_rnint, 0x30180002B5E3AF19676BAF16B1880001u128, 2147483649   , 0x00); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_rnint_120, bid128_to_uint32_rnint, 0x301800056BC75E2AAD2C50E94E77FFFFu128, 4294967295   , 0x00); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_rnint_121, bid128_to_uint32_rnint, 0x301800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_rnint_122, bid128_to_uint32_rnint, 0x301800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_rnint_123, bid128_to_uint32_rnint, 0x301800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_rnint_124, bid128_to_uint32_rnint, 0x301800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_rnint_125, bid128_to_uint32_rnint, 0x301800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_rnint_126, bid128_to_uint32_rnint, 0x301A0000000000004563918244F3FFFFu128, 0            , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_uint32_rnint_127, bid128_to_uint32_rnint, 0x301A0000000000004563918244F40000u128, 0            , 0x00); // -- 0.5
dec_test!(bid128_to_uint32_rnint_128, bid128_to_uint32_rnint, 0x301A0000000000004563918244F40001u128, 1            , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_uint32_rnint_129, bid128_to_uint32_rnint, 0x301A0000000000008AC7230489E7FFFFu128, 1            , 0x00); // -- 1-ulp
dec_test!(bid128_to_uint32_rnint_130, bid128_to_uint32_rnint, 0x301A0000000000008AC7230489E80000u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_rnint_131, bid128_to_uint32_rnint, 0x301A0000000000008AC7230489E80001u128, 1            , 0x00); // -- 1+ulp
dec_test!(bid128_to_uint32_rnint_132, bid128_to_uint32_rnint, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_rnint_133, bid128_to_uint32_rnint, 0x301A0000000000A2E6C09AD3E0D40000u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_rnint_134, bid128_to_uint32_rnint, 0x301A0000000000A2E6C09AD3E0D40001u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_rnint_135, bid128_to_uint32_rnint, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483647   , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_rnint_136, bid128_to_uint32_rnint, 0x301A000045639181BA2CDCFB76180000u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_rnint_137, bid128_to_uint32_rnint, 0x301A000045639181BA2CDCFB76180001u128, 2147483647   , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_rnint_138, bid128_to_uint32_rnint, 0x301A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_uint32_rnint_139, bid128_to_uint32_rnint, 0x301A00004563918244F4000000000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_rnint_140, bid128_to_uint32_rnint, 0x301A00004563918244F4000000000001u128, 2147483648   , 0x00); // -- 2^31+ulp
dec_test!(bid128_to_uint32_rnint_141, bid128_to_uint32_rnint, 0x301A000045639182CFBB230489E7FFFFu128, 2147483649   , 0x00); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_rnint_142, bid128_to_uint32_rnint, 0x301A000045639182CFBB230489E80000u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_rnint_143, bid128_to_uint32_rnint, 0x301A000045639182CFBB230489E80001u128, 2147483649   , 0x00); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_rnint_144, bid128_to_uint32_rnint, 0x301A00008AC72303FF20DCFB7617FFFFu128, 4294967295   , 0x00); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_rnint_145, bid128_to_uint32_rnint, 0x301A00008AC72303FF20DCFB76180000u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_rnint_146, bid128_to_uint32_rnint, 0x301A00008AC72303FF20DCFB76180001u128, 4294967295   , 0x00); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_rnint_147, bid128_to_uint32_rnint, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_uint32_rnint_148, bid128_to_uint32_rnint, 0x301A00008AC7230489E8000000000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_rnint_149, bid128_to_uint32_rnint, 0x301A00008AC7230489E8000000000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_rnint_150, bid128_to_uint32_rnint, 0x301A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_rnint_151, bid128_to_uint32_rnint, 0x301A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_rnint_152, bid128_to_uint32_rnint, 0x301A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_rnint_153, bid128_to_uint32_rnint, 0x301C00000000000014D1120D7B15FFFFu128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_rnint_154, bid128_to_uint32_rnint, 0x301C00000000000014D1120D7B160000u128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_rnint_155, bid128_to_uint32_rnint, 0x301C00000000000014D1120D7B160001u128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_rnint_156, bid128_to_uint32_rnint, 0x301E000000000001A055690D9DB7FFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_157, bid128_to_uint32_rnint, 0x301E000000000001A055690D9DB80000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_158, bid128_to_uint32_rnint, 0x301E000000000001A055690D9DB80001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_159, bid128_to_uint32_rnint, 0x302000000000000029A2241AF62BFFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_160, bid128_to_uint32_rnint, 0x302000000000000029A2241AF62C0000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_161, bid128_to_uint32_rnint, 0x302000000000000029A2241AF62C0001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_162, bid128_to_uint32_rnint, 0x302000000021400028004460c2808304u128, 0xEF976DAEu32, 0x00);
dec_test!(bid128_to_uint32_rnint_163, bid128_to_uint32_rnint, 0x3024000000000000006A94D74F42FFFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_164, bid128_to_uint32_rnint, 0x3024000000000000006A94D74F430000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_165, bid128_to_uint32_rnint, 0x3024000000000000006A94D74F430001u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_166, bid128_to_uint32_rnint, 0x302A00000000000000000017428106FFu128, 1            , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_uint32_rnint_167, bid128_to_uint32_rnint, 0x302A0000000000000000001742810700u128, 1            , 0x00); // -- 0.999
dec_test!(bid128_to_uint32_rnint_168, bid128_to_uint32_rnint, 0x302A0000000000000000001742810701u128, 1            , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_uint32_rnint_169, bid128_to_uint32_rnint, 0x302A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_rnint_170, bid128_to_uint32_rnint, 0x302A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_rnint_171, bid128_to_uint32_rnint, 0x302A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_rnint_172, bid128_to_uint32_rnint, 0x302A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_rnint_173, bid128_to_uint32_rnint, 0x302A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_rnint_174, bid128_to_uint32_rnint, 0x302A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_rnint_175, bid128_to_uint32_rnint, 0x302C000000000000000002BBA7F521FFu128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_rnint_176, bid128_to_uint32_rnint, 0x302C000000000000000002BBA7F52200u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_rnint_177, bid128_to_uint32_rnint, 0x302C000000000000000002BBA7F52201u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_rnint_178, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_rnint_179, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_rnint_180, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_rnint_181, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_rnint_182, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_rnint_183, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_rnint_184, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_rnint_185, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_rnint_186, bid128_to_uint32_rnint, 0x302C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_rnint_187, bid128_to_uint32_rnint, 0x302E000000000000000000001DCD64FFu128, 0            , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_uint32_rnint_188, bid128_to_uint32_rnint, 0x302E000000000000000000001DCD6500u128, 0            , 0x00); // -- 0.5
dec_test!(bid128_to_uint32_rnint_189, bid128_to_uint32_rnint, 0x302E000000000000000000001DCD6501u128, 1            , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_uint32_rnint_190, bid128_to_uint32_rnint, 0x302E000000000000000000003B9AC9FFu128, 1            , 0x00); // -- 1-ulp
dec_test!(bid128_to_uint32_rnint_191, bid128_to_uint32_rnint, 0x302E000000000000000000003B9ACA00u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_rnint_192, bid128_to_uint32_rnint, 0x302E000000000000000000003B9ACA01u128, 1            , 0x00); // -- 1+ulp
dec_test!(bid128_to_uint32_rnint_193, bid128_to_uint32_rnint, 0x302E0000000000000000000059682EFFu128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_rnint_194, bid128_to_uint32_rnint, 0x302E0000000000000000000059682F00u128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_rnint_195, bid128_to_uint32_rnint, 0x302E0000000000000000000059682F01u128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_rnint_196, bid128_to_uint32_rnint, 0x302E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_rnint_197, bid128_to_uint32_rnint, 0x302E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_rnint_198, bid128_to_uint32_rnint, 0x302E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_rnint_199, bid128_to_uint32_rnint, 0x303000000000000000000006FC23ABFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_200, bid128_to_uint32_rnint, 0x303000000000000000000006FC23AC00u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_201, bid128_to_uint32_rnint, 0x303000000000000000000006FC23AC01u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_202, bid128_to_uint32_rnint, 0x303200000000000000000000B2D05DFFu128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_203, bid128_to_uint32_rnint, 0x303200000000000000000000B2D05E00u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_204, bid128_to_uint32_rnint, 0x303200000000000000000000B2D05E01u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_205, bid128_to_uint32_rnint, 0x303800000000000000000000002DDA47u128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_rnint_206, bid128_to_uint32_rnint, 0x303800000000000000000000002DDA48u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_rnint_207, bid128_to_uint32_rnint, 0x303800000000000000000000002DDA49u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_rnint_208, bid128_to_uint32_rnint, 0x303A00000000000000000000000003E7u128, 1            , 0x00); // -- 0.999
dec_test!(bid128_to_uint32_rnint_209, bid128_to_uint32_rnint, 0x303A00000000000000000000000005DBu128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_rnint_210, bid128_to_uint32_rnint, 0x303A00000000000000000000000005DCu128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_rnint_211, bid128_to_uint32_rnint, 0x303A00000000000000000000000005DDu128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_rnint_212, bid128_to_uint32_rnint, 0x303A00000000000000000000000495D3u128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_rnint_213, bid128_to_uint32_rnint, 0x303A00000000000000000000000495D4u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_rnint_214, bid128_to_uint32_rnint, 0x303A00000000000000000000000495D5u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_rnint_215, bid128_to_uint32_rnint, 0x303C0000000000000000000000000095u128, 1            , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint32_rnint_216, bid128_to_uint32_rnint, 0x303C0000000000000000000000000096u128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_rnint_217, bid128_to_uint32_rnint, 0x303C0000000000000000000000000097u128, 2            , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint32_rnint_218, bid128_to_uint32_rnint, 0x303C0000000000000000000000007561u128, 300          , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint32_rnint_219, bid128_to_uint32_rnint, 0x303C0000000000000000000000007562u128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_rnint_220, bid128_to_uint32_rnint, 0x303C0000000000000000000000007563u128, 301          , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint32_rnint_221, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFF69u128, 2147483646   , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_rnint_222, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFF6Au128, 2147483646   , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_uint32_rnint_223, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFF6Bu128, 2147483647   , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_rnint_224, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFFCDu128, 2147483647   , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_rnint_225, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFFCEu128, 2147483648   , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_uint32_rnint_226, bid128_to_uint32_rnint, 0x303C00000000000000000031FFFFFFCFu128, 2147483648   , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_rnint_227, bid128_to_uint32_rnint, 0x303C0000000000000000003200000031u128, 2147483648   , 0x00); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_rnint_228, bid128_to_uint32_rnint, 0x303C0000000000000000003200000032u128, 2147483648   , 0x00); // -- 2^31+0.5
dec_test!(bid128_to_uint32_rnint_229, bid128_to_uint32_rnint, 0x303C0000000000000000003200000033u128, 2147483649   , 0x00); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_rnint_230, bid128_to_uint32_rnint, 0x303C00000000000000000063FFFFFFCDu128, 4294967295   , 0x00); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_rnint_231, bid128_to_uint32_rnint, 0x303C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_rnint_232, bid128_to_uint32_rnint, 0x303C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_rnint_233, bid128_to_uint32_rnint, 0x303C0000000000000000006400000031u128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_rnint_234, bid128_to_uint32_rnint, 0x303C0000000000000000006400000032u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_rnint_235, bid128_to_uint32_rnint, 0x303C0000000000000000006400000033u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_rnint_236, bid128_to_uint32_rnint, 0x303E0000000000000000000000000005u128, 0            , 0x00); // -- 0.5
dec_test!(bid128_to_uint32_rnint_237, bid128_to_uint32_rnint, 0x303E000000000000000000000000000Fu128, 2            , 0x00); // -- 1.5
dec_test!(bid128_to_uint32_rnint_238, bid128_to_uint32_rnint, 0x303E0000000000000000000000000BB7u128, 300          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_239, bid128_to_uint32_rnint, 0x303E0000000000000000000000000BB8u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_240, bid128_to_uint32_rnint, 0x303E0000000000000000000000000BB9u128, 300          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_241, bid128_to_uint32_rnint, 0x303E0000000000000000000000000BBDu128, 300          , 0x00); // -- 300.5
dec_test!(bid128_to_uint32_rnint_242, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFF1u128, 2147483646   , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_uint32_rnint_243, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFF5u128, 2147483647   , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_rnint_244, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFF6u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_rnint_245, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFF7u128, 2147483647   , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_rnint_246, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFFBu128, 2147483648   , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_uint32_rnint_247, bid128_to_uint32_rnint, 0x303E00000000000000000004FFFFFFFFu128, 2147483648   , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_uint32_rnint_248, bid128_to_uint32_rnint, 0x303E0000000000000000000500000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_rnint_249, bid128_to_uint32_rnint, 0x303E0000000000000000000500000001u128, 2147483648   , 0x00); // -- 2^31+ulp
dec_test!(bid128_to_uint32_rnint_250, bid128_to_uint32_rnint, 0x303E0000000000000000000500000005u128, 2147483648   , 0x00); // -- 2^31+0.5
dec_test!(bid128_to_uint32_rnint_251, bid128_to_uint32_rnint, 0x303E0000000000000000000500000009u128, 2147483649   , 0x00); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_rnint_252, bid128_to_uint32_rnint, 0x303E000000000000000000050000000Au128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_rnint_253, bid128_to_uint32_rnint, 0x303E000000000000000000050000000Bu128, 2147483649   , 0x00); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_rnint_254, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFF5u128, 4294967295   , 0x00); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_rnint_255, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFF6u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_rnint_256, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFF7u128, 4294967295   , 0x00); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_rnint_257, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_rnint_258, bid128_to_uint32_rnint, 0x303E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_uint32_rnint_259, bid128_to_uint32_rnint, 0x303E0000000000000000000A00000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_rnint_260, bid128_to_uint32_rnint, 0x303E0000000000000000000A00000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_rnint_261, bid128_to_uint32_rnint, 0x303E0000000000000000000A00000005u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_rnint_262, bid128_to_uint32_rnint, 0x303E0000000000000000000A00000009u128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_rnint_263, bid128_to_uint32_rnint, 0x303E0000000000000000000A0000000Au128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_rnint_264, bid128_to_uint32_rnint, 0x303E0000000000000000000A0000000Bu128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_rnint_265, bid128_to_uint32_rnint, 0x303E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_rnint_266, bid128_to_uint32_rnint, 0x303E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_rnint_267, bid128_to_uint32_rnint, 0x303E0000000000000000002E90EDD005u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_rnint_268, bid128_to_uint32_rnint, 0x303E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_rnint_269, bid128_to_uint32_rnint, 0x30400000000000000000000000000001u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_rnint_270, bid128_to_uint32_rnint, 0x3040000000000000000000000000012Bu128, 299          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_271, bid128_to_uint32_rnint, 0x3040000000000000000000000000012Cu128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_272, bid128_to_uint32_rnint, 0x3040000000000000000000000000012Du128, 301          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_273, bid128_to_uint32_rnint, 0x3040000000000000000000007FFFFFFFu128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_rnint_274, bid128_to_uint32_rnint, 0x30400000000000000000000080000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_rnint_275, bid128_to_uint32_rnint, 0x30400000000000000000000080000001u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_rnint_276, bid128_to_uint32_rnint, 0x304000000000000000000000FFFFFFFFu128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_rnint_277, bid128_to_uint32_rnint, 0x30400000000000000000000100000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_rnint_278, bid128_to_uint32_rnint, 0x30400000000000000000000100000001u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_rnint_279, bid128_to_uint32_rnint, 0x304000000000000000000004A817C7FFu128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_rnint_280, bid128_to_uint32_rnint, 0x304000000000000000000004A817C801u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_rnint_281, bid128_to_uint32_rnint, 0x3041ED09BEAD87C0378D8E6400000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_282, bid128_to_uint32_rnint, 0x3042000000000000000000000000001Du128, 290          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_rnint_283, bid128_to_uint32_rnint, 0x3042000000000000000000000000001Eu128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_284, bid128_to_uint32_rnint, 0x3042000000000000000000000000001Fu128, 310          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_rnint_285, bid128_to_uint32_rnint, 0x304200000000000000000000773593FFu128, 2147483648   , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_uint32_rnint_286, bid128_to_uint32_rnint, 0x30420000000000000000000077359400u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_rnint_287, bid128_to_uint32_rnint, 0x30420000000000000000000077359401u128, 2147483648   , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_uint32_rnint_288, bid128_to_uint32_rnint, 0x30440000000000000000000000000003u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_rnint_289, bid128_to_uint32_rnint, 0x30520000000000000000000000000004u128, 4000000000   , 0x00); // -- 4e9
dec_test!(bid128_to_uint32_rnint_290, bid128_to_uint32_rnint, 0x30520000000000000000000000000005u128, 2147483648   , 0x01); // -- 5e9
dec_test!(bid128_to_uint32_rnint_291, bid128_to_uint32_rnint, 0x30540000000000000000000000000002u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_rnint_292, bid128_to_uint32_rnint, 0x38a80127243d5e393f8d670de6e9ff87u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_293, bid128_to_uint32_rnint, "4294967296"                          , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_294, bid128_to_uint32_rnint, 0x49f83dd5788fbf5876ebde8bb9a40e82u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_295, bid128_to_uint32_rnint, 0x53f20000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_296, bid128_to_uint32_rnint, 0x544ba795b9bb9926658c2582b222c055u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_297, bid128_to_uint32_rnint, "5.5"                                 , 6            , 0x00);
dec_test!(bid128_to_uint32_rnint_298, bid128_to_uint32_rnint, 0x57e91fc2692729d68cdb9420d7383914u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_299, bid128_to_uint32_rnint, 0x620a256801220ee0eeee5e779eefd79bu128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_300, bid128_to_uint32_rnint, 0x78000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_rnint_301, bid128_to_uint32_rnint, 0x7ae75c2ec2fbf9f7fffffdbdfe7fff5fu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_302, bid128_to_uint32_rnint, 0x7bff7fffaff7fffcfffffeffffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_303, bid128_to_uint32_rnint, 0x7c000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_rnint_304, bid128_to_uint32_rnint, 0x7c003fffffffffff38c15b08ffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_305, bid128_to_uint32_rnint, 0x7c003fffffffffff38c15b0affffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_306, bid128_to_uint32_rnint, 0x7e000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_rnint_307, bid128_to_uint32_rnint, 0x923a2ad67a9ea6348010000001010401u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_308, bid128_to_uint32_rnint, 0xa06bb3c35abf22030f3897599156be4eu128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_309, bid128_to_uint32_rnint, 0xabb60000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_310, bid128_to_uint32_rnint, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_rnint_311, bid128_to_uint32_rnint, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0            , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint32_rnint_312, bid128_to_uint32_rnint, 0xAFFCF684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_rnint_313, bid128_to_uint32_rnint, 0xaffddfbcffff9dfc0000412000000020u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_314, bid128_to_uint32_rnint, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_rnint_315, bid128_to_uint32_rnint, 0xAFFDEC8B86EF679D76FC433D80000000u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_rnint_316, bid128_to_uint32_rnint, 0xAFFDEC8B86EF679D76FC433D80000001u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_rnint_317, bid128_to_uint32_rnint, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_rnint_318, bid128_to_uint32_rnint, 0xAFFE314DC6448D9338C15B0A00000000u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_rnint_319, bid128_to_uint32_rnint, 0xAFFE314DC6448D9338C15B0A00000001u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_rnint_320, bid128_to_uint32_rnint, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_rnint_321, bid128_to_uint32_rnint, 0xAFFE49F4A966D45CD522088F00000000u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_rnint_322, bid128_to_uint32_rnint, 0xAFFE49F4A966D45CD522088F00000001u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_rnint_323, bid128_to_uint32_rnint, 0xb0000112018a008164c08d66a22003cau128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_324, bid128_to_uint32_rnint, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_325, bid128_to_uint32_rnint, 0xB00293E952CDA8B9AA44111E00000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_326, bid128_to_uint32_rnint, 0xB00293E952CDA8B9AA44111E00000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_327, bid128_to_uint32_rnint, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_rnint_328, bid128_to_uint32_rnint, 0xB00294286EACB8CB0A8CB6B140000000u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_rnint_329, bid128_to_uint32_rnint, 0xB00294286EACB8CB0A8CB6B140000001u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_rnint_330, bid128_to_uint32_rnint, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_331, bid128_to_uint32_rnint, 0xB0040ECA8847C4129106CE8300000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_332, bid128_to_uint32_rnint, 0xB0040ECA8847C4129106CE8300000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_333, bid128_to_uint32_rnint, 0xb008c84384c1b86e60d63da585053b7fu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_334, bid128_to_uint32_rnint, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_335, bid128_to_uint32_rnint, 0xB00A0003C95A2F0B4856475FE0000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_336, bid128_to_uint32_rnint, 0xB00A0003C95A2F0B4856475FE0000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_337, bid128_to_uint32_rnint, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_338, bid128_to_uint32_rnint, 0xB00C000060EF6B1ABA6F072330000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_339, bid128_to_uint32_rnint, 0xB00C000060EF6B1ABA6F072330000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_340, bid128_to_uint32_rnint, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_rnint_341, bid128_to_uint32_rnint, 0xB01069E10DE628D3A6C9CC9B8E800000u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_rnint_342, bid128_to_uint32_rnint, 0xB01069E10DE628D3A6C9CC9B8E800001u128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_rnint_343, bid128_to_uint32_rnint, 0xB01069E10DE692B4B4B133125EFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_rnint_344, bid128_to_uint32_rnint, 0xB01069E10DE692B4B4B133125F000000u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_rnint_345, bid128_to_uint32_rnint, 0xB01069E10DE692B4B4B133125F000001u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_rnint_346, bid128_to_uint32_rnint, 0xB01069E10DE6FC95C29899892F7FFFFFu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_rnint_347, bid128_to_uint32_rnint, 0xB01069E10DE6FC95C29899892F800000u128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_rnint_348, bid128_to_uint32_rnint, 0xB01069E10DE6FC95C29899892F800001u128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_rnint_349, bid128_to_uint32_rnint, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_rnint_350, bid128_to_uint32_rnint, 0xB01069E10DE76676D080000000000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_rnint_351, bid128_to_uint32_rnint, 0xB01069E10DE76676D080000000000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_rnint_352, bid128_to_uint32_rnint, 0xB01069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_rnint_353, bid128_to_uint32_rnint, 0xB01069E10DE7D057DE676676D0800000u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_rnint_354, bid128_to_uint32_rnint, 0xB01069E10DE7D057DE676676D0800001u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_rnint_355, bid128_to_uint32_rnint, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_rnint_356, bid128_to_uint32_rnint, 0xB01069E10DE83A38EC4ECCEDA1000000u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_rnint_357, bid128_to_uint32_rnint, 0xB01069E10DE83A38EC4ECCEDA1000001u128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_rnint_358, bid128_to_uint32_rnint, 0xb010a58000004846be48dd32fd8eebb6u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_359, bid128_to_uint32_rnint, 0xB010C5371912364CE3056C27FFFFFFFFu128, 2147483648   , 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_uint32_rnint_360, bid128_to_uint32_rnint, 0xB010C5371912364CE3056C2800000000u128, 2147483648   , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint32_rnint_361, bid128_to_uint32_rnint, 0xB010C5371912364CE3056C2800000001u128, 2147483648   , 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_uint32_rnint_362, bid128_to_uint32_rnint, 0xB010D3C21BCDF92B853133125EFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_rnint_363, bid128_to_uint32_rnint, 0xB010D3C21BCDF92B853133125F000000u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_rnint_364, bid128_to_uint32_rnint, 0xB010D3C21BCDF92B853133125F000001u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_rnint_365, bid128_to_uint32_rnint, 0xB010D3C21BCE630C931899892F7FFFFFu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_rnint_366, bid128_to_uint32_rnint, 0xB010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_rnint_367, bid128_to_uint32_rnint, 0xB010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_rnint_368, bid128_to_uint32_rnint, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_rnint_369, bid128_to_uint32_rnint, 0xB010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_rnint_370, bid128_to_uint32_rnint, 0xB010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_rnint_371, bid128_to_uint32_rnint, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_rnint_372, bid128_to_uint32_rnint, 0xB010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_rnint_373, bid128_to_uint32_rnint, 0xB010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_rnint_374, bid128_to_uint32_rnint, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_rnint_375, bid128_to_uint32_rnint, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_rnint_376, bid128_to_uint32_rnint, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_rnint_377, bid128_to_uint32_rnint, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_uint32_rnint_378, bid128_to_uint32_rnint, 0xB010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint32_rnint_379, bid128_to_uint32_rnint, 0xB010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_uint32_rnint_380, bid128_to_uint32_rnint, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_rnint_381, bid128_to_uint32_rnint, 0xB012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_rnint_382, bid128_to_uint32_rnint, 0xB012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_rnint_383, bid128_to_uint32_rnint, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_rnint_384, bid128_to_uint32_rnint, 0xB012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_rnint_385, bid128_to_uint32_rnint, 0xB012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_rnint_386, bid128_to_uint32_rnint, 0xB012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_rnint_387, bid128_to_uint32_rnint, 0xB012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_rnint_388, bid128_to_uint32_rnint, 0xB012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_rnint_389, bid128_to_uint32_rnint, 0xB012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_rnint_390, bid128_to_uint32_rnint, 0xB012629B8C891B267182B61400000000u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_rnint_391, bid128_to_uint32_rnint, 0xB012629B8C891B267182B61400000001u128, 2147483648   , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_rnint_392, bid128_to_uint32_rnint, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_rnint_393, bid128_to_uint32_rnint, 0xB012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_rnint_394, bid128_to_uint32_rnint, 0xB012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_rnint_395, bid128_to_uint32_rnint, 0xB012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_rnint_396, bid128_to_uint32_rnint, 0xB012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_rnint_397, bid128_to_uint32_rnint, 0xB012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_rnint_398, bid128_to_uint32_rnint, 0xB012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_rnint_399, bid128_to_uint32_rnint, 0xB012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_rnint_400, bid128_to_uint32_rnint, 0xB012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_rnint_401, bid128_to_uint32_rnint, 0xB01600000000003627E8F712373BFFFFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_rnint_402, bid128_to_uint32_rnint, 0xB01600000000003627E8F712373C0000u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_rnint_403, bid128_to_uint32_rnint, 0xB01600000000003627E8F712373C0001u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_rnint_404, bid128_to_uint32_rnint, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_rnint_405, bid128_to_uint32_rnint, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_rnint_406, bid128_to_uint32_rnint, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_rnint_407, bid128_to_uint32_rnint, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_rnint_408, bid128_to_uint32_rnint, 0xB0180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_rnint_409, bid128_to_uint32_rnint, 0xB0180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_rnint_410, bid128_to_uint32_rnint, 0xB0180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_rnint_411, bid128_to_uint32_rnint, 0xB0180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_rnint_412, bid128_to_uint32_rnint, 0xB0180002B5E3AF19676BAF16B1880001u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_rnint_413, bid128_to_uint32_rnint, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_rnint_414, bid128_to_uint32_rnint, 0xB01800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_rnint_415, bid128_to_uint32_rnint, 0xB01800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_rnint_416, bid128_to_uint32_rnint, 0xB01800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_rnint_417, bid128_to_uint32_rnint, 0xB01800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_rnint_418, bid128_to_uint32_rnint, 0xB01800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_rnint_419, bid128_to_uint32_rnint, 0xb018000c00000000deffffbfddfdfbffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_420, bid128_to_uint32_rnint, 0xB01A0000000000004563918244F3FFFFu128, 0            , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_rnint_421, bid128_to_uint32_rnint, 0xB01A0000000000004563918244F40000u128, 0            , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint32_rnint_422, bid128_to_uint32_rnint, 0xB01A0000000000004563918244F40001u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_rnint_423, bid128_to_uint32_rnint, 0xB01A0000000000008AC7230489E7FFFFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_rnint_424, bid128_to_uint32_rnint, 0xB01A0000000000008AC7230489E80000u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_rnint_425, bid128_to_uint32_rnint, 0xB01A0000000000008AC7230489E80001u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_rnint_426, bid128_to_uint32_rnint, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_rnint_427, bid128_to_uint32_rnint, 0xB01A0000000000A2E6C09AD3E0D40000u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_rnint_428, bid128_to_uint32_rnint, 0xB01A0000000000A2E6C09AD3E0D40001u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_rnint_429, bid128_to_uint32_rnint, 0xB01A000045639181BA2CDCFB7617FFFFu128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_rnint_430, bid128_to_uint32_rnint, 0xB01A000045639181BA2CDCFB76180000u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_rnint_431, bid128_to_uint32_rnint, 0xB01A000045639181BA2CDCFB76180001u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_rnint_432, bid128_to_uint32_rnint, 0xB01A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_rnint_433, bid128_to_uint32_rnint, 0xB01A00004563918244F4000000000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_rnint_434, bid128_to_uint32_rnint, 0xB01A00004563918244F4000000000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_rnint_435, bid128_to_uint32_rnint, 0xB01A000045639182CFBB230489E7FFFFu128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_rnint_436, bid128_to_uint32_rnint, 0xB01A000045639182CFBB230489E80000u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_rnint_437, bid128_to_uint32_rnint, 0xB01A000045639182CFBB230489E80001u128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_rnint_438, bid128_to_uint32_rnint, 0xB01A00008AC72303FF20DCFB7617FFFFu128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_rnint_439, bid128_to_uint32_rnint, 0xB01A00008AC72303FF20DCFB76180000u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_rnint_440, bid128_to_uint32_rnint, 0xB01A00008AC72303FF20DCFB76180001u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_rnint_441, bid128_to_uint32_rnint, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_rnint_442, bid128_to_uint32_rnint, 0xB01A00008AC7230489E8000000000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_rnint_443, bid128_to_uint32_rnint, 0xB01A00008AC7230489E8000000000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_rnint_444, bid128_to_uint32_rnint, 0xB01A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_rnint_445, bid128_to_uint32_rnint, 0xB01A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_rnint_446, bid128_to_uint32_rnint, 0xB01A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_rnint_447, bid128_to_uint32_rnint, 0xB01C00000000000014D1120D7B15FFFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_rnint_448, bid128_to_uint32_rnint, 0xB01C00000000000014D1120D7B160000u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_rnint_449, bid128_to_uint32_rnint, 0xB01C00000000000014D1120D7B160001u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_rnint_450, bid128_to_uint32_rnint, 0xB01E000000000001A055690D9DB7FFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_451, bid128_to_uint32_rnint, 0xB01E000000000001A055690D9DB80000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_452, bid128_to_uint32_rnint, 0xB01E000000000001A055690D9DB80001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_453, bid128_to_uint32_rnint, 0xB02000000000000029A2241AF62BFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_454, bid128_to_uint32_rnint, 0xB02000000000000029A2241AF62C0000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_455, bid128_to_uint32_rnint, 0xB02000000000000029A2241AF62C0001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_456, bid128_to_uint32_rnint, 0xB024000000000000006A94D74F42FFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_457, bid128_to_uint32_rnint, 0xB024000000000000006A94D74F430000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_458, bid128_to_uint32_rnint, 0xB024000000000000006A94D74F430001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_459, bid128_to_uint32_rnint, 0xB02A00000000000000000017428106FFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_rnint_460, bid128_to_uint32_rnint, 0xB02A0000000000000000001742810700u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_rnint_461, bid128_to_uint32_rnint, 0xB02A0000000000000000001742810701u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_rnint_462, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_rnint_463, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_rnint_464, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_rnint_465, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_rnint_466, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_rnint_467, bid128_to_uint32_rnint, 0xB02A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_rnint_468, bid128_to_uint32_rnint, 0xB02C000000000000000002BBA7F521FFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_rnint_469, bid128_to_uint32_rnint, 0xB02C000000000000000002BBA7F52200u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_rnint_470, bid128_to_uint32_rnint, 0xB02C000000000000000002BBA7F52201u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_rnint_471, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_rnint_472, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_rnint_473, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_rnint_474, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_rnint_475, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_rnint_476, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_rnint_477, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_rnint_478, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_rnint_479, bid128_to_uint32_rnint, 0xB02C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_rnint_480, bid128_to_uint32_rnint, 0xB02E000000000000000000001DCD64FFu128, 0            , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_rnint_481, bid128_to_uint32_rnint, 0xB02E000000000000000000001DCD6500u128, 0            , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint32_rnint_482, bid128_to_uint32_rnint, 0xB02E000000000000000000001DCD6501u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_rnint_483, bid128_to_uint32_rnint, 0xB02E000000000000000000003B9AC9FFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_rnint_484, bid128_to_uint32_rnint, 0xB02E000000000000000000003B9ACA00u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_rnint_485, bid128_to_uint32_rnint, 0xB02E000000000000000000003B9ACA01u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_rnint_486, bid128_to_uint32_rnint, 0xB02E0000000000000000000059682EFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_rnint_487, bid128_to_uint32_rnint, 0xB02E0000000000000000000059682F00u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_rnint_488, bid128_to_uint32_rnint, 0xB02E0000000000000000000059682F01u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_rnint_489, bid128_to_uint32_rnint, 0xB02E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_rnint_490, bid128_to_uint32_rnint, 0xB02E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_rnint_491, bid128_to_uint32_rnint, 0xB02E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_rnint_492, bid128_to_uint32_rnint, 0xB03000000000000000000006FC23ABFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_493, bid128_to_uint32_rnint, 0xB03000000000000000000006FC23AC00u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_494, bid128_to_uint32_rnint, 0xB03000000000000000000006FC23AC01u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_495, bid128_to_uint32_rnint, 0xB03200000000000000000000B2D05DFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_496, bid128_to_uint32_rnint, 0xB03200000000000000000000B2D05E00u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_497, bid128_to_uint32_rnint, 0xB03200000000000000000000B2D05E01u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_498, bid128_to_uint32_rnint, 0xB03800000000000000000000002DDA47u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_rnint_499, bid128_to_uint32_rnint, 0xB03800000000000000000000002DDA48u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_rnint_500, bid128_to_uint32_rnint, 0xB03800000000000000000000002DDA49u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_rnint_501, bid128_to_uint32_rnint, 0xB03A00000000000000000000000003E7u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_rnint_502, bid128_to_uint32_rnint, 0xB03A00000000000000000000000005DBu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_rnint_503, bid128_to_uint32_rnint, 0xB03A00000000000000000000000005DCu128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_rnint_504, bid128_to_uint32_rnint, 0xB03A00000000000000000000000005DDu128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_rnint_505, bid128_to_uint32_rnint, 0xB03A00000000000000000000000495D3u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_rnint_506, bid128_to_uint32_rnint, 0xB03A00000000000000000000000495D4u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_rnint_507, bid128_to_uint32_rnint, 0xB03A00000000000000000000000495D5u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_rnint_508, bid128_to_uint32_rnint, 0xB03C0000000000000000000000000095u128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_rnint_509, bid128_to_uint32_rnint, 0xB03C0000000000000000000000000096u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_rnint_510, bid128_to_uint32_rnint, 0xB03C0000000000000000000000000097u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_rnint_511, bid128_to_uint32_rnint, 0xB03C0000000000000000000000007561u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_rnint_512, bid128_to_uint32_rnint, 0xB03C0000000000000000000000007562u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_rnint_513, bid128_to_uint32_rnint, 0xB03C0000000000000000000000007563u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_rnint_514, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFF69u128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_rnint_515, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFF6Au128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_rnint_516, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFF6Bu128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_rnint_517, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFFCDu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_rnint_518, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFFCEu128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_rnint_519, bid128_to_uint32_rnint, 0xB03C00000000000000000031FFFFFFCFu128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_rnint_520, bid128_to_uint32_rnint, 0xB03C0000000000000000003200000031u128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_rnint_521, bid128_to_uint32_rnint, 0xB03C0000000000000000003200000032u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_rnint_522, bid128_to_uint32_rnint, 0xB03C0000000000000000003200000033u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_rnint_523, bid128_to_uint32_rnint, 0xB03C00000000000000000063FFFFFFCDu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_rnint_524, bid128_to_uint32_rnint, 0xB03C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_rnint_525, bid128_to_uint32_rnint, 0xB03C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_rnint_526, bid128_to_uint32_rnint, 0xB03C0000000000000000006400000031u128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_rnint_527, bid128_to_uint32_rnint, 0xB03C0000000000000000006400000032u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_rnint_528, bid128_to_uint32_rnint, 0xB03C0000000000000000006400000033u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_rnint_529, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000005u128, 0            , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint32_rnint_530, bid128_to_uint32_rnint, 0xB03E000000000000000000000000000Fu128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_rnint_531, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000BB7u128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_532, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000BB8u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_533, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000BB9u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_534, bid128_to_uint32_rnint, 0xB03E0000000000000000000000000BBDu128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_rnint_535, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFF1u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_rnint_536, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFF5u128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_rnint_537, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFF6u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_rnint_538, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFF7u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_rnint_539, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFFBu128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_rnint_540, bid128_to_uint32_rnint, 0xB03E00000000000000000004FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_rnint_541, bid128_to_uint32_rnint, 0xB03E0000000000000000000500000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_rnint_542, bid128_to_uint32_rnint, 0xB03E0000000000000000000500000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_rnint_543, bid128_to_uint32_rnint, 0xB03E0000000000000000000500000005u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_rnint_544, bid128_to_uint32_rnint, 0xB03E0000000000000000000500000009u128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_rnint_545, bid128_to_uint32_rnint, 0xB03E000000000000000000050000000Au128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_rnint_546, bid128_to_uint32_rnint, 0xB03E000000000000000000050000000Bu128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_rnint_547, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFF5u128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_rnint_548, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFF6u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_rnint_549, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFF7u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_rnint_550, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_rnint_551, bid128_to_uint32_rnint, 0xB03E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_rnint_552, bid128_to_uint32_rnint, 0xB03E0000000000000000000A00000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_rnint_553, bid128_to_uint32_rnint, 0xB03E0000000000000000000A00000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_rnint_554, bid128_to_uint32_rnint, 0xB03E0000000000000000000A00000005u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_rnint_555, bid128_to_uint32_rnint, 0xB03E0000000000000000000A00000009u128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_rnint_556, bid128_to_uint32_rnint, 0xB03E0000000000000000000A0000000Au128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_rnint_557, bid128_to_uint32_rnint, 0xB03E0000000000000000000A0000000Bu128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_rnint_558, bid128_to_uint32_rnint, 0xB03E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_rnint_559, bid128_to_uint32_rnint, 0xB03E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_rnint_560, bid128_to_uint32_rnint, 0xB03E0000000000000000002E90EDD005u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_rnint_561, bid128_to_uint32_rnint, 0xB03E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_rnint_562, bid128_to_uint32_rnint, 0xB0400000000000000000000000000001u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_rnint_563, bid128_to_uint32_rnint, 0xB040000000000000000000000000012Bu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_564, bid128_to_uint32_rnint, 0xB040000000000000000000000000012Cu128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_565, bid128_to_uint32_rnint, 0xB040000000000000000000000000012Du128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_566, bid128_to_uint32_rnint, 0xB040000000000000000000007FFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_rnint_567, bid128_to_uint32_rnint, 0xB0400000000000000000000080000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_rnint_568, bid128_to_uint32_rnint, 0xB0400000000000000000000080000001u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_rnint_569, bid128_to_uint32_rnint, 0xB04000000000000000000000FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_rnint_570, bid128_to_uint32_rnint, 0xB0400000000000000000000100000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_rnint_571, bid128_to_uint32_rnint, 0xB0400000000000000000000100000001u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_rnint_572, bid128_to_uint32_rnint, 0xB04000000000000000000004A817C7FFu128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_rnint_573, bid128_to_uint32_rnint, 0xB04000000000000000000004A817C801u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_rnint_574, bid128_to_uint32_rnint, 0xB042000000000000000000000000001Du128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_rnint_575, bid128_to_uint32_rnint, 0xB042000000000000000000000000001Eu128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_576, bid128_to_uint32_rnint, 0xB042000000000000000000000000001Fu128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_rnint_577, bid128_to_uint32_rnint, 0xB04200000000000000000000773593FFu128, 2147483648   , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_rnint_578, bid128_to_uint32_rnint, 0xB0420000000000000000000077359400u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_rnint_579, bid128_to_uint32_rnint, 0xB0420000000000000000000077359401u128, 2147483648   , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_rnint_580, bid128_to_uint32_rnint, 0xB0440000000000000000000000000003u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_rnint_581, bid128_to_uint32_rnint, 0xB0520000000000000000000000000004u128, 2147483648   , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint32_rnint_582, bid128_to_uint32_rnint, 0xB0520000000000000000000000000005u128, 2147483648   , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint32_rnint_583, bid128_to_uint32_rnint, 0xB0540000000000000000000000000002u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_rnint_584, bid128_to_uint32_rnint, 0xb6c25dfe6fec677de578aa8aa40f4b1du128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_585, bid128_to_uint32_rnint, 0xc0240000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_586, bid128_to_uint32_rnint, 0xd8749bc1592c005392cb8ac564abffa5u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_587, bid128_to_uint32_rnint, 0xe453ad749a5880dbff893a32625017f1u128, 0            , 0x00);
dec_test!(bid128_to_uint32_rnint_588, bid128_to_uint32_rnint, 0xfa2152399330f7fd42afeb0123ae5dc1u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_589, bid128_to_uint32_rnint, 0xfbfbef1e1fbffe2f0000422200400210u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_590, bid128_to_uint32_rnint, 0xfddfffffffffffff3bfebbef5c91ea5bu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_591, bid128_to_uint32_rnint, 0xfdfddffd7f6ffefffd48f47fb8fbeaefu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_592, bid128_to_uint32_rnint, 0xfeffefbfffffeff72a97bf5e030d67c8u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_593, bid128_to_uint32_rnint, 0xfffbffffffffffffffffffffffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_594, bid128_to_uint32_rnint, "-Infinity"                           , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_595, bid128_to_uint32_rnint, "Infinity"                            , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_596, bid128_to_uint32_rnint, "QNaN"                                , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_rnint_597, bid128_to_uint32_rnint, "SNaN"                                , 0x80000000u32, 0x01);