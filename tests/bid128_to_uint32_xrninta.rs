/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_uint32_xrninta_001, bid128_to_uint32_xrninta, "-0"                                  , 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_002, bid128_to_uint32_xrninta, 0                                     , 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_003, bid128_to_uint32_xrninta, 0x00000000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_004, bid128_to_uint32_xrninta, 0x00000000000000000000002000080800u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_005, bid128_to_uint32_xrninta, 0x00000000000000007fbffefdf27fc3feu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_006, bid128_to_uint32_xrninta, 0x000034108c0200335110e100182660c9u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_007, bid128_to_uint32_xrninta, 0x0001ed09bead87c0378d8e62ffffffffu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_008, bid128_to_uint32_xrninta, 0x0001ed09bead87c0378d8e64ffffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_009, bid128_to_uint32_xrninta, "-0.01000E0"                          , 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_010, bid128_to_uint32_xrninta, "0.1"                                 , 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_011, bid128_to_uint32_xrninta, "0.5"                                 , 1            , 0x20);
dec_test!(bid128_to_uint32_xrninta_012, bid128_to_uint32_xrninta, 0x05c08ebfc8fbba42a8e546929d4f9620u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_013, bid128_to_uint32_xrninta, 0x0ce20aab07e9368a95f6b8d42db96383u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_014, bid128_to_uint32_xrninta, 0x0fce0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_015, bid128_to_uint32_xrninta, "1.0"                                 , 1            , 0x00);
dec_test!(bid128_to_uint32_xrninta_016, bid128_to_uint32_xrninta, 1073741824                            , 1073741824   , 0x00);
dec_test!(bid128_to_uint32_xrninta_017, bid128_to_uint32_xrninta, 1                                     , 1            , 0x00);
dec_test!(bid128_to_uint32_xrninta_018, bid128_to_uint32_xrninta, "+1.11111000E0"                       , 1            , 0x20);
dec_test!(bid128_to_uint32_xrninta_019, bid128_to_uint32_xrninta, 0x14180000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_020, bid128_to_uint32_xrninta, 0x1df20000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_021, bid128_to_uint32_xrninta, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_uint32_xrninta_022, bid128_to_uint32_xrninta, 0x2FFCF684DF56C3E01BC6C73200000000u128, 1            , 0x20); // -- 0.5
dec_test!(bid128_to_uint32_xrninta_023, bid128_to_uint32_xrninta, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1            , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_uint32_xrninta_024, bid128_to_uint32_xrninta, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1            , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_uint32_xrninta_025, bid128_to_uint32_xrninta, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1            , 0x20); // -- 0.999
dec_test!(bid128_to_uint32_xrninta_026, bid128_to_uint32_xrninta, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1            , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_uint32_xrninta_027, bid128_to_uint32_xrninta, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1            , 0x20); // -- 1-ulp
dec_test!(bid128_to_uint32_xrninta_028, bid128_to_uint32_xrninta, 0x2FFE314DC6448D9338C15B0A00000000u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_xrninta_029, bid128_to_uint32_xrninta, 0x2FFE314DC6448D9338C15B0A00000001u128, 1            , 0x20); // -- 1+ulp
dec_test!(bid128_to_uint32_xrninta_030, bid128_to_uint32_xrninta, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xrninta_031, bid128_to_uint32_xrninta, 0x2FFE49F4A966D45CD522088F00000000u128, 2            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xrninta_032, bid128_to_uint32_xrninta, 0x2FFE49F4A966D45CD522088F00000001u128, 2            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xrninta_033, bid128_to_uint32_xrninta, 0x3000102000000128f7ffb8feb7bffb7fu128, 3            , 0x20);
dec_test!(bid128_to_uint32_xrninta_034, bid128_to_uint32_xrninta, 0x3002004811041a0802c428a42f80025bu128, 1            , 0x20);
dec_test!(bid128_to_uint32_xrninta_035, bid128_to_uint32_xrninta, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_036, bid128_to_uint32_xrninta, 0x300293E952CDA8B9AA44111E00000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_037, bid128_to_uint32_xrninta, 0x300293E952CDA8B9AA44111E00000001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_038, bid128_to_uint32_xrninta, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xrninta_039, bid128_to_uint32_xrninta, 0x300294286EACB8CB0A8CB6B140000000u128, 301          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xrninta_040, bid128_to_uint32_xrninta, 0x300294286EACB8CB0A8CB6B140000001u128, 301          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xrninta_041, bid128_to_uint32_xrninta, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_042, bid128_to_uint32_xrninta, 0x30040ECA8847C4129106CE8300000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_043, bid128_to_uint32_xrninta, 0x30040ECA8847C4129106CE8300000001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_044, bid128_to_uint32_xrninta, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_045, bid128_to_uint32_xrninta, 0x300A0003C95A2F0B4856475FE0000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_046, bid128_to_uint32_xrninta, 0x300A0003C95A2F0B4856475FE0000001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_047, bid128_to_uint32_xrninta, 0x300a086808110000a327c113bda6a500u128, 170502       , 0x20);
dec_test!(bid128_to_uint32_xrninta_048, bid128_to_uint32_xrninta, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_049, bid128_to_uint32_xrninta, 0x300C000060EF6B1ABA6F072330000000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_050, bid128_to_uint32_xrninta, 0x300C000060EF6B1ABA6F072330000001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_051, bid128_to_uint32_xrninta, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646   , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_xrninta_052, bid128_to_uint32_xrninta, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483647   , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_uint32_xrninta_053, bid128_to_uint32_xrninta, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483647   , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_xrninta_054, bid128_to_uint32_xrninta, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483647   , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_xrninta_055, bid128_to_uint32_xrninta, 0x301069E10DE692B4B4B133125F000000u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_xrninta_056, bid128_to_uint32_xrninta, 0x301069E10DE692B4B4B133125F000001u128, 2147483647   , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_xrninta_057, bid128_to_uint32_xrninta, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647   , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_xrninta_058, bid128_to_uint32_xrninta, 0x301069E10DE6FC95C29899892F800000u128, 2147483648   , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_uint32_xrninta_059, bid128_to_uint32_xrninta, 0x301069E10DE6FC95C29899892F800001u128, 2147483648   , 0x20); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_xrninta_060, bid128_to_uint32_xrninta, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x20); // -- 2^31-ulp
dec_test!(bid128_to_uint32_xrninta_061, bid128_to_uint32_xrninta, 0x301069E10DE76676D080000000000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_xrninta_062, bid128_to_uint32_xrninta, 0x301069E10DE76676D080000000000001u128, 2147483648   , 0x20); // -- 2^31+ulp
dec_test!(bid128_to_uint32_xrninta_063, bid128_to_uint32_xrninta, 0x301069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x20); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_xrninta_064, bid128_to_uint32_xrninta, 0x301069E10DE7D057DE676676D0800000u128, 2147483649   , 0x20); // -- 2^31+0.5
dec_test!(bid128_to_uint32_xrninta_065, bid128_to_uint32_xrninta, 0x301069E10DE7D057DE676676D0800001u128, 2147483649   , 0x20); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_xrninta_066, bid128_to_uint32_xrninta, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483649   , 0x20); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_xrninta_067, bid128_to_uint32_xrninta, 0x301069E10DE83A38EC4ECCEDA1000000u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_xrninta_068, bid128_to_uint32_xrninta, 0x301069E10DE83A38EC4ECCEDA1000001u128, 2147483649   , 0x20); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_xrninta_069, bid128_to_uint32_xrninta, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000   , 0x20); // -- 4e9-ulp
dec_test!(bid128_to_uint32_xrninta_070, bid128_to_uint32_xrninta, 0x3010C5371912364CE3056C2800000000u128, 4000000000   , 0x00); // -- 4e9
dec_test!(bid128_to_uint32_xrninta_071, bid128_to_uint32_xrninta, 0x3010C5371912364CE3056C2800000001u128, 4000000000   , 0x20); // -- 4e9+ulp
dec_test!(bid128_to_uint32_xrninta_072, bid128_to_uint32_xrninta, 0x3010D3C21BCDF92B853133125EFFFFFFu128, 4294967295   , 0x20); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_xrninta_073, bid128_to_uint32_xrninta, 0x3010D3C21BCDF92B853133125F000000u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_xrninta_074, bid128_to_uint32_xrninta, 0x3010D3C21BCDF92B853133125F000001u128, 4294967295   , 0x20); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_xrninta_075, bid128_to_uint32_xrninta, 0x3010D3C21BCE630C931899892F7FFFFFu128, 4294967295   , 0x20); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_xrninta_076, bid128_to_uint32_xrninta, 0x3010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_xrninta_077, bid128_to_uint32_xrninta, 0x3010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_xrninta_078, bid128_to_uint32_xrninta, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_uint32_xrninta_079, bid128_to_uint32_xrninta, 0x3010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_xrninta_080, bid128_to_uint32_xrninta, 0x3010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_xrninta_081, bid128_to_uint32_xrninta, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_xrninta_082, bid128_to_uint32_xrninta, 0x3010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_xrninta_083, bid128_to_uint32_xrninta, 0x3010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_xrninta_084, bid128_to_uint32_xrninta, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_xrninta_085, bid128_to_uint32_xrninta, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_xrninta_086, bid128_to_uint32_xrninta, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_xrninta_087, bid128_to_uint32_xrninta, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); // -- 5e9-ulp
dec_test!(bid128_to_uint32_xrninta_088, bid128_to_uint32_xrninta, 0x3010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- 5e9
dec_test!(bid128_to_uint32_xrninta_089, bid128_to_uint32_xrninta, 0x3010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- 5e9+ulp
dec_test!(bid128_to_uint32_xrninta_090, bid128_to_uint32_xrninta, 0x301128041a080e06e07bf3e0fba69fddu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_091, bid128_to_uint32_xrninta, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_xrninta_092, bid128_to_uint32_xrninta, 0x3012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_xrninta_093, bid128_to_uint32_xrninta, 0x3012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_xrninta_094, bid128_to_uint32_xrninta, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_xrninta_095, bid128_to_uint32_xrninta, 0x3012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_xrninta_096, bid128_to_uint32_xrninta, 0x3012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_xrninta_097, bid128_to_uint32_xrninta, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_xrninta_098, bid128_to_uint32_xrninta, 0x3012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_xrninta_099, bid128_to_uint32_xrninta, 0x3012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_xrninta_100, bid128_to_uint32_xrninta, 0x3012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_uint32_xrninta_101, bid128_to_uint32_xrninta, 0x3012629B8C891B267182B61400000000u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_xrninta_102, bid128_to_uint32_xrninta, 0x3012629B8C891B267182B61400000001u128, 2147483648   , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_uint32_xrninta_103, bid128_to_uint32_xrninta, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_xrninta_104, bid128_to_uint32_xrninta, 0x3012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_xrninta_105, bid128_to_uint32_xrninta, 0x3012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_xrninta_106, bid128_to_uint32_xrninta, 0x3012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_xrninta_107, bid128_to_uint32_xrninta, 0x3012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_xrninta_108, bid128_to_uint32_xrninta, 0x3012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_xrninta_109, bid128_to_uint32_xrninta, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_xrninta_110, bid128_to_uint32_xrninta, 0x3012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_xrninta_111, bid128_to_uint32_xrninta, 0x3012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_xrninta_112, bid128_to_uint32_xrninta, 0x301402088010000097325a8c63670c8du128, 0xF5CC9E51   , 0x20);
dec_test!(bid128_to_uint32_xrninta_113, bid128_to_uint32_xrninta, 0x301600000000003627E8F712373BFFFFu128, 1            , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_uint32_xrninta_114, bid128_to_uint32_xrninta, 0x301600000000003627E8F712373C0000u128, 1            , 0x20); // -- 0.999
dec_test!(bid128_to_uint32_xrninta_115, bid128_to_uint32_xrninta, 0x301600000000003627E8F712373C0001u128, 1            , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_uint32_xrninta_116, bid128_to_uint32_xrninta, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646   , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_xrninta_117, bid128_to_uint32_xrninta, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483647   , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_uint32_xrninta_118, bid128_to_uint32_xrninta, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483647   , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_xrninta_119, bid128_to_uint32_xrninta, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647   , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_xrninta_120, bid128_to_uint32_xrninta, 0x30180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_uint32_xrninta_121, bid128_to_uint32_xrninta, 0x30180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x20); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_xrninta_122, bid128_to_uint32_xrninta, 0x30180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x20); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_xrninta_123, bid128_to_uint32_xrninta, 0x30180002B5E3AF19676BAF16B1880000u128, 2147483649   , 0x20); // -- 2^31+0.5
dec_test!(bid128_to_uint32_xrninta_124, bid128_to_uint32_xrninta, 0x30180002B5E3AF19676BAF16B1880001u128, 2147483649   , 0x20); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_xrninta_125, bid128_to_uint32_xrninta, 0x301800040d00100066b67bbbf3b3bcefu128, 0xBF4AF3B3   , 0x20);
dec_test!(bid128_to_uint32_xrninta_126, bid128_to_uint32_xrninta, 0x301800056BC75E2AAD2C50E94E77FFFFu128, 4294967295   , 0x20); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_xrninta_127, bid128_to_uint32_xrninta, 0x301800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_xrninta_128, bid128_to_uint32_xrninta, 0x301800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_xrninta_129, bid128_to_uint32_xrninta, 0x301800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_xrninta_130, bid128_to_uint32_xrninta, 0x301800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_xrninta_131, bid128_to_uint32_xrninta, 0x301800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_xrninta_132, bid128_to_uint32_xrninta, 0x301A0000000000004563918244F3FFFFu128, 0            , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_uint32_xrninta_133, bid128_to_uint32_xrninta, 0x301A0000000000004563918244F40000u128, 1            , 0x20); // -- 0.5
dec_test!(bid128_to_uint32_xrninta_134, bid128_to_uint32_xrninta, 0x301A0000000000004563918244F40001u128, 1            , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_uint32_xrninta_135, bid128_to_uint32_xrninta, 0x301A0000000000008AC7230489E7FFFFu128, 1            , 0x20); // -- 1-ulp
dec_test!(bid128_to_uint32_xrninta_136, bid128_to_uint32_xrninta, 0x301A0000000000008AC7230489E80000u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_xrninta_137, bid128_to_uint32_xrninta, 0x301A0000000000008AC7230489E80001u128, 1            , 0x20); // -- 1+ulp
dec_test!(bid128_to_uint32_xrninta_138, bid128_to_uint32_xrninta, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xrninta_139, bid128_to_uint32_xrninta, 0x301A0000000000A2E6C09AD3E0D40000u128, 301          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xrninta_140, bid128_to_uint32_xrninta, 0x301A0000000000A2E6C09AD3E0D40001u128, 301          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xrninta_141, bid128_to_uint32_xrninta, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483647   , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_xrninta_142, bid128_to_uint32_xrninta, 0x301A000045639181BA2CDCFB76180000u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_xrninta_143, bid128_to_uint32_xrninta, 0x301A000045639181BA2CDCFB76180001u128, 2147483647   , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_xrninta_144, bid128_to_uint32_xrninta, 0x301A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x20); // -- 2^31-ulp
dec_test!(bid128_to_uint32_xrninta_145, bid128_to_uint32_xrninta, 0x301A00004563918244F4000000000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_xrninta_146, bid128_to_uint32_xrninta, 0x301A00004563918244F4000000000001u128, 2147483648   , 0x20); // -- 2^31+ulp
dec_test!(bid128_to_uint32_xrninta_147, bid128_to_uint32_xrninta, 0x301A000045639182CFBB230489E7FFFFu128, 2147483649   , 0x20); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_xrninta_148, bid128_to_uint32_xrninta, 0x301A000045639182CFBB230489E80000u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_xrninta_149, bid128_to_uint32_xrninta, 0x301A000045639182CFBB230489E80001u128, 2147483649   , 0x20); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_xrninta_150, bid128_to_uint32_xrninta, 0x301A00008AC72303FF20DCFB7617FFFFu128, 4294967295   , 0x20); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_xrninta_151, bid128_to_uint32_xrninta, 0x301A00008AC72303FF20DCFB76180000u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_xrninta_152, bid128_to_uint32_xrninta, 0x301A00008AC72303FF20DCFB76180001u128, 4294967295   , 0x20); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_xrninta_153, bid128_to_uint32_xrninta, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_uint32_xrninta_154, bid128_to_uint32_xrninta, 0x301A00008AC7230489E8000000000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_xrninta_155, bid128_to_uint32_xrninta, 0x301A00008AC7230489E8000000000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_xrninta_156, bid128_to_uint32_xrninta, 0x301A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_xrninta_157, bid128_to_uint32_xrninta, 0x301A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_xrninta_158, bid128_to_uint32_xrninta, 0x301A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_xrninta_159, bid128_to_uint32_xrninta, 0x301C00000000000014D1120D7B15FFFFu128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xrninta_160, bid128_to_uint32_xrninta, 0x301C00000000000014D1120D7B160000u128, 2            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xrninta_161, bid128_to_uint32_xrninta, 0x301C00000000000014D1120D7B160001u128, 2            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xrninta_162, bid128_to_uint32_xrninta, 0x301E000000000001A055690D9DB7FFFFu128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_163, bid128_to_uint32_xrninta, 0x301E000000000001A055690D9DB80000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_164, bid128_to_uint32_xrninta, 0x301E000000000001A055690D9DB80001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_165, bid128_to_uint32_xrninta, 0x302000000000000029A2241AF62BFFFFu128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_166, bid128_to_uint32_xrninta, 0x302000000000000029A2241AF62C0000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_167, bid128_to_uint32_xrninta, 0x302000000000000029A2241AF62C0001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_168, bid128_to_uint32_xrninta, 0x30200000000400000d4bcce1909a232au128, 483570424    , 0x20);
dec_test!(bid128_to_uint32_xrninta_169, bid128_to_uint32_xrninta, 0x3024000000000000006A94D74F42FFFFu128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_170, bid128_to_uint32_xrninta, 0x3024000000000000006A94D74F430000u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_171, bid128_to_uint32_xrninta, 0x3024000000000000006A94D74F430001u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_172, bid128_to_uint32_xrninta, 0x302A00000000000000000017428106FFu128, 1            , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_uint32_xrninta_173, bid128_to_uint32_xrninta, 0x302A0000000000000000001742810700u128, 1            , 0x20); // -- 0.999
dec_test!(bid128_to_uint32_xrninta_174, bid128_to_uint32_xrninta, 0x302A0000000000000000001742810701u128, 1            , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_uint32_xrninta_175, bid128_to_uint32_xrninta, 0x302A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_xrninta_176, bid128_to_uint32_xrninta, 0x302A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_xrninta_177, bid128_to_uint32_xrninta, 0x302A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_xrninta_178, bid128_to_uint32_xrninta, 0x302A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_xrninta_179, bid128_to_uint32_xrninta, 0x302A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_xrninta_180, bid128_to_uint32_xrninta, 0x302A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_xrninta_181, bid128_to_uint32_xrninta, 0x302C000000000000000002BBA7F521FFu128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xrninta_182, bid128_to_uint32_xrninta, 0x302C000000000000000002BBA7F52200u128, 301          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xrninta_183, bid128_to_uint32_xrninta, 0x302C000000000000000002BBA7F52201u128, 301          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xrninta_184, bid128_to_uint32_xrninta, 0x302C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_xrninta_185, bid128_to_uint32_xrninta, 0x302C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_xrninta_186, bid128_to_uint32_xrninta, 0x302C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_xrninta_187, bid128_to_uint32_xrninta, 0x302C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_xrninta_188, bid128_to_uint32_xrninta, 0x302C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_xrninta_189, bid128_to_uint32_xrninta, 0x302C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_xrninta_190, bid128_to_uint32_xrninta, 0x302C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_xrninta_191, bid128_to_uint32_xrninta, 0x302C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_xrninta_192, bid128_to_uint32_xrninta, 0x302C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_xrninta_193, bid128_to_uint32_xrninta, 0x302E000000000000000000001DCD64FFu128, 0            , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_uint32_xrninta_194, bid128_to_uint32_xrninta, 0x302E000000000000000000001DCD6500u128, 1            , 0x20); // -- 0.5
dec_test!(bid128_to_uint32_xrninta_195, bid128_to_uint32_xrninta, 0x302E000000000000000000001DCD6501u128, 1            , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_uint32_xrninta_196, bid128_to_uint32_xrninta, 0x302E000000000000000000003B9AC9FFu128, 1            , 0x20); // -- 1-ulp
dec_test!(bid128_to_uint32_xrninta_197, bid128_to_uint32_xrninta, 0x302E000000000000000000003B9ACA00u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_xrninta_198, bid128_to_uint32_xrninta, 0x302E000000000000000000003B9ACA01u128, 1            , 0x20); // -- 1+ulp
dec_test!(bid128_to_uint32_xrninta_199, bid128_to_uint32_xrninta, 0x302E0000000000000000000059682EFFu128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xrninta_200, bid128_to_uint32_xrninta, 0x302E0000000000000000000059682F00u128, 2            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xrninta_201, bid128_to_uint32_xrninta, 0x302E0000000000000000000059682F01u128, 2            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xrninta_202, bid128_to_uint32_xrninta, 0x302E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_xrninta_203, bid128_to_uint32_xrninta, 0x302E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_xrninta_204, bid128_to_uint32_xrninta, 0x302E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_xrninta_205, bid128_to_uint32_xrninta, 0x303000000000000000000006FC23ABFFu128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_206, bid128_to_uint32_xrninta, 0x303000000000000000000006FC23AC00u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_207, bid128_to_uint32_xrninta, 0x303000000000000000000006FC23AC01u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_208, bid128_to_uint32_xrninta, 0x303200000000000000000000B2D05DFFu128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_209, bid128_to_uint32_xrninta, 0x303200000000000000000000B2D05E00u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_210, bid128_to_uint32_xrninta, 0x303200000000000000000000B2D05E01u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_211, bid128_to_uint32_xrninta, 0x303800000000000000000000002DDA47u128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xrninta_212, bid128_to_uint32_xrninta, 0x303800000000000000000000002DDA48u128, 301          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xrninta_213, bid128_to_uint32_xrninta, 0x303800000000000000000000002DDA49u128, 301          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xrninta_214, bid128_to_uint32_xrninta, 0x303A00000000000000000000000003E7u128, 1            , 0x20); // -- 0.999
dec_test!(bid128_to_uint32_xrninta_215, bid128_to_uint32_xrninta, 0x303A00000000000000000000000005DBu128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xrninta_216, bid128_to_uint32_xrninta, 0x303A00000000000000000000000005DCu128, 2            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xrninta_217, bid128_to_uint32_xrninta, 0x303A00000000000000000000000005DDu128, 2            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xrninta_218, bid128_to_uint32_xrninta, 0x303A00000000000000000000000495D3u128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xrninta_219, bid128_to_uint32_xrninta, 0x303A00000000000000000000000495D4u128, 301          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xrninta_220, bid128_to_uint32_xrninta, 0x303A00000000000000000000000495D5u128, 301          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xrninta_221, bid128_to_uint32_xrninta, 0x303C0000000000000000000000000095u128, 1            , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint32_xrninta_222, bid128_to_uint32_xrninta, 0x303C0000000000000000000000000096u128, 2            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xrninta_223, bid128_to_uint32_xrninta, 0x303C0000000000000000000000000097u128, 2            , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint32_xrninta_224, bid128_to_uint32_xrninta, 0x303C0000000000000000000000007561u128, 300          , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint32_xrninta_225, bid128_to_uint32_xrninta, 0x303C0000000000000000000000007562u128, 301          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xrninta_226, bid128_to_uint32_xrninta, 0x303C0000000000000000000000007563u128, 301          , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint32_xrninta_227, bid128_to_uint32_xrninta, 0x303C00000000000000000031FFFFFF69u128, 2147483646   , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_xrninta_228, bid128_to_uint32_xrninta, 0x303C00000000000000000031FFFFFF6Au128, 2147483647   , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_uint32_xrninta_229, bid128_to_uint32_xrninta, 0x303C00000000000000000031FFFFFF6Bu128, 2147483647   , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_xrninta_230, bid128_to_uint32_xrninta, 0x303C00000000000000000031FFFFFFCDu128, 2147483647   , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_xrninta_231, bid128_to_uint32_xrninta, 0x303C00000000000000000031FFFFFFCEu128, 2147483648   , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_uint32_xrninta_232, bid128_to_uint32_xrninta, 0x303C00000000000000000031FFFFFFCFu128, 2147483648   , 0x20); // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_xrninta_233, bid128_to_uint32_xrninta, 0x303C0000000000000000003200000031u128, 2147483648   , 0x20); // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_xrninta_234, bid128_to_uint32_xrninta, 0x303C0000000000000000003200000032u128, 2147483649   , 0x20); // -- 2^31+0.5
dec_test!(bid128_to_uint32_xrninta_235, bid128_to_uint32_xrninta, 0x303C0000000000000000003200000033u128, 2147483649   , 0x20); // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_xrninta_236, bid128_to_uint32_xrninta, 0x303C00000000000000000063FFFFFFCDu128, 4294967295   , 0x20); // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_xrninta_237, bid128_to_uint32_xrninta, 0x303C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_xrninta_238, bid128_to_uint32_xrninta, 0x303C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_xrninta_239, bid128_to_uint32_xrninta, 0x303C0000000000000000006400000031u128, 2147483648   , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_xrninta_240, bid128_to_uint32_xrninta, 0x303C0000000000000000006400000032u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_xrninta_241, bid128_to_uint32_xrninta, 0x303C0000000000000000006400000033u128, 2147483648   , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_xrninta_242, bid128_to_uint32_xrninta, 0x303E0000000000000000000000000005u128, 1            , 0x20); // -- 0.5
dec_test!(bid128_to_uint32_xrninta_243, bid128_to_uint32_xrninta, 0x303E000000000000000000000000000Fu128, 2            , 0x20); // -- 1.5
dec_test!(bid128_to_uint32_xrninta_244, bid128_to_uint32_xrninta, 0x303E0000000000000000000000000BB7u128, 300          , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_245, bid128_to_uint32_xrninta, 0x303E0000000000000000000000000BB8u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_246, bid128_to_uint32_xrninta, 0x303E0000000000000000000000000BB9u128, 300          , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_247, bid128_to_uint32_xrninta, 0x303E0000000000000000000000000BBDu128, 301          , 0x20); // -- 300.5
dec_test!(bid128_to_uint32_xrninta_248, bid128_to_uint32_xrninta, 0x303E00000000000000000004FFFFFFF1u128, 2147483647   , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_uint32_xrninta_249, bid128_to_uint32_xrninta, 0x303E00000000000000000004FFFFFFF5u128, 2147483647   , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_xrninta_250, bid128_to_uint32_xrninta, 0x303E00000000000000000004FFFFFFF6u128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_xrninta_251, bid128_to_uint32_xrninta, 0x303E00000000000000000004FFFFFFF7u128, 2147483647   , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_xrninta_252, bid128_to_uint32_xrninta, 0x303E00000000000000000004FFFFFFFBu128, 2147483648   , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_uint32_xrninta_253, bid128_to_uint32_xrninta, 0x303E00000000000000000004FFFFFFFFu128, 2147483648   , 0x20); // -- 2^31-ulp
dec_test!(bid128_to_uint32_xrninta_254, bid128_to_uint32_xrninta, 0x303E0000000000000000000500000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_xrninta_255, bid128_to_uint32_xrninta, 0x303E0000000000000000000500000001u128, 2147483648   , 0x20); // -- 2^31+ulp
dec_test!(bid128_to_uint32_xrninta_256, bid128_to_uint32_xrninta, 0x303E0000000000000000000500000005u128, 2147483649   , 0x20); // -- 2^31+0.5
dec_test!(bid128_to_uint32_xrninta_257, bid128_to_uint32_xrninta, 0x303E0000000000000000000500000009u128, 2147483649   , 0x20); // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_xrninta_258, bid128_to_uint32_xrninta, 0x303E000000000000000000050000000Au128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_xrninta_259, bid128_to_uint32_xrninta, 0x303E000000000000000000050000000Bu128, 2147483649   , 0x20); // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_xrninta_260, bid128_to_uint32_xrninta, 0x303E00000000000000000009FFFFFFF5u128, 4294967295   , 0x20); // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_xrninta_261, bid128_to_uint32_xrninta, 0x303E00000000000000000009FFFFFFF6u128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_xrninta_262, bid128_to_uint32_xrninta, 0x303E00000000000000000009FFFFFFF7u128, 4294967295   , 0x20); // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_xrninta_263, bid128_to_uint32_xrninta, 0x303E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_uint32_xrninta_264, bid128_to_uint32_xrninta, 0x303E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_uint32_xrninta_265, bid128_to_uint32_xrninta, 0x303E0000000000000000000A00000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_xrninta_266, bid128_to_uint32_xrninta, 0x303E0000000000000000000A00000001u128, 2147483648   , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_uint32_xrninta_267, bid128_to_uint32_xrninta, 0x303E0000000000000000000A00000005u128, 2147483648   , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_uint32_xrninta_268, bid128_to_uint32_xrninta, 0x303E0000000000000000000A00000009u128, 2147483648   , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_xrninta_269, bid128_to_uint32_xrninta, 0x303E0000000000000000000A0000000Au128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_xrninta_270, bid128_to_uint32_xrninta, 0x303E0000000000000000000A0000000Bu128, 2147483648   , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_xrninta_271, bid128_to_uint32_xrninta, 0x303E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_uint32_xrninta_272, bid128_to_uint32_xrninta, 0x303E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_uint32_xrninta_273, bid128_to_uint32_xrninta, 0x303E0000000000000000002E90EDD005u128, 2147483648   , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_uint32_xrninta_274, bid128_to_uint32_xrninta, 0x303E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_uint32_xrninta_275, bid128_to_uint32_xrninta, 0x30400000000000000000000000000001u128, 1            , 0x00); // -- 1
dec_test!(bid128_to_uint32_xrninta_276, bid128_to_uint32_xrninta, 0x3040000000000000000000000000012Bu128, 299          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_277, bid128_to_uint32_xrninta, 0x3040000000000000000000000000012Cu128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_278, bid128_to_uint32_xrninta, 0x3040000000000000000000000000012Du128, 301          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_279, bid128_to_uint32_xrninta, 0x3040000000000000000000007FFFFFFFu128, 2147483647   , 0x00); // -- 2^31-1
dec_test!(bid128_to_uint32_xrninta_280, bid128_to_uint32_xrninta, 0x30400000000000000000000080000000u128, 2147483648   , 0x00); // -- 2^31
dec_test!(bid128_to_uint32_xrninta_281, bid128_to_uint32_xrninta, 0x30400000000000000000000080000001u128, 2147483649   , 0x00); // -- 2^31+1
dec_test!(bid128_to_uint32_xrninta_282, bid128_to_uint32_xrninta, 0x304000000000000000000000FFFFFFFFu128, 4294967295   , 0x00); // -- 2^32-1
dec_test!(bid128_to_uint32_xrninta_283, bid128_to_uint32_xrninta, 0x30400000000000000000000100000000u128, 2147483648   , 0x01); // -- 2^32
dec_test!(bid128_to_uint32_xrninta_284, bid128_to_uint32_xrninta, 0x30400000000000000000000100000001u128, 2147483648   , 0x01); // -- 2^32+1
dec_test!(bid128_to_uint32_xrninta_285, bid128_to_uint32_xrninta, 0x304000000000000000000004A817C7FFu128, 2147483648   , 0x01); // -- 2e10-1
dec_test!(bid128_to_uint32_xrninta_286, bid128_to_uint32_xrninta, 0x304000000000000000000004A817C801u128, 2147483648   , 0x01); // -- 2e10+1
dec_test!(bid128_to_uint32_xrninta_287, bid128_to_uint32_xrninta, 0x3041ED09BEAD87C0378D8E6400000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_288, bid128_to_uint32_xrninta, 0x3042000000000000000000000000001Du128, 290          , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint32_xrninta_289, bid128_to_uint32_xrninta, 0x3042000000000000000000000000001Eu128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_290, bid128_to_uint32_xrninta, 0x3042000000000000000000000000001Fu128, 310          , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint32_xrninta_291, bid128_to_uint32_xrninta, 0x304200000000000000000000773593FFu128, 2147483648   , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_uint32_xrninta_292, bid128_to_uint32_xrninta, 0x30420000000000000000000077359400u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_xrninta_293, bid128_to_uint32_xrninta, 0x30420000000000000000000077359401u128, 2147483648   , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_uint32_xrninta_294, bid128_to_uint32_xrninta, 0x30440000000000000000000000000003u128, 300          , 0x00); // -- 300
dec_test!(bid128_to_uint32_xrninta_295, bid128_to_uint32_xrninta, 0x30520000000000000000000000000004u128, 4000000000   , 0x00); // -- 4e9
dec_test!(bid128_to_uint32_xrninta_296, bid128_to_uint32_xrninta, 0x30520000000000000000000000000005u128, 2147483648   , 0x01); // -- 5e9
dec_test!(bid128_to_uint32_xrninta_297, bid128_to_uint32_xrninta, 0x30540000000000000000000000000002u128, 2147483648   , 0x01); // -- 2e10
dec_test!(bid128_to_uint32_xrninta_298, bid128_to_uint32_xrninta, 0x310684325cc8616f9318d60769df9596u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_299, bid128_to_uint32_xrninta, 0x3c000000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_300, bid128_to_uint32_xrninta, 0x3e670503b2dce865f60e58bdf99542b6u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_301, bid128_to_uint32_xrninta, 4294967296u64                         , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_302, bid128_to_uint32_xrninta, 0x440c220d5cc400b40a994f35994354aau128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_303, bid128_to_uint32_xrninta, 0x45460000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_304, bid128_to_uint32_xrninta, 0x502a0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_305, bid128_to_uint32_xrninta, "5.05"                                , 5            , 0x20);
dec_test!(bid128_to_uint32_xrninta_306, bid128_to_uint32_xrninta, 0x53580000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_307, bid128_to_uint32_xrninta, 0x544ca06677d8f2547075b42461ea503du128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_308, bid128_to_uint32_xrninta, 0x55160000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_309, bid128_to_uint32_xrninta, "+5.575669595855E0"                   , 6            , 0x20);
dec_test!(bid128_to_uint32_xrninta_310, bid128_to_uint32_xrninta, 0x59332a546b06a77035f47048a2ddea90u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_311, bid128_to_uint32_xrninta, 0x5d2036e29a48d5388ed8157a4a9378a8u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_312, bid128_to_uint32_xrninta, "+67887.6597666885E0"                 , 67888        , 0x20);
dec_test!(bid128_to_uint32_xrninta_313, bid128_to_uint32_xrninta, "+777.55858E0"                        , 778          , 0x20);
dec_test!(bid128_to_uint32_xrninta_314, bid128_to_uint32_xrninta, 0x78000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xrninta_315, bid128_to_uint32_xrninta, "+796579.2527637E0"                   , 796579       , 0x20);
dec_test!(bid128_to_uint32_xrninta_316, bid128_to_uint32_xrninta, 0x7b8382c5060d098b0000000000000000u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_317, bid128_to_uint32_xrninta, 0x7c000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xrninta_318, bid128_to_uint32_xrninta, 0x7c003fffffffffff38c15b08ffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_319, bid128_to_uint32_xrninta, 0x7c003fffffffffff38c15b0affffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_320, bid128_to_uint32_xrninta, 0x7e000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xrninta_321, bid128_to_uint32_xrninta, 0x7f835cfddaf9dfbdc98d334f91461f5au128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_322, bid128_to_uint32_xrninta, "+8899898.8E0"                        , 8899899      , 0x20);
dec_test!(bid128_to_uint32_xrninta_323, bid128_to_uint32_xrninta, 0x8f5bdada414a292d9f06c1664a6091f0u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_324, bid128_to_uint32_xrninta, 0x956952cf117c3d67a494caf20def6d50u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_325, bid128_to_uint32_xrninta, 0x96ec0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_326, bid128_to_uint32_xrninta, "+988898898.89889998E0"               , 988898899    , 0x20);
dec_test!(bid128_to_uint32_xrninta_327, bid128_to_uint32_xrninta, 0x9c6f2bf6b31ab797036dde78044524a0u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_328, bid128_to_uint32_xrninta, 0x9d19586b73ffa50178481becb0f0f509u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_329, bid128_to_uint32_xrninta, 0xa06ac49406efa0807646edbec45fadf2u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_330, bid128_to_uint32_xrninta, 0xa1c20000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_331, bid128_to_uint32_xrninta, 0xa3016dfbe181bf380ef45737708c0f4fu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_332, bid128_to_uint32_xrninta, 0xa6af0fa847e4d6b4e2f22d47240a4486u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_333, bid128_to_uint32_xrninta, 0xa869dbb96f4580192500ef2e9ffb5a8du128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_334, bid128_to_uint32_xrninta, 0xa953045064499e02d2e329d369f1b9a8u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_335, bid128_to_uint32_xrninta, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_336, bid128_to_uint32_xrninta, 0xAFFCF684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- -(0.5)
dec_test!(bid128_to_uint32_xrninta_337, bid128_to_uint32_xrninta, 0xAFFCF684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_338, bid128_to_uint32_xrninta, 0xaffdb6bff65fdfff154003700142001au128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_339, bid128_to_uint32_xrninta, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_xrninta_340, bid128_to_uint32_xrninta, 0xAFFDEC8B86EF679D76FC433D80000000u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_xrninta_341, bid128_to_uint32_xrninta, 0xAFFDEC8B86EF679D76FC433D80000001u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_xrninta_342, bid128_to_uint32_xrninta, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_xrninta_343, bid128_to_uint32_xrninta, 0xAFFE314DC6448D9338C15B0A00000000u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_xrninta_344, bid128_to_uint32_xrninta, 0xAFFE314DC6448D9338C15B0A00000001u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_xrninta_345, bid128_to_uint32_xrninta, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_346, bid128_to_uint32_xrninta, 0xAFFE49F4A966D45CD522088F00000000u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xrninta_347, bid128_to_uint32_xrninta, 0xAFFE49F4A966D45CD522088F00000001u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_348, bid128_to_uint32_xrninta, 0xb0000081410000400a10800000060000u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xrninta_349, bid128_to_uint32_xrninta, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_350, bid128_to_uint32_xrninta, 0xB00293E952CDA8B9AA44111E00000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_351, bid128_to_uint32_xrninta, 0xB00293E952CDA8B9AA44111E00000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_352, bid128_to_uint32_xrninta, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xrninta_353, bid128_to_uint32_xrninta, 0xB00294286EACB8CB0A8CB6B140000000u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xrninta_354, bid128_to_uint32_xrninta, 0xB00294286EACB8CB0A8CB6B140000001u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xrninta_355, bid128_to_uint32_xrninta, 0xb0032b7d201e69e75482dc7f79b305a0u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_356, bid128_to_uint32_xrninta, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_357, bid128_to_uint32_xrninta, 0xB0040ECA8847C4129106CE8300000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_358, bid128_to_uint32_xrninta, 0xB0040ECA8847C4129106CE8300000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_359, bid128_to_uint32_xrninta, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_360, bid128_to_uint32_xrninta, 0xB00A0003C95A2F0B4856475FE0000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_361, bid128_to_uint32_xrninta, 0xB00A0003C95A2F0B4856475FE0000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_362, bid128_to_uint32_xrninta, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_363, bid128_to_uint32_xrninta, 0xB00C000060EF6B1ABA6F072330000000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_364, bid128_to_uint32_xrninta, 0xB00C000060EF6B1ABA6F072330000001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_365, bid128_to_uint32_xrninta, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_366, bid128_to_uint32_xrninta, 0xB01069E10DE628D3A6C9CC9B8E800000u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xrninta_367, bid128_to_uint32_xrninta, 0xB01069E10DE628D3A6C9CC9B8E800001u128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_368, bid128_to_uint32_xrninta, 0xB01069E10DE692B4B4B133125EFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xrninta_369, bid128_to_uint32_xrninta, 0xB01069E10DE692B4B4B133125F000000u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_xrninta_370, bid128_to_uint32_xrninta, 0xB01069E10DE692B4B4B133125F000001u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xrninta_371, bid128_to_uint32_xrninta, 0xB01069E10DE6FC95C29899892F7FFFFFu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_372, bid128_to_uint32_xrninta, 0xB01069E10DE6FC95C29899892F800000u128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xrninta_373, bid128_to_uint32_xrninta, 0xB01069E10DE6FC95C29899892F800001u128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_374, bid128_to_uint32_xrninta, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_xrninta_375, bid128_to_uint32_xrninta, 0xB01069E10DE76676D080000000000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_xrninta_376, bid128_to_uint32_xrninta, 0xB01069E10DE76676D080000000000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_xrninta_377, bid128_to_uint32_xrninta, 0xB01069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_378, bid128_to_uint32_xrninta, 0xB01069E10DE7D057DE676676D0800000u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xrninta_379, bid128_to_uint32_xrninta, 0xB01069E10DE7D057DE676676D0800001u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_380, bid128_to_uint32_xrninta, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xrninta_381, bid128_to_uint32_xrninta, 0xB01069E10DE83A38EC4ECCEDA1000000u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_xrninta_382, bid128_to_uint32_xrninta, 0xB01069E10DE83A38EC4ECCEDA1000001u128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xrninta_383, bid128_to_uint32_xrninta, 0xB010C5371912364CE3056C27FFFFFFFFu128, 2147483648   , 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_uint32_xrninta_384, bid128_to_uint32_xrninta, 0xB010C5371912364CE3056C2800000000u128, 2147483648   , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint32_xrninta_385, bid128_to_uint32_xrninta, 0xB010C5371912364CE3056C2800000001u128, 2147483648   , 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_uint32_xrninta_386, bid128_to_uint32_xrninta, 0xB010D3C21BCDF92B853133125EFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xrninta_387, bid128_to_uint32_xrninta, 0xB010D3C21BCDF92B853133125F000000u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_xrninta_388, bid128_to_uint32_xrninta, 0xB010D3C21BCDF92B853133125F000001u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xrninta_389, bid128_to_uint32_xrninta, 0xB010D3C21BCE630C931899892F7FFFFFu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_390, bid128_to_uint32_xrninta, 0xB010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xrninta_391, bid128_to_uint32_xrninta, 0xB010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_392, bid128_to_uint32_xrninta, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_xrninta_393, bid128_to_uint32_xrninta, 0xB010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_xrninta_394, bid128_to_uint32_xrninta, 0xB010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_xrninta_395, bid128_to_uint32_xrninta, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_396, bid128_to_uint32_xrninta, 0xB010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xrninta_397, bid128_to_uint32_xrninta, 0xB010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_398, bid128_to_uint32_xrninta, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xrninta_399, bid128_to_uint32_xrninta, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_xrninta_400, bid128_to_uint32_xrninta, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xrninta_401, bid128_to_uint32_xrninta, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_uint32_xrninta_402, bid128_to_uint32_xrninta, 0xB010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint32_xrninta_403, bid128_to_uint32_xrninta, 0xB010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_uint32_xrninta_404, bid128_to_uint32_xrninta, 0xb01198c8829cb0ddd631a06392c381e6u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_405, bid128_to_uint32_xrninta, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_406, bid128_to_uint32_xrninta, 0xB012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_xrninta_407, bid128_to_uint32_xrninta, 0xB012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_408, bid128_to_uint32_xrninta, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_xrninta_409, bid128_to_uint32_xrninta, 0xB012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_xrninta_410, bid128_to_uint32_xrninta, 0xB012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_xrninta_411, bid128_to_uint32_xrninta, 0xB012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_412, bid128_to_uint32_xrninta, 0xB012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_xrninta_413, bid128_to_uint32_xrninta, 0xB012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_414, bid128_to_uint32_xrninta, 0xB012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_xrninta_415, bid128_to_uint32_xrninta, 0xB012629B8C891B267182B61400000000u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_xrninta_416, bid128_to_uint32_xrninta, 0xB012629B8C891B267182B61400000001u128, 2147483648   , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_xrninta_417, bid128_to_uint32_xrninta, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_418, bid128_to_uint32_xrninta, 0xB012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_xrninta_419, bid128_to_uint32_xrninta, 0xB012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_420, bid128_to_uint32_xrninta, 0xB012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_xrninta_421, bid128_to_uint32_xrninta, 0xB012629B8C893053744A975EF6800000u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_xrninta_422, bid128_to_uint32_xrninta, 0xB012629B8C893053744A975EF6800001u128, 2147483648   , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_xrninta_423, bid128_to_uint32_xrninta, 0xB012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_424, bid128_to_uint32_xrninta, 0xB012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_xrninta_425, bid128_to_uint32_xrninta, 0xB012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_426, bid128_to_uint32_xrninta, 0xB01600000000003627E8F712373BFFFFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_xrninta_427, bid128_to_uint32_xrninta, 0xB01600000000003627E8F712373C0000u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_xrninta_428, bid128_to_uint32_xrninta, 0xB01600000000003627E8F712373C0001u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_xrninta_429, bid128_to_uint32_xrninta, 0xb018000190402642cffbfdfffff6dfffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_430, bid128_to_uint32_xrninta, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_431, bid128_to_uint32_xrninta, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xrninta_432, bid128_to_uint32_xrninta, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_433, bid128_to_uint32_xrninta, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_434, bid128_to_uint32_xrninta, 0xB0180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xrninta_435, bid128_to_uint32_xrninta, 0xB0180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_436, bid128_to_uint32_xrninta, 0xB0180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_437, bid128_to_uint32_xrninta, 0xB0180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xrninta_438, bid128_to_uint32_xrninta, 0xB0180002B5E3AF19676BAF16B1880001u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_439, bid128_to_uint32_xrninta, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_440, bid128_to_uint32_xrninta, 0xB01800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xrninta_441, bid128_to_uint32_xrninta, 0xB01800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_442, bid128_to_uint32_xrninta, 0xB01800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_443, bid128_to_uint32_xrninta, 0xB01800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xrninta_444, bid128_to_uint32_xrninta, 0xB01800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_445, bid128_to_uint32_xrninta, 0xB01A0000000000004563918244F3FFFFu128, 0            , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_446, bid128_to_uint32_xrninta, 0xB01A0000000000004563918244F40000u128, 2147483648   , 0x01); // -- -(0.5)
dec_test!(bid128_to_uint32_xrninta_447, bid128_to_uint32_xrninta, 0xB01A0000000000004563918244F40001u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_448, bid128_to_uint32_xrninta, 0xB01A0000000000008AC7230489E7FFFFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_xrninta_449, bid128_to_uint32_xrninta, 0xB01A0000000000008AC7230489E80000u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_xrninta_450, bid128_to_uint32_xrninta, 0xB01A0000000000008AC7230489E80001u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_xrninta_451, bid128_to_uint32_xrninta, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xrninta_452, bid128_to_uint32_xrninta, 0xB01A0000000000A2E6C09AD3E0D40000u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xrninta_453, bid128_to_uint32_xrninta, 0xB01A0000000000A2E6C09AD3E0D40001u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xrninta_454, bid128_to_uint32_xrninta, 0xB01A000045639181BA2CDCFB7617FFFFu128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xrninta_455, bid128_to_uint32_xrninta, 0xB01A000045639181BA2CDCFB76180000u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_xrninta_456, bid128_to_uint32_xrninta, 0xB01A000045639181BA2CDCFB76180001u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xrninta_457, bid128_to_uint32_xrninta, 0xB01A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_xrninta_458, bid128_to_uint32_xrninta, 0xB01A00004563918244F4000000000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_xrninta_459, bid128_to_uint32_xrninta, 0xB01A00004563918244F4000000000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_xrninta_460, bid128_to_uint32_xrninta, 0xB01A000045639182CFBB230489E7FFFFu128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xrninta_461, bid128_to_uint32_xrninta, 0xB01A000045639182CFBB230489E80000u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_xrninta_462, bid128_to_uint32_xrninta, 0xB01A000045639182CFBB230489E80001u128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xrninta_463, bid128_to_uint32_xrninta, 0xB01A00008AC72303FF20DCFB7617FFFFu128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xrninta_464, bid128_to_uint32_xrninta, 0xB01A00008AC72303FF20DCFB76180000u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_xrninta_465, bid128_to_uint32_xrninta, 0xB01A00008AC72303FF20DCFB76180001u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xrninta_466, bid128_to_uint32_xrninta, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_xrninta_467, bid128_to_uint32_xrninta, 0xB01A00008AC7230489E8000000000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_xrninta_468, bid128_to_uint32_xrninta, 0xB01A00008AC7230489E8000000000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_xrninta_469, bid128_to_uint32_xrninta, 0xB01A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xrninta_470, bid128_to_uint32_xrninta, 0xB01A00008AC7230514AF230489E80000u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_xrninta_471, bid128_to_uint32_xrninta, 0xB01A00008AC7230514AF230489E80001u128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xrninta_472, bid128_to_uint32_xrninta, 0xB01C00000000000014D1120D7B15FFFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_473, bid128_to_uint32_xrninta, 0xB01C00000000000014D1120D7B160000u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xrninta_474, bid128_to_uint32_xrninta, 0xB01C00000000000014D1120D7B160001u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_475, bid128_to_uint32_xrninta, 0xB01E000000000001A055690D9DB7FFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_476, bid128_to_uint32_xrninta, 0xB01E000000000001A055690D9DB80000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_477, bid128_to_uint32_xrninta, 0xB01E000000000001A055690D9DB80001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_478, bid128_to_uint32_xrninta, 0xB02000000000000029A2241AF62BFFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_479, bid128_to_uint32_xrninta, 0xB02000000000000029A2241AF62C0000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_480, bid128_to_uint32_xrninta, 0xB02000000000000029A2241AF62C0001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_481, bid128_to_uint32_xrninta, 0xB024000000000000006A94D74F42FFFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_482, bid128_to_uint32_xrninta, 0xB024000000000000006A94D74F430000u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_483, bid128_to_uint32_xrninta, 0xB024000000000000006A94D74F430001u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_484, bid128_to_uint32_xrninta, 0xB02A00000000000000000017428106FFu128, 2147483648   , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_xrninta_485, bid128_to_uint32_xrninta, 0xB02A0000000000000000001742810700u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_xrninta_486, bid128_to_uint32_xrninta, 0xB02A0000000000000000001742810701u128, 2147483648   , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_xrninta_487, bid128_to_uint32_xrninta, 0xB02A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_488, bid128_to_uint32_xrninta, 0xB02A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_xrninta_489, bid128_to_uint32_xrninta, 0xB02A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_490, bid128_to_uint32_xrninta, 0xB02A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_491, bid128_to_uint32_xrninta, 0xB02A00000000006C6B935B8019048C00u128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_xrninta_492, bid128_to_uint32_xrninta, 0xB02A00000000006C6B935B8019048C01u128, 2147483648   , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_493, bid128_to_uint32_xrninta, 0xB02C000000000000000002BBA7F521FFu128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xrninta_494, bid128_to_uint32_xrninta, 0xB02C000000000000000002BBA7F52200u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xrninta_495, bid128_to_uint32_xrninta, 0xB02C000000000000000002BBA7F52201u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xrninta_496, bid128_to_uint32_xrninta, 0xB02C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_xrninta_497, bid128_to_uint32_xrninta, 0xB02C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_xrninta_498, bid128_to_uint32_xrninta, 0xB02C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_xrninta_499, bid128_to_uint32_xrninta, 0xB02C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_500, bid128_to_uint32_xrninta, 0xB02C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_xrninta_501, bid128_to_uint32_xrninta, 0xB02C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_502, bid128_to_uint32_xrninta, 0xB02C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_503, bid128_to_uint32_xrninta, 0xB02C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_xrninta_504, bid128_to_uint32_xrninta, 0xB02C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_505, bid128_to_uint32_xrninta, 0xB02E000000000000000000001DCD64FFu128, 0            , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_506, bid128_to_uint32_xrninta, 0xB02E000000000000000000001DCD6500u128, 2147483648   , 0x01); // -- -(0.5)
dec_test!(bid128_to_uint32_xrninta_507, bid128_to_uint32_xrninta, 0xB02E000000000000000000001DCD6501u128, 2147483648   , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_508, bid128_to_uint32_xrninta, 0xB02E000000000000000000003B9AC9FFu128, 2147483648   , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint32_xrninta_509, bid128_to_uint32_xrninta, 0xB02E000000000000000000003B9ACA00u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_xrninta_510, bid128_to_uint32_xrninta, 0xB02E000000000000000000003B9ACA01u128, 2147483648   , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint32_xrninta_511, bid128_to_uint32_xrninta, 0xB02E0000000000000000000059682EFFu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_512, bid128_to_uint32_xrninta, 0xB02E0000000000000000000059682F00u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xrninta_513, bid128_to_uint32_xrninta, 0xB02E0000000000000000000059682F01u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_514, bid128_to_uint32_xrninta, 0xB02E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_xrninta_515, bid128_to_uint32_xrninta, 0xB02E000000000001158E46094F6ACA00u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_xrninta_516, bid128_to_uint32_xrninta, 0xB02E000000000001158E46094F6ACA01u128, 2147483648   , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_xrninta_517, bid128_to_uint32_xrninta, 0xB03000000000000000000006FC23ABFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_518, bid128_to_uint32_xrninta, 0xB03000000000000000000006FC23AC00u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_519, bid128_to_uint32_xrninta, 0xB03000000000000000000006FC23AC01u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_520, bid128_to_uint32_xrninta, 0xB03200000000000000000000B2D05DFFu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_521, bid128_to_uint32_xrninta, 0xB03200000000000000000000B2D05E00u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_522, bid128_to_uint32_xrninta, 0xB03200000000000000000000B2D05E01u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_523, bid128_to_uint32_xrninta, 0xB03800000000000000000000002DDA47u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xrninta_524, bid128_to_uint32_xrninta, 0xB03800000000000000000000002DDA48u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xrninta_525, bid128_to_uint32_xrninta, 0xB03800000000000000000000002DDA49u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xrninta_526, bid128_to_uint32_xrninta, 0xB03A00000000000000000000000003E7u128, 2147483648   , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint32_xrninta_527, bid128_to_uint32_xrninta, 0xB03A00000000000000000000000005DBu128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_528, bid128_to_uint32_xrninta, 0xB03A00000000000000000000000005DCu128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xrninta_529, bid128_to_uint32_xrninta, 0xB03A00000000000000000000000005DDu128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_530, bid128_to_uint32_xrninta, 0xB03A00000000000000000000000495D3u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xrninta_531, bid128_to_uint32_xrninta, 0xB03A00000000000000000000000495D4u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xrninta_532, bid128_to_uint32_xrninta, 0xB03A00000000000000000000000495D5u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xrninta_533, bid128_to_uint32_xrninta, 0xB03C0000000000000000000000000095u128, 2147483648   , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_534, bid128_to_uint32_xrninta, 0xB03C0000000000000000000000000096u128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xrninta_535, bid128_to_uint32_xrninta, 0xB03C0000000000000000000000000097u128, 2147483648   , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_536, bid128_to_uint32_xrninta, 0xB03C0000000000000000000000007561u128, 2147483648   , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xrninta_537, bid128_to_uint32_xrninta, 0xB03C0000000000000000000000007562u128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xrninta_538, bid128_to_uint32_xrninta, 0xB03C0000000000000000000000007563u128, 2147483648   , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xrninta_539, bid128_to_uint32_xrninta, 0xB03C00000000000000000031FFFFFF69u128, 2147483648   , 0x01); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xrninta_540, bid128_to_uint32_xrninta, 0xB03C00000000000000000031FFFFFF6Au128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xrninta_541, bid128_to_uint32_xrninta, 0xB03C00000000000000000031FFFFFF6Bu128, 2147483648   , 0x01); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xrninta_542, bid128_to_uint32_xrninta, 0xB03C00000000000000000031FFFFFFCDu128, 2147483648   , 0x01); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_543, bid128_to_uint32_xrninta, 0xB03C00000000000000000031FFFFFFCEu128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xrninta_544, bid128_to_uint32_xrninta, 0xB03C00000000000000000031FFFFFFCFu128, 2147483648   , 0x01); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_545, bid128_to_uint32_xrninta, 0xB03C0000000000000000003200000031u128, 2147483648   , 0x01); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_546, bid128_to_uint32_xrninta, 0xB03C0000000000000000003200000032u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xrninta_547, bid128_to_uint32_xrninta, 0xB03C0000000000000000003200000033u128, 2147483648   , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_548, bid128_to_uint32_xrninta, 0xB03C00000000000000000063FFFFFFCDu128, 2147483648   , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_549, bid128_to_uint32_xrninta, 0xB03C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xrninta_550, bid128_to_uint32_xrninta, 0xB03C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_551, bid128_to_uint32_xrninta, 0xB03C0000000000000000006400000031u128, 2147483648   , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xrninta_552, bid128_to_uint32_xrninta, 0xB03C0000000000000000006400000032u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xrninta_553, bid128_to_uint32_xrninta, 0xB03C0000000000000000006400000033u128, 2147483648   , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xrninta_554, bid128_to_uint32_xrninta, 0xB03E0000000000000000000000000005u128, 2147483648   , 0x01); // -- -(0.5)
dec_test!(bid128_to_uint32_xrninta_555, bid128_to_uint32_xrninta, 0xB03E000000000000000000000000000Fu128, 2147483648   , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint32_xrninta_556, bid128_to_uint32_xrninta, 0xB03E0000000000000000000000000BB7u128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_557, bid128_to_uint32_xrninta, 0xB03E0000000000000000000000000BB8u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_558, bid128_to_uint32_xrninta, 0xB03E0000000000000000000000000BB9u128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_559, bid128_to_uint32_xrninta, 0xB03E0000000000000000000000000BBDu128, 2147483648   , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint32_xrninta_560, bid128_to_uint32_xrninta, 0xB03E00000000000000000004FFFFFFF1u128, 2147483648   , 0x01); // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xrninta_561, bid128_to_uint32_xrninta, 0xB03E00000000000000000004FFFFFFF5u128, 2147483648   , 0x01); // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xrninta_562, bid128_to_uint32_xrninta, 0xB03E00000000000000000004FFFFFFF6u128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_xrninta_563, bid128_to_uint32_xrninta, 0xB03E00000000000000000004FFFFFFF7u128, 2147483648   , 0x01); // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xrninta_564, bid128_to_uint32_xrninta, 0xB03E00000000000000000004FFFFFFFBu128, 2147483648   , 0x01); // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xrninta_565, bid128_to_uint32_xrninta, 0xB03E00000000000000000004FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_xrninta_566, bid128_to_uint32_xrninta, 0xB03E0000000000000000000500000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_xrninta_567, bid128_to_uint32_xrninta, 0xB03E0000000000000000000500000001u128, 2147483648   , 0x01); // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_xrninta_568, bid128_to_uint32_xrninta, 0xB03E0000000000000000000500000005u128, 2147483648   , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xrninta_569, bid128_to_uint32_xrninta, 0xB03E0000000000000000000500000009u128, 2147483648   , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xrninta_570, bid128_to_uint32_xrninta, 0xB03E000000000000000000050000000Au128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_xrninta_571, bid128_to_uint32_xrninta, 0xB03E000000000000000000050000000Bu128, 2147483648   , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xrninta_572, bid128_to_uint32_xrninta, 0xB03E00000000000000000009FFFFFFF5u128, 2147483648   , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xrninta_573, bid128_to_uint32_xrninta, 0xB03E00000000000000000009FFFFFFF6u128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_xrninta_574, bid128_to_uint32_xrninta, 0xB03E00000000000000000009FFFFFFF7u128, 2147483648   , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xrninta_575, bid128_to_uint32_xrninta, 0xB03E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xrninta_576, bid128_to_uint32_xrninta, 0xB03E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_xrninta_577, bid128_to_uint32_xrninta, 0xB03E0000000000000000000A00000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_xrninta_578, bid128_to_uint32_xrninta, 0xB03E0000000000000000000A00000001u128, 2147483648   , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_xrninta_579, bid128_to_uint32_xrninta, 0xB03E0000000000000000000A00000005u128, 2147483648   , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xrninta_580, bid128_to_uint32_xrninta, 0xB03E0000000000000000000A00000009u128, 2147483648   , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xrninta_581, bid128_to_uint32_xrninta, 0xB03E0000000000000000000A0000000Au128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_xrninta_582, bid128_to_uint32_xrninta, 0xB03E0000000000000000000A0000000Bu128, 2147483648   , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xrninta_583, bid128_to_uint32_xrninta, 0xB03E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_xrninta_584, bid128_to_uint32_xrninta, 0xB03E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_xrninta_585, bid128_to_uint32_xrninta, 0xB03E0000000000000000002E90EDD005u128, 2147483648   , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_xrninta_586, bid128_to_uint32_xrninta, 0xB03E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_xrninta_587, bid128_to_uint32_xrninta, 0xB0400000000000000000000000000001u128, 2147483648   , 0x01); // -- -(1)
dec_test!(bid128_to_uint32_xrninta_588, bid128_to_uint32_xrninta, 0xB040000000000000000000000000012Bu128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_589, bid128_to_uint32_xrninta, 0xB040000000000000000000000000012Cu128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_590, bid128_to_uint32_xrninta, 0xB040000000000000000000000000012Du128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_591, bid128_to_uint32_xrninta, 0xB040000000000000000000007FFFFFFFu128, 2147483648   , 0x01); // -- -(2^31-1)
dec_test!(bid128_to_uint32_xrninta_592, bid128_to_uint32_xrninta, 0xB0400000000000000000000080000000u128, 2147483648   , 0x01); // -- -(2^31)
dec_test!(bid128_to_uint32_xrninta_593, bid128_to_uint32_xrninta, 0xB0400000000000000000000080000001u128, 2147483648   , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_uint32_xrninta_594, bid128_to_uint32_xrninta, 0xB04000000000000000000000FFFFFFFFu128, 2147483648   , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_uint32_xrninta_595, bid128_to_uint32_xrninta, 0xB0400000000000000000000100000000u128, 2147483648   , 0x01); // -- -(2^32)
dec_test!(bid128_to_uint32_xrninta_596, bid128_to_uint32_xrninta, 0xB0400000000000000000000100000001u128, 2147483648   , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_uint32_xrninta_597, bid128_to_uint32_xrninta, 0xB04000000000000000000004A817C7FFu128, 2147483648   , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint32_xrninta_598, bid128_to_uint32_xrninta, 0xB04000000000000000000004A817C801u128, 2147483648   , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint32_xrninta_599, bid128_to_uint32_xrninta, 0xB042000000000000000000000000001Du128, 2147483648   , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint32_xrninta_600, bid128_to_uint32_xrninta, 0xB042000000000000000000000000001Eu128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_601, bid128_to_uint32_xrninta, 0xB042000000000000000000000000001Fu128, 2147483648   , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint32_xrninta_602, bid128_to_uint32_xrninta, 0xB04200000000000000000000773593FFu128, 2147483648   , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_xrninta_603, bid128_to_uint32_xrninta, 0xB0420000000000000000000077359400u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_xrninta_604, bid128_to_uint32_xrninta, 0xB0420000000000000000000077359401u128, 2147483648   , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_xrninta_605, bid128_to_uint32_xrninta, 0xB0440000000000000000000000000003u128, 2147483648   , 0x01); // -- -(300)
dec_test!(bid128_to_uint32_xrninta_606, bid128_to_uint32_xrninta, 0xB0520000000000000000000000000004u128, 2147483648   , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint32_xrninta_607, bid128_to_uint32_xrninta, 0xB0520000000000000000000000000005u128, 2147483648   , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint32_xrninta_608, bid128_to_uint32_xrninta, 0xB0540000000000000000000000000002u128, 2147483648   , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint32_xrninta_609, bid128_to_uint32_xrninta, 0xcaff92e24129d1e6058c8f313c03ef44u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_610, bid128_to_uint32_xrninta, 0xcbffffaffeffffbf89f79370dbf777a7u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xrninta_611, bid128_to_uint32_xrninta, 0xcd2833bce77d892a3543b570982cf57eu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_612, bid128_to_uint32_xrninta, 0xce682273c7dd244eaa945d14deab1e41u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_613, bid128_to_uint32_xrninta, 0xd22f59c4a430bc8d05a23e4238ad815cu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_614, bid128_to_uint32_xrninta, 0xd762038b2d7888e8b5a7b0215f1b1930u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_615, bid128_to_uint32_xrninta, 0xf8b5ae2fb7a77f9b72da1288def1ab84u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_616, bid128_to_uint32_xrninta, 0xfdffffffffdfffff0001000004000000u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_617, bid128_to_uint32_xrninta,  "Infinity"                           , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xrninta_618, bid128_to_uint32_xrninta,  "QNaN"                               , 0x80000000u32, 0x01);
