/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_uint32_xceil_001, bid128_to_uint32_xceil, "-0"                                  , 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_002, bid128_to_uint32_xceil, 0                                     , 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_003, bid128_to_uint32_xceil, 0x00000000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_004, bid128_to_uint32_xceil, 0x00000000000000000000000001010000u128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_005, bid128_to_uint32_xceil, 0x000000000000000043c0251c5d6225aeu128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_006, bid128_to_uint32_xceil, 0x0000000000000000fffffeffffff7bffu128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_007, bid128_to_uint32_xceil, "+0.000000000010000E0"                , 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_008, bid128_to_uint32_xceil, 0x0001ed09bead87c0378d8e62ffffffffu128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_009, bid128_to_uint32_xceil, 0x0001ed09bead87c0378d8e64ffffffffu128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_010, bid128_to_uint32_xceil, 0x02620000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_011, bid128_to_uint32_xceil, 0x0474a2fb3301523c6e0e252b5c10f781u128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_012, bid128_to_uint32_xceil, 0x0c0e0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_013, bid128_to_uint32_xceil, 0x0f53278c5ecfbed77892b934a613368bu128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_014, bid128_to_uint32_xceil, 1073741824                            , 1073741824   , 0x00);
dec_test!(bid128_to_uint32_xceil_015, bid128_to_uint32_xceil, 1                                     , 1            , 0x00);
dec_test!(bid128_to_uint32_xceil_016, bid128_to_uint32_xceil, "+110.01E0"                           , 111          , 0x20);
dec_test!(bid128_to_uint32_xceil_017, bid128_to_uint32_xceil, 0x1ae675b11cdf8b2426c161e794d9ed88u128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_018, bid128_to_uint32_xceil, 0x1c9c0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_019, bid128_to_uint32_xceil, 0x20052bedec4188893f8be0a06b33aa41u128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_020, bid128_to_uint32_xceil, 0x21a06fe6f201ac1c792be00d63d4f4beu128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_021, bid128_to_uint32_xceil, 0x25fa0a239b7be1be797f373779ebebe2u128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_022, bid128_to_uint32_xceil, 0x2a9b19df6c0b1393dcc7b75da61dc21au128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_023, bid128_to_uint32_xceil, 0x2e1d3e562627d9636a6f87546d18e1e7u128, 1            , 0x20);
dec_test!(bid128_to_uint32_xceil_024, bid128_to_uint32_xceil, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 1            , 0x20);  // -- 0.5-ulp
dec_test!(bid128_to_uint32_xceil_025, bid128_to_uint32_xceil, 0x2FFCF684DF56C3E01BC6C73200000000u128, 1            , 0x20);  // -- 0.5
dec_test!(bid128_to_uint32_xceil_026, bid128_to_uint32_xceil, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1            , 0x20);  // -- 0.5+ulp
dec_test!(bid128_to_uint32_xceil_027, bid128_to_uint32_xceil, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1            , 0x20);  // -- 0.999-ulp
dec_test!(bid128_to_uint32_xceil_028, bid128_to_uint32_xceil, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1            , 0x20);  // -- 0.999
dec_test!(bid128_to_uint32_xceil_029, bid128_to_uint32_xceil, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1            , 0x20);  // -- 0.999+ulp
dec_test!(bid128_to_uint32_xceil_030, bid128_to_uint32_xceil, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1            , 0x20);  // -- 1-ulp
dec_test!(bid128_to_uint32_xceil_031, bid128_to_uint32_xceil, 0x2FFE314DC6448D9338C15B0A00000000u128, 1            , 0x00);  // -- 1
dec_test!(bid128_to_uint32_xceil_032, bid128_to_uint32_xceil, 0x2FFE314DC6448D9338C15B0A00000001u128, 2            , 0x20);  // -- 1+ulp
dec_test!(bid128_to_uint32_xceil_033, bid128_to_uint32_xceil, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 2            , 0x20);  // -- 1.5-ulp
dec_test!(bid128_to_uint32_xceil_034, bid128_to_uint32_xceil, 0x2FFE49F4A966D45CD522088F00000000u128, 2            , 0x20);  // -- 1.5
dec_test!(bid128_to_uint32_xceil_035, bid128_to_uint32_xceil, 0x2FFE49F4A966D45CD522088F00000001u128, 2            , 0x20);  // -- 1.5+ulp
dec_test!(bid128_to_uint32_xceil_036, bid128_to_uint32_xceil, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_037, bid128_to_uint32_xceil, 0x300293E952CDA8B9AA44111E00000000u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_038, bid128_to_uint32_xceil, 0x300293E952CDA8B9AA44111E00000001u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_039, bid128_to_uint32_xceil, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 301          , 0x20);  // -- 300.5-ulp
dec_test!(bid128_to_uint32_xceil_040, bid128_to_uint32_xceil, 0x300294286EACB8CB0A8CB6B140000000u128, 301          , 0x20);  // -- 300.5
dec_test!(bid128_to_uint32_xceil_041, bid128_to_uint32_xceil, 0x300294286EACB8CB0A8CB6B140000001u128, 301          , 0x20);  // -- 300.5+ulp
dec_test!(bid128_to_uint32_xceil_042, bid128_to_uint32_xceil, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_043, bid128_to_uint32_xceil, 0x30040ECA8847C4129106CE8300000000u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_044, bid128_to_uint32_xceil, 0x30040ECA8847C4129106CE8300000001u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_045, bid128_to_uint32_xceil, 0x30062cca3a48850edf5fffffdf3cfff6u128, 9085         , 0x20);
dec_test!(bid128_to_uint32_xceil_046, bid128_to_uint32_xceil, 0x30080008080082780008000404100000u128, 64           , 0x20);
dec_test!(bid128_to_uint32_xceil_047, bid128_to_uint32_xceil, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_048, bid128_to_uint32_xceil, 0x300A0003C95A2F0B4856475FE0000000u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_049, bid128_to_uint32_xceil, 0x300A0003C95A2F0B4856475FE0000001u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_050, bid128_to_uint32_xceil, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_051, bid128_to_uint32_xceil, 0x300C000060EF6B1ABA6F072330000000u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_052, bid128_to_uint32_xceil, 0x300C000060EF6B1ABA6F072330000001u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_053, bid128_to_uint32_xceil, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483647   , 0x20);  // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_xceil_054, bid128_to_uint32_xceil, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483647   , 0x20);  // -- 2^31-1.5
dec_test!(bid128_to_uint32_xceil_055, bid128_to_uint32_xceil, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483647   , 0x20);  // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_xceil_056, bid128_to_uint32_xceil, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483647   , 0x20);  // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_xceil_057, bid128_to_uint32_xceil, 0x301069E10DE692B4B4B133125F000000u128, 2147483647   , 0x00);  // -- 2^31-1
dec_test!(bid128_to_uint32_xceil_058, bid128_to_uint32_xceil, 0x301069E10DE692B4B4B133125F000001u128, 2147483648   , 0x20);  // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_xceil_059, bid128_to_uint32_xceil, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483648   , 0x20);  // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_xceil_060, bid128_to_uint32_xceil, 0x301069E10DE6FC95C29899892F800000u128, 2147483648   , 0x20);  // -- 2^31-0.5
dec_test!(bid128_to_uint32_xceil_061, bid128_to_uint32_xceil, 0x301069E10DE6FC95C29899892F800001u128, 2147483648   , 0x20);  // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_xceil_062, bid128_to_uint32_xceil, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x20);  // -- 2^31-ulp
dec_test!(bid128_to_uint32_xceil_063, bid128_to_uint32_xceil, 0x301069E10DE76676D080000000000000u128, 2147483648   , 0x00);  // -- 2^31
dec_test!(bid128_to_uint32_xceil_064, bid128_to_uint32_xceil, 0x301069E10DE76676D080000000000001u128, 2147483649   , 0x20);  // -- 2^31+ulp
dec_test!(bid128_to_uint32_xceil_065, bid128_to_uint32_xceil, 0x301069E10DE7D057DE676676D07FFFFFu128, 2147483649   , 0x20);  // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_xceil_066, bid128_to_uint32_xceil, 0x301069E10DE7D057DE676676D0800000u128, 2147483649   , 0x20);  // -- 2^31+0.5
dec_test!(bid128_to_uint32_xceil_067, bid128_to_uint32_xceil, 0x301069E10DE7D057DE676676D0800001u128, 2147483649   , 0x20);  // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_xceil_068, bid128_to_uint32_xceil, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483649   , 0x20);  // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_xceil_069, bid128_to_uint32_xceil, 0x301069E10DE83A38EC4ECCEDA1000000u128, 2147483649   , 0x00);  // -- 2^31+1
dec_test!(bid128_to_uint32_xceil_070, bid128_to_uint32_xceil, 0x301069E10DE83A38EC4ECCEDA1000001u128, 2147483650   , 0x20);  // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_xceil_071, bid128_to_uint32_xceil, 0x30108000000100000010024502000840u128, 0x9ABE14CF   , 0x20);
dec_test!(bid128_to_uint32_xceil_072, bid128_to_uint32_xceil, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000   , 0x20);  // -- 4e9-ulp
dec_test!(bid128_to_uint32_xceil_073, bid128_to_uint32_xceil, 0x3010C5371912364CE3056C2800000000u128, 4000000000   , 0x00);  // -- 4e9
dec_test!(bid128_to_uint32_xceil_074, bid128_to_uint32_xceil, 0x3010C5371912364CE3056C2800000001u128, 4000000001   , 0x20);  // -- 4e9+ulp
dec_test!(bid128_to_uint32_xceil_075, bid128_to_uint32_xceil, 0x3010D3C21BCDF92B853133125EFFFFFFu128, 4294967295   , 0x20);  // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_xceil_076, bid128_to_uint32_xceil, 0x3010D3C21BCDF92B853133125F000000u128, 4294967295   , 0x00);  // -- 2^32-1
dec_test!(bid128_to_uint32_xceil_077, bid128_to_uint32_xceil, 0x3010D3C21BCDF92B853133125F000001u128, 2147483648   , 0x01);  // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_xceil_078, bid128_to_uint32_xceil, 0x3010D3C21BCE630C931899892F7FFFFFu128, 2147483648   , 0x01);  // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_xceil_079, bid128_to_uint32_xceil, 0x3010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01);  // -- 2^32-0.5
dec_test!(bid128_to_uint32_xceil_080, bid128_to_uint32_xceil, 0x3010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01);  // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_xceil_081, bid128_to_uint32_xceil, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01);  // -- 2^32-ulp
dec_test!(bid128_to_uint32_xceil_082, bid128_to_uint32_xceil, 0x3010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01);  // -- 2^32
dec_test!(bid128_to_uint32_xceil_083, bid128_to_uint32_xceil, 0x3010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01);  // -- 2^32+ulp
dec_test!(bid128_to_uint32_xceil_084, bid128_to_uint32_xceil, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01);  // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_xceil_085, bid128_to_uint32_xceil, 0x3010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01);  // -- 2^32+0.5
dec_test!(bid128_to_uint32_xceil_086, bid128_to_uint32_xceil, 0x3010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01);  // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_xceil_087, bid128_to_uint32_xceil, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01);  // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_xceil_088, bid128_to_uint32_xceil, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01);  // -- 2^32+1
dec_test!(bid128_to_uint32_xceil_089, bid128_to_uint32_xceil, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01);  // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_xceil_090, bid128_to_uint32_xceil, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01);  // -- 5e9-ulp
dec_test!(bid128_to_uint32_xceil_091, bid128_to_uint32_xceil, 0x3010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01);  // -- 5e9
dec_test!(bid128_to_uint32_xceil_092, bid128_to_uint32_xceil, 0x3010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01);  // -- 5e9+ulp
dec_test!(bid128_to_uint32_xceil_093, bid128_to_uint32_xceil, 0x30117e6b61d790ab7fde7feddde9b97du128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_094, bid128_to_uint32_xceil, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01);  // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_xceil_095, bid128_to_uint32_xceil, 0x3012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01);  // -- 2e10-1.5
dec_test!(bid128_to_uint32_xceil_096, bid128_to_uint32_xceil, 0x3012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01);  // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_xceil_097, bid128_to_uint32_xceil, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01);  // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_xceil_098, bid128_to_uint32_xceil, 0x3012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01);  // -- 2e10-1
dec_test!(bid128_to_uint32_xceil_099, bid128_to_uint32_xceil, 0x3012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01);  // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_xceil_100, bid128_to_uint32_xceil, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01);  // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_xceil_101, bid128_to_uint32_xceil, 0x3012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01);  // -- 2e10-0.5
dec_test!(bid128_to_uint32_xceil_102, bid128_to_uint32_xceil, 0x3012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01);  // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_xceil_103, bid128_to_uint32_xceil, 0x3012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01);  // -- 2e10-ulp
dec_test!(bid128_to_uint32_xceil_104, bid128_to_uint32_xceil, 0x3012629B8C891B267182B61400000000u128, 2147483648   , 0x01);  // -- 2e10
dec_test!(bid128_to_uint32_xceil_105, bid128_to_uint32_xceil, 0x3012629B8C891B267182B61400000001u128, 2147483648   , 0x01);  // -- 2e10+ulp
dec_test!(bid128_to_uint32_xceil_106, bid128_to_uint32_xceil, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01);  // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_xceil_107, bid128_to_uint32_xceil, 0x3012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01);  // -- 2e10+0.5
dec_test!(bid128_to_uint32_xceil_108, bid128_to_uint32_xceil, 0x3012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01);  // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_xceil_109, bid128_to_uint32_xceil, 0x3012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01);  // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_xceil_110, bid128_to_uint32_xceil, 0x3012629B8C893053744A975EF6800000u128, 2147483648   , 0x01);  // -- 2e10+1
dec_test!(bid128_to_uint32_xceil_111, bid128_to_uint32_xceil, 0x3012629B8C893053744A975EF6800001u128, 2147483648   , 0x01);  // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_xceil_112, bid128_to_uint32_xceil, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01);  // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_xceil_113, bid128_to_uint32_xceil, 0x3012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01);  // -- 2e10+1.5
dec_test!(bid128_to_uint32_xceil_114, bid128_to_uint32_xceil, 0x3012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01);  // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_xceil_115, bid128_to_uint32_xceil, 0x3014000050048499b045c090e2abc942u128, 2476427      , 0x20);
dec_test!(bid128_to_uint32_xceil_116, bid128_to_uint32_xceil, 0x301600000000003627E8F712373BFFFFu128, 1            , 0x20);  // -- 0.999-ulp
dec_test!(bid128_to_uint32_xceil_117, bid128_to_uint32_xceil, 0x301600000000003627E8F712373C0000u128, 1            , 0x20);  // -- 0.999
dec_test!(bid128_to_uint32_xceil_118, bid128_to_uint32_xceil, 0x301600000000003627E8F712373C0001u128, 1            , 0x20);  // -- 0.999+ulp
dec_test!(bid128_to_uint32_xceil_119, bid128_to_uint32_xceil, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483647   , 0x20);  // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_xceil_120, bid128_to_uint32_xceil, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483647   , 0x20);  // -- 2^31-1.5
dec_test!(bid128_to_uint32_xceil_121, bid128_to_uint32_xceil, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483647   , 0x20);  // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_xceil_122, bid128_to_uint32_xceil, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483648   , 0x20);  // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_xceil_123, bid128_to_uint32_xceil, 0x30180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x20);  // -- 2^31-0.5
dec_test!(bid128_to_uint32_xceil_124, bid128_to_uint32_xceil, 0x30180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x20);  // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_xceil_125, bid128_to_uint32_xceil, 0x30180002B5E3AF19676BAF16B187FFFFu128, 2147483649   , 0x20);  // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_xceil_126, bid128_to_uint32_xceil, 0x30180002B5E3AF19676BAF16B1880000u128, 2147483649   , 0x20);  // -- 2^31+0.5
dec_test!(bid128_to_uint32_xceil_127, bid128_to_uint32_xceil, 0x30180002B5E3AF19676BAF16B1880001u128, 2147483649   , 0x20);  // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_xceil_128, bid128_to_uint32_xceil, 0x301800056BC75E2AAD2C50E94E77FFFFu128, 2147483648   , 0x01);  // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_xceil_129, bid128_to_uint32_xceil, 0x301800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01);  // -- 2^32-0.5
dec_test!(bid128_to_uint32_xceil_130, bid128_to_uint32_xceil, 0x301800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01);  // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_xceil_131, bid128_to_uint32_xceil, 0x301800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01);  // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_xceil_132, bid128_to_uint32_xceil, 0x301800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01);  // -- 2^32+0.5
dec_test!(bid128_to_uint32_xceil_133, bid128_to_uint32_xceil, 0x301800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01);  // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_xceil_134, bid128_to_uint32_xceil, 0x301A0000000000004563918244F3FFFFu128, 1            , 0x20);  // -- 0.5-ulp
dec_test!(bid128_to_uint32_xceil_135, bid128_to_uint32_xceil, 0x301A0000000000004563918244F40000u128, 1            , 0x20);  // -- 0.5
dec_test!(bid128_to_uint32_xceil_136, bid128_to_uint32_xceil, 0x301A0000000000004563918244F40001u128, 1            , 0x20);  // -- 0.5+ulp
dec_test!(bid128_to_uint32_xceil_137, bid128_to_uint32_xceil, 0x301A0000000000008AC7230489E7FFFFu128, 1            , 0x20);  // -- 1-ulp
dec_test!(bid128_to_uint32_xceil_138, bid128_to_uint32_xceil, 0x301A0000000000008AC7230489E80000u128, 1            , 0x00);  // -- 1
dec_test!(bid128_to_uint32_xceil_139, bid128_to_uint32_xceil, 0x301A0000000000008AC7230489E80001u128, 2            , 0x20);  // -- 1+ulp
dec_test!(bid128_to_uint32_xceil_140, bid128_to_uint32_xceil, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 301          , 0x20);  // -- 300.5-ulp
dec_test!(bid128_to_uint32_xceil_141, bid128_to_uint32_xceil, 0x301A0000000000A2E6C09AD3E0D40000u128, 301          , 0x20);  // -- 300.5
dec_test!(bid128_to_uint32_xceil_142, bid128_to_uint32_xceil, 0x301A0000000000A2E6C09AD3E0D40001u128, 301          , 0x20);  // -- 300.5+ulp
dec_test!(bid128_to_uint32_xceil_143, bid128_to_uint32_xceil, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483647   , 0x20);  // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_xceil_144, bid128_to_uint32_xceil, 0x301A000045639181BA2CDCFB76180000u128, 2147483647   , 0x00);  // -- 2^31-1
dec_test!(bid128_to_uint32_xceil_145, bid128_to_uint32_xceil, 0x301A000045639181BA2CDCFB76180001u128, 2147483648   , 0x20);  // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_xceil_146, bid128_to_uint32_xceil, 0x301A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x20);  // -- 2^31-ulp
dec_test!(bid128_to_uint32_xceil_147, bid128_to_uint32_xceil, 0x301A00004563918244F4000000000000u128, 2147483648   , 0x00);  // -- 2^31
dec_test!(bid128_to_uint32_xceil_148, bid128_to_uint32_xceil, 0x301A00004563918244F4000000000001u128, 2147483649   , 0x20);  // -- 2^31+ulp
dec_test!(bid128_to_uint32_xceil_149, bid128_to_uint32_xceil, 0x301A000045639182CFBB230489E7FFFFu128, 2147483649   , 0x20);  // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_xceil_150, bid128_to_uint32_xceil, 0x301A000045639182CFBB230489E80000u128, 2147483649   , 0x00);  // -- 2^31+1
dec_test!(bid128_to_uint32_xceil_151, bid128_to_uint32_xceil, 0x301A000045639182CFBB230489E80001u128, 2147483650   , 0x20);  // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_xceil_152, bid128_to_uint32_xceil, 0x301a000060984001ffdfbffff7ffffffu128, 0xB22F91EA   , 0x20);
dec_test!(bid128_to_uint32_xceil_153, bid128_to_uint32_xceil, 0x301A00008AC72303FF20DCFB7617FFFFu128, 4294967295   , 0x20);  // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_xceil_154, bid128_to_uint32_xceil, 0x301A00008AC72303FF20DCFB76180000u128, 4294967295   , 0x00);  // -- 2^32-1
dec_test!(bid128_to_uint32_xceil_155, bid128_to_uint32_xceil, 0x301A00008AC72303FF20DCFB76180001u128, 2147483648   , 0x01);  // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_xceil_156, bid128_to_uint32_xceil, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01);  // -- 2^32-ulp
dec_test!(bid128_to_uint32_xceil_157, bid128_to_uint32_xceil, 0x301A00008AC7230489E8000000000000u128, 2147483648   , 0x01);  // -- 2^32
dec_test!(bid128_to_uint32_xceil_158, bid128_to_uint32_xceil, 0x301A00008AC7230489E8000000000001u128, 2147483648   , 0x01);  // -- 2^32+ulp
dec_test!(bid128_to_uint32_xceil_159, bid128_to_uint32_xceil, 0x301A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01);  // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_xceil_160, bid128_to_uint32_xceil, 0x301A00008AC7230514AF230489E80000u128, 2147483648   , 0x01);  // -- 2^32+1
dec_test!(bid128_to_uint32_xceil_161, bid128_to_uint32_xceil, 0x301A00008AC7230514AF230489E80001u128, 2147483648   , 0x01);  // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_xceil_162, bid128_to_uint32_xceil, 0x301C00000000000014D1120D7B15FFFFu128, 2            , 0x20);  // -- 1.5-ulp
dec_test!(bid128_to_uint32_xceil_163, bid128_to_uint32_xceil, 0x301C00000000000014D1120D7B160000u128, 2            , 0x20);  // -- 1.5
dec_test!(bid128_to_uint32_xceil_164, bid128_to_uint32_xceil, 0x301C00000000000014D1120D7B160001u128, 2            , 0x20);  // -- 1.5+ulp
dec_test!(bid128_to_uint32_xceil_165, bid128_to_uint32_xceil, 0x301E000000000001A055690D9DB7FFFFu128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_166, bid128_to_uint32_xceil, 0x301E000000000001A055690D9DB80000u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_167, bid128_to_uint32_xceil, 0x301E000000000001A055690D9DB80001u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_168, bid128_to_uint32_xceil, 0x302000000000000029A2241AF62BFFFFu128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_169, bid128_to_uint32_xceil, 0x302000000000000029A2241AF62C0000u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_170, bid128_to_uint32_xceil, 0x302000000000000029A2241AF62C0001u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_171, bid128_to_uint32_xceil, 0x3024000000000000006A94D74F42FFFFu128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_172, bid128_to_uint32_xceil, 0x3024000000000000006A94D74F430000u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_173, bid128_to_uint32_xceil, 0x3024000000000000006A94D74F430001u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_174, bid128_to_uint32_xceil, 0x302A00000000000000000017428106FFu128, 1            , 0x20);  // -- 0.999-ulp
dec_test!(bid128_to_uint32_xceil_175, bid128_to_uint32_xceil, 0x302A0000000000000000001742810700u128, 1            , 0x20);  // -- 0.999
dec_test!(bid128_to_uint32_xceil_176, bid128_to_uint32_xceil, 0x302A0000000000000000001742810701u128, 1            , 0x20);  // -- 0.999+ulp
dec_test!(bid128_to_uint32_xceil_177, bid128_to_uint32_xceil, 0x302A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01);  // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint32_xceil_178, bid128_to_uint32_xceil, 0x302A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01);  // -- 2e10-1.5
dec_test!(bid128_to_uint32_xceil_179, bid128_to_uint32_xceil, 0x302A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01);  // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint32_xceil_180, bid128_to_uint32_xceil, 0x302A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01);  // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint32_xceil_181, bid128_to_uint32_xceil, 0x302A00000000006C6B935B8019048C00u128, 2147483648   , 0x01);  // -- 2e10-0.5
dec_test!(bid128_to_uint32_xceil_182, bid128_to_uint32_xceil, 0x302A00000000006C6B935B8019048C01u128, 2147483648   , 0x01);  // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint32_xceil_183, bid128_to_uint32_xceil, 0x302C000000000000000002BBA7F521FFu128, 301          , 0x20);  // -- 300.5-ulp
dec_test!(bid128_to_uint32_xceil_184, bid128_to_uint32_xceil, 0x302C000000000000000002BBA7F52200u128, 301          , 0x20);  // -- 300.5
dec_test!(bid128_to_uint32_xceil_185, bid128_to_uint32_xceil, 0x302C000000000000000002BBA7F52201u128, 301          , 0x20);  // -- 300.5+ulp
dec_test!(bid128_to_uint32_xceil_186, bid128_to_uint32_xceil, 0x302C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01);  // -- 2e10-1-ulp
dec_test!(bid128_to_uint32_xceil_187, bid128_to_uint32_xceil, 0x302C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01);  // -- 2e10-1
dec_test!(bid128_to_uint32_xceil_188, bid128_to_uint32_xceil, 0x302C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01);  // -- 2e10-1+ulp
dec_test!(bid128_to_uint32_xceil_189, bid128_to_uint32_xceil, 0x302C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01);  // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint32_xceil_190, bid128_to_uint32_xceil, 0x302C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01);  // -- 2e10+0.5
dec_test!(bid128_to_uint32_xceil_191, bid128_to_uint32_xceil, 0x302C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01);  // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint32_xceil_192, bid128_to_uint32_xceil, 0x302C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01);  // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint32_xceil_193, bid128_to_uint32_xceil, 0x302C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01);  // -- 2e10+1.5
dec_test!(bid128_to_uint32_xceil_194, bid128_to_uint32_xceil, 0x302C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01);  // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint32_xceil_195, bid128_to_uint32_xceil, 0x302E000000000000000000001DCD64FFu128, 1            , 0x20);  // -- 0.5-ulp
dec_test!(bid128_to_uint32_xceil_196, bid128_to_uint32_xceil, 0x302E000000000000000000001DCD6500u128, 1            , 0x20);  // -- 0.5
dec_test!(bid128_to_uint32_xceil_197, bid128_to_uint32_xceil, 0x302E000000000000000000001DCD6501u128, 1            , 0x20);  // -- 0.5+ulp
dec_test!(bid128_to_uint32_xceil_198, bid128_to_uint32_xceil, 0x302E000000000000000000003B9AC9FFu128, 1            , 0x20);  // -- 1-ulp
dec_test!(bid128_to_uint32_xceil_199, bid128_to_uint32_xceil, 0x302E000000000000000000003B9ACA00u128, 1            , 0x00);  // -- 1
dec_test!(bid128_to_uint32_xceil_200, bid128_to_uint32_xceil, 0x302E000000000000000000003B9ACA01u128, 2            , 0x20);  // -- 1+ulp
dec_test!(bid128_to_uint32_xceil_201, bid128_to_uint32_xceil, 0x302E0000000000000000000059682EFFu128, 2            , 0x20);  // -- 1.5-ulp
dec_test!(bid128_to_uint32_xceil_202, bid128_to_uint32_xceil, 0x302E0000000000000000000059682F00u128, 2            , 0x20);  // -- 1.5
dec_test!(bid128_to_uint32_xceil_203, bid128_to_uint32_xceil, 0x302E0000000000000000000059682F01u128, 2            , 0x20);  // -- 1.5+ulp
dec_test!(bid128_to_uint32_xceil_204, bid128_to_uint32_xceil, 0x302E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01);  // -- 2e10+1-ulp
dec_test!(bid128_to_uint32_xceil_205, bid128_to_uint32_xceil, 0x302E000000000001158E46094F6ACA00u128, 2147483648   , 0x01);  // -- 2e10+1
dec_test!(bid128_to_uint32_xceil_206, bid128_to_uint32_xceil, 0x302E000000000001158E46094F6ACA01u128, 2147483648   , 0x01);  // -- 2e10+1+ulp
dec_test!(bid128_to_uint32_xceil_207, bid128_to_uint32_xceil, 0x303000000000000000000006FC23ABFFu128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_208, bid128_to_uint32_xceil, 0x303000000000000000000006FC23AC00u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_209, bid128_to_uint32_xceil, 0x303000000000000000000006FC23AC01u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_210, bid128_to_uint32_xceil, 0x303200000000000000000000B2D05DFFu128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_211, bid128_to_uint32_xceil, 0x303200000000000000000000B2D05E00u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_212, bid128_to_uint32_xceil, 0x303200000000000000000000B2D05E01u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_213, bid128_to_uint32_xceil, 0x303800000000000000000000002DDA47u128, 301          , 0x20);  // -- 300.5-ulp
dec_test!(bid128_to_uint32_xceil_214, bid128_to_uint32_xceil, 0x303800000000000000000000002DDA48u128, 301          , 0x20);  // -- 300.5
dec_test!(bid128_to_uint32_xceil_215, bid128_to_uint32_xceil, 0x303800000000000000000000002DDA49u128, 301          , 0x20);  // -- 300.5+ulp
dec_test!(bid128_to_uint32_xceil_216, bid128_to_uint32_xceil, 0x303A00000000000000000000000003E7u128, 1            , 0x20);  // -- 0.999
dec_test!(bid128_to_uint32_xceil_217, bid128_to_uint32_xceil, 0x303A00000000000000000000000005DBu128, 2            , 0x20);  // -- 1.5-ulp
dec_test!(bid128_to_uint32_xceil_218, bid128_to_uint32_xceil, 0x303A00000000000000000000000005DCu128, 2            , 0x20);  // -- 1.5
dec_test!(bid128_to_uint32_xceil_219, bid128_to_uint32_xceil, 0x303A00000000000000000000000005DDu128, 2            , 0x20);  // -- 1.5+ulp
dec_test!(bid128_to_uint32_xceil_220, bid128_to_uint32_xceil, 0x303A00000000000000000000000495D3u128, 301          , 0x20);  // -- 300.5-ulp
dec_test!(bid128_to_uint32_xceil_221, bid128_to_uint32_xceil, 0x303A00000000000000000000000495D4u128, 301          , 0x20);  // -- 300.5
dec_test!(bid128_to_uint32_xceil_222, bid128_to_uint32_xceil, 0x303A00000000000000000000000495D5u128, 301          , 0x20);  // -- 300.5+ulp
dec_test!(bid128_to_uint32_xceil_223, bid128_to_uint32_xceil, 0x303C0000000000000000000000000095u128, 2            , 0x20);  // -- 1.5-ulp
dec_test!(bid128_to_uint32_xceil_224, bid128_to_uint32_xceil, 0x303C0000000000000000000000000096u128, 2            , 0x20);  // -- 1.5
dec_test!(bid128_to_uint32_xceil_225, bid128_to_uint32_xceil, 0x303C0000000000000000000000000097u128, 2            , 0x20);  // -- 1.5+ulp
dec_test!(bid128_to_uint32_xceil_226, bid128_to_uint32_xceil, 0x303C0000000000000000000000007561u128, 301          , 0x20);  // -- 300.5-ulp
dec_test!(bid128_to_uint32_xceil_227, bid128_to_uint32_xceil, 0x303C0000000000000000000000007562u128, 301          , 0x20);  // -- 300.5
dec_test!(bid128_to_uint32_xceil_228, bid128_to_uint32_xceil, 0x303C0000000000000000000000007563u128, 301          , 0x20);  // -- 300.5+ulp
dec_test!(bid128_to_uint32_xceil_229, bid128_to_uint32_xceil, 0x303C00000000000000000031FFFFFF69u128, 2147483647   , 0x20);  // -- 2^31-1.5-ulp
dec_test!(bid128_to_uint32_xceil_230, bid128_to_uint32_xceil, 0x303C00000000000000000031FFFFFF6Au128, 2147483647   , 0x20);  // -- 2^31-1.5
dec_test!(bid128_to_uint32_xceil_231, bid128_to_uint32_xceil, 0x303C00000000000000000031FFFFFF6Bu128, 2147483647   , 0x20);  // -- 2^31-1.5+ulp
dec_test!(bid128_to_uint32_xceil_232, bid128_to_uint32_xceil, 0x303C00000000000000000031FFFFFFCDu128, 2147483648   , 0x20);  // -- 2^31-0.5-ulp
dec_test!(bid128_to_uint32_xceil_233, bid128_to_uint32_xceil, 0x303C00000000000000000031FFFFFFCEu128, 2147483648   , 0x20);  // -- 2^31-0.5
dec_test!(bid128_to_uint32_xceil_234, bid128_to_uint32_xceil, 0x303C00000000000000000031FFFFFFCFu128, 2147483648   , 0x20);  // -- 2^31-0.5+ulp
dec_test!(bid128_to_uint32_xceil_235, bid128_to_uint32_xceil, 0x303C0000000000000000003200000031u128, 2147483649   , 0x20);  // -- 2^31+0.5-ulp
dec_test!(bid128_to_uint32_xceil_236, bid128_to_uint32_xceil, 0x303C0000000000000000003200000032u128, 2147483649   , 0x20);  // -- 2^31+0.5
dec_test!(bid128_to_uint32_xceil_237, bid128_to_uint32_xceil, 0x303C0000000000000000003200000033u128, 2147483649   , 0x20);  // -- 2^31+0.5+ulp
dec_test!(bid128_to_uint32_xceil_238, bid128_to_uint32_xceil, 0x303C00000000000000000063FFFFFFCDu128, 2147483648   , 0x01);  // -- 2^32-0.5-ulp
dec_test!(bid128_to_uint32_xceil_239, bid128_to_uint32_xceil, 0x303C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01);  // -- 2^32-0.5
dec_test!(bid128_to_uint32_xceil_240, bid128_to_uint32_xceil, 0x303C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01);  // -- 2^32-0.5+ulp
dec_test!(bid128_to_uint32_xceil_241, bid128_to_uint32_xceil, 0x303C0000000000000000006400000031u128, 2147483648   , 0x01);  // -- 2^32+0.5-ulp
dec_test!(bid128_to_uint32_xceil_242, bid128_to_uint32_xceil, 0x303C0000000000000000006400000032u128, 2147483648   , 0x01);  // -- 2^32+0.5
dec_test!(bid128_to_uint32_xceil_243, bid128_to_uint32_xceil, 0x303C0000000000000000006400000033u128, 2147483648   , 0x01);  // -- 2^32+0.5+ulp
dec_test!(bid128_to_uint32_xceil_244, bid128_to_uint32_xceil, 0x303E0000000000000000000000000005u128, 1            , 0x20);  // -- 0.5
dec_test!(bid128_to_uint32_xceil_245, bid128_to_uint32_xceil, 0x303E000000000000000000000000000Fu128, 2            , 0x20);  // -- 1.5
dec_test!(bid128_to_uint32_xceil_246, bid128_to_uint32_xceil, 0x303E0000000000000000000000000BB7u128, 300          , 0x20);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_247, bid128_to_uint32_xceil, 0x303E0000000000000000000000000BB8u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_248, bid128_to_uint32_xceil, 0x303E0000000000000000000000000BB9u128, 301          , 0x20);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_249, bid128_to_uint32_xceil, 0x303E0000000000000000000000000BBDu128, 301          , 0x20);  // -- 300.5
dec_test!(bid128_to_uint32_xceil_250, bid128_to_uint32_xceil, 0x303E00000000000000000004FFFFFFF1u128, 2147483647   , 0x20);  // -- 2^31-1.5
dec_test!(bid128_to_uint32_xceil_251, bid128_to_uint32_xceil, 0x303E00000000000000000004FFFFFFF5u128, 2147483647   , 0x20);  // -- 2^31-1-ulp
dec_test!(bid128_to_uint32_xceil_252, bid128_to_uint32_xceil, 0x303E00000000000000000004FFFFFFF6u128, 2147483647   , 0x00);  // -- 2^31-1
dec_test!(bid128_to_uint32_xceil_253, bid128_to_uint32_xceil, 0x303E00000000000000000004FFFFFFF7u128, 2147483648   , 0x20);  // -- 2^31-1+ulp
dec_test!(bid128_to_uint32_xceil_254, bid128_to_uint32_xceil, 0x303E00000000000000000004FFFFFFFBu128, 2147483648   , 0x20);  // -- 2^31-0.5
dec_test!(bid128_to_uint32_xceil_255, bid128_to_uint32_xceil, 0x303E00000000000000000004FFFFFFFFu128, 2147483648   , 0x20);  // -- 2^31-ulp
dec_test!(bid128_to_uint32_xceil_256, bid128_to_uint32_xceil, 0x303E0000000000000000000500000000u128, 2147483648   , 0x00);  // -- 2^31
dec_test!(bid128_to_uint32_xceil_257, bid128_to_uint32_xceil, 0x303E0000000000000000000500000001u128, 2147483649   , 0x20);  // -- 2^31+ulp
dec_test!(bid128_to_uint32_xceil_258, bid128_to_uint32_xceil, 0x303E0000000000000000000500000005u128, 2147483649   , 0x20);  // -- 2^31+0.5
dec_test!(bid128_to_uint32_xceil_259, bid128_to_uint32_xceil, 0x303E0000000000000000000500000009u128, 2147483649   , 0x20);  // -- 2^31+1-ulp
dec_test!(bid128_to_uint32_xceil_260, bid128_to_uint32_xceil, 0x303E000000000000000000050000000Au128, 2147483649   , 0x00);  // -- 2^31+1
dec_test!(bid128_to_uint32_xceil_261, bid128_to_uint32_xceil, 0x303E000000000000000000050000000Bu128, 2147483650   , 0x20);  // -- 2^31+1+ulp
dec_test!(bid128_to_uint32_xceil_262, bid128_to_uint32_xceil, 0x303E00000000000000000009FFFFFFF5u128, 4294967295   , 0x20);  // -- 2^32-1-ulp
dec_test!(bid128_to_uint32_xceil_263, bid128_to_uint32_xceil, 0x303E00000000000000000009FFFFFFF6u128, 4294967295   , 0x00);  // -- 2^32-1
dec_test!(bid128_to_uint32_xceil_264, bid128_to_uint32_xceil, 0x303E00000000000000000009FFFFFFF7u128, 2147483648   , 0x01);  // -- 2^32-1+ulp
dec_test!(bid128_to_uint32_xceil_265, bid128_to_uint32_xceil, 0x303E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01);  // -- 2^32-0.5
dec_test!(bid128_to_uint32_xceil_266, bid128_to_uint32_xceil, 0x303E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01);  // -- 2^32-ulp
dec_test!(bid128_to_uint32_xceil_267, bid128_to_uint32_xceil, 0x303E0000000000000000000A00000000u128, 2147483648   , 0x01);  // -- 2^32
dec_test!(bid128_to_uint32_xceil_268, bid128_to_uint32_xceil, 0x303E0000000000000000000A00000001u128, 2147483648   , 0x01);  // -- 2^32+ulp
dec_test!(bid128_to_uint32_xceil_269, bid128_to_uint32_xceil, 0x303E0000000000000000000A00000005u128, 2147483648   , 0x01);  // -- 2^32+0.5
dec_test!(bid128_to_uint32_xceil_270, bid128_to_uint32_xceil, 0x303E0000000000000000000A00000009u128, 2147483648   , 0x01);  // -- 2^32+1-ulp
dec_test!(bid128_to_uint32_xceil_271, bid128_to_uint32_xceil, 0x303E0000000000000000000A0000000Au128, 2147483648   , 0x01);  // -- 2^32+1
dec_test!(bid128_to_uint32_xceil_272, bid128_to_uint32_xceil, 0x303E0000000000000000000A0000000Bu128, 2147483648   , 0x01);  // -- 2^32+1+ulp
dec_test!(bid128_to_uint32_xceil_273, bid128_to_uint32_xceil, 0x303E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01);  // -- 2e10-1.5
dec_test!(bid128_to_uint32_xceil_274, bid128_to_uint32_xceil, 0x303E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01);  // -- 2e10-0.5
dec_test!(bid128_to_uint32_xceil_275, bid128_to_uint32_xceil, 0x303E0000000000000000002E90EDD005u128, 2147483648   , 0x01);  // -- 2e10+0.5
dec_test!(bid128_to_uint32_xceil_276, bid128_to_uint32_xceil, 0x303E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01);  // -- 2e10+1.5
dec_test!(bid128_to_uint32_xceil_277, bid128_to_uint32_xceil, 0x30400000000000000000000000000001u128, 1            , 0x00);  // -- 1
dec_test!(bid128_to_uint32_xceil_278, bid128_to_uint32_xceil, 0x3040000000000000000000000000012Bu128, 299          , 0x00);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_279, bid128_to_uint32_xceil, 0x3040000000000000000000000000012Cu128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_280, bid128_to_uint32_xceil, 0x3040000000000000000000000000012Du128, 301          , 0x00);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_281, bid128_to_uint32_xceil, 0x3040000000000000000000007FFFFFFFu128, 2147483647   , 0x00);  // -- 2^31-1
dec_test!(bid128_to_uint32_xceil_282, bid128_to_uint32_xceil, 0x30400000000000000000000080000000u128, 2147483648   , 0x00);  // -- 2^31
dec_test!(bid128_to_uint32_xceil_283, bid128_to_uint32_xceil, 0x30400000000000000000000080000001u128, 2147483649   , 0x00);  // -- 2^31+1
dec_test!(bid128_to_uint32_xceil_284, bid128_to_uint32_xceil, 0x304000000000000000000000FFFFFFFFu128, 4294967295   , 0x00);  // -- 2^32-1
dec_test!(bid128_to_uint32_xceil_285, bid128_to_uint32_xceil, 0x30400000000000000000000100000000u128, 2147483648   , 0x01);  // -- 2^32
dec_test!(bid128_to_uint32_xceil_286, bid128_to_uint32_xceil, 0x30400000000000000000000100000001u128, 2147483648   , 0x01);  // -- 2^32+1
dec_test!(bid128_to_uint32_xceil_287, bid128_to_uint32_xceil, 0x304000000000000000000004A817C7FFu128, 2147483648   , 0x01);  // -- 2e10-1
dec_test!(bid128_to_uint32_xceil_288, bid128_to_uint32_xceil, 0x304000000000000000000004A817C801u128, 2147483648   , 0x01);  // -- 2e10+1
dec_test!(bid128_to_uint32_xceil_289, bid128_to_uint32_xceil, 0x3041ED09BEAD87C0378D8E6400000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_290, bid128_to_uint32_xceil, 0x3042000000000000000000000000001Du128, 290          , 0x00);  // -- 300-ulp
dec_test!(bid128_to_uint32_xceil_291, bid128_to_uint32_xceil, 0x3042000000000000000000000000001Eu128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_292, bid128_to_uint32_xceil, 0x3042000000000000000000000000001Fu128, 310          , 0x00);  // -- 300+ulp
dec_test!(bid128_to_uint32_xceil_293, bid128_to_uint32_xceil, 0x304200000000000000000000773593FFu128, 2147483648   , 0x01);  // -- 2e10-ulp
dec_test!(bid128_to_uint32_xceil_294, bid128_to_uint32_xceil, 0x30420000000000000000000077359400u128, 2147483648   , 0x01);  // -- 2e10
dec_test!(bid128_to_uint32_xceil_295, bid128_to_uint32_xceil, 0x30420000000000000000000077359401u128, 2147483648   , 0x01);  // -- 2e10+ulp
dec_test!(bid128_to_uint32_xceil_296, bid128_to_uint32_xceil, 0x30440000000000000000000000000003u128, 300          , 0x00);  // -- 300
dec_test!(bid128_to_uint32_xceil_297, bid128_to_uint32_xceil, 0x30520000000000000000000000000004u128, 4000000000   , 0x00);  // -- 4e9
dec_test!(bid128_to_uint32_xceil_298, bid128_to_uint32_xceil, 0x30520000000000000000000000000005u128, 2147483648   , 0x01);  // -- 5e9
dec_test!(bid128_to_uint32_xceil_299, bid128_to_uint32_xceil, 0x30540000000000000000000000000002u128, 2147483648   , 0x01);  // -- 2e10
dec_test!(bid128_to_uint32_xceil_300, bid128_to_uint32_xceil, "+366357.9E0"                         , 366358       , 0x20);
dec_test!(bid128_to_uint32_xceil_301, bid128_to_uint32_xceil, 4294967296u64                         , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_302, bid128_to_uint32_xceil, "5.5"                                 , 6            , 0x20);
dec_test!(bid128_to_uint32_xceil_303, bid128_to_uint32_xceil, 0x78000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xceil_304, bid128_to_uint32_xceil, 0x7abc96128723e0752a9c59be63a0b0b1u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_305, bid128_to_uint32_xceil, 0x7c000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xceil_306, bid128_to_uint32_xceil, 0x7c003fffffffffff38c15b08ffffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_307, bid128_to_uint32_xceil, 0x7c003fffffffffff38c15b0affffffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_308, bid128_to_uint32_xceil, 0x7e000000000000000000000000000000u128, 2147483648   , 0x01);
dec_test!(bid128_to_uint32_xceil_309, bid128_to_uint32_xceil, 0x81140000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_310, bid128_to_uint32_xceil, 0x88706b5e5b03086696d36db713235d0fu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xceil_311, bid128_to_uint32_xceil, "+8989.9988898888898898E0"            , 8990         , 0x20);
dec_test!(bid128_to_uint32_xceil_312, bid128_to_uint32_xceil, 0x8eae0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_313, bid128_to_uint32_xceil, 0x8fa7e9e139ece090809eba3f42ed437fu128, 0            , 0x20);
dec_test!(bid128_to_uint32_xceil_314, bid128_to_uint32_xceil, 0x951e0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_315, bid128_to_uint32_xceil, "+9989998.8889E0"                     , 9989999      , 0x20);
dec_test!(bid128_to_uint32_xceil_316, bid128_to_uint32_xceil, 0xac51e543dce9466afd0768f76a4915a8u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xceil_317, bid128_to_uint32_xceil, 0xacea5fa5d3686f7462a1235048c0c5e2u128, 0            , 0x20);
dec_test!(bid128_to_uint32_xceil_318, bid128_to_uint32_xceil, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x20);  // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_xceil_319, bid128_to_uint32_xceil, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0            , 0x20);  // -- -(0.5)
dec_test!(bid128_to_uint32_xceil_320, bid128_to_uint32_xceil, 0xAFFCF684DF56C3E01BC6C73200000001u128, 0            , 0x20);  // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_xceil_321, bid128_to_uint32_xceil, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 0            , 0x20);  // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_xceil_322, bid128_to_uint32_xceil, 0xAFFDEC8B86EF679D76FC433D80000000u128, 0            , 0x20);  // -- -(0.999)
dec_test!(bid128_to_uint32_xceil_323, bid128_to_uint32_xceil, 0xAFFDEC8B86EF679D76FC433D80000001u128, 0            , 0x20);  // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_xceil_324, bid128_to_uint32_xceil, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 0            , 0x20);  // -- -(1-ulp)
dec_test!(bid128_to_uint32_xceil_325, bid128_to_uint32_xceil, 0xAFFE314DC6448D9338C15B0A00000000u128, 2147483648   , 0x01);  // -- -(1)
dec_test!(bid128_to_uint32_xceil_326, bid128_to_uint32_xceil, 0xAFFE314DC6448D9338C15B0A00000001u128, 2147483648   , 0x01);  // -- -(1+ulp)
dec_test!(bid128_to_uint32_xceil_327, bid128_to_uint32_xceil, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, 2147483648   , 0x01);  // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xceil_328, bid128_to_uint32_xceil, 0xAFFE49F4A966D45CD522088F00000000u128, 2147483648   , 0x01);  // -- -(1.5)
dec_test!(bid128_to_uint32_xceil_329, bid128_to_uint32_xceil, 0xAFFE49F4A966D45CD522088F00000001u128, 2147483648   , 0x01);  // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xceil_330, bid128_to_uint32_xceil, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_331, bid128_to_uint32_xceil, 0xB00293E952CDA8B9AA44111E00000000u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_332, bid128_to_uint32_xceil, 0xB00293E952CDA8B9AA44111E00000001u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_333, bid128_to_uint32_xceil, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, 2147483648   , 0x01);  // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xceil_334, bid128_to_uint32_xceil, 0xB00294286EACB8CB0A8CB6B140000000u128, 2147483648   , 0x01);  // -- -(300.5)
dec_test!(bid128_to_uint32_xceil_335, bid128_to_uint32_xceil, 0xB00294286EACB8CB0A8CB6B140000001u128, 2147483648   , 0x01);  // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xceil_336, bid128_to_uint32_xceil, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_337, bid128_to_uint32_xceil, 0xB0040ECA8847C4129106CE8300000000u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_338, bid128_to_uint32_xceil, 0xB0040ECA8847C4129106CE8300000001u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_339, bid128_to_uint32_xceil, 0xb009a1c2e3ca9de67b6473eac5baecadu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_340, bid128_to_uint32_xceil, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_341, bid128_to_uint32_xceil, 0xB00A0003C95A2F0B4856475FE0000000u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_342, bid128_to_uint32_xceil, 0xB00A0003C95A2F0B4856475FE0000001u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_343, bid128_to_uint32_xceil, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_344, bid128_to_uint32_xceil, 0xB00C000060EF6B1ABA6F072330000000u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_345, bid128_to_uint32_xceil, 0xB00C000060EF6B1ABA6F072330000001u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_346, bid128_to_uint32_xceil, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483648   , 0x01);  // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xceil_347, bid128_to_uint32_xceil, 0xB01069E10DE628D3A6C9CC9B8E800000u128, 2147483648   , 0x01);  // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xceil_348, bid128_to_uint32_xceil, 0xB01069E10DE628D3A6C9CC9B8E800001u128, 2147483648   , 0x01);  // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xceil_349, bid128_to_uint32_xceil, 0xB01069E10DE692B4B4B133125EFFFFFFu128, 2147483648   , 0x01);  // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xceil_350, bid128_to_uint32_xceil, 0xB01069E10DE692B4B4B133125F000000u128, 2147483648   , 0x01);  // -- -(2^31-1)
dec_test!(bid128_to_uint32_xceil_351, bid128_to_uint32_xceil, 0xB01069E10DE692B4B4B133125F000001u128, 2147483648   , 0x01);  // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xceil_352, bid128_to_uint32_xceil, 0xB01069E10DE6FC95C29899892F7FFFFFu128, 2147483648   , 0x01);  // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xceil_353, bid128_to_uint32_xceil, 0xB01069E10DE6FC95C29899892F800000u128, 2147483648   , 0x01);  // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xceil_354, bid128_to_uint32_xceil, 0xB01069E10DE6FC95C29899892F800001u128, 2147483648   , 0x01);  // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xceil_355, bid128_to_uint32_xceil, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, 2147483648   , 0x01);  // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_xceil_356, bid128_to_uint32_xceil, 0xB01069E10DE76676D080000000000000u128, 2147483648   , 0x01);  // -- -(2^31)
dec_test!(bid128_to_uint32_xceil_357, bid128_to_uint32_xceil, 0xB01069E10DE76676D080000000000001u128, 2147483648   , 0x01);  // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_xceil_358, bid128_to_uint32_xceil, 0xB01069E10DE7D057DE676676D07FFFFFu128, 2147483648   , 0x01);  // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xceil_359, bid128_to_uint32_xceil, 0xB01069E10DE7D057DE676676D0800000u128, 2147483648   , 0x01);  // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xceil_360, bid128_to_uint32_xceil, 0xB01069E10DE7D057DE676676D0800001u128, 2147483648   , 0x01);  // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xceil_361, bid128_to_uint32_xceil, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, 2147483648   , 0x01);  // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xceil_362, bid128_to_uint32_xceil, 0xB01069E10DE83A38EC4ECCEDA1000000u128, 2147483648   , 0x01);  // -- -(2^31+1)
dec_test!(bid128_to_uint32_xceil_363, bid128_to_uint32_xceil, 0xB01069E10DE83A38EC4ECCEDA1000001u128, 2147483648   , 0x01);  // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xceil_364, bid128_to_uint32_xceil, 0xB010C5371912364CE3056C27FFFFFFFFu128, 2147483648   , 0x01);  // -- -(4e9-ulp)
dec_test!(bid128_to_uint32_xceil_365, bid128_to_uint32_xceil, 0xB010C5371912364CE3056C2800000000u128, 2147483648   , 0x01);  // -- -(4e9)
dec_test!(bid128_to_uint32_xceil_366, bid128_to_uint32_xceil, 0xB010C5371912364CE3056C2800000001u128, 2147483648   , 0x01);  // -- -(4e9+ulp)
dec_test!(bid128_to_uint32_xceil_367, bid128_to_uint32_xceil, 0xB010D3C21BCDF92B853133125EFFFFFFu128, 2147483648   , 0x01);  // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xceil_368, bid128_to_uint32_xceil, 0xB010D3C21BCDF92B853133125F000000u128, 2147483648   , 0x01);  // -- -(2^32-1)
dec_test!(bid128_to_uint32_xceil_369, bid128_to_uint32_xceil, 0xB010D3C21BCDF92B853133125F000001u128, 2147483648   , 0x01);  // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xceil_370, bid128_to_uint32_xceil, 0xB010D3C21BCE630C931899892F7FFFFFu128, 2147483648   , 0x01);  // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xceil_371, bid128_to_uint32_xceil, 0xB010D3C21BCE630C931899892F800000u128, 2147483648   , 0x01);  // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xceil_372, bid128_to_uint32_xceil, 0xB010D3C21BCE630C931899892F800001u128, 2147483648   , 0x01);  // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xceil_373, bid128_to_uint32_xceil, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, 2147483648   , 0x01);  // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_xceil_374, bid128_to_uint32_xceil, 0xB010D3C21BCECCEDA100000000000000u128, 2147483648   , 0x01);  // -- -(2^32)
dec_test!(bid128_to_uint32_xceil_375, bid128_to_uint32_xceil, 0xB010D3C21BCECCEDA100000000000001u128, 2147483648   , 0x01);  // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_xceil_376, bid128_to_uint32_xceil, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, 2147483648   , 0x01);  // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xceil_377, bid128_to_uint32_xceil, 0xB010D3C21BCF36CEAEE76676D0800000u128, 2147483648   , 0x01);  // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xceil_378, bid128_to_uint32_xceil, 0xB010D3C21BCF36CEAEE76676D0800001u128, 2147483648   , 0x01);  // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xceil_379, bid128_to_uint32_xceil, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, 2147483648   , 0x01);  // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xceil_380, bid128_to_uint32_xceil, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, 2147483648   , 0x01);  // -- -(2^32+1)
dec_test!(bid128_to_uint32_xceil_381, bid128_to_uint32_xceil, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, 2147483648   , 0x01);  // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xceil_382, bid128_to_uint32_xceil, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, 2147483648   , 0x01);  // -- -(5e9-ulp)
dec_test!(bid128_to_uint32_xceil_383, bid128_to_uint32_xceil, 0xB010F684DF56C3E01BC6C73200000000u128, 2147483648   , 0x01);  // -- -(5e9)
dec_test!(bid128_to_uint32_xceil_384, bid128_to_uint32_xceil, 0xB010F684DF56C3E01BC6C73200000001u128, 2147483648   , 0x01);  // -- -(5e9+ulp)
dec_test!(bid128_to_uint32_xceil_385, bid128_to_uint32_xceil, 0xb0117b0f91e6d11bb013b25a401cd49cu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_386, bid128_to_uint32_xceil, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, 2147483648   , 0x01);  // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_xceil_387, bid128_to_uint32_xceil, 0xB012629B8C88FB62ED56E4238E400000u128, 2147483648   , 0x01);  // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_xceil_388, bid128_to_uint32_xceil, 0xB012629B8C88FB62ED56E4238E400001u128, 2147483648   , 0x01);  // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_xceil_389, bid128_to_uint32_xceil, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, 2147483648   , 0x01);  // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_xceil_390, bid128_to_uint32_xceil, 0xB012629B8C8905F96EBAD4C909800000u128, 2147483648   , 0x01);  // -- -(2e10-1)
dec_test!(bid128_to_uint32_xceil_391, bid128_to_uint32_xceil, 0xB012629B8C8905F96EBAD4C909800001u128, 2147483648   , 0x01);  // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_xceil_392, bid128_to_uint32_xceil, 0xB012629B8C89108FF01EC56E84BFFFFFu128, 2147483648   , 0x01);  // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_xceil_393, bid128_to_uint32_xceil, 0xB012629B8C89108FF01EC56E84C00000u128, 2147483648   , 0x01);  // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_xceil_394, bid128_to_uint32_xceil, 0xB012629B8C89108FF01EC56E84C00001u128, 2147483648   , 0x01);  // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_xceil_395, bid128_to_uint32_xceil, 0xB012629B8C891B267182B613FFFFFFFFu128, 2147483648   , 0x01);  // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_xceil_396, bid128_to_uint32_xceil, 0xB012629B8C891B267182B61400000000u128, 2147483648   , 0x01);  // -- -(2e10)
dec_test!(bid128_to_uint32_xceil_397, bid128_to_uint32_xceil, 0xB012629B8C891B267182B61400000001u128, 2147483648   , 0x01);  // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_xceil_398, bid128_to_uint32_xceil, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, 2147483648   , 0x01);  // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_xceil_399, bid128_to_uint32_xceil, 0xB012629B8C8925BCF2E6A6B97B400000u128, 2147483648   , 0x01);  // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_xceil_400, bid128_to_uint32_xceil, 0xB012629B8C8925BCF2E6A6B97B400001u128, 2147483648   , 0x01);  // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_xceil_401, bid128_to_uint32_xceil, 0xB012629B8C893053744A975EF67FFFFFu128, 2147483648   , 0x01);  // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_xceil_402, bid128_to_uint32_xceil, 0xB012629B8C893053744A975EF6800000u128, 2147483648   , 0x01);  // -- -(2e10+1)
dec_test!(bid128_to_uint32_xceil_403, bid128_to_uint32_xceil, 0xB012629B8C893053744A975EF6800001u128, 2147483648   , 0x01);  // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_xceil_404, bid128_to_uint32_xceil, 0xB012629B8C893AE9F5AE880471BFFFFFu128, 2147483648   , 0x01);  // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_xceil_405, bid128_to_uint32_xceil, 0xB012629B8C893AE9F5AE880471C00000u128, 2147483648   , 0x01);  // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_xceil_406, bid128_to_uint32_xceil, 0xB012629B8C893AE9F5AE880471C00001u128, 2147483648   , 0x01);  // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_xceil_407, bid128_to_uint32_xceil, 0xB01600000000003627E8F712373BFFFFu128, 0            , 0x20);  // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_xceil_408, bid128_to_uint32_xceil, 0xB01600000000003627E8F712373C0000u128, 0            , 0x20);  // -- -(0.999)
dec_test!(bid128_to_uint32_xceil_409, bid128_to_uint32_xceil, 0xB01600000000003627E8F712373C0001u128, 0            , 0x20);  // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_xceil_410, bid128_to_uint32_xceil, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483648   , 0x01);  // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xceil_411, bid128_to_uint32_xceil, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, 2147483648   , 0x01);  // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xceil_412, bid128_to_uint32_xceil, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, 2147483648   , 0x01);  // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xceil_413, bid128_to_uint32_xceil, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, 2147483648   , 0x01);  // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xceil_414, bid128_to_uint32_xceil, 0xB0180002B5E3AF13FBA450E94E780000u128, 2147483648   , 0x01);  // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xceil_415, bid128_to_uint32_xceil, 0xB0180002B5E3AF13FBA450E94E780001u128, 2147483648   , 0x01);  // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xceil_416, bid128_to_uint32_xceil, 0xB0180002B5E3AF19676BAF16B187FFFFu128, 2147483648   , 0x01);  // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xceil_417, bid128_to_uint32_xceil, 0xB0180002B5E3AF19676BAF16B1880000u128, 2147483648   , 0x01);  // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xceil_418, bid128_to_uint32_xceil, 0xB0180002B5E3AF19676BAF16B1880001u128, 2147483648   , 0x01);  // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xceil_419, bid128_to_uint32_xceil, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, 2147483648   , 0x01);  // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xceil_420, bid128_to_uint32_xceil, 0xB01800056BC75E2AAD2C50E94E780000u128, 2147483648   , 0x01);  // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xceil_421, bid128_to_uint32_xceil, 0xB01800056BC75E2AAD2C50E94E780001u128, 2147483648   , 0x01);  // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xceil_422, bid128_to_uint32_xceil, 0xB01800056BC75E3018F3AF16B187FFFFu128, 2147483648   , 0x01);  // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xceil_423, bid128_to_uint32_xceil, 0xB01800056BC75E3018F3AF16B1880000u128, 2147483648   , 0x01);  // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xceil_424, bid128_to_uint32_xceil, 0xB01800056BC75E3018F3AF16B1880001u128, 2147483648   , 0x01);  // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xceil_425, bid128_to_uint32_xceil, 0xB01A0000000000004563918244F3FFFFu128, 0            , 0x20);  // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_xceil_426, bid128_to_uint32_xceil, 0xB01A0000000000004563918244F40000u128, 0            , 0x20);  // -- -(0.5)
dec_test!(bid128_to_uint32_xceil_427, bid128_to_uint32_xceil, 0xB01A0000000000004563918244F40001u128, 0            , 0x20);  // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_xceil_428, bid128_to_uint32_xceil, 0xB01A0000000000008AC7230489E7FFFFu128, 0            , 0x20);  // -- -(1-ulp)
dec_test!(bid128_to_uint32_xceil_429, bid128_to_uint32_xceil, 0xB01A0000000000008AC7230489E80000u128, 2147483648   , 0x01);  // -- -(1)
dec_test!(bid128_to_uint32_xceil_430, bid128_to_uint32_xceil, 0xB01A0000000000008AC7230489E80001u128, 2147483648   , 0x01);  // -- -(1+ulp)
dec_test!(bid128_to_uint32_xceil_431, bid128_to_uint32_xceil, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, 2147483648   , 0x01);  // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xceil_432, bid128_to_uint32_xceil, 0xB01A0000000000A2E6C09AD3E0D40000u128, 2147483648   , 0x01);  // -- -(300.5)
dec_test!(bid128_to_uint32_xceil_433, bid128_to_uint32_xceil, 0xB01A0000000000A2E6C09AD3E0D40001u128, 2147483648   , 0x01);  // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xceil_434, bid128_to_uint32_xceil, 0xB01A000045639181BA2CDCFB7617FFFFu128, 2147483648   , 0x01);  // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xceil_435, bid128_to_uint32_xceil, 0xB01A000045639181BA2CDCFB76180000u128, 2147483648   , 0x01);  // -- -(2^31-1)
dec_test!(bid128_to_uint32_xceil_436, bid128_to_uint32_xceil, 0xB01A000045639181BA2CDCFB76180001u128, 2147483648   , 0x01);  // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xceil_437, bid128_to_uint32_xceil, 0xB01A00004563918244F3FFFFFFFFFFFFu128, 2147483648   , 0x01);  // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_xceil_438, bid128_to_uint32_xceil, 0xB01A00004563918244F4000000000000u128, 2147483648   , 0x01);  // -- -(2^31)
dec_test!(bid128_to_uint32_xceil_439, bid128_to_uint32_xceil, 0xB01A00004563918244F4000000000001u128, 2147483648   , 0x01);  // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_xceil_440, bid128_to_uint32_xceil, 0xB01A000045639182CFBB230489E7FFFFu128, 2147483648   , 0x01);  // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xceil_441, bid128_to_uint32_xceil, 0xB01A000045639182CFBB230489E80000u128, 2147483648   , 0x01);  // -- -(2^31+1)
dec_test!(bid128_to_uint32_xceil_442, bid128_to_uint32_xceil, 0xB01A000045639182CFBB230489E80001u128, 2147483648   , 0x01);  // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xceil_443, bid128_to_uint32_xceil, 0xB01A00008AC72303FF20DCFB7617FFFFu128, 2147483648   , 0x01);  // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xceil_444, bid128_to_uint32_xceil, 0xB01A00008AC72303FF20DCFB76180000u128, 2147483648   , 0x01);  // -- -(2^32-1)
dec_test!(bid128_to_uint32_xceil_445, bid128_to_uint32_xceil, 0xB01A00008AC72303FF20DCFB76180001u128, 2147483648   , 0x01);  // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xceil_446, bid128_to_uint32_xceil, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, 2147483648   , 0x01);  // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_xceil_447, bid128_to_uint32_xceil, 0xB01A00008AC7230489E8000000000000u128, 2147483648   , 0x01);  // -- -(2^32)
dec_test!(bid128_to_uint32_xceil_448, bid128_to_uint32_xceil, 0xB01A00008AC7230489E8000000000001u128, 2147483648   , 0x01);  // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_xceil_449, bid128_to_uint32_xceil, 0xB01A00008AC7230514AF230489E7FFFFu128, 2147483648   , 0x01);  // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xceil_450, bid128_to_uint32_xceil, 0xB01A00008AC7230514AF230489E80000u128, 2147483648   , 0x01);  // -- -(2^32+1)
dec_test!(bid128_to_uint32_xceil_451, bid128_to_uint32_xceil, 0xB01A00008AC7230514AF230489E80001u128, 2147483648   , 0x01);  // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xceil_452, bid128_to_uint32_xceil, 0xB01C00000000000014D1120D7B15FFFFu128, 2147483648   , 0x01);  // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xceil_453, bid128_to_uint32_xceil, 0xB01C00000000000014D1120D7B160000u128, 2147483648   , 0x01);  // -- -(1.5)
dec_test!(bid128_to_uint32_xceil_454, bid128_to_uint32_xceil, 0xB01C00000000000014D1120D7B160001u128, 2147483648   , 0x01);  // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xceil_455, bid128_to_uint32_xceil, 0xB01E000000000001A055690D9DB7FFFFu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_456, bid128_to_uint32_xceil, 0xB01E000000000001A055690D9DB80000u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_457, bid128_to_uint32_xceil, 0xB01E000000000001A055690D9DB80001u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_458, bid128_to_uint32_xceil, 0xB02000000000000029A2241AF62BFFFFu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_459, bid128_to_uint32_xceil, 0xB02000000000000029A2241AF62C0000u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_460, bid128_to_uint32_xceil, 0xB02000000000000029A2241AF62C0001u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_461, bid128_to_uint32_xceil, 0xb020000000401004cf4ef3fea7eff1ebu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_462, bid128_to_uint32_xceil, 0xB024000000000000006A94D74F42FFFFu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_463, bid128_to_uint32_xceil, 0xB024000000000000006A94D74F430000u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_464, bid128_to_uint32_xceil, 0xB024000000000000006A94D74F430001u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_465, bid128_to_uint32_xceil, 0xB02A00000000000000000017428106FFu128, 0            , 0x20);  // -- -(0.999-ulp)
dec_test!(bid128_to_uint32_xceil_466, bid128_to_uint32_xceil, 0xB02A0000000000000000001742810700u128, 0            , 0x20);  // -- -(0.999)
dec_test!(bid128_to_uint32_xceil_467, bid128_to_uint32_xceil, 0xB02A0000000000000000001742810701u128, 0            , 0x20);  // -- -(0.999+ulp)
dec_test!(bid128_to_uint32_xceil_468, bid128_to_uint32_xceil, 0xB02A00000000006C6B935B68D08DA3FFu128, 2147483648   , 0x01);  // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint32_xceil_469, bid128_to_uint32_xceil, 0xB02A00000000006C6B935B68D08DA400u128, 2147483648   , 0x01);  // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_xceil_470, bid128_to_uint32_xceil, 0xB02A00000000006C6B935B68D08DA401u128, 2147483648   , 0x01);  // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint32_xceil_471, bid128_to_uint32_xceil, 0xB02A00000000006C6B935B8019048BFFu128, 2147483648   , 0x01);  // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint32_xceil_472, bid128_to_uint32_xceil, 0xB02A00000000006C6B935B8019048C00u128, 2147483648   , 0x01);  // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_xceil_473, bid128_to_uint32_xceil, 0xB02A00000000006C6B935B8019048C01u128, 2147483648   , 0x01);  // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint32_xceil_474, bid128_to_uint32_xceil, 0xB02C000000000000000002BBA7F521FFu128, 2147483648   , 0x01);  // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xceil_475, bid128_to_uint32_xceil, 0xB02C000000000000000002BBA7F52200u128, 2147483648   , 0x01);  // -- -(300.5)
dec_test!(bid128_to_uint32_xceil_476, bid128_to_uint32_xceil, 0xB02C000000000000000002BBA7F52201u128, 2147483648   , 0x01);  // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xceil_477, bid128_to_uint32_xceil, 0xB02C00000000000AD78EBC5872141BFFu128, 2147483648   , 0x01);  // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint32_xceil_478, bid128_to_uint32_xceil, 0xB02C00000000000AD78EBC5872141C00u128, 2147483648   , 0x01);  // -- -(2e10-1)
dec_test!(bid128_to_uint32_xceil_479, bid128_to_uint32_xceil, 0xB02C00000000000AD78EBC5872141C01u128, 2147483648   , 0x01);  // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint32_xceil_480, bid128_to_uint32_xceil, 0xB02C00000000000AD78EBC5BF025F1FFu128, 2147483648   , 0x01);  // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint32_xceil_481, bid128_to_uint32_xceil, 0xB02C00000000000AD78EBC5BF025F200u128, 2147483648   , 0x01);  // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_xceil_482, bid128_to_uint32_xceil, 0xB02C00000000000AD78EBC5BF025F201u128, 2147483648   , 0x01);  // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint32_xceil_483, bid128_to_uint32_xceil, 0xB02C00000000000AD78EBC5E4431D5FFu128, 2147483648   , 0x01);  // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint32_xceil_484, bid128_to_uint32_xceil, 0xB02C00000000000AD78EBC5E4431D600u128, 2147483648   , 0x01);  // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_xceil_485, bid128_to_uint32_xceil, 0xB02C00000000000AD78EBC5E4431D601u128, 2147483648   , 0x01);  // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint32_xceil_486, bid128_to_uint32_xceil, 0xB02E000000000000000000001DCD64FFu128, 0            , 0x20);  // -- -(0.5-ulp)
dec_test!(bid128_to_uint32_xceil_487, bid128_to_uint32_xceil, 0xB02E000000000000000000001DCD6500u128, 0            , 0x20);  // -- -(0.5)
dec_test!(bid128_to_uint32_xceil_488, bid128_to_uint32_xceil, 0xB02E000000000000000000001DCD6501u128, 0            , 0x20);  // -- -(0.5+ulp)
dec_test!(bid128_to_uint32_xceil_489, bid128_to_uint32_xceil, 0xB02E000000000000000000003B9AC9FFu128, 0            , 0x20);  // -- -(1-ulp)
dec_test!(bid128_to_uint32_xceil_490, bid128_to_uint32_xceil, 0xB02E000000000000000000003B9ACA00u128, 2147483648   , 0x01);  // -- -(1)
dec_test!(bid128_to_uint32_xceil_491, bid128_to_uint32_xceil, 0xB02E000000000000000000003B9ACA01u128, 2147483648   , 0x01);  // -- -(1+ulp)
dec_test!(bid128_to_uint32_xceil_492, bid128_to_uint32_xceil, 0xB02E0000000000000000000059682EFFu128, 2147483648   , 0x01);  // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xceil_493, bid128_to_uint32_xceil, 0xB02E0000000000000000000059682F00u128, 2147483648   , 0x01);  // -- -(1.5)
dec_test!(bid128_to_uint32_xceil_494, bid128_to_uint32_xceil, 0xB02E0000000000000000000059682F01u128, 2147483648   , 0x01);  // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xceil_495, bid128_to_uint32_xceil, 0xB02E000000000001158E46094F6AC9FFu128, 2147483648   , 0x01);  // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint32_xceil_496, bid128_to_uint32_xceil, 0xB02E000000000001158E46094F6ACA00u128, 2147483648   , 0x01);  // -- -(2e10+1)
dec_test!(bid128_to_uint32_xceil_497, bid128_to_uint32_xceil, 0xB02E000000000001158E46094F6ACA01u128, 2147483648   , 0x01);  // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint32_xceil_498, bid128_to_uint32_xceil, 0xB03000000000000000000006FC23ABFFu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_499, bid128_to_uint32_xceil, 0xB03000000000000000000006FC23AC00u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_500, bid128_to_uint32_xceil, 0xB03000000000000000000006FC23AC01u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_501, bid128_to_uint32_xceil, 0xB03200000000000000000000B2D05DFFu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_502, bid128_to_uint32_xceil, 0xB03200000000000000000000B2D05E00u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_503, bid128_to_uint32_xceil, 0xB03200000000000000000000B2D05E01u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_504, bid128_to_uint32_xceil, 0xB03800000000000000000000002DDA47u128, 2147483648   , 0x01);  // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xceil_505, bid128_to_uint32_xceil, 0xB03800000000000000000000002DDA48u128, 2147483648   , 0x01);  // -- -(300.5)
dec_test!(bid128_to_uint32_xceil_506, bid128_to_uint32_xceil, 0xB03800000000000000000000002DDA49u128, 2147483648   , 0x01);  // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xceil_507, bid128_to_uint32_xceil, 0xB03A00000000000000000000000003E7u128, 0            , 0x20);  // -- -(0.999)
dec_test!(bid128_to_uint32_xceil_508, bid128_to_uint32_xceil, 0xB03A00000000000000000000000005DBu128, 2147483648   , 0x01);  // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xceil_509, bid128_to_uint32_xceil, 0xB03A00000000000000000000000005DCu128, 2147483648   , 0x01);  // -- -(1.5)
dec_test!(bid128_to_uint32_xceil_510, bid128_to_uint32_xceil, 0xB03A00000000000000000000000005DDu128, 2147483648   , 0x01);  // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xceil_511, bid128_to_uint32_xceil, 0xB03A00000000000000000000000495D3u128, 2147483648   , 0x01);  // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xceil_512, bid128_to_uint32_xceil, 0xB03A00000000000000000000000495D4u128, 2147483648   , 0x01);  // -- -(300.5)
dec_test!(bid128_to_uint32_xceil_513, bid128_to_uint32_xceil, 0xB03A00000000000000000000000495D5u128, 2147483648   , 0x01);  // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xceil_514, bid128_to_uint32_xceil, 0xB03C0000000000000000000000000095u128, 2147483648   , 0x01);  // -- -(1.5-ulp)
dec_test!(bid128_to_uint32_xceil_515, bid128_to_uint32_xceil, 0xB03C0000000000000000000000000096u128, 2147483648   , 0x01);  // -- -(1.5)
dec_test!(bid128_to_uint32_xceil_516, bid128_to_uint32_xceil, 0xB03C0000000000000000000000000097u128, 2147483648   , 0x01);  // -- -(1.5+ulp)
dec_test!(bid128_to_uint32_xceil_517, bid128_to_uint32_xceil, 0xB03C0000000000000000000000007561u128, 2147483648   , 0x01);  // -- -(300.5-ulp)
dec_test!(bid128_to_uint32_xceil_518, bid128_to_uint32_xceil, 0xB03C0000000000000000000000007562u128, 2147483648   , 0x01);  // -- -(300.5)
dec_test!(bid128_to_uint32_xceil_519, bid128_to_uint32_xceil, 0xB03C0000000000000000000000007563u128, 2147483648   , 0x01);  // -- -(300.5+ulp)
dec_test!(bid128_to_uint32_xceil_520, bid128_to_uint32_xceil, 0xB03C00000000000000000031FFFFFF69u128, 2147483648   , 0x01);  // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_uint32_xceil_521, bid128_to_uint32_xceil, 0xB03C00000000000000000031FFFFFF6Au128, 2147483648   , 0x01);  // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xceil_522, bid128_to_uint32_xceil, 0xB03C00000000000000000031FFFFFF6Bu128, 2147483648   , 0x01);  // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_uint32_xceil_523, bid128_to_uint32_xceil, 0xB03C00000000000000000031FFFFFFCDu128, 2147483648   , 0x01);  // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_uint32_xceil_524, bid128_to_uint32_xceil, 0xB03C00000000000000000031FFFFFFCEu128, 2147483648   , 0x01);  // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xceil_525, bid128_to_uint32_xceil, 0xB03C00000000000000000031FFFFFFCFu128, 2147483648   , 0x01);  // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_uint32_xceil_526, bid128_to_uint32_xceil, 0xB03C0000000000000000003200000031u128, 2147483648   , 0x01);  // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_uint32_xceil_527, bid128_to_uint32_xceil, 0xB03C0000000000000000003200000032u128, 2147483648   , 0x01);  // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xceil_528, bid128_to_uint32_xceil, 0xB03C0000000000000000003200000033u128, 2147483648   , 0x01);  // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_uint32_xceil_529, bid128_to_uint32_xceil, 0xB03C00000000000000000063FFFFFFCDu128, 2147483648   , 0x01);  // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_uint32_xceil_530, bid128_to_uint32_xceil, 0xB03C00000000000000000063FFFFFFCEu128, 2147483648   , 0x01);  // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xceil_531, bid128_to_uint32_xceil, 0xB03C00000000000000000063FFFFFFCFu128, 2147483648   , 0x01);  // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_uint32_xceil_532, bid128_to_uint32_xceil, 0xB03C0000000000000000006400000031u128, 2147483648   , 0x01);  // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_uint32_xceil_533, bid128_to_uint32_xceil, 0xB03C0000000000000000006400000032u128, 2147483648   , 0x01);  // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xceil_534, bid128_to_uint32_xceil, 0xB03C0000000000000000006400000033u128, 2147483648   , 0x01);  // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_uint32_xceil_535, bid128_to_uint32_xceil, 0xB03E0000000000000000000000000005u128, 0            , 0x20);  // -- -(0.5)
dec_test!(bid128_to_uint32_xceil_536, bid128_to_uint32_xceil, 0xB03E000000000000000000000000000Fu128, 2147483648   , 0x01);  // -- -(1.5)
dec_test!(bid128_to_uint32_xceil_537, bid128_to_uint32_xceil, 0xB03E0000000000000000000000000BB7u128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_538, bid128_to_uint32_xceil, 0xB03E0000000000000000000000000BB8u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_539, bid128_to_uint32_xceil, 0xB03E0000000000000000000000000BB9u128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_540, bid128_to_uint32_xceil, 0xB03E0000000000000000000000000BBDu128, 2147483648   , 0x01);  // -- -(300.5)
dec_test!(bid128_to_uint32_xceil_541, bid128_to_uint32_xceil, 0xB03E00000000000000000004FFFFFFF1u128, 2147483648   , 0x01);  // -- -(2^31-1.5)
dec_test!(bid128_to_uint32_xceil_542, bid128_to_uint32_xceil, 0xB03E00000000000000000004FFFFFFF5u128, 2147483648   , 0x01);  // -- -(2^31-1-ulp)
dec_test!(bid128_to_uint32_xceil_543, bid128_to_uint32_xceil, 0xB03E00000000000000000004FFFFFFF6u128, 2147483648   , 0x01);  // -- -(2^31-1)
dec_test!(bid128_to_uint32_xceil_544, bid128_to_uint32_xceil, 0xB03E00000000000000000004FFFFFFF7u128, 2147483648   , 0x01);  // -- -(2^31-1+ulp)
dec_test!(bid128_to_uint32_xceil_545, bid128_to_uint32_xceil, 0xB03E00000000000000000004FFFFFFFBu128, 2147483648   , 0x01);  // -- -(2^31-0.5)
dec_test!(bid128_to_uint32_xceil_546, bid128_to_uint32_xceil, 0xB03E00000000000000000004FFFFFFFFu128, 2147483648   , 0x01);  // -- -(2^31-ulp)
dec_test!(bid128_to_uint32_xceil_547, bid128_to_uint32_xceil, 0xB03E0000000000000000000500000000u128, 2147483648   , 0x01);  // -- -(2^31)
dec_test!(bid128_to_uint32_xceil_548, bid128_to_uint32_xceil, 0xB03E0000000000000000000500000001u128, 2147483648   , 0x01);  // -- -(2^31+ulp)
dec_test!(bid128_to_uint32_xceil_549, bid128_to_uint32_xceil, 0xB03E0000000000000000000500000005u128, 2147483648   , 0x01);  // -- -(2^31+0.5)
dec_test!(bid128_to_uint32_xceil_550, bid128_to_uint32_xceil, 0xB03E0000000000000000000500000009u128, 2147483648   , 0x01);  // -- -(2^31+1-ulp)
dec_test!(bid128_to_uint32_xceil_551, bid128_to_uint32_xceil, 0xB03E000000000000000000050000000Au128, 2147483648   , 0x01);  // -- -(2^31+1)
dec_test!(bid128_to_uint32_xceil_552, bid128_to_uint32_xceil, 0xB03E000000000000000000050000000Bu128, 2147483648   , 0x01);  // -- -(2^31+1+ulp)
dec_test!(bid128_to_uint32_xceil_553, bid128_to_uint32_xceil, 0xB03E00000000000000000009FFFFFFF5u128, 2147483648   , 0x01);  // -- -(2^32-1-ulp)
dec_test!(bid128_to_uint32_xceil_554, bid128_to_uint32_xceil, 0xB03E00000000000000000009FFFFFFF6u128, 2147483648   , 0x01);  // -- -(2^32-1)
dec_test!(bid128_to_uint32_xceil_555, bid128_to_uint32_xceil, 0xB03E00000000000000000009FFFFFFF7u128, 2147483648   , 0x01);  // -- -(2^32-1+ulp)
dec_test!(bid128_to_uint32_xceil_556, bid128_to_uint32_xceil, 0xB03E00000000000000000009FFFFFFFBu128, 2147483648   , 0x01);  // -- -(2^32-0.5)
dec_test!(bid128_to_uint32_xceil_557, bid128_to_uint32_xceil, 0xB03E00000000000000000009FFFFFFFFu128, 2147483648   , 0x01);  // -- -(2^32-ulp)
dec_test!(bid128_to_uint32_xceil_558, bid128_to_uint32_xceil, 0xB03E0000000000000000000A00000000u128, 2147483648   , 0x01);  // -- -(2^32)
dec_test!(bid128_to_uint32_xceil_559, bid128_to_uint32_xceil, 0xB03E0000000000000000000A00000001u128, 2147483648   , 0x01);  // -- -(2^32+ulp)
dec_test!(bid128_to_uint32_xceil_560, bid128_to_uint32_xceil, 0xB03E0000000000000000000A00000005u128, 2147483648   , 0x01);  // -- -(2^32+0.5)
dec_test!(bid128_to_uint32_xceil_561, bid128_to_uint32_xceil, 0xB03E0000000000000000000A00000009u128, 2147483648   , 0x01);  // -- -(2^32+1-ulp)
dec_test!(bid128_to_uint32_xceil_562, bid128_to_uint32_xceil, 0xB03E0000000000000000000A0000000Au128, 2147483648   , 0x01);  // -- -(2^32+1)
dec_test!(bid128_to_uint32_xceil_563, bid128_to_uint32_xceil, 0xB03E0000000000000000000A0000000Bu128, 2147483648   , 0x01);  // -- -(2^32+1+ulp)
dec_test!(bid128_to_uint32_xceil_564, bid128_to_uint32_xceil, 0xB03E0000000000000000002E90EDCFF1u128, 2147483648   , 0x01);  // -- -(2e10-1.5)
dec_test!(bid128_to_uint32_xceil_565, bid128_to_uint32_xceil, 0xB03E0000000000000000002E90EDCFFBu128, 2147483648   , 0x01);  // -- -(2e10-0.5)
dec_test!(bid128_to_uint32_xceil_566, bid128_to_uint32_xceil, 0xB03E0000000000000000002E90EDD005u128, 2147483648   , 0x01);  // -- -(2e10+0.5)
dec_test!(bid128_to_uint32_xceil_567, bid128_to_uint32_xceil, 0xB03E0000000000000000002E90EDD00Fu128, 2147483648   , 0x01);  // -- -(2e10+1.5)
dec_test!(bid128_to_uint32_xceil_568, bid128_to_uint32_xceil, 0xB0400000000000000000000000000001u128, 2147483648   , 0x01);  // -- -(1)
dec_test!(bid128_to_uint32_xceil_569, bid128_to_uint32_xceil, 0xB040000000000000000000000000012Bu128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_570, bid128_to_uint32_xceil, 0xB040000000000000000000000000012Cu128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_571, bid128_to_uint32_xceil, 0xB040000000000000000000000000012Du128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_572, bid128_to_uint32_xceil, 0xB040000000000000000000007FFFFFFFu128, 2147483648   , 0x01);  // -- -(2^31-1)
dec_test!(bid128_to_uint32_xceil_573, bid128_to_uint32_xceil, 0xB0400000000000000000000080000000u128, 2147483648   , 0x01);  // -- -(2^31)
dec_test!(bid128_to_uint32_xceil_574, bid128_to_uint32_xceil, 0xB0400000000000000000000080000001u128, 2147483648   , 0x01);  // -- -(2^31+1)
dec_test!(bid128_to_uint32_xceil_575, bid128_to_uint32_xceil, 0xB04000000000000000000000FFFFFFFFu128, 2147483648   , 0x01);  // -- -(2^32-1)
dec_test!(bid128_to_uint32_xceil_576, bid128_to_uint32_xceil, 0xB0400000000000000000000100000000u128, 2147483648   , 0x01);  // -- -(2^32)
dec_test!(bid128_to_uint32_xceil_577, bid128_to_uint32_xceil, 0xB0400000000000000000000100000001u128, 2147483648   , 0x01);  // -- -(2^32+1)
dec_test!(bid128_to_uint32_xceil_578, bid128_to_uint32_xceil, 0xB04000000000000000000004A817C7FFu128, 2147483648   , 0x01);  // -- -(2e10-1)
dec_test!(bid128_to_uint32_xceil_579, bid128_to_uint32_xceil, 0xB04000000000000000000004A817C801u128, 2147483648   , 0x01);  // -- -(2e10+1)
dec_test!(bid128_to_uint32_xceil_580, bid128_to_uint32_xceil, 0xB042000000000000000000000000001Du128, 2147483648   , 0x01);  // -- -(300-ulp)
dec_test!(bid128_to_uint32_xceil_581, bid128_to_uint32_xceil, 0xB042000000000000000000000000001Eu128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_582, bid128_to_uint32_xceil, 0xB042000000000000000000000000001Fu128, 2147483648   , 0x01);  // -- -(300+ulp)
dec_test!(bid128_to_uint32_xceil_583, bid128_to_uint32_xceil, 0xB04200000000000000000000773593FFu128, 2147483648   , 0x01);  // -- -(2e10-ulp)
dec_test!(bid128_to_uint32_xceil_584, bid128_to_uint32_xceil, 0xB0420000000000000000000077359400u128, 2147483648   , 0x01);  // -- -(2e10)
dec_test!(bid128_to_uint32_xceil_585, bid128_to_uint32_xceil, 0xB0420000000000000000000077359401u128, 2147483648   , 0x01);  // -- -(2e10+ulp)
dec_test!(bid128_to_uint32_xceil_586, bid128_to_uint32_xceil, 0xB0440000000000000000000000000003u128, 2147483648   , 0x01);  // -- -(300)
dec_test!(bid128_to_uint32_xceil_587, bid128_to_uint32_xceil, 0xB0520000000000000000000000000004u128, 2147483648   , 0x01);  // -- -(4e9)
dec_test!(bid128_to_uint32_xceil_588, bid128_to_uint32_xceil, 0xB0520000000000000000000000000005u128, 2147483648   , 0x01);  // -- -(5e9)
dec_test!(bid128_to_uint32_xceil_589, bid128_to_uint32_xceil, 0xB0540000000000000000000000000002u128, 2147483648   , 0x01);  // -- -(2e10)
dec_test!(bid128_to_uint32_xceil_590, bid128_to_uint32_xceil, 0xbd1c0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_591, bid128_to_uint32_xceil, 0xbec79e4270af2494fb32ee4dc039e51bu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_592, bid128_to_uint32_xceil, 0xbecd63e40053c3046b69c96d4805d810u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_593, bid128_to_uint32_xceil, 0xc078f7eb8d183803fe7ffdbfffefffffu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_594, bid128_to_uint32_xceil, 0xca92039e6933c6798cac1c431e599b3du128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_595, bid128_to_uint32_xceil, 0xe0844400516000112000c006b3802042u128, 0            , 0x00);
dec_test!(bid128_to_uint32_xceil_596, bid128_to_uint32_xceil, 0xfada7fffed29bfef2412528009e0b6dcu128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_597, bid128_to_uint32_xceil, 0xfdfffff7dff7ffff995987966a5caba5u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_598, bid128_to_uint32_xceil, 0xffffffffffffcfbf5120dc2d800d0002u128, 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_599, bid128_to_uint32_xceil, "-Infinity"                           , 0x80000000u32, 0x01);
dec_test!(bid128_to_uint32_xceil_600, bid128_to_uint32_xceil, "Infinity"                            , 0x80000000u32, 0x01);
