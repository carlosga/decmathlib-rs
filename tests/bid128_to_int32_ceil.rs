/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int32_ceil_001, bid128_to_int32_ceil, "-0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_ceil_002, bid128_to_int32_ceil,  "0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_ceil_003, bid128_to_int32_ceil, 0x00000000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_ceil_004, bid128_to_int32_ceil, 0x0000000000000000000114a409a08080u128, 1          , 0x00);
dec_test!(bid128_to_int32_ceil_005, bid128_to_int32_ceil, 0x000000000000000084bf280afe3db8bfu128, 1          , 0x00);
dec_test!(bid128_to_int32_ceil_006, bid128_to_int32_ceil, 0x0001ed09bead87c0378d8e62ffffffffu128, 1          , 0x00);
dec_test!(bid128_to_int32_ceil_007, bid128_to_int32_ceil, 0x0001ed09bead87c0378d8e64ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_ceil_008, bid128_to_int32_ceil, 0x00a038000821d448bf7e1b8d6fe9ea97u128, 1          , 0x00);
dec_test!(bid128_to_int32_ceil_009, bid128_to_int32_ceil, 0x0b3338a4e395a599abb6d2370e605859u128, 1          , 0x00);
dec_test!(bid128_to_int32_ceil_010, bid128_to_int32_ceil, "1.0"                                 , 1          , 0x00);
dec_test!(bid128_to_int32_ceil_011, bid128_to_int32_ceil, "1073741824"                          , 1073741824 , 0x00);
dec_test!(bid128_to_int32_ceil_012, bid128_to_int32_ceil, "1"                                   , 1          , 0x00);
dec_test!(bid128_to_int32_ceil_013, bid128_to_int32_ceil, 0x1439a12c3261105ef1b0fb7822655799u128, 1          , 0x00);
dec_test!(bid128_to_int32_ceil_014, bid128_to_int32_ceil, 0x183fd8eb018ee4556548a59b00094a36u128, 1          , 0x00);
dec_test!(bid128_to_int32_ceil_015, bid128_to_int32_ceil, "2147483648"                          , -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_016, bid128_to_int32_ceil, 0x2d35b8e5f582b2127af6c8d1b9125f70u128, 1          , 0x00);
dec_test!(bid128_to_int32_ceil_017, bid128_to_int32_ceil, 0x2fc40000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_ceil_018, bid128_to_int32_ceil, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 1          , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_int32_ceil_019, bid128_to_int32_ceil, 0x2FFCF684DF56C3E01BC6C73200000000u128, 1          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_ceil_020, bid128_to_int32_ceil, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1          , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_int32_ceil_021, bid128_to_int32_ceil, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1          , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_int32_ceil_022, bid128_to_int32_ceil, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_ceil_023, bid128_to_int32_ceil, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1          , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_int32_ceil_024, bid128_to_int32_ceil, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1          , 0x00); // -- 1-ulp
dec_test!(bid128_to_int32_ceil_025, bid128_to_int32_ceil, 0x2FFE314DC6448D9338C15B0A00000000u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_ceil_026, bid128_to_int32_ceil, 0x2FFE314DC6448D9338C15B0A00000001u128, 2          , 0x00); // -- 1+ulp
dec_test!(bid128_to_int32_ceil_027, bid128_to_int32_ceil, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 2          , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_int32_ceil_028, bid128_to_int32_ceil, 0x2FFE49F4A966D45CD522088F00000000u128, 2          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_ceil_029, bid128_to_int32_ceil, 0x2FFE49F4A966D45CD522088F00000001u128, 2          , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_int32_ceil_030, bid128_to_int32_ceil, 0x3000401000032408fef8f6fc17e9efd2u128, 13         , 0x00);
dec_test!(bid128_to_int32_ceil_031, bid128_to_int32_ceil, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_032, bid128_to_int32_ceil, 0x300293E952CDA8B9AA44111E00000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_033, bid128_to_int32_ceil, 0x300293E952CDA8B9AA44111E00000001u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_034, bid128_to_int32_ceil, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 301        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_ceil_035, bid128_to_int32_ceil, 0x300294286EACB8CB0A8CB6B140000000u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_ceil_036, bid128_to_int32_ceil, 0x300294286EACB8CB0A8CB6B140000001u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_ceil_037, bid128_to_int32_ceil, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_038, bid128_to_int32_ceil, 0x30040ECA8847C4129106CE8300000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_039, bid128_to_int32_ceil, 0x30040ECA8847C4129106CE8300000001u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_040, bid128_to_int32_ceil, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_041, bid128_to_int32_ceil, 0x300A0003C95A2F0B4856475FE0000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_042, bid128_to_int32_ceil, 0x300A0003C95A2F0B4856475FE0000001u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_043, bid128_to_int32_ceil, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_044, bid128_to_int32_ceil, 0x300C000060EF6B1ABA6F072330000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_045, bid128_to_int32_ceil, 0x300C000060EF6B1ABA6F072330000001u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_046, bid128_to_int32_ceil, 0x300c3803410045091801c14004921108u128, 11360728   , 0x00); //
dec_test!(bid128_to_int32_ceil_047, bid128_to_int32_ceil, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483647 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_ceil_048, bid128_to_int32_ceil, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483647 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_ceil_049, bid128_to_int32_ceil, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483647 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_ceil_050, bid128_to_int32_ceil, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_ceil_051, bid128_to_int32_ceil, 0x301069E10DE692B4B4B133125F000000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_ceil_052, bid128_to_int32_ceil, 0x301069E10DE692B4B4B133125F000001u128, -2147483648, 0x01); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_ceil_053, bid128_to_int32_ceil, 0x301069E10DE6FC95C29899892F7FFFFFu128, -2147483648, 0x01); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_ceil_054, bid128_to_int32_ceil, 0x301069E10DE6FC95C29899892F800000u128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_ceil_055, bid128_to_int32_ceil, 0x301069E10DE6FC95C29899892F800001u128, -2147483648, 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_ceil_056, bid128_to_int32_ceil, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_ceil_057, bid128_to_int32_ceil, 0x301069E10DE76676D080000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_ceil_058, bid128_to_int32_ceil, 0x301069E10DE76676D080000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_ceil_059, bid128_to_int32_ceil, 0x301069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_ceil_060, bid128_to_int32_ceil, 0x301069E10DE7D057DE676676D0800000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_ceil_061, bid128_to_int32_ceil, 0x301069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_ceil_062, bid128_to_int32_ceil, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_ceil_063, bid128_to_int32_ceil, 0x301069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_ceil_064, bid128_to_int32_ceil, 0x301069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_ceil_065, bid128_to_int32_ceil, 0x3010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- 4e9-ulp
dec_test!(bid128_to_int32_ceil_066, bid128_to_int32_ceil, 0x3010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_ceil_067, bid128_to_int32_ceil, 0x3010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- 4e9+ulp
dec_test!(bid128_to_int32_ceil_068, bid128_to_int32_ceil, 0x3010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_ceil_069, bid128_to_int32_ceil, 0x3010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_ceil_070, bid128_to_int32_ceil, 0x3010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_ceil_071, bid128_to_int32_ceil, 0x3010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_ceil_072, bid128_to_int32_ceil, 0x3010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_ceil_073, bid128_to_int32_ceil, 0x3010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_ceil_074, bid128_to_int32_ceil, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_ceil_075, bid128_to_int32_ceil, 0x3010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_ceil_076, bid128_to_int32_ceil, 0x3010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_ceil_077, bid128_to_int32_ceil, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_ceil_078, bid128_to_int32_ceil, 0x3010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_ceil_079, bid128_to_int32_ceil, 0x3010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_ceil_080, bid128_to_int32_ceil, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_ceil_081, bid128_to_int32_ceil, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_ceil_082, bid128_to_int32_ceil, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_ceil_083, bid128_to_int32_ceil, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- 5e9-ulp
dec_test!(bid128_to_int32_ceil_084, bid128_to_int32_ceil, 0x3010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_ceil_085, bid128_to_int32_ceil, 0x3010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- 5e9+ulp
dec_test!(bid128_to_int32_ceil_086, bid128_to_int32_ceil, 0x30111240000042087fffffffffffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_087, bid128_to_int32_ceil, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_ceil_088, bid128_to_int32_ceil, 0x3012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_ceil_089, bid128_to_int32_ceil, 0x3012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_ceil_090, bid128_to_int32_ceil, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_ceil_091, bid128_to_int32_ceil, 0x3012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_ceil_092, bid128_to_int32_ceil, 0x3012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_ceil_093, bid128_to_int32_ceil, 0x3012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_ceil_094, bid128_to_int32_ceil, 0x3012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_ceil_095, bid128_to_int32_ceil, 0x3012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_ceil_096, bid128_to_int32_ceil, 0x3012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_ceil_097, bid128_to_int32_ceil, 0x3012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_ceil_098, bid128_to_int32_ceil, 0x3012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_ceil_099, bid128_to_int32_ceil, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_ceil_100, bid128_to_int32_ceil, 0x3012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_ceil_101, bid128_to_int32_ceil, 0x3012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_ceil_102, bid128_to_int32_ceil, 0x3012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_ceil_103, bid128_to_int32_ceil, 0x3012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_ceil_104, bid128_to_int32_ceil, 0x3012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_ceil_105, bid128_to_int32_ceil, 0x3012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_ceil_106, bid128_to_int32_ceil, 0x3012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_ceil_107, bid128_to_int32_ceil, 0x3012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_ceil_108, bid128_to_int32_ceil, 0x301400010042c811971a8281cc618f2eu128, 7930890    , 0x00);
dec_test!(bid128_to_int32_ceil_109, bid128_to_int32_ceil, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483647 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_ceil_110, bid128_to_int32_ceil, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483647 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_ceil_111, bid128_to_int32_ceil, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483647 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_ceil_112, bid128_to_int32_ceil, 0x30180002B5E3AF13FBA450E94E77FFFFu128, -2147483648, 0x01); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_ceil_113, bid128_to_int32_ceil, 0x30180002B5E3AF13FBA450E94E780000u128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_ceil_114, bid128_to_int32_ceil, 0x30180002B5E3AF13FBA450E94E780001u128, -2147483648, 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_ceil_115, bid128_to_int32_ceil, 0x30180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_ceil_116, bid128_to_int32_ceil, 0x30180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_ceil_117, bid128_to_int32_ceil, 0x30180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_ceil_118, bid128_to_int32_ceil, 0x301800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_ceil_119, bid128_to_int32_ceil, 0x301800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_ceil_120, bid128_to_int32_ceil, 0x301800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_ceil_121, bid128_to_int32_ceil, 0x301800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_ceil_122, bid128_to_int32_ceil, 0x301800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_ceil_123, bid128_to_int32_ceil, 0x301800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_ceil_124, bid128_to_int32_ceil, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 301        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_ceil_125, bid128_to_int32_ceil, 0x301A0000000000A2E6C09AD3E0D40000u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_ceil_126, bid128_to_int32_ceil, 0x301A0000000000A2E6C09AD3E0D40001u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_ceil_127, bid128_to_int32_ceil, 0x301a000001400068080c9900409990c0u128, 38685819   , 0x00);
dec_test!(bid128_to_int32_ceil_128, bid128_to_int32_ceil, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483647 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_ceil_129, bid128_to_int32_ceil, 0x301A000045639181BA2CDCFB76180000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_ceil_130, bid128_to_int32_ceil, 0x301A000045639181BA2CDCFB76180001u128, -2147483648, 0x01); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_ceil_131, bid128_to_int32_ceil, 0x301A00004563918244F3FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_ceil_132, bid128_to_int32_ceil, 0x301A00004563918244F4000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_ceil_133, bid128_to_int32_ceil, 0x301A00004563918244F4000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_ceil_134, bid128_to_int32_ceil, 0x301A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_ceil_135, bid128_to_int32_ceil, 0x301A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_ceil_136, bid128_to_int32_ceil, 0x301A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_ceil_137, bid128_to_int32_ceil, 0x301A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_ceil_138, bid128_to_int32_ceil, 0x301A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_ceil_139, bid128_to_int32_ceil, 0x301A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_ceil_140, bid128_to_int32_ceil, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_ceil_141, bid128_to_int32_ceil, 0x301A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_ceil_142, bid128_to_int32_ceil, 0x301A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_ceil_143, bid128_to_int32_ceil, 0x301A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_ceil_144, bid128_to_int32_ceil, 0x301A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_ceil_145, bid128_to_int32_ceil, 0x301A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_ceil_146, bid128_to_int32_ceil, 0x301E000000000001A055690D9DB7FFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_147, bid128_to_int32_ceil, 0x301E000000000001A055690D9DB80000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_148, bid128_to_int32_ceil, 0x301E000000000001A055690D9DB80001u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_149, bid128_to_int32_ceil, 0x302000000000000029A2241AF62BFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_150, bid128_to_int32_ceil, 0x302000000000000029A2241AF62C0000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_151, bid128_to_int32_ceil, 0x302000000000000029A2241AF62C0001u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_152, bid128_to_int32_ceil, 0x3020000000201240cfaf46ff83ff7fffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_153, bid128_to_int32_ceil, 0x3024000000000000006A94D74F42FFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_154, bid128_to_int32_ceil, 0x3024000000000000006A94D74F430000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_155, bid128_to_int32_ceil, 0x3024000000000000006A94D74F430001u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_156, bid128_to_int32_ceil, 0x302A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_ceil_157, bid128_to_int32_ceil, 0x302A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_ceil_158, bid128_to_int32_ceil, 0x302A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_ceil_159, bid128_to_int32_ceil, 0x302A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_ceil_160, bid128_to_int32_ceil, 0x302A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_ceil_161, bid128_to_int32_ceil, 0x302A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_ceil_162, bid128_to_int32_ceil, 0x302C000000000000000002BBA7F521FFu128, 301        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_ceil_163, bid128_to_int32_ceil, 0x302C000000000000000002BBA7F52200u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_ceil_164, bid128_to_int32_ceil, 0x302C000000000000000002BBA7F52201u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_ceil_165, bid128_to_int32_ceil, 0x302C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_ceil_166, bid128_to_int32_ceil, 0x302C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_ceil_167, bid128_to_int32_ceil, 0x302C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_ceil_168, bid128_to_int32_ceil, 0x302C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_ceil_169, bid128_to_int32_ceil, 0x302C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_ceil_170, bid128_to_int32_ceil, 0x302C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_ceil_171, bid128_to_int32_ceil, 0x302C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_ceil_172, bid128_to_int32_ceil, 0x302C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_ceil_173, bid128_to_int32_ceil, 0x302C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_ceil_174, bid128_to_int32_ceil, 0x302E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_ceil_175, bid128_to_int32_ceil, 0x302E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_ceil_176, bid128_to_int32_ceil, 0x302E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_ceil_177, bid128_to_int32_ceil, 0x303000000000000000000006FC23ABFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_178, bid128_to_int32_ceil, 0x303000000000000000000006FC23AC00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_179, bid128_to_int32_ceil, 0x303000000000000000000006FC23AC01u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_180, bid128_to_int32_ceil, 0x303200000000000000000000B2D05DFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_181, bid128_to_int32_ceil, 0x303200000000000000000000B2D05E00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_182, bid128_to_int32_ceil, 0x303200000000000000000000B2D05E01u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_183, bid128_to_int32_ceil, 0x303800000000000000000000002DDA47u128, 301        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_ceil_184, bid128_to_int32_ceil, 0x303800000000000000000000002DDA48u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_ceil_185, bid128_to_int32_ceil, 0x303800000000000000000000002DDA49u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_ceil_186, bid128_to_int32_ceil, 0x303A00000000000000000000000003E7u128, 1          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_ceil_187, bid128_to_int32_ceil, 0x303A00000000000000000000000495D3u128, 301        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_ceil_188, bid128_to_int32_ceil, 0x303A00000000000000000000000495D4u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_ceil_189, bid128_to_int32_ceil, 0x303A00000000000000000000000495D5u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_ceil_190, bid128_to_int32_ceil, 0x303C0000000000000000000000007561u128, 301        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_ceil_191, bid128_to_int32_ceil, 0x303C0000000000000000000000007562u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_ceil_192, bid128_to_int32_ceil, 0x303C0000000000000000000000007563u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_ceil_193, bid128_to_int32_ceil, 0x303E0000000000000000000000000005u128, 1          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_ceil_194, bid128_to_int32_ceil, 0x303E000000000000000000000000000Fu128, 2          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_ceil_195, bid128_to_int32_ceil, 0x303E0000000000000000000000000BB7u128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_196, bid128_to_int32_ceil, 0x303E0000000000000000000000000BB8u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_197, bid128_to_int32_ceil, 0x303E0000000000000000000000000BB9u128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_198, bid128_to_int32_ceil, 0x303E0000000000000000000000000BBDu128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_ceil_199, bid128_to_int32_ceil, 0x303E00000000000000000004FFFFFFF1u128, 2147483647 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_ceil_200, bid128_to_int32_ceil, 0x303E00000000000000000004FFFFFFFBu128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_ceil_201, bid128_to_int32_ceil, 0x303E0000000000000000000500000005u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_ceil_202, bid128_to_int32_ceil, 0x303E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_ceil_203, bid128_to_int32_ceil, 0x303E0000000000000000000A00000005u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_ceil_204, bid128_to_int32_ceil, 0x303E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_ceil_205, bid128_to_int32_ceil, 0x303E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_ceil_206, bid128_to_int32_ceil, 0x303E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_ceil_207, bid128_to_int32_ceil, 0x303E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_ceil_208, bid128_to_int32_ceil, 0x30400000000000000000000000000001u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_ceil_209, bid128_to_int32_ceil, 0x3040000000000000000000000000012Bu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_210, bid128_to_int32_ceil, 0x3040000000000000000000000000012Cu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_211, bid128_to_int32_ceil, 0x3040000000000000000000000000012Du128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_212, bid128_to_int32_ceil, 0x3040000000000000000000007FFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_ceil_213, bid128_to_int32_ceil, 0x30400000000000000000000080000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_ceil_214, bid128_to_int32_ceil, 0x30400000000000000000000080000001u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_ceil_215, bid128_to_int32_ceil, 0x304000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_ceil_216, bid128_to_int32_ceil, 0x30400000000000000000000100000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_ceil_217, bid128_to_int32_ceil, 0x30400000000000000000000100000001u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_ceil_218, bid128_to_int32_ceil, 0x304000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_ceil_219, bid128_to_int32_ceil, 0x304000000000000000000004A817C801u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_ceil_220, bid128_to_int32_ceil, 0x3041ED09BEAD87C0378D8E6400000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_ceil_221, bid128_to_int32_ceil, 0x3042000000000000000000000000001Du128, 290        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_ceil_222, bid128_to_int32_ceil, 0x3042000000000000000000000000001Eu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_223, bid128_to_int32_ceil, 0x3042000000000000000000000000001Fu128, 310        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_ceil_224, bid128_to_int32_ceil, 0x304200000000000000000000773593FFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_ceil_225, bid128_to_int32_ceil, 0x30420000000000000000000077359400u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_ceil_226, bid128_to_int32_ceil, 0x30420000000000000000000077359401u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_ceil_227, bid128_to_int32_ceil, 0x30440000000000000000000000000003u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_ceil_228, bid128_to_int32_ceil, 0x30520000000000000000000000000004u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_ceil_229, bid128_to_int32_ceil, 0x30520000000000000000000000000005u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_ceil_230, bid128_to_int32_ceil, 0x30540000000000000000000000000002u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_ceil_231, bid128_to_int32_ceil, 0x4918f959f2e23dc268ce9bb9a88e4011u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_232, bid128_to_int32_ceil, "5.05"                                , 6          , 0x00);
dec_test!(bid128_to_int32_ceil_233, bid128_to_int32_ceil, 0x50afc0fdc842c3ac34774350084198bbu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_234, bid128_to_int32_ceil, "5.5"                                 , 6          , 0x00);
dec_test!(bid128_to_int32_ceil_235, bid128_to_int32_ceil, 0x647902186878316905824ea1600b001au128, 0          , 0x00);
dec_test!(bid128_to_int32_ceil_236, bid128_to_int32_ceil, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_237, bid128_to_int32_ceil, 0x78485446e5d40a02c4483688108d3857u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_238, bid128_to_int32_ceil, 0x7c000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_239, bid128_to_int32_ceil, 0x7c003fffffffffff38c15b08ffffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_240, bid128_to_int32_ceil, 0x7c003fffffffffff38c15b0affffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_241, bid128_to_int32_ceil, 0x7e000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_242, bid128_to_int32_ceil, 0x9332ad54e6331009042244ce2a8684f1u128, 0          , 0x00);
dec_test!(bid128_to_int32_ceil_243, bid128_to_int32_ceil, "-9"                                  , -9         , 0x00);
dec_test!(bid128_to_int32_ceil_244, bid128_to_int32_ceil, 0xa96715f9325a44e13f413f0de340cbd3u128, 0          , 0x00);
dec_test!(bid128_to_int32_ceil_245, bid128_to_int32_ceil, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_int32_ceil_246, bid128_to_int32_ceil, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0          , 0x00); // -- -(0.5)
dec_test!(bid128_to_int32_ceil_247, bid128_to_int32_ceil, 0xAFFCF684DF56C3E01BC6C73200000001u128, 0          , 0x00); // -- -(0.5+ulp)
dec_test!(bid128_to_int32_ceil_248, bid128_to_int32_ceil, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 0          , 0x00); // -- -(0.999-ulp)
dec_test!(bid128_to_int32_ceil_249, bid128_to_int32_ceil, 0xAFFDEC8B86EF679D76FC433D80000000u128, 0          , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_ceil_250, bid128_to_int32_ceil, 0xAFFDEC8B86EF679D76FC433D80000001u128, 0          , 0x00); // -- -(0.999+ulp)
dec_test!(bid128_to_int32_ceil_251, bid128_to_int32_ceil, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 0          , 0x00); // -- -(1-ulp)
dec_test!(bid128_to_int32_ceil_252, bid128_to_int32_ceil, 0xAFFE314DC6448D9338C15B0A00000000u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_ceil_253, bid128_to_int32_ceil, 0xAFFE314DC6448D9338C15B0A00000001u128, -1         , 0x00); // -- -(1+ulp)
dec_test!(bid128_to_int32_ceil_254, bid128_to_int32_ceil, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1         , 0x00); // -- -(1.5-ulp)
dec_test!(bid128_to_int32_ceil_255, bid128_to_int32_ceil, 0xAFFE49F4A966D45CD522088F00000000u128, -1         , 0x00); // -- -(1.5)
dec_test!(bid128_to_int32_ceil_256, bid128_to_int32_ceil, 0xAFFE49F4A966D45CD522088F00000001u128, -1         , 0x00); // -- -(1.5+ulp)
dec_test!(bid128_to_int32_ceil_257, bid128_to_int32_ceil, 0xafff77fab7feedfe45a2409a9ba02e42u128, -7         , 0x00);
dec_test!(bid128_to_int32_ceil_258, bid128_to_int32_ceil, 0xb0017c6d988ec8d125b24d469a54f5c6u128, -77        , 0x00);
dec_test!(bid128_to_int32_ceil_259, bid128_to_int32_ceil, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_260, bid128_to_int32_ceil, 0xB00293E952CDA8B9AA44111E00000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_261, bid128_to_int32_ceil, 0xB00293E952CDA8B9AA44111E00000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_262, bid128_to_int32_ceil, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_ceil_263, bid128_to_int32_ceil, 0xB00294286EACB8CB0A8CB6B140000000u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_ceil_264, bid128_to_int32_ceil, 0xB00294286EACB8CB0A8CB6B140000001u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_ceil_265, bid128_to_int32_ceil, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_266, bid128_to_int32_ceil, 0xB0040ECA8847C4129106CE8300000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_267, bid128_to_int32_ceil, 0xB0040ECA8847C4129106CE8300000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_268, bid128_to_int32_ceil, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_269, bid128_to_int32_ceil, 0xB00A0003C95A2F0B4856475FE0000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_270, bid128_to_int32_ceil, 0xB00A0003C95A2F0B4856475FE0000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_271, bid128_to_int32_ceil, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_272, bid128_to_int32_ceil, 0xB00C000060EF6B1ABA6F072330000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_273, bid128_to_int32_ceil, 0xB00C000060EF6B1ABA6F072330000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_274, bid128_to_int32_ceil, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, -2147483646, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_ceil_275, bid128_to_int32_ceil, 0xB01069E10DE628D3A6C9CC9B8E800000u128, -2147483646, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_ceil_276, bid128_to_int32_ceil, 0xB01069E10DE628D3A6C9CC9B8E800001u128, -2147483646, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_ceil_277, bid128_to_int32_ceil, 0xB01069E10DE692B4B4B133125EFFFFFFu128, -2147483646, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_ceil_278, bid128_to_int32_ceil, 0xB01069E10DE692B4B4B133125F000000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_ceil_279, bid128_to_int32_ceil, 0xB01069E10DE692B4B4B133125F000001u128, -2147483647, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_ceil_280, bid128_to_int32_ceil, 0xB01069E10DE6FC95C29899892F7FFFFFu128, -2147483647, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_ceil_281, bid128_to_int32_ceil, 0xB01069E10DE6FC95C29899892F800000u128, -2147483647, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_ceil_282, bid128_to_int32_ceil, 0xB01069E10DE6FC95C29899892F800001u128, -2147483647, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_ceil_283, bid128_to_int32_ceil, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, -2147483647, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_ceil_284, bid128_to_int32_ceil, 0xB01069E10DE76676D080000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_ceil_285, bid128_to_int32_ceil, 0xB01069E10DE76676D080000000000001u128, -2147483648, 0x00); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_ceil_286, bid128_to_int32_ceil, 0xB01069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x00); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_ceil_287, bid128_to_int32_ceil, 0xB01069E10DE7D057DE676676D0800000u128, -2147483648, 0x00); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_ceil_288, bid128_to_int32_ceil, 0xB01069E10DE7D057DE676676D0800001u128, -2147483648, 0x00); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_ceil_289, bid128_to_int32_ceil, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x00); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_ceil_290, bid128_to_int32_ceil, 0xB01069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_ceil_291, bid128_to_int32_ceil, 0xB01069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_ceil_292, bid128_to_int32_ceil, 0xB010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_int32_ceil_293, bid128_to_int32_ceil, 0xB010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_ceil_294, bid128_to_int32_ceil, 0xB010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_int32_ceil_295, bid128_to_int32_ceil, 0xB010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_ceil_296, bid128_to_int32_ceil, 0xB010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_ceil_297, bid128_to_int32_ceil, 0xB010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_ceil_298, bid128_to_int32_ceil, 0xB010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_ceil_299, bid128_to_int32_ceil, 0xB010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_ceil_300, bid128_to_int32_ceil, 0xB010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_ceil_301, bid128_to_int32_ceil, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_ceil_302, bid128_to_int32_ceil, 0xB010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_ceil_303, bid128_to_int32_ceil, 0xB010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_ceil_304, bid128_to_int32_ceil, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_ceil_305, bid128_to_int32_ceil, 0xB010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_ceil_306, bid128_to_int32_ceil, 0xB010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_ceil_307, bid128_to_int32_ceil, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_ceil_308, bid128_to_int32_ceil, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_ceil_309, bid128_to_int32_ceil, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_ceil_310, bid128_to_int32_ceil, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_int32_ceil_311, bid128_to_int32_ceil, 0xB010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_ceil_312, bid128_to_int32_ceil, 0xB010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_int32_ceil_313, bid128_to_int32_ceil, 0xb0116a00b5002803d682fff98ffdd5ffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_314, bid128_to_int32_ceil, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_ceil_315, bid128_to_int32_ceil, 0xB012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_ceil_316, bid128_to_int32_ceil, 0xB012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_ceil_317, bid128_to_int32_ceil, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_ceil_318, bid128_to_int32_ceil, 0xB012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_ceil_319, bid128_to_int32_ceil, 0xB012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_ceil_320, bid128_to_int32_ceil, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_ceil_321, bid128_to_int32_ceil, 0xB012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_ceil_322, bid128_to_int32_ceil, 0xB012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_ceil_323, bid128_to_int32_ceil, 0xB012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_ceil_324, bid128_to_int32_ceil, 0xB012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_ceil_325, bid128_to_int32_ceil, 0xB012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_ceil_326, bid128_to_int32_ceil, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_ceil_327, bid128_to_int32_ceil, 0xB012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_ceil_328, bid128_to_int32_ceil, 0xB012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_ceil_329, bid128_to_int32_ceil, 0xB012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_ceil_330, bid128_to_int32_ceil, 0xB012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_ceil_331, bid128_to_int32_ceil, 0xB012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_ceil_332, bid128_to_int32_ceil, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_ceil_333, bid128_to_int32_ceil, 0xB012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_ceil_334, bid128_to_int32_ceil, 0xB012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_ceil_335, bid128_to_int32_ceil, 0xb0140000440840010000080000000020u128, -2105495   , 0x00);
dec_test!(bid128_to_int32_ceil_336, bid128_to_int32_ceil, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, -2147483646, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_ceil_337, bid128_to_int32_ceil, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, -2147483646, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_ceil_338, bid128_to_int32_ceil, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, -2147483646, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_ceil_339, bid128_to_int32_ceil, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, -2147483647, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_ceil_340, bid128_to_int32_ceil, 0xB0180002B5E3AF13FBA450E94E780000u128, -2147483647, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_ceil_341, bid128_to_int32_ceil, 0xB0180002B5E3AF13FBA450E94E780001u128, -2147483647, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_ceil_342, bid128_to_int32_ceil, 0xB0180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x00); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_ceil_343, bid128_to_int32_ceil, 0xB0180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x00); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_ceil_344, bid128_to_int32_ceil, 0xB0180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x00); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_ceil_345, bid128_to_int32_ceil, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_ceil_346, bid128_to_int32_ceil, 0xB01800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_ceil_347, bid128_to_int32_ceil, 0xB01800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_ceil_348, bid128_to_int32_ceil, 0xB01800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_ceil_349, bid128_to_int32_ceil, 0xB01800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_ceil_350, bid128_to_int32_ceil, 0xB01800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_ceil_351, bid128_to_int32_ceil, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_ceil_352, bid128_to_int32_ceil, 0xB01A0000000000A2E6C09AD3E0D40000u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_ceil_353, bid128_to_int32_ceil, 0xB01A0000000000A2E6C09AD3E0D40001u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_ceil_354, bid128_to_int32_ceil, 0xB01A000045639181BA2CDCFB7617FFFFu128, -2147483646, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_ceil_355, bid128_to_int32_ceil, 0xB01A000045639181BA2CDCFB76180000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_ceil_356, bid128_to_int32_ceil, 0xB01A000045639181BA2CDCFB76180001u128, -2147483647, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_ceil_357, bid128_to_int32_ceil, 0xB01A00004563918244F3FFFFFFFFFFFFu128, -2147483647, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_ceil_358, bid128_to_int32_ceil, 0xB01A00004563918244F4000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_ceil_359, bid128_to_int32_ceil, 0xB01A00004563918244F4000000000001u128, -2147483648, 0x00); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_ceil_360, bid128_to_int32_ceil, 0xB01A000045639182CFBB230489E7FFFFu128, -2147483648, 0x00); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_ceil_361, bid128_to_int32_ceil, 0xB01A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_ceil_362, bid128_to_int32_ceil, 0xB01A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_ceil_363, bid128_to_int32_ceil, 0xB01A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_ceil_364, bid128_to_int32_ceil, 0xB01A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_ceil_365, bid128_to_int32_ceil, 0xB01A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_ceil_366, bid128_to_int32_ceil, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_ceil_367, bid128_to_int32_ceil, 0xB01A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_ceil_368, bid128_to_int32_ceil, 0xB01A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_ceil_369, bid128_to_int32_ceil, 0xB01A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_ceil_370, bid128_to_int32_ceil, 0xB01A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_ceil_371, bid128_to_int32_ceil, 0xB01A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_ceil_372, bid128_to_int32_ceil, 0xB01E000000000001A055690D9DB7FFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_373, bid128_to_int32_ceil, 0xB01E000000000001A055690D9DB80000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_374, bid128_to_int32_ceil, 0xB01E000000000001A055690D9DB80001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_375, bid128_to_int32_ceil, 0xB02000000000000029A2241AF62BFFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_376, bid128_to_int32_ceil, 0xB02000000000000029A2241AF62C0000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_377, bid128_to_int32_ceil, 0xB02000000000000029A2241AF62C0001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_378, bid128_to_int32_ceil, 0xb020000000412100a00000c000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_ceil_379, bid128_to_int32_ceil, 0xB024000000000000006A94D74F42FFFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_380, bid128_to_int32_ceil, 0xB024000000000000006A94D74F430000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_381, bid128_to_int32_ceil, 0xB024000000000000006A94D74F430001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_382, bid128_to_int32_ceil, 0xB02A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_ceil_383, bid128_to_int32_ceil, 0xB02A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_ceil_384, bid128_to_int32_ceil, 0xB02A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_ceil_385, bid128_to_int32_ceil, 0xB02A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_ceil_386, bid128_to_int32_ceil, 0xB02A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_ceil_387, bid128_to_int32_ceil, 0xB02A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_ceil_388, bid128_to_int32_ceil, 0xB02C000000000000000002BBA7F521FFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_ceil_389, bid128_to_int32_ceil, 0xB02C000000000000000002BBA7F52200u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_ceil_390, bid128_to_int32_ceil, 0xB02C000000000000000002BBA7F52201u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_ceil_391, bid128_to_int32_ceil, 0xB02C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_ceil_392, bid128_to_int32_ceil, 0xB02C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_ceil_393, bid128_to_int32_ceil, 0xB02C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_ceil_394, bid128_to_int32_ceil, 0xB02C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_ceil_395, bid128_to_int32_ceil, 0xB02C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_ceil_396, bid128_to_int32_ceil, 0xB02C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_ceil_397, bid128_to_int32_ceil, 0xB02C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_ceil_398, bid128_to_int32_ceil, 0xB02C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_ceil_399, bid128_to_int32_ceil, 0xB02C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_ceil_400, bid128_to_int32_ceil, 0xB02E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_ceil_401, bid128_to_int32_ceil, 0xB02E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_ceil_402, bid128_to_int32_ceil, 0xB02E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_ceil_403, bid128_to_int32_ceil, 0xB03000000000000000000006FC23ABFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_404, bid128_to_int32_ceil, 0xB03000000000000000000006FC23AC00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_405, bid128_to_int32_ceil, 0xB03000000000000000000006FC23AC01u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_406, bid128_to_int32_ceil, 0xB03200000000000000000000B2D05DFFu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_407, bid128_to_int32_ceil, 0xB03200000000000000000000B2D05E00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_408, bid128_to_int32_ceil, 0xB03200000000000000000000B2D05E01u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_409, bid128_to_int32_ceil, 0xB03800000000000000000000002DDA47u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_ceil_410, bid128_to_int32_ceil, 0xB03800000000000000000000002DDA48u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_ceil_411, bid128_to_int32_ceil, 0xB03800000000000000000000002DDA49u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_ceil_412, bid128_to_int32_ceil, 0xB03A00000000000000000000000003E7u128, 0          , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_ceil_413, bid128_to_int32_ceil, 0xB03A00000000000000000000000495D3u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_ceil_414, bid128_to_int32_ceil, 0xB03A00000000000000000000000495D4u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_ceil_415, bid128_to_int32_ceil, 0xB03A00000000000000000000000495D5u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_ceil_416, bid128_to_int32_ceil, 0xB03C0000000000000000000000007561u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_ceil_417, bid128_to_int32_ceil, 0xB03C0000000000000000000000007562u128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_ceil_418, bid128_to_int32_ceil, 0xB03C0000000000000000000000007563u128, -300       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_ceil_419, bid128_to_int32_ceil, 0xB03E0000000000000000000000000005u128, 0          , 00); // -- -(0.5)
dec_test!(bid128_to_int32_ceil_420, bid128_to_int32_ceil, 0xB03E000000000000000000000000000Fu128, -1         , 00); // -- -(1.5)
dec_test!(bid128_to_int32_ceil_421, bid128_to_int32_ceil, 0xB03E0000000000000000000000000BB7u128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_422, bid128_to_int32_ceil, 0xB03E0000000000000000000000000BB8u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_ceil_423, bid128_to_int32_ceil, 0xB03E0000000000000000000000000BB9u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_424, bid128_to_int32_ceil, 0xB03E0000000000000000000000000BBDu128, -300       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_ceil_425, bid128_to_int32_ceil, 0xB03E00000000000000000004FFFFFFF1u128, -2147483646, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_ceil_426, bid128_to_int32_ceil, 0xB03E00000000000000000004FFFFFFFBu128, -2147483647, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_ceil_427, bid128_to_int32_ceil, 0xB03E0000000000000000000500000005u128, -2147483648, 0x00); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_ceil_428, bid128_to_int32_ceil, 0xB03E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_ceil_429, bid128_to_int32_ceil, 0xB03E0000000000000000000A00000005u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_ceil_430, bid128_to_int32_ceil, 0xB03E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_ceil_431, bid128_to_int32_ceil, 0xB03E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_ceil_432, bid128_to_int32_ceil, 0xB03E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_ceil_433, bid128_to_int32_ceil, 0xB03E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_ceil_434, bid128_to_int32_ceil, 0xB0400000000000000000000000000001u128, -1         , 00); // -- -(1)
dec_test!(bid128_to_int32_ceil_435, bid128_to_int32_ceil, 0xB040000000000000000000000000012Bu128, -299       , 00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_436, bid128_to_int32_ceil, 0xB040000000000000000000000000012Cu128, -300       , 00); // -- -(300)
dec_test!(bid128_to_int32_ceil_437, bid128_to_int32_ceil, 0xB040000000000000000000000000012Du128, -301       , 00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_438, bid128_to_int32_ceil, 0xB040000000000000000000007FFFFFFFu128, -2147483647, 00); // -- -(2^31-1)
dec_test!(bid128_to_int32_ceil_439, bid128_to_int32_ceil, 0xB0400000000000000000000080000000u128, -2147483648, 00); // -- -(2^31)
dec_test!(bid128_to_int32_ceil_440, bid128_to_int32_ceil, 0xB0400000000000000000000080000001u128, -2147483648, 01); // -- -(2^31+1)
dec_test!(bid128_to_int32_ceil_441, bid128_to_int32_ceil, 0xB04000000000000000000000FFFFFFFFu128, -2147483648, 01); // -- -(2^32-1)
dec_test!(bid128_to_int32_ceil_442, bid128_to_int32_ceil, 0xB0400000000000000000000100000000u128, -2147483648, 01); // -- -(2^32)
dec_test!(bid128_to_int32_ceil_443, bid128_to_int32_ceil, 0xB0400000000000000000000100000001u128, -2147483648, 01); // -- -(2^32+1)
dec_test!(bid128_to_int32_ceil_444, bid128_to_int32_ceil, 0xB04000000000000000000004A817C7FFu128, -2147483648, 01); // -- -(2e10-1)
dec_test!(bid128_to_int32_ceil_445, bid128_to_int32_ceil, 0xB04000000000000000000004A817C801u128, -2147483648, 01); // -- -(2e10+1)
dec_test!(bid128_to_int32_ceil_446, bid128_to_int32_ceil, 0xB042000000000000000000000000001Du128, -290       , 00); // -- -(300-ulp)
dec_test!(bid128_to_int32_ceil_447, bid128_to_int32_ceil, 0xB042000000000000000000000000001Eu128, -300       , 00); // -- -(300)
dec_test!(bid128_to_int32_ceil_448, bid128_to_int32_ceil, 0xB042000000000000000000000000001Fu128, -310       , 00); // -- -(300+ulp)
dec_test!(bid128_to_int32_ceil_449, bid128_to_int32_ceil, 0xB04200000000000000000000773593FFu128, -2147483648, 01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_ceil_450, bid128_to_int32_ceil, 0xB0420000000000000000000077359400u128, -2147483648, 01); // -- -(2e10)
dec_test!(bid128_to_int32_ceil_451, bid128_to_int32_ceil, 0xB0420000000000000000000077359401u128, -2147483648, 01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_ceil_452, bid128_to_int32_ceil, 0xB0440000000000000000000000000003u128, -300       , 00); // -- -(300)
dec_test!(bid128_to_int32_ceil_453, bid128_to_int32_ceil, 0xB0520000000000000000000000000004u128, -2147483648, 01); // -- -(4e9)
dec_test!(bid128_to_int32_ceil_454, bid128_to_int32_ceil, 0xB0520000000000000000000000000005u128, -2147483648, 01); // -- -(5e9)
dec_test!(bid128_to_int32_ceil_455, bid128_to_int32_ceil, 0xB0540000000000000000000000000002u128, -2147483648, 01); // -- -(2e10)
dec_test!(bid128_to_int32_ceil_456, bid128_to_int32_ceil, 0xc0acaab4903a5d352a97091f09a48523u128, -2147483648, 01);
dec_test!(bid128_to_int32_ceil_457, bid128_to_int32_ceil, 0xc5799e203ad96a1192d17cb72860e6b3u128, -2147483648, 01);
dec_test!(bid128_to_int32_ceil_458, bid128_to_int32_ceil, 0xcae587a584dd961034721b920398ad2fu128, -2147483648, 01);
dec_test!(bid128_to_int32_ceil_459, bid128_to_int32_ceil, 0xcfb54af90e99798e8100000000000014u128, -2147483648, 01);
dec_test!(bid128_to_int32_ceil_460, bid128_to_int32_ceil, 0xfbd6b77b4b5fdb3eff97fa7d5f72ffffu128, -2147483648, 01);
dec_test!(bid128_to_int32_ceil_461, bid128_to_int32_ceil, 0xfd66946885a384a3760997164b5e1975u128, -2147483648, 01);
dec_test!(bid128_to_int32_ceil_462, bid128_to_int32_ceil, 0xfffbbbefff5df77f302020b4aea4dc82u128, -2147483648, 01);
dec_test!(bid128_to_int32_ceil_463, bid128_to_int32_ceil, "-Infinity"                           , -2147483648, 01);
dec_test!(bid128_to_int32_ceil_464, bid128_to_int32_ceil,  "Infinity"                           , -2147483648, 01);