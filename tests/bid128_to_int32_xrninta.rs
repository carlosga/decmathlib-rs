/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int32_xrninta_001, bid128_to_int32_xrninta, "-0"                                  ,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_002, bid128_to_int32_xrninta,  "0"                                  ,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_003, bid128_to_int32_xrninta, 0x00000000000000000000020000000080u128,  0           , 0x20);
dec_test!(bid128_to_int32_xrninta_004, bid128_to_int32_xrninta, 0x0000000000000000fe3fbd6ffbfdffffu128,  0           , 0x20);
dec_test!(bid128_to_int32_xrninta_005, bid128_to_int32_xrninta, "-0.000010E0"                         ,  0           , 0x20);
dec_test!(bid128_to_int32_xrninta_006, bid128_to_int32_xrninta, 0x0001ed09bead87c0378d8e62ffffffffu128,  0           , 0x20);
dec_test!(bid128_to_int32_xrninta_007, bid128_to_int32_xrninta, 0x0001ed09bead87c0378d8e64ffffffffu128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_008, bid128_to_int32_xrninta, 0x002038000020504840c5e5f698973568u128,  0           , 0x20);
dec_test!(bid128_to_int32_xrninta_009, bid128_to_int32_xrninta, "0.1"                                 ,  0           , 0x20);
dec_test!(bid128_to_int32_xrninta_010, bid128_to_int32_xrninta, 0x0c000000000000000000000000000000u128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_011, bid128_to_int32_xrninta, 0x0c7c0000000000000000000000000000u128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_012, bid128_to_int32_xrninta, 0x0d0a2e144640142778f00c00425f7b04u128,  0           , 0x20);
dec_test!(bid128_to_int32_xrninta_013, bid128_to_int32_xrninta, "+1.001010010E0"                      ,  1           , 0x20);
dec_test!(bid128_to_int32_xrninta_014, bid128_to_int32_xrninta, "1.0"                                 ,  1           , 0x00);
dec_test!(bid128_to_int32_xrninta_015, bid128_to_int32_xrninta, "1073741824"                          ,  1073741824  , 0x00);
dec_test!(bid128_to_int32_xrninta_016, bid128_to_int32_xrninta, "1"                                   ,  1           , 0x00);
dec_test!(bid128_to_int32_xrninta_017, bid128_to_int32_xrninta, 0x11640000000000000000000000000000u128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_018, bid128_to_int32_xrninta, 0x17060000000000000000000000000000u128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_019, bid128_to_int32_xrninta, 0x1e5fffaa4f778d660420200012005101u128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_020, bid128_to_int32_xrninta, "2147483648"                          ,  -2147483648 , 0x01);
dec_test!(bid128_to_int32_xrninta_021, bid128_to_int32_xrninta, 0x2631b75c991c965046b6c7f51244d5fbu128,  0           , 0x20);
dec_test!(bid128_to_int32_xrninta_022, bid128_to_int32_xrninta, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128,  0           , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_int32_xrninta_023, bid128_to_int32_xrninta, 0x2FFCF684DF56C3E01BC6C73200000000u128,  1           , 0x20); // -- 0.5
dec_test!(bid128_to_int32_xrninta_024, bid128_to_int32_xrninta, 0x2FFCF684DF56C3E01BC6C73200000001u128,  1           , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_int32_xrninta_025, bid128_to_int32_xrninta, 0x2ffd6dfff6fffebf0012000004001000u128,  1           , 0x20);
dec_test!(bid128_to_int32_xrninta_026, bid128_to_int32_xrninta, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128,  1           , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_int32_xrninta_027, bid128_to_int32_xrninta, 0x2FFDEC8B86EF679D76FC433D80000000u128,  1           , 0x20); // -- 0.999
dec_test!(bid128_to_int32_xrninta_028, bid128_to_int32_xrninta, 0x2FFDEC8B86EF679D76FC433D80000001u128,  1           , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_int32_xrninta_029, bid128_to_int32_xrninta, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128,  1           , 0x20); // -- 1-ulp
dec_test!(bid128_to_int32_xrninta_030, bid128_to_int32_xrninta, 0x2FFE314DC6448D9338C15B0A00000000u128,  1           , 0x00); // -- 1
dec_test!(bid128_to_int32_xrninta_031, bid128_to_int32_xrninta, 0x2FFE314DC6448D9338C15B0A00000001u128,  1           , 0x20); // -- 1+ulp
dec_test!(bid128_to_int32_xrninta_032, bid128_to_int32_xrninta, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128,  1           , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_int32_xrninta_033, bid128_to_int32_xrninta, 0x2FFE49F4A966D45CD522088F00000000u128,  2           , 0x20); // -- 1.5
dec_test!(bid128_to_int32_xrninta_034, bid128_to_int32_xrninta, 0x2FFE49F4A966D45CD522088F00000001u128,  2           , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_int32_xrninta_035, bid128_to_int32_xrninta, 0x3000009000000480a248bebbece3fbf3u128,  0           , 0x20);
dec_test!(bid128_to_int32_xrninta_036, bid128_to_int32_xrninta, 0x3000c008086072000000020000300104u128,  39          , 0x20);
dec_test!(bid128_to_int32_xrninta_037, bid128_to_int32_xrninta, 0x300293E952CDA8B9AA44111DFFFFFFFFu128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_038, bid128_to_int32_xrninta, 0x300293E952CDA8B9AA44111E00000000u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_039, bid128_to_int32_xrninta, 0x300293E952CDA8B9AA44111E00000001u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_040, bid128_to_int32_xrninta, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128,  300         , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrninta_041, bid128_to_int32_xrninta, 0x300294286EACB8CB0A8CB6B140000000u128,  301         , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrninta_042, bid128_to_int32_xrninta, 0x300294286EACB8CB0A8CB6B140000001u128,  301         , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrninta_043, bid128_to_int32_xrninta, 0x30040ECA8847C4129106CE82FFFFFFFFu128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_044, bid128_to_int32_xrninta, 0x30040ECA8847C4129106CE8300000000u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_045, bid128_to_int32_xrninta, 0x30040ECA8847C4129106CE8300000001u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_046, bid128_to_int32_xrninta, 0x300A0003C95A2F0B4856475FDFFFFFFFu128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_047, bid128_to_int32_xrninta, 0x300A0003C95A2F0B4856475FE0000000u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_048, bid128_to_int32_xrninta, 0x300A0003C95A2F0B4856475FE0000001u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_049, bid128_to_int32_xrninta, 0x300C000060EF6B1ABA6F07232FFFFFFFu128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_050, bid128_to_int32_xrninta, 0x300C000060EF6B1ABA6F072330000000u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_051, bid128_to_int32_xrninta, 0x300C000060EF6B1ABA6F072330000001u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_052, bid128_to_int32_xrninta, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128,  2147483646  , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_xrninta_053, bid128_to_int32_xrninta, 0x301069E10DE628D3A6C9CC9B8E800000u128,  2147483647  , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_int32_xrninta_054, bid128_to_int32_xrninta, 0x301069E10DE628D3A6C9CC9B8E800001u128,  2147483647  , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_xrninta_055, bid128_to_int32_xrninta, 0x301069E10DE692B4B4B133125EFFFFFFu128,  2147483647  , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_xrninta_056, bid128_to_int32_xrninta, 0x301069E10DE692B4B4B133125F000000u128,  2147483647  , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_xrninta_057, bid128_to_int32_xrninta, 0x301069E10DE692B4B4B133125F000001u128,  2147483647  , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_xrninta_058, bid128_to_int32_xrninta, 0x301069E10DE6FC95C29899892F7FFFFFu128,  2147483647  , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_xrninta_059, bid128_to_int32_xrninta, 0x301069E10DE6FC95C29899892F800000u128,  -2147483648 , 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_xrninta_060, bid128_to_int32_xrninta, 0x301069E10DE6FC95C29899892F800001u128,  -2147483648 , 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_xrninta_061, bid128_to_int32_xrninta, 0x301069E10DE76676D07FFFFFFFFFFFFFu128,  -2147483648 , 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_xrninta_062, bid128_to_int32_xrninta, 0x301069E10DE76676D080000000000000u128,  -2147483648 , 0x01); // -- 2^31
dec_test!(bid128_to_int32_xrninta_063, bid128_to_int32_xrninta, 0x301069E10DE76676D080000000000001u128,  -2147483648 , 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_xrninta_064, bid128_to_int32_xrninta, 0x301069E10DE7D057DE676676D07FFFFFu128,  -2147483648 , 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_xrninta_065, bid128_to_int32_xrninta, 0x301069E10DE7D057DE676676D0800000u128,  -2147483648 , 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_xrninta_066, bid128_to_int32_xrninta, 0x301069E10DE7D057DE676676D0800001u128,  -2147483648 , 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_xrninta_067, bid128_to_int32_xrninta, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128,  -2147483648 , 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_xrninta_068, bid128_to_int32_xrninta, 0x301069E10DE83A38EC4ECCEDA1000000u128,  -2147483648 , 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_xrninta_069, bid128_to_int32_xrninta, 0x301069E10DE83A38EC4ECCEDA1000001u128,  -2147483648 , 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_xrninta_070, bid128_to_int32_xrninta, 0x3010C5371912364CE3056C27FFFFFFFFu128,  -2147483648 , 0x01); // -- 4e9-ulp
dec_test!(bid128_to_int32_xrninta_071, bid128_to_int32_xrninta, 0x3010C5371912364CE3056C2800000000u128,  -2147483648 , 0x01); // -- 4e9
dec_test!(bid128_to_int32_xrninta_072, bid128_to_int32_xrninta, 0x3010C5371912364CE3056C2800000001u128,  -2147483648 , 0x01); // -- 4e9+ulp
dec_test!(bid128_to_int32_xrninta_073, bid128_to_int32_xrninta, 0x3010D3C21BCDF92B853133125EFFFFFFu128,  -2147483648 , 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_xrninta_074, bid128_to_int32_xrninta, 0x3010D3C21BCDF92B853133125F000000u128,  -2147483648 , 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_xrninta_075, bid128_to_int32_xrninta, 0x3010D3C21BCDF92B853133125F000001u128,  -2147483648 , 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_xrninta_076, bid128_to_int32_xrninta, 0x3010D3C21BCE630C931899892F7FFFFFu128,  -2147483648 , 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_xrninta_077, bid128_to_int32_xrninta, 0x3010D3C21BCE630C931899892F800000u128,  -2147483648 , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_xrninta_078, bid128_to_int32_xrninta, 0x3010D3C21BCE630C931899892F800001u128,  -2147483648 , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_xrninta_079, bid128_to_int32_xrninta, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128,  -2147483648 , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_xrninta_080, bid128_to_int32_xrninta, 0x3010D3C21BCECCEDA100000000000000u128,  -2147483648 , 0x01); // -- 2^32
dec_test!(bid128_to_int32_xrninta_081, bid128_to_int32_xrninta, 0x3010D3C21BCECCEDA100000000000001u128,  -2147483648 , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_xrninta_082, bid128_to_int32_xrninta, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128,  -2147483648 , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_xrninta_083, bid128_to_int32_xrninta, 0x3010D3C21BCF36CEAEE76676D0800000u128,  -2147483648 , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_xrninta_084, bid128_to_int32_xrninta, 0x3010D3C21BCF36CEAEE76676D0800001u128,  -2147483648 , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_xrninta_085, bid128_to_int32_xrninta, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128,  -2147483648 , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_xrninta_086, bid128_to_int32_xrninta, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128,  -2147483648 , 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_xrninta_087, bid128_to_int32_xrninta, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128,  -2147483648 , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_xrninta_088, bid128_to_int32_xrninta, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128,  -2147483648 , 0x01); // -- 5e9-ulp
dec_test!(bid128_to_int32_xrninta_089, bid128_to_int32_xrninta, 0x3010F684DF56C3E01BC6C73200000000u128,  -2147483648 , 0x01); // -- 5e9
dec_test!(bid128_to_int32_xrninta_090, bid128_to_int32_xrninta, 0x3010F684DF56C3E01BC6C73200000001u128,  -2147483648 , 0x01); // -- 5e9+ulp
dec_test!(bid128_to_int32_xrninta_091, bid128_to_int32_xrninta, 0x30111aa0f8d01cc95489c4c53380001fu128,  -2147483648 , 0x01);
dec_test!(bid128_to_int32_xrninta_092, bid128_to_int32_xrninta, 0x3012629B8C88FB62ED56E4238E3FFFFFu128,  -2147483648 , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_xrninta_093, bid128_to_int32_xrninta, 0x3012629B8C88FB62ED56E4238E400000u128,  -2147483648 , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_xrninta_094, bid128_to_int32_xrninta, 0x3012629B8C88FB62ED56E4238E400001u128,  -2147483648 , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_xrninta_095, bid128_to_int32_xrninta, 0x3012629B8C8905F96EBAD4C9097FFFFFu128,  -2147483648 , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_xrninta_096, bid128_to_int32_xrninta, 0x3012629B8C8905F96EBAD4C909800000u128,  -2147483648 , 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_xrninta_097, bid128_to_int32_xrninta, 0x3012629B8C8905F96EBAD4C909800001u128,  -2147483648 , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_xrninta_098, bid128_to_int32_xrninta, 0x3012629B8C89108FF01EC56E84BFFFFFu128,  -2147483648 , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_xrninta_099, bid128_to_int32_xrninta, 0x3012629B8C89108FF01EC56E84C00000u128,  -2147483648 , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_xrninta_100, bid128_to_int32_xrninta, 0x3012629B8C89108FF01EC56E84C00001u128,  -2147483648 , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_xrninta_101, bid128_to_int32_xrninta, 0x3012629B8C891B267182B613FFFFFFFFu128,  -2147483648 , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_xrninta_102, bid128_to_int32_xrninta, 0x3012629B8C891B267182B61400000000u128,  -2147483648 , 0x01); // -- 2e10
dec_test!(bid128_to_int32_xrninta_103, bid128_to_int32_xrninta, 0x3012629B8C891B267182B61400000001u128,  -2147483648 , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_xrninta_104, bid128_to_int32_xrninta, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128,  -2147483648 , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_xrninta_105, bid128_to_int32_xrninta, 0x3012629B8C8925BCF2E6A6B97B400000u128,  -2147483648 , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_xrninta_106, bid128_to_int32_xrninta, 0x3012629B8C8925BCF2E6A6B97B400001u128,  -2147483648 , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_xrninta_107, bid128_to_int32_xrninta, 0x3012629B8C893053744A975EF67FFFFFu128,  -2147483648 , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_xrninta_108, bid128_to_int32_xrninta, 0x3012629B8C893053744A975EF6800000u128,  -2147483648 , 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_xrninta_109, bid128_to_int32_xrninta, 0x3012629B8C893053744A975EF6800001u128,  -2147483648 , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_xrninta_110, bid128_to_int32_xrninta, 0x3012629B8C893AE9F5AE880471BFFFFFu128,  -2147483648 , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_xrninta_111, bid128_to_int32_xrninta, 0x3012629B8C893AE9F5AE880471C00000u128,  -2147483648 , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_xrninta_112, bid128_to_int32_xrninta, 0x3012629B8C893AE9F5AE880471C00001u128,  -2147483648 , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_xrninta_113, bid128_to_int32_xrninta, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128,  2147483646  , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_xrninta_114, bid128_to_int32_xrninta, 0x30180002B5E3AF0E8FDCF2BBEB680000u128,  2147483647  , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_int32_xrninta_115, bid128_to_int32_xrninta, 0x30180002B5E3AF0E8FDCF2BBEB680001u128,  2147483647  , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_xrninta_116, bid128_to_int32_xrninta, 0x30180002B5E3AF13FBA450E94E77FFFFu128,  2147483647  , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_xrninta_117, bid128_to_int32_xrninta, 0x30180002B5E3AF13FBA450E94E780000u128,  -2147483648 , 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_xrninta_118, bid128_to_int32_xrninta, 0x30180002B5E3AF13FBA450E94E780001u128,  -2147483648 , 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_xrninta_119, bid128_to_int32_xrninta, 0x30180002B5E3AF19676BAF16B187FFFFu128,  -2147483648 , 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_xrninta_120, bid128_to_int32_xrninta, 0x30180002B5E3AF19676BAF16B1880000u128,  -2147483648 , 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_xrninta_121, bid128_to_int32_xrninta, 0x30180002B5E3AF19676BAF16B1880001u128,  -2147483648 , 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_xrninta_122, bid128_to_int32_xrninta, 0x301800056BC75E2AAD2C50E94E77FFFFu128,  -2147483648 , 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_xrninta_123, bid128_to_int32_xrninta, 0x301800056BC75E2AAD2C50E94E780000u128,  -2147483648 , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_xrninta_124, bid128_to_int32_xrninta, 0x301800056BC75E2AAD2C50E94E780001u128,  -2147483648 , 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_xrninta_125, bid128_to_int32_xrninta, 0x301800056BC75E3018F3AF16B187FFFFu128,  -2147483648 , 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_xrninta_126, bid128_to_int32_xrninta, 0x301800056BC75E3018F3AF16B1880000u128,  -2147483648 , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_xrninta_127, bid128_to_int32_xrninta, 0x301800056BC75E3018F3AF16B1880001u128,  -2147483648 , 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_xrninta_128, bid128_to_int32_xrninta, 0x30180008900280007f5af4bcfbafc3efu128,  -2147483648 , 0x01);
dec_test!(bid128_to_int32_xrninta_129, bid128_to_int32_xrninta, 0x301A0000000000A2E6C09AD3E0D3FFFFu128,  300         , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrninta_130, bid128_to_int32_xrninta, 0x301A0000000000A2E6C09AD3E0D40000u128,  301         , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrninta_131, bid128_to_int32_xrninta, 0x301A0000000000A2E6C09AD3E0D40001u128,  301         , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrninta_132, bid128_to_int32_xrninta, 0x301A000045639181BA2CDCFB7617FFFFu128,  2147483647  , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_xrninta_133, bid128_to_int32_xrninta, 0x301A000045639181BA2CDCFB76180000u128,  2147483647  , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_xrninta_134, bid128_to_int32_xrninta, 0x301A000045639181BA2CDCFB76180001u128,  2147483647  , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_xrninta_135, bid128_to_int32_xrninta, 0x301A00004563918244F3FFFFFFFFFFFFu128,  -2147483648 , 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_xrninta_136, bid128_to_int32_xrninta, 0x301A00004563918244F4000000000000u128,  -2147483648 , 0x01); // -- 2^31
dec_test!(bid128_to_int32_xrninta_137, bid128_to_int32_xrninta, 0x301A00004563918244F4000000000001u128,  -2147483648 , 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_xrninta_138, bid128_to_int32_xrninta, 0x301A000045639182CFBB230489E7FFFFu128,  -2147483648 , 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_xrninta_139, bid128_to_int32_xrninta, 0x301A000045639182CFBB230489E80000u128,  -2147483648 , 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_xrninta_140, bid128_to_int32_xrninta, 0x301A000045639182CFBB230489E80001u128,  -2147483648 , 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_xrninta_141, bid128_to_int32_xrninta, 0x301A00008AC72303FF20DCFB7617FFFFu128,  -2147483648 , 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_xrninta_142, bid128_to_int32_xrninta, 0x301A00008AC72303FF20DCFB76180000u128,  -2147483648 , 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_xrninta_143, bid128_to_int32_xrninta, 0x301A00008AC72303FF20DCFB76180001u128,  -2147483648 , 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_xrninta_144, bid128_to_int32_xrninta, 0x301A00008AC7230489E7FFFFFFFFFFFFu128,  -2147483648 , 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_xrninta_145, bid128_to_int32_xrninta, 0x301A00008AC7230489E8000000000000u128,  -2147483648 , 0x01); // -- 2^32
dec_test!(bid128_to_int32_xrninta_146, bid128_to_int32_xrninta, 0x301A00008AC7230489E8000000000001u128,  -2147483648 , 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_xrninta_147, bid128_to_int32_xrninta, 0x301A00008AC7230514AF230489E7FFFFu128,  -2147483648 , 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_xrninta_148, bid128_to_int32_xrninta, 0x301A00008AC7230514AF230489E80000u128,  -2147483648 , 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_xrninta_149, bid128_to_int32_xrninta, 0x301A00008AC7230514AF230489E80001u128,  -2147483648 , 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_xrninta_150, bid128_to_int32_xrninta, 0x301c000000000000082a4414c12a92abu128,  1           , 0x20);
dec_test!(bid128_to_int32_xrninta_151, bid128_to_int32_xrninta, 0x301E000000000001A055690D9DB7FFFFu128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_152, bid128_to_int32_xrninta, 0x301E000000000001A055690D9DB80000u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_153, bid128_to_int32_xrninta, 0x301E000000000001A055690D9DB80001u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_154, bid128_to_int32_xrninta, 0x302000000000000029A2241AF62BFFFFu128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_155, bid128_to_int32_xrninta, 0x302000000000000029A2241AF62C0000u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_156, bid128_to_int32_xrninta, 0x302000000000000029A2241AF62C0001u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_157, bid128_to_int32_xrninta, 0x30200000000001020000000000000000u128,  475926      , 0x20);
dec_test!(bid128_to_int32_xrninta_158, bid128_to_int32_xrninta, 0x30200000000080007ffdffffabfdf7f5u128,  60447213    , 0x20);
dec_test!(bid128_to_int32_xrninta_159, bid128_to_int32_xrninta, 0x3024000000000000006A94D74F42FFFFu128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_160, bid128_to_int32_xrninta, 0x3024000000000000006A94D74F430000u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_161, bid128_to_int32_xrninta, 0x3024000000000000006A94D74F430001u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_162, bid128_to_int32_xrninta, 0x302A00000000006C6B935B68D08DA3FFu128,  -2147483648 , 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_xrninta_163, bid128_to_int32_xrninta, 0x302A00000000006C6B935B68D08DA400u128,  -2147483648 , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_xrninta_164, bid128_to_int32_xrninta, 0x302A00000000006C6B935B68D08DA401u128,  -2147483648 , 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_xrninta_165, bid128_to_int32_xrninta, 0x302A00000000006C6B935B8019048BFFu128,  -2147483648 , 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_xrninta_166, bid128_to_int32_xrninta, 0x302A00000000006C6B935B8019048C00u128,  -2147483648 , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_xrninta_167, bid128_to_int32_xrninta, 0x302A00000000006C6B935B8019048C01u128,  -2147483648 , 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_xrninta_168, bid128_to_int32_xrninta, 0x302C000000000000000002BBA7F521FFu128,  300         , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrninta_169, bid128_to_int32_xrninta, 0x302C000000000000000002BBA7F52200u128,  301         , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrninta_170, bid128_to_int32_xrninta, 0x302C000000000000000002BBA7F52201u128,  301         , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrninta_171, bid128_to_int32_xrninta, 0x302C00000000000AD78EBC5872141BFFu128,  -2147483648 , 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_xrninta_172, bid128_to_int32_xrninta, 0x302C00000000000AD78EBC5872141C00u128,  -2147483648 , 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_xrninta_173, bid128_to_int32_xrninta, 0x302C00000000000AD78EBC5872141C01u128,  -2147483648 , 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_xrninta_174, bid128_to_int32_xrninta, 0x302C00000000000AD78EBC5BF025F1FFu128,  -2147483648 , 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_xrninta_175, bid128_to_int32_xrninta, 0x302C00000000000AD78EBC5BF025F200u128,  -2147483648 , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_xrninta_176, bid128_to_int32_xrninta, 0x302C00000000000AD78EBC5BF025F201u128,  -2147483648 , 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_xrninta_177, bid128_to_int32_xrninta, 0x302C00000000000AD78EBC5E4431D5FFu128,  -2147483648 , 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_xrninta_178, bid128_to_int32_xrninta, 0x302C00000000000AD78EBC5E4431D600u128,  -2147483648 , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_xrninta_179, bid128_to_int32_xrninta, 0x302C00000000000AD78EBC5E4431D601u128,  -2147483648 , 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_xrninta_180, bid128_to_int32_xrninta, 0x302E000000000001158E46094F6AC9FFu128,  -2147483648 , 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_xrninta_181, bid128_to_int32_xrninta, 0x302E000000000001158E46094F6ACA00u128,  -2147483648 , 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_xrninta_182, bid128_to_int32_xrninta, 0x302E000000000001158E46094F6ACA01u128,  -2147483648 , 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_xrninta_183, bid128_to_int32_xrninta, 0x303000000000000000000006FC23ABFFu128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_184, bid128_to_int32_xrninta, 0x303000000000000000000006FC23AC00u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_185, bid128_to_int32_xrninta, 0x303000000000000000000006FC23AC01u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_186, bid128_to_int32_xrninta, 0x303200000000000000000000B2D05DFFu128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_187, bid128_to_int32_xrninta, 0x303200000000000000000000B2D05E00u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_188, bid128_to_int32_xrninta, 0x303200000000000000000000B2D05E01u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_189, bid128_to_int32_xrninta, 0x303800000000000000000000002DDA47u128,  300         , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrninta_190, bid128_to_int32_xrninta, 0x303800000000000000000000002DDA48u128,  301         , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrninta_191, bid128_to_int32_xrninta, 0x303800000000000000000000002DDA49u128,  301         , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrninta_192, bid128_to_int32_xrninta, 0x303A00000000000000000000000003E7u128,  1           , 0x20); // -- 0.999
dec_test!(bid128_to_int32_xrninta_193, bid128_to_int32_xrninta, 0x303A00000000000000000000000495D3u128,  300         , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrninta_194, bid128_to_int32_xrninta, 0x303A00000000000000000000000495D4u128,  301         , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrninta_195, bid128_to_int32_xrninta, 0x303A00000000000000000000000495D5u128,  301         , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrninta_196, bid128_to_int32_xrninta, 0x303C0000000000000000000000007561u128,  300         , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrninta_197, bid128_to_int32_xrninta, 0x303C0000000000000000000000007562u128,  301         , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrninta_198, bid128_to_int32_xrninta, 0x303C0000000000000000000000007563u128,  301         , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrninta_199, bid128_to_int32_xrninta, 0x303E0000000000000000000000000005u128,  1           , 0x20); // -- 0.5
dec_test!(bid128_to_int32_xrninta_200, bid128_to_int32_xrninta, 0x303E000000000000000000000000000Fu128,  2           , 0x20); // -- 1.5
dec_test!(bid128_to_int32_xrninta_201, bid128_to_int32_xrninta, 0x303E0000000000000000000000000BB7u128,  300         , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_202, bid128_to_int32_xrninta, 0x303E0000000000000000000000000BB8u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_203, bid128_to_int32_xrninta, 0x303E0000000000000000000000000BB9u128,  300         , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_204, bid128_to_int32_xrninta, 0x303E0000000000000000000000000BBDu128,  301         , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrninta_205, bid128_to_int32_xrninta, 0x303E00000000000000000004FFFFFFF1u128,  2147483647  , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_int32_xrninta_206, bid128_to_int32_xrninta, 0x303E00000000000000000004FFFFFFFBu128,  -2147483648 , 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_xrninta_207, bid128_to_int32_xrninta, 0x303E0000000000000000000500000005u128,  -2147483648 , 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_xrninta_208, bid128_to_int32_xrninta, 0x303E00000000000000000009FFFFFFFBu128,  -2147483648 , 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_xrninta_209, bid128_to_int32_xrninta, 0x303E0000000000000000000A00000005u128,  -2147483648 , 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_xrninta_210, bid128_to_int32_xrninta, 0x303E0000000000000000002E90EDCFF1u128,  -2147483648 , 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_xrninta_211, bid128_to_int32_xrninta, 0x303E0000000000000000002E90EDCFFBu128,  -2147483648 , 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_xrninta_212, bid128_to_int32_xrninta, 0x303E0000000000000000002E90EDD005u128,  -2147483648 , 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_xrninta_213, bid128_to_int32_xrninta, 0x303E0000000000000000002E90EDD00Fu128,  -2147483648 , 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_xrninta_214, bid128_to_int32_xrninta, 0x30400000000000000000000000000001u128,  1           , 0x00); // -- 1
dec_test!(bid128_to_int32_xrninta_215, bid128_to_int32_xrninta, 0x3040000000000000000000000000012Bu128,  299         , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_216, bid128_to_int32_xrninta, 0x3040000000000000000000000000012Cu128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_217, bid128_to_int32_xrninta, 0x3040000000000000000000000000012Du128,  301         , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_218, bid128_to_int32_xrninta, 0x3040000000000000000000007FFFFFFFu128,  2147483647  , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_xrninta_219, bid128_to_int32_xrninta, 0x30400000000000000000000080000000u128,  -2147483648 , 0x01); // -- 2^31
dec_test!(bid128_to_int32_xrninta_220, bid128_to_int32_xrninta, 0x30400000000000000000000080000001u128,  -2147483648 , 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_xrninta_221, bid128_to_int32_xrninta, 0x304000000000000000000000FFFFFFFFu128,  -2147483648 , 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_xrninta_222, bid128_to_int32_xrninta, 0x30400000000000000000000100000000u128,  -2147483648 , 0x01); // -- 2^32
dec_test!(bid128_to_int32_xrninta_223, bid128_to_int32_xrninta, 0x30400000000000000000000100000001u128,  -2147483648 , 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_xrninta_224, bid128_to_int32_xrninta, 0x304000000000000000000004A817C7FFu128,  -2147483648 , 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_xrninta_225, bid128_to_int32_xrninta, 0x304000000000000000000004A817C801u128,  -2147483648 , 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_xrninta_226, bid128_to_int32_xrninta, 0x3041ED09BEAD87C0378D8E6400000000u128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_227, bid128_to_int32_xrninta, 0x3042000000000000000000000000001Du128,  290         , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_xrninta_228, bid128_to_int32_xrninta, 0x3042000000000000000000000000001Eu128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_229, bid128_to_int32_xrninta, 0x3042000000000000000000000000001Fu128,  310         , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_xrninta_230, bid128_to_int32_xrninta, 0x304200000000000000000000773593FFu128,  -2147483648 , 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_xrninta_231, bid128_to_int32_xrninta, 0x30420000000000000000000077359400u128,  -2147483648 , 0x01); // -- 2e10
dec_test!(bid128_to_int32_xrninta_232, bid128_to_int32_xrninta, 0x30420000000000000000000077359401u128,  -2147483648 , 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_xrninta_233, bid128_to_int32_xrninta, 0x30440000000000000000000000000003u128,  300         , 0x00); // -- 300
dec_test!(bid128_to_int32_xrninta_234, bid128_to_int32_xrninta, 0x30520000000000000000000000000004u128,  -2147483648 , 0x01); // -- 4e9
dec_test!(bid128_to_int32_xrninta_235, bid128_to_int32_xrninta, 0x30520000000000000000000000000005u128,  -2147483648 , 0x01); // -- 5e9
dec_test!(bid128_to_int32_xrninta_236, bid128_to_int32_xrninta, 0x30540000000000000000000000000002u128,  -2147483648 , 0x01); // -- 2e10
dec_test!(bid128_to_int32_xrninta_237, bid128_to_int32_xrninta, 0x31e40000000000000000000000000000u128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_238, bid128_to_int32_xrninta, 0x3b880000000000000000000000000000u128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_239, bid128_to_int32_xrninta, 0x45939b4de155e68b0f314aa0dd7771bbu128,  -2147483648 , 0x01);
dec_test!(bid128_to_int32_xrninta_240, bid128_to_int32_xrninta, "5.05"                                ,  5           , 0x20);
dec_test!(bid128_to_int32_xrninta_241, bid128_to_int32_xrninta, 0x51620000000000000000000000000000u128,  0           , 0x00);
dec_test!(bid128_to_int32_xrninta_242, bid128_to_int32_xrninta, 0x5301d2cd3055bac71593dc94bd23fe25u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_243, bid128_to_int32_xrninta, "5.5"                                 ,  6           , 0x20);
dec_test!(bid128_to_int32_xrninta_244, bid128_to_int32_xrninta, 0x59b83bca01ef092315404b234eb30d35u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_245, bid128_to_int32_xrninta, "+677758.88757977578559E0"            ,  677759      , 0x20);
dec_test!(bid128_to_int32_xrninta_246, bid128_to_int32_xrninta, "-69558.666799957E0"                  ,  -69559      , 0x20);
dec_test!(bid128_to_int32_xrninta_247, bid128_to_int32_xrninta, "+695979986.76578E0"                  ,  695979987   , 0x20);
dec_test!(bid128_to_int32_xrninta_248, bid128_to_int32_xrninta, 0x78000000000000000000000000000000u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_249, bid128_to_int32_xrninta, 0x7b4dbbdff5c4f7be0040000000000000u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_250, bid128_to_int32_xrninta, 0x7c000000000000000000000000000000u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_251, bid128_to_int32_xrninta, 0x7c003fffffffffff38c15b08ffffffffu128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_252, bid128_to_int32_xrninta, 0x7c003fffffffffff38c15b0affffffffu128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_253, bid128_to_int32_xrninta, 0x7e000000000000000000000000000000u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_254, bid128_to_int32_xrninta, 0x82b40000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_int32_xrninta_255, bid128_to_int32_xrninta, "-9"                                  ,  -9          , 0x00);
dec_test!(bid128_to_int32_xrninta_256, bid128_to_int32_xrninta, 0xa1b80000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_int32_xrninta_257, bid128_to_int32_xrninta, 0xa2f02624c669655d309fda457a025819u128, 0            , 0x20);
dec_test!(bid128_to_int32_xrninta_258, bid128_to_int32_xrninta, 0xa7d028586efbde1effd62d649e4def9du128, 0            , 0x20);
dec_test!(bid128_to_int32_xrninta_259, bid128_to_int32_xrninta, 0xabea26c6965f210b3c96572bc15ef28au128, 0            , 0x20);
dec_test!(bid128_to_int32_xrninta_260, bid128_to_int32_xrninta, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0            , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_int32_xrninta_261, bid128_to_int32_xrninta, 0xAFFCF684DF56C3E01BC6C73200000000u128, -1           , 0x20); // -- -(0.5)
dec_test!(bid128_to_int32_xrninta_262, bid128_to_int32_xrninta, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1           , 0x20); // -- -(0.5+ulp)
dec_test!(bid128_to_int32_xrninta_263, bid128_to_int32_xrninta, 0xaffd5ff7fffffffef9cfbdde7ff7f7f9u128, -1           , 0x20);
dec_test!(bid128_to_int32_xrninta_264, bid128_to_int32_xrninta, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1           , 0x20); // -- -(0.999-ulp)
dec_test!(bid128_to_int32_xrninta_265, bid128_to_int32_xrninta, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1           , 0x20); // -- -(0.999)
dec_test!(bid128_to_int32_xrninta_266, bid128_to_int32_xrninta, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1           , 0x20); // -- -(0.999+ulp)
dec_test!(bid128_to_int32_xrninta_267, bid128_to_int32_xrninta, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1           , 0x20); // -- -(1-ulp)
dec_test!(bid128_to_int32_xrninta_268, bid128_to_int32_xrninta, 0xAFFE314DC6448D9338C15B0A00000000u128, -1           , 0x00); // -- -(1)
dec_test!(bid128_to_int32_xrninta_269, bid128_to_int32_xrninta, 0xAFFE314DC6448D9338C15B0A00000001u128, -1           , 0x20); // -- -(1+ulp)
dec_test!(bid128_to_int32_xrninta_270, bid128_to_int32_xrninta, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1           , 0x20); // -- -(1.5-ulp)
dec_test!(bid128_to_int32_xrninta_271, bid128_to_int32_xrninta, 0xAFFE49F4A966D45CD522088F00000000u128, -2           , 0x20); // -- -(1.5)
dec_test!(bid128_to_int32_xrninta_272, bid128_to_int32_xrninta, 0xAFFE49F4A966D45CD522088F00000001u128, -2           , 0x20); // -- -(1.5+ulp)
dec_test!(bid128_to_int32_xrninta_273, bid128_to_int32_xrninta, 0xaffedbbdcdff7c7dce3fd3738aea6ffcu128, -4           , 0x20);
dec_test!(bid128_to_int32_xrninta_274, bid128_to_int32_xrninta, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_275, bid128_to_int32_xrninta, 0xB00293E952CDA8B9AA44111E00000000u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_276, bid128_to_int32_xrninta, 0xB00293E952CDA8B9AA44111E00000001u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_277, bid128_to_int32_xrninta, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300         , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrninta_278, bid128_to_int32_xrninta, 0xB00294286EACB8CB0A8CB6B140000000u128, -301         , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrninta_279, bid128_to_int32_xrninta, 0xB00294286EACB8CB0A8CB6B140000001u128, -301         , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrninta_280, bid128_to_int32_xrninta, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_281, bid128_to_int32_xrninta, 0xB0040ECA8847C4129106CE8300000000u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_282, bid128_to_int32_xrninta, 0xB0040ECA8847C4129106CE8300000001u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_283, bid128_to_int32_xrninta, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_284, bid128_to_int32_xrninta, 0xB00A0003C95A2F0B4856475FE0000000u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_285, bid128_to_int32_xrninta, 0xB00A0003C95A2F0B4856475FE0000001u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_286, bid128_to_int32_xrninta, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_287, bid128_to_int32_xrninta, 0xB00C000060EF6B1ABA6F072330000000u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_288, bid128_to_int32_xrninta, 0xB00C000060EF6B1ABA6F072330000001u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_289, bid128_to_int32_xrninta, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, -2147483646  , 0x20); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_xrninta_290, bid128_to_int32_xrninta, 0xB01069E10DE628D3A6C9CC9B8E800000u128, -2147483647  , 0x20); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_xrninta_291, bid128_to_int32_xrninta, 0xB01069E10DE628D3A6C9CC9B8E800001u128, -2147483647  , 0x20); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_xrninta_292, bid128_to_int32_xrninta, 0xB01069E10DE692B4B4B133125EFFFFFFu128, -2147483647  , 0x20); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_xrninta_293, bid128_to_int32_xrninta, 0xB01069E10DE692B4B4B133125F000000u128, -2147483647  , 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_xrninta_294, bid128_to_int32_xrninta, 0xB01069E10DE692B4B4B133125F000001u128, -2147483647  , 0x20); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_xrninta_295, bid128_to_int32_xrninta, 0xB01069E10DE6FC95C29899892F7FFFFFu128, -2147483647  , 0x20); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_xrninta_296, bid128_to_int32_xrninta, 0xB01069E10DE6FC95C29899892F800000u128, -2147483648  , 0x20); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_xrninta_297, bid128_to_int32_xrninta, 0xB01069E10DE6FC95C29899892F800001u128, -2147483648  , 0x20); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_xrninta_298, bid128_to_int32_xrninta, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, -2147483648  , 0x20); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_xrninta_299, bid128_to_int32_xrninta, 0xB01069E10DE76676D080000000000000u128, -2147483648  , 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_xrninta_300, bid128_to_int32_xrninta, 0xB01069E10DE76676D080000000000001u128, -2147483648  , 0x20); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_xrninta_301, bid128_to_int32_xrninta, 0xB01069E10DE7D057DE676676D07FFFFFu128, -2147483648  , 0x20); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_xrninta_302, bid128_to_int32_xrninta, 0xB01069E10DE7D057DE676676D0800000u128, -2147483648  , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_xrninta_303, bid128_to_int32_xrninta, 0xB01069E10DE7D057DE676676D0800001u128, -2147483648  , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_xrninta_304, bid128_to_int32_xrninta, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648  , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_xrninta_305, bid128_to_int32_xrninta, 0xB01069E10DE83A38EC4ECCEDA1000000u128, -2147483648  , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_xrninta_306, bid128_to_int32_xrninta, 0xB01069E10DE83A38EC4ECCEDA1000001u128, -2147483648  , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_xrninta_307, bid128_to_int32_xrninta, 0xB010C5371912364CE3056C27FFFFFFFFu128, -2147483648  , 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_int32_xrninta_308, bid128_to_int32_xrninta, 0xB010C5371912364CE3056C2800000000u128, -2147483648  , 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_xrninta_309, bid128_to_int32_xrninta, 0xB010C5371912364CE3056C2800000001u128, -2147483648  , 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_int32_xrninta_310, bid128_to_int32_xrninta, 0xB010D3C21BCDF92B853133125EFFFFFFu128, -2147483648  , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_xrninta_311, bid128_to_int32_xrninta, 0xB010D3C21BCDF92B853133125F000000u128, -2147483648  , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_xrninta_312, bid128_to_int32_xrninta, 0xB010D3C21BCDF92B853133125F000001u128, -2147483648  , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_xrninta_313, bid128_to_int32_xrninta, 0xB010D3C21BCE630C931899892F7FFFFFu128, -2147483648  , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_xrninta_314, bid128_to_int32_xrninta, 0xB010D3C21BCE630C931899892F800000u128, -2147483648  , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_xrninta_315, bid128_to_int32_xrninta, 0xB010D3C21BCE630C931899892F800001u128, -2147483648  , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_xrninta_316, bid128_to_int32_xrninta, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648  , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_xrninta_317, bid128_to_int32_xrninta, 0xB010D3C21BCECCEDA100000000000000u128, -2147483648  , 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_xrninta_318, bid128_to_int32_xrninta, 0xB010D3C21BCECCEDA100000000000001u128, -2147483648  , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_xrninta_319, bid128_to_int32_xrninta, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648  , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_xrninta_320, bid128_to_int32_xrninta, 0xB010D3C21BCF36CEAEE76676D0800000u128, -2147483648  , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_xrninta_321, bid128_to_int32_xrninta, 0xB010D3C21BCF36CEAEE76676D0800001u128, -2147483648  , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_xrninta_322, bid128_to_int32_xrninta, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648  , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_xrninta_323, bid128_to_int32_xrninta, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648  , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_xrninta_324, bid128_to_int32_xrninta, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648  , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_xrninta_325, bid128_to_int32_xrninta, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648  , 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_int32_xrninta_326, bid128_to_int32_xrninta, 0xB010F684DF56C3E01BC6C73200000000u128, -2147483648  , 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_xrninta_327, bid128_to_int32_xrninta, 0xB010F684DF56C3E01BC6C73200000001u128, -2147483648  , 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_int32_xrninta_328, bid128_to_int32_xrninta, 0xb0118100100e0600fffbfffbeeedffdfu128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_329, bid128_to_int32_xrninta, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648  , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_xrninta_330, bid128_to_int32_xrninta, 0xB012629B8C88FB62ED56E4238E400000u128, -2147483648  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_xrninta_331, bid128_to_int32_xrninta, 0xB012629B8C88FB62ED56E4238E400001u128, -2147483648  , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_xrninta_332, bid128_to_int32_xrninta, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648  , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_xrninta_333, bid128_to_int32_xrninta, 0xB012629B8C8905F96EBAD4C909800000u128, -2147483648  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_xrninta_334, bid128_to_int32_xrninta, 0xB012629B8C8905F96EBAD4C909800001u128, -2147483648  , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_xrninta_335, bid128_to_int32_xrninta, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -2147483648  , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_xrninta_336, bid128_to_int32_xrninta, 0xB012629B8C89108FF01EC56E84C00000u128, -2147483648  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_xrninta_337, bid128_to_int32_xrninta, 0xB012629B8C89108FF01EC56E84C00001u128, -2147483648  , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_xrninta_338, bid128_to_int32_xrninta, 0xB012629B8C891B267182B613FFFFFFFFu128, -2147483648  , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_xrninta_339, bid128_to_int32_xrninta, 0xB012629B8C891B267182B61400000000u128, -2147483648  , 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_xrninta_340, bid128_to_int32_xrninta, 0xB012629B8C891B267182B61400000001u128, -2147483648  , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_xrninta_341, bid128_to_int32_xrninta, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648  , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_xrninta_342, bid128_to_int32_xrninta, 0xB012629B8C8925BCF2E6A6B97B400000u128, -2147483648  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_xrninta_343, bid128_to_int32_xrninta, 0xB012629B8C8925BCF2E6A6B97B400001u128, -2147483648  , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_xrninta_344, bid128_to_int32_xrninta, 0xB012629B8C893053744A975EF67FFFFFu128, -2147483648  , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_xrninta_345, bid128_to_int32_xrninta, 0xB012629B8C893053744A975EF6800000u128, -2147483648  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_xrninta_346, bid128_to_int32_xrninta, 0xB012629B8C893053744A975EF6800001u128, -2147483648  , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_xrninta_347, bid128_to_int32_xrninta, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -2147483648  , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_xrninta_348, bid128_to_int32_xrninta, 0xB012629B8C893AE9F5AE880471C00000u128, -2147483648  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_xrninta_349, bid128_to_int32_xrninta, 0xB012629B8C893AE9F5AE880471C00001u128, -2147483648  , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_xrninta_350, bid128_to_int32_xrninta, 0xb014010838c00c004000080200480020u128, -2093379823  , 0x20);
dec_test!(bid128_to_int32_xrninta_351, bid128_to_int32_xrninta, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, -2147483646  , 0x20); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_xrninta_352, bid128_to_int32_xrninta, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, -2147483647  , 0x20); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_xrninta_353, bid128_to_int32_xrninta, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, -2147483647  , 0x20); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_xrninta_354, bid128_to_int32_xrninta, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, -2147483647  , 0x20); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_xrninta_355, bid128_to_int32_xrninta, 0xB0180002B5E3AF13FBA450E94E780000u128, -2147483648  , 0x20); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_xrninta_356, bid128_to_int32_xrninta, 0xB0180002B5E3AF13FBA450E94E780001u128, -2147483648  , 0x20); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_xrninta_357, bid128_to_int32_xrninta, 0xB0180002B5E3AF19676BAF16B187FFFFu128, -2147483648  , 0x20); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_xrninta_358, bid128_to_int32_xrninta, 0xB0180002B5E3AF19676BAF16B1880000u128, -2147483648  , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_xrninta_359, bid128_to_int32_xrninta, 0xB0180002B5E3AF19676BAF16B1880001u128, -2147483648  , 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_xrninta_360, bid128_to_int32_xrninta, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, -2147483648  , 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_xrninta_361, bid128_to_int32_xrninta, 0xB01800056BC75E2AAD2C50E94E780000u128, -2147483648  , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_xrninta_362, bid128_to_int32_xrninta, 0xB01800056BC75E2AAD2C50E94E780001u128, -2147483648  , 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_xrninta_363, bid128_to_int32_xrninta, 0xB01800056BC75E3018F3AF16B187FFFFu128, -2147483648  , 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_xrninta_364, bid128_to_int32_xrninta, 0xB01800056BC75E3018F3AF16B1880000u128, -2147483648  , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_xrninta_365, bid128_to_int32_xrninta, 0xB01800056BC75E3018F3AF16B1880001u128, -2147483648  , 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_xrninta_366, bid128_to_int32_xrninta, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300         , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrninta_367, bid128_to_int32_xrninta, 0xB01A0000000000A2E6C09AD3E0D40000u128, -301         , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrninta_368, bid128_to_int32_xrninta, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301         , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrninta_369, bid128_to_int32_xrninta, 0xB01A000045639181BA2CDCFB7617FFFFu128, -2147483647  , 0x20); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_xrninta_370, bid128_to_int32_xrninta, 0xB01A000045639181BA2CDCFB76180000u128, -2147483647  , 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_xrninta_371, bid128_to_int32_xrninta, 0xB01A000045639181BA2CDCFB76180001u128, -2147483647  , 0x20); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_xrninta_372, bid128_to_int32_xrninta, 0xB01A00004563918244F3FFFFFFFFFFFFu128, -2147483648  , 0x20); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_xrninta_373, bid128_to_int32_xrninta, 0xB01A00004563918244F4000000000000u128, -2147483648  , 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_xrninta_374, bid128_to_int32_xrninta, 0xB01A00004563918244F4000000000001u128, -2147483648  , 0x20); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_xrninta_375, bid128_to_int32_xrninta, 0xB01A000045639182CFBB230489E7FFFFu128, -2147483648  , 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_xrninta_376, bid128_to_int32_xrninta, 0xB01A000045639182CFBB230489E80000u128, -2147483648  , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_xrninta_377, bid128_to_int32_xrninta, 0xB01A000045639182CFBB230489E80001u128, -2147483648  , 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_xrninta_378, bid128_to_int32_xrninta, 0xB01A00008AC72303FF20DCFB7617FFFFu128, -2147483648  , 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_xrninta_379, bid128_to_int32_xrninta, 0xB01A00008AC72303FF20DCFB76180000u128, -2147483648  , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_xrninta_380, bid128_to_int32_xrninta, 0xB01A00008AC72303FF20DCFB76180001u128, -2147483648  , 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_xrninta_381, bid128_to_int32_xrninta, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648  , 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_xrninta_382, bid128_to_int32_xrninta, 0xB01A00008AC7230489E8000000000000u128, -2147483648  , 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_xrninta_383, bid128_to_int32_xrninta, 0xB01A00008AC7230489E8000000000001u128, -2147483648  , 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_xrninta_384, bid128_to_int32_xrninta, 0xB01A00008AC7230514AF230489E7FFFFu128, -2147483648  , 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_xrninta_385, bid128_to_int32_xrninta, 0xB01A00008AC7230514AF230489E80000u128, -2147483648  , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_xrninta_386, bid128_to_int32_xrninta, 0xB01A00008AC7230514AF230489E80001u128, -2147483648  , 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_xrninta_387, bid128_to_int32_xrninta, 0xB01E000000000001A055690D9DB7FFFFu128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_388, bid128_to_int32_xrninta, 0xB01E000000000001A055690D9DB80000u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_389, bid128_to_int32_xrninta, 0xB01E000000000001A055690D9DB80001u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_390, bid128_to_int32_xrninta, 0xB02000000000000029A2241AF62BFFFFu128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_391, bid128_to_int32_xrninta, 0xB02000000000000029A2241AF62C0000u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_392, bid128_to_int32_xrninta, 0xB02000000000000029A2241AF62C0001u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_393, bid128_to_int32_xrninta, 0xb0200000004010280010200000800008u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_394, bid128_to_int32_xrninta, 0xB024000000000000006A94D74F42FFFFu128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_395, bid128_to_int32_xrninta, 0xB024000000000000006A94D74F430000u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_396, bid128_to_int32_xrninta, 0xB024000000000000006A94D74F430001u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_397, bid128_to_int32_xrninta, 0xB02A00000000006C6B935B68D08DA3FFu128, -2147483648  , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_xrninta_398, bid128_to_int32_xrninta, 0xB02A00000000006C6B935B68D08DA400u128, -2147483648  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_xrninta_399, bid128_to_int32_xrninta, 0xB02A00000000006C6B935B68D08DA401u128, -2147483648  , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_xrninta_400, bid128_to_int32_xrninta, 0xB02A00000000006C6B935B8019048BFFu128, -2147483648  , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_xrninta_401, bid128_to_int32_xrninta, 0xB02A00000000006C6B935B8019048C00u128, -2147483648  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_xrninta_402, bid128_to_int32_xrninta, 0xB02A00000000006C6B935B8019048C01u128, -2147483648  , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_xrninta_403, bid128_to_int32_xrninta, 0xB02C000000000000000002BBA7F521FFu128, -300         , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrninta_404, bid128_to_int32_xrninta, 0xB02C000000000000000002BBA7F52200u128, -301         , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrninta_405, bid128_to_int32_xrninta, 0xB02C000000000000000002BBA7F52201u128, -301         , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrninta_406, bid128_to_int32_xrninta, 0xB02C00000000000AD78EBC5872141BFFu128, -2147483648  , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_xrninta_407, bid128_to_int32_xrninta, 0xB02C00000000000AD78EBC5872141C00u128, -2147483648  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_xrninta_408, bid128_to_int32_xrninta, 0xB02C00000000000AD78EBC5872141C01u128, -2147483648  , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_xrninta_409, bid128_to_int32_xrninta, 0xB02C00000000000AD78EBC5BF025F1FFu128, -2147483648  , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_xrninta_410, bid128_to_int32_xrninta, 0xB02C00000000000AD78EBC5BF025F200u128, -2147483648  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_xrninta_411, bid128_to_int32_xrninta, 0xB02C00000000000AD78EBC5BF025F201u128, -2147483648  , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_xrninta_412, bid128_to_int32_xrninta, 0xB02C00000000000AD78EBC5E4431D5FFu128, -2147483648  , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_xrninta_413, bid128_to_int32_xrninta, 0xB02C00000000000AD78EBC5E4431D600u128, -2147483648  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_xrninta_414, bid128_to_int32_xrninta, 0xB02C00000000000AD78EBC5E4431D601u128, -2147483648  , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_xrninta_415, bid128_to_int32_xrninta, 0xB02E000000000001158E46094F6AC9FFu128, -2147483648  , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_xrninta_416, bid128_to_int32_xrninta, 0xB02E000000000001158E46094F6ACA00u128, -2147483648  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_xrninta_417, bid128_to_int32_xrninta, 0xB02E000000000001158E46094F6ACA01u128, -2147483648  , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_xrninta_418, bid128_to_int32_xrninta, 0xB03000000000000000000006FC23ABFFu128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_419, bid128_to_int32_xrninta, 0xB03000000000000000000006FC23AC00u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_420, bid128_to_int32_xrninta, 0xB03000000000000000000006FC23AC01u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_421, bid128_to_int32_xrninta, 0xB03200000000000000000000B2D05DFFu128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_422, bid128_to_int32_xrninta, 0xB03200000000000000000000B2D05E00u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_423, bid128_to_int32_xrninta, 0xB03200000000000000000000B2D05E01u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_424, bid128_to_int32_xrninta, 0xB03800000000000000000000002DDA47u128, -300         , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrninta_425, bid128_to_int32_xrninta, 0xB03800000000000000000000002DDA48u128, -301         , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrninta_426, bid128_to_int32_xrninta, 0xB03800000000000000000000002DDA49u128, -301         , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrninta_427, bid128_to_int32_xrninta, 0xB03A00000000000000000000000003E7u128, -1           , 0x20); // -- -(0.999)
dec_test!(bid128_to_int32_xrninta_428, bid128_to_int32_xrninta, 0xB03A00000000000000000000000495D3u128, -300         , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrninta_429, bid128_to_int32_xrninta, 0xB03A00000000000000000000000495D4u128, -301         , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrninta_430, bid128_to_int32_xrninta, 0xB03A00000000000000000000000495D5u128, -301         , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrninta_431, bid128_to_int32_xrninta, 0xB03C0000000000000000000000007561u128, -300         , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrninta_432, bid128_to_int32_xrninta, 0xB03C0000000000000000000000007562u128, -301         , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrninta_433, bid128_to_int32_xrninta, 0xB03C0000000000000000000000007563u128, -301         , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrninta_434, bid128_to_int32_xrninta, 0xB03E0000000000000000000000000005u128, -1           , 0x20); // -- -(0.5)
dec_test!(bid128_to_int32_xrninta_435, bid128_to_int32_xrninta, 0xB03E000000000000000000000000000Fu128, -2           , 0x20); // -- -(1.5)
dec_test!(bid128_to_int32_xrninta_436, bid128_to_int32_xrninta, 0xB03E0000000000000000000000000BB7u128, -300         , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_437, bid128_to_int32_xrninta, 0xB03E0000000000000000000000000BB8u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_438, bid128_to_int32_xrninta, 0xB03E0000000000000000000000000BB9u128, -300         , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_439, bid128_to_int32_xrninta, 0xB03E0000000000000000000000000BBDu128, -301         , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrninta_440, bid128_to_int32_xrninta, 0xB03E00000000000000000004FFFFFFF1u128, -2147483647  , 0x20); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_xrninta_441, bid128_to_int32_xrninta, 0xB03E00000000000000000004FFFFFFFBu128, -2147483648  , 0x20); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_xrninta_442, bid128_to_int32_xrninta, 0xB03E0000000000000000000500000005u128, -2147483648  , 0x01); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_xrninta_443, bid128_to_int32_xrninta, 0xB03E00000000000000000009FFFFFFFBu128, -2147483648  , 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_xrninta_444, bid128_to_int32_xrninta, 0xB03E0000000000000000000A00000005u128, -2147483648  , 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_xrninta_445, bid128_to_int32_xrninta, 0xB03E0000000000000000002E90EDCFF1u128, -2147483648  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_xrninta_446, bid128_to_int32_xrninta, 0xB03E0000000000000000002E90EDCFFBu128, -2147483648  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_xrninta_447, bid128_to_int32_xrninta, 0xB03E0000000000000000002E90EDD005u128, -2147483648  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_xrninta_448, bid128_to_int32_xrninta, 0xB03E0000000000000000002E90EDD00Fu128, -2147483648  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_xrninta_449, bid128_to_int32_xrninta, 0xB0400000000000000000000000000001u128, -1           , 0x00); // -- -(1)
dec_test!(bid128_to_int32_xrninta_450, bid128_to_int32_xrninta, 0xB040000000000000000000000000012Bu128, -299         , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_451, bid128_to_int32_xrninta, 0xB040000000000000000000000000012Cu128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_452, bid128_to_int32_xrninta, 0xB040000000000000000000000000012Du128, -301         , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_453, bid128_to_int32_xrninta, 0xB040000000000000000000007FFFFFFFu128, -2147483647  , 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_xrninta_454, bid128_to_int32_xrninta, 0xB0400000000000000000000080000000u128, -2147483648  , 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_xrninta_455, bid128_to_int32_xrninta, 0xB0400000000000000000000080000001u128, -2147483648  , 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_xrninta_456, bid128_to_int32_xrninta, 0xB04000000000000000000000FFFFFFFFu128, -2147483648  , 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_xrninta_457, bid128_to_int32_xrninta, 0xB0400000000000000000000100000000u128, -2147483648  , 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_xrninta_458, bid128_to_int32_xrninta, 0xB0400000000000000000000100000001u128, -2147483648  , 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_xrninta_459, bid128_to_int32_xrninta, 0xB04000000000000000000004A817C7FFu128, -2147483648  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_xrninta_460, bid128_to_int32_xrninta, 0xB04000000000000000000004A817C801u128, -2147483648  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_xrninta_461, bid128_to_int32_xrninta, 0xB042000000000000000000000000001Du128, -290         , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrninta_462, bid128_to_int32_xrninta, 0xB042000000000000000000000000001Eu128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_463, bid128_to_int32_xrninta, 0xB042000000000000000000000000001Fu128, -310         , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrninta_464, bid128_to_int32_xrninta, 0xB04200000000000000000000773593FFu128, -2147483648  , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_xrninta_465, bid128_to_int32_xrninta, 0xB0420000000000000000000077359400u128, -2147483648  , 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_xrninta_466, bid128_to_int32_xrninta, 0xB0420000000000000000000077359401u128, -2147483648  , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_xrninta_467, bid128_to_int32_xrninta, 0xB0440000000000000000000000000003u128, -300         , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrninta_468, bid128_to_int32_xrninta, 0xB0520000000000000000000000000004u128, -2147483648  , 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_xrninta_469, bid128_to_int32_xrninta, 0xB0520000000000000000000000000005u128, -2147483648  , 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_xrninta_470, bid128_to_int32_xrninta, 0xB0540000000000000000000000000002u128, -2147483648  , 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_xrninta_471, bid128_to_int32_xrninta, 0xbaec0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_int32_xrninta_472, bid128_to_int32_xrninta, 0xc854bd58deaabdd1be52f52fb5f30bacu128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_473, bid128_to_int32_xrninta, 0xceee0000000000000000000000000000u128, 0            , 0x00);
dec_test!(bid128_to_int32_xrninta_474, bid128_to_int32_xrninta, 0xd9529328105beb741bdf9fc4ae540033u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_475, bid128_to_int32_xrninta, 0xda8ef6e3679a94585313659fe16c3f11u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_476, bid128_to_int32_xrninta, 0xdffefdffffefffd7bb86ffe79787ce96u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_477, bid128_to_int32_xrninta, 0xfbfcfff7bfbfffdfb410600ae6233809u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_478, bid128_to_int32_xrninta, 0xfda236319c5db2cfd062b9a8e7239c9au128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_479, bid128_to_int32_xrninta, 0xff6dffffcfeffffb3800083200110200u128, -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_480, bid128_to_int32_xrninta, "-Infinity"                           , -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_481, bid128_to_int32_xrninta, "Infinity"                            , -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_482, bid128_to_int32_xrninta, "QNaN"                                , -2147483648  , 0x01);
dec_test!(bid128_to_int32_xrninta_483, bid128_to_int32_xrninta, "SNaN"                                , -2147483648  , 0x01);
