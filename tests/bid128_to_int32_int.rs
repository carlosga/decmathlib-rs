/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int32_int_001, bid128_to_int32_int, "-0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_int_002, bid128_to_int32_int,  "0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_int_003, bid128_to_int32_int, 0x00000000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_004, bid128_to_int32_int, 0x00000000000000000000002000100000u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_005, bid128_to_int32_int, 0x0000000000000000cd8842fcb6bc5b7eu128, 0          , 0x00);
dec_test!(bid128_to_int32_int_006, bid128_to_int32_int, 0x00000000010040800000000000800000u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_007, bid128_to_int32_int, 0x0001ed09bead87c0378d8e62ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_int_008, bid128_to_int32_int, 0x0001ed09bead87c0378d8e64ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_int_009, bid128_to_int32_int, "1.0"                                 , 1          , 0x00);
dec_test!(bid128_to_int32_int_010, bid128_to_int32_int, "1073741824"                          , 1073741824 , 0x00);
dec_test!(bid128_to_int32_int_011, bid128_to_int32_int, "1"                                   , 1          , 0x00);
dec_test!(bid128_to_int32_int_012, bid128_to_int32_int, 0x1b7b39ff8d6a089e87a3e129e01f6c37u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_013, bid128_to_int32_int, 0x1bf97452eb9fbfb890e22ee467c25e5bu128, 0          , 0x00);
dec_test!(bid128_to_int32_int_014, bid128_to_int32_int, 0x1c6a0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_015, bid128_to_int32_int, "2147483648"                          , -2147483648, 0x01);
dec_test!(bid128_to_int32_int_016, bid128_to_int32_int, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_int32_int_017, bid128_to_int32_int, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_int_018, bid128_to_int32_int, 0x2FFCF684DF56C3E01BC6C73200000001u128, 0          , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_int32_int_019, bid128_to_int32_int, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 0          , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_int32_int_020, bid128_to_int32_int, 0x2FFDEC8B86EF679D76FC433D80000000u128, 0          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_int_021, bid128_to_int32_int, 0x2FFDEC8B86EF679D76FC433D80000001u128, 0          , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_int32_int_022, bid128_to_int32_int, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 0          , 0x00); // -- 1-ulp
dec_test!(bid128_to_int32_int_023, bid128_to_int32_int, 0x2FFE314DC6448D9338C15B0A00000000u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_int_024, bid128_to_int32_int, 0x2FFE314DC6448D9338C15B0A00000001u128, 1          , 0x00); // -- 1+ulp
dec_test!(bid128_to_int32_int_025, bid128_to_int32_int, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1          , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_int32_int_026, bid128_to_int32_int, 0x2FFE49F4A966D45CD522088F00000000u128, 1          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_int_027, bid128_to_int32_int, 0x2FFE49F4A966D45CD522088F00000001u128, 1          , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_int32_int_028, bid128_to_int32_int, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_029, bid128_to_int32_int, 0x300293E952CDA8B9AA44111E00000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_030, bid128_to_int32_int, 0x300293E952CDA8B9AA44111E00000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_031, bid128_to_int32_int, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_int_032, bid128_to_int32_int, 0x300294286EACB8CB0A8CB6B140000000u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_int_033, bid128_to_int32_int, 0x300294286EACB8CB0A8CB6B140000001u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_int_034, bid128_to_int32_int, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_035, bid128_to_int32_int, 0x30040ECA8847C4129106CE8300000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_036, bid128_to_int32_int, 0x30040ECA8847C4129106CE8300000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_037, bid128_to_int32_int, 0x3004b32e479d24addcfe149692a5065du128, 3634       , 0x00);
dec_test!(bid128_to_int32_int_038, bid128_to_int32_int, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_039, bid128_to_int32_int, 0x300A0003C95A2F0B4856475FE0000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_040, bid128_to_int32_int, 0x300A0003C95A2F0B4856475FE0000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_041, bid128_to_int32_int, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_042, bid128_to_int32_int, 0x300C000060EF6B1ABA6F072330000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_043, bid128_to_int32_int, 0x300C000060EF6B1ABA6F072330000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_044, bid128_to_int32_int, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_int_045, bid128_to_int32_int, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483646 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_int_046, bid128_to_int32_int, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483646 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_int_047, bid128_to_int32_int, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483646 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_int_048, bid128_to_int32_int, 0x301069E10DE692B4B4B133125F000000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_int_049, bid128_to_int32_int, 0x301069E10DE692B4B4B133125F000001u128, 2147483647 , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_int_050, bid128_to_int32_int, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647 , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_int_051, bid128_to_int32_int, 0x301069E10DE6FC95C29899892F800000u128, 2147483647 , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_int32_int_052, bid128_to_int32_int, 0x301069E10DE6FC95C29899892F800001u128, 2147483647 , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_int_053, bid128_to_int32_int, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, 2147483647 , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_int32_int_054, bid128_to_int32_int, 0x301069E10DE76676D080000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_int_055, bid128_to_int32_int, 0x301069E10DE76676D080000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_int_056, bid128_to_int32_int, 0x301069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_int_057, bid128_to_int32_int, 0x301069E10DE7D057DE676676D0800000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_int_058, bid128_to_int32_int, 0x301069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_int_059, bid128_to_int32_int, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_int_060, bid128_to_int32_int, 0x301069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_int_061, bid128_to_int32_int, 0x301069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_int_062, bid128_to_int32_int, 0x3010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- 4e9-ulp
dec_test!(bid128_to_int32_int_063, bid128_to_int32_int, 0x3010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_int_064, bid128_to_int32_int, 0x3010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- 4e9+ulp
dec_test!(bid128_to_int32_int_065, bid128_to_int32_int, 0x3010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_int_066, bid128_to_int32_int, 0x3010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_int_067, bid128_to_int32_int, 0x3010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_int_068, bid128_to_int32_int, 0x3010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_int_069, bid128_to_int32_int, 0x3010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_int_070, bid128_to_int32_int, 0x3010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_int_071, bid128_to_int32_int, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_int_072, bid128_to_int32_int, 0x3010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_int_073, bid128_to_int32_int, 0x3010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_int_074, bid128_to_int32_int, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_int_075, bid128_to_int32_int, 0x3010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_int_076, bid128_to_int32_int, 0x3010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_int_077, bid128_to_int32_int, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_int_078, bid128_to_int32_int, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_int_079, bid128_to_int32_int, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_int_080, bid128_to_int32_int, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- 5e9-ulp
dec_test!(bid128_to_int32_int_081, bid128_to_int32_int, 0x3010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_int_082, bid128_to_int32_int, 0x3010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- 5e9+ulp
dec_test!(bid128_to_int32_int_083, bid128_to_int32_int, 0x3011a40044000004e2e883e5753ece0fu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_084, bid128_to_int32_int, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_int_085, bid128_to_int32_int, 0x3012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_int_086, bid128_to_int32_int, 0x3012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_int_087, bid128_to_int32_int, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_int_088, bid128_to_int32_int, 0x3012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_int_089, bid128_to_int32_int, 0x3012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_int_090, bid128_to_int32_int, 0x3012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_int_091, bid128_to_int32_int, 0x3012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_int_092, bid128_to_int32_int, 0x3012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_int_093, bid128_to_int32_int, 0x3012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_int_094, bid128_to_int32_int, 0x3012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_int_095, bid128_to_int32_int, 0x3012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_int_096, bid128_to_int32_int, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_int_097, bid128_to_int32_int, 0x3012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_int_098, bid128_to_int32_int, 0x3012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_int_099, bid128_to_int32_int, 0x3012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_int_100, bid128_to_int32_int, 0x3012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_int_101, bid128_to_int32_int, 0x3012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_int_102, bid128_to_int32_int, 0x3012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_int_103, bid128_to_int32_int, 0x3012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_int_104, bid128_to_int32_int, 0x3012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_int_105, bid128_to_int32_int, 0x30140000000441100000000000000000u128, 514        , 0x00);
dec_test!(bid128_to_int32_int_106, bid128_to_int32_int, 0x3014000056101000fd88bc71eeeae550u128, 2663512    , 0x00);
dec_test!(bid128_to_int32_int_107, bid128_to_int32_int, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_int_108, bid128_to_int32_int, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483646 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_int_109, bid128_to_int32_int, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483646 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_int_110, bid128_to_int32_int, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647 , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_int_111, bid128_to_int32_int, 0x30180002B5E3AF13FBA450E94E780000u128, 2147483647 , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_int32_int_112, bid128_to_int32_int, 0x30180002B5E3AF13FBA450E94E780001u128, 2147483647 , 0x00); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_int_113, bid128_to_int32_int, 0x30180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_int_114, bid128_to_int32_int, 0x30180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_int_115, bid128_to_int32_int, 0x30180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_int_116, bid128_to_int32_int, 0x301800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_int_117, bid128_to_int32_int, 0x301800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_int_118, bid128_to_int32_int, 0x301800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_int_119, bid128_to_int32_int, 0x301800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_int_120, bid128_to_int32_int, 0x301800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_int_121, bid128_to_int32_int, 0x301800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_int_122, bid128_to_int32_int, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_int_123, bid128_to_int32_int, 0x301A0000000000A2E6C09AD3E0D40000u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_int_124, bid128_to_int32_int, 0x301A0000000000A2E6C09AD3E0D40001u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_int_125, bid128_to_int32_int, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483646 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_int_126, bid128_to_int32_int, 0x301A000045639181BA2CDCFB76180000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_int_127, bid128_to_int32_int, 0x301A000045639181BA2CDCFB76180001u128, 2147483647 , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_int_128, bid128_to_int32_int, 0x301A00004563918244F3FFFFFFFFFFFFu128, 2147483647 , 0x00); // -- 2^31-ulp
dec_test!(bid128_to_int32_int_129, bid128_to_int32_int, 0x301A00004563918244F4000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_int_130, bid128_to_int32_int, 0x301A00004563918244F4000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_int_131, bid128_to_int32_int, 0x301A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_int_132, bid128_to_int32_int, 0x301A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_int_133, bid128_to_int32_int, 0x301A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_int_134, bid128_to_int32_int, 0x301A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_int_135, bid128_to_int32_int, 0x301A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_int_136, bid128_to_int32_int, 0x301A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_int_137, bid128_to_int32_int, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_int_138, bid128_to_int32_int, 0x301A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_int_139, bid128_to_int32_int, 0x301A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_int_140, bid128_to_int32_int, 0x301A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_int_141, bid128_to_int32_int, 0x301A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_int_142, bid128_to_int32_int, 0x301A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_int_143, bid128_to_int32_int, 0x301E000000000001A055690D9DB7FFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_144, bid128_to_int32_int, 0x301E000000000001A055690D9DB80000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_145, bid128_to_int32_int, 0x301E000000000001A055690D9DB80001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_146, bid128_to_int32_int, 0x302000000000000029A2241AF62BFFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_147, bid128_to_int32_int, 0x302000000000000029A2241AF62C0000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_148, bid128_to_int32_int, 0x302000000000000029A2241AF62C0001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_149, bid128_to_int32_int, 0x3020000000010000ff57ffffeffffffdu128, 120894421  , 0x00);
dec_test!(bid128_to_int32_int_150, bid128_to_int32_int, 0x30200000002000400408028006040001u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_151, bid128_to_int32_int, 0x3024000000000000006A94D74F42FFFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_152, bid128_to_int32_int, 0x3024000000000000006A94D74F430000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_153, bid128_to_int32_int, 0x3024000000000000006A94D74F430001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_154, bid128_to_int32_int, 0x302A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_int_155, bid128_to_int32_int, 0x302A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_int_156, bid128_to_int32_int, 0x302A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_int_157, bid128_to_int32_int, 0x302A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_int_158, bid128_to_int32_int, 0x302A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_int_159, bid128_to_int32_int, 0x302A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_int_160, bid128_to_int32_int, 0x302C000000000000000002BBA7F521FFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_int_161, bid128_to_int32_int, 0x302C000000000000000002BBA7F52200u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_int_162, bid128_to_int32_int, 0x302C000000000000000002BBA7F52201u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_int_163, bid128_to_int32_int, 0x302C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_int_164, bid128_to_int32_int, 0x302C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_int_165, bid128_to_int32_int, 0x302C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_int_166, bid128_to_int32_int, 0x302C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_int_167, bid128_to_int32_int, 0x302C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_int_168, bid128_to_int32_int, 0x302C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_int_169, bid128_to_int32_int, 0x302C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_int_170, bid128_to_int32_int, 0x302C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_int_171, bid128_to_int32_int, 0x302C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_int_172, bid128_to_int32_int, 0x302E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_int_173, bid128_to_int32_int, 0x302E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_int_174, bid128_to_int32_int, 0x302E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_int_175, bid128_to_int32_int, 0x303000000000000000000006FC23ABFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_176, bid128_to_int32_int, 0x303000000000000000000006FC23AC00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_177, bid128_to_int32_int, 0x303000000000000000000006FC23AC01u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_178, bid128_to_int32_int, 0x303200000000000000000000B2D05DFFu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_179, bid128_to_int32_int, 0x303200000000000000000000B2D05E00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_180, bid128_to_int32_int, 0x303200000000000000000000B2D05E01u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_181, bid128_to_int32_int, 0x303800000000000000000000002DDA47u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_int_182, bid128_to_int32_int, 0x303800000000000000000000002DDA48u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_int_183, bid128_to_int32_int, 0x303800000000000000000000002DDA49u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_int_184, bid128_to_int32_int, 0x303A00000000000000000000000003E7u128, 0          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_int_185, bid128_to_int32_int, 0x303A00000000000000000000000495D3u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_int_186, bid128_to_int32_int, 0x303A00000000000000000000000495D4u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_int_187, bid128_to_int32_int, 0x303A00000000000000000000000495D5u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_int_188, bid128_to_int32_int, 0x303C0000000000000000000000007561u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_int_189, bid128_to_int32_int, 0x303C0000000000000000000000007562u128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_int_190, bid128_to_int32_int, 0x303C0000000000000000000000007563u128, 300        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_int_191, bid128_to_int32_int, 0x303E0000000000000000000000000005u128, 0          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_int_192, bid128_to_int32_int, 0x303E000000000000000000000000000Fu128, 1          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_int_193, bid128_to_int32_int, 0x303E0000000000000000000000000BB7u128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_194, bid128_to_int32_int, 0x303E0000000000000000000000000BB8u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_195, bid128_to_int32_int, 0x303E0000000000000000000000000BB9u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_196, bid128_to_int32_int, 0x303E0000000000000000000000000BBDu128, 300        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_int_197, bid128_to_int32_int, 0x303E00000000000000000004FFFFFFF1u128, 2147483646 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_int_198, bid128_to_int32_int, 0x303E00000000000000000004FFFFFFFBu128, 2147483647 , 0x00); // -- 2^31-0.5
dec_test!(bid128_to_int32_int_199, bid128_to_int32_int, 0x303E0000000000000000000500000005u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_int_200, bid128_to_int32_int, 0x303E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_int_201, bid128_to_int32_int, 0x303E0000000000000000000A00000005u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_int_202, bid128_to_int32_int, 0x303E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_int_203, bid128_to_int32_int, 0x303E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_int_204, bid128_to_int32_int, 0x303E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_int_205, bid128_to_int32_int, 0x303E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_int_206, bid128_to_int32_int, 0x30400000000000000000000000000001u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_int_207, bid128_to_int32_int, 0x3040000000000000000000000000012Bu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_208, bid128_to_int32_int, 0x3040000000000000000000000000012Cu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_209, bid128_to_int32_int, 0x3040000000000000000000000000012Du128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_210, bid128_to_int32_int, 0x3040000000000000000000007FFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_int_211, bid128_to_int32_int, 0x30400000000000000000000080000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_int_212, bid128_to_int32_int, 0x30400000000000000000000080000001u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_int_213, bid128_to_int32_int, 0x304000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_int_214, bid128_to_int32_int, 0x30400000000000000000000100000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_int_215, bid128_to_int32_int, 0x30400000000000000000000100000001u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_int_216, bid128_to_int32_int, 0x304000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_int_217, bid128_to_int32_int, 0x304000000000000000000004A817C801u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_int_218, bid128_to_int32_int, 0x3041ED09BEAD87C0378D8E6400000000u128, 0          , 00);
dec_test!(bid128_to_int32_int_219, bid128_to_int32_int, 0x3042000000000000000000000000001Du128, 290        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_int_220, bid128_to_int32_int, 0x3042000000000000000000000000001Eu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_221, bid128_to_int32_int, 0x3042000000000000000000000000001Fu128, 310        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_int_222, bid128_to_int32_int, 0x304200000000000000000000773593FFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_int_223, bid128_to_int32_int, 0x30420000000000000000000077359400u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_int_224, bid128_to_int32_int, 0x30420000000000000000000077359401u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_int_225, bid128_to_int32_int, 0x30440000000000000000000000000003u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_int_226, bid128_to_int32_int, 0x30520000000000000000000000000004u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_int_227, bid128_to_int32_int, 0x30520000000000000000000000000005u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_int_228, bid128_to_int32_int, 0x30540000000000000000000000000002u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_int_229, bid128_to_int32_int, 0x30d60000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_230, bid128_to_int32_int, 0x37ba0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_231, bid128_to_int32_int, 0x4a4c0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_232, bid128_to_int32_int, "5.5"                                 , 5          , 0x00);
dec_test!(bid128_to_int32_int_233, bid128_to_int32_int, 0x5b1e0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_234, bid128_to_int32_int, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_235, bid128_to_int32_int, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_236, bid128_to_int32_int, 0x79ffecdee11b1bff58a3091417010442u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_237, bid128_to_int32_int, 0x7c000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_238, bid128_to_int32_int, 0x7c003fffffffffff38c15b08ffffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_239, bid128_to_int32_int, 0x7c003fffffffffff38c15b0affffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_240, bid128_to_int32_int, 0x7e000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_241, bid128_to_int32_int, 0x923774827bc97c77823345d217ca41f6u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_242, bid128_to_int32_int, "-9"                                  , -9         , 0x00);
dec_test!(bid128_to_int32_int_243, bid128_to_int32_int, 0x99cebb57b0d4f19ac82e536d4dc6f1f4u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_244, bid128_to_int32_int, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_int32_int_245, bid128_to_int32_int, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0          , 0x00); // -- -(0.5)
dec_test!(bid128_to_int32_int_246, bid128_to_int32_int, 0xAFFCF684DF56C3E01BC6C73200000001u128, 0          , 0x00); // -- -(0.5+ulp)
dec_test!(bid128_to_int32_int_247, bid128_to_int32_int, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 0          , 0x00); // -- -(0.999-ulp)
dec_test!(bid128_to_int32_int_248, bid128_to_int32_int, 0xAFFDEC8B86EF679D76FC433D80000000u128, 0          , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_int_249, bid128_to_int32_int, 0xAFFDEC8B86EF679D76FC433D80000001u128, 0          , 0x00); // -- -(0.999+ulp)
dec_test!(bid128_to_int32_int_250, bid128_to_int32_int, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 0          , 0x00); // -- -(1-ulp)
dec_test!(bid128_to_int32_int_251, bid128_to_int32_int, 0xAFFE314DC6448D9338C15B0A00000000u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_int_252, bid128_to_int32_int, 0xAFFE314DC6448D9338C15B0A00000001u128, -1         , 0x00); // -- -(1+ulp)
dec_test!(bid128_to_int32_int_253, bid128_to_int32_int, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1         , 0x00); // -- -(1.5-ulp)
dec_test!(bid128_to_int32_int_254, bid128_to_int32_int, 0xAFFE49F4A966D45CD522088F00000000u128, -1         , 0x00); // -- -(1.5)
dec_test!(bid128_to_int32_int_255, bid128_to_int32_int, 0xAFFE49F4A966D45CD522088F00000001u128, -1         , 0x00); // -- -(1.5+ulp)
dec_test!(bid128_to_int32_int_256, bid128_to_int32_int, 0xafffaf7fbffbfbbb9928700620089144u128, -8         , 0x00);
dec_test!(bid128_to_int32_int_257, bid128_to_int32_int, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_258, bid128_to_int32_int, 0xB00293E952CDA8B9AA44111E00000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_259, bid128_to_int32_int, 0xB00293E952CDA8B9AA44111E00000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_260, bid128_to_int32_int, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_int_261, bid128_to_int32_int, 0xB00294286EACB8CB0A8CB6B140000000u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_int_262, bid128_to_int32_int, 0xB00294286EACB8CB0A8CB6B140000001u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_int_263, bid128_to_int32_int, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_264, bid128_to_int32_int, 0xB0040ECA8847C4129106CE8300000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_265, bid128_to_int32_int, 0xB0040ECA8847C4129106CE8300000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_266, bid128_to_int32_int, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_267, bid128_to_int32_int, 0xB00A0003C95A2F0B4856475FE0000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_268, bid128_to_int32_int, 0xB00A0003C95A2F0B4856475FE0000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_269, bid128_to_int32_int, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_270, bid128_to_int32_int, 0xB00C000060EF6B1ABA6F072330000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_271, bid128_to_int32_int, 0xB00C000060EF6B1ABA6F072330000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_272, bid128_to_int32_int, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, -2147483646, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_int_273, bid128_to_int32_int, 0xB01069E10DE628D3A6C9CC9B8E800000u128, -2147483646, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_int_274, bid128_to_int32_int, 0xB01069E10DE628D3A6C9CC9B8E800001u128, -2147483646, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_int_275, bid128_to_int32_int, 0xB01069E10DE692B4B4B133125EFFFFFFu128, -2147483646, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_int_276, bid128_to_int32_int, 0xB01069E10DE692B4B4B133125F000000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_int_277, bid128_to_int32_int, 0xB01069E10DE692B4B4B133125F000001u128, -2147483647, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_int_278, bid128_to_int32_int, 0xB01069E10DE6FC95C29899892F7FFFFFu128, -2147483647, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_int_279, bid128_to_int32_int, 0xB01069E10DE6FC95C29899892F800000u128, -2147483647, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_int_280, bid128_to_int32_int, 0xB01069E10DE6FC95C29899892F800001u128, -2147483647, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_int_281, bid128_to_int32_int, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, -2147483647, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_int_282, bid128_to_int32_int, 0xB01069E10DE76676D080000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_int_283, bid128_to_int32_int, 0xB01069E10DE76676D080000000000001u128, -2147483648, 0x00); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_int_284, bid128_to_int32_int, 0xB01069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x00); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_int_285, bid128_to_int32_int, 0xB01069E10DE7D057DE676676D0800000u128, -2147483648, 0x00); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_int_286, bid128_to_int32_int, 0xB01069E10DE7D057DE676676D0800001u128, -2147483648, 0x00); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_int_287, bid128_to_int32_int, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x00); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_int_288, bid128_to_int32_int, 0xB01069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_int_289, bid128_to_int32_int, 0xB01069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_int_290, bid128_to_int32_int, 0xB010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_int32_int_291, bid128_to_int32_int, 0xB010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_int_292, bid128_to_int32_int, 0xB010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_int32_int_293, bid128_to_int32_int, 0xB010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_int_294, bid128_to_int32_int, 0xB010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_int_295, bid128_to_int32_int, 0xB010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_int_296, bid128_to_int32_int, 0xB010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_int_297, bid128_to_int32_int, 0xB010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_int_298, bid128_to_int32_int, 0xB010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_int_299, bid128_to_int32_int, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_int_300, bid128_to_int32_int, 0xB010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_int_301, bid128_to_int32_int, 0xB010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_int_302, bid128_to_int32_int, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_int_303, bid128_to_int32_int, 0xB010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_int_304, bid128_to_int32_int, 0xB010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_int_305, bid128_to_int32_int, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_int_306, bid128_to_int32_int, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_int_307, bid128_to_int32_int, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_int_308, bid128_to_int32_int, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_int32_int_309, bid128_to_int32_int, 0xB010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_int_310, bid128_to_int32_int, 0xB010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_int32_int_311, bid128_to_int32_int, 0xb01128888b3098055ffff3ffef7717bfu128, -2147483648, 01);
dec_test!(bid128_to_int32_int_312, bid128_to_int32_int, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_int_313, bid128_to_int32_int, 0xB012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_int_314, bid128_to_int32_int, 0xB012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_int_315, bid128_to_int32_int, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_int_316, bid128_to_int32_int, 0xB012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_int_317, bid128_to_int32_int, 0xB012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_int_318, bid128_to_int32_int, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_int_319, bid128_to_int32_int, 0xB012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_int_320, bid128_to_int32_int, 0xB012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_int_321, bid128_to_int32_int, 0xB012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_int_322, bid128_to_int32_int, 0xB012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_int_323, bid128_to_int32_int, 0xB012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_int_324, bid128_to_int32_int, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_int_325, bid128_to_int32_int, 0xB012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_int_326, bid128_to_int32_int, 0xB012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_int_327, bid128_to_int32_int, 0xB012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_int_328, bid128_to_int32_int, 0xB012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_int_329, bid128_to_int32_int, 0xB012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_int_330, bid128_to_int32_int, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_int_331, bid128_to_int32_int, 0xB012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_int_332, bid128_to_int32_int, 0xB012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_int_333, bid128_to_int32_int, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, -2147483646, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_int_334, bid128_to_int32_int, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, -2147483646, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_int_335, bid128_to_int32_int, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, -2147483646, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_int_336, bid128_to_int32_int, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, -2147483647, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_int_337, bid128_to_int32_int, 0xB0180002B5E3AF13FBA450E94E780000u128, -2147483647, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_int_338, bid128_to_int32_int, 0xB0180002B5E3AF13FBA450E94E780001u128, -2147483647, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_int_339, bid128_to_int32_int, 0xB0180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x00); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_int_340, bid128_to_int32_int, 0xB0180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x00); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_int_341, bid128_to_int32_int, 0xB0180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x00); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_int_342, bid128_to_int32_int, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_int_343, bid128_to_int32_int, 0xB01800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_int_344, bid128_to_int32_int, 0xB01800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_int_345, bid128_to_int32_int, 0xB01800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_int_346, bid128_to_int32_int, 0xB01800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_int_347, bid128_to_int32_int, 0xB01800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_int_348, bid128_to_int32_int, 0xb0180008e093868005c06028201e8008u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_349, bid128_to_int32_int, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_int_350, bid128_to_int32_int, 0xB01A0000000000A2E6C09AD3E0D40000u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_int_351, bid128_to_int32_int, 0xB01A0000000000A2E6C09AD3E0D40001u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_int_352, bid128_to_int32_int, 0xB01A000045639181BA2CDCFB7617FFFFu128, -2147483646, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_int_353, bid128_to_int32_int, 0xB01A000045639181BA2CDCFB76180000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_int_354, bid128_to_int32_int, 0xB01A000045639181BA2CDCFB76180001u128, -2147483647, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_int_355, bid128_to_int32_int, 0xB01A00004563918244F3FFFFFFFFFFFFu128, -2147483647, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_int_356, bid128_to_int32_int, 0xB01A00004563918244F4000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_int_357, bid128_to_int32_int, 0xB01A00004563918244F4000000000001u128, -2147483648, 0x00); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_int_358, bid128_to_int32_int, 0xB01A000045639182CFBB230489E7FFFFu128, -2147483648, 0x00); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_int_359, bid128_to_int32_int, 0xB01A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_int_360, bid128_to_int32_int, 0xB01A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_int_361, bid128_to_int32_int, 0xB01A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_int_362, bid128_to_int32_int, 0xB01A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_int_363, bid128_to_int32_int, 0xB01A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_int_364, bid128_to_int32_int, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_int_365, bid128_to_int32_int, 0xB01A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_int_366, bid128_to_int32_int, 0xB01A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_int_367, bid128_to_int32_int, 0xB01A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_int_368, bid128_to_int32_int, 0xB01A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_int_369, bid128_to_int32_int, 0xB01A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_int_370, bid128_to_int32_int, 0xB01E000000000001A055690D9DB7FFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_371, bid128_to_int32_int, 0xB01E000000000001A055690D9DB80000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_372, bid128_to_int32_int, 0xB01E000000000001A055690D9DB80001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_373, bid128_to_int32_int, 0xB02000000000000029A2241AF62BFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_374, bid128_to_int32_int, 0xB02000000000000029A2241AF62C0000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_375, bid128_to_int32_int, 0xB02000000000000029A2241AF62C0001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_376, bid128_to_int32_int, 0xB024000000000000006A94D74F42FFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_377, bid128_to_int32_int, 0xB024000000000000006A94D74F430000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_378, bid128_to_int32_int, 0xB024000000000000006A94D74F430001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_379, bid128_to_int32_int, 0xB02A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_int_380, bid128_to_int32_int, 0xB02A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_int_381, bid128_to_int32_int, 0xB02A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_int_382, bid128_to_int32_int, 0xB02A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_int_383, bid128_to_int32_int, 0xB02A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_int_384, bid128_to_int32_int, 0xB02A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_int_385, bid128_to_int32_int, 0xB02C000000000000000002BBA7F521FFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_int_386, bid128_to_int32_int, 0xB02C000000000000000002BBA7F52200u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_int_387, bid128_to_int32_int, 0xB02C000000000000000002BBA7F52201u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_int_388, bid128_to_int32_int, 0xB02C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_int_389, bid128_to_int32_int, 0xB02C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_int_390, bid128_to_int32_int, 0xB02C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_int_391, bid128_to_int32_int, 0xB02C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_int_392, bid128_to_int32_int, 0xB02C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_int_393, bid128_to_int32_int, 0xB02C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_int_394, bid128_to_int32_int, 0xB02C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_int_395, bid128_to_int32_int, 0xB02C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_int_396, bid128_to_int32_int, 0xB02C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_int_397, bid128_to_int32_int, 0xB02E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_int_398, bid128_to_int32_int, 0xB02E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_int_399, bid128_to_int32_int, 0xB02E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_int_400, bid128_to_int32_int, 0xB03000000000000000000006FC23ABFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_401, bid128_to_int32_int, 0xB03000000000000000000006FC23AC00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_402, bid128_to_int32_int, 0xB03000000000000000000006FC23AC01u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_403, bid128_to_int32_int, 0xB03200000000000000000000B2D05DFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_404, bid128_to_int32_int, 0xB03200000000000000000000B2D05E00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_405, bid128_to_int32_int, 0xB03200000000000000000000B2D05E01u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_406, bid128_to_int32_int, 0xB03800000000000000000000002DDA47u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_int_407, bid128_to_int32_int, 0xB03800000000000000000000002DDA48u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_int_408, bid128_to_int32_int, 0xB03800000000000000000000002DDA49u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_int_409, bid128_to_int32_int, 0xB03A00000000000000000000000003E7u128, 0          , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_int_410, bid128_to_int32_int, 0xB03A00000000000000000000000495D3u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_int_411, bid128_to_int32_int, 0xB03A00000000000000000000000495D4u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_int_412, bid128_to_int32_int, 0xB03A00000000000000000000000495D5u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_int_413, bid128_to_int32_int, 0xB03C0000000000000000000000007561u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_int_414, bid128_to_int32_int, 0xB03C0000000000000000000000007562u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_int_415, bid128_to_int32_int, 0xB03C0000000000000000000000007563u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_int_416, bid128_to_int32_int, 0xB03E0000000000000000000000000005u128, 0          , 0x00); // -- -(0.5)
dec_test!(bid128_to_int32_int_417, bid128_to_int32_int, 0xB03E000000000000000000000000000Fu128, -1         , 0x00); // -- -(1.5)
dec_test!(bid128_to_int32_int_418, bid128_to_int32_int, 0xB03E0000000000000000000000000BB7u128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_419, bid128_to_int32_int, 0xB03E0000000000000000000000000BB8u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_420, bid128_to_int32_int, 0xB03E0000000000000000000000000BB9u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_421, bid128_to_int32_int, 0xB03E0000000000000000000000000BBDu128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_int_422, bid128_to_int32_int, 0xB03E00000000000000000004FFFFFFF1u128, -2147483646, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_int_423, bid128_to_int32_int, 0xB03E00000000000000000004FFFFFFFBu128, -2147483647, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_int_424, bid128_to_int32_int, 0xB03E0000000000000000000500000005u128, -2147483648, 0x00); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_int_425, bid128_to_int32_int, 0xB03E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_int_426, bid128_to_int32_int, 0xB03E0000000000000000000A00000005u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_int_427, bid128_to_int32_int, 0xB03E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_int_428, bid128_to_int32_int, 0xB03E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_int_429, bid128_to_int32_int, 0xB03E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_int_430, bid128_to_int32_int, 0xB03E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_int_431, bid128_to_int32_int, 0xB0400000000000000000000000000001u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_int_432, bid128_to_int32_int, 0xB040000000000000000000000000012Bu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_433, bid128_to_int32_int, 0xB040000000000000000000000000012Cu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_434, bid128_to_int32_int, 0xB040000000000000000000000000012Du128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_435, bid128_to_int32_int, 0xB040000000000000000000007FFFFFFFu128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_int_436, bid128_to_int32_int, 0xB0400000000000000000000080000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_int_437, bid128_to_int32_int, 0xB0400000000000000000000080000001u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_int_438, bid128_to_int32_int, 0xB04000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_int_439, bid128_to_int32_int, 0xB0400000000000000000000100000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_int_440, bid128_to_int32_int, 0xB0400000000000000000000100000001u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_int_441, bid128_to_int32_int, 0xB04000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_int_442, bid128_to_int32_int, 0xB04000000000000000000004A817C801u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_int_443, bid128_to_int32_int, 0xB042000000000000000000000000001Du128, -290       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_int_444, bid128_to_int32_int, 0xB042000000000000000000000000001Eu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_445, bid128_to_int32_int, 0xB042000000000000000000000000001Fu128, -310       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_int_446, bid128_to_int32_int, 0xB04200000000000000000000773593FFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_int_447, bid128_to_int32_int, 0xB0420000000000000000000077359400u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_int_448, bid128_to_int32_int, 0xB0420000000000000000000077359401u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_int_449, bid128_to_int32_int, 0xB0440000000000000000000000000003u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_int_450, bid128_to_int32_int, 0xB0520000000000000000000000000004u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_int_451, bid128_to_int32_int, 0xB0520000000000000000000000000005u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_int_452, bid128_to_int32_int, 0xB0540000000000000000000000000002u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_int_453, bid128_to_int32_int, 0xbef7dbff6bdf3bfeb4288d1808034018u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_454, bid128_to_int32_int, 0xc7943840d5b8dfdf31a307e22851f769u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_455, bid128_to_int32_int, 0xdfffffffffffbfff6830012051003800u128, 0          , 0x00);
dec_test!(bid128_to_int32_int_456, bid128_to_int32_int, 0xfafffffa7f7ff1ffe0a8340008028408u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_457, bid128_to_int32_int, 0xfd5681fcbdfa7b5fffff7bfaf57ffffeu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_458, bid128_to_int32_int, 0xfffffdfffffffefbb7ebdff9fdf7efb6u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_int_459, bid128_to_int32_int, "QNaN"                                , -2147483648, 0x01);
dec_test!(bid128_to_int32_int_460, bid128_to_int32_int, "SNaN"                                , -2147483648, 0x01);
