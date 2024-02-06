/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int32_rninta_001, bid128_to_int32_rninta, "-0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_rninta_002, bid128_to_int32_rninta,  "0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_rninta_003, bid128_to_int32_rninta, 0x00000000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_004, bid128_to_int32_rninta, 0x0000000000000000ffffffffffffffbfu128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_005, bid128_to_int32_rninta, 0x0001ed09bead87c0378d8e62ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_006, bid128_to_int32_rninta, 0x0001ed09bead87c0378d8e64ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_007, bid128_to_int32_rninta, 0x00020000000000000008000000005000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_008, bid128_to_int32_rninta, "0.1"                                 , 0          , 0x00);
dec_test!(bid128_to_int32_rninta_009, bid128_to_int32_rninta, "0.5"                                 , 1          , 0x00);
dec_test!(bid128_to_int32_rninta_010, bid128_to_int32_rninta, "1"                                   , 1          , 0x00);
dec_test!(bid128_to_int32_rninta_011, bid128_to_int32_rninta, 0x1ae2de17eb17720e0d92afec340d6f83u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_012, bid128_to_int32_rninta, 0x1ee40000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_013, bid128_to_int32_rninta, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_int32_rninta_014, bid128_to_int32_rninta, 0x2FFCF684DF56C3E01BC6C73200000000u128, 1          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_rninta_015, bid128_to_int32_rninta, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1          , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_int32_rninta_016, bid128_to_int32_rninta, 0x2ffddfaffbdf9dff0020000000000540u128, 1          , 0x00);
dec_test!(bid128_to_int32_rninta_017, bid128_to_int32_rninta, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1          , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_int32_rninta_018, bid128_to_int32_rninta, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_rninta_019, bid128_to_int32_rninta, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1          , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_int32_rninta_020, bid128_to_int32_rninta, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1          , 0x00); // -- 1-ulp
dec_test!(bid128_to_int32_rninta_021, bid128_to_int32_rninta, 0x2FFE314DC6448D9338C15B0A00000000u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_rninta_022, bid128_to_int32_rninta, 0x2FFE314DC6448D9338C15B0A00000001u128, 1          , 0x00); // -- 1+ulp
dec_test!(bid128_to_int32_rninta_023, bid128_to_int32_rninta, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1          , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_int32_rninta_024, bid128_to_int32_rninta, 0x2FFE49F4A966D45CD522088F00000000u128, 2          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_rninta_025, bid128_to_int32_rninta, 0x2FFE49F4A966D45CD522088F00000001u128, 2          , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_int32_rninta_026, bid128_to_int32_rninta, 0x30000148008540206ccaf9ed512d36e8u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_027, bid128_to_int32_rninta, 0x300010572a6100038200000000002180u128, 3          , 0x00);
dec_test!(bid128_to_int32_rninta_028, bid128_to_int32_rninta, 0x30010081021120a00400000000000004u128, 52         , 0x00);
dec_test!(bid128_to_int32_rninta_029, bid128_to_int32_rninta, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_030, bid128_to_int32_rninta, 0x300293E952CDA8B9AA44111E00000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_031, bid128_to_int32_rninta, 0x300293E952CDA8B9AA44111E00000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_032, bid128_to_int32_rninta, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rninta_033, bid128_to_int32_rninta, 0x300294286EACB8CB0A8CB6B140000000u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rninta_034, bid128_to_int32_rninta, 0x300294286EACB8CB0A8CB6B140000001u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rninta_035, bid128_to_int32_rninta, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_036, bid128_to_int32_rninta, 0x30040ECA8847C4129106CE8300000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_037, bid128_to_int32_rninta, 0x30040ECA8847C4129106CE8300000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_038, bid128_to_int32_rninta, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_039, bid128_to_int32_rninta, 0x300A0003C95A2F0B4856475FE0000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_040, bid128_to_int32_rninta, 0x300A0003C95A2F0B4856475FE0000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_041, bid128_to_int32_rninta, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_042, bid128_to_int32_rninta, 0x300C000060EF6B1ABA6F072330000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_043, bid128_to_int32_rninta, 0x300C000060EF6B1ABA6F072330000001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_044, bid128_to_int32_rninta, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_rninta_045, bid128_to_int32_rninta, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483647 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_rninta_046, bid128_to_int32_rninta, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483647 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_rninta_047, bid128_to_int32_rninta, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_rninta_048, bid128_to_int32_rninta, 0x301069E10DE692B4B4B133125F000000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_rninta_049, bid128_to_int32_rninta, 0x301069E10DE692B4B4B133125F000001u128, 2147483647 , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_rninta_050, bid128_to_int32_rninta, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647 , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_rninta_051, bid128_to_int32_rninta, 0x301069E10DE6FC95C29899892F800000u128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_rninta_052, bid128_to_int32_rninta, 0x301069E10DE6FC95C29899892F800001u128, -2147483648, 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_rninta_053, bid128_to_int32_rninta, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_rninta_054, bid128_to_int32_rninta, 0x301069E10DE76676D080000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_rninta_055, bid128_to_int32_rninta, 0x301069E10DE76676D080000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_rninta_056, bid128_to_int32_rninta, 0x301069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_rninta_057, bid128_to_int32_rninta, 0x301069E10DE7D057DE676676D0800000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_rninta_058, bid128_to_int32_rninta, 0x301069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_rninta_059, bid128_to_int32_rninta, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_rninta_060, bid128_to_int32_rninta, 0x301069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_rninta_061, bid128_to_int32_rninta, 0x301069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_rninta_062, bid128_to_int32_rninta, 0x3010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- 4e9-ulp
dec_test!(bid128_to_int32_rninta_063, bid128_to_int32_rninta, 0x3010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_rninta_064, bid128_to_int32_rninta, 0x3010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- 4e9+ulp
dec_test!(bid128_to_int32_rninta_065, bid128_to_int32_rninta, 0x3010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_rninta_066, bid128_to_int32_rninta, 0x3010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_rninta_067, bid128_to_int32_rninta, 0x3010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_rninta_068, bid128_to_int32_rninta, 0x3010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_rninta_069, bid128_to_int32_rninta, 0x3010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_rninta_070, bid128_to_int32_rninta, 0x3010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_rninta_071, bid128_to_int32_rninta, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_rninta_072, bid128_to_int32_rninta, 0x3010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_rninta_073, bid128_to_int32_rninta, 0x3010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_rninta_074, bid128_to_int32_rninta, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_rninta_075, bid128_to_int32_rninta, 0x3010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_rninta_076, bid128_to_int32_rninta, 0x3010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_rninta_077, bid128_to_int32_rninta, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_rninta_078, bid128_to_int32_rninta, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_rninta_079, bid128_to_int32_rninta, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_rninta_080, bid128_to_int32_rninta, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- 5e9-ulp
dec_test!(bid128_to_int32_rninta_081, bid128_to_int32_rninta, 0x3010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_rninta_082, bid128_to_int32_rninta, 0x3010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- 5e9+ulp
dec_test!(bid128_to_int32_rninta_083, bid128_to_int32_rninta, 0x3012228e3416501458769c6c977989f7u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_084, bid128_to_int32_rninta, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_rninta_085, bid128_to_int32_rninta, 0x3012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_rninta_086, bid128_to_int32_rninta, 0x3012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_rninta_087, bid128_to_int32_rninta, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_rninta_088, bid128_to_int32_rninta, 0x3012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_rninta_089, bid128_to_int32_rninta, 0x3012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_rninta_090, bid128_to_int32_rninta, 0x3012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_rninta_091, bid128_to_int32_rninta, 0x3012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_rninta_092, bid128_to_int32_rninta, 0x3012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_rninta_093, bid128_to_int32_rninta, 0x3012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_rninta_094, bid128_to_int32_rninta, 0x3012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_rninta_095, bid128_to_int32_rninta, 0x3012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_rninta_096, bid128_to_int32_rninta, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_rninta_097, bid128_to_int32_rninta, 0x3012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_rninta_098, bid128_to_int32_rninta, 0x3012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_rninta_099, bid128_to_int32_rninta, 0x3012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_rninta_100, bid128_to_int32_rninta, 0x3012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_rninta_101, bid128_to_int32_rninta, 0x3012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_rninta_102, bid128_to_int32_rninta, 0x3012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_rninta_103, bid128_to_int32_rninta, 0x3012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_rninta_104, bid128_to_int32_rninta, 0x3012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_rninta_105, bid128_to_int32_rninta, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646 , 0x00); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_rninta_106, bid128_to_int32_rninta, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483647 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_rninta_107, bid128_to_int32_rninta, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483647 , 0x00); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_rninta_108, bid128_to_int32_rninta, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647 , 0x00); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_rninta_109, bid128_to_int32_rninta, 0x30180002B5E3AF13FBA450E94E780000u128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_rninta_110, bid128_to_int32_rninta, 0x30180002B5E3AF13FBA450E94E780001u128, -2147483648, 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_rninta_111, bid128_to_int32_rninta, 0x30180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_rninta_112, bid128_to_int32_rninta, 0x30180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_rninta_113, bid128_to_int32_rninta, 0x30180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_rninta_114, bid128_to_int32_rninta, 0x301800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_rninta_115, bid128_to_int32_rninta, 0x301800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_rninta_116, bid128_to_int32_rninta, 0x301800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_rninta_117, bid128_to_int32_rninta, 0x301800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_rninta_118, bid128_to_int32_rninta, 0x301800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_rninta_119, bid128_to_int32_rninta, 0x301800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_rninta_120, bid128_to_int32_rninta, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rninta_121, bid128_to_int32_rninta, 0x301A0000000000A2E6C09AD3E0D40000u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rninta_122, bid128_to_int32_rninta, 0x301A0000000000A2E6C09AD3E0D40001u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rninta_123, bid128_to_int32_rninta, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483647 , 0x00); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_rninta_124, bid128_to_int32_rninta, 0x301A000045639181BA2CDCFB76180000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_rninta_125, bid128_to_int32_rninta, 0x301A000045639181BA2CDCFB76180001u128, 2147483647 , 0x00); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_rninta_126, bid128_to_int32_rninta, 0x301A00004563918244F3FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_rninta_127, bid128_to_int32_rninta, 0x301A00004563918244F4000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_rninta_128, bid128_to_int32_rninta, 0x301A00004563918244F4000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_rninta_129, bid128_to_int32_rninta, 0x301A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_rninta_130, bid128_to_int32_rninta, 0x301A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_rninta_131, bid128_to_int32_rninta, 0x301A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_rninta_132, bid128_to_int32_rninta, 0x301A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_rninta_133, bid128_to_int32_rninta, 0x301A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_rninta_134, bid128_to_int32_rninta, 0x301A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_rninta_135, bid128_to_int32_rninta, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_rninta_136, bid128_to_int32_rninta, 0x301A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_rninta_137, bid128_to_int32_rninta, 0x301A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_rninta_138, bid128_to_int32_rninta, 0x301A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_rninta_139, bid128_to_int32_rninta, 0x301A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_rninta_140, bid128_to_int32_rninta, 0x301A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_rninta_141, bid128_to_int32_rninta, 0x301c000002208c60dffbf7f7d3ffffffu128, 658318564  , 0x00);
dec_test!(bid128_to_int32_rninta_142, bid128_to_int32_rninta, 0x301E000000000001A055690D9DB7FFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_143, bid128_to_int32_rninta, 0x301E000000000001A055690D9DB80000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_144, bid128_to_int32_rninta, 0x301E000000000001A055690D9DB80001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_145, bid128_to_int32_rninta, 0x302000000000000029A2241AF62BFFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_146, bid128_to_int32_rninta, 0x302000000000000029A2241AF62C0000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_147, bid128_to_int32_rninta, 0x302000000000000029A2241AF62C0001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_148, bid128_to_int32_rninta, 0x3020000000500200480ff80130f61059u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_149, bid128_to_int32_rninta, 0x3024000000000000006A94D74F42FFFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_150, bid128_to_int32_rninta, 0x3024000000000000006A94D74F430000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_151, bid128_to_int32_rninta, 0x3024000000000000006A94D74F430001u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_152, bid128_to_int32_rninta, 0x302A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_rninta_153, bid128_to_int32_rninta, 0x302A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_rninta_154, bid128_to_int32_rninta, 0x302A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_rninta_155, bid128_to_int32_rninta, 0x302A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_rninta_156, bid128_to_int32_rninta, 0x302A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_rninta_157, bid128_to_int32_rninta, 0x302A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_rninta_158, bid128_to_int32_rninta, 0x302C000000000000000002BBA7F521FFu128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rninta_159, bid128_to_int32_rninta, 0x302C000000000000000002BBA7F52200u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rninta_160, bid128_to_int32_rninta, 0x302C000000000000000002BBA7F52201u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rninta_161, bid128_to_int32_rninta, 0x302C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_rninta_162, bid128_to_int32_rninta, 0x302C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_rninta_163, bid128_to_int32_rninta, 0x302C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_rninta_164, bid128_to_int32_rninta, 0x302C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_rninta_165, bid128_to_int32_rninta, 0x302C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_rninta_166, bid128_to_int32_rninta, 0x302C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_rninta_167, bid128_to_int32_rninta, 0x302C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_rninta_168, bid128_to_int32_rninta, 0x302C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_rninta_169, bid128_to_int32_rninta, 0x302C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_rninta_170, bid128_to_int32_rninta, 0x302E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_rninta_171, bid128_to_int32_rninta, 0x302E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_rninta_172, bid128_to_int32_rninta, 0x302E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_rninta_173, bid128_to_int32_rninta, 0x303000000000000000000006FC23ABFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_174, bid128_to_int32_rninta, 0x303000000000000000000006FC23AC00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_175, bid128_to_int32_rninta, 0x303000000000000000000006FC23AC01u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_176, bid128_to_int32_rninta, 0x303200000000000000000000B2D05DFFu128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_177, bid128_to_int32_rninta, 0x303200000000000000000000B2D05E00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_178, bid128_to_int32_rninta, 0x303200000000000000000000B2D05E01u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_179, bid128_to_int32_rninta, 0x303800000000000000000000002DDA47u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rninta_180, bid128_to_int32_rninta, 0x303800000000000000000000002DDA48u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rninta_181, bid128_to_int32_rninta, 0x303800000000000000000000002DDA49u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rninta_182, bid128_to_int32_rninta, 0x303A00000000000000000000000003E7u128, 1          , 0x00); // -- 0.999
dec_test!(bid128_to_int32_rninta_183, bid128_to_int32_rninta, 0x303A00000000000000000000000495D3u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rninta_184, bid128_to_int32_rninta, 0x303A00000000000000000000000495D4u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rninta_185, bid128_to_int32_rninta, 0x303A00000000000000000000000495D5u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rninta_186, bid128_to_int32_rninta, 0x303C0000000000000000000000007561u128, 300        , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int32_rninta_187, bid128_to_int32_rninta, 0x303C0000000000000000000000007562u128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rninta_188, bid128_to_int32_rninta, 0x303C0000000000000000000000007563u128, 301        , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int32_rninta_189, bid128_to_int32_rninta, 0x303E0000000000000000000000000005u128, 1          , 0x00); // -- 0.5
dec_test!(bid128_to_int32_rninta_190, bid128_to_int32_rninta, 0x303E000000000000000000000000000Fu128, 2          , 0x00); // -- 1.5
dec_test!(bid128_to_int32_rninta_191, bid128_to_int32_rninta, 0x303E0000000000000000000000000BB7u128, 300        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_192, bid128_to_int32_rninta, 0x303E0000000000000000000000000BB8u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_193, bid128_to_int32_rninta, 0x303E0000000000000000000000000BB9u128, 300        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_194, bid128_to_int32_rninta, 0x303E0000000000000000000000000BBDu128, 301        , 0x00); // -- 300.5
dec_test!(bid128_to_int32_rninta_195, bid128_to_int32_rninta, 0x303E00000000000000000004FFFFFFF1u128, 2147483647 , 0x00); // -- 2^31-1.5
dec_test!(bid128_to_int32_rninta_196, bid128_to_int32_rninta, 0x303E00000000000000000004FFFFFFFBu128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_rninta_197, bid128_to_int32_rninta, 0x303E0000000000000000000500000005u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_rninta_198, bid128_to_int32_rninta, 0x303E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_rninta_199, bid128_to_int32_rninta, 0x303E0000000000000000000A00000005u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_rninta_200, bid128_to_int32_rninta, 0x303E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_rninta_201, bid128_to_int32_rninta, 0x303E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_rninta_202, bid128_to_int32_rninta, 0x303E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_rninta_203, bid128_to_int32_rninta, 0x303E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_rninta_204, bid128_to_int32_rninta, 0x30400000000000000000000000000001u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_rninta_205, bid128_to_int32_rninta, 0x3040000000000000000000000000012Bu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_206, bid128_to_int32_rninta, 0x3040000000000000000000000000012Cu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_207, bid128_to_int32_rninta, 0x3040000000000000000000000000012Du128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_208, bid128_to_int32_rninta, 0x3040000000000000000000007FFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_rninta_209, bid128_to_int32_rninta, 0x30400000000000000000000080000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_rninta_210, bid128_to_int32_rninta, 0x30400000000000000000000080000001u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_rninta_211, bid128_to_int32_rninta, 0x30400000000000000000000080000100u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_212, bid128_to_int32_rninta, 0x304000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_rninta_213, bid128_to_int32_rninta, 0x30400000000000000000000100000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_rninta_214, bid128_to_int32_rninta, 0x30400000000000000000000100000001u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_rninta_215, bid128_to_int32_rninta, 0x304000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_rninta_216, bid128_to_int32_rninta, 0x304000000000000000000004A817C801u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_rninta_217, bid128_to_int32_rninta, 0x3041ED09BEAD87C0378D8E6400000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_218, bid128_to_int32_rninta, 0x3042000000000000000000000000001Du128, 290        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_rninta_219, bid128_to_int32_rninta, 0x3042000000000000000000000000001Eu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_220, bid128_to_int32_rninta, 0x3042000000000000000000000000001Fu128, 310        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_rninta_221, bid128_to_int32_rninta, 0x304200000000000000000000773593FFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_rninta_222, bid128_to_int32_rninta, 0x30420000000000000000000077359400u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_rninta_223, bid128_to_int32_rninta, 0x30420000000000000000000077359401u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_rninta_224, bid128_to_int32_rninta, 0x30440000000000000000000000000003u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_rninta_225, bid128_to_int32_rninta, 0x30520000000000000000000000000004u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_rninta_226, bid128_to_int32_rninta, 0x30520000000000000000000000000005u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_rninta_227, bid128_to_int32_rninta, 0x30540000000000000000000000000002u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_rninta_228, bid128_to_int32_rninta, 0x55b442b6b3d9863cfe1df6a10f6ec188u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_229, bid128_to_int32_rninta, 0x5e27675a09185309fcc433f3b407a0afu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_230, bid128_to_int32_rninta, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_231, bid128_to_int32_rninta, 0x7a3f85a4d81a9792afb73b7dbdbd163au128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_232, bid128_to_int32_rninta, 0x7c000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_233, bid128_to_int32_rninta, 0x7c000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_234, bid128_to_int32_rninta, 0x7c003fffffffffff38c15b08ffffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_235, bid128_to_int32_rninta, 0x7c003fffffffffff38c15b0affffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_236, bid128_to_int32_rninta, 0x7c06bd08350a7445d2d21d1976e263d3u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_237, bid128_to_int32_rninta, 0x7e000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_238, bid128_to_int32_rninta, 0x80415215ef760404c3d21c0317be99d2u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_239, bid128_to_int32_rninta, 0x85140000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_240, bid128_to_int32_rninta, 0x949b6e50aa8e8f5cc4ec8be0d9828800u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_241, bid128_to_int32_rninta, "-9"                                  , -9         , 0x00);
dec_test!(bid128_to_int32_rninta_242, bid128_to_int32_rninta, 0xa3600000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_243, bid128_to_int32_rninta, 0xab211252b59b4156047c695a9596b259u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_244, bid128_to_int32_rninta, 0xade6ec3325eb11d1fb2ae478af3408e3u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_245, bid128_to_int32_rninta, 0xafcc0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_246, bid128_to_int32_rninta, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_int32_rninta_247, bid128_to_int32_rninta, 0xAFFCF684DF56C3E01BC6C73200000000u128, -1         , 0x00); // -- -(0.5)
dec_test!(bid128_to_int32_rninta_248, bid128_to_int32_rninta, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1         , 0x00); // -- -(0.5+ulp)
dec_test!(bid128_to_int32_rninta_249, bid128_to_int32_rninta, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1         , 0x00); // -- -(0.999-ulp)
dec_test!(bid128_to_int32_rninta_250, bid128_to_int32_rninta, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1         , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_rninta_251, bid128_to_int32_rninta, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1         , 0x00); // -- -(0.999+ulp)
dec_test!(bid128_to_int32_rninta_252, bid128_to_int32_rninta, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1         , 0x00); // -- -(1-ulp)
dec_test!(bid128_to_int32_rninta_253, bid128_to_int32_rninta, 0xAFFE314DC6448D9338C15B0A00000000u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_rninta_254, bid128_to_int32_rninta, 0xAFFE314DC6448D9338C15B0A00000001u128, -1         , 0x00); // -- -(1+ulp)
dec_test!(bid128_to_int32_rninta_255, bid128_to_int32_rninta, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1         , 0x00); // -- -(1.5-ulp)
dec_test!(bid128_to_int32_rninta_256, bid128_to_int32_rninta, 0xAFFE49F4A966D45CD522088F00000000u128, -2         , 0x00); // -- -(1.5)
dec_test!(bid128_to_int32_rninta_257, bid128_to_int32_rninta, 0xAFFE49F4A966D45CD522088F00000001u128, -2         , 0x00); // -- -(1.5+ulp)
dec_test!(bid128_to_int32_rninta_258, bid128_to_int32_rninta, 0xb000044010821280c008006062003000u128, -1         , 0x00);
dec_test!(bid128_to_int32_rninta_259, bid128_to_int32_rninta, 0xb001c7ff702422b8938108bc1cb4a6d9u128, -92        , 0x00);
dec_test!(bid128_to_int32_rninta_260, bid128_to_int32_rninta, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_261, bid128_to_int32_rninta, 0xB00293E952CDA8B9AA44111E00000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_262, bid128_to_int32_rninta, 0xB00293E952CDA8B9AA44111E00000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_263, bid128_to_int32_rninta, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rninta_264, bid128_to_int32_rninta, 0xB00294286EACB8CB0A8CB6B140000000u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rninta_265, bid128_to_int32_rninta, 0xB00294286EACB8CB0A8CB6B140000001u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rninta_266, bid128_to_int32_rninta, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_267, bid128_to_int32_rninta, 0xB0040ECA8847C4129106CE8300000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_268, bid128_to_int32_rninta, 0xB0040ECA8847C4129106CE8300000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_269, bid128_to_int32_rninta, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_270, bid128_to_int32_rninta, 0xB00A0003C95A2F0B4856475FE0000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_271, bid128_to_int32_rninta, 0xB00A0003C95A2F0B4856475FE0000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_272, bid128_to_int32_rninta, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_273, bid128_to_int32_rninta, 0xB00C000060EF6B1ABA6F072330000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_274, bid128_to_int32_rninta, 0xB00C000060EF6B1ABA6F072330000001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_275, bid128_to_int32_rninta, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, -2147483646, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_rninta_276, bid128_to_int32_rninta, 0xB01069E10DE628D3A6C9CC9B8E800000u128, -2147483647, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_rninta_277, bid128_to_int32_rninta, 0xB01069E10DE628D3A6C9CC9B8E800001u128, -2147483647, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_rninta_278, bid128_to_int32_rninta, 0xB01069E10DE692B4B4B133125EFFFFFFu128, -2147483647, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_rninta_279, bid128_to_int32_rninta, 0xB01069E10DE692B4B4B133125F000000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_rninta_280, bid128_to_int32_rninta, 0xB01069E10DE692B4B4B133125F000001u128, -2147483647, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_rninta_281, bid128_to_int32_rninta, 0xB01069E10DE6FC95C29899892F7FFFFFu128, -2147483647, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_rninta_282, bid128_to_int32_rninta, 0xB01069E10DE6FC95C29899892F800000u128, -2147483648, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_rninta_283, bid128_to_int32_rninta, 0xB01069E10DE6FC95C29899892F800001u128, -2147483648, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_rninta_284, bid128_to_int32_rninta, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, -2147483648, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_rninta_285, bid128_to_int32_rninta, 0xB01069E10DE76676D080000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_rninta_286, bid128_to_int32_rninta, 0xB01069E10DE76676D080000000000001u128, -2147483648, 0x00); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_rninta_287, bid128_to_int32_rninta, 0xB01069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x00); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_rninta_288, bid128_to_int32_rninta, 0xB01069E10DE7D057DE676676D0800000u128, -2147483648, 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_rninta_289, bid128_to_int32_rninta, 0xB01069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_rninta_290, bid128_to_int32_rninta, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_rninta_291, bid128_to_int32_rninta, 0xB01069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_rninta_292, bid128_to_int32_rninta, 0xB01069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_rninta_293, bid128_to_int32_rninta, 0xB010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_int32_rninta_294, bid128_to_int32_rninta, 0xB010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_rninta_295, bid128_to_int32_rninta, 0xB010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_int32_rninta_296, bid128_to_int32_rninta, 0xB010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_rninta_297, bid128_to_int32_rninta, 0xB010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_rninta_298, bid128_to_int32_rninta, 0xB010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_rninta_299, bid128_to_int32_rninta, 0xB010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_rninta_300, bid128_to_int32_rninta, 0xB010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_rninta_301, bid128_to_int32_rninta, 0xB010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_rninta_302, bid128_to_int32_rninta, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_rninta_303, bid128_to_int32_rninta, 0xB010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_rninta_304, bid128_to_int32_rninta, 0xB010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_rninta_305, bid128_to_int32_rninta, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_rninta_306, bid128_to_int32_rninta, 0xB010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_rninta_307, bid128_to_int32_rninta, 0xB010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_rninta_308, bid128_to_int32_rninta, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_rninta_309, bid128_to_int32_rninta, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_rninta_310, bid128_to_int32_rninta, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_rninta_311, bid128_to_int32_rninta, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_int32_rninta_312, bid128_to_int32_rninta, 0xB010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_rninta_313, bid128_to_int32_rninta, 0xB010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_int32_rninta_314, bid128_to_int32_rninta, 0xb011dc54498405287f3da05c70c3064au128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_315, bid128_to_int32_rninta, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_rninta_316, bid128_to_int32_rninta, 0xB012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_rninta_317, bid128_to_int32_rninta, 0xB012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_rninta_318, bid128_to_int32_rninta, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_rninta_319, bid128_to_int32_rninta, 0xB012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_rninta_320, bid128_to_int32_rninta, 0xB012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_rninta_321, bid128_to_int32_rninta, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_rninta_322, bid128_to_int32_rninta, 0xB012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_rninta_323, bid128_to_int32_rninta, 0xB012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_rninta_324, bid128_to_int32_rninta, 0xB012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_rninta_325, bid128_to_int32_rninta, 0xB012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_rninta_326, bid128_to_int32_rninta, 0xB012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_rninta_327, bid128_to_int32_rninta, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_rninta_328, bid128_to_int32_rninta, 0xB012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_rninta_329, bid128_to_int32_rninta, 0xB012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_rninta_330, bid128_to_int32_rninta, 0xB012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_rninta_331, bid128_to_int32_rninta, 0xB012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_rninta_332, bid128_to_int32_rninta, 0xB012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_rninta_333, bid128_to_int32_rninta, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_rninta_334, bid128_to_int32_rninta, 0xB012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_rninta_335, bid128_to_int32_rninta, 0xB012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_rninta_336, bid128_to_int32_rninta, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, -2147483646, 0x00); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_rninta_337, bid128_to_int32_rninta, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, -2147483647, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_rninta_338, bid128_to_int32_rninta, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, -2147483647, 0x00); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_rninta_339, bid128_to_int32_rninta, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, -2147483647, 0x00); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_rninta_340, bid128_to_int32_rninta, 0xB0180002B5E3AF13FBA450E94E780000u128, -2147483648, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_rninta_341, bid128_to_int32_rninta, 0xB0180002B5E3AF13FBA450E94E780001u128, -2147483648, 0x00); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_rninta_342, bid128_to_int32_rninta, 0xB0180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x00); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_rninta_343, bid128_to_int32_rninta, 0xB0180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_rninta_344, bid128_to_int32_rninta, 0xB0180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_rninta_345, bid128_to_int32_rninta, 0xb018000418021054202600c48c264840u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_346, bid128_to_int32_rninta, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_rninta_347, bid128_to_int32_rninta, 0xB01800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_rninta_348, bid128_to_int32_rninta, 0xB01800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_rninta_349, bid128_to_int32_rninta, 0xB01800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_rninta_350, bid128_to_int32_rninta, 0xB01800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_rninta_351, bid128_to_int32_rninta, 0xB01800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_rninta_352, bid128_to_int32_rninta, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rninta_353, bid128_to_int32_rninta, 0xB01A0000000000A2E6C09AD3E0D40000u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rninta_354, bid128_to_int32_rninta, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rninta_355, bid128_to_int32_rninta, 0xB01A000045639181BA2CDCFB7617FFFFu128, -2147483647, 0x00); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_rninta_356, bid128_to_int32_rninta, 0xB01A000045639181BA2CDCFB76180000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_rninta_357, bid128_to_int32_rninta, 0xB01A000045639181BA2CDCFB76180001u128, -2147483647, 0x00); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_rninta_358, bid128_to_int32_rninta, 0xB01A00004563918244F3FFFFFFFFFFFFu128, -2147483648, 0x00); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_rninta_359, bid128_to_int32_rninta, 0xB01A00004563918244F4000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_rninta_360, bid128_to_int32_rninta, 0xB01A00004563918244F4000000000001u128, -2147483648, 0x00); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_rninta_361, bid128_to_int32_rninta, 0xB01A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_rninta_362, bid128_to_int32_rninta, 0xB01A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_rninta_363, bid128_to_int32_rninta, 0xB01A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_rninta_364, bid128_to_int32_rninta, 0xB01A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_rninta_365, bid128_to_int32_rninta, 0xB01A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_rninta_366, bid128_to_int32_rninta, 0xB01A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_rninta_367, bid128_to_int32_rninta, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_rninta_368, bid128_to_int32_rninta, 0xB01A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_rninta_369, bid128_to_int32_rninta, 0xB01A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_rninta_370, bid128_to_int32_rninta, 0xB01A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_rninta_371, bid128_to_int32_rninta, 0xB01A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_rninta_372, bid128_to_int32_rninta, 0xB01A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_rninta_373, bid128_to_int32_rninta, 0xB01E000000000001A055690D9DB7FFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_374, bid128_to_int32_rninta, 0xB01E000000000001A055690D9DB80000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_375, bid128_to_int32_rninta, 0xB01E000000000001A055690D9DB80001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_376, bid128_to_int32_rninta, 0xB02000000000000029A2241AF62BFFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_377, bid128_to_int32_rninta, 0xB02000000000000029A2241AF62C0000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_378, bid128_to_int32_rninta, 0xB02000000000000029A2241AF62C0001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_379, bid128_to_int32_rninta, 0xB024000000000000006A94D74F42FFFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_380, bid128_to_int32_rninta, 0xB024000000000000006A94D74F430000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_381, bid128_to_int32_rninta, 0xB024000000000000006A94D74F430001u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_382, bid128_to_int32_rninta, 0xB02A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_rninta_383, bid128_to_int32_rninta, 0xB02A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_rninta_384, bid128_to_int32_rninta, 0xB02A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_rninta_385, bid128_to_int32_rninta, 0xB02A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_rninta_386, bid128_to_int32_rninta, 0xB02A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_rninta_387, bid128_to_int32_rninta, 0xB02A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_rninta_388, bid128_to_int32_rninta, 0xB02C000000000000000002BBA7F521FFu128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rninta_389, bid128_to_int32_rninta, 0xB02C000000000000000002BBA7F52200u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rninta_390, bid128_to_int32_rninta, 0xB02C000000000000000002BBA7F52201u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rninta_391, bid128_to_int32_rninta, 0xB02C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_rninta_392, bid128_to_int32_rninta, 0xB02C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_rninta_393, bid128_to_int32_rninta, 0xB02C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_rninta_394, bid128_to_int32_rninta, 0xB02C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_rninta_395, bid128_to_int32_rninta, 0xB02C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_rninta_396, bid128_to_int32_rninta, 0xB02C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_rninta_397, bid128_to_int32_rninta, 0xB02C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_rninta_398, bid128_to_int32_rninta, 0xB02C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_rninta_399, bid128_to_int32_rninta, 0xB02C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_rninta_400, bid128_to_int32_rninta, 0xB02E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_rninta_401, bid128_to_int32_rninta, 0xB02E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_rninta_402, bid128_to_int32_rninta, 0xB02E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_rninta_403, bid128_to_int32_rninta, 0xB03000000000000000000006FC23ABFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_404, bid128_to_int32_rninta, 0xB03000000000000000000006FC23AC00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_405, bid128_to_int32_rninta, 0xB03000000000000000000006FC23AC01u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_406, bid128_to_int32_rninta, 0xB03200000000000000000000B2D05DFFu128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_407, bid128_to_int32_rninta, 0xB03200000000000000000000B2D05E00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_408, bid128_to_int32_rninta, 0xB03200000000000000000000B2D05E01u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_409, bid128_to_int32_rninta, 0xB03800000000000000000000002DDA47u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rninta_410, bid128_to_int32_rninta, 0xB03800000000000000000000002DDA48u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rninta_411, bid128_to_int32_rninta, 0xB03800000000000000000000002DDA49u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rninta_412, bid128_to_int32_rninta, 0xB03A00000000000000000000000003E7u128, -1         , 0x00); // -- -(0.999)
dec_test!(bid128_to_int32_rninta_413, bid128_to_int32_rninta, 0xB03A00000000000000000000000495D3u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rninta_414, bid128_to_int32_rninta, 0xB03A00000000000000000000000495D4u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rninta_415, bid128_to_int32_rninta, 0xB03A00000000000000000000000495D5u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rninta_416, bid128_to_int32_rninta, 0xB03C0000000000000000000000007561u128, -300       , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_rninta_417, bid128_to_int32_rninta, 0xB03C0000000000000000000000007562u128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rninta_418, bid128_to_int32_rninta, 0xB03C0000000000000000000000007563u128, -301       , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_rninta_419, bid128_to_int32_rninta, 0xB03E0000000000000000000000000005u128, -1         , 0x00); // -- -(0.5)
dec_test!(bid128_to_int32_rninta_420, bid128_to_int32_rninta, 0xB03E000000000000000000000000000Fu128, -2         , 0x00); // -- -(1.5)
dec_test!(bid128_to_int32_rninta_421, bid128_to_int32_rninta, 0xB03E0000000000000000000000000BB7u128, -300       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_422, bid128_to_int32_rninta, 0xB03E0000000000000000000000000BB8u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_423, bid128_to_int32_rninta, 0xB03E0000000000000000000000000BB9u128, -300       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_424, bid128_to_int32_rninta, 0xB03E0000000000000000000000000BBDu128, -301       , 0x00); // -- -(300.5)
dec_test!(bid128_to_int32_rninta_425, bid128_to_int32_rninta, 0xB03E00000000000000000004FFFFFFF1u128, -2147483647, 0x00); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_rninta_426, bid128_to_int32_rninta, 0xB03E00000000000000000004FFFFFFFBu128, -2147483648, 0x00); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_rninta_427, bid128_to_int32_rninta, 0xB03E0000000000000000000500000005u128, -2147483648, 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_rninta_428, bid128_to_int32_rninta, 0xB03E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_rninta_429, bid128_to_int32_rninta, 0xB03E0000000000000000000A00000005u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_rninta_430, bid128_to_int32_rninta, 0xB03E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_rninta_431, bid128_to_int32_rninta, 0xB03E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_rninta_432, bid128_to_int32_rninta, 0xB03E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_rninta_433, bid128_to_int32_rninta, 0xB03E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_rninta_434, bid128_to_int32_rninta, 0xB0400000000000000000000000000001u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_rninta_435, bid128_to_int32_rninta, 0xB040000000000000000000000000012Bu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_436, bid128_to_int32_rninta, 0xB040000000000000000000000000012Cu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_437, bid128_to_int32_rninta, 0xB040000000000000000000000000012Du128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_438, bid128_to_int32_rninta, 0xB040000000000000000000007FFFFFFFu128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_rninta_439, bid128_to_int32_rninta, 0xB0400000000000000000000080000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_rninta_440, bid128_to_int32_rninta, 0xB0400000000000000000000080000001u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_rninta_441, bid128_to_int32_rninta, 0xB04000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_rninta_442, bid128_to_int32_rninta, 0xB0400000000000000000000100000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_rninta_443, bid128_to_int32_rninta, 0xB0400000000000000000000100000001u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_rninta_444, bid128_to_int32_rninta, 0xB04000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_rninta_445, bid128_to_int32_rninta, 0xB04000000000000000000004A817C801u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_rninta_446, bid128_to_int32_rninta, 0xB042000000000000000000000000001Du128, -290       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_rninta_447, bid128_to_int32_rninta, 0xB042000000000000000000000000001Eu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_448, bid128_to_int32_rninta, 0xB042000000000000000000000000001Fu128, -310       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_rninta_449, bid128_to_int32_rninta, 0xB04200000000000000000000773593FFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_rninta_450, bid128_to_int32_rninta, 0xB0420000000000000000000077359400u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_rninta_451, bid128_to_int32_rninta, 0xB0420000000000000000000077359401u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_rninta_452, bid128_to_int32_rninta, 0xB0440000000000000000000000000003u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_rninta_453, bid128_to_int32_rninta, 0xB0520000000000000000000000000004u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_rninta_454, bid128_to_int32_rninta, 0xB0520000000000000000000000000005u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_rninta_455, bid128_to_int32_rninta, 0xB0540000000000000000000000000002u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_rninta_456, bid128_to_int32_rninta, 0xeffffff7ffffffff201202b65c09d002u128, 0          , 0x00);
dec_test!(bid128_to_int32_rninta_457, bid128_to_int32_rninta, 0xfb2de79a9d146bdd0000180906000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_458, bid128_to_int32_rninta, 0xfffffffffb7fffff93d6d9630e17fa18u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_459, bid128_to_int32_rninta, "Infinity"                            , -2147483648, 0x01);
dec_test!(bid128_to_int32_rninta_460, bid128_to_int32_rninta, "SNaN"                                , -2147483648, 0x01);