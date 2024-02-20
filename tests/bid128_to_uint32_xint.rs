/* // -------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* // ------------------------------------------------------------------------------------------------ */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* // ------------------------------------------------------------------------------------------------ */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.                        */
/* // ------------------------------------------------------------------------------------------------ */

mod common;

dec_test!(bid128_to_uint32_xint_001, bid128_to_uint32_xint, "-0"                                  , 0            , 0x00);
dec_test!(bid128_to_uint32_xint_002, bid128_to_uint32_xint, 0                                     , 0            , 0x00);
dec_test!(bid128_to_uint32_xint_003, bid128_to_uint32_xint, 0x00000000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xint_004, bid128_to_uint32_xint, 0x0001ed09bead87c0378d8e62ffffffffu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xint_005, bid128_to_uint32_xint, 0x0001ed09bead87c0378d8e64ffffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_xint_006, bid128_to_uint32_xint, 0x0040000000000000cdffffffef77eeeeu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xint_007, bid128_to_uint32_xint, 0x013aca97c65584b4f3370a551ac49502u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xint_008, bid128_to_uint32_xint, "+0.1E0"                              , 0            , 0x20);
dec_test!(bid128_to_uint32_xint_009, bid128_to_uint32_xint, 0x02bc3823f528ac3a5ae912b01b7c2dcau128, 0            , 0x20);
dec_test!(bid128_to_uint32_xint_010, bid128_to_uint32_xint, "+10.00000000000010000E0"             , 10           , 0x20);
dec_test!(bid128_to_uint32_xint_011, bid128_to_uint32_xint, "1.0"                                 , 1            , 0x00);
dec_test!(bid128_to_uint32_xint_012, bid128_to_uint32_xint, 1073741824                            , 1073741824   , 0x00);
dec_test!(bid128_to_uint32_xint_013, bid128_to_uint32_xint, 1                                     , 1            , 0x00);
dec_test!(bid128_to_uint32_xint_014, bid128_to_uint32_xint, "+110.1000111E0"                      , 110          , 0x20);
dec_test!(bid128_to_uint32_xint_015, bid128_to_uint32_xint, 0x20400000000000000000000010000002u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xint_016, bid128_to_uint32_xint, "+24256236.597649E0"                  , 24256236     , 0x20);
dec_test!(bid128_to_uint32_xint_017, bid128_to_uint32_xint, 0x269c25d4832a52e21c7df302fc64396cu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xint_018, bid128_to_uint32_xint, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x20); //  0.5-ulp
dec_test!(bid128_to_uint32_xint_019, bid128_to_uint32_xint, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0            , 0x20); //  0.5
dec_test!(bid128_to_uint32_xint_020, bid128_to_uint32_xint, 0x2FFCF684DF56C3E01BC6C73200000001u128, 0            , 0x20); //  0.5+ulp
dec_test!(bid128_to_uint32_xint_021, bid128_to_uint32_xint, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 0            , 0x20); //  0.999-ulp
dec_test!(bid128_to_uint32_xint_022, bid128_to_uint32_xint, 0x2FFDEC8B86EF679D76FC433D80000000u128, 0            , 0x20); //  0.999
dec_test!(bid128_to_uint32_xint_023, bid128_to_uint32_xint, 0x2FFDEC8B86EF679D76FC433D80000001u128, 0            , 0x20); //  0.999+ulp
dec_test!(bid128_to_uint32_xint_024, bid128_to_uint32_xint, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 0            , 0x20); //  1-ulp
dec_test!(bid128_to_uint32_xint_025, bid128_to_uint32_xint, 0x2FFE314DC6448D9338C15B0A00000000u128, 1            , 0x00); //  1
dec_test!(bid128_to_uint32_xint_026, bid128_to_uint32_xint, 0x2FFE314DC6448D9338C15B0A00000001u128, 1            , 0x20); //  1+ulp
dec_test!(bid128_to_uint32_xint_027, bid128_to_uint32_xint, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1            , 0x20); //  1.5-ulp
dec_test!(bid128_to_uint32_xint_028, bid128_to_uint32_xint, 0x2FFE49F4A966D45CD522088F00000000u128, 1            , 0x20); //  1.5
dec_test!(bid128_to_uint32_xint_029, bid128_to_uint32_xint, 0x2FFE49F4A966D45CD522088F00000001u128, 1            , 0x20); //  1.5+ulp
dec_test!(bid128_to_uint32_xint_030, bid128_to_uint32_xint, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_031, bid128_to_uint32_xint, 0x300293E952CDA8B9AA44111E00000000u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_032, bid128_to_uint32_xint, 0x300293E952CDA8B9AA44111E00000001u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_033, bid128_to_uint32_xint, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300          , 0x20); //  300.5-ulp
dec_test!(bid128_to_uint32_xint_034, bid128_to_uint32_xint, 0x300294286EACB8CB0A8CB6B140000000u128, 300          , 0x20); //  300.5
dec_test!(bid128_to_uint32_xint_035, bid128_to_uint32_xint, 0x300294286EACB8CB0A8CB6B140000001u128, 300          , 0x20); //  300.5+ulp
dec_test!(bid128_to_uint32_xint_036, bid128_to_uint32_xint, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_037, bid128_to_uint32_xint, 0x30040ECA8847C4129106CE8300000000u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_038, bid128_to_uint32_xint, 0x30040ECA8847C4129106CE8300000001u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_039, bid128_to_uint32_xint, 0x3004880000371240288c029108140a0au128, 2758         , 0x20);
dec_test!(bid128_to_uint32_xint_040, bid128_to_uint32_xint, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_041, bid128_to_uint32_xint, 0x300A0003C95A2F0B4856475FE0000000u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_042, bid128_to_uint32_xint, 0x300A0003C95A2F0B4856475FE0000001u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_043, bid128_to_uint32_xint, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_044, bid128_to_uint32_xint, 0x300C000060EF6B1ABA6F072330000000u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_045, bid128_to_uint32_xint, 0x300C000060EF6B1ABA6F072330000001u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_046, bid128_to_uint32_xint, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646   , 0x20); //  2^31-1.5-ulp
dec_test!(bid128_to_uint32_xint_047, bid128_to_uint32_xint, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483646   , 0x20); //  2^31-1.5
dec_test!(bid128_to_uint32_xint_048, bid128_to_uint32_xint, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483646   , 0x20); //  2^31-1.5+ulp
dec_test!(bid128_to_uint32_xint_049, bid128_to_uint32_xint, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483646   , 0x20); //  2^31-1-ulp
dec_test!(bid128_to_uint32_xint_050, bid128_to_uint32_xint, 0x301069E10DE692B4B4B133125F000000u128, 2147483647   , 0x00); //  2^31-1
dec_test!(bid128_to_uint32_xint_051, bid128_to_uint32_xint, 0x301069E10DE692B4B4B133125F000001u128, 2147483647   , 0x20); //  2^31-1+ulp
dec_test!(bid128_to_uint32_xint_052, bid128_to_uint32_xint, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647   , 0x20); //  2^31-0.5-ulp
dec_test!(bid128_to_uint32_xint_053, bid128_to_uint32_xint, 0x301069E10DE6FC95C29899892F800000u128, 2147483647   , 0x20); //  2^31-0.5
dec_test!(bid128_to_uint32_xint_054, bid128_to_uint32_xint, 0x301069E10DE6FC95C29899892F800001u128, 2147483647   , 0x20); //  2^31-0.5+ulp
dec_test!(bid128_to_uint32_xint_055, bid128_to_uint32_xint, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, 2147483647   , 0x20); //  2^31-ulp
dec_test!(bid128_to_uint32_xint_056, bid128_to_uint32_xint, 0x301069E10DE76676D080000000000000u128, 2147483648   , 0x00); //  2^31
dec_test!(bid128_to_uint32_xint_057, bid128_to_uint32_xint, 0x301069E10DE76676D080000000000001u128, 2147483648   , 0x20); //  2^31+ulp
dec_test!(bid128_to_uint32_xint_058, bid128_to_uint32_xint, 0x301069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x20); //  2^31+0.5-ulp
dec_test!(bid128_to_uint32_xint_059, bid128_to_uint32_xint, 0x301069E10DE7D057DE676676D0800000u128, 2147483648   , 0x20); //  2^31+0.5
dec_test!(bid128_to_uint32_xint_060, bid128_to_uint32_xint, 0x301069E10DE7D057DE676676D0800001u128, 2147483648   , 0x20); //  2^31+0.5+ulp
dec_test!(bid128_to_uint32_xint_061, bid128_to_uint32_xint, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483648   , 0x20); //  2^31+1-ulp
dec_test!(bid128_to_uint32_xint_062, bid128_to_uint32_xint, 0x301069E10DE83A38EC4ECCEDA1000000u128, 2147483649   , 0x00); //  2^31+1
dec_test!(bid128_to_uint32_xint_063, bid128_to_uint32_xint, 0x301069E10DE83A38EC4ECCEDA1000001u128, 2147483649   , 0x20); //  2^31+1+ulp
dec_test!(bid128_to_uint32_xint_064, bid128_to_uint32_xint, 0x3010ab1629028504ffbff77ffaf77dffu128, 0xCED4BA09   , 0x20);
dec_test!(bid128_to_uint32_xint_065, bid128_to_uint32_xint, 0x3010C5371912364CE3056C27FFFFFFFFu128, 3999999999   , 0x20); //  4e9-ulp
dec_test!(bid128_to_uint32_xint_066, bid128_to_uint32_xint, 0x3010C5371912364CE3056C2800000000u128, 4000000000   , 0x00); //  4e9
dec_test!(bid128_to_uint32_xint_067, bid128_to_uint32_xint, 0x3010C5371912364CE3056C2800000001u128, 4000000000   , 0x20); //  4e9+ulp
dec_test!(bid128_to_uint32_xint_068, bid128_to_uint32_xint, 0x3010D3C21BCDF92B853133125EFFFFFFu128, 4294967294   , 0x20); //  2^32-1-ulp
dec_test!(bid128_to_uint32_xint_069, bid128_to_uint32_xint, 0x3010D3C21BCDF92B853133125F000000u128, 4294967295   , 0x00); //  2^32-1
dec_test!(bid128_to_uint32_xint_070, bid128_to_uint32_xint, 0x3010D3C21BCDF92B853133125F000001u128, 4294967295   , 0x20); //  2^32-1+ulp
dec_test!(bid128_to_uint32_xint_071, bid128_to_uint32_xint, 0x3010D3C21BCE630C931899892F7FFFFFu128, 4294967295   , 0x20); //  2^32-0.5-ulp
dec_test!(bid128_to_uint32_xint_072, bid128_to_uint32_xint, 0x3010D3C21BCE630C931899892F800000u128, 4294967295   , 0x20); //  2^32-0.5
dec_test!(bid128_to_uint32_xint_073, bid128_to_uint32_xint, 0x3010D3C21BCE630C931899892F800001u128, 4294967295   , 0x20); //  2^32-0.5+ulp
dec_test!(bid128_to_uint32_xint_074, bid128_to_uint32_xint, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 4294967295   , 0x20); //  2^32-ulp
dec_test!(bid128_to_uint32_xint_075, bid128_to_uint32_xint, 0x3010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); //  2^32
dec_test!(bid128_to_uint32_xint_076, bid128_to_uint32_xint, 0x3010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); //  2^32+ulp
dec_test!(bid128_to_uint32_xint_077, bid128_to_uint32_xint, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); //  2^32+0.5-ulp
dec_test!(bid128_to_uint32_xint_078, bid128_to_uint32_xint, 0x3010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); //  2^32+0.5
dec_test!(bid128_to_uint32_xint_079, bid128_to_uint32_xint, 0x3010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); //  2^32+0.5+ulp
dec_test!(bid128_to_uint32_xint_080, bid128_to_uint32_xint, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); //  2^32+1-ulp
dec_test!(bid128_to_uint32_xint_081, bid128_to_uint32_xint, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); //  2^32+1
dec_test!(bid128_to_uint32_xint_082, bid128_to_uint32_xint, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); //  2^32+1+ulp
dec_test!(bid128_to_uint32_xint_083, bid128_to_uint32_xint, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); //  5e9-ulp
dec_test!(bid128_to_uint32_xint_084, bid128_to_uint32_xint, 0x3010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); //  5e9
dec_test!(bid128_to_uint32_xint_085, bid128_to_uint32_xint, 0x3010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); //  5e9+ulp
dec_test!(bid128_to_uint32_xint_086, bid128_to_uint32_xint, 0x3011d88100b80253f7f5cde1ff85fcfeu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_087, bid128_to_uint32_xint, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); //  2e10-1.5-ulp
dec_test!(bid128_to_uint32_xint_088, bid128_to_uint32_xint, 0x3012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); //  2e10-1.5
dec_test!(bid128_to_uint32_xint_089, bid128_to_uint32_xint, 0x3012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); //  2e10-1.5+ulp
dec_test!(bid128_to_uint32_xint_090, bid128_to_uint32_xint, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); //  2e10-1-ulp
dec_test!(bid128_to_uint32_xint_091, bid128_to_uint32_xint, 0x3012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); //  2e10-1
dec_test!(bid128_to_uint32_xint_092, bid128_to_uint32_xint, 0x3012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); //  2e10-1+ulp
dec_test!(bid128_to_uint32_xint_093, bid128_to_uint32_xint, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); //  2e10-0.5-ulp
dec_test!(bid128_to_uint32_xint_094, bid128_to_uint32_xint, 0x3012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); //  2e10-0.5
dec_test!(bid128_to_uint32_xint_095, bid128_to_uint32_xint, 0x3012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); //  2e10-0.5+ulp
dec_test!(bid128_to_uint32_xint_096, bid128_to_uint32_xint, 0x3012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); //  2e10-ulp
dec_test!(bid128_to_uint32_xint_097, bid128_to_uint32_xint, 0x3012629B8C891B267182B61400000000u128, 2147483648   , 0x01); //  2e10
dec_test!(bid128_to_uint32_xint_098, bid128_to_uint32_xint, 0x3012629B8C891B267182B61400000001u128, 2147483648   , 0x01); //  2e10+ulp
dec_test!(bid128_to_uint32_xint_099, bid128_to_uint32_xint, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); //  2e10+0.5-ulp
dec_test!(bid128_to_uint32_xint_100, bid128_to_uint32_xint, 0x3012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); //  2e10+0.5
dec_test!(bid128_to_uint32_xint_101, bid128_to_uint32_xint, 0x3012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); //  2e10+0.5+ulp
dec_test!(bid128_to_uint32_xint_102, bid128_to_uint32_xint, 0x3012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); //  2e10+1-ulp
dec_test!(bid128_to_uint32_xint_103, bid128_to_uint32_xint, 0x3012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); //  2e10+1
dec_test!(bid128_to_uint32_xint_104, bid128_to_uint32_xint, 0x3012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); //  2e10+1+ulp
dec_test!(bid128_to_uint32_xint_105, bid128_to_uint32_xint, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); //  2e10+1.5-ulp
dec_test!(bid128_to_uint32_xint_106, bid128_to_uint32_xint, 0x3012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); //  2e10+1.5
dec_test!(bid128_to_uint32_xint_107, bid128_to_uint32_xint, 0x3012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); //  2e10+1.5+ulp
dec_test!(bid128_to_uint32_xint_108, bid128_to_uint32_xint, 0x301600000000003627E8F712373BFFFFu128, 0            , 0x20); //  0.999-ulp
dec_test!(bid128_to_uint32_xint_109, bid128_to_uint32_xint, 0x301600000000003627E8F712373C0000u128, 0            , 0x20); //  0.999
dec_test!(bid128_to_uint32_xint_110, bid128_to_uint32_xint, 0x301600000000003627E8F712373C0001u128, 0            , 0x20); //  0.999+ulp
dec_test!(bid128_to_uint32_xint_111, bid128_to_uint32_xint, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646   , 0x20); //  2^31-1.5-ulp
dec_test!(bid128_to_uint32_xint_112, bid128_to_uint32_xint, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483646   , 0x20); //  2^31-1.5
dec_test!(bid128_to_uint32_xint_113, bid128_to_uint32_xint, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483646   , 0x20); //  2^31-1.5+ulp
dec_test!(bid128_to_uint32_xint_114, bid128_to_uint32_xint, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647   , 0x20); //  2^31-0.5-ulp
dec_test!(bid128_to_uint32_xint_115, bid128_to_uint32_xint, 0x30180002B5E3AF13FBA450E94E780000u128, 2147483647   , 0x20); //  2^31-0.5
dec_test!(bid128_to_uint32_xint_116, bid128_to_uint32_xint, 0x30180002B5E3AF13FBA450E94E780001u128, 2147483647   , 0x20); //  2^31-0.5+ulp
dec_test!(bid128_to_uint32_xint_117, bid128_to_uint32_xint, 0x30180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x20); //  2^31+0.5-ulp
dec_test!(bid128_to_uint32_xint_118, bid128_to_uint32_xint, 0x30180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x20); //  2^31+0.5
dec_test!(bid128_to_uint32_xint_119, bid128_to_uint32_xint, 0x30180002B5E3AF19676BAF16B1880001u128, 2147483648   , 0x20); //  2^31+0.5+ulp
dec_test!(bid128_to_uint32_xint_120, bid128_to_uint32_xint, 0x301800056BC75E2AAD2C50E94E77FFFFu128, 4294967295   , 0x20); //  2^32-0.5-ulp
dec_test!(bid128_to_uint32_xint_121, bid128_to_uint32_xint, 0x301800056BC75E2AAD2C50E94E780000u128, 4294967295   , 0x20); //  2^32-0.5
dec_test!(bid128_to_uint32_xint_122, bid128_to_uint32_xint, 0x301800056BC75E2AAD2C50E94E780001u128, 4294967295   , 0x20); //  2^32-0.5+ulp
dec_test!(bid128_to_uint32_xint_123, bid128_to_uint32_xint, 0x301800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); //  2^32+0.5-ulp
dec_test!(bid128_to_uint32_xint_124, bid128_to_uint32_xint, 0x301800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); //  2^32+0.5
dec_test!(bid128_to_uint32_xint_125, bid128_to_uint32_xint, 0x301800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); //  2^32+0.5+ulp
dec_test!(bid128_to_uint32_xint_126, bid128_to_uint32_xint, 0x301A0000000000004563918244F3FFFFu128, 0            , 0x20); //  0.5-ulp
dec_test!(bid128_to_uint32_xint_127, bid128_to_uint32_xint, 0x301A0000000000004563918244F40000u128, 0            , 0x20); //  0.5
dec_test!(bid128_to_uint32_xint_128, bid128_to_uint32_xint, 0x301A0000000000004563918244F40001u128, 0            , 0x20); //  0.5+ulp
dec_test!(bid128_to_uint32_xint_129, bid128_to_uint32_xint, 0x301A0000000000008AC7230489E7FFFFu128, 0            , 0x20); //  1-ulp
dec_test!(bid128_to_uint32_xint_130, bid128_to_uint32_xint, 0x301A0000000000008AC7230489E80000u128, 1            , 0x00); //  1
dec_test!(bid128_to_uint32_xint_131, bid128_to_uint32_xint, 0x301A0000000000008AC7230489E80001u128, 1            , 0x20); //  1+ulp
dec_test!(bid128_to_uint32_xint_132, bid128_to_uint32_xint, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300          , 0x20); //  300.5-ulp
dec_test!(bid128_to_uint32_xint_133, bid128_to_uint32_xint, 0x301A0000000000A2E6C09AD3E0D40000u128, 300          , 0x20); //  300.5
dec_test!(bid128_to_uint32_xint_134, bid128_to_uint32_xint, 0x301A0000000000A2E6C09AD3E0D40001u128, 300          , 0x20); //  300.5+ulp
dec_test!(bid128_to_uint32_xint_135, bid128_to_uint32_xint, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483646   , 0x20); //  2^31-1-ulp
dec_test!(bid128_to_uint32_xint_136, bid128_to_uint32_xint, 0x301A000045639181BA2CDCFB76180000u128, 2147483647   , 0x00); //  2^31-1
dec_test!(bid128_to_uint32_xint_137, bid128_to_uint32_xint, 0x301A000045639181BA2CDCFB76180001u128, 2147483647   , 0x20); //  2^31-1+ulp
dec_test!(bid128_to_uint32_xint_138, bid128_to_uint32_xint, 0x301A00004563918244F3FFFFFFFFFFFFu128, 2147483647   , 0x20); //  2^31-ulp
dec_test!(bid128_to_uint32_xint_139, bid128_to_uint32_xint, 0x301A00004563918244F4000000000000u128, 2147483648   , 0x00); //  2^31
dec_test!(bid128_to_uint32_xint_140, bid128_to_uint32_xint, 0x301A00004563918244F4000000000001u128, 2147483648   , 0x20); //  2^31+ulp
dec_test!(bid128_to_uint32_xint_141, bid128_to_uint32_xint, 0x301A000045639182CFBB230489E7FFFFu128, 2147483648   , 0x20); //  2^31+1-ulp
dec_test!(bid128_to_uint32_xint_142, bid128_to_uint32_xint, 0x301A000045639182CFBB230489E80000u128, 2147483649   , 0x00); //  2^31+1
dec_test!(bid128_to_uint32_xint_143, bid128_to_uint32_xint, 0x301A000045639182CFBB230489E80001u128, 2147483649   , 0x20); //  2^31+1+ulp
dec_test!(bid128_to_uint32_xint_144, bid128_to_uint32_xint, 0x301A00008AC72303FF20DCFB7617FFFFu128, 4294967294   , 0x20); //  2^32-1-ulp
dec_test!(bid128_to_uint32_xint_145, bid128_to_uint32_xint, 0x301A00008AC72303FF20DCFB76180000u128, 4294967295   , 0x00); //  2^32-1
dec_test!(bid128_to_uint32_xint_146, bid128_to_uint32_xint, 0x301A00008AC72303FF20DCFB76180001u128, 4294967295   , 0x20); //  2^32-1+ulp
dec_test!(bid128_to_uint32_xint_147, bid128_to_uint32_xint, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, 4294967295   , 0x20); //  2^32-ulp
dec_test!(bid128_to_uint32_xint_148, bid128_to_uint32_xint, 0x301A00008AC7230489E8000000000000u128, 2147483648   , 0x01); //  2^32
dec_test!(bid128_to_uint32_xint_149, bid128_to_uint32_xint, 0x301A00008AC7230489E8000000000001u128, 2147483648   , 0x01); //  2^32+ulp
dec_test!(bid128_to_uint32_xint_150, bid128_to_uint32_xint, 0x301A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); //  2^32+1-ulp
dec_test!(bid128_to_uint32_xint_151, bid128_to_uint32_xint, 0x301A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); //  2^32+1
dec_test!(bid128_to_uint32_xint_152, bid128_to_uint32_xint, 0x301A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); //  2^32+1+ulp
dec_test!(bid128_to_uint32_xint_153, bid128_to_uint32_xint, 0x301C00000000000014D1120D7B15FFFFu128, 1            , 0x20); //  1.5-ulp
dec_test!(bid128_to_uint32_xint_154, bid128_to_uint32_xint, 0x301C00000000000014D1120D7B160000u128, 1            , 0x20); //  1.5
dec_test!(bid128_to_uint32_xint_155, bid128_to_uint32_xint, 0x301C00000000000014D1120D7B160001u128, 1            , 0x20); //  1.5+ulp
dec_test!(bid128_to_uint32_xint_156, bid128_to_uint32_xint, 0x301E000000000001A055690D9DB7FFFFu128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_157, bid128_to_uint32_xint, 0x301E000000000001A055690D9DB80000u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_158, bid128_to_uint32_xint, 0x301E000000000001A055690D9DB80001u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_159, bid128_to_uint32_xint, 0x302000000000000029A2241AF62BFFFFu128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_160, bid128_to_uint32_xint, 0x302000000000000029A2241AF62C0000u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_161, bid128_to_uint32_xint, 0x302000000000000029A2241AF62C0001u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_162, bid128_to_uint32_xint, 0x30200000000000007fffbfffffffffe7u128, 922          , 0x20);
dec_test!(bid128_to_uint32_xint_163, bid128_to_uint32_xint, 0x3020000000108004fefffaf7fff5ffefu128, 1994736818   , 0x20);
dec_test!(bid128_to_uint32_xint_164, bid128_to_uint32_xint, 0x3024000000000000006A94D74F42FFFFu128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_165, bid128_to_uint32_xint, 0x3024000000000000006A94D74F430000u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_166, bid128_to_uint32_xint, 0x3024000000000000006A94D74F430001u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_167, bid128_to_uint32_xint, 0x302A00000000000000000017428106FFu128, 0            , 0x20); //  0.999-ulp
dec_test!(bid128_to_uint32_xint_168, bid128_to_uint32_xint, 0x302A0000000000000000001742810700u128, 0            , 0x20); //  0.999
dec_test!(bid128_to_uint32_xint_169, bid128_to_uint32_xint, 0x302A0000000000000000001742810701u128, 0            , 0x20); //  0.999+ulp
dec_test!(bid128_to_uint32_xint_170, bid128_to_uint32_xint, 0x302A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); //  2e10-1.5-ulp
dec_test!(bid128_to_uint32_xint_171, bid128_to_uint32_xint, 0x302A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); //  2e10-1.5
dec_test!(bid128_to_uint32_xint_172, bid128_to_uint32_xint, 0x302A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); //  2e10-1.5+ulp
dec_test!(bid128_to_uint32_xint_173, bid128_to_uint32_xint, 0x302A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); //  2e10-0.5-ulp
dec_test!(bid128_to_uint32_xint_174, bid128_to_uint32_xint, 0x302A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); //  2e10-0.5
dec_test!(bid128_to_uint32_xint_175, bid128_to_uint32_xint, 0x302A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); //  2e10-0.5+ulp
dec_test!(bid128_to_uint32_xint_176, bid128_to_uint32_xint, 0x302C000000000000000002BBA7F521FFu128, 300          , 0x20); //  300.5-ulp
dec_test!(bid128_to_uint32_xint_177, bid128_to_uint32_xint, 0x302C000000000000000002BBA7F52200u128, 300          , 0x20); //  300.5
dec_test!(bid128_to_uint32_xint_178, bid128_to_uint32_xint, 0x302C000000000000000002BBA7F52201u128, 300          , 0x20); //  300.5+ulp
dec_test!(bid128_to_uint32_xint_179, bid128_to_uint32_xint, 0x302C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); //  2e10-1-ulp
dec_test!(bid128_to_uint32_xint_180, bid128_to_uint32_xint, 0x302C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); //  2e10-1
dec_test!(bid128_to_uint32_xint_181, bid128_to_uint32_xint, 0x302C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); //  2e10-1+ulp
dec_test!(bid128_to_uint32_xint_182, bid128_to_uint32_xint, 0x302C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); //  2e10+0.5-ulp
dec_test!(bid128_to_uint32_xint_183, bid128_to_uint32_xint, 0x302C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); //  2e10+0.5
dec_test!(bid128_to_uint32_xint_184, bid128_to_uint32_xint, 0x302C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); //  2e10+0.5+ulp
dec_test!(bid128_to_uint32_xint_185, bid128_to_uint32_xint, 0x302C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); //  2e10+1.5-ulp
dec_test!(bid128_to_uint32_xint_186, bid128_to_uint32_xint, 0x302C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); //  2e10+1.5
dec_test!(bid128_to_uint32_xint_187, bid128_to_uint32_xint, 0x302C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); //  2e10+1.5+ulp
dec_test!(bid128_to_uint32_xint_188, bid128_to_uint32_xint, 0x302E000000000000000000001DCD64FFu128, 0            , 0x20); //  0.5-ulp
dec_test!(bid128_to_uint32_xint_189, bid128_to_uint32_xint, 0x302E000000000000000000001DCD6500u128, 0            , 0x20); //  0.5
dec_test!(bid128_to_uint32_xint_190, bid128_to_uint32_xint, 0x302E000000000000000000001DCD6501u128, 0            , 0x20); //  0.5+ulp
dec_test!(bid128_to_uint32_xint_191, bid128_to_uint32_xint, 0x302E000000000000000000003B9AC9FFu128, 0            , 0x20); //  1-ulp
dec_test!(bid128_to_uint32_xint_192, bid128_to_uint32_xint, 0x302E000000000000000000003B9ACA00u128, 1            , 0x00); //  1
dec_test!(bid128_to_uint32_xint_193, bid128_to_uint32_xint, 0x302E000000000000000000003B9ACA01u128, 1            , 0x20); //  1+ulp
dec_test!(bid128_to_uint32_xint_194, bid128_to_uint32_xint, 0x302E0000000000000000000059682EFFu128, 1            , 0x20); //  1.5-ulp
dec_test!(bid128_to_uint32_xint_195, bid128_to_uint32_xint, 0x302E0000000000000000000059682F00u128, 1            , 0x20); //  1.5
dec_test!(bid128_to_uint32_xint_196, bid128_to_uint32_xint, 0x302E0000000000000000000059682F01u128, 1            , 0x20); //  1.5+ulp
dec_test!(bid128_to_uint32_xint_197, bid128_to_uint32_xint, 0x302E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); //  2e10+1-ulp
dec_test!(bid128_to_uint32_xint_198, bid128_to_uint32_xint, 0x302E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); //  2e10+1
dec_test!(bid128_to_uint32_xint_199, bid128_to_uint32_xint, 0x302E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); //  2e10+1+ulp
dec_test!(bid128_to_uint32_xint_200, bid128_to_uint32_xint, 0x303000000000000000000006FC23ABFFu128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_201, bid128_to_uint32_xint, 0x303000000000000000000006FC23AC00u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_202, bid128_to_uint32_xint, 0x303000000000000000000006FC23AC01u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_203, bid128_to_uint32_xint, 0x303200000000000000000000B2D05DFFu128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_204, bid128_to_uint32_xint, 0x303200000000000000000000B2D05E00u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_205, bid128_to_uint32_xint, 0x303200000000000000000000B2D05E01u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_206, bid128_to_uint32_xint, 0x303800000000000000000000002DDA47u128, 300          , 0x20); //  300.5-ulp
dec_test!(bid128_to_uint32_xint_207, bid128_to_uint32_xint, 0x303800000000000000000000002DDA48u128, 300          , 0x20); //  300.5
dec_test!(bid128_to_uint32_xint_208, bid128_to_uint32_xint, 0x303800000000000000000000002DDA49u128, 300          , 0x20); //  300.5+ulp
dec_test!(bid128_to_uint32_xint_209, bid128_to_uint32_xint, 0x303A00000000000000000000000003E7u128, 0            , 0x20); //  0.999
dec_test!(bid128_to_uint32_xint_210, bid128_to_uint32_xint, 0x303A00000000000000000000000005DBu128, 1            , 0x20); //  1.5-ulp
dec_test!(bid128_to_uint32_xint_211, bid128_to_uint32_xint, 0x303A00000000000000000000000005DCu128, 1            , 0x20); //  1.5
dec_test!(bid128_to_uint32_xint_212, bid128_to_uint32_xint, 0x303A00000000000000000000000005DDu128, 1            , 0x20); //  1.5+ulp
dec_test!(bid128_to_uint32_xint_213, bid128_to_uint32_xint, 0x303A00000000000000000000000495D3u128, 300          , 0x20); //  300.5-ulp
dec_test!(bid128_to_uint32_xint_214, bid128_to_uint32_xint, 0x303A00000000000000000000000495D4u128, 300          , 0x20); //  300.5
dec_test!(bid128_to_uint32_xint_215, bid128_to_uint32_xint, 0x303A00000000000000000000000495D5u128, 300          , 0x20); //  300.5+ulp
dec_test!(bid128_to_uint32_xint_216, bid128_to_uint32_xint, 0x303C0000000000000000000000000095u128, 1            , 0x20); //  1.5-ulp
dec_test!(bid128_to_uint32_xint_217, bid128_to_uint32_xint, 0x303C0000000000000000000000000096u128, 1            , 0x20); //  1.5
dec_test!(bid128_to_uint32_xint_218, bid128_to_uint32_xint, 0x303C0000000000000000000000000097u128, 1            , 0x20); //  1.5+ulp
dec_test!(bid128_to_uint32_xint_219, bid128_to_uint32_xint, 0x303C0000000000000000000000007561u128, 300          , 0x20); //  300.5-ulp
dec_test!(bid128_to_uint32_xint_220, bid128_to_uint32_xint, 0x303C0000000000000000000000007562u128, 300          , 0x20); //  300.5
dec_test!(bid128_to_uint32_xint_221, bid128_to_uint32_xint, 0x303C0000000000000000000000007563u128, 300          , 0x20); //  300.5+ulp
dec_test!(bid128_to_uint32_xint_222, bid128_to_uint32_xint, 0x303C00000000000000000031FFFFFF69u128, 2147483646   , 0x20); //  2^31-1.5-ulp
dec_test!(bid128_to_uint32_xint_223, bid128_to_uint32_xint, 0x303C00000000000000000031FFFFFF6Au128, 2147483646   , 0x20); //  2^31-1.5
dec_test!(bid128_to_uint32_xint_224, bid128_to_uint32_xint, 0x303C00000000000000000031FFFFFF6Bu128, 2147483646   , 0x20); //  2^31-1.5+ulp
dec_test!(bid128_to_uint32_xint_225, bid128_to_uint32_xint, 0x303C00000000000000000031FFFFFFCDu128, 2147483647   , 0x20); //  2^31-0.5-ulp
dec_test!(bid128_to_uint32_xint_226, bid128_to_uint32_xint, 0x303C00000000000000000031FFFFFFCEu128, 2147483647   , 0x20); //  2^31-0.5
dec_test!(bid128_to_uint32_xint_227, bid128_to_uint32_xint, 0x303C00000000000000000031FFFFFFCFu128, 2147483647   , 0x20); //  2^31-0.5+ulp
dec_test!(bid128_to_uint32_xint_228, bid128_to_uint32_xint, 0x303C0000000000000000003200000031u128, 2147483648   , 0x20); //  2^31+0.5-ulp
dec_test!(bid128_to_uint32_xint_229, bid128_to_uint32_xint, 0x303C0000000000000000003200000032u128, 2147483648   , 0x20); //  2^31+0.5
dec_test!(bid128_to_uint32_xint_230, bid128_to_uint32_xint, 0x303C0000000000000000003200000033u128, 2147483648   , 0x20); //  2^31+0.5+ulp
dec_test!(bid128_to_uint32_xint_231, bid128_to_uint32_xint, 0x303C00000000000000000063FFFFFFCDu128, 4294967295   , 0x20); //  2^32-0.5-ulp
dec_test!(bid128_to_uint32_xint_232, bid128_to_uint32_xint, 0x303C00000000000000000063FFFFFFCEu128, 4294967295   , 0x20); //  2^32-0.5
dec_test!(bid128_to_uint32_xint_233, bid128_to_uint32_xint, 0x303C00000000000000000063FFFFFFCFu128, 4294967295   , 0x20); //  2^32-0.5+ulp
dec_test!(bid128_to_uint32_xint_234, bid128_to_uint32_xint, 0x303C0000000000000000006400000031u128, 2147483648   , 0x01); //  2^32+0.5-ulp
dec_test!(bid128_to_uint32_xint_235, bid128_to_uint32_xint, 0x303C0000000000000000006400000032u128, 2147483648   , 0x01); //  2^32+0.5
dec_test!(bid128_to_uint32_xint_236, bid128_to_uint32_xint, 0x303C0000000000000000006400000033u128, 2147483648   , 0x01); //  2^32+0.5+ulp
dec_test!(bid128_to_uint32_xint_237, bid128_to_uint32_xint, 0x303E0000000000000000000000000005u128, 0            , 0x20); //  0.5
dec_test!(bid128_to_uint32_xint_238, bid128_to_uint32_xint, 0x303E000000000000000000000000000Fu128, 1            , 0x20); //  1.5
dec_test!(bid128_to_uint32_xint_239, bid128_to_uint32_xint, 0x303E0000000000000000000000000BB7u128, 299          , 0x20); //  300-ulp
dec_test!(bid128_to_uint32_xint_240, bid128_to_uint32_xint, 0x303E0000000000000000000000000BB8u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_241, bid128_to_uint32_xint, 0x303E0000000000000000000000000BB9u128, 300          , 0x20); //  300+ulp
dec_test!(bid128_to_uint32_xint_242, bid128_to_uint32_xint, 0x303E0000000000000000000000000BBDu128, 300          , 0x20); //  300.5
dec_test!(bid128_to_uint32_xint_243, bid128_to_uint32_xint, 0x303E00000000000000000004FFFFFFF1u128, 2147483646   , 0x20); //  2^31-1.5
dec_test!(bid128_to_uint32_xint_244, bid128_to_uint32_xint, 0x303E00000000000000000004FFFFFFF5u128, 2147483646   , 0x20); //  2^31-1-ulp
dec_test!(bid128_to_uint32_xint_245, bid128_to_uint32_xint, 0x303E00000000000000000004FFFFFFF6u128, 2147483647   , 0x00); //  2^31-1
dec_test!(bid128_to_uint32_xint_246, bid128_to_uint32_xint, 0x303E00000000000000000004FFFFFFF7u128, 2147483647   , 0x20); //  2^31-1+ulp
dec_test!(bid128_to_uint32_xint_247, bid128_to_uint32_xint, 0x303E00000000000000000004FFFFFFFBu128, 2147483647   , 0x20); //  2^31-0.5
dec_test!(bid128_to_uint32_xint_248, bid128_to_uint32_xint, 0x303E00000000000000000004FFFFFFFFu128, 2147483647   , 0x20); //  2^31-ulp
dec_test!(bid128_to_uint32_xint_249, bid128_to_uint32_xint, 0x303E0000000000000000000500000000u128, 2147483648   , 0x00); //  2^31
dec_test!(bid128_to_uint32_xint_250, bid128_to_uint32_xint, 0x303E0000000000000000000500000001u128, 2147483648   , 0x20); //  2^31+ulp
dec_test!(bid128_to_uint32_xint_251, bid128_to_uint32_xint, 0x303E0000000000000000000500000005u128, 2147483648   , 0x20); //  2^31+0.5
dec_test!(bid128_to_uint32_xint_252, bid128_to_uint32_xint, 0x303E0000000000000000000500000009u128, 2147483648   , 0x20); //  2^31+1-ulp
dec_test!(bid128_to_uint32_xint_253, bid128_to_uint32_xint, 0x303E000000000000000000050000000Au128, 2147483649   , 0x00); //  2^31+1
dec_test!(bid128_to_uint32_xint_254, bid128_to_uint32_xint, 0x303E000000000000000000050000000Bu128, 2147483649   , 0x20); //  2^31+1+ulp
dec_test!(bid128_to_uint32_xint_255, bid128_to_uint32_xint, 0x303E00000000000000000009FFFFFFF5u128, 4294967294   , 0x20); //  2^32-1-ulp
dec_test!(bid128_to_uint32_xint_256, bid128_to_uint32_xint, 0x303E00000000000000000009FFFFFFF6u128, 4294967295   , 0x00); //  2^32-1
dec_test!(bid128_to_uint32_xint_257, bid128_to_uint32_xint, 0x303E00000000000000000009FFFFFFF7u128, 4294967295   , 0x20); //  2^32-1+ulp
dec_test!(bid128_to_uint32_xint_258, bid128_to_uint32_xint, 0x303E00000000000000000009FFFFFFFBu128, 4294967295   , 0x20); //  2^32-0.5
dec_test!(bid128_to_uint32_xint_259, bid128_to_uint32_xint, 0x303E00000000000000000009FFFFFFFFu128, 4294967295   , 0x20); //  2^32-ulp
dec_test!(bid128_to_uint32_xint_260, bid128_to_uint32_xint, 0x303E0000000000000000000A00000000u128, 2147483648   , 0x01); //  2^32
dec_test!(bid128_to_uint32_xint_261, bid128_to_uint32_xint, 0x303E0000000000000000000A00000001u128, 2147483648   , 0x01); //  2^32+ulp
dec_test!(bid128_to_uint32_xint_262, bid128_to_uint32_xint, 0x303E0000000000000000000A00000005u128, 2147483648   , 0x01); //  2^32+0.5
dec_test!(bid128_to_uint32_xint_263, bid128_to_uint32_xint, 0x303E0000000000000000000A00000009u128, 2147483648   , 0x01); //  2^32+1-ulp
dec_test!(bid128_to_uint32_xint_264, bid128_to_uint32_xint, 0x303E0000000000000000000A0000000Au128, 2147483648   , 0x01); //  2^32+1
dec_test!(bid128_to_uint32_xint_265, bid128_to_uint32_xint, 0x303E0000000000000000000A0000000Bu128, 2147483648   , 0x01); //  2^32+1+ulp
dec_test!(bid128_to_uint32_xint_266, bid128_to_uint32_xint, 0x303E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); //  2e10-1.5
dec_test!(bid128_to_uint32_xint_267, bid128_to_uint32_xint, 0x303E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); //  2e10-0.5
dec_test!(bid128_to_uint32_xint_268, bid128_to_uint32_xint, 0x303E0000000000000000002E90EDD005u128, 2147483648   , 0x01); //  2e10+0.5
dec_test!(bid128_to_uint32_xint_269, bid128_to_uint32_xint, 0x303E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); //  2e10+1.5
dec_test!(bid128_to_uint32_xint_270, bid128_to_uint32_xint, 0x30400000000000000000000000000001u128, 1            , 0x00); //  1
dec_test!(bid128_to_uint32_xint_271, bid128_to_uint32_xint, 0x3040000000000000000000000000012Bu128, 299          , 0x00); //  300-ulp
dec_test!(bid128_to_uint32_xint_272, bid128_to_uint32_xint, 0x3040000000000000000000000000012Cu128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_273, bid128_to_uint32_xint, 0x3040000000000000000000000000012Du128, 301          , 0x00); //  300+ulp
dec_test!(bid128_to_uint32_xint_274, bid128_to_uint32_xint, 0x3040000000000000000000007FFFFFFFu128, 2147483647   , 0x00); //  2^31-1
dec_test!(bid128_to_uint32_xint_275, bid128_to_uint32_xint, 0x30400000000000000000000080000000u128, 2147483648   , 0x00); //  2^31
dec_test!(bid128_to_uint32_xint_276, bid128_to_uint32_xint, 0x30400000000000000000000080000001u128, 2147483649   , 0x00); //  2^31+1
dec_test!(bid128_to_uint32_xint_277, bid128_to_uint32_xint, 0x304000000000000000000000FFFFFFFFu128, 4294967295   , 0x00); //  2^32-1
dec_test!(bid128_to_uint32_xint_278, bid128_to_uint32_xint, 0x30400000000000000000000100000000u128, 2147483648   , 0x01); //  2^32
dec_test!(bid128_to_uint32_xint_279, bid128_to_uint32_xint, 0x30400000000000000000000100000001u128, 2147483648   , 0x01); //  2^32+1
dec_test!(bid128_to_uint32_xint_280, bid128_to_uint32_xint, 0x304000000000000000000004A817C7FFu128, 2147483648   , 0x01); //  2e10-1
dec_test!(bid128_to_uint32_xint_281, bid128_to_uint32_xint, 0x304000000000000000000004A817C801u128, 2147483648   , 0x01); //  2e10+1
dec_test!(bid128_to_uint32_xint_282, bid128_to_uint32_xint, 0x3041ED09BEAD87C0378D8E6400000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xint_283, bid128_to_uint32_xint, 0x3042000000000000000000000000001Du128, 290          , 0x00); //  300-ulp
dec_test!(bid128_to_uint32_xint_284, bid128_to_uint32_xint, 0x3042000000000000000000000000001Eu128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_285, bid128_to_uint32_xint, 0x3042000000000000000000000000001Fu128, 310          , 0x00); //  300+ulp
dec_test!(bid128_to_uint32_xint_286, bid128_to_uint32_xint, 0x304200000000000000000000773593FFu128, 2147483648   , 0x01); //  2e10-ulp
dec_test!(bid128_to_uint32_xint_287, bid128_to_uint32_xint, 0x30420000000000000000000077359400u128, 2147483648   , 0x01); //  2e10
dec_test!(bid128_to_uint32_xint_288, bid128_to_uint32_xint, 0x30420000000000000000000077359401u128, 2147483648   , 0x01); //  2e10+ulp
dec_test!(bid128_to_uint32_xint_289, bid128_to_uint32_xint, 0x30440000000000000000000000000003u128, 300          , 0x00); //  300
dec_test!(bid128_to_uint32_xint_290, bid128_to_uint32_xint, 0x30520000000000000000000000000004u128, 4000000000   , 0x00); //  4e9
dec_test!(bid128_to_uint32_xint_291, bid128_to_uint32_xint, 0x30520000000000000000000000000005u128, 2147483648   , 0x01); //  5e9
dec_test!(bid128_to_uint32_xint_292, bid128_to_uint32_xint, 0x30540000000000000000000000000002u128, 2147483648   , 0x01); //  2e10
dec_test!(bid128_to_uint32_xint_293, bid128_to_uint32_xint, 0x31c00000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xint_294, bid128_to_uint32_xint, 0x36fe0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xint_295, bid128_to_uint32_xint, 4294967296u64                         , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_296, bid128_to_uint32_xint, 0x438f05162c6d6d4c6bf427ae950bce88u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_297, bid128_to_uint32_xint, 0x43ebb89d88797a9d62cb7c75ab88bae1u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_298, bid128_to_uint32_xint, "5.05"                                , 5            , 0x20);
dec_test!(bid128_to_uint32_xint_299, bid128_to_uint32_xint, "5.5"                                 , 5            , 0x20);
dec_test!(bid128_to_uint32_xint_300, bid128_to_uint32_xint, "+58867.98687879696999E0"             , 58867        , 0x20);
dec_test!(bid128_to_uint32_xint_301, bid128_to_uint32_xint, 0x597ad56ddb577840b0a46c4e01b0c82au128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_302, bid128_to_uint32_xint, "+658.5E0"                            , 658          , 0x20);
dec_test!(bid128_to_uint32_xint_303, bid128_to_uint32_xint, "+6.8E0"                              , 6            , 0x20);
dec_test!(bid128_to_uint32_xint_304, bid128_to_uint32_xint, 0x71198fa75ea3a4e6af28c490c5a27622u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xint_305, bid128_to_uint32_xint, 0x78000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xint_306, bid128_to_uint32_xint, 0x7bfffb2fe79a9d16102206021809062au128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_307, bid128_to_uint32_xint, 0x7c000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xint_308, bid128_to_uint32_xint, 0x7c003fffffffffff38c15b08ffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_309, bid128_to_uint32_xint, 0x7c003fffffffffff38c15b0affffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_310, bid128_to_uint32_xint, 0x7e000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xint_311, bid128_to_uint32_xint, "+856696.5568779E0"                   , 856696       , 0x20);
dec_test!(bid128_to_uint32_xint_312, bid128_to_uint32_xint, "+8898888.998988E0"                   , 8898888      , 0x20);
dec_test!(bid128_to_uint32_xint_313, bid128_to_uint32_xint, 0x8fbe0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xint_314, bid128_to_uint32_xint, 0x918aabf124f3341d7b8d0a6f0811905cu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xint_315, bid128_to_uint32_xint, 0x93ce0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xint_316, bid128_to_uint32_xint, "+98989.99889889888E0"                , 98989        , 0x20);
dec_test!(bid128_to_uint32_xint_317, bid128_to_uint32_xint, 0xa571e9d766268215e433418e42edf5e0u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xint_318, bid128_to_uint32_xint, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x20); //  -(0.5-ulp)
dec_test!(bid128_to_uint32_xint_319, bid128_to_uint32_xint, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0            , 0x20); //  -(0.5)
dec_test!(bid128_to_uint32_xint_320, bid128_to_uint32_xint, 0xAFFCF684DF56C3E01BC6C73200000001u128, 0            , 0x20); //  -(0.5+ulp)
dec_test!(bid128_to_uint32_xint_321, bid128_to_uint32_xint, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 0            , 0x20); //  -(0.999-ulp)
dec_test!(bid128_to_uint32_xint_322, bid128_to_uint32_xint, 0xAFFDEC8B86EF679D76FC433D80000000u128, 0            , 0x20); //  -(0.999)
dec_test!(bid128_to_uint32_xint_323, bid128_to_uint32_xint, 0xAFFDEC8B86EF679D76FC433D80000001u128, 0            , 0x20); //  -(0.999+ulp)
dec_test!(bid128_to_uint32_xint_324, bid128_to_uint32_xint, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 0            , 0x20); //  -(1-ulp)
dec_test!(bid128_to_uint32_xint_325, bid128_to_uint32_xint, 0xAFFE314DC6448D9338C15B0A00000000u128, 2147483648   , 0x01); //  -(1)
dec_test!(bid128_to_uint32_xint_326, bid128_to_uint32_xint, 0xAFFE314DC6448D9338C15B0A00000001u128, 2147483648   , 0x01); //  -(1+ulp)
dec_test!(bid128_to_uint32_xint_327, bid128_to_uint32_xint, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, 2147483648   , 0x01); //  -(1.5-ulp)
dec_test!(bid128_to_uint32_xint_328, bid128_to_uint32_xint, 0xAFFE49F4A966D45CD522088F00000000u128, 2147483648   , 0x01); //  -(1.5)
dec_test!(bid128_to_uint32_xint_329, bid128_to_uint32_xint, 0xAFFE49F4A966D45CD522088F00000001u128, 2147483648   , 0x01); //  -(1.5+ulp)
dec_test!(bid128_to_uint32_xint_330, bid128_to_uint32_xint, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_331, bid128_to_uint32_xint, 0xB00293E952CDA8B9AA44111E00000000u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_332, bid128_to_uint32_xint, 0xB00293E952CDA8B9AA44111E00000001u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_333, bid128_to_uint32_xint, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, 2147483648   , 0x01); //  -(300.5-ulp)
dec_test!(bid128_to_uint32_xint_334, bid128_to_uint32_xint, 0xB00294286EACB8CB0A8CB6B140000000u128, 2147483648   , 0x01); //  -(300.5)
dec_test!(bid128_to_uint32_xint_335, bid128_to_uint32_xint, 0xB00294286EACB8CB0A8CB6B140000001u128, 2147483648   , 0x01); //  -(300.5+ulp)
dec_test!(bid128_to_uint32_xint_336, bid128_to_uint32_xint, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_337, bid128_to_uint32_xint, 0xB0040ECA8847C4129106CE8300000000u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_338, bid128_to_uint32_xint, 0xB0040ECA8847C4129106CE8300000001u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_339, bid128_to_uint32_xint, 0xb0062020504002282678d0de6951d884u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_340, bid128_to_uint32_xint, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_341, bid128_to_uint32_xint, 0xB00A0003C95A2F0B4856475FE0000000u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_342, bid128_to_uint32_xint, 0xB00A0003C95A2F0B4856475FE0000001u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_343, bid128_to_uint32_xint, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_344, bid128_to_uint32_xint, 0xB00C000060EF6B1ABA6F072330000000u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_345, bid128_to_uint32_xint, 0xB00C000060EF6B1ABA6F072330000001u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_346, bid128_to_uint32_xint, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483648   , 0x01); //  -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xint_347, bid128_to_uint32_xint, 0xB01069E10DE628D3A6C9CC9B8E800000u128, 2147483648   , 0x01); //  -(2^31-1.5)
dec_test!(bid128_to_uint32_xint_348, bid128_to_uint32_xint, 0xB01069E10DE628D3A6C9CC9B8E800001u128, 2147483648   , 0x01); //  -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xint_349, bid128_to_uint32_xint, 0xB01069E10DE692B4B4B133125EFFFFFFu128, 2147483648   , 0x01); //  -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xint_350, bid128_to_uint32_xint, 0xB01069E10DE692B4B4B133125F000000u128, 2147483648   , 0x01); //  -(2^31-1)
dec_test!(bid128_to_uint32_xint_351, bid128_to_uint32_xint, 0xB01069E10DE692B4B4B133125F000001u128, 2147483648   , 0x01); //  -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xint_352, bid128_to_uint32_xint, 0xB01069E10DE6FC95C29899892F7FFFFFu128, 2147483648   , 0x01); //  -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xint_353, bid128_to_uint32_xint, 0xB01069E10DE6FC95C29899892F800000u128, 2147483648   , 0x01); //  -(2^31-0.5)
dec_test!(bid128_to_uint32_xint_354, bid128_to_uint32_xint, 0xB01069E10DE6FC95C29899892F800001u128, 2147483648   , 0x01); //  -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xint_355, bid128_to_uint32_xint, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x01); //  -(2^31-ulp)
dec_test!(bid128_to_uint32_xint_356, bid128_to_uint32_xint, 0xB01069E10DE76676D080000000000000u128, 2147483648   , 0x01); //  -(2^31)
dec_test!(bid128_to_uint32_xint_357, bid128_to_uint32_xint, 0xB01069E10DE76676D080000000000001u128, 2147483648   , 0x01); //  -(2^31+ulp)
dec_test!(bid128_to_uint32_xint_358, bid128_to_uint32_xint, 0xB01069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x01); //  -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xint_359, bid128_to_uint32_xint, 0xB01069E10DE7D057DE676676D0800000u128, 2147483648   , 0x01); //  -(2^31+0.5)
dec_test!(bid128_to_uint32_xint_360, bid128_to_uint32_xint, 0xB01069E10DE7D057DE676676D0800001u128, 2147483648   , 0x01); //  -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xint_361, bid128_to_uint32_xint, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483648   , 0x01); //  -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xint_362, bid128_to_uint32_xint, 0xB01069E10DE83A38EC4ECCEDA1000000u128, 2147483648   , 0x01); //  -(2^31+1)
dec_test!(bid128_to_uint32_xint_363, bid128_to_uint32_xint, 0xB01069E10DE83A38EC4ECCEDA1000001u128, 2147483648   , 0x01); //  -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xint_364, bid128_to_uint32_xint, 0xB010C5371912364CE3056C27FFFFFFFFu128, 2147483648   , 0x01); //  -(4e9-ulp)
dec_test!(bid128_to_uint32_xint_365, bid128_to_uint32_xint, 0xB010C5371912364CE3056C2800000000u128, 2147483648   , 0x01); //  -(4e9)
dec_test!(bid128_to_uint32_xint_366, bid128_to_uint32_xint, 0xB010C5371912364CE3056C2800000001u128, 2147483648   , 0x01); //  -(4e9+ulp)
dec_test!(bid128_to_uint32_xint_367, bid128_to_uint32_xint, 0xB010D3C21BCDF92B853133125EFFFFFFu128, 2147483648   , 0x01); //  -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xint_368, bid128_to_uint32_xint, 0xB010D3C21BCDF92B853133125F000000u128, 2147483648   , 0x01); //  -(2^32-1)
dec_test!(bid128_to_uint32_xint_369, bid128_to_uint32_xint, 0xB010D3C21BCDF92B853133125F000001u128, 2147483648   , 0x01); //  -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xint_370, bid128_to_uint32_xint, 0xB010D3C21BCE630C931899892F7FFFFFu128, 2147483648   , 0x01); //  -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xint_371, bid128_to_uint32_xint, 0xB010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01); //  -(2^32-0.5)
dec_test!(bid128_to_uint32_xint_372, bid128_to_uint32_xint, 0xB010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01); //  -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xint_373, bid128_to_uint32_xint, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01); //  -(2^32-ulp)
dec_test!(bid128_to_uint32_xint_374, bid128_to_uint32_xint, 0xB010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); //  -(2^32)
dec_test!(bid128_to_uint32_xint_375, bid128_to_uint32_xint, 0xB010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); //  -(2^32+ulp)
dec_test!(bid128_to_uint32_xint_376, bid128_to_uint32_xint, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); //  -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xint_377, bid128_to_uint32_xint, 0xB010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); //  -(2^32+0.5)
dec_test!(bid128_to_uint32_xint_378, bid128_to_uint32_xint, 0xB010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); //  -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xint_379, bid128_to_uint32_xint, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); //  -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xint_380, bid128_to_uint32_xint, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); //  -(2^32+1)
dec_test!(bid128_to_uint32_xint_381, bid128_to_uint32_xint, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); //  -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xint_382, bid128_to_uint32_xint, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); //  -(5e9-ulp)
dec_test!(bid128_to_uint32_xint_383, bid128_to_uint32_xint, 0xB010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); //  -(5e9)
dec_test!(bid128_to_uint32_xint_384, bid128_to_uint32_xint, 0xB010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); //  -(5e9+ulp)
dec_test!(bid128_to_uint32_xint_385, bid128_to_uint32_xint, 0xb01129d7f720598e53405deb29db5098u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_386, bid128_to_uint32_xint, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); //  -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_xint_387, bid128_to_uint32_xint, 0xB012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); //  -(2e10-1.5)
dec_test!(bid128_to_uint32_xint_388, bid128_to_uint32_xint, 0xB012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); //  -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_xint_389, bid128_to_uint32_xint, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); //  -(2e10-1-ulp)
dec_test!(bid128_to_uint32_xint_390, bid128_to_uint32_xint, 0xB012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); //  -(2e10-1)
dec_test!(bid128_to_uint32_xint_391, bid128_to_uint32_xint, 0xB012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); //  -(2e10-1+ulp)
dec_test!(bid128_to_uint32_xint_392, bid128_to_uint32_xint, 0xB012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); //  -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_xint_393, bid128_to_uint32_xint, 0xB012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); //  -(2e10-0.5)
dec_test!(bid128_to_uint32_xint_394, bid128_to_uint32_xint, 0xB012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); //  -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_xint_395, bid128_to_uint32_xint, 0xB012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); //  -(2e10-ulp)
dec_test!(bid128_to_uint32_xint_396, bid128_to_uint32_xint, 0xB012629B8C891B267182B61400000000u128, 2147483648   , 0x01); //  -(2e10)
dec_test!(bid128_to_uint32_xint_397, bid128_to_uint32_xint, 0xB012629B8C891B267182B61400000001u128, 2147483648   , 0x01); //  -(2e10+ulp)
dec_test!(bid128_to_uint32_xint_398, bid128_to_uint32_xint, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); //  -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_xint_399, bid128_to_uint32_xint, 0xB012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); //  -(2e10+0.5)
dec_test!(bid128_to_uint32_xint_400, bid128_to_uint32_xint, 0xB012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); //  -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_xint_401, bid128_to_uint32_xint, 0xB012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); //  -(2e10+1-ulp)
dec_test!(bid128_to_uint32_xint_402, bid128_to_uint32_xint, 0xB012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); //  -(2e10+1)
dec_test!(bid128_to_uint32_xint_403, bid128_to_uint32_xint, 0xB012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); //  -(2e10+1+ulp)
dec_test!(bid128_to_uint32_xint_404, bid128_to_uint32_xint, 0xB012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); //  -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_xint_405, bid128_to_uint32_xint, 0xB012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); //  -(2e10+1.5)
dec_test!(bid128_to_uint32_xint_406, bid128_to_uint32_xint, 0xB012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); //  -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_xint_407, bid128_to_uint32_xint, 0xB01600000000003627E8F712373BFFFFu128, 0            , 0x20); //  -(0.999-ulp)
dec_test!(bid128_to_uint32_xint_408, bid128_to_uint32_xint, 0xB01600000000003627E8F712373C0000u128, 0            , 0x20); //  -(0.999)
dec_test!(bid128_to_uint32_xint_409, bid128_to_uint32_xint, 0xB01600000000003627E8F712373C0001u128, 0            , 0x20); //  -(0.999+ulp)
dec_test!(bid128_to_uint32_xint_410, bid128_to_uint32_xint, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483648   , 0x01); //  -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xint_411, bid128_to_uint32_xint, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, 2147483648   , 0x01); //  -(2^31-1.5)
dec_test!(bid128_to_uint32_xint_412, bid128_to_uint32_xint, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, 2147483648   , 0x01); //  -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xint_413, bid128_to_uint32_xint, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, 2147483648   , 0x01); //  -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xint_414, bid128_to_uint32_xint, 0xB0180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x01); //  -(2^31-0.5)
dec_test!(bid128_to_uint32_xint_415, bid128_to_uint32_xint, 0xB0180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x01); //  -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xint_416, bid128_to_uint32_xint, 0xB0180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x01); //  -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xint_417, bid128_to_uint32_xint, 0xB0180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x01); //  -(2^31+0.5)
dec_test!(bid128_to_uint32_xint_418, bid128_to_uint32_xint, 0xB0180002B5E3AF19676BAF16B1880001u128, 2147483648   , 0x01); //  -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xint_419, bid128_to_uint32_xint, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, 2147483648   , 0x01); //  -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xint_420, bid128_to_uint32_xint, 0xB01800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01); //  -(2^32-0.5)
dec_test!(bid128_to_uint32_xint_421, bid128_to_uint32_xint, 0xB01800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01); //  -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xint_422, bid128_to_uint32_xint, 0xB01800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); //  -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xint_423, bid128_to_uint32_xint, 0xB01800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); //  -(2^32+0.5)
dec_test!(bid128_to_uint32_xint_424, bid128_to_uint32_xint, 0xB01800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); //  -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xint_425, bid128_to_uint32_xint, 0xB01A0000000000004563918244F3FFFFu128, 0            , 0x20); //  -(0.5-ulp)
dec_test!(bid128_to_uint32_xint_426, bid128_to_uint32_xint, 0xB01A0000000000004563918244F40000u128, 0            , 0x20); //  -(0.5)
dec_test!(bid128_to_uint32_xint_427, bid128_to_uint32_xint, 0xB01A0000000000004563918244F40001u128, 0            , 0x20); //  -(0.5+ulp)
dec_test!(bid128_to_uint32_xint_428, bid128_to_uint32_xint, 0xB01A0000000000008AC7230489E7FFFFu128, 0            , 0x20); //  -(1-ulp)
dec_test!(bid128_to_uint32_xint_429, bid128_to_uint32_xint, 0xB01A0000000000008AC7230489E80000u128, 2147483648   , 0x01); //  -(1)
dec_test!(bid128_to_uint32_xint_430, bid128_to_uint32_xint, 0xB01A0000000000008AC7230489E80001u128, 2147483648   , 0x01); //  -(1+ulp)
dec_test!(bid128_to_uint32_xint_431, bid128_to_uint32_xint, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, 2147483648   , 0x01); //  -(300.5-ulp)
dec_test!(bid128_to_uint32_xint_432, bid128_to_uint32_xint, 0xB01A0000000000A2E6C09AD3E0D40000u128, 2147483648   , 0x01); //  -(300.5)
dec_test!(bid128_to_uint32_xint_433, bid128_to_uint32_xint, 0xB01A0000000000A2E6C09AD3E0D40001u128, 2147483648   , 0x01); //  -(300.5+ulp)
dec_test!(bid128_to_uint32_xint_434, bid128_to_uint32_xint, 0xB01A000045639181BA2CDCFB7617FFFFu128, 2147483648   , 0x01); //  -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xint_435, bid128_to_uint32_xint, 0xB01A000045639181BA2CDCFB76180000u128, 2147483648   , 0x01); //  -(2^31-1)
dec_test!(bid128_to_uint32_xint_436, bid128_to_uint32_xint, 0xB01A000045639181BA2CDCFB76180001u128, 2147483648   , 0x01); //  -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xint_437, bid128_to_uint32_xint, 0xB01A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x01); //  -(2^31-ulp)
dec_test!(bid128_to_uint32_xint_438, bid128_to_uint32_xint, 0xB01A00004563918244F4000000000000u128, 2147483648   , 0x01); //  -(2^31)
dec_test!(bid128_to_uint32_xint_439, bid128_to_uint32_xint, 0xB01A00004563918244F4000000000001u128, 2147483648   , 0x01); //  -(2^31+ulp)
dec_test!(bid128_to_uint32_xint_440, bid128_to_uint32_xint, 0xB01A000045639182CFBB230489E7FFFFu128, 2147483648   , 0x01); //  -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xint_441, bid128_to_uint32_xint, 0xB01A000045639182CFBB230489E80000u128, 2147483648   , 0x01); //  -(2^31+1)
dec_test!(bid128_to_uint32_xint_442, bid128_to_uint32_xint, 0xB01A000045639182CFBB230489E80001u128, 2147483648   , 0x01); //  -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xint_443, bid128_to_uint32_xint, 0xB01A00008AC72303FF20DCFB7617FFFFu128, 2147483648   , 0x01); //  -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xint_444, bid128_to_uint32_xint, 0xB01A00008AC72303FF20DCFB76180000u128, 2147483648   , 0x01); //  -(2^32-1)
dec_test!(bid128_to_uint32_xint_445, bid128_to_uint32_xint, 0xB01A00008AC72303FF20DCFB76180001u128, 2147483648   , 0x01); //  -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xint_446, bid128_to_uint32_xint, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01); //  -(2^32-ulp)
dec_test!(bid128_to_uint32_xint_447, bid128_to_uint32_xint, 0xB01A00008AC7230489E8000000000000u128, 2147483648   , 0x01); //  -(2^32)
dec_test!(bid128_to_uint32_xint_448, bid128_to_uint32_xint, 0xB01A00008AC7230489E8000000000001u128, 2147483648   , 0x01); //  -(2^32+ulp)
dec_test!(bid128_to_uint32_xint_449, bid128_to_uint32_xint, 0xB01A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); //  -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xint_450, bid128_to_uint32_xint, 0xB01A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); //  -(2^32+1)
dec_test!(bid128_to_uint32_xint_451, bid128_to_uint32_xint, 0xB01A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); //  -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xint_452, bid128_to_uint32_xint, 0xb01a0000f0a08611eeaf3e7edf7e5fffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_453, bid128_to_uint32_xint, 0xB01C00000000000014D1120D7B15FFFFu128, 2147483648   , 0x01); //  -(1.5-ulp)
dec_test!(bid128_to_uint32_xint_454, bid128_to_uint32_xint, 0xB01C00000000000014D1120D7B160000u128, 2147483648   , 0x01); //  -(1.5)
dec_test!(bid128_to_uint32_xint_455, bid128_to_uint32_xint, 0xB01C00000000000014D1120D7B160001u128, 2147483648   , 0x01); //  -(1.5+ulp)
dec_test!(bid128_to_uint32_xint_456, bid128_to_uint32_xint, 0xB01E000000000001A055690D9DB7FFFFu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_457, bid128_to_uint32_xint, 0xB01E000000000001A055690D9DB80000u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_458, bid128_to_uint32_xint, 0xB01E000000000001A055690D9DB80001u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_459, bid128_to_uint32_xint, 0xB02000000000000029A2241AF62BFFFFu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_460, bid128_to_uint32_xint, 0xB02000000000000029A2241AF62C0000u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_461, bid128_to_uint32_xint, 0xB02000000000000029A2241AF62C0001u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_462, bid128_to_uint32_xint, 0xB024000000000000006A94D74F42FFFFu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_463, bid128_to_uint32_xint, 0xB024000000000000006A94D74F430000u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_464, bid128_to_uint32_xint, 0xB024000000000000006A94D74F430001u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_465, bid128_to_uint32_xint, 0xB02A00000000000000000017428106FFu128, 0            , 0x20); //  -(0.999-ulp)
dec_test!(bid128_to_uint32_xint_466, bid128_to_uint32_xint, 0xB02A0000000000000000001742810700u128, 0            , 0x20); //  -(0.999)
dec_test!(bid128_to_uint32_xint_467, bid128_to_uint32_xint, 0xB02A0000000000000000001742810701u128, 0            , 0x20); //  -(0.999+ulp)
dec_test!(bid128_to_uint32_xint_468, bid128_to_uint32_xint, 0xB02A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); //  -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_xint_469, bid128_to_uint32_xint, 0xB02A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); //  -(2e10-1.5)
dec_test!(bid128_to_uint32_xint_470, bid128_to_uint32_xint, 0xB02A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); //  -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_xint_471, bid128_to_uint32_xint, 0xB02A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); //  -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_xint_472, bid128_to_uint32_xint, 0xB02A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); //  -(2e10-0.5)
dec_test!(bid128_to_uint32_xint_473, bid128_to_uint32_xint, 0xB02A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); //  -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_xint_474, bid128_to_uint32_xint, 0xB02C000000000000000002BBA7F521FFu128, 2147483648   , 0x01); //  -(300.5-ulp)
dec_test!(bid128_to_uint32_xint_475, bid128_to_uint32_xint, 0xB02C000000000000000002BBA7F52200u128, 2147483648   , 0x01); //  -(300.5)
dec_test!(bid128_to_uint32_xint_476, bid128_to_uint32_xint, 0xB02C000000000000000002BBA7F52201u128, 2147483648   , 0x01); //  -(300.5+ulp)
dec_test!(bid128_to_uint32_xint_477, bid128_to_uint32_xint, 0xB02C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); //  -(2e10-1-ulp)
dec_test!(bid128_to_uint32_xint_478, bid128_to_uint32_xint, 0xB02C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); //  -(2e10-1)
dec_test!(bid128_to_uint32_xint_479, bid128_to_uint32_xint, 0xB02C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); //  -(2e10-1+ulp)
dec_test!(bid128_to_uint32_xint_480, bid128_to_uint32_xint, 0xB02C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); //  -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_xint_481, bid128_to_uint32_xint, 0xB02C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); //  -(2e10+0.5)
dec_test!(bid128_to_uint32_xint_482, bid128_to_uint32_xint, 0xB02C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); //  -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_xint_483, bid128_to_uint32_xint, 0xB02C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); //  -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_xint_484, bid128_to_uint32_xint, 0xB02C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); //  -(2e10+1.5)
dec_test!(bid128_to_uint32_xint_485, bid128_to_uint32_xint, 0xB02C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); //  -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_xint_486, bid128_to_uint32_xint, 0xB02E000000000000000000001DCD64FFu128, 0            , 0x20); //  -(0.5-ulp)
dec_test!(bid128_to_uint32_xint_487, bid128_to_uint32_xint, 0xB02E000000000000000000001DCD6500u128, 0            , 0x20); //  -(0.5)
dec_test!(bid128_to_uint32_xint_488, bid128_to_uint32_xint, 0xB02E000000000000000000001DCD6501u128, 0            , 0x20); //  -(0.5+ulp)
dec_test!(bid128_to_uint32_xint_489, bid128_to_uint32_xint, 0xB02E000000000000000000003B9AC9FFu128, 0            , 0x20); //  -(1-ulp)
dec_test!(bid128_to_uint32_xint_490, bid128_to_uint32_xint, 0xB02E000000000000000000003B9ACA00u128, 2147483648   , 0x01); //  -(1)
dec_test!(bid128_to_uint32_xint_491, bid128_to_uint32_xint, 0xB02E000000000000000000003B9ACA01u128, 2147483648   , 0x01); //  -(1+ulp)
dec_test!(bid128_to_uint32_xint_492, bid128_to_uint32_xint, 0xB02E0000000000000000000059682EFFu128, 2147483648   , 0x01); //  -(1.5-ulp)
dec_test!(bid128_to_uint32_xint_493, bid128_to_uint32_xint, 0xB02E0000000000000000000059682F00u128, 2147483648   , 0x01); //  -(1.5)
dec_test!(bid128_to_uint32_xint_494, bid128_to_uint32_xint, 0xB02E0000000000000000000059682F01u128, 2147483648   , 0x01); //  -(1.5+ulp)
dec_test!(bid128_to_uint32_xint_495, bid128_to_uint32_xint, 0xB02E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); //  -(2e10+1-ulp)
dec_test!(bid128_to_uint32_xint_496, bid128_to_uint32_xint, 0xB02E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); //  -(2e10+1)
dec_test!(bid128_to_uint32_xint_497, bid128_to_uint32_xint, 0xB02E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); //  -(2e10+1+ulp)
dec_test!(bid128_to_uint32_xint_498, bid128_to_uint32_xint, 0xB03000000000000000000006FC23ABFFu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_499, bid128_to_uint32_xint, 0xB03000000000000000000006FC23AC00u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_500, bid128_to_uint32_xint, 0xB03000000000000000000006FC23AC01u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_501, bid128_to_uint32_xint, 0xB03200000000000000000000B2D05DFFu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_502, bid128_to_uint32_xint, 0xB03200000000000000000000B2D05E00u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_503, bid128_to_uint32_xint, 0xB03200000000000000000000B2D05E01u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_504, bid128_to_uint32_xint, 0xB03800000000000000000000002DDA47u128, 2147483648   , 0x01); //  -(300.5-ulp)
dec_test!(bid128_to_uint32_xint_505, bid128_to_uint32_xint, 0xB03800000000000000000000002DDA48u128, 2147483648   , 0x01); //  -(300.5)
dec_test!(bid128_to_uint32_xint_506, bid128_to_uint32_xint, 0xB03800000000000000000000002DDA49u128, 2147483648   , 0x01); //  -(300.5+ulp)
dec_test!(bid128_to_uint32_xint_507, bid128_to_uint32_xint, 0xB03A00000000000000000000000003E7u128, 0            , 0x20); //  -(0.999)
dec_test!(bid128_to_uint32_xint_508, bid128_to_uint32_xint, 0xB03A00000000000000000000000005DBu128, 2147483648   , 0x01); //  -(1.5-ulp)
dec_test!(bid128_to_uint32_xint_509, bid128_to_uint32_xint, 0xB03A00000000000000000000000005DCu128, 2147483648   , 0x01); //  -(1.5)
dec_test!(bid128_to_uint32_xint_510, bid128_to_uint32_xint, 0xB03A00000000000000000000000005DDu128, 2147483648   , 0x01); //  -(1.5+ulp)
dec_test!(bid128_to_uint32_xint_511, bid128_to_uint32_xint, 0xB03A00000000000000000000000495D3u128, 2147483648   , 0x01); //  -(300.5-ulp)
dec_test!(bid128_to_uint32_xint_512, bid128_to_uint32_xint, 0xB03A00000000000000000000000495D4u128, 2147483648   , 0x01); //  -(300.5)
dec_test!(bid128_to_uint32_xint_513, bid128_to_uint32_xint, 0xB03A00000000000000000000000495D5u128, 2147483648   , 0x01); //  -(300.5+ulp)
dec_test!(bid128_to_uint32_xint_514, bid128_to_uint32_xint, 0xB03C0000000000000000000000000095u128, 2147483648   , 0x01); //  -(1.5-ulp)
dec_test!(bid128_to_uint32_xint_515, bid128_to_uint32_xint, 0xB03C0000000000000000000000000096u128, 2147483648   , 0x01); //  -(1.5)
dec_test!(bid128_to_uint32_xint_516, bid128_to_uint32_xint, 0xB03C0000000000000000000000000097u128, 2147483648   , 0x01); //  -(1.5+ulp)
dec_test!(bid128_to_uint32_xint_517, bid128_to_uint32_xint, 0xB03C0000000000000000000000007561u128, 2147483648   , 0x01); //  -(300.5-ulp)
dec_test!(bid128_to_uint32_xint_518, bid128_to_uint32_xint, 0xB03C0000000000000000000000007562u128, 2147483648   , 0x01); //  -(300.5)
dec_test!(bid128_to_uint32_xint_519, bid128_to_uint32_xint, 0xB03C0000000000000000000000007563u128, 2147483648   , 0x01); //  -(300.5+ulp)
dec_test!(bid128_to_uint32_xint_520, bid128_to_uint32_xint, 0xB03C00000000000000000031FFFFFF69u128, 2147483648   , 0x01); //  -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xint_521, bid128_to_uint32_xint, 0xB03C00000000000000000031FFFFFF6Au128, 2147483648   , 0x01); //  -(2^31-1.5)
dec_test!(bid128_to_uint32_xint_522, bid128_to_uint32_xint, 0xB03C00000000000000000031FFFFFF6Bu128, 2147483648   , 0x01); //  -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xint_523, bid128_to_uint32_xint, 0xB03C00000000000000000031FFFFFFCDu128, 2147483648   , 0x01); //  -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xint_524, bid128_to_uint32_xint, 0xB03C00000000000000000031FFFFFFCEu128, 2147483648   , 0x01); //  -(2^31-0.5)
dec_test!(bid128_to_uint32_xint_525, bid128_to_uint32_xint, 0xB03C00000000000000000031FFFFFFCFu128, 2147483648   , 0x01); //  -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xint_526, bid128_to_uint32_xint, 0xB03C0000000000000000003200000031u128, 2147483648   , 0x01); //  -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xint_527, bid128_to_uint32_xint, 0xB03C0000000000000000003200000032u128, 2147483648   , 0x01); //  -(2^31+0.5)
dec_test!(bid128_to_uint32_xint_528, bid128_to_uint32_xint, 0xB03C0000000000000000003200000033u128, 2147483648   , 0x01); //  -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xint_529, bid128_to_uint32_xint, 0xB03C00000000000000000063FFFFFFCDu128, 2147483648   , 0x01); //  -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xint_530, bid128_to_uint32_xint, 0xB03C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01); //  -(2^32-0.5)
dec_test!(bid128_to_uint32_xint_531, bid128_to_uint32_xint, 0xB03C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01); //  -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xint_532, bid128_to_uint32_xint, 0xB03C0000000000000000006400000031u128, 2147483648   , 0x01); //  -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xint_533, bid128_to_uint32_xint, 0xB03C0000000000000000006400000032u128, 2147483648   , 0x01); //  -(2^32+0.5)
dec_test!(bid128_to_uint32_xint_534, bid128_to_uint32_xint, 0xB03C0000000000000000006400000033u128, 2147483648   , 0x01); //  -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xint_535, bid128_to_uint32_xint, 0xB03E0000000000000000000000000005u128, 0            , 0x20); //  -(0.5)
dec_test!(bid128_to_uint32_xint_536, bid128_to_uint32_xint, 0xB03E000000000000000000000000000Fu128, 2147483648   , 0x01); //  -(1.5)
dec_test!(bid128_to_uint32_xint_537, bid128_to_uint32_xint, 0xB03E0000000000000000000000000BB7u128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_538, bid128_to_uint32_xint, 0xB03E0000000000000000000000000BB8u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_539, bid128_to_uint32_xint, 0xB03E0000000000000000000000000BB9u128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_540, bid128_to_uint32_xint, 0xB03E0000000000000000000000000BBDu128, 2147483648   , 0x01); //  -(300.5)
dec_test!(bid128_to_uint32_xint_541, bid128_to_uint32_xint, 0xB03E00000000000000000004FFFFFFF1u128, 2147483648   , 0x01); //  -(2^31-1.5)
dec_test!(bid128_to_uint32_xint_542, bid128_to_uint32_xint, 0xB03E00000000000000000004FFFFFFF5u128, 2147483648   , 0x01); //  -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xint_543, bid128_to_uint32_xint, 0xB03E00000000000000000004FFFFFFF6u128, 2147483648   , 0x01); //  -(2^31-1)
dec_test!(bid128_to_uint32_xint_544, bid128_to_uint32_xint, 0xB03E00000000000000000004FFFFFFF7u128, 2147483648   , 0x01); //  -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xint_545, bid128_to_uint32_xint, 0xB03E00000000000000000004FFFFFFFBu128, 2147483648   , 0x01); //  -(2^31-0.5)
dec_test!(bid128_to_uint32_xint_546, bid128_to_uint32_xint, 0xB03E00000000000000000004FFFFFFFFu128, 2147483648   , 0x01); //  -(2^31-ulp)
dec_test!(bid128_to_uint32_xint_547, bid128_to_uint32_xint, 0xB03E0000000000000000000500000000u128, 2147483648   , 0x01); //  -(2^31)
dec_test!(bid128_to_uint32_xint_548, bid128_to_uint32_xint, 0xB03E0000000000000000000500000001u128, 2147483648   , 0x01); //  -(2^31+ulp)
dec_test!(bid128_to_uint32_xint_549, bid128_to_uint32_xint, 0xB03E0000000000000000000500000005u128, 2147483648   , 0x01); //  -(2^31+0.5)
dec_test!(bid128_to_uint32_xint_550, bid128_to_uint32_xint, 0xB03E0000000000000000000500000009u128, 2147483648   , 0x01); //  -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xint_551, bid128_to_uint32_xint, 0xB03E000000000000000000050000000Au128, 2147483648   , 0x01); //  -(2^31+1)
dec_test!(bid128_to_uint32_xint_552, bid128_to_uint32_xint, 0xB03E000000000000000000050000000Bu128, 2147483648   , 0x01); //  -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xint_553, bid128_to_uint32_xint, 0xB03E00000000000000000009FFFFFFF5u128, 2147483648   , 0x01); //  -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xint_554, bid128_to_uint32_xint, 0xB03E00000000000000000009FFFFFFF6u128, 2147483648   , 0x01); //  -(2^32-1)
dec_test!(bid128_to_uint32_xint_555, bid128_to_uint32_xint, 0xB03E00000000000000000009FFFFFFF7u128, 2147483648   , 0x01); //  -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xint_556, bid128_to_uint32_xint, 0xB03E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01); //  -(2^32-0.5)
dec_test!(bid128_to_uint32_xint_557, bid128_to_uint32_xint, 0xB03E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01); //  -(2^32-ulp)
dec_test!(bid128_to_uint32_xint_558, bid128_to_uint32_xint, 0xB03E0000000000000000000A00000000u128, 2147483648   , 0x01); //  -(2^32)
dec_test!(bid128_to_uint32_xint_559, bid128_to_uint32_xint, 0xB03E0000000000000000000A00000001u128, 2147483648   , 0x01); //  -(2^32+ulp)
dec_test!(bid128_to_uint32_xint_560, bid128_to_uint32_xint, 0xB03E0000000000000000000A00000005u128, 2147483648   , 0x01); //  -(2^32+0.5)
dec_test!(bid128_to_uint32_xint_561, bid128_to_uint32_xint, 0xB03E0000000000000000000A00000009u128, 2147483648   , 0x01); //  -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xint_562, bid128_to_uint32_xint, 0xB03E0000000000000000000A0000000Au128, 2147483648   , 0x01); //  -(2^32+1)
dec_test!(bid128_to_uint32_xint_563, bid128_to_uint32_xint, 0xB03E0000000000000000000A0000000Bu128, 2147483648   , 0x01); //  -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xint_564, bid128_to_uint32_xint, 0xB03E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); //  -(2e10-1.5)
dec_test!(bid128_to_uint32_xint_565, bid128_to_uint32_xint, 0xB03E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); //  -(2e10-0.5)
dec_test!(bid128_to_uint32_xint_566, bid128_to_uint32_xint, 0xB03E0000000000000000002E90EDD005u128, 2147483648   , 0x01); //  -(2e10+0.5)
dec_test!(bid128_to_uint32_xint_567, bid128_to_uint32_xint, 0xB03E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); //  -(2e10+1.5)
dec_test!(bid128_to_uint32_xint_568, bid128_to_uint32_xint, 0xB0400000000000000000000000000001u128, 2147483648   , 0x01); //  -(1)
dec_test!(bid128_to_uint32_xint_569, bid128_to_uint32_xint, 0xB040000000000000000000000000012Bu128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_570, bid128_to_uint32_xint, 0xB040000000000000000000000000012Cu128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_571, bid128_to_uint32_xint, 0xB040000000000000000000000000012Du128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_572, bid128_to_uint32_xint, 0xB040000000000000000000007FFFFFFFu128, 2147483648   , 0x01); //  -(2^31-1)
dec_test!(bid128_to_uint32_xint_573, bid128_to_uint32_xint, 0xB0400000000000000000000080000000u128, 2147483648   , 0x01); //  -(2^31)
dec_test!(bid128_to_uint32_xint_574, bid128_to_uint32_xint, 0xB0400000000000000000000080000001u128, 2147483648   , 0x01); //  -(2^31+1)
dec_test!(bid128_to_uint32_xint_575, bid128_to_uint32_xint, 0xB04000000000000000000000FFFFFFFFu128, 2147483648   , 0x01); //  -(2^32-1)
dec_test!(bid128_to_uint32_xint_576, bid128_to_uint32_xint, 0xB0400000000000000000000100000000u128, 2147483648   , 0x01); //  -(2^32)
dec_test!(bid128_to_uint32_xint_577, bid128_to_uint32_xint, 0xB0400000000000000000000100000001u128, 2147483648   , 0x01); //  -(2^32+1)
dec_test!(bid128_to_uint32_xint_578, bid128_to_uint32_xint, 0xB04000000000000000000004A817C7FFu128, 2147483648   , 0x01); //  -(2e10-1)
dec_test!(bid128_to_uint32_xint_579, bid128_to_uint32_xint, 0xB04000000000000000000004A817C801u128, 2147483648   , 0x01); //  -(2e10+1)
dec_test!(bid128_to_uint32_xint_580, bid128_to_uint32_xint, 0xB042000000000000000000000000001Du128, 2147483648   , 0x01); //  -(300-ulp)
dec_test!(bid128_to_uint32_xint_581, bid128_to_uint32_xint, 0xB042000000000000000000000000001Eu128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_582, bid128_to_uint32_xint, 0xB042000000000000000000000000001Fu128, 2147483648   , 0x01); //  -(300+ulp)
dec_test!(bid128_to_uint32_xint_583, bid128_to_uint32_xint, 0xB04200000000000000000000773593FFu128, 2147483648   , 0x01); //  -(2e10-ulp)
dec_test!(bid128_to_uint32_xint_584, bid128_to_uint32_xint, 0xB0420000000000000000000077359400u128, 2147483648   , 0x01); //  -(2e10)
dec_test!(bid128_to_uint32_xint_585, bid128_to_uint32_xint, 0xB0420000000000000000000077359401u128, 2147483648   , 0x01); //  -(2e10+ulp)
dec_test!(bid128_to_uint32_xint_586, bid128_to_uint32_xint, 0xB0440000000000000000000000000003u128, 2147483648   , 0x01); //  -(300)
dec_test!(bid128_to_uint32_xint_587, bid128_to_uint32_xint, 0xB0520000000000000000000000000004u128, 2147483648   , 0x01); //  -(4e9)
dec_test!(bid128_to_uint32_xint_588, bid128_to_uint32_xint, 0xB0520000000000000000000000000005u128, 2147483648   , 0x01); //  -(5e9)
dec_test!(bid128_to_uint32_xint_589, bid128_to_uint32_xint, 0xB0540000000000000000000000000002u128, 2147483648   , 0x01); //  -(2e10)
dec_test!(bid128_to_uint32_xint_590, bid128_to_uint32_xint, 0xc74ccd978165f2c27777b28ece449d3eu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_591, bid128_to_uint32_xint, 0xd75a0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xint_592, bid128_to_uint32_xint, 0xfbf6d2ffffbb7e7dfdffc3fd7ff9bffeu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_593, bid128_to_uint32_xint, 0xfd2f6f492805eafed0fd9559c8a82056u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_594, bid128_to_uint32_xint, 0xfffff3bfcfffffffa7f61c581a384ab0u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_595, bid128_to_uint32_xint, "-Infinity"                           , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_596, bid128_to_uint32_xint, "QNaN"                                , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xint_597, bid128_to_uint32_xint, "SNaN"                                , 0x80000000u32, 0x01);
