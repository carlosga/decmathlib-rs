/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int32_floor_001, bid128_to_int32_floor, "-0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_floor_002, bid128_to_int32_floor,  "0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_floor_003, bid128_to_int32_floor, 0x00000000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_004, bid128_to_int32_floor, 0x00000000000000000000040004000010u128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_005, bid128_to_int32_floor, 0x00000000000000006610d0133b800304u128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_006, bid128_to_int32_floor, 0x0001ed09bead87c0378d8e62ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_007, bid128_to_int32_floor, 0x0001ed09bead87c0378d8e64ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_008, bid128_to_int32_floor, 0x02000002000800803f73efdafde7ae7bu128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_009, bid128_to_int32_floor, 0x0400000000000000fffffffeffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_010, bid128_to_int32_floor, 0x075e0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_011, bid128_to_int32_floor, "1.0"                                 , 1          , 0x00);
dec_test!(bid128_to_int32_floor_012, bid128_to_int32_floor, "1073741824"                          , 1073741824 , 0x00);
dec_test!(bid128_to_int32_floor_013, bid128_to_int32_floor, "1"                                   , 1          , 0x00);
dec_test!(bid128_to_int32_floor_014, bid128_to_int32_floor, "2147483648"                          , -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_015, bid128_to_int32_floor, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_int32_floor_016, bid128_to_int32_floor, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_floor_017, bid128_to_int32_floor, 0x2FFCF684DF56C3E01BC6C73200000001u128, 0          , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_int32_floor_018, bid128_to_int32_floor, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 0          , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_int32_floor_019, bid128_to_int32_floor, 0x2FFDEC8B86EF679D76FC433D80000000u128, 0          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_floor_020, bid128_to_int32_floor, 0x2FFDEC8B86EF679D76FC433D80000001u128, 0          , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_int32_floor_021, bid128_to_int32_floor, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 0          , 0x00); // -- 1-ulp
dec_test!(bid128_to_int32_floor_022, bid128_to_int32_floor, 0x2FFE314DC6448D9338C15B0A00000000u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_floor_023, bid128_to_int32_floor, 0x2FFE314DC6448D9338C15B0A00000001u128, 1          , 0x00); // -- 1+ulp
dec_test!(bid128_to_int32_floor_024, bid128_to_int32_floor, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1          , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_int32_floor_025, bid128_to_int32_floor, 0x2FFE49F4A966D45CD522088F00000000u128, 1          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_floor_026, bid128_to_int32_floor, 0x2FFE49F4A966D45CD522088F00000001u128, 1          , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_int32_floor_027, bid128_to_int32_floor, 0x2fff9e5fffeadbef0008724400210002u128, 8          , 0x00);
dec_test!(bid128_to_int32_floor_028, bid128_to_int32_floor, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_029, bid128_to_int32_floor, 0x300293E952CDA8B9AA44111E00000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_030, bid128_to_int32_floor, 0x300293E952CDA8B9AA44111E00000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_031, bid128_to_int32_floor, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_floor_032, bid128_to_int32_floor, 0x300294286EACB8CB0A8CB6B140000000u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_floor_033, bid128_to_int32_floor, 0x300294286EACB8CB0A8CB6B140000001u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_floor_034, bid128_to_int32_floor, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_035, bid128_to_int32_floor, 0x30040ECA8847C4129106CE8300000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_036, bid128_to_int32_floor, 0x30040ECA8847C4129106CE8300000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_037, bid128_to_int32_floor, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_038, bid128_to_int32_floor, 0x300A0003C95A2F0B4856475FE0000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_039, bid128_to_int32_floor, 0x300A0003C95A2F0B4856475FE0000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_040, bid128_to_int32_floor, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_041, bid128_to_int32_floor, 0x300C000060EF6B1ABA6F072330000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_042, bid128_to_int32_floor, 0x300C000060EF6B1ABA6F072330000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_043, bid128_to_int32_floor, 0x300e7768669bd191b6573e76bfb7096fu128, 242187822  , 0x00);
dec_test!(bid128_to_int32_floor_044, bid128_to_int32_floor, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_floor_045, bid128_to_int32_floor, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483646 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_floor_046, bid128_to_int32_floor, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483646 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_floor_047, bid128_to_int32_floor, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483646 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_floor_048, bid128_to_int32_floor, 0x301069E10DE692B4B4B133125F000000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_floor_049, bid128_to_int32_floor, 0x301069E10DE692B4B4B133125F000001u128, 2147483647 , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_floor_050, bid128_to_int32_floor, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647 , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_floor_051, bid128_to_int32_floor, 0x301069E10DE6FC95C29899892F800000u128, 2147483647 , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_int32_floor_052, bid128_to_int32_floor, 0x301069E10DE6FC95C29899892F800001u128, 2147483647 , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_floor_053, bid128_to_int32_floor, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, 2147483647 , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_int32_floor_054, bid128_to_int32_floor, 0x301069E10DE76676D080000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_floor_055, bid128_to_int32_floor, 0x301069E10DE76676D080000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_floor_056, bid128_to_int32_floor, 0x301069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_floor_057, bid128_to_int32_floor, 0x301069E10DE7D057DE676676D0800000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_floor_058, bid128_to_int32_floor, 0x301069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_floor_059, bid128_to_int32_floor, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_floor_060, bid128_to_int32_floor, 0x301069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_floor_061, bid128_to_int32_floor, 0x301069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_floor_062, bid128_to_int32_floor, 0x3010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- 4e9-ulp
dec_test!(bid128_to_int32_floor_063, bid128_to_int32_floor, 0x3010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_floor_064, bid128_to_int32_floor, 0x3010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- 4e9+ulp
dec_test!(bid128_to_int32_floor_065, bid128_to_int32_floor, 0x3010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_floor_066, bid128_to_int32_floor, 0x3010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_floor_067, bid128_to_int32_floor, 0x3010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_floor_068, bid128_to_int32_floor, 0x3010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_floor_069, bid128_to_int32_floor, 0x3010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_floor_070, bid128_to_int32_floor, 0x3010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_floor_071, bid128_to_int32_floor, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_floor_072, bid128_to_int32_floor, 0x3010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_floor_073, bid128_to_int32_floor, 0x3010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_floor_074, bid128_to_int32_floor, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_floor_075, bid128_to_int32_floor, 0x3010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_floor_076, bid128_to_int32_floor, 0x3010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_floor_077, bid128_to_int32_floor, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_floor_078, bid128_to_int32_floor, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_floor_079, bid128_to_int32_floor, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_floor_080, bid128_to_int32_floor, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- 5e9-ulp
dec_test!(bid128_to_int32_floor_081, bid128_to_int32_floor, 0x3010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_floor_082, bid128_to_int32_floor, 0x3010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- 5e9+ulp
dec_test!(bid128_to_int32_floor_083, bid128_to_int32_floor, 0x301193a68058f4c2a100288064021012u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_084, bid128_to_int32_floor, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_floor_085, bid128_to_int32_floor, 0x3012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_floor_086, bid128_to_int32_floor, 0x3012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_floor_087, bid128_to_int32_floor, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_floor_088, bid128_to_int32_floor, 0x3012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_floor_089, bid128_to_int32_floor, 0x3012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_floor_090, bid128_to_int32_floor, 0x3012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_floor_091, bid128_to_int32_floor, 0x3012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_floor_092, bid128_to_int32_floor, 0x3012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_floor_093, bid128_to_int32_floor, 0x3012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_floor_094, bid128_to_int32_floor, 0x3012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_floor_095, bid128_to_int32_floor, 0x3012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_floor_096, bid128_to_int32_floor, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_floor_097, bid128_to_int32_floor, 0x3012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_floor_098, bid128_to_int32_floor, 0x3012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_floor_099, bid128_to_int32_floor, 0x3012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_floor_100, bid128_to_int32_floor, 0x3012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_floor_101, bid128_to_int32_floor, 0x3012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_floor_102, bid128_to_int32_floor, 0x3012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_floor_103, bid128_to_int32_floor, 0x3012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_floor_104, bid128_to_int32_floor, 0x3012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_floor_105, bid128_to_int32_floor, 0x3016000000202031fffbffff6fffffffu128, 38837      , 0x00);
dec_test!(bid128_to_int32_floor_106, bid128_to_int32_floor, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_floor_107, bid128_to_int32_floor, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483646 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_floor_108, bid128_to_int32_floor, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483646 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_floor_109, bid128_to_int32_floor, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647 , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_floor_110, bid128_to_int32_floor, 0x30180002B5E3AF13FBA450E94E780000u128, 2147483647 , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_int32_floor_111, bid128_to_int32_floor, 0x30180002B5E3AF13FBA450E94E780001u128, 2147483647 , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_floor_112, bid128_to_int32_floor, 0x30180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_floor_113, bid128_to_int32_floor, 0x30180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_floor_114, bid128_to_int32_floor, 0x30180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_floor_115, bid128_to_int32_floor, 0x301800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_floor_116, bid128_to_int32_floor, 0x301800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_floor_117, bid128_to_int32_floor, 0x301800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_floor_118, bid128_to_int32_floor, 0x301800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_floor_119, bid128_to_int32_floor, 0x301800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_floor_120, bid128_to_int32_floor, 0x301800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_floor_121, bid128_to_int32_floor, 0x3018000c0d141800ffbf7bffffbffeffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_122, bid128_to_int32_floor, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_floor_123, bid128_to_int32_floor, 0x301A0000000000A2E6C09AD3E0D40000u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_floor_124, bid128_to_int32_floor, 0x301A0000000000A2E6C09AD3E0D40001u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_floor_125, bid128_to_int32_floor, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483646 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_floor_126, bid128_to_int32_floor, 0x301A000045639181BA2CDCFB76180000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_floor_127, bid128_to_int32_floor, 0x301A000045639181BA2CDCFB76180001u128, 2147483647 , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_floor_128, bid128_to_int32_floor, 0x301A00004563918244F3FFFFFFFFFFFFu128, 2147483647 , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_int32_floor_129, bid128_to_int32_floor, 0x301A00004563918244F4000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_floor_130, bid128_to_int32_floor, 0x301A00004563918244F4000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_floor_131, bid128_to_int32_floor, 0x301A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_floor_132, bid128_to_int32_floor, 0x301A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_floor_133, bid128_to_int32_floor, 0x301A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_floor_134, bid128_to_int32_floor, 0x301A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_floor_135, bid128_to_int32_floor, 0x301A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_floor_136, bid128_to_int32_floor, 0x301A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_floor_137, bid128_to_int32_floor, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_floor_138, bid128_to_int32_floor, 0x301A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_floor_139, bid128_to_int32_floor, 0x301A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_floor_140, bid128_to_int32_floor, 0x301A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_floor_141, bid128_to_int32_floor, 0x301A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_floor_142, bid128_to_int32_floor, 0x301A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_floor_143, bid128_to_int32_floor, 0x301E000000000001A055690D9DB7FFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_144, bid128_to_int32_floor, 0x301E000000000001A055690D9DB80000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_145, bid128_to_int32_floor, 0x301E000000000001A055690D9DB80001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_146, bid128_to_int32_floor, 0x302000000000000029A2241AF62BFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_147, bid128_to_int32_floor, 0x302000000000000029A2241AF62C0000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_148, bid128_to_int32_floor, 0x302000000000000029A2241AF62C0001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_149, bid128_to_int32_floor, 0x302000000000800023724bb255e92dfdu128, 60446546   , 0x00);
dec_test!(bid128_to_int32_floor_150, bid128_to_int32_floor, 0x3024000000000000006A94D74F42FFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_151, bid128_to_int32_floor, 0x3024000000000000006A94D74F430000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_152, bid128_to_int32_floor, 0x3024000000000000006A94D74F430001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_153, bid128_to_int32_floor, 0x302A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_floor_154, bid128_to_int32_floor, 0x302A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_floor_155, bid128_to_int32_floor, 0x302A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_floor_156, bid128_to_int32_floor, 0x302A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_floor_157, bid128_to_int32_floor, 0x302A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_floor_158, bid128_to_int32_floor, 0x302A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_floor_159, bid128_to_int32_floor, 0x302C000000000000000002BBA7F521FFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_floor_160, bid128_to_int32_floor, 0x302C000000000000000002BBA7F52200u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_floor_161, bid128_to_int32_floor, 0x302C000000000000000002BBA7F52201u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_floor_162, bid128_to_int32_floor, 0x302C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_floor_163, bid128_to_int32_floor, 0x302C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_floor_164, bid128_to_int32_floor, 0x302C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_floor_165, bid128_to_int32_floor, 0x302C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_floor_166, bid128_to_int32_floor, 0x302C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_floor_167, bid128_to_int32_floor, 0x302C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_floor_168, bid128_to_int32_floor, 0x302C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_floor_169, bid128_to_int32_floor, 0x302C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_floor_170, bid128_to_int32_floor, 0x302C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_floor_171, bid128_to_int32_floor, 0x302E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_floor_172, bid128_to_int32_floor, 0x302E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_floor_173, bid128_to_int32_floor, 0x302E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_floor_174, bid128_to_int32_floor, 0x303000000000000000000006FC23ABFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_175, bid128_to_int32_floor, 0x303000000000000000000006FC23AC00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_176, bid128_to_int32_floor, 0x303000000000000000000006FC23AC01u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_177, bid128_to_int32_floor, 0x303200000000000000000000B2D05DFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_178, bid128_to_int32_floor, 0x303200000000000000000000B2D05E00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_179, bid128_to_int32_floor, 0x303200000000000000000000B2D05E01u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_180, bid128_to_int32_floor, 0x303800000000000000000000002DDA47u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_floor_181, bid128_to_int32_floor, 0x303800000000000000000000002DDA48u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_floor_182, bid128_to_int32_floor, 0x303800000000000000000000002DDA49u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_floor_183, bid128_to_int32_floor, 0x303A00000000000000000000000003E7u128, 0          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_floor_184, bid128_to_int32_floor, 0x303A00000000000000000000000495D3u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_floor_185, bid128_to_int32_floor, 0x303A00000000000000000000000495D4u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_floor_186, bid128_to_int32_floor, 0x303A00000000000000000000000495D5u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_floor_187, bid128_to_int32_floor, 0x303C0000000000000000000000007561u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_floor_188, bid128_to_int32_floor, 0x303C0000000000000000000000007562u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_floor_189, bid128_to_int32_floor, 0x303C0000000000000000000000007563u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_floor_190, bid128_to_int32_floor, 0x303E0000000000000000000000000005u128, 0          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_floor_191, bid128_to_int32_floor, 0x303E000000000000000000000000000Fu128, 1          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_floor_192, bid128_to_int32_floor, 0x303E0000000000000000000000000BB7u128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_193, bid128_to_int32_floor, 0x303E0000000000000000000000000BB8u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_194, bid128_to_int32_floor, 0x303E0000000000000000000000000BB9u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_195, bid128_to_int32_floor, 0x303E0000000000000000000000000BBDu128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_floor_196, bid128_to_int32_floor, 0x303E00000000000000000004FFFFFFF1u128, 2147483646 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_floor_197, bid128_to_int32_floor, 0x303E00000000000000000004FFFFFFFBu128, 2147483647 , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_int32_floor_198, bid128_to_int32_floor, 0x303E0000000000000000000500000005u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_floor_199, bid128_to_int32_floor, 0x303E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_floor_200, bid128_to_int32_floor, 0x303E0000000000000000000A00000005u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_floor_201, bid128_to_int32_floor, 0x303E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_floor_202, bid128_to_int32_floor, 0x303E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_floor_203, bid128_to_int32_floor, 0x303E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_floor_204, bid128_to_int32_floor, 0x303E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_floor_205, bid128_to_int32_floor, 0x30400000000000000000000000000001u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_floor_206, bid128_to_int32_floor, 0x3040000000000000000000000000012Bu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_207, bid128_to_int32_floor, 0x3040000000000000000000000000012Cu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_208, bid128_to_int32_floor, 0x3040000000000000000000000000012Du128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_209, bid128_to_int32_floor, 0x3040000000000000000000007FFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_floor_210, bid128_to_int32_floor, 0x30400000000000000000000080000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_floor_211, bid128_to_int32_floor, 0x30400000000000000000000080000001u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_floor_212, bid128_to_int32_floor, 0x304000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_floor_213, bid128_to_int32_floor, 0x30400000000000000000000100000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_floor_214, bid128_to_int32_floor, 0x30400000000000000000000100000001u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_floor_215, bid128_to_int32_floor, 0x304000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_floor_216, bid128_to_int32_floor, 0x304000000000000000000004A817C801u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_floor_217, bid128_to_int32_floor, 0x3041ED09BEAD87C0378D8E6400000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_218, bid128_to_int32_floor, 0x3042000000000000000000000000001Du128, 290        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_floor_219, bid128_to_int32_floor, 0x3042000000000000000000000000001Eu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_220, bid128_to_int32_floor, 0x3042000000000000000000000000001Fu128, 310        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_floor_221, bid128_to_int32_floor, 0x304200000000000000000000773593FFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_floor_222, bid128_to_int32_floor, 0x30420000000000000000000077359400u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_floor_223, bid128_to_int32_floor, 0x30420000000000000000000077359401u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_floor_224, bid128_to_int32_floor, 0x30440000000000000000000000000003u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_floor_225, bid128_to_int32_floor, 0x30520000000000000000000000000004u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_floor_226, bid128_to_int32_floor, 0x30520000000000000000000000000005u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_floor_227, bid128_to_int32_floor, 0x30540000000000000000000000000002u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_floor_228, bid128_to_int32_floor, 0x45b40000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_229, bid128_to_int32_floor, "5.05"                                , 5          , 0x00);
dec_test!(bid128_to_int32_floor_230, bid128_to_int32_floor, "5.5"                                 , 5          , 0x00);
dec_test!(bid128_to_int32_floor_231, bid128_to_int32_floor, "-6.69887795686678786E0"              , -7         , 0x00);
dec_test!(bid128_to_int32_floor_232, bid128_to_int32_floor, 0x712e5910a82fe6ba9f8bf498780c9ad4u128, 0          , 0x00);
dec_test!(bid128_to_int32_floor_233, bid128_to_int32_floor, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_234, bid128_to_int32_floor, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_235, bid128_to_int32_floor, 0x7994c6d5d34928bce8d503302f51f278u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_236, bid128_to_int32_floor, 0x7c000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_237, bid128_to_int32_floor, 0x7c003fffffffffff38c15b08ffffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_238, bid128_to_int32_floor, 0x7c003fffffffffff38c15b0affffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_239, bid128_to_int32_floor, 0x7c5e7954ba9123c3615ecf4a27a20ef6u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_240, bid128_to_int32_floor, 0x7e000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_241, bid128_to_int32_floor, 0x7e000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_242, bid128_to_int32_floor, 0x7fdefd3de9fdfff96efafffe7dfdeffdu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_243, bid128_to_int32_floor, 0x97a5a7f69948f1389da2ea208613a566u128, -1         , 0x00);
dec_test!(bid128_to_int32_floor_244, bid128_to_int32_floor, 0x9f0e8b3b56699b01bf8ebfff7bf705beu128, -1         , 0x00);
dec_test!(bid128_to_int32_floor_245, bid128_to_int32_floor, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, -1         , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_int32_floor_246, bid128_to_int32_floor, 0xAFFCF684DF56C3E01BC6C73200000000u128, -1         , 0x00); // -- -(0.5)
dec_test!(bid128_to_int32_floor_247, bid128_to_int32_floor, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1         , 0x00); // -- -(0.5+ulp)
dec_test!(bid128_to_int32_floor_248, bid128_to_int32_floor, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1         , 0x00); // -- -(0.999-ulp)
dec_test!(bid128_to_int32_floor_249, bid128_to_int32_floor, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1         , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_floor_250, bid128_to_int32_floor, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1         , 0x00); // -- -(0.999+ulp)
dec_test!(bid128_to_int32_floor_251, bid128_to_int32_floor, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1         , 0x00); // -- -(1-ulp)
dec_test!(bid128_to_int32_floor_252, bid128_to_int32_floor, 0xAFFE314DC6448D9338C15B0A00000000u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_floor_253, bid128_to_int32_floor, 0xAFFE314DC6448D9338C15B0A00000001u128, -2         , 0x00); // -- -(1+ulp)
dec_test!(bid128_to_int32_floor_254, bid128_to_int32_floor, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -2         , 0x00); // -- -(1.5-ulp)
dec_test!(bid128_to_int32_floor_255, bid128_to_int32_floor, 0xAFFE49F4A966D45CD522088F00000000u128, -2         , 0x00); // -- -(1.5)
dec_test!(bid128_to_int32_floor_256, bid128_to_int32_floor, 0xAFFE49F4A966D45CD522088F00000001u128, -2         , 0x00); // -- -(1.5+ulp)
dec_test!(bid128_to_int32_floor_257, bid128_to_int32_floor, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_258, bid128_to_int32_floor, 0xB00293E952CDA8B9AA44111E00000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_259, bid128_to_int32_floor, 0xB00293E952CDA8B9AA44111E00000001u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_260, bid128_to_int32_floor, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -301       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_floor_261, bid128_to_int32_floor, 0xB00294286EACB8CB0A8CB6B140000000u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_floor_262, bid128_to_int32_floor, 0xB00294286EACB8CB0A8CB6B140000001u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_floor_263, bid128_to_int32_floor, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_264, bid128_to_int32_floor, 0xB0040ECA8847C4129106CE8300000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_265, bid128_to_int32_floor, 0xB0040ECA8847C4129106CE8300000001u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_266, bid128_to_int32_floor, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_267, bid128_to_int32_floor, 0xB00A0003C95A2F0B4856475FE0000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_268, bid128_to_int32_floor, 0xB00A0003C95A2F0B4856475FE0000001u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_269, bid128_to_int32_floor, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_270, bid128_to_int32_floor, 0xB00C000060EF6B1ABA6F072330000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_271, bid128_to_int32_floor, 0xB00C000060EF6B1ABA6F072330000001u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_272, bid128_to_int32_floor, 0xb00c59421d179059e5b2cb6508cc45d6u128, -18103726  , 0x00);
dec_test!(bid128_to_int32_floor_273, bid128_to_int32_floor, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, -2147483647, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_floor_274, bid128_to_int32_floor, 0xB01069E10DE628D3A6C9CC9B8E800000u128, -2147483647, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_floor_275, bid128_to_int32_floor, 0xB01069E10DE628D3A6C9CC9B8E800001u128, -2147483647, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_floor_276, bid128_to_int32_floor, 0xB01069E10DE692B4B4B133125EFFFFFFu128, -2147483647, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_floor_277, bid128_to_int32_floor, 0xB01069E10DE692B4B4B133125F000000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_floor_278, bid128_to_int32_floor, 0xB01069E10DE692B4B4B133125F000001u128, -2147483648, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_floor_279, bid128_to_int32_floor, 0xB01069E10DE6FC95C29899892F7FFFFFu128, -2147483648, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_floor_280, bid128_to_int32_floor, 0xB01069E10DE6FC95C29899892F800000u128, -2147483648, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_floor_281, bid128_to_int32_floor, 0xB01069E10DE6FC95C29899892F800001u128, -2147483648, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_floor_282, bid128_to_int32_floor, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, -2147483648, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_floor_283, bid128_to_int32_floor, 0xB01069E10DE76676D080000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_floor_284, bid128_to_int32_floor, 0xB01069E10DE76676D080000000000001u128, -2147483648, 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_floor_285, bid128_to_int32_floor, 0xB01069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_floor_286, bid128_to_int32_floor, 0xB01069E10DE7D057DE676676D0800000u128, -2147483648, 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_floor_287, bid128_to_int32_floor, 0xB01069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_floor_288, bid128_to_int32_floor, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_floor_289, bid128_to_int32_floor, 0xB01069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_floor_290, bid128_to_int32_floor, 0xB01069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_floor_291, bid128_to_int32_floor, 0xB010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_int32_floor_292, bid128_to_int32_floor, 0xB010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_floor_293, bid128_to_int32_floor, 0xB010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_int32_floor_294, bid128_to_int32_floor, 0xB010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_floor_295, bid128_to_int32_floor, 0xB010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_floor_296, bid128_to_int32_floor, 0xB010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_floor_297, bid128_to_int32_floor, 0xB010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_floor_298, bid128_to_int32_floor, 0xB010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_floor_299, bid128_to_int32_floor, 0xB010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_floor_300, bid128_to_int32_floor, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_floor_301, bid128_to_int32_floor, 0xB010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_floor_302, bid128_to_int32_floor, 0xB010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_floor_303, bid128_to_int32_floor, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_floor_304, bid128_to_int32_floor, 0xB010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_floor_305, bid128_to_int32_floor, 0xB010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_floor_306, bid128_to_int32_floor, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_floor_307, bid128_to_int32_floor, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_floor_308, bid128_to_int32_floor, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_floor_309, bid128_to_int32_floor, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_int32_floor_310, bid128_to_int32_floor, 0xB010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_floor_311, bid128_to_int32_floor, 0xB010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_int32_floor_312, bid128_to_int32_floor, 0xb0112c889fb9980580a048004509e84au128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_313, bid128_to_int32_floor, 0xb01208a3e7b48298de66fcd2b776fdc9u128, -1752451766, 0x00);
dec_test!(bid128_to_int32_floor_314, bid128_to_int32_floor, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_floor_315, bid128_to_int32_floor, 0xB012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_floor_316, bid128_to_int32_floor, 0xB012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_floor_317, bid128_to_int32_floor, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_floor_318, bid128_to_int32_floor, 0xB012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_floor_319, bid128_to_int32_floor, 0xB012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_floor_320, bid128_to_int32_floor, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_floor_321, bid128_to_int32_floor, 0xB012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_floor_322, bid128_to_int32_floor, 0xB012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_floor_323, bid128_to_int32_floor, 0xB012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_floor_324, bid128_to_int32_floor, 0xB012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_floor_325, bid128_to_int32_floor, 0xB012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_floor_326, bid128_to_int32_floor, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_floor_327, bid128_to_int32_floor, 0xB012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_floor_328, bid128_to_int32_floor, 0xB012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_floor_329, bid128_to_int32_floor, 0xB012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_floor_330, bid128_to_int32_floor, 0xB012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_floor_331, bid128_to_int32_floor, 0xB012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_floor_332, bid128_to_int32_floor, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_floor_333, bid128_to_int32_floor, 0xB012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_floor_334, bid128_to_int32_floor, 0xB012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_floor_335, bid128_to_int32_floor, 0xb018000200122051ff77ffafff2fff7fu128, -1584782384, 0x00);
dec_test!(bid128_to_int32_floor_336, bid128_to_int32_floor, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, -2147483647, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_floor_337, bid128_to_int32_floor, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, -2147483647, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_floor_338, bid128_to_int32_floor, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, -2147483647, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_floor_339, bid128_to_int32_floor, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, -2147483648, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_floor_340, bid128_to_int32_floor, 0xB0180002B5E3AF13FBA450E94E780000u128, -2147483648, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_floor_341, bid128_to_int32_floor, 0xB0180002B5E3AF13FBA450E94E780001u128, -2147483648, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_floor_342, bid128_to_int32_floor, 0xB0180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_floor_343, bid128_to_int32_floor, 0xB0180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_floor_344, bid128_to_int32_floor, 0xB0180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_floor_345, bid128_to_int32_floor, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_floor_346, bid128_to_int32_floor, 0xB01800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_floor_347, bid128_to_int32_floor, 0xB01800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_floor_348, bid128_to_int32_floor, 0xB01800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_floor_349, bid128_to_int32_floor, 0xB01800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_floor_350, bid128_to_int32_floor, 0xB01800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_floor_351, bid128_to_int32_floor, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -301       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_floor_352, bid128_to_int32_floor, 0xB01A0000000000A2E6C09AD3E0D40000u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_floor_353, bid128_to_int32_floor, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_floor_354, bid128_to_int32_floor, 0xB01A000045639181BA2CDCFB7617FFFFu128, -2147483647, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_floor_355, bid128_to_int32_floor, 0xB01A000045639181BA2CDCFB76180000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_floor_356, bid128_to_int32_floor, 0xB01A000045639181BA2CDCFB76180001u128, -2147483648, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_floor_357, bid128_to_int32_floor, 0xB01A00004563918244F3FFFFFFFFFFFFu128, -2147483648, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_floor_358, bid128_to_int32_floor, 0xB01A00004563918244F4000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_floor_359, bid128_to_int32_floor, 0xB01A00004563918244F4000000000001u128, -2147483648, 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_floor_360, bid128_to_int32_floor, 0xB01A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_floor_361, bid128_to_int32_floor, 0xB01A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_floor_362, bid128_to_int32_floor, 0xB01A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_floor_363, bid128_to_int32_floor, 0xB01A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_floor_364, bid128_to_int32_floor, 0xB01A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_floor_365, bid128_to_int32_floor, 0xB01A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_floor_366, bid128_to_int32_floor, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_floor_367, bid128_to_int32_floor, 0xB01A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_floor_368, bid128_to_int32_floor, 0xB01A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_floor_369, bid128_to_int32_floor, 0xB01A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_floor_370, bid128_to_int32_floor, 0xB01A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_floor_371, bid128_to_int32_floor, 0xB01A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_floor_372, bid128_to_int32_floor, 0xB01E000000000001A055690D9DB7FFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_373, bid128_to_int32_floor, 0xB01E000000000001A055690D9DB80000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_374, bid128_to_int32_floor, 0xB01E000000000001A055690D9DB80001u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_375, bid128_to_int32_floor, 0xB02000000000000029A2241AF62BFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_376, bid128_to_int32_floor, 0xB02000000000000029A2241AF62C0000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_377, bid128_to_int32_floor, 0xB02000000000000029A2241AF62C0001u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_378, bid128_to_int32_floor, 0xB024000000000000006A94D74F42FFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_379, bid128_to_int32_floor, 0xB024000000000000006A94D74F430000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_380, bid128_to_int32_floor, 0xB024000000000000006A94D74F430001u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_381, bid128_to_int32_floor, 0xB02A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_floor_382, bid128_to_int32_floor, 0xB02A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_floor_383, bid128_to_int32_floor, 0xB02A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_floor_384, bid128_to_int32_floor, 0xB02A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_floor_385, bid128_to_int32_floor, 0xB02A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_floor_386, bid128_to_int32_floor, 0xB02A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_floor_387, bid128_to_int32_floor, 0xB02C000000000000000002BBA7F521FFu128, -301       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_floor_388, bid128_to_int32_floor, 0xB02C000000000000000002BBA7F52200u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_floor_389, bid128_to_int32_floor, 0xB02C000000000000000002BBA7F52201u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_floor_390, bid128_to_int32_floor, 0xB02C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_floor_391, bid128_to_int32_floor, 0xB02C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_floor_392, bid128_to_int32_floor, 0xB02C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_floor_393, bid128_to_int32_floor, 0xB02C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_floor_394, bid128_to_int32_floor, 0xB02C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_floor_395, bid128_to_int32_floor, 0xB02C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_floor_396, bid128_to_int32_floor, 0xB02C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_floor_397, bid128_to_int32_floor, 0xB02C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_floor_398, bid128_to_int32_floor, 0xB02C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_floor_399, bid128_to_int32_floor, 0xB02E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_floor_400, bid128_to_int32_floor, 0xB02E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_floor_401, bid128_to_int32_floor, 0xB02E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_floor_402, bid128_to_int32_floor, 0xB03000000000000000000006FC23ABFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_403, bid128_to_int32_floor, 0xB03000000000000000000006FC23AC00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_404, bid128_to_int32_floor, 0xB03000000000000000000006FC23AC01u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_405, bid128_to_int32_floor, 0xB03200000000000000000000B2D05DFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_406, bid128_to_int32_floor, 0xB03200000000000000000000B2D05E00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_407, bid128_to_int32_floor, 0xB03200000000000000000000B2D05E01u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_408, bid128_to_int32_floor, 0xB03800000000000000000000002DDA47u128, -301       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_floor_409, bid128_to_int32_floor, 0xB03800000000000000000000002DDA48u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_floor_410, bid128_to_int32_floor, 0xB03800000000000000000000002DDA49u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_floor_411, bid128_to_int32_floor, 0xB03A00000000000000000000000003E7u128, -1         , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_floor_412, bid128_to_int32_floor, 0xB03A00000000000000000000000495D3u128, -301       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_floor_413, bid128_to_int32_floor, 0xB03A00000000000000000000000495D4u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_floor_414, bid128_to_int32_floor, 0xB03A00000000000000000000000495D5u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_floor_415, bid128_to_int32_floor, 0xB03C0000000000000000000000007561u128, -301       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_floor_416, bid128_to_int32_floor, 0xB03C0000000000000000000000007562u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_floor_417, bid128_to_int32_floor, 0xB03C0000000000000000000000007563u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_floor_418, bid128_to_int32_floor, 0xB03E0000000000000000000000000005u128, -1         , 0x00); // -- -(0.5)
dec_test!(bid128_to_int32_floor_419, bid128_to_int32_floor, 0xB03E000000000000000000000000000Fu128, -2         , 0x00); // -- -(1.5)
dec_test!(bid128_to_int32_floor_420, bid128_to_int32_floor, 0xB03E0000000000000000000000000BB7u128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_421, bid128_to_int32_floor, 0xB03E0000000000000000000000000BB8u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_422, bid128_to_int32_floor, 0xB03E0000000000000000000000000BB9u128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_423, bid128_to_int32_floor, 0xB03E0000000000000000000000000BBDu128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_floor_424, bid128_to_int32_floor, 0xB03E00000000000000000004FFFFFFF1u128, -2147483647, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_floor_425, bid128_to_int32_floor, 0xB03E00000000000000000004FFFFFFFBu128, -2147483648, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_floor_426, bid128_to_int32_floor, 0xB03E0000000000000000000500000005u128, -2147483648, 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_floor_427, bid128_to_int32_floor, 0xB03E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_floor_428, bid128_to_int32_floor, 0xB03E0000000000000000000A00000005u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_floor_429, bid128_to_int32_floor, 0xB03E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_floor_430, bid128_to_int32_floor, 0xB03E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_floor_431, bid128_to_int32_floor, 0xB03E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_floor_432, bid128_to_int32_floor, 0xB03E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_floor_433, bid128_to_int32_floor, 0xB0400000000000000000000000000001u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_floor_434, bid128_to_int32_floor, 0xB040000000000000000000000000012Bu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_435, bid128_to_int32_floor, 0xB040000000000000000000000000012Cu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_436, bid128_to_int32_floor, 0xB040000000000000000000000000012Du128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_437, bid128_to_int32_floor, 0xb0400000000000000000000000006100u128, -24832     , 0x00);
dec_test!(bid128_to_int32_floor_438, bid128_to_int32_floor, 0xB040000000000000000000007FFFFFFFu128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_floor_439, bid128_to_int32_floor, 0xB0400000000000000000000080000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_floor_440, bid128_to_int32_floor, 0xB0400000000000000000000080000001u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_floor_441, bid128_to_int32_floor, 0xB04000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_floor_442, bid128_to_int32_floor, 0xB0400000000000000000000100000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_floor_443, bid128_to_int32_floor, 0xB0400000000000000000000100000001u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_floor_444, bid128_to_int32_floor, 0xB04000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_floor_445, bid128_to_int32_floor, 0xB04000000000000000000004A817C801u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_floor_446, bid128_to_int32_floor, 0xB042000000000000000000000000001Du128, -290       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_floor_447, bid128_to_int32_floor, 0xB042000000000000000000000000001Eu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_448, bid128_to_int32_floor, 0xB042000000000000000000000000001Fu128, -310       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_floor_449, bid128_to_int32_floor, 0xB04200000000000000000000773593FFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_floor_450, bid128_to_int32_floor, 0xB0420000000000000000000077359400u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_floor_451, bid128_to_int32_floor, 0xB0420000000000000000000077359401u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_floor_452, bid128_to_int32_floor, 0xB0440000000000000000000000000003u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_floor_453, bid128_to_int32_floor, 0xB0520000000000000000000000000004u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_floor_454, bid128_to_int32_floor, 0xB0520000000000000000000000000005u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_floor_455, bid128_to_int32_floor, 0xB0540000000000000000000000000002u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_floor_456, bid128_to_int32_floor, 0xb7881c2dfea4f8a0a1f42bb168953548u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_457, bid128_to_int32_floor, 0xc09c634c31f58ebb7656793ffe3b0660u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_458, bid128_to_int32_floor, 0xc17ec4d75cfdb9dbdd6db66fce1dad32u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_459, bid128_to_int32_floor, 0xcb4dd07a1a9a849a17e3b1bdebd51564u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_460, bid128_to_int32_floor, 0xcfc40fa61ff4de484b1435237d5f3a74u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_461, bid128_to_int32_floor, 0xdc86ca6974f6083d89620a1751399443u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_462, bid128_to_int32_floor, 0xfbefdf6d2fffa6ffef5fdff81bff7cf7u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_463, bid128_to_int32_floor, 0xfc000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_464, bid128_to_int32_floor, "-Infinity"                           , -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_465, bid128_to_int32_floor, "QNaN"                                , -2147483648, 0x01);
dec_test!(bid128_to_int32_floor_466, bid128_to_int32_floor, "SNaN"                                , -2147483648, 0x01);