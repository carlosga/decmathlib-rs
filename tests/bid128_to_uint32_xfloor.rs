/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.                        */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_uint32_xfloor_001, bid128_to_uint32_xfloor, "-0"                                  , 0            , 0x00);
dec_test!(bid128_to_uint32_xfloor_002, bid128_to_uint32_xfloor, 0                                     , 0            , 0x00);
dec_test!(bid128_to_uint32_xfloor_003, bid128_to_uint32_xfloor, 0x0000000000000000ad7fffe5e47c9ff1u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xfloor_004, bid128_to_uint32_xfloor, 0x0001ed09bead87c0378d8e62ffffffffu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xfloor_005, bid128_to_uint32_xfloor, 0x0001ed09bead87c0378d8e64ffffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_xfloor_006, bid128_to_uint32_xfloor, 0x00020000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xfloor_007, bid128_to_uint32_xfloor, 0x02080000000000000000000040000000u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xfloor_008, bid128_to_uint32_xfloor, "1.0"                                 , 1            , 0x00);
dec_test!(bid128_to_uint32_xfloor_009, bid128_to_uint32_xfloor, 0x1904100018004000d01930281cc000a0u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xfloor_010, bid128_to_uint32_xfloor, 0x2a2d6371e8cb5053dbf7cb9f39e2eb5fu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xfloor_011, bid128_to_uint32_xfloor, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_uint32_xfloor_012, bid128_to_uint32_xfloor, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0            , 0x20); // -- 0.5
dec_test!(bid128_to_uint32_xfloor_013, bid128_to_uint32_xfloor, 0x2FFCF684DF56C3E01BC6C73200000001u128, 0            , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_uint32_xfloor_014, bid128_to_uint32_xfloor, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 0            , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_uint32_xfloor_015, bid128_to_uint32_xfloor, 0x2FFDEC8B86EF679D76FC433D80000000u128, 0            , 0x20); // -- 0.999
dec_test!(bid128_to_uint32_xfloor_016, bid128_to_uint32_xfloor, 0x2FFDEC8B86EF679D76FC433D80000001u128, 0            , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_uint32_xfloor_017, bid128_to_uint32_xfloor, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 0            , 0x20); // -- 1-ulp
dec_test!(bid128_to_uint32_xfloor_018, bid128_to_uint32_xfloor, 0x2FFE314DC6448D9338C15B0A00000000u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_xfloor_019, bid128_to_uint32_xfloor, 0x2FFE314DC6448D9338C15B0A00000001u128, 1            , 0x20); // -- 1+ulp
dec_test!(bid128_to_uint32_xfloor_020, bid128_to_uint32_xfloor, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xfloor_021, bid128_to_uint32_xfloor, 0x2FFE49F4A966D45CD522088F00000000u128, 1            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xfloor_022, bid128_to_uint32_xfloor, 0x2FFE49F4A966D45CD522088F00000001u128, 1            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xfloor_023, bid128_to_uint32_xfloor, 0x2fff3feeffffffbf0000000000000000u128, 6            , 0x20);
dec_test!(bid128_to_uint32_xfloor_024, bid128_to_uint32_xfloor, 0x2fff4f7ffdffb7ed0600000090088040u128, 6            , 0x20);
dec_test!(bid128_to_uint32_xfloor_025, bid128_to_uint32_xfloor, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_026, bid128_to_uint32_xfloor, 0x300293E952CDA8B9AA44111E00000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_027, bid128_to_uint32_xfloor, 0x300293E952CDA8B9AA44111E00000001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_028, bid128_to_uint32_xfloor, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xfloor_029, bid128_to_uint32_xfloor, 0x300294286EACB8CB0A8CB6B140000000u128, 300          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xfloor_030, bid128_to_uint32_xfloor, 0x300294286EACB8CB0A8CB6B140000001u128, 300          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xfloor_031, bid128_to_uint32_xfloor, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_032, bid128_to_uint32_xfloor, 0x30040ECA8847C4129106CE8300000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_033, bid128_to_uint32_xfloor, 0x30040ECA8847C4129106CE8300000001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_034, bid128_to_uint32_xfloor, 0x3007000008100021f3dfbb9efff7d7cfu128, 51922        , 0x20);
dec_test!(bid128_to_uint32_xfloor_035, bid128_to_uint32_xfloor, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_036, bid128_to_uint32_xfloor, 0x300A0003C95A2F0B4856475FE0000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_037, bid128_to_uint32_xfloor, 0x300A0003C95A2F0B4856475FE0000001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_038, bid128_to_uint32_xfloor, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_039, bid128_to_uint32_xfloor, 0x300C000060EF6B1ABA6F072330000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_040, bid128_to_uint32_xfloor, 0x300C000060EF6B1ABA6F072330000001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_041, bid128_to_uint32_xfloor, 0x3010552d04901900fff63af7ff7bbfbfu128, 1727571495   , 0x20);
dec_test!(bid128_to_uint32_xfloor_042, bid128_to_uint32_xfloor, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646   , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_xfloor_043, bid128_to_uint32_xfloor, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483646   , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_uint32_xfloor_044, bid128_to_uint32_xfloor, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483646   , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_xfloor_045, bid128_to_uint32_xfloor, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483646   , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_xfloor_046, bid128_to_uint32_xfloor, 0x301069E10DE692B4B4B133125F000000u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_xfloor_047, bid128_to_uint32_xfloor, 0x301069E10DE692B4B4B133125F000001u128, 2147483647   , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_xfloor_048, bid128_to_uint32_xfloor, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647   , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_xfloor_049, bid128_to_uint32_xfloor, 0x301069E10DE6FC95C29899892F800000u128, 2147483647   , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_uint32_xfloor_050, bid128_to_uint32_xfloor, 0x301069E10DE6FC95C29899892F800001u128, 2147483647   , 0x20); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_xfloor_051, bid128_to_uint32_xfloor, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, 2147483647   , 0x20); // -- 2^31-ulp
dec_test!(bid128_to_uint32_xfloor_052, bid128_to_uint32_xfloor, 0x301069E10DE76676D080000000000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_xfloor_053, bid128_to_uint32_xfloor, 0x301069E10DE76676D080000000000001u128, 2147483648   , 0x20); // -- 2^31+ulp
dec_test!(bid128_to_uint32_xfloor_054, bid128_to_uint32_xfloor, 0x301069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x20); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_xfloor_055, bid128_to_uint32_xfloor, 0x301069E10DE7D057DE676676D0800000u128, 2147483648   , 0x20); // -- 2^31+0.5
dec_test!(bid128_to_uint32_xfloor_056, bid128_to_uint32_xfloor, 0x301069E10DE7D057DE676676D0800001u128, 2147483648   , 0x20); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_xfloor_057, bid128_to_uint32_xfloor, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483648   , 0x20); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_xfloor_058, bid128_to_uint32_xfloor, 0x301069E10DE83A38EC4ECCEDA1000000u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_xfloor_059, bid128_to_uint32_xfloor, 0x301069E10DE83A38EC4ECCEDA1000001u128, 2147483649   , 0x20); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_xfloor_060, bid128_to_uint32_xfloor, 0x3010C5371912364CE3056C27FFFFFFFFu128, 3999999999   , 0x20); // -- 4e9-ulp
dec_test!(bid128_to_uint32_xfloor_061, bid128_to_uint32_xfloor, 0x3010C5371912364CE3056C2800000000u128, 4000000000   , 0x00); // -- 4e9
dec_test!(bid128_to_uint32_xfloor_062, bid128_to_uint32_xfloor, 0x3010C5371912364CE3056C2800000001u128, 4000000000   , 0x20); // -- 4e9+ulp
dec_test!(bid128_to_uint32_xfloor_063, bid128_to_uint32_xfloor, 0x3010D3C21BCDF92B853133125EFFFFFFu128, 4294967294   , 0x20); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_xfloor_064, bid128_to_uint32_xfloor, 0x3010D3C21BCDF92B853133125F000000u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_xfloor_065, bid128_to_uint32_xfloor, 0x3010D3C21BCDF92B853133125F000001u128, 4294967295   , 0x20); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_xfloor_066, bid128_to_uint32_xfloor, 0x3010D3C21BCE630C931899892F7FFFFFu128, 4294967295   , 0x20); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_xfloor_067, bid128_to_uint32_xfloor, 0x3010D3C21BCE630C931899892F800000u128, 4294967295   , 0x20); // -- 2^32-0.5
dec_test!(bid128_to_uint32_xfloor_068, bid128_to_uint32_xfloor, 0x3010D3C21BCE630C931899892F800001u128, 4294967295   , 0x20); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_xfloor_069, bid128_to_uint32_xfloor, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 4294967295   , 0x20); // -- 2^32-ulp
dec_test!(bid128_to_uint32_xfloor_070, bid128_to_uint32_xfloor, 0x3010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_xfloor_071, bid128_to_uint32_xfloor, 0x3010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_xfloor_072, bid128_to_uint32_xfloor, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_xfloor_073, bid128_to_uint32_xfloor, 0x3010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_xfloor_074, bid128_to_uint32_xfloor, 0x3010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_xfloor_075, bid128_to_uint32_xfloor, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_xfloor_076, bid128_to_uint32_xfloor, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_xfloor_077, bid128_to_uint32_xfloor, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_xfloor_078, bid128_to_uint32_xfloor, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); // -- 5e9-ulp
dec_test!(bid128_to_uint32_xfloor_079, bid128_to_uint32_xfloor, 0x3010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- 5e9
dec_test!(bid128_to_uint32_xfloor_080, bid128_to_uint32_xfloor, 0x3010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- 5e9+ulp
dec_test!(bid128_to_uint32_xfloor_081, bid128_to_uint32_xfloor, 0x30116700201058205703a6194e04e4bbu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_082, bid128_to_uint32_xfloor, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_xfloor_083, bid128_to_uint32_xfloor, 0x3012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_xfloor_084, bid128_to_uint32_xfloor, 0x3012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_xfloor_085, bid128_to_uint32_xfloor, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_xfloor_086, bid128_to_uint32_xfloor, 0x3012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_xfloor_087, bid128_to_uint32_xfloor, 0x3012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_xfloor_088, bid128_to_uint32_xfloor, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_xfloor_089, bid128_to_uint32_xfloor, 0x3012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_xfloor_090, bid128_to_uint32_xfloor, 0x3012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_xfloor_091, bid128_to_uint32_xfloor, 0x3012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_uint32_xfloor_092, bid128_to_uint32_xfloor, 0x3012629B8C891B267182B61400000000u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_xfloor_093, bid128_to_uint32_xfloor, 0x3012629B8C891B267182B61400000001u128, 2147483648   , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_uint32_xfloor_094, bid128_to_uint32_xfloor, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_xfloor_095, bid128_to_uint32_xfloor, 0x3012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_xfloor_096, bid128_to_uint32_xfloor, 0x3012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_xfloor_097, bid128_to_uint32_xfloor, 0x3012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_xfloor_098, bid128_to_uint32_xfloor, 0x3012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_xfloor_099, bid128_to_uint32_xfloor, 0x3012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_xfloor_100, bid128_to_uint32_xfloor, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_xfloor_101, bid128_to_uint32_xfloor, 0x3012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_xfloor_102, bid128_to_uint32_xfloor, 0x3012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_xfloor_103, bid128_to_uint32_xfloor, 0x3014000b8642050191eb289508194018u128, 91306059     , 0x20);
dec_test!(bid128_to_uint32_xfloor_104, bid128_to_uint32_xfloor, 0x30140200002400400000000000040000u128, 0xF1C91180   , 0x20);
dec_test!(bid128_to_uint32_xfloor_105, bid128_to_uint32_xfloor, 0x301600000000003627E8F712373BFFFFu128, 0            , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_uint32_xfloor_106, bid128_to_uint32_xfloor, 0x301600000000003627E8F712373C0000u128, 0            , 0x20); // -- 0.999
dec_test!(bid128_to_uint32_xfloor_107, bid128_to_uint32_xfloor, 0x301600000000003627E8F712373C0001u128, 0            , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_uint32_xfloor_108, bid128_to_uint32_xfloor, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646   , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_xfloor_109, bid128_to_uint32_xfloor, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483646   , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_uint32_xfloor_110, bid128_to_uint32_xfloor, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483646   , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_xfloor_111, bid128_to_uint32_xfloor, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647   , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_xfloor_112, bid128_to_uint32_xfloor, 0x30180002B5E3AF13FBA450E94E780000u128, 2147483647   , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_uint32_xfloor_113, bid128_to_uint32_xfloor, 0x30180002B5E3AF13FBA450E94E780001u128, 2147483647   , 0x20); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_xfloor_114, bid128_to_uint32_xfloor, 0x30180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x20); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_xfloor_115, bid128_to_uint32_xfloor, 0x30180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x20); // -- 2^31+0.5
dec_test!(bid128_to_uint32_xfloor_116, bid128_to_uint32_xfloor, 0x30180002B5E3AF19676BAF16B1880001u128, 2147483648   , 0x20); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_xfloor_117, bid128_to_uint32_xfloor, 0x301800056BC75E2AAD2C50E94E77FFFFu128, 4294967295   , 0x20); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_xfloor_118, bid128_to_uint32_xfloor, 0x301800056BC75E2AAD2C50E94E780000u128, 4294967295   , 0x20); // -- 2^32-0.5
dec_test!(bid128_to_uint32_xfloor_119, bid128_to_uint32_xfloor, 0x301800056BC75E2AAD2C50E94E780001u128, 4294967295   , 0x20); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_xfloor_120, bid128_to_uint32_xfloor, 0x301800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_xfloor_121, bid128_to_uint32_xfloor, 0x301800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_xfloor_122, bid128_to_uint32_xfloor, 0x301800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_xfloor_123, bid128_to_uint32_xfloor, 0x3018000886801000fdfdfffffffffffdu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_124, bid128_to_uint32_xfloor, 0x301A0000000000004563918244F3FFFFu128, 0            , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_uint32_xfloor_125, bid128_to_uint32_xfloor, 0x301A0000000000004563918244F40000u128, 0            , 0x20); // -- 0.5
dec_test!(bid128_to_uint32_xfloor_126, bid128_to_uint32_xfloor, 0x301A0000000000004563918244F40001u128, 0            , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_uint32_xfloor_127, bid128_to_uint32_xfloor, 0x301A0000000000008AC7230489E7FFFFu128, 0            , 0x20); // -- 1-ulp
dec_test!(bid128_to_uint32_xfloor_128, bid128_to_uint32_xfloor, 0x301A0000000000008AC7230489E80000u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_xfloor_129, bid128_to_uint32_xfloor, 0x301A0000000000008AC7230489E80001u128, 1            , 0x20); // -- 1+ulp
dec_test!(bid128_to_uint32_xfloor_130, bid128_to_uint32_xfloor, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xfloor_131, bid128_to_uint32_xfloor, 0x301A0000000000A2E6C09AD3E0D40000u128, 300          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xfloor_132, bid128_to_uint32_xfloor, 0x301A0000000000A2E6C09AD3E0D40001u128, 300          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xfloor_133, bid128_to_uint32_xfloor, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483646   , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_xfloor_134, bid128_to_uint32_xfloor, 0x301A000045639181BA2CDCFB76180000u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_xfloor_135, bid128_to_uint32_xfloor, 0x301A000045639181BA2CDCFB76180001u128, 2147483647   , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_xfloor_136, bid128_to_uint32_xfloor, 0x301A00004563918244F3FFFFFFFFFFFFu128, 2147483647   , 0x20); // -- 2^31-ulp
dec_test!(bid128_to_uint32_xfloor_137, bid128_to_uint32_xfloor, 0x301A00004563918244F4000000000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_xfloor_138, bid128_to_uint32_xfloor, 0x301A00004563918244F4000000000001u128, 2147483648   , 0x20); // -- 2^31+ulp
dec_test!(bid128_to_uint32_xfloor_139, bid128_to_uint32_xfloor, 0x301A000045639182CFBB230489E7FFFFu128, 2147483648   , 0x20); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_xfloor_140, bid128_to_uint32_xfloor, 0x301A000045639182CFBB230489E80000u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_xfloor_141, bid128_to_uint32_xfloor, 0x301A000045639182CFBB230489E80001u128, 2147483649   , 0x20); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_xfloor_142, bid128_to_uint32_xfloor, 0x301A00008AC72303FF20DCFB7617FFFFu128, 4294967294   , 0x20); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_xfloor_143, bid128_to_uint32_xfloor, 0x301A00008AC72303FF20DCFB76180000u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_xfloor_144, bid128_to_uint32_xfloor, 0x301A00008AC72303FF20DCFB76180001u128, 4294967295   , 0x20); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_xfloor_145, bid128_to_uint32_xfloor, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, 4294967295   , 0x20); // -- 2^32-ulp
dec_test!(bid128_to_uint32_xfloor_146, bid128_to_uint32_xfloor, 0x301A00008AC7230489E8000000000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_xfloor_147, bid128_to_uint32_xfloor, 0x301A00008AC7230489E8000000000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_xfloor_148, bid128_to_uint32_xfloor, 0x301A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_xfloor_149, bid128_to_uint32_xfloor, 0x301A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_xfloor_150, bid128_to_uint32_xfloor, 0x301A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_xfloor_151, bid128_to_uint32_xfloor, 0x301C00000000000014D1120D7B15FFFFu128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xfloor_152, bid128_to_uint32_xfloor, 0x301C00000000000014D1120D7B160000u128, 1            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xfloor_153, bid128_to_uint32_xfloor, 0x301C00000000000014D1120D7B160001u128, 1            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xfloor_154, bid128_to_uint32_xfloor, 0x301E000000000001A055690D9DB7FFFFu128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_155, bid128_to_uint32_xfloor, 0x301E000000000001A055690D9DB80000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_156, bid128_to_uint32_xfloor, 0x301E000000000001A055690D9DB80001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_157, bid128_to_uint32_xfloor, 0x302000000000000029A2241AF62BFFFFu128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_158, bid128_to_uint32_xfloor, 0x302000000000000029A2241AF62C0000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_159, bid128_to_uint32_xfloor, 0x302000000000000029A2241AF62C0001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_160, bid128_to_uint32_xfloor, 0x3020000000000000612d150849c54208u128, 700          , 0x20);
dec_test!(bid128_to_uint32_xfloor_161, bid128_to_uint32_xfloor, 0x3024000000000000006A94D74F42FFFFu128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_162, bid128_to_uint32_xfloor, 0x3024000000000000006A94D74F430000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_163, bid128_to_uint32_xfloor, 0x3024000000000000006A94D74F430001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_164, bid128_to_uint32_xfloor, 0x302A00000000000000000017428106FFu128, 0            , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_uint32_xfloor_165, bid128_to_uint32_xfloor, 0x302A0000000000000000001742810700u128, 0            , 0x20); // -- 0.999
dec_test!(bid128_to_uint32_xfloor_166, bid128_to_uint32_xfloor, 0x302A0000000000000000001742810701u128, 0            , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_uint32_xfloor_167, bid128_to_uint32_xfloor, 0x302A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_xfloor_168, bid128_to_uint32_xfloor, 0x302A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_xfloor_169, bid128_to_uint32_xfloor, 0x302A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_xfloor_170, bid128_to_uint32_xfloor, 0x302A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_xfloor_171, bid128_to_uint32_xfloor, 0x302A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_xfloor_172, bid128_to_uint32_xfloor, 0x302A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_xfloor_173, bid128_to_uint32_xfloor, 0x302C000000000000000002BBA7F521FFu128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xfloor_174, bid128_to_uint32_xfloor, 0x302C000000000000000002BBA7F52200u128, 300          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xfloor_175, bid128_to_uint32_xfloor, 0x302C000000000000000002BBA7F52201u128, 300          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xfloor_176, bid128_to_uint32_xfloor, 0x302C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_xfloor_177, bid128_to_uint32_xfloor, 0x302C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_xfloor_178, bid128_to_uint32_xfloor, 0x302C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_xfloor_179, bid128_to_uint32_xfloor, 0x302C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_xfloor_180, bid128_to_uint32_xfloor, 0x302C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_xfloor_181, bid128_to_uint32_xfloor, 0x302C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_xfloor_182, bid128_to_uint32_xfloor, 0x302C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_xfloor_183, bid128_to_uint32_xfloor, 0x302C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_xfloor_184, bid128_to_uint32_xfloor, 0x302C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_xfloor_185, bid128_to_uint32_xfloor, 0x302E000000000000000000001DCD64FFu128, 0            , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_uint32_xfloor_186, bid128_to_uint32_xfloor, 0x302E000000000000000000001DCD6500u128, 0            , 0x20); // -- 0.5
dec_test!(bid128_to_uint32_xfloor_187, bid128_to_uint32_xfloor, 0x302E000000000000000000001DCD6501u128, 0            , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_uint32_xfloor_188, bid128_to_uint32_xfloor, 0x302E000000000000000000003B9AC9FFu128, 0            , 0x20); // -- 1-ulp
dec_test!(bid128_to_uint32_xfloor_189, bid128_to_uint32_xfloor, 0x302E000000000000000000003B9ACA00u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_xfloor_190, bid128_to_uint32_xfloor, 0x302E000000000000000000003B9ACA01u128, 1            , 0x20); // -- 1+ulp
dec_test!(bid128_to_uint32_xfloor_191, bid128_to_uint32_xfloor, 0x302E0000000000000000000059682EFFu128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xfloor_192, bid128_to_uint32_xfloor, 0x302E0000000000000000000059682F00u128, 1            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xfloor_193, bid128_to_uint32_xfloor, 0x302E0000000000000000000059682F01u128, 1            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xfloor_194, bid128_to_uint32_xfloor, 0x302E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_xfloor_195, bid128_to_uint32_xfloor, 0x302E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_xfloor_196, bid128_to_uint32_xfloor, 0x302E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_xfloor_197, bid128_to_uint32_xfloor, 0x303000000000000000000006FC23ABFFu128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_198, bid128_to_uint32_xfloor, 0x303000000000000000000006FC23AC00u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_199, bid128_to_uint32_xfloor, 0x303000000000000000000006FC23AC01u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_200, bid128_to_uint32_xfloor, 0x303200000000000000000000B2D05DFFu128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_201, bid128_to_uint32_xfloor, 0x303200000000000000000000B2D05E00u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_202, bid128_to_uint32_xfloor, 0x303200000000000000000000B2D05E01u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_203, bid128_to_uint32_xfloor, 0x303800000000000000000000002DDA47u128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xfloor_204, bid128_to_uint32_xfloor, 0x303800000000000000000000002DDA48u128, 300          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xfloor_205, bid128_to_uint32_xfloor, 0x303800000000000000000000002DDA49u128, 300          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xfloor_206, bid128_to_uint32_xfloor, 0x303A00000000000000000000000003E7u128, 0            , 0x20); // -- 0.999
dec_test!(bid128_to_uint32_xfloor_207, bid128_to_uint32_xfloor, 0x303A00000000000000000000000005DBu128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xfloor_208, bid128_to_uint32_xfloor, 0x303A00000000000000000000000005DCu128, 1            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xfloor_209, bid128_to_uint32_xfloor, 0x303A00000000000000000000000005DDu128, 1            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xfloor_210, bid128_to_uint32_xfloor, 0x303A00000000000000000000000495D3u128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xfloor_211, bid128_to_uint32_xfloor, 0x303A00000000000000000000000495D4u128, 300          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xfloor_212, bid128_to_uint32_xfloor, 0x303A00000000000000000000000495D5u128, 300          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xfloor_213, bid128_to_uint32_xfloor, 0x303C0000000000000000000000000095u128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xfloor_214, bid128_to_uint32_xfloor, 0x303C0000000000000000000000000096u128, 1            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xfloor_215, bid128_to_uint32_xfloor, 0x303C0000000000000000000000000097u128, 1            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xfloor_216, bid128_to_uint32_xfloor, 0x303C0000000000000000000000007561u128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xfloor_217, bid128_to_uint32_xfloor, 0x303C0000000000000000000000007562u128, 300          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xfloor_218, bid128_to_uint32_xfloor, 0x303C0000000000000000000000007563u128, 300          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xfloor_219, bid128_to_uint32_xfloor, 0x303C00000000000000000031FFFFFF69u128, 2147483646   , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_xfloor_220, bid128_to_uint32_xfloor, 0x303C00000000000000000031FFFFFF6Au128, 2147483646   , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_uint32_xfloor_221, bid128_to_uint32_xfloor, 0x303C00000000000000000031FFFFFF6Bu128, 2147483646   , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_xfloor_222, bid128_to_uint32_xfloor, 0x303C00000000000000000031FFFFFFCDu128, 2147483647   , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_xfloor_223, bid128_to_uint32_xfloor, 0x303C00000000000000000031FFFFFFCEu128, 2147483647   , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_uint32_xfloor_224, bid128_to_uint32_xfloor, 0x303C00000000000000000031FFFFFFCFu128, 2147483647   , 0x20); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_xfloor_225, bid128_to_uint32_xfloor, 0x303C0000000000000000003200000031u128, 2147483648   , 0x20); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_xfloor_226, bid128_to_uint32_xfloor, 0x303C0000000000000000003200000032u128, 2147483648   , 0x20); // -- 2^31+0.5
dec_test!(bid128_to_uint32_xfloor_227, bid128_to_uint32_xfloor, 0x303C0000000000000000003200000033u128, 2147483648   , 0x20); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_xfloor_228, bid128_to_uint32_xfloor, 0x303C00000000000000000063FFFFFFCDu128, 4294967295   , 0x20); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_xfloor_229, bid128_to_uint32_xfloor, 0x303C00000000000000000063FFFFFFCEu128, 4294967295   , 0x20); // -- 2^32-0.5
dec_test!(bid128_to_uint32_xfloor_230, bid128_to_uint32_xfloor, 0x303C00000000000000000063FFFFFFCFu128, 4294967295   , 0x20); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_xfloor_231, bid128_to_uint32_xfloor, 0x303C0000000000000000006400000031u128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_xfloor_232, bid128_to_uint32_xfloor, 0x303C0000000000000000006400000032u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_xfloor_233, bid128_to_uint32_xfloor, 0x303C0000000000000000006400000033u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_xfloor_234, bid128_to_uint32_xfloor, 0x303E0000000000000000000000000005u128, 0            , 0x20); // -- 0.5
dec_test!(bid128_to_uint32_xfloor_235, bid128_to_uint32_xfloor, 0x303E000000000000000000000000000Fu128, 1            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xfloor_236, bid128_to_uint32_xfloor, 0x303E0000000000000000000000000BB7u128, 299          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_237, bid128_to_uint32_xfloor, 0x303E0000000000000000000000000BB8u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_238, bid128_to_uint32_xfloor, 0x303E0000000000000000000000000BB9u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_239, bid128_to_uint32_xfloor, 0x303E0000000000000000000000000BBDu128, 300          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xfloor_240, bid128_to_uint32_xfloor, 0x303E00000000000000000004FFFFFFF1u128, 2147483646   , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_uint32_xfloor_241, bid128_to_uint32_xfloor, 0x303E00000000000000000004FFFFFFF5u128, 2147483646   , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_xfloor_242, bid128_to_uint32_xfloor, 0x303E00000000000000000004FFFFFFF6u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_xfloor_243, bid128_to_uint32_xfloor, 0x303E00000000000000000004FFFFFFF7u128, 2147483647   , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_xfloor_244, bid128_to_uint32_xfloor, 0x303E00000000000000000004FFFFFFFBu128, 2147483647   , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_uint32_xfloor_245, bid128_to_uint32_xfloor, 0x303E00000000000000000004FFFFFFFFu128, 2147483647   , 0x20); // -- 2^31-ulp
dec_test!(bid128_to_uint32_xfloor_246, bid128_to_uint32_xfloor, 0x303E0000000000000000000500000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_xfloor_247, bid128_to_uint32_xfloor, 0x303E0000000000000000000500000001u128, 2147483648   , 0x20); // -- 2^31+ulp
dec_test!(bid128_to_uint32_xfloor_248, bid128_to_uint32_xfloor, 0x303E0000000000000000000500000005u128, 2147483648   , 0x20); // -- 2^31+0.5
dec_test!(bid128_to_uint32_xfloor_249, bid128_to_uint32_xfloor, 0x303E0000000000000000000500000009u128, 2147483648   , 0x20); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_xfloor_250, bid128_to_uint32_xfloor, 0x303E000000000000000000050000000Au128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_xfloor_251, bid128_to_uint32_xfloor, 0x303E000000000000000000050000000Bu128, 2147483649   , 0x20); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_xfloor_252, bid128_to_uint32_xfloor, 0x303E00000000000000000009FFFFFFF5u128, 4294967294   , 0x20); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_xfloor_253, bid128_to_uint32_xfloor, 0x303E00000000000000000009FFFFFFF6u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_xfloor_254, bid128_to_uint32_xfloor, 0x303E00000000000000000009FFFFFFF7u128, 4294967295   , 0x20); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_xfloor_255, bid128_to_uint32_xfloor, 0x303E00000000000000000009FFFFFFFBu128, 4294967295   , 0x20); // -- 2^32-0.5
dec_test!(bid128_to_uint32_xfloor_256, bid128_to_uint32_xfloor, 0x303E00000000000000000009FFFFFFFFu128, 4294967295   , 0x20); // -- 2^32-ulp
dec_test!(bid128_to_uint32_xfloor_257, bid128_to_uint32_xfloor, 0x303E0000000000000000000A00000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_xfloor_258, bid128_to_uint32_xfloor, 0x303E0000000000000000000A00000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_xfloor_259, bid128_to_uint32_xfloor, 0x303E0000000000000000000A00000005u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_xfloor_260, bid128_to_uint32_xfloor, 0x303E0000000000000000000A00000009u128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_xfloor_261, bid128_to_uint32_xfloor, 0x303E0000000000000000000A0000000Au128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_xfloor_262, bid128_to_uint32_xfloor, 0x303E0000000000000000000A0000000Bu128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_xfloor_263, bid128_to_uint32_xfloor, 0x303E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_xfloor_264, bid128_to_uint32_xfloor, 0x303E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_xfloor_265, bid128_to_uint32_xfloor, 0x303E0000000000000000002E90EDD005u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_xfloor_266, bid128_to_uint32_xfloor, 0x303E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_xfloor_267, bid128_to_uint32_xfloor, 0x30400000000000000000000000000001u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_xfloor_268, bid128_to_uint32_xfloor, 0x3040000000000000000000000000012Bu128, 299          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_269, bid128_to_uint32_xfloor, 0x3040000000000000000000000000012Cu128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_270, bid128_to_uint32_xfloor, 0x3040000000000000000000000000012Du128, 301          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_271, bid128_to_uint32_xfloor, 0x3040000000000000000000007FFFFFFFu128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_xfloor_272, bid128_to_uint32_xfloor, 0x30400000000000000000000080000000u128, 0x80000000u32, 0x00);
dec_test!(bid128_to_uint32_xfloor_273, bid128_to_uint32_xfloor, 0x30400000000000000000000080000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_xfloor_274, bid128_to_uint32_xfloor, 0x30400000000000000000000080000001u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_xfloor_275, bid128_to_uint32_xfloor, 0x304000000000000000000000FFFFFFFFu128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_xfloor_276, bid128_to_uint32_xfloor, 0x30400000000000000000000100000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_xfloor_277, bid128_to_uint32_xfloor, 0x30400000000000000000000100000001u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_xfloor_278, bid128_to_uint32_xfloor, 0x304000000000000000000004A817C7FFu128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_xfloor_279, bid128_to_uint32_xfloor, 0x304000000000000000000004A817C801u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_xfloor_280, bid128_to_uint32_xfloor, 0x3041ED09BEAD87C0378D8E6400000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xfloor_281, bid128_to_uint32_xfloor, 0x3042000000000000000000000000001Du128, 290          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_xfloor_282, bid128_to_uint32_xfloor, 0x3042000000000000000000000000001Eu128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_283, bid128_to_uint32_xfloor, 0x3042000000000000000000000000001Fu128, 310          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_xfloor_284, bid128_to_uint32_xfloor, 0x304200000000000000000000773593FFu128, 2147483648   , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_uint32_xfloor_285, bid128_to_uint32_xfloor, 0x30420000000000000000000077359400u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_xfloor_286, bid128_to_uint32_xfloor, 0x30420000000000000000000077359401u128, 2147483648   , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_uint32_xfloor_287, bid128_to_uint32_xfloor, 0x30440000000000000000000000000003u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xfloor_288, bid128_to_uint32_xfloor, 0x30520000000000000000000000000004u128, 4000000000   , 0x00); // -- 4e9
dec_test!(bid128_to_uint32_xfloor_289, bid128_to_uint32_xfloor, 0x30520000000000000000000000000005u128, 2147483648   , 0x01); // -- 5e9
dec_test!(bid128_to_uint32_xfloor_290, bid128_to_uint32_xfloor, 0x30540000000000000000000000000002u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_xfloor_291, bid128_to_uint32_xfloor, 4294967296u64                         , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_292, bid128_to_uint32_xfloor, 0x4b2db667f824ef88376542d57019d1b8u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_293, bid128_to_uint32_xfloor, "5.05"                                , 5            , 0x20);
dec_test!(bid128_to_uint32_xfloor_294, bid128_to_uint32_xfloor, 0x50a40000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xfloor_295, bid128_to_uint32_xfloor, 0x5145bb8560dfd5748201165dc7769c43u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_296, bid128_to_uint32_xfloor, "5.5"                                 , 5            , 0x20);
dec_test!(bid128_to_uint32_xfloor_297, bid128_to_uint32_xfloor, "+75768587.877575778979E0"            , 75768587     , 0x20);
dec_test!(bid128_to_uint32_xfloor_298, bid128_to_uint32_xfloor, 0x78000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xfloor_299, bid128_to_uint32_xfloor, 0x79a6cf39c887f92d7d1f9c3fb2b0316au128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_300, bid128_to_uint32_xfloor, 0x7c000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xfloor_301, bid128_to_uint32_xfloor, 0x7c003fffffffffff38c15b08ffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_302, bid128_to_uint32_xfloor, 0x7c003fffffffffff38c15b0affffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_303, bid128_to_uint32_xfloor, 0x7e000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xfloor_304, bid128_to_uint32_xfloor, "+8.72858427458277826E0"              , 8            , 0x20);
dec_test!(bid128_to_uint32_xfloor_305, bid128_to_uint32_xfloor, "+88988.8898999889899E0"              , 88988        , 0x20);
dec_test!(bid128_to_uint32_xfloor_306, bid128_to_uint32_xfloor, "+8988.899E0"                         , 8988         , 0x20);
dec_test!(bid128_to_uint32_xfloor_307, bid128_to_uint32_xfloor, 0x8d5142ff9bcb8b7d13ac67f81e65992bu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_308, bid128_to_uint32_xfloor, 0x904a667e28ecca670737e894469a6b92u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_309, bid128_to_uint32_xfloor, 0x942e9ded505c54d1708c6c6c098eab79u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_310, bid128_to_uint32_xfloor, "+9.88E0"                             , 9            , 0x20);
dec_test!(bid128_to_uint32_xfloor_311, bid128_to_uint32_xfloor, 0xaec7e6995c749961086e505c0a9ff5e1u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_312, bid128_to_uint32_xfloor, 0xafcd9863c741069dedb49e58cadd9a3eu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_313, bid128_to_uint32_xfloor, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_314, bid128_to_uint32_xfloor, 0xAFFCF684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- -(0.5)
dec_test!(bid128_to_uint32_xfloor_315, bid128_to_uint32_xfloor, 0xAFFCF684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_316, bid128_to_uint32_xfloor, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_xfloor_317, bid128_to_uint32_xfloor, 0xAFFDEC8B86EF679D76FC433D80000000u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_xfloor_318, bid128_to_uint32_xfloor, 0xAFFDEC8B86EF679D76FC433D80000001u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_xfloor_319, bid128_to_uint32_xfloor, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_xfloor_320, bid128_to_uint32_xfloor, 0xAFFE314DC6448D9338C15B0A00000000u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_xfloor_321, bid128_to_uint32_xfloor, 0xAFFE314DC6448D9338C15B0A00000001u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_xfloor_322, bid128_to_uint32_xfloor, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_323, bid128_to_uint32_xfloor, 0xAFFE49F4A966D45CD522088F00000000u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xfloor_324, bid128_to_uint32_xfloor, 0xAFFE49F4A966D45CD522088F00000001u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_325, bid128_to_uint32_xfloor, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_326, bid128_to_uint32_xfloor, 0xB00293E952CDA8B9AA44111E00000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_327, bid128_to_uint32_xfloor, 0xB00293E952CDA8B9AA44111E00000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_328, bid128_to_uint32_xfloor, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xfloor_329, bid128_to_uint32_xfloor, 0xB00294286EACB8CB0A8CB6B140000000u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xfloor_330, bid128_to_uint32_xfloor, 0xB00294286EACB8CB0A8CB6B140000001u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xfloor_331, bid128_to_uint32_xfloor, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_332, bid128_to_uint32_xfloor, 0xB0040ECA8847C4129106CE8300000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_333, bid128_to_uint32_xfloor, 0xB0040ECA8847C4129106CE8300000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_334, bid128_to_uint32_xfloor, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_335, bid128_to_uint32_xfloor, 0xB00A0003C95A2F0B4856475FE0000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_336, bid128_to_uint32_xfloor, 0xB00A0003C95A2F0B4856475FE0000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_337, bid128_to_uint32_xfloor, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_338, bid128_to_uint32_xfloor, 0xB00C000060EF6B1ABA6F072330000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_339, bid128_to_uint32_xfloor, 0xB00C000060EF6B1ABA6F072330000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_340, bid128_to_uint32_xfloor, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_341, bid128_to_uint32_xfloor, 0xB01069E10DE628D3A6C9CC9B8E800000u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xfloor_342, bid128_to_uint32_xfloor, 0xB01069E10DE628D3A6C9CC9B8E800001u128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_343, bid128_to_uint32_xfloor, 0xB01069E10DE692B4B4B133125EFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xfloor_344, bid128_to_uint32_xfloor, 0xB01069E10DE692B4B4B133125F000000u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_xfloor_345, bid128_to_uint32_xfloor, 0xB01069E10DE692B4B4B133125F000001u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xfloor_346, bid128_to_uint32_xfloor, 0xB01069E10DE6FC95C29899892F7FFFFFu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_347, bid128_to_uint32_xfloor, 0xB01069E10DE6FC95C29899892F800000u128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xfloor_348, bid128_to_uint32_xfloor, 0xB01069E10DE6FC95C29899892F800001u128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_349, bid128_to_uint32_xfloor, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_xfloor_350, bid128_to_uint32_xfloor, 0xB01069E10DE76676D080000000000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_xfloor_351, bid128_to_uint32_xfloor, 0xB01069E10DE76676D080000000000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_xfloor_352, bid128_to_uint32_xfloor, 0xB01069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_353, bid128_to_uint32_xfloor, 0xB01069E10DE7D057DE676676D0800000u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xfloor_354, bid128_to_uint32_xfloor, 0xB01069E10DE7D057DE676676D0800001u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_355, bid128_to_uint32_xfloor, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xfloor_356, bid128_to_uint32_xfloor, 0xB01069E10DE83A38EC4ECCEDA1000000u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_xfloor_357, bid128_to_uint32_xfloor, 0xB01069E10DE83A38EC4ECCEDA1000001u128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xfloor_358, bid128_to_uint32_xfloor, 0xB010C5371912364CE3056C27FFFFFFFFu128, 2147483648   , 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_uint32_xfloor_359, bid128_to_uint32_xfloor, 0xB010C5371912364CE3056C2800000000u128, 2147483648   , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint32_xfloor_360, bid128_to_uint32_xfloor, 0xB010C5371912364CE3056C2800000001u128, 2147483648   , 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_uint32_xfloor_361, bid128_to_uint32_xfloor, 0xB010D3C21BCDF92B853133125EFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xfloor_362, bid128_to_uint32_xfloor, 0xB010D3C21BCDF92B853133125F000000u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_xfloor_363, bid128_to_uint32_xfloor, 0xB010D3C21BCDF92B853133125F000001u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xfloor_364, bid128_to_uint32_xfloor, 0xB010D3C21BCE630C931899892F7FFFFFu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_365, bid128_to_uint32_xfloor, 0xB010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xfloor_366, bid128_to_uint32_xfloor, 0xB010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_367, bid128_to_uint32_xfloor, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_xfloor_368, bid128_to_uint32_xfloor, 0xB010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_xfloor_369, bid128_to_uint32_xfloor, 0xB010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_xfloor_370, bid128_to_uint32_xfloor, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_371, bid128_to_uint32_xfloor, 0xB010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xfloor_372, bid128_to_uint32_xfloor, 0xB010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_373, bid128_to_uint32_xfloor, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xfloor_374, bid128_to_uint32_xfloor, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_xfloor_375, bid128_to_uint32_xfloor, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xfloor_376, bid128_to_uint32_xfloor, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_uint32_xfloor_377, bid128_to_uint32_xfloor, 0xB010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint32_xfloor_378, bid128_to_uint32_xfloor, 0xB010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_uint32_xfloor_379, bid128_to_uint32_xfloor, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_380, bid128_to_uint32_xfloor, 0xB012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_xfloor_381, bid128_to_uint32_xfloor, 0xB012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_382, bid128_to_uint32_xfloor, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_xfloor_383, bid128_to_uint32_xfloor, 0xB012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_xfloor_384, bid128_to_uint32_xfloor, 0xB012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_xfloor_385, bid128_to_uint32_xfloor, 0xB012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_386, bid128_to_uint32_xfloor, 0xB012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_xfloor_387, bid128_to_uint32_xfloor, 0xB012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_388, bid128_to_uint32_xfloor, 0xB012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_xfloor_389, bid128_to_uint32_xfloor, 0xB012629B8C891B267182B61400000000u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_xfloor_390, bid128_to_uint32_xfloor, 0xB012629B8C891B267182B61400000001u128, 2147483648   , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_xfloor_391, bid128_to_uint32_xfloor, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_392, bid128_to_uint32_xfloor, 0xB012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_xfloor_393, bid128_to_uint32_xfloor, 0xB012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_394, bid128_to_uint32_xfloor, 0xB012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_xfloor_395, bid128_to_uint32_xfloor, 0xB012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_xfloor_396, bid128_to_uint32_xfloor, 0xB012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_xfloor_397, bid128_to_uint32_xfloor, 0xB012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_398, bid128_to_uint32_xfloor, 0xB012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_xfloor_399, bid128_to_uint32_xfloor, 0xB012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_400, bid128_to_uint32_xfloor, 0xB01600000000003627E8F712373BFFFFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_xfloor_401, bid128_to_uint32_xfloor, 0xB01600000000003627E8F712373C0000u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_xfloor_402, bid128_to_uint32_xfloor, 0xB01600000000003627E8F712373C0001u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_xfloor_403, bid128_to_uint32_xfloor, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_404, bid128_to_uint32_xfloor, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xfloor_405, bid128_to_uint32_xfloor, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_406, bid128_to_uint32_xfloor, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_407, bid128_to_uint32_xfloor, 0xB0180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xfloor_408, bid128_to_uint32_xfloor, 0xB0180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_409, bid128_to_uint32_xfloor, 0xB0180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_410, bid128_to_uint32_xfloor, 0xB0180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xfloor_411, bid128_to_uint32_xfloor, 0xB0180002B5E3AF19676BAF16B1880001u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_412, bid128_to_uint32_xfloor, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_413, bid128_to_uint32_xfloor, 0xB01800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xfloor_414, bid128_to_uint32_xfloor, 0xB01800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_415, bid128_to_uint32_xfloor, 0xB01800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_416, bid128_to_uint32_xfloor, 0xB01800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xfloor_417, bid128_to_uint32_xfloor, 0xB01800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_418, bid128_to_uint32_xfloor, 0xB01A0000000000004563918244F3FFFFu128, 2147483648   , 0x01); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_419, bid128_to_uint32_xfloor, 0xB01A0000000000004563918244F40000u128, 2147483648   , 0x01); // -- -(0.5)
dec_test!(bid128_to_uint32_xfloor_420, bid128_to_uint32_xfloor, 0xB01A0000000000004563918244F40001u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_421, bid128_to_uint32_xfloor, 0xB01A0000000000008AC7230489E7FFFFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_xfloor_422, bid128_to_uint32_xfloor, 0xB01A0000000000008AC7230489E80000u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_xfloor_423, bid128_to_uint32_xfloor, 0xB01A0000000000008AC7230489E80001u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_xfloor_424, bid128_to_uint32_xfloor, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xfloor_425, bid128_to_uint32_xfloor, 0xB01A0000000000A2E6C09AD3E0D40000u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xfloor_426, bid128_to_uint32_xfloor, 0xB01A0000000000A2E6C09AD3E0D40001u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xfloor_427, bid128_to_uint32_xfloor, 0xB01A000045639181BA2CDCFB7617FFFFu128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xfloor_428, bid128_to_uint32_xfloor, 0xB01A000045639181BA2CDCFB76180000u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_xfloor_429, bid128_to_uint32_xfloor, 0xB01A000045639181BA2CDCFB76180001u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xfloor_430, bid128_to_uint32_xfloor, 0xB01A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_xfloor_431, bid128_to_uint32_xfloor, 0xB01A00004563918244F4000000000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_xfloor_432, bid128_to_uint32_xfloor, 0xB01A00004563918244F4000000000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_xfloor_433, bid128_to_uint32_xfloor, 0xB01A000045639182CFBB230489E7FFFFu128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xfloor_434, bid128_to_uint32_xfloor, 0xB01A000045639182CFBB230489E80000u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_xfloor_435, bid128_to_uint32_xfloor, 0xB01A000045639182CFBB230489E80001u128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xfloor_436, bid128_to_uint32_xfloor, 0xB01A00008AC72303FF20DCFB7617FFFFu128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xfloor_437, bid128_to_uint32_xfloor, 0xB01A00008AC72303FF20DCFB76180000u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_xfloor_438, bid128_to_uint32_xfloor, 0xB01A00008AC72303FF20DCFB76180001u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xfloor_439, bid128_to_uint32_xfloor, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_xfloor_440, bid128_to_uint32_xfloor, 0xB01A00008AC7230489E8000000000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_xfloor_441, bid128_to_uint32_xfloor, 0xB01A00008AC7230489E8000000000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_xfloor_442, bid128_to_uint32_xfloor, 0xB01A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xfloor_443, bid128_to_uint32_xfloor, 0xB01A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_xfloor_444, bid128_to_uint32_xfloor, 0xB01A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xfloor_445, bid128_to_uint32_xfloor, 0xB01C00000000000014D1120D7B15FFFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_446, bid128_to_uint32_xfloor, 0xB01C00000000000014D1120D7B160000u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xfloor_447, bid128_to_uint32_xfloor, 0xB01C00000000000014D1120D7B160001u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_448, bid128_to_uint32_xfloor, 0xB01E000000000001A055690D9DB7FFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_449, bid128_to_uint32_xfloor, 0xB01E000000000001A055690D9DB80000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_450, bid128_to_uint32_xfloor, 0xB01E000000000001A055690D9DB80001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_451, bid128_to_uint32_xfloor, 0xB02000000000000029A2241AF62BFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_452, bid128_to_uint32_xfloor, 0xB02000000000000029A2241AF62C0000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_453, bid128_to_uint32_xfloor, 0xB02000000000000029A2241AF62C0001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_454, bid128_to_uint32_xfloor, 0xB024000000000000006A94D74F42FFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_455, bid128_to_uint32_xfloor, 0xB024000000000000006A94D74F430000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_456, bid128_to_uint32_xfloor, 0xB024000000000000006A94D74F430001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_457, bid128_to_uint32_xfloor, 0xB02A00000000000000000017428106FFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_xfloor_458, bid128_to_uint32_xfloor, 0xB02A0000000000000000001742810700u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_xfloor_459, bid128_to_uint32_xfloor, 0xB02A0000000000000000001742810701u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_xfloor_460, bid128_to_uint32_xfloor, 0xB02A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_461, bid128_to_uint32_xfloor, 0xB02A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_xfloor_462, bid128_to_uint32_xfloor, 0xB02A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_463, bid128_to_uint32_xfloor, 0xB02A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_464, bid128_to_uint32_xfloor, 0xB02A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_xfloor_465, bid128_to_uint32_xfloor, 0xB02A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_466, bid128_to_uint32_xfloor, 0xB02C000000000000000002BBA7F521FFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xfloor_467, bid128_to_uint32_xfloor, 0xB02C000000000000000002BBA7F52200u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xfloor_468, bid128_to_uint32_xfloor, 0xB02C000000000000000002BBA7F52201u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xfloor_469, bid128_to_uint32_xfloor, 0xB02C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_xfloor_470, bid128_to_uint32_xfloor, 0xB02C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_xfloor_471, bid128_to_uint32_xfloor, 0xB02C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_xfloor_472, bid128_to_uint32_xfloor, 0xB02C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_473, bid128_to_uint32_xfloor, 0xB02C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_xfloor_474, bid128_to_uint32_xfloor, 0xB02C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_475, bid128_to_uint32_xfloor, 0xB02C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_476, bid128_to_uint32_xfloor, 0xB02C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_xfloor_477, bid128_to_uint32_xfloor, 0xB02C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_478, bid128_to_uint32_xfloor, 0xB02E000000000000000000001DCD64FFu128, 2147483648   , 0x01); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_479, bid128_to_uint32_xfloor, 0xB02E000000000000000000001DCD6500u128, 2147483648   , 0x01); // -- -(0.5)
dec_test!(bid128_to_uint32_xfloor_480, bid128_to_uint32_xfloor, 0xB02E000000000000000000001DCD6501u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_481, bid128_to_uint32_xfloor, 0xB02E000000000000000000003B9AC9FFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_xfloor_482, bid128_to_uint32_xfloor, 0xB02E000000000000000000003B9ACA00u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_xfloor_483, bid128_to_uint32_xfloor, 0xB02E000000000000000000003B9ACA01u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_xfloor_484, bid128_to_uint32_xfloor, 0xB02E0000000000000000000059682EFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_485, bid128_to_uint32_xfloor, 0xB02E0000000000000000000059682F00u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xfloor_486, bid128_to_uint32_xfloor, 0xB02E0000000000000000000059682F01u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_487, bid128_to_uint32_xfloor, 0xB02E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_xfloor_488, bid128_to_uint32_xfloor, 0xB02E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_xfloor_489, bid128_to_uint32_xfloor, 0xB02E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_xfloor_490, bid128_to_uint32_xfloor, 0xB03000000000000000000006FC23ABFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_491, bid128_to_uint32_xfloor, 0xB03000000000000000000006FC23AC00u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_492, bid128_to_uint32_xfloor, 0xB03000000000000000000006FC23AC01u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_493, bid128_to_uint32_xfloor, 0xB03200000000000000000000B2D05DFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_494, bid128_to_uint32_xfloor, 0xB03200000000000000000000B2D05E00u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_495, bid128_to_uint32_xfloor, 0xB03200000000000000000000B2D05E01u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_496, bid128_to_uint32_xfloor, 0xB03800000000000000000000002DDA47u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xfloor_497, bid128_to_uint32_xfloor, 0xB03800000000000000000000002DDA48u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xfloor_498, bid128_to_uint32_xfloor, 0xB03800000000000000000000002DDA49u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xfloor_499, bid128_to_uint32_xfloor, 0xB03A00000000000000000000000003E7u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_xfloor_500, bid128_to_uint32_xfloor, 0xB03A00000000000000000000000005DBu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_501, bid128_to_uint32_xfloor, 0xB03A00000000000000000000000005DCu128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xfloor_502, bid128_to_uint32_xfloor, 0xB03A00000000000000000000000005DDu128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_503, bid128_to_uint32_xfloor, 0xB03A00000000000000000000000495D3u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xfloor_504, bid128_to_uint32_xfloor, 0xB03A00000000000000000000000495D4u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xfloor_505, bid128_to_uint32_xfloor, 0xB03A00000000000000000000000495D5u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xfloor_506, bid128_to_uint32_xfloor, 0xB03C0000000000000000000000000095u128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_507, bid128_to_uint32_xfloor, 0xB03C0000000000000000000000000096u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xfloor_508, bid128_to_uint32_xfloor, 0xB03C0000000000000000000000000097u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_509, bid128_to_uint32_xfloor, 0xB03C0000000000000000000000007561u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xfloor_510, bid128_to_uint32_xfloor, 0xB03C0000000000000000000000007562u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xfloor_511, bid128_to_uint32_xfloor, 0xB03C0000000000000000000000007563u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xfloor_512, bid128_to_uint32_xfloor, 0xB03C00000000000000000031FFFFFF69u128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xfloor_513, bid128_to_uint32_xfloor, 0xB03C00000000000000000031FFFFFF6Au128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xfloor_514, bid128_to_uint32_xfloor, 0xB03C00000000000000000031FFFFFF6Bu128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xfloor_515, bid128_to_uint32_xfloor, 0xB03C00000000000000000031FFFFFFCDu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_516, bid128_to_uint32_xfloor, 0xB03C00000000000000000031FFFFFFCEu128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xfloor_517, bid128_to_uint32_xfloor, 0xB03C00000000000000000031FFFFFFCFu128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_518, bid128_to_uint32_xfloor, 0xB03C0000000000000000003200000031u128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_519, bid128_to_uint32_xfloor, 0xB03C0000000000000000003200000032u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xfloor_520, bid128_to_uint32_xfloor, 0xB03C0000000000000000003200000033u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_521, bid128_to_uint32_xfloor, 0xB03C00000000000000000063FFFFFFCDu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_522, bid128_to_uint32_xfloor, 0xB03C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xfloor_523, bid128_to_uint32_xfloor, 0xB03C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_524, bid128_to_uint32_xfloor, 0xB03C0000000000000000006400000031u128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xfloor_525, bid128_to_uint32_xfloor, 0xB03C0000000000000000006400000032u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xfloor_526, bid128_to_uint32_xfloor, 0xB03C0000000000000000006400000033u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xfloor_527, bid128_to_uint32_xfloor, 0xB03E0000000000000000000000000005u128, 2147483648   , 0x01); // -- -(0.5)
dec_test!(bid128_to_uint32_xfloor_528, bid128_to_uint32_xfloor, 0xB03E000000000000000000000000000Fu128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xfloor_529, bid128_to_uint32_xfloor, 0xB03E0000000000000000000000000BB7u128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_530, bid128_to_uint32_xfloor, 0xB03E0000000000000000000000000BB8u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_531, bid128_to_uint32_xfloor, 0xB03E0000000000000000000000000BB9u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_532, bid128_to_uint32_xfloor, 0xB03E0000000000000000000000000BBDu128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xfloor_533, bid128_to_uint32_xfloor, 0xB03E00000000000000000004FFFFFFF1u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xfloor_534, bid128_to_uint32_xfloor, 0xB03E00000000000000000004FFFFFFF5u128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xfloor_535, bid128_to_uint32_xfloor, 0xB03E00000000000000000004FFFFFFF6u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_xfloor_536, bid128_to_uint32_xfloor, 0xB03E00000000000000000004FFFFFFF7u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xfloor_537, bid128_to_uint32_xfloor, 0xB03E00000000000000000004FFFFFFFBu128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xfloor_538, bid128_to_uint32_xfloor, 0xB03E00000000000000000004FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_xfloor_539, bid128_to_uint32_xfloor, 0xB03E0000000000000000000500000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_xfloor_540, bid128_to_uint32_xfloor, 0xB03E0000000000000000000500000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_xfloor_541, bid128_to_uint32_xfloor, 0xB03E0000000000000000000500000005u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xfloor_542, bid128_to_uint32_xfloor, 0xB03E0000000000000000000500000009u128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xfloor_543, bid128_to_uint32_xfloor, 0xB03E000000000000000000050000000Au128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_xfloor_544, bid128_to_uint32_xfloor, 0xB03E000000000000000000050000000Bu128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xfloor_545, bid128_to_uint32_xfloor, 0xB03E00000000000000000009FFFFFFF5u128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xfloor_546, bid128_to_uint32_xfloor, 0xB03E00000000000000000009FFFFFFF6u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_xfloor_547, bid128_to_uint32_xfloor, 0xB03E00000000000000000009FFFFFFF7u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xfloor_548, bid128_to_uint32_xfloor, 0xB03E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xfloor_549, bid128_to_uint32_xfloor, 0xB03E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_xfloor_550, bid128_to_uint32_xfloor, 0xB03E0000000000000000000A00000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_xfloor_551, bid128_to_uint32_xfloor, 0xB03E0000000000000000000A00000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_xfloor_552, bid128_to_uint32_xfloor, 0xB03E0000000000000000000A00000005u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xfloor_553, bid128_to_uint32_xfloor, 0xB03E0000000000000000000A00000009u128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xfloor_554, bid128_to_uint32_xfloor, 0xB03E0000000000000000000A0000000Au128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_xfloor_555, bid128_to_uint32_xfloor, 0xB03E0000000000000000000A0000000Bu128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xfloor_556, bid128_to_uint32_xfloor, 0xB03E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_xfloor_557, bid128_to_uint32_xfloor, 0xB03E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_xfloor_558, bid128_to_uint32_xfloor, 0xB03E0000000000000000002E90EDD005u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_xfloor_559, bid128_to_uint32_xfloor, 0xB03E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_xfloor_560, bid128_to_uint32_xfloor, 0xB0400000000000000000000000000001u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_xfloor_561, bid128_to_uint32_xfloor, 0xB040000000000000000000000000012Bu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_562, bid128_to_uint32_xfloor, 0xB040000000000000000000000000012Cu128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_563, bid128_to_uint32_xfloor, 0xB040000000000000000000000000012Du128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_564, bid128_to_uint32_xfloor, 0xB040000000000000000000007FFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_xfloor_565, bid128_to_uint32_xfloor, 0xB0400000000000000000000080000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_xfloor_566, bid128_to_uint32_xfloor, 0xB0400000000000000000000080000001u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_xfloor_567, bid128_to_uint32_xfloor, 0xB04000000000000000000000FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_xfloor_568, bid128_to_uint32_xfloor, 0xB0400000000000000000000100000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_xfloor_569, bid128_to_uint32_xfloor, 0xB0400000000000000000000100000001u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_xfloor_570, bid128_to_uint32_xfloor, 0xB04000000000000000000004A817C7FFu128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_xfloor_571, bid128_to_uint32_xfloor, 0xB04000000000000000000004A817C801u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_xfloor_572, bid128_to_uint32_xfloor, 0xB042000000000000000000000000001Du128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xfloor_573, bid128_to_uint32_xfloor, 0xB042000000000000000000000000001Eu128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_574, bid128_to_uint32_xfloor, 0xB042000000000000000000000000001Fu128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xfloor_575, bid128_to_uint32_xfloor, 0xB04200000000000000000000773593FFu128, 2147483648   , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_xfloor_576, bid128_to_uint32_xfloor, 0xB0420000000000000000000077359400u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_xfloor_577, bid128_to_uint32_xfloor, 0xB0420000000000000000000077359401u128, 2147483648   , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_xfloor_578, bid128_to_uint32_xfloor, 0xB0440000000000000000000000000003u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xfloor_579, bid128_to_uint32_xfloor, 0xB0520000000000000000000000000004u128, 2147483648   , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint32_xfloor_580, bid128_to_uint32_xfloor, 0xB0520000000000000000000000000005u128, 2147483648   , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint32_xfloor_581, bid128_to_uint32_xfloor, 0xB0540000000000000000000000000002u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_xfloor_582, bid128_to_uint32_xfloor, 0xcfa2f974b7b6d9b6000215318c204105u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_583, bid128_to_uint32_xfloor, 0xd6e33c41da0335587dca5cdf53114a37u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_584, bid128_to_uint32_xfloor, 0xe68640cdc2f46e19bc59861e4d97e33fu128, 0            , 0x00);
dec_test!(bid128_to_uint32_xfloor_585, bid128_to_uint32_xfloor, 0xf8000000000000000000000000000000u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_586, bid128_to_uint32_xfloor, 0xfbeedbdf79dd5fdbfdffff7fffedffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_587, bid128_to_uint32_xfloor, 0xfc09872e2fed566d0080000000080000u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_588, bid128_to_uint32_xfloor, 0xfffffdfdbffff9fbd03d1c818b97b637u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_589, bid128_to_uint32_xfloor, "-Infinity"                           , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_590, bid128_to_uint32_xfloor, "Infinity"                            , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_591, bid128_to_uint32_xfloor, "QNaN"                                , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xfloor_592, bid128_to_uint32_xfloor, "SNaN"                                , 0x80000000u32, 0x01);