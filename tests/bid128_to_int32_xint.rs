/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int32_xint_001, bid128_to_int32_xint, "-0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_xint_002, bid128_to_int32_xint,  "0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_xint_003, bid128_to_int32_xint, 0x00000000000000000000000000010000u128, 0          , 0x20);
dec_test!(bid128_to_int32_xint_004, bid128_to_int32_xint, 0x0000000000000000ffbffffebfbc7effu128, 0          , 0x20);
dec_test!(bid128_to_int32_xint_005, bid128_to_int32_xint, 0x0001ed09bead87c0378d8e62ffffffffu128, 0          , 0x20);
dec_test!(bid128_to_int32_xint_006, bid128_to_int32_xint, 0x0001ed09bead87c0378d8e64ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_007, bid128_to_int32_xint, "-10010.0101001011E0"                 , -10010     , 0x20);
dec_test!(bid128_to_int32_xint_008, bid128_to_int32_xint, "1.0"                                 , 1          , 0x00);
dec_test!(bid128_to_int32_xint_009, bid128_to_int32_xint, "1073741824"                          , 1073741824 , 0x00);
dec_test!(bid128_to_int32_xint_010, bid128_to_int32_xint, "1"                                   , 1          , 0x00);
dec_test!(bid128_to_int32_xint_011, bid128_to_int32_xint, 0x1183523648dbf5556db98fc8b4f9d339u128, 0          , 0x20);
dec_test!(bid128_to_int32_xint_012, bid128_to_int32_xint, 0x1a340000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_013, bid128_to_int32_xint, 0x20020000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_014, bid128_to_int32_xint, "2147483648"                          , -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_015, bid128_to_int32_xint, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_int32_xint_016, bid128_to_int32_xint, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0          , 0x20); // -- 0.5
dec_test!(bid128_to_int32_xint_017, bid128_to_int32_xint, 0x2FFCF684DF56C3E01BC6C73200000001u128, 0          , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_int32_xint_018, bid128_to_int32_xint, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 0          , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_int32_xint_019, bid128_to_int32_xint, 0x2FFDEC8B86EF679D76FC433D80000000u128, 0          , 0x20); // -- 0.999
dec_test!(bid128_to_int32_xint_020, bid128_to_int32_xint, 0x2FFDEC8B86EF679D76FC433D80000001u128, 0          , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_int32_xint_021, bid128_to_int32_xint, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 0          , 0x20); // -- 1-ulp
dec_test!(bid128_to_int32_xint_022, bid128_to_int32_xint, 0x2FFE314DC6448D9338C15B0A00000000u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_xint_023, bid128_to_int32_xint, 0x2FFE314DC6448D9338C15B0A00000001u128, 1          , 0x20); // -- 1+ulp
dec_test!(bid128_to_int32_xint_024, bid128_to_int32_xint, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1          , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_int32_xint_025, bid128_to_int32_xint, 0x2FFE49F4A966D45CD522088F00000000u128, 1          , 0x20); // -- 1.5
dec_test!(bid128_to_int32_xint_026, bid128_to_int32_xint, 0x2FFE49F4A966D45CD522088F00000001u128, 1          , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_int32_xint_027, bid128_to_int32_xint, 0x3001034200100008683f4be6eb49f455u128, 52         , 0x20);
dec_test!(bid128_to_int32_xint_028, bid128_to_int32_xint, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_029, bid128_to_int32_xint, 0x300293E952CDA8B9AA44111E00000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_030, bid128_to_int32_xint, 0x300293E952CDA8B9AA44111E00000001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_031, bid128_to_int32_xint, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xint_032, bid128_to_int32_xint, 0x300294286EACB8CB0A8CB6B140000000u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xint_033, bid128_to_int32_xint, 0x300294286EACB8CB0A8CB6B140000001u128, 300        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xint_034, bid128_to_int32_xint, 0x30032010000090c0fffffffdff7ff7ffu128, 584        , 0x20);
dec_test!(bid128_to_int32_xint_035, bid128_to_int32_xint, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_036, bid128_to_int32_xint, 0x30040ECA8847C4129106CE8300000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_037, bid128_to_int32_xint, 0x30040ECA8847C4129106CE8300000001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_038, bid128_to_int32_xint, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_039, bid128_to_int32_xint, 0x300A0003C95A2F0B4856475FE0000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_040, bid128_to_int32_xint, 0x300A0003C95A2F0B4856475FE0000001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_041, bid128_to_int32_xint, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_042, bid128_to_int32_xint, 0x300C000060EF6B1ABA6F072330000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_043, bid128_to_int32_xint, 0x300C000060EF6B1ABA6F072330000001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_044, bid128_to_int32_xint, 0x301039882d042880968c705a1b38ce1eu128, 1166886309 , 0x20);
dec_test!(bid128_to_int32_xint_045, bid128_to_int32_xint, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646 , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_xint_046, bid128_to_int32_xint, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483646 , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_int32_xint_047, bid128_to_int32_xint, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483646 , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_xint_048, bid128_to_int32_xint, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483646 , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_xint_049, bid128_to_int32_xint, 0x301069E10DE692B4B4B133125F000000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_xint_050, bid128_to_int32_xint, 0x301069E10DE692B4B4B133125F000001u128, 2147483647 , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_xint_051, bid128_to_int32_xint, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647 , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_xint_052, bid128_to_int32_xint, 0x301069E10DE6FC95C29899892F800000u128, 2147483647 , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_int32_xint_053, bid128_to_int32_xint, 0x301069E10DE6FC95C29899892F800001u128, 2147483647 , 0x20); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_xint_054, bid128_to_int32_xint, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, 2147483647 , 0x20); // -- 2^31-ulp
dec_test!(bid128_to_int32_xint_055, bid128_to_int32_xint, 0x301069E10DE76676D080000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_xint_056, bid128_to_int32_xint, 0x301069E10DE76676D080000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_xint_057, bid128_to_int32_xint, 0x301069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_xint_058, bid128_to_int32_xint, 0x301069E10DE7D057DE676676D0800000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_xint_059, bid128_to_int32_xint, 0x301069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_xint_060, bid128_to_int32_xint, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_xint_061, bid128_to_int32_xint, 0x301069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_xint_062, bid128_to_int32_xint, 0x301069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_xint_063, bid128_to_int32_xint, 0x3010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- 4e9-ulp
dec_test!(bid128_to_int32_xint_064, bid128_to_int32_xint, 0x3010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_xint_065, bid128_to_int32_xint, 0x3010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- 4e9+ulp
dec_test!(bid128_to_int32_xint_066, bid128_to_int32_xint, 0x3010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_xint_067, bid128_to_int32_xint, 0x3010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_xint_068, bid128_to_int32_xint, 0x3010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_xint_069, bid128_to_int32_xint, 0x3010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_xint_070, bid128_to_int32_xint, 0x3010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_xint_071, bid128_to_int32_xint, 0x3010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_xint_072, bid128_to_int32_xint, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_xint_073, bid128_to_int32_xint, 0x3010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_xint_074, bid128_to_int32_xint, 0x3010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_xint_075, bid128_to_int32_xint, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_xint_076, bid128_to_int32_xint, 0x3010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_xint_077, bid128_to_int32_xint, 0x3010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_xint_078, bid128_to_int32_xint, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_xint_079, bid128_to_int32_xint, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_xint_080, bid128_to_int32_xint, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_xint_081, bid128_to_int32_xint, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- 5e9-ulp
dec_test!(bid128_to_int32_xint_082, bid128_to_int32_xint, 0x3010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_xint_083, bid128_to_int32_xint, 0x3010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- 5e9+ulp
dec_test!(bid128_to_int32_xint_084, bid128_to_int32_xint, 0x30111fb9f1b6d1f5c7e495226c5c54efu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_085, bid128_to_int32_xint, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_xint_086, bid128_to_int32_xint, 0x3012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_xint_087, bid128_to_int32_xint, 0x3012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_xint_088, bid128_to_int32_xint, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_xint_089, bid128_to_int32_xint, 0x3012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_xint_090, bid128_to_int32_xint, 0x3012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_xint_091, bid128_to_int32_xint, 0x3012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_xint_092, bid128_to_int32_xint, 0x3012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_xint_093, bid128_to_int32_xint, 0x3012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_xint_094, bid128_to_int32_xint, 0x3012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_xint_095, bid128_to_int32_xint, 0x3012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_xint_096, bid128_to_int32_xint, 0x3012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_xint_097, bid128_to_int32_xint, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_xint_098, bid128_to_int32_xint, 0x3012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_xint_099, bid128_to_int32_xint, 0x3012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_xint_100, bid128_to_int32_xint, 0x3012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_xint_101, bid128_to_int32_xint, 0x3012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_xint_102, bid128_to_int32_xint, 0x3012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_xint_103, bid128_to_int32_xint, 0x3012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_xint_104, bid128_to_int32_xint, 0x3012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_xint_105, bid128_to_int32_xint, 0x3012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_xint_106, bid128_to_int32_xint, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646 , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_xint_107, bid128_to_int32_xint, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483646 , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_int32_xint_108, bid128_to_int32_xint, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483646 , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_xint_109, bid128_to_int32_xint, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647 , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_xint_110, bid128_to_int32_xint, 0x30180002B5E3AF13FBA450E94E780000u128, 2147483647 , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_int32_xint_111, bid128_to_int32_xint, 0x30180002B5E3AF13FBA450E94E780001u128, 2147483647 , 0x20); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_xint_112, bid128_to_int32_xint, 0x30180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_xint_113, bid128_to_int32_xint, 0x30180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_xint_114, bid128_to_int32_xint, 0x30180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_xint_115, bid128_to_int32_xint, 0x301800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_xint_116, bid128_to_int32_xint, 0x301800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_xint_117, bid128_to_int32_xint, 0x301800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_xint_118, bid128_to_int32_xint, 0x301800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_xint_119, bid128_to_int32_xint, 0x301800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_xint_120, bid128_to_int32_xint, 0x301800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_xint_121, bid128_to_int32_xint, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xint_122, bid128_to_int32_xint, 0x301A0000000000A2E6C09AD3E0D40000u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xint_123, bid128_to_int32_xint, 0x301A0000000000A2E6C09AD3E0D40001u128, 300        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xint_124, bid128_to_int32_xint, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483646 , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_xint_125, bid128_to_int32_xint, 0x301A000045639181BA2CDCFB76180000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_xint_126, bid128_to_int32_xint, 0x301A000045639181BA2CDCFB76180001u128, 2147483647 , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_xint_127, bid128_to_int32_xint, 0x301A00004563918244F3FFFFFFFFFFFFu128, 2147483647 , 0x20); // -- 2^31-ulp
dec_test!(bid128_to_int32_xint_128, bid128_to_int32_xint, 0x301A00004563918244F4000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_xint_129, bid128_to_int32_xint, 0x301A00004563918244F4000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_xint_130, bid128_to_int32_xint, 0x301A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_xint_131, bid128_to_int32_xint, 0x301A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_xint_132, bid128_to_int32_xint, 0x301A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_xint_133, bid128_to_int32_xint, 0x301A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_xint_134, bid128_to_int32_xint, 0x301A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_xint_135, bid128_to_int32_xint, 0x301A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_xint_136, bid128_to_int32_xint, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_xint_137, bid128_to_int32_xint, 0x301A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_xint_138, bid128_to_int32_xint, 0x301A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_xint_139, bid128_to_int32_xint, 0x301A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_xint_140, bid128_to_int32_xint, 0x301A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_xint_141, bid128_to_int32_xint, 0x301A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_xint_142, bid128_to_int32_xint, 0x301E000000000001A055690D9DB7FFFFu128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_143, bid128_to_int32_xint, 0x301E000000000001A055690D9DB80000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_144, bid128_to_int32_xint, 0x301E000000000001A055690D9DB80001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_145, bid128_to_int32_xint, 0x302000000000000029A2241AF62BFFFFu128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_146, bid128_to_int32_xint, 0x302000000000000029A2241AF62C0000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_147, bid128_to_int32_xint, 0x302000000000000029A2241AF62C0001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_148, bid128_to_int32_xint, 0x302000000018486877ffdfeebdfbcc7au128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_149, bid128_to_int32_xint, 0x3024000000000000006A94D74F42FFFFu128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_150, bid128_to_int32_xint, 0x3024000000000000006A94D74F430000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_151, bid128_to_int32_xint, 0x3024000000000000006A94D74F430001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_152, bid128_to_int32_xint, 0x30240000000010000efe4fe5bfd7fffcu128, 755589441  , 0x20);
dec_test!(bid128_to_int32_xint_153, bid128_to_int32_xint, 0x302A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_xint_154, bid128_to_int32_xint, 0x302A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_xint_155, bid128_to_int32_xint, 0x302A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_xint_156, bid128_to_int32_xint, 0x302A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_xint_157, bid128_to_int32_xint, 0x302A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_xint_158, bid128_to_int32_xint, 0x302A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_xint_159, bid128_to_int32_xint, 0x302C000000000000000002BBA7F521FFu128, 300        , 0x20); //, 0x-- 300.5-ulp
dec_test!(bid128_to_int32_xint_160, bid128_to_int32_xint, 0x302C000000000000000002BBA7F52200u128, 300        , 0x20); //, 0x-- 300.5
dec_test!(bid128_to_int32_xint_161, bid128_to_int32_xint, 0x302C000000000000000002BBA7F52201u128, 300        , 0x20); //, 0x-- 300.5+ulp
dec_test!(bid128_to_int32_xint_162, bid128_to_int32_xint, 0x302C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_xint_163, bid128_to_int32_xint, 0x302C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_xint_164, bid128_to_int32_xint, 0x302C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_xint_165, bid128_to_int32_xint, 0x302C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_xint_166, bid128_to_int32_xint, 0x302C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_xint_167, bid128_to_int32_xint, 0x302C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_xint_168, bid128_to_int32_xint, 0x302C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_xint_169, bid128_to_int32_xint, 0x302C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_xint_170, bid128_to_int32_xint, 0x302C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_xint_171, bid128_to_int32_xint, 0x302E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_xint_172, bid128_to_int32_xint, 0x302E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_xint_173, bid128_to_int32_xint, 0x302E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_xint_174, bid128_to_int32_xint, 0x303000000000000000000006FC23ABFFu128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_175, bid128_to_int32_xint, 0x303000000000000000000006FC23AC00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_176, bid128_to_int32_xint, 0x303000000000000000000006FC23AC01u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_177, bid128_to_int32_xint, 0x303200000000000000000000B2D05DFFu128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_178, bid128_to_int32_xint, 0x303200000000000000000000B2D05E00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_179, bid128_to_int32_xint, 0x303200000000000000000000B2D05E01u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_180, bid128_to_int32_xint, 0x303800000000000000000000002DDA47u128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xint_181, bid128_to_int32_xint, 0x303800000000000000000000002DDA48u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xint_182, bid128_to_int32_xint, 0x303800000000000000000000002DDA49u128, 300        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xint_183, bid128_to_int32_xint, 0x303A00000000000000000000000003E7u128, 0          , 0x20); // -- 0.999
dec_test!(bid128_to_int32_xint_184, bid128_to_int32_xint, 0x303A00000000000000000000000495D3u128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xint_185, bid128_to_int32_xint, 0x303A00000000000000000000000495D4u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xint_186, bid128_to_int32_xint, 0x303A00000000000000000000000495D5u128, 300        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xint_187, bid128_to_int32_xint, 0x303C0000000000000000000000007561u128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xint_188, bid128_to_int32_xint, 0x303C0000000000000000000000007562u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xint_189, bid128_to_int32_xint, 0x303C0000000000000000000000007563u128, 300        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xint_190, bid128_to_int32_xint, 0x303E0000000000000000000000000005u128, 0          , 0x20); // -- 0.5
dec_test!(bid128_to_int32_xint_191, bid128_to_int32_xint, 0x303E000000000000000000000000000Fu128, 1          , 0x20); // -- 1.5
dec_test!(bid128_to_int32_xint_192, bid128_to_int32_xint, 0x303E0000000000000000000000000BB7u128, 299        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xint_193, bid128_to_int32_xint, 0x303E0000000000000000000000000BB8u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_194, bid128_to_int32_xint, 0x303E0000000000000000000000000BB9u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xint_195, bid128_to_int32_xint, 0x303E0000000000000000000000000BBDu128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xint_196, bid128_to_int32_xint, 0x303E00000000000000000004FFFFFFF1u128, 2147483646 , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_int32_xint_197, bid128_to_int32_xint, 0x303E00000000000000000004FFFFFFFBu128, 2147483647 , 0x20); // -- 2^31-0.5
dec_test!(bid128_to_int32_xint_198, bid128_to_int32_xint, 0x303E0000000000000000000500000005u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_xint_199, bid128_to_int32_xint, 0x303E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_xint_200, bid128_to_int32_xint, 0x303E0000000000000000000A00000005u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_xint_201, bid128_to_int32_xint, 0x303E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_xint_202, bid128_to_int32_xint, 0x303E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_xint_203, bid128_to_int32_xint, 0x303E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_xint_204, bid128_to_int32_xint, 0x303E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_xint_205, bid128_to_int32_xint, 0x30400000000000000000000000000001u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_xint_206, bid128_to_int32_xint, 0x3040000000000000000000000000012Bu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_xint_207, bid128_to_int32_xint, 0x3040000000000000000000000000012Cu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_208, bid128_to_int32_xint, 0x3040000000000000000000000000012Du128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_xint_209, bid128_to_int32_xint, 0x3040000000000000000000007FFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_xint_210, bid128_to_int32_xint, 0x30400000000000000000000080000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_xint_211, bid128_to_int32_xint, 0x30400000000000000000000080000001u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_xint_212, bid128_to_int32_xint, 0x304000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_xint_213, bid128_to_int32_xint, 0x30400000000000000000000100000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_xint_214, bid128_to_int32_xint, 0x30400000000000000000000100000001u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_xint_215, bid128_to_int32_xint, 0x304000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_xint_216, bid128_to_int32_xint, 0x304000000000000000000004A817C801u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_xint_217, bid128_to_int32_xint, 0x3041ED09BEAD87C0378D8E6400000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_218, bid128_to_int32_xint, 0x3042000000000000000000000000001Du128, 290        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_xint_219, bid128_to_int32_xint, 0x3042000000000000000000000000001Eu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_220, bid128_to_int32_xint, 0x3042000000000000000000000000001Fu128, 310        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_xint_221, bid128_to_int32_xint, 0x304200000000000000000000773593FFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_xint_222, bid128_to_int32_xint, 0x30420000000000000000000077359400u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_xint_223, bid128_to_int32_xint, 0x30420000000000000000000077359401u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_xint_224, bid128_to_int32_xint, 0x30440000000000000000000000000003u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xint_225, bid128_to_int32_xint, 0x30520000000000000000000000000004u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_xint_226, bid128_to_int32_xint, 0x30520000000000000000000000000005u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_xint_227, bid128_to_int32_xint, 0x30540000000000000000000000000002u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_xint_228, bid128_to_int32_xint, 0x4dad0d604cfd12ecfebd876aad8c197au128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_229, bid128_to_int32_xint, 0x4e900cf74ee41b13d633d2bd650087c7u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_230, bid128_to_int32_xint, 0x4f260000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_231, bid128_to_int32_xint, "5.05"                                , 5          , 0x20);
dec_test!(bid128_to_int32_xint_232, bid128_to_int32_xint, 0x523498299d31d1f4ef0e9e8a082fb528u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_233, bid128_to_int32_xint, "5.5"                                 , 5          , 0x20);
dec_test!(bid128_to_int32_xint_234, bid128_to_int32_xint, "+57.86598966685658E0"                , 57         , 0x20);
dec_test!(bid128_to_int32_xint_235, bid128_to_int32_xint, "-59579.585E0"                        , -59579     , 0x20);
dec_test!(bid128_to_int32_xint_236, bid128_to_int32_xint, 0x5b7ef4916d264c183a8fedf23dc4512bu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_237, bid128_to_int32_xint, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_238, bid128_to_int32_xint, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_239, bid128_to_int32_xint, 0x799b353c7e733926efffffffefffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_240, bid128_to_int32_xint, 0x7c000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_241, bid128_to_int32_xint, 0x7c003fffffffffff38c15b08ffffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_242, bid128_to_int32_xint, 0x7c003fffffffffff38c15b0affffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_243, bid128_to_int32_xint, 0x7e000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_244, bid128_to_int32_xint, 0x844aefb46082e9bf143c8709d1949f63u128, 0          , 0x20);
dec_test!(bid128_to_int32_xint_245, bid128_to_int32_xint, 0x84f40000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_246, bid128_to_int32_xint, "+898889.988E0"                       , 898889     , 0x20);
dec_test!(bid128_to_int32_xint_247, bid128_to_int32_xint, 0x8a4f53d703ca99f3e723ef4d8dea58c7u128, 0          , 0x20);
dec_test!(bid128_to_int32_xint_248, bid128_to_int32_xint, 0x93b2e361542c3cf61c783c01767545a5u128, 0          , 0x20);
dec_test!(bid128_to_int32_xint_249, bid128_to_int32_xint, 0x96600000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_250, bid128_to_int32_xint, "-975.98E0"                           , -975       , 0x20);
dec_test!(bid128_to_int32_xint_251, bid128_to_int32_xint, "-9"                                  , -9         , 0x00);
dec_test!(bid128_to_int32_xint_252, bid128_to_int32_xint, 0xa00048040000a080d7da2a1f3efd5d0au128, 0          , 0x20);
dec_test!(bid128_to_int32_xint_253, bid128_to_int32_xint, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_int32_xint_254, bid128_to_int32_xint, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0          , 0x20); // -- -(0.5)
dec_test!(bid128_to_int32_xint_255, bid128_to_int32_xint, 0xAFFCF684DF56C3E01BC6C73200000001u128, 0          , 0x20); // -- -(0.5+ulp)
dec_test!(bid128_to_int32_xint_256, bid128_to_int32_xint, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 0          , 0x20); // -- -(0.999-ulp)
dec_test!(bid128_to_int32_xint_257, bid128_to_int32_xint, 0xAFFDEC8B86EF679D76FC433D80000000u128, 0          , 0x20); // -- -(0.999)
dec_test!(bid128_to_int32_xint_258, bid128_to_int32_xint, 0xAFFDEC8B86EF679D76FC433D80000001u128, 0          , 0x20); // -- -(0.999+ulp)
dec_test!(bid128_to_int32_xint_259, bid128_to_int32_xint, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 0          , 0x20); // -- -(1-ulp)
dec_test!(bid128_to_int32_xint_260, bid128_to_int32_xint, 0xAFFE314DC6448D9338C15B0A00000000u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_xint_261, bid128_to_int32_xint, 0xAFFE314DC6448D9338C15B0A00000001u128, -1         , 0x20); // -- -(1+ulp)
dec_test!(bid128_to_int32_xint_262, bid128_to_int32_xint, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1         , 0x20); // -- -(1.5-ulp)
dec_test!(bid128_to_int32_xint_263, bid128_to_int32_xint, 0xAFFE49F4A966D45CD522088F00000000u128, -1         , 0x20); // -- -(1.5)
dec_test!(bid128_to_int32_xint_264, bid128_to_int32_xint, 0xAFFE49F4A966D45CD522088F00000001u128, -1         , 0x20); // -- -(1.5+ulp)
dec_test!(bid128_to_int32_xint_265, bid128_to_int32_xint, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_266, bid128_to_int32_xint, 0xB00293E952CDA8B9AA44111E00000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_267, bid128_to_int32_xint, 0xB00293E952CDA8B9AA44111E00000001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_268, bid128_to_int32_xint, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xint_269, bid128_to_int32_xint, 0xB00294286EACB8CB0A8CB6B140000000u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xint_270, bid128_to_int32_xint, 0xB00294286EACB8CB0A8CB6B140000001u128, -300       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xint_271, bid128_to_int32_xint, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_272, bid128_to_int32_xint, 0xB0040ECA8847C4129106CE8300000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_273, bid128_to_int32_xint, 0xB0040ECA8847C4129106CE8300000001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_274, bid128_to_int32_xint, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_275, bid128_to_int32_xint, 0xB00A0003C95A2F0B4856475FE0000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_276, bid128_to_int32_xint, 0xB00A0003C95A2F0B4856475FE0000001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_277, bid128_to_int32_xint, 0xb00ba4ff9a60824df1ccfffa68ee3b6au128, -8538862   , 0x20);
dec_test!(bid128_to_int32_xint_278, bid128_to_int32_xint, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_279, bid128_to_int32_xint, 0xB00C000060EF6B1ABA6F072330000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_280, bid128_to_int32_xint, 0xB00C000060EF6B1ABA6F072330000001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_281, bid128_to_int32_xint, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, -2147483646, 0x20); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_xint_282, bid128_to_int32_xint, 0xB01069E10DE628D3A6C9CC9B8E800000u128, -2147483646, 0x20); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_xint_283, bid128_to_int32_xint, 0xB01069E10DE628D3A6C9CC9B8E800001u128, -2147483646, 0x20); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_xint_284, bid128_to_int32_xint, 0xB01069E10DE692B4B4B133125EFFFFFFu128, -2147483646, 0x20); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_xint_285, bid128_to_int32_xint, 0xB01069E10DE692B4B4B133125F000000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_xint_286, bid128_to_int32_xint, 0xB01069E10DE692B4B4B133125F000001u128, -2147483647, 0x20); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_xint_287, bid128_to_int32_xint, 0xB01069E10DE6FC95C29899892F7FFFFFu128, -2147483647, 0x20); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_xint_288, bid128_to_int32_xint, 0xB01069E10DE6FC95C29899892F800000u128, -2147483647, 0x20); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_xint_289, bid128_to_int32_xint, 0xB01069E10DE6FC95C29899892F800001u128, -2147483647, 0x20); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_xint_290, bid128_to_int32_xint, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, -2147483647, 0x20); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_xint_291, bid128_to_int32_xint, 0xB01069E10DE76676D080000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_xint_292, bid128_to_int32_xint, 0xB01069E10DE76676D080000000000001u128, -2147483648, 0x20); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_xint_293, bid128_to_int32_xint, 0xB01069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x20); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_xint_294, bid128_to_int32_xint, 0xB01069E10DE7D057DE676676D0800000u128, -2147483648, 0x20); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_xint_295, bid128_to_int32_xint, 0xB01069E10DE7D057DE676676D0800001u128, -2147483648, 0x20); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_xint_296, bid128_to_int32_xint, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x20); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_xint_297, bid128_to_int32_xint, 0xB01069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_xint_298, bid128_to_int32_xint, 0xB01069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_xint_299, bid128_to_int32_xint, 0xB010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_int32_xint_300, bid128_to_int32_xint, 0xB010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_xint_301, bid128_to_int32_xint, 0xB010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_int32_xint_302, bid128_to_int32_xint, 0xB010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_xint_303, bid128_to_int32_xint, 0xB010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_xint_304, bid128_to_int32_xint, 0xB010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_xint_305, bid128_to_int32_xint, 0xB010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_xint_306, bid128_to_int32_xint, 0xB010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_xint_307, bid128_to_int32_xint, 0xB010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_xint_308, bid128_to_int32_xint, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_xint_309, bid128_to_int32_xint, 0xB010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_xint_310, bid128_to_int32_xint, 0xB010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_xint_311, bid128_to_int32_xint, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_xint_312, bid128_to_int32_xint, 0xB010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_xint_313, bid128_to_int32_xint, 0xB010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_xint_314, bid128_to_int32_xint, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_xint_315, bid128_to_int32_xint, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_xint_316, bid128_to_int32_xint, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_xint_317, bid128_to_int32_xint, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_int32_xint_318, bid128_to_int32_xint, 0xB010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_xint_319, bid128_to_int32_xint, 0xB010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_int32_xint_320, bid128_to_int32_xint, 0xb011b60ac2888b0f7512d97a116a14bcu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_321, bid128_to_int32_xint, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_xint_322, bid128_to_int32_xint, 0xB012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_xint_323, bid128_to_int32_xint, 0xB012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_xint_324, bid128_to_int32_xint, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_xint_325, bid128_to_int32_xint, 0xB012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_xint_326, bid128_to_int32_xint, 0xB012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_xint_327, bid128_to_int32_xint, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_xint_328, bid128_to_int32_xint, 0xB012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_xint_329, bid128_to_int32_xint, 0xB012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_xint_330, bid128_to_int32_xint, 0xB012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_xint_331, bid128_to_int32_xint, 0xB012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_xint_332, bid128_to_int32_xint, 0xB012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_xint_333, bid128_to_int32_xint, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_xint_334, bid128_to_int32_xint, 0xB012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_xint_335, bid128_to_int32_xint, 0xB012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_xint_336, bid128_to_int32_xint, 0xB012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_xint_337, bid128_to_int32_xint, 0xB012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_xint_338, bid128_to_int32_xint, 0xB012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_xint_339, bid128_to_int32_xint, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_xint_340, bid128_to_int32_xint, 0xB012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_xint_341, bid128_to_int32_xint, 0xB012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_xint_342, bid128_to_int32_xint, 0xb014000001012884422480a081918148u128, -31088     , 0x20);
dec_test!(bid128_to_int32_xint_343, bid128_to_int32_xint, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, -2147483646, 0x20); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_xint_344, bid128_to_int32_xint, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, -2147483646, 0x20); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_xint_345, bid128_to_int32_xint, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, -2147483646, 0x20); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_xint_346, bid128_to_int32_xint, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, -2147483647, 0x20); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_xint_347, bid128_to_int32_xint, 0xB0180002B5E3AF13FBA450E94E780000u128, -2147483647, 0x20); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_xint_348, bid128_to_int32_xint, 0xB0180002B5E3AF13FBA450E94E780001u128, -2147483647, 0x20); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_xint_349, bid128_to_int32_xint, 0xB0180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x20); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_xint_350, bid128_to_int32_xint, 0xB0180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x20); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_xint_351, bid128_to_int32_xint, 0xB0180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x20); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_xint_352, bid128_to_int32_xint, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_xint_353, bid128_to_int32_xint, 0xB01800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_xint_354, bid128_to_int32_xint, 0xB01800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_xint_355, bid128_to_int32_xint, 0xB01800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_xint_356, bid128_to_int32_xint, 0xB01800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_xint_357, bid128_to_int32_xint, 0xB01800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_xint_358, bid128_to_int32_xint, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xint_359, bid128_to_int32_xint, 0xB01A0000000000A2E6C09AD3E0D40000u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xint_360, bid128_to_int32_xint, 0xB01A0000000000A2E6C09AD3E0D40001u128, -300       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xint_361, bid128_to_int32_xint, 0xB01A000045639181BA2CDCFB7617FFFFu128, -2147483646, 0x20); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_xint_362, bid128_to_int32_xint, 0xB01A000045639181BA2CDCFB76180000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_xint_363, bid128_to_int32_xint, 0xB01A000045639181BA2CDCFB76180001u128, -2147483647, 0x20); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_xint_364, bid128_to_int32_xint, 0xB01A00004563918244F3FFFFFFFFFFFFu128, -2147483647, 0x20); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_xint_365, bid128_to_int32_xint, 0xB01A00004563918244F4000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_xint_366, bid128_to_int32_xint, 0xB01A00004563918244F4000000000001u128, -2147483648, 0x20); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_xint_367, bid128_to_int32_xint, 0xB01A000045639182CFBB230489E7FFFFu128, -2147483648, 0x20); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_xint_368, bid128_to_int32_xint, 0xB01A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_xint_369, bid128_to_int32_xint, 0xB01A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_xint_370, bid128_to_int32_xint, 0xB01A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_xint_371, bid128_to_int32_xint, 0xB01A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_xint_372, bid128_to_int32_xint, 0xB01A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_xint_373, bid128_to_int32_xint, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_xint_374, bid128_to_int32_xint, 0xB01A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_xint_375, bid128_to_int32_xint, 0xB01A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_xint_376, bid128_to_int32_xint, 0xB01A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_xint_377, bid128_to_int32_xint, 0xB01A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_xint_378, bid128_to_int32_xint, 0xB01A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_xint_379, bid128_to_int32_xint, 0xb01a0000f0a086111150d1812085a000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_380, bid128_to_int32_xint, 0xB01E000000000001A055690D9DB7FFFFu128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_381, bid128_to_int32_xint, 0xB01E000000000001A055690D9DB80000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_382, bid128_to_int32_xint, 0xB01E000000000001A055690D9DB80001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_383, bid128_to_int32_xint, 0xB02000000000000029A2241AF62BFFFFu128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_384, bid128_to_int32_xint, 0xB02000000000000029A2241AF62C0000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_385, bid128_to_int32_xint, 0xB02000000000000029A2241AF62C0001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_386, bid128_to_int32_xint, 0xB024000000000000006A94D74F42FFFFu128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_387, bid128_to_int32_xint, 0xB024000000000000006A94D74F430000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_388, bid128_to_int32_xint, 0xB024000000000000006A94D74F430001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_389, bid128_to_int32_xint, 0xb024383c37e60fba624c8999ea86724du128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_390, bid128_to_int32_xint, 0xB02A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_xint_391, bid128_to_int32_xint, 0xB02A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_xint_392, bid128_to_int32_xint, 0xB02A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_xint_393, bid128_to_int32_xint, 0xB02A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_xint_394, bid128_to_int32_xint, 0xB02A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_xint_395, bid128_to_int32_xint, 0xB02A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_xint_396, bid128_to_int32_xint, 0xB02C000000000000000002BBA7F521FFu128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xint_397, bid128_to_int32_xint, 0xB02C000000000000000002BBA7F52200u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xint_398, bid128_to_int32_xint, 0xB02C000000000000000002BBA7F52201u128, -300       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xint_399, bid128_to_int32_xint, 0xB02C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_xint_400, bid128_to_int32_xint, 0xB02C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_xint_401, bid128_to_int32_xint, 0xB02C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_xint_402, bid128_to_int32_xint, 0xB02C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_xint_403, bid128_to_int32_xint, 0xB02C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_xint_404, bid128_to_int32_xint, 0xB02C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_xint_405, bid128_to_int32_xint, 0xB02C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_xint_406, bid128_to_int32_xint, 0xB02C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_xint_407, bid128_to_int32_xint, 0xB02C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_xint_408, bid128_to_int32_xint, 0xB02E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_xint_409, bid128_to_int32_xint, 0xB02E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_xint_410, bid128_to_int32_xint, 0xB02E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_xint_411, bid128_to_int32_xint, 0xB03000000000000000000006FC23ABFFu128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_412, bid128_to_int32_xint, 0xB03000000000000000000006FC23AC00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_413, bid128_to_int32_xint, 0xB03000000000000000000006FC23AC01u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_414, bid128_to_int32_xint, 0xB03200000000000000000000B2D05DFFu128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_415, bid128_to_int32_xint, 0xB03200000000000000000000B2D05E00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_416, bid128_to_int32_xint, 0xB03200000000000000000000B2D05E01u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_417, bid128_to_int32_xint, 0xB03800000000000000000000002DDA47u128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xint_418, bid128_to_int32_xint, 0xB03800000000000000000000002DDA48u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xint_419, bid128_to_int32_xint, 0xB03800000000000000000000002DDA49u128, -300       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xint_420, bid128_to_int32_xint, 0xB03A00000000000000000000000003E7u128, 0          , 0x20); // -- -(0.999)
dec_test!(bid128_to_int32_xint_421, bid128_to_int32_xint, 0xB03A00000000000000000000000495D3u128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xint_422, bid128_to_int32_xint, 0xB03A00000000000000000000000495D4u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xint_423, bid128_to_int32_xint, 0xB03A00000000000000000000000495D5u128, -300       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xint_424, bid128_to_int32_xint, 0xB03C0000000000000000000000007561u128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xint_425, bid128_to_int32_xint, 0xB03C0000000000000000000000007562u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xint_426, bid128_to_int32_xint, 0xB03C0000000000000000000000007563u128, -300       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xint_427, bid128_to_int32_xint, 0xB03E0000000000000000000000000005u128, 0          , 0x20); // -- -(0.5)
dec_test!(bid128_to_int32_xint_428, bid128_to_int32_xint, 0xB03E000000000000000000000000000Fu128, -1         , 0x20); // -- -(1.5)
dec_test!(bid128_to_int32_xint_429, bid128_to_int32_xint, 0xB03E0000000000000000000000000BB7u128, -299       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_430, bid128_to_int32_xint, 0xB03E0000000000000000000000000BB8u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_431, bid128_to_int32_xint, 0xB03E0000000000000000000000000BB9u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_432, bid128_to_int32_xint, 0xB03E0000000000000000000000000BBDu128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xint_433, bid128_to_int32_xint, 0xB03E00000000000000000004FFFFFFF1u128, -2147483646, 0x20); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_xint_434, bid128_to_int32_xint, 0xB03E00000000000000000004FFFFFFFBu128, -2147483647, 0x20); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_xint_435, bid128_to_int32_xint, 0xB03E0000000000000000000500000005u128, -2147483648, 0x20); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_xint_436, bid128_to_int32_xint, 0xB03E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_xint_437, bid128_to_int32_xint, 0xB03E0000000000000000000A00000005u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_xint_438, bid128_to_int32_xint, 0xB03E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_xint_439, bid128_to_int32_xint, 0xB03E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_xint_440, bid128_to_int32_xint, 0xB03E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_xint_441, bid128_to_int32_xint, 0xB03E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_xint_442, bid128_to_int32_xint, 0xB0400000000000000000000000000001u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_xint_443, bid128_to_int32_xint, 0xB040000000000000000000000000012Bu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_444, bid128_to_int32_xint, 0xB040000000000000000000000000012Cu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_445, bid128_to_int32_xint, 0xB040000000000000000000000000012Du128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_446, bid128_to_int32_xint, 0xB040000000000000000000007FFFFFFFu128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_xint_447, bid128_to_int32_xint, 0xB0400000000000000000000080000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_xint_448, bid128_to_int32_xint, 0xB0400000000000000000000080000001u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_xint_449, bid128_to_int32_xint, 0xB04000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_xint_450, bid128_to_int32_xint, 0xB0400000000000000000000100000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_xint_451, bid128_to_int32_xint, 0xB0400000000000000000000100000001u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_xint_452, bid128_to_int32_xint, 0xB04000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_xint_453, bid128_to_int32_xint, 0xB04000000000000000000004A817C801u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_xint_454, bid128_to_int32_xint, 0xB042000000000000000000000000001Du128, -290       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_xint_455, bid128_to_int32_xint, 0xB042000000000000000000000000001Eu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_456, bid128_to_int32_xint, 0xB042000000000000000000000000001Fu128, -310       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_xint_457, bid128_to_int32_xint, 0xB04200000000000000000000773593FFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_xint_458, bid128_to_int32_xint, 0xB0420000000000000000000077359400u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_xint_459, bid128_to_int32_xint, 0xB0420000000000000000000077359401u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_xint_460, bid128_to_int32_xint, 0xB0440000000000000000000000000003u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xint_461, bid128_to_int32_xint, 0xB0520000000000000000000000000004u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_xint_462, bid128_to_int32_xint, 0xB0520000000000000000000000000005u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_xint_463, bid128_to_int32_xint, 0xB0540000000000000000000000000002u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_xint_464, bid128_to_int32_xint, 0xbafa3f2e6cfac5e9dd6a483d40cc1e81u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_465, bid128_to_int32_xint, 0xbbf766a597cc86c6b72e9b02961ff2c5u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_466, bid128_to_int32_xint, 0xbe5b384b10b3bfc5a94503cc94d8d490u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_467, bid128_to_int32_xint, 0xc70c36015aac1d9a129de484f80bc5d2u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_468, bid128_to_int32_xint, 0xd1d1bc6c97272cf5e0f9f474b0089ae7u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_469, bid128_to_int32_xint, 0xd82c0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_470, bid128_to_int32_xint, 0xdaca0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_471, bid128_to_int32_xint, 0xdcbe215c200c4a3aefc64d5f041d441du128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_472, bid128_to_int32_xint, 0xf7f4f639fbf4edef2398101f69432143u128, 0          , 0x00);
dec_test!(bid128_to_int32_xint_473, bid128_to_int32_xint, 0xf8dadcdd9d42b5ba00000c0010042000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_474, bid128_to_int32_xint, 0xfdffffffffffefffa0244a115d959211u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_475, bid128_to_int32_xint, 0xfffdeafefbffbffd0000000200800022u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_476, bid128_to_int32_xint, "-Infinity"                           , -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_477, bid128_to_int32_xint, "Infinity"                            , -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_478, bid128_to_int32_xint, "QNaN"                                , -2147483648, 0x01);
dec_test!(bid128_to_int32_xint_479, bid128_to_int32_xint, "SNaN"                                , -2147483648, 0x01);
