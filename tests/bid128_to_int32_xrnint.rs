/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int32_xrnint_001, bid128_to_int32_xrnint, "-0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_002, bid128_to_int32_xrnint,  "0"                                  , 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_003, bid128_to_int32_xrnint, 0x00000000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_004, bid128_to_int32_xrnint, 0x00000000000000000000800400021000u128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_005, bid128_to_int32_xrnint, "-0.000010E0"                         , 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_006, bid128_to_int32_xrnint, 0x0001ed09bead87c0378d8e62ffffffffu128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_007, bid128_to_int32_xrnint, 0x0001ed09bead87c0378d8e64ffffffffu128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_008, bid128_to_int32_xrnint, 0x04000000000000003ad8487f5e2917fdu128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_009, bid128_to_int32_xrnint, "0.5"                                 , 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_010, bid128_to_int32_xrnint, 0x09312878d47281dd70d14704f89319bfu128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_011, bid128_to_int32_xrnint, "+1.001010010E0"                      , 1          , 0x20);
dec_test!(bid128_to_int32_xrnint_012, bid128_to_int32_xrnint, "1.0"                                 , 1          , 0x00);
dec_test!(bid128_to_int32_xrnint_013, bid128_to_int32_xrnint, "1073741824"                          , 1073741824 , 0x00);
dec_test!(bid128_to_int32_xrnint_014, bid128_to_int32_xrnint, "1"                                   , 1          , 0x00);
dec_test!(bid128_to_int32_xrnint_015, bid128_to_int32_xrnint, 0x12e159e71348a0467f9b7792e493653bu128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_016, bid128_to_int32_xrnint, 0x1964644cc704e1243acc322f3ec7d581u128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_017, bid128_to_int32_xrnint, 0x197173d7fa5c9ca09410d762b8f5fd85u128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_018, bid128_to_int32_xrnint, 0x2082ad8c2be9375e95db30737e42d995u128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_019, bid128_to_int32_xrnint, "2147483648"                          , -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_020, bid128_to_int32_xrnint, 0x2b204dd6afa790061f427e41a64e1da9u128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_021, bid128_to_int32_xrnint, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_int32_xrnint_022, bid128_to_int32_xrnint, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0          , 0x20); // -- 0.5
dec_test!(bid128_to_int32_xrnint_023, bid128_to_int32_xrnint, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1          , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_int32_xrnint_024, bid128_to_int32_xrnint, 0x2ffd7dd9f7f7bdb88e757266c4d548a8u128, 1          , 0x20);
dec_test!(bid128_to_int32_xrnint_025, bid128_to_int32_xrnint, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1          , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_int32_xrnint_026, bid128_to_int32_xrnint, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1          , 0x20); // -- 0.999
dec_test!(bid128_to_int32_xrnint_027, bid128_to_int32_xrnint, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1          , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_int32_xrnint_028, bid128_to_int32_xrnint, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1          , 0x20); // -- 1-ulp
dec_test!(bid128_to_int32_xrnint_029, bid128_to_int32_xrnint, 0x2FFE314DC6448D9338C15B0A00000000u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_xrnint_030, bid128_to_int32_xrnint, 0x2FFE314DC6448D9338C15B0A00000001u128, 1          , 0x20); // -- 1+ulp
dec_test!(bid128_to_int32_xrnint_031, bid128_to_int32_xrnint, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1          , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_int32_xrnint_032, bid128_to_int32_xrnint, 0x2FFE49F4A966D45CD522088F00000000u128, 2          , 0x20); // -- 1.5
dec_test!(bid128_to_int32_xrnint_033, bid128_to_int32_xrnint, 0x2FFE49F4A966D45CD522088F00000001u128, 2          , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_int32_xrnint_034, bid128_to_int32_xrnint, 0x2ffedf7ff2b6757f10210330080c0084u128, 5          , 0x20);
dec_test!(bid128_to_int32_xrnint_035, bid128_to_int32_xrnint, 0x300000a410024210e679af471102f6adu128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_036, bid128_to_int32_xrnint, 0x3000281880001200f64ebf491de79cffu128, 8          , 0x20);
dec_test!(bid128_to_int32_xrnint_037, bid128_to_int32_xrnint, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_038, bid128_to_int32_xrnint, 0x300293E952CDA8B9AA44111E00000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_039, bid128_to_int32_xrnint, 0x300293E952CDA8B9AA44111E00000001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_040, bid128_to_int32_xrnint, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrnint_041, bid128_to_int32_xrnint, 0x300294286EACB8CB0A8CB6B140000000u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrnint_042, bid128_to_int32_xrnint, 0x300294286EACB8CB0A8CB6B140000001u128, 301        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrnint_043, bid128_to_int32_xrnint, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_044, bid128_to_int32_xrnint, 0x30040ECA8847C4129106CE8300000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_045, bid128_to_int32_xrnint, 0x30040ECA8847C4129106CE8300000001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_046, bid128_to_int32_xrnint, 0x30094cd57238fe033e1f2a916276af0cu128, 675067     , 0x20);
dec_test!(bid128_to_int32_xrnint_047, bid128_to_int32_xrnint, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_048, bid128_to_int32_xrnint, 0x300A0003C95A2F0B4856475FE0000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_049, bid128_to_int32_xrnint, 0x300A0003C95A2F0B4856475FE0000001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_050, bid128_to_int32_xrnint, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_051, bid128_to_int32_xrnint, 0x300C000060EF6B1ABA6F072330000000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_052, bid128_to_int32_xrnint, 0x300C000060EF6B1ABA6F072330000001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_053, bid128_to_int32_xrnint, 0x30104804421145ad299fa79f950f9f6cu128, 1460670851 , 0x20);
dec_test!(bid128_to_int32_xrnint_054, bid128_to_int32_xrnint, 0x301069E10DE628D3A6C9CC9B8E7FFFFFu128, 2147483646 , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_xrnint_055, bid128_to_int32_xrnint, 0x301069E10DE628D3A6C9CC9B8E800000u128, 2147483646 , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_int32_xrnint_056, bid128_to_int32_xrnint, 0x301069E10DE628D3A6C9CC9B8E800001u128, 2147483647 , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_xrnint_057, bid128_to_int32_xrnint, 0x301069E10DE692B4B4B133125EFFFFFFu128, 2147483647 , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_xrnint_058, bid128_to_int32_xrnint, 0x301069E10DE692B4B4B133125F000000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_xrnint_059, bid128_to_int32_xrnint, 0x301069E10DE692B4B4B133125F000001u128, 2147483647 , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_xrnint_060, bid128_to_int32_xrnint, 0x301069E10DE6FC95C29899892F7FFFFFu128, 2147483647 , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_xrnint_061, bid128_to_int32_xrnint, 0x301069E10DE6FC95C29899892F800000u128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_xrnint_062, bid128_to_int32_xrnint, 0x301069E10DE6FC95C29899892F800001u128, -2147483648, 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_xrnint_063, bid128_to_int32_xrnint, 0x301069E10DE76676D07FFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_xrnint_064, bid128_to_int32_xrnint, 0x301069E10DE76676D080000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_xrnint_065, bid128_to_int32_xrnint, 0x301069E10DE76676D080000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_xrnint_066, bid128_to_int32_xrnint, 0x301069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_xrnint_067, bid128_to_int32_xrnint, 0x301069E10DE7D057DE676676D0800000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_xrnint_068, bid128_to_int32_xrnint, 0x301069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_xrnint_069, bid128_to_int32_xrnint, 0x301069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_xrnint_070, bid128_to_int32_xrnint, 0x301069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_xrnint_071, bid128_to_int32_xrnint, 0x301069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_xrnint_072, bid128_to_int32_xrnint, 0x3010a9882104280116800000841a2000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_073, bid128_to_int32_xrnint, 0x3010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- 4e9-ulp
dec_test!(bid128_to_int32_xrnint_074, bid128_to_int32_xrnint, 0x3010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_xrnint_075, bid128_to_int32_xrnint, 0x3010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- 4e9+ulp
dec_test!(bid128_to_int32_xrnint_076, bid128_to_int32_xrnint, 0x3010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_xrnint_077, bid128_to_int32_xrnint, 0x3010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_xrnint_078, bid128_to_int32_xrnint, 0x3010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_xrnint_079, bid128_to_int32_xrnint, 0x3010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_xrnint_080, bid128_to_int32_xrnint, 0x3010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_xrnint_081, bid128_to_int32_xrnint, 0x3010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_xrnint_082, bid128_to_int32_xrnint, 0x3010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_xrnint_083, bid128_to_int32_xrnint, 0x3010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_xrnint_084, bid128_to_int32_xrnint, 0x3010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_xrnint_085, bid128_to_int32_xrnint, 0x3010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_xrnint_086, bid128_to_int32_xrnint, 0x3010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_xrnint_087, bid128_to_int32_xrnint, 0x3010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_xrnint_088, bid128_to_int32_xrnint, 0x3010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_xrnint_089, bid128_to_int32_xrnint, 0x3010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_xrnint_090, bid128_to_int32_xrnint, 0x3010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_xrnint_091, bid128_to_int32_xrnint, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- 5e9-ulp
dec_test!(bid128_to_int32_xrnint_092, bid128_to_int32_xrnint, 0x3010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_xrnint_093, bid128_to_int32_xrnint, 0x3010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- 5e9+ulp
dec_test!(bid128_to_int32_xrnint_094, bid128_to_int32_xrnint, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_xrnint_095, bid128_to_int32_xrnint, 0x3012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_xrnint_096, bid128_to_int32_xrnint, 0x3012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_xrnint_097, bid128_to_int32_xrnint, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_xrnint_098, bid128_to_int32_xrnint, 0x3012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_xrnint_099, bid128_to_int32_xrnint, 0x3012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_xrnint_100, bid128_to_int32_xrnint, 0x3012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_xrnint_101, bid128_to_int32_xrnint, 0x3012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_xrnint_102, bid128_to_int32_xrnint, 0x3012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_xrnint_103, bid128_to_int32_xrnint, 0x3012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_xrnint_104, bid128_to_int32_xrnint, 0x3012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_xrnint_105, bid128_to_int32_xrnint, 0x3012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_xrnint_106, bid128_to_int32_xrnint, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_xrnint_107, bid128_to_int32_xrnint, 0x3012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_xrnint_108, bid128_to_int32_xrnint, 0x3012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_xrnint_109, bid128_to_int32_xrnint, 0x3012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_xrnint_110, bid128_to_int32_xrnint, 0x3012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_xrnint_111, bid128_to_int32_xrnint, 0x3012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_xrnint_112, bid128_to_int32_xrnint, 0x3012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_xrnint_113, bid128_to_int32_xrnint, 0x3012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_xrnint_114, bid128_to_int32_xrnint, 0x3012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_xrnint_115, bid128_to_int32_xrnint, 0x30140007089c511015ba1fe18a16e0cau128, 55726199   , 0x20);
dec_test!(bid128_to_int32_xrnint_116, bid128_to_int32_xrnint, 0x30140080006c02000217500003081000u128, 1014133538 , 0x20);
dec_test!(bid128_to_int32_xrnint_117, bid128_to_int32_xrnint, 0x30180002B5E3AF0E8FDCF2BBEB67FFFFu128, 2147483646 , 0x20); // -- 2^31-1.5-ulp
dec_test!(bid128_to_int32_xrnint_118, bid128_to_int32_xrnint, 0x30180002B5E3AF0E8FDCF2BBEB680000u128, 2147483646 , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_int32_xrnint_119, bid128_to_int32_xrnint, 0x30180002B5E3AF0E8FDCF2BBEB680001u128, 2147483647 , 0x20); // -- 2^31-1.5+ulp
dec_test!(bid128_to_int32_xrnint_120, bid128_to_int32_xrnint, 0x30180002B5E3AF13FBA450E94E77FFFFu128, 2147483647 , 0x20); // -- 2^31-0.5-ulp
dec_test!(bid128_to_int32_xrnint_121, bid128_to_int32_xrnint, 0x30180002B5E3AF13FBA450E94E780000u128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_xrnint_122, bid128_to_int32_xrnint, 0x30180002B5E3AF13FBA450E94E780001u128, -2147483648, 0x01); // -- 2^31-0.5+ulp
dec_test!(bid128_to_int32_xrnint_123, bid128_to_int32_xrnint, 0x30180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x01); // -- 2^31+0.5-ulp
dec_test!(bid128_to_int32_xrnint_124, bid128_to_int32_xrnint, 0x30180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_xrnint_125, bid128_to_int32_xrnint, 0x30180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- 2^31+0.5+ulp
dec_test!(bid128_to_int32_xrnint_126, bid128_to_int32_xrnint, 0x301800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- 2^32-0.5-ulp
dec_test!(bid128_to_int32_xrnint_127, bid128_to_int32_xrnint, 0x301800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_xrnint_128, bid128_to_int32_xrnint, 0x301800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- 2^32-0.5+ulp
dec_test!(bid128_to_int32_xrnint_129, bid128_to_int32_xrnint, 0x301800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- 2^32+0.5-ulp
dec_test!(bid128_to_int32_xrnint_130, bid128_to_int32_xrnint, 0x301800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_xrnint_131, bid128_to_int32_xrnint, 0x301800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- 2^32+0.5+ulp
dec_test!(bid128_to_int32_xrnint_132, bid128_to_int32_xrnint, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrnint_133, bid128_to_int32_xrnint, 0x301A0000000000A2E6C09AD3E0D40000u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrnint_134, bid128_to_int32_xrnint, 0x301A0000000000A2E6C09AD3E0D40001u128, 301        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrnint_135, bid128_to_int32_xrnint, 0x301A000045639181BA2CDCFB7617FFFFu128, 2147483647 , 0x20); // -- 2^31-1-ulp
dec_test!(bid128_to_int32_xrnint_136, bid128_to_int32_xrnint, 0x301A000045639181BA2CDCFB76180000u128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_xrnint_137, bid128_to_int32_xrnint, 0x301A000045639181BA2CDCFB76180001u128, 2147483647 , 0x20); // -- 2^31-1+ulp
dec_test!(bid128_to_int32_xrnint_138, bid128_to_int32_xrnint, 0x301A00004563918244F3FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^31-ulp
dec_test!(bid128_to_int32_xrnint_139, bid128_to_int32_xrnint, 0x301A00004563918244F4000000000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_xrnint_140, bid128_to_int32_xrnint, 0x301A00004563918244F4000000000001u128, -2147483648, 0x01); // -- 2^31+ulp
dec_test!(bid128_to_int32_xrnint_141, bid128_to_int32_xrnint, 0x301A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- 2^31+1-ulp
dec_test!(bid128_to_int32_xrnint_142, bid128_to_int32_xrnint, 0x301A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_xrnint_143, bid128_to_int32_xrnint, 0x301A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- 2^31+1+ulp
dec_test!(bid128_to_int32_xrnint_144, bid128_to_int32_xrnint, 0x301A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- 2^32-1-ulp
dec_test!(bid128_to_int32_xrnint_145, bid128_to_int32_xrnint, 0x301A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_xrnint_146, bid128_to_int32_xrnint, 0x301A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- 2^32-1+ulp
dec_test!(bid128_to_int32_xrnint_147, bid128_to_int32_xrnint, 0x301A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- 2^32-ulp
dec_test!(bid128_to_int32_xrnint_148, bid128_to_int32_xrnint, 0x301A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_xrnint_149, bid128_to_int32_xrnint, 0x301A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- 2^32+ulp
dec_test!(bid128_to_int32_xrnint_150, bid128_to_int32_xrnint, 0x301A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- 2^32+1-ulp
dec_test!(bid128_to_int32_xrnint_151, bid128_to_int32_xrnint, 0x301A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_xrnint_152, bid128_to_int32_xrnint, 0x301A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- 2^32+1+ulp
dec_test!(bid128_to_int32_xrnint_153, bid128_to_int32_xrnint, 0x301E000000000001A055690D9DB7FFFFu128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_154, bid128_to_int32_xrnint, 0x301E000000000001A055690D9DB80000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_155, bid128_to_int32_xrnint, 0x301E000000000001A055690D9DB80001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_156, bid128_to_int32_xrnint, 0x302000000000000029A2241AF62BFFFFu128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_157, bid128_to_int32_xrnint, 0x302000000000000029A2241AF62C0000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_158, bid128_to_int32_xrnint, 0x302000000000000029A2241AF62C0001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_159, bid128_to_int32_xrnint, 0x3020000000000000ffffffffffffefb7u128, 1845       , 0x20);
dec_test!(bid128_to_int32_xrnint_160, bid128_to_int32_xrnint, 0x302000000050005084a563b86fb9677du128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_161, bid128_to_int32_xrnint, 0x3024000000000000006A94D74F42FFFFu128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_162, bid128_to_int32_xrnint, 0x3024000000000000006A94D74F430000u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_163, bid128_to_int32_xrnint, 0x3024000000000000006A94D74F430001u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_164, bid128_to_int32_xrnint, 0x302A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int32_xrnint_165, bid128_to_int32_xrnint, 0x302A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_xrnint_166, bid128_to_int32_xrnint, 0x302A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int32_xrnint_167, bid128_to_int32_xrnint, 0x302A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int32_xrnint_168, bid128_to_int32_xrnint, 0x302A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_xrnint_169, bid128_to_int32_xrnint, 0x302A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int32_xrnint_170, bid128_to_int32_xrnint, 0x302C000000000000000002BBA7F521FFu128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrnint_171, bid128_to_int32_xrnint, 0x302C000000000000000002BBA7F52200u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrnint_172, bid128_to_int32_xrnint, 0x302C000000000000000002BBA7F52201u128, 301        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrnint_173, bid128_to_int32_xrnint, 0x302C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- 2e10-1-ulp
dec_test!(bid128_to_int32_xrnint_174, bid128_to_int32_xrnint, 0x302C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_xrnint_175, bid128_to_int32_xrnint, 0x302C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- 2e10-1+ulp
dec_test!(bid128_to_int32_xrnint_176, bid128_to_int32_xrnint, 0x302C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int32_xrnint_177, bid128_to_int32_xrnint, 0x302C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_xrnint_178, bid128_to_int32_xrnint, 0x302C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int32_xrnint_179, bid128_to_int32_xrnint, 0x302C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int32_xrnint_180, bid128_to_int32_xrnint, 0x302C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_xrnint_181, bid128_to_int32_xrnint, 0x302C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int32_xrnint_182, bid128_to_int32_xrnint, 0x302E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- 2e10+1-ulp
dec_test!(bid128_to_int32_xrnint_183, bid128_to_int32_xrnint, 0x302E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_xrnint_184, bid128_to_int32_xrnint, 0x302E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- 2e10+1+ulp
dec_test!(bid128_to_int32_xrnint_185, bid128_to_int32_xrnint, 0x303000000000000000000006FC23ABFFu128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_186, bid128_to_int32_xrnint, 0x303000000000000000000006FC23AC00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_187, bid128_to_int32_xrnint, 0x303000000000000000000006FC23AC01u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_188, bid128_to_int32_xrnint, 0x303200000000000000000000B2D05DFFu128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_189, bid128_to_int32_xrnint, 0x303200000000000000000000B2D05E00u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_190, bid128_to_int32_xrnint, 0x303200000000000000000000B2D05E01u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_191, bid128_to_int32_xrnint, 0x303800000000000000000000002DDA47u128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrnint_192, bid128_to_int32_xrnint, 0x303800000000000000000000002DDA48u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrnint_193, bid128_to_int32_xrnint, 0x303800000000000000000000002DDA49u128, 301        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrnint_194, bid128_to_int32_xrnint, 0x303A00000000000000000000000003E7u128, 1          , 0x20); // -- 0.999
dec_test!(bid128_to_int32_xrnint_195, bid128_to_int32_xrnint, 0x303A00000000000000000000000495D3u128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrnint_196, bid128_to_int32_xrnint, 0x303A00000000000000000000000495D4u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrnint_197, bid128_to_int32_xrnint, 0x303A00000000000000000000000495D5u128, 301        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrnint_198, bid128_to_int32_xrnint, 0x303C0000000000000000000000007561u128, 300        , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int32_xrnint_199, bid128_to_int32_xrnint, 0x303C0000000000000000000000007562u128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrnint_200, bid128_to_int32_xrnint, 0x303C0000000000000000000000007563u128, 301        , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int32_xrnint_201, bid128_to_int32_xrnint, 0x303E0000000000000000000000000005u128, 0          , 0x20); // -- 0.5
dec_test!(bid128_to_int32_xrnint_202, bid128_to_int32_xrnint, 0x303E000000000000000000000000000Fu128, 2          , 0x20); // -- 1.5
dec_test!(bid128_to_int32_xrnint_203, bid128_to_int32_xrnint, 0x303E0000000000000000000000000BB7u128, 300        , 0x20); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_204, bid128_to_int32_xrnint, 0x303E0000000000000000000000000BB8u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_205, bid128_to_int32_xrnint, 0x303E0000000000000000000000000BB9u128, 300        , 0x20); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_206, bid128_to_int32_xrnint, 0x303E0000000000000000000000000BBDu128, 300        , 0x20); // -- 300.5
dec_test!(bid128_to_int32_xrnint_207, bid128_to_int32_xrnint, 0x303E00000000000000000004FFFFFFF1u128, 2147483646 , 0x20); // -- 2^31-1.5
dec_test!(bid128_to_int32_xrnint_208, bid128_to_int32_xrnint, 0x303E00000000000000000004FFFFFFFBu128, -2147483648, 0x01); // -- 2^31-0.5
dec_test!(bid128_to_int32_xrnint_209, bid128_to_int32_xrnint, 0x303E0000000000000000000500000005u128, -2147483648, 0x01); // -- 2^31+0.5
dec_test!(bid128_to_int32_xrnint_210, bid128_to_int32_xrnint, 0x303E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- 2^32-0.5
dec_test!(bid128_to_int32_xrnint_211, bid128_to_int32_xrnint, 0x303E0000000000000000000A00000005u128, -2147483648, 0x01); // -- 2^32+0.5
dec_test!(bid128_to_int32_xrnint_212, bid128_to_int32_xrnint, 0x303E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- 2e10-1.5
dec_test!(bid128_to_int32_xrnint_213, bid128_to_int32_xrnint, 0x303E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- 2e10-0.5
dec_test!(bid128_to_int32_xrnint_214, bid128_to_int32_xrnint, 0x303E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- 2e10+0.5
dec_test!(bid128_to_int32_xrnint_215, bid128_to_int32_xrnint, 0x303E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- 2e10+1.5
dec_test!(bid128_to_int32_xrnint_216, bid128_to_int32_xrnint, 0x30400000000000000000000000000001u128, 1          , 0x00); // -- 1
dec_test!(bid128_to_int32_xrnint_217, bid128_to_int32_xrnint, 0x3040000000000000000000000000012Bu128, 299        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_218, bid128_to_int32_xrnint, 0x3040000000000000000000000000012Cu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_219, bid128_to_int32_xrnint, 0x3040000000000000000000000000012Du128, 301        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_220, bid128_to_int32_xrnint, 0x3040000000000000000000007FFFFFFFu128, 2147483647 , 0x00); // -- 2^31-1
dec_test!(bid128_to_int32_xrnint_221, bid128_to_int32_xrnint, 0x30400000000000000000000080000000u128, -2147483648, 0x01); // -- 2^31
dec_test!(bid128_to_int32_xrnint_222, bid128_to_int32_xrnint, 0x30400000000000000000000080000001u128, -2147483648, 0x01); // -- 2^31+1
dec_test!(bid128_to_int32_xrnint_223, bid128_to_int32_xrnint, 0x304000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- 2^32-1
dec_test!(bid128_to_int32_xrnint_224, bid128_to_int32_xrnint, 0x30400000000000000000000100000000u128, -2147483648, 0x01); // -- 2^32
dec_test!(bid128_to_int32_xrnint_225, bid128_to_int32_xrnint, 0x30400000000000000000000100000001u128, -2147483648, 0x01); // -- 2^32+1
dec_test!(bid128_to_int32_xrnint_226, bid128_to_int32_xrnint, 0x304000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- 2e10-1
dec_test!(bid128_to_int32_xrnint_227, bid128_to_int32_xrnint, 0x304000000000000000000004A817C801u128, -2147483648, 0x01); // -- 2e10+1
dec_test!(bid128_to_int32_xrnint_228, bid128_to_int32_xrnint, 0x3041ED09BEAD87C0378D8E6400000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_229, bid128_to_int32_xrnint, 0x3042000000000000000000000000001Du128, 290        , 0x00); // -- 300-ulp
dec_test!(bid128_to_int32_xrnint_230, bid128_to_int32_xrnint, 0x3042000000000000000000000000001Eu128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_231, bid128_to_int32_xrnint, 0x3042000000000000000000000000001Fu128, 310        , 0x00); // -- 300+ulp
dec_test!(bid128_to_int32_xrnint_232, bid128_to_int32_xrnint, 0x304200000000000000000000773593FFu128, -2147483648, 0x01); // -- 2e10-ulp
dec_test!(bid128_to_int32_xrnint_233, bid128_to_int32_xrnint, 0x30420000000000000000000077359400u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_xrnint_234, bid128_to_int32_xrnint, 0x30420000000000000000000077359401u128, -2147483648, 0x01); // -- 2e10+ulp
dec_test!(bid128_to_int32_xrnint_235, bid128_to_int32_xrnint, 0x30440000000000000000000000000003u128, 300        , 0x00); // -- 300
dec_test!(bid128_to_int32_xrnint_236, bid128_to_int32_xrnint, 0x30520000000000000000000000000004u128, -2147483648, 0x01); // -- 4e9
dec_test!(bid128_to_int32_xrnint_237, bid128_to_int32_xrnint, 0x30520000000000000000000000000005u128, -2147483648, 0x01); // -- 5e9
dec_test!(bid128_to_int32_xrnint_238, bid128_to_int32_xrnint, 0x30540000000000000000000000000002u128, -2147483648, 0x01); // -- 2e10
dec_test!(bid128_to_int32_xrnint_239, bid128_to_int32_xrnint, 0x38860000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_240, bid128_to_int32_xrnint, 0x412db094345f81edb0aaf73587ef2621u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_241, bid128_to_int32_xrnint, 0x48e6cd306f4cdf48c1c1cd461e942ef2u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_242, bid128_to_int32_xrnint, "5.05"                                , 5          , 0x20);
dec_test!(bid128_to_int32_xrnint_243, bid128_to_int32_xrnint, 0x52ac3467cbc0e4cd5fefaefd4dfddd69u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_244, bid128_to_int32_xrnint, "5.5"                                 , 6          , 0x20);
dec_test!(bid128_to_int32_xrnint_245, bid128_to_int32_xrnint, "+677758.88757977578559E0"            , 677759     , 0x20);
dec_test!(bid128_to_int32_xrnint_246, bid128_to_int32_xrnint, "-69558.666799957E0"                  , -69559     , 0x20);
dec_test!(bid128_to_int32_xrnint_247, bid128_to_int32_xrnint, "+695979986.76578E0"                  , 695979987  , 0x20);
dec_test!(bid128_to_int32_xrnint_248, bid128_to_int32_xrnint, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_249, bid128_to_int32_xrnint, 0x78000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_250, bid128_to_int32_xrnint, 0x78b3620c6b5eae70ee9bcfbf7dfb956fu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_251, bid128_to_int32_xrnint, 0x7c000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_252, bid128_to_int32_xrnint, 0x7c003fffffffffff38c15b08ffffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_253, bid128_to_int32_xrnint, 0x7c003fffffffffff38c15b0affffffffu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_254, bid128_to_int32_xrnint, 0x7e000000000000000000000000000000u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_255, bid128_to_int32_xrnint, 0x856a0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_256, bid128_to_int32_xrnint, 0x8cbc23f62c90b8c47a40737ba2d26303u128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_257, bid128_to_int32_xrnint, 0x8e76ea27b03af84e400f53fa2cc87162u128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_258, bid128_to_int32_xrnint, "-9"                                  , -9         , 0x00);
dec_test!(bid128_to_int32_xrnint_259, bid128_to_int32_xrnint, 0xa14d99a44549c3613d75018813cdbda5u128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_260, bid128_to_int32_xrnint, 0xa158b78a8e8df208c161d0722c70922bu128, 0          , 0x20);
dec_test!(bid128_to_int32_xrnint_261, bid128_to_int32_xrnint, 0xa8580000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_262, bid128_to_int32_xrnint, 0xab100000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_263, bid128_to_int32_xrnint, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0          , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_int32_xrnint_264, bid128_to_int32_xrnint, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0          , 0x20); // -- -(0.5)
dec_test!(bid128_to_int32_xrnint_265, bid128_to_int32_xrnint, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1         , 0x20); // -- -(0.5+ulp)
dec_test!(bid128_to_int32_xrnint_266, bid128_to_int32_xrnint, 0xaffd7bf775ffffb40030000002000020u128, -1         , 0x20);
dec_test!(bid128_to_int32_xrnint_267, bid128_to_int32_xrnint, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1         , 0x20); // -- -(0.999-ulp)
dec_test!(bid128_to_int32_xrnint_268, bid128_to_int32_xrnint, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1         , 0x20); // -- -(0.999)
dec_test!(bid128_to_int32_xrnint_269, bid128_to_int32_xrnint, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1         , 0x20); // -- -(0.999+ulp)
dec_test!(bid128_to_int32_xrnint_270, bid128_to_int32_xrnint, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1         , 0x20); // -- -(1-ulp)
dec_test!(bid128_to_int32_xrnint_271, bid128_to_int32_xrnint, 0xAFFE314DC6448D9338C15B0A00000000u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_xrnint_272, bid128_to_int32_xrnint, 0xAFFE314DC6448D9338C15B0A00000001u128, -1         , 0x20); // -- -(1+ulp)
dec_test!(bid128_to_int32_xrnint_273, bid128_to_int32_xrnint, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1         , 0x20); // -- -(1.5-ulp)
dec_test!(bid128_to_int32_xrnint_274, bid128_to_int32_xrnint, 0xAFFE49F4A966D45CD522088F00000000u128, -2         , 0x20); // -- -(1.5)
dec_test!(bid128_to_int32_xrnint_275, bid128_to_int32_xrnint, 0xAFFE49F4A966D45CD522088F00000001u128, -2         , 0x20); // -- -(1.5+ulp)
dec_test!(bid128_to_int32_xrnint_276, bid128_to_int32_xrnint, 0xafffdff71537dfed1488400020000004u128, -10        , 0x20);
dec_test!(bid128_to_int32_xrnint_277, bid128_to_int32_xrnint, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_278, bid128_to_int32_xrnint, 0xB00293E952CDA8B9AA44111E00000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_279, bid128_to_int32_xrnint, 0xB00293E952CDA8B9AA44111E00000001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_280, bid128_to_int32_xrnint, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrnint_281, bid128_to_int32_xrnint, 0xB00294286EACB8CB0A8CB6B140000000u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrnint_282, bid128_to_int32_xrnint, 0xB00294286EACB8CB0A8CB6B140000001u128, -301       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrnint_283, bid128_to_int32_xrnint, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_284, bid128_to_int32_xrnint, 0xB0040ECA8847C4129106CE8300000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_285, bid128_to_int32_xrnint, 0xB0040ECA8847C4129106CE8300000001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_286, bid128_to_int32_xrnint, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_287, bid128_to_int32_xrnint, 0xB00A0003C95A2F0B4856475FE0000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_288, bid128_to_int32_xrnint, 0xB00A0003C95A2F0B4856475FE0000001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_289, bid128_to_int32_xrnint, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_290, bid128_to_int32_xrnint, 0xB00C000060EF6B1ABA6F072330000000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_291, bid128_to_int32_xrnint, 0xB00C000060EF6B1ABA6F072330000001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_292, bid128_to_int32_xrnint, 0xB01069E10DE628D3A6C9CC9B8E7FFFFFu128, -2147483646, 0x20); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_xrnint_293, bid128_to_int32_xrnint, 0xB01069E10DE628D3A6C9CC9B8E800000u128, -2147483646, 0x20); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_xrnint_294, bid128_to_int32_xrnint, 0xB01069E10DE628D3A6C9CC9B8E800001u128, -2147483647, 0x20); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_xrnint_295, bid128_to_int32_xrnint, 0xB01069E10DE692B4B4B133125EFFFFFFu128, -2147483647, 0x20); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_xrnint_296, bid128_to_int32_xrnint, 0xB01069E10DE692B4B4B133125F000000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_xrnint_297, bid128_to_int32_xrnint, 0xB01069E10DE692B4B4B133125F000001u128, -2147483647, 0x20); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_xrnint_298, bid128_to_int32_xrnint, 0xB01069E10DE6FC95C29899892F7FFFFFu128, -2147483647, 0x20); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_xrnint_299, bid128_to_int32_xrnint, 0xB01069E10DE6FC95C29899892F800000u128, -2147483648, 0x20); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_xrnint_300, bid128_to_int32_xrnint, 0xB01069E10DE6FC95C29899892F800001u128, -2147483648, 0x20); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_xrnint_301, bid128_to_int32_xrnint, 0xB01069E10DE76676D07FFFFFFFFFFFFFu128, -2147483648, 0x20); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_xrnint_302, bid128_to_int32_xrnint, 0xB01069E10DE76676D080000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_xrnint_303, bid128_to_int32_xrnint, 0xB01069E10DE76676D080000000000001u128, -2147483648, 0x20); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_xrnint_304, bid128_to_int32_xrnint, 0xB01069E10DE7D057DE676676D07FFFFFu128, -2147483648, 0x20); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_xrnint_305, bid128_to_int32_xrnint, 0xB01069E10DE7D057DE676676D0800000u128, -2147483648, 0x20); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_xrnint_306, bid128_to_int32_xrnint, 0xB01069E10DE7D057DE676676D0800001u128, -2147483648, 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_xrnint_307, bid128_to_int32_xrnint, 0xB01069E10DE83A38EC4ECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_xrnint_308, bid128_to_int32_xrnint, 0xB01069E10DE83A38EC4ECCEDA1000000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_xrnint_309, bid128_to_int32_xrnint, 0xB01069E10DE83A38EC4ECCEDA1000001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_xrnint_310, bid128_to_int32_xrnint, 0xB010C5371912364CE3056C27FFFFFFFFu128, -2147483648, 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_int32_xrnint_311, bid128_to_int32_xrnint, 0xB010C5371912364CE3056C2800000000u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_xrnint_312, bid128_to_int32_xrnint, 0xB010C5371912364CE3056C2800000001u128, -2147483648, 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_int32_xrnint_313, bid128_to_int32_xrnint, 0xB010D3C21BCDF92B853133125EFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_xrnint_314, bid128_to_int32_xrnint, 0xB010D3C21BCDF92B853133125F000000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_xrnint_315, bid128_to_int32_xrnint, 0xB010D3C21BCDF92B853133125F000001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_xrnint_316, bid128_to_int32_xrnint, 0xB010D3C21BCE630C931899892F7FFFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_xrnint_317, bid128_to_int32_xrnint, 0xB010D3C21BCE630C931899892F800000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_xrnint_318, bid128_to_int32_xrnint, 0xB010D3C21BCE630C931899892F800001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_xrnint_319, bid128_to_int32_xrnint, 0xB010D3C21BCECCEDA0FFFFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_xrnint_320, bid128_to_int32_xrnint, 0xB010D3C21BCECCEDA100000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_xrnint_321, bid128_to_int32_xrnint, 0xB010D3C21BCECCEDA100000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_xrnint_322, bid128_to_int32_xrnint, 0xB010D3C21BCF36CEAEE76676D07FFFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_xrnint_323, bid128_to_int32_xrnint, 0xB010D3C21BCF36CEAEE76676D0800000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_xrnint_324, bid128_to_int32_xrnint, 0xB010D3C21BCF36CEAEE76676D0800001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_xrnint_325, bid128_to_int32_xrnint, 0xB010D3C21BCFA0AFBCCECCEDA0FFFFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_xrnint_326, bid128_to_int32_xrnint, 0xB010D3C21BCFA0AFBCCECCEDA1000000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_xrnint_327, bid128_to_int32_xrnint, 0xB010D3C21BCFA0AFBCCECCEDA1000001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_xrnint_328, bid128_to_int32_xrnint, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -2147483648, 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_int32_xrnint_329, bid128_to_int32_xrnint, 0xB010F684DF56C3E01BC6C73200000000u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_xrnint_330, bid128_to_int32_xrnint, 0xB010F684DF56C3E01BC6C73200000001u128, -2147483648, 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_int32_xrnint_331, bid128_to_int32_xrnint, 0xb0112c8acfa7b0236da8aae0534fa699u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_332, bid128_to_int32_xrnint, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_xrnint_333, bid128_to_int32_xrnint, 0xB012629B8C88FB62ED56E4238E400000u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_xrnint_334, bid128_to_int32_xrnint, 0xB012629B8C88FB62ED56E4238E400001u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_xrnint_335, bid128_to_int32_xrnint, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_xrnint_336, bid128_to_int32_xrnint, 0xB012629B8C8905F96EBAD4C909800000u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_xrnint_337, bid128_to_int32_xrnint, 0xB012629B8C8905F96EBAD4C909800001u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_xrnint_338, bid128_to_int32_xrnint, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_xrnint_339, bid128_to_int32_xrnint, 0xB012629B8C89108FF01EC56E84C00000u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_xrnint_340, bid128_to_int32_xrnint, 0xB012629B8C89108FF01EC56E84C00001u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_xrnint_341, bid128_to_int32_xrnint, 0xB012629B8C891B267182B613FFFFFFFFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_xrnint_342, bid128_to_int32_xrnint, 0xB012629B8C891B267182B61400000000u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_xrnint_343, bid128_to_int32_xrnint, 0xB012629B8C891B267182B61400000001u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_xrnint_344, bid128_to_int32_xrnint, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_xrnint_345, bid128_to_int32_xrnint, 0xB012629B8C8925BCF2E6A6B97B400000u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_xrnint_346, bid128_to_int32_xrnint, 0xB012629B8C8925BCF2E6A6B97B400001u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_xrnint_347, bid128_to_int32_xrnint, 0xB012629B8C893053744A975EF67FFFFFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_xrnint_348, bid128_to_int32_xrnint, 0xB012629B8C893053744A975EF6800000u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_xrnint_349, bid128_to_int32_xrnint, 0xB012629B8C893053744A975EF6800001u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_xrnint_350, bid128_to_int32_xrnint, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_xrnint_351, bid128_to_int32_xrnint, 0xB012629B8C893AE9F5AE880471C00000u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_xrnint_352, bid128_to_int32_xrnint, 0xB012629B8C893AE9F5AE880471C00001u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_xrnint_353, bid128_to_int32_xrnint, 0xB0180002B5E3AF0E8FDCF2BBEB67FFFFu128, -2147483646, 0x20); // -- -(2^31-1.5-ulp)
dec_test!(bid128_to_int32_xrnint_354, bid128_to_int32_xrnint, 0xB0180002B5E3AF0E8FDCF2BBEB680000u128, -2147483646, 0x20); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_xrnint_355, bid128_to_int32_xrnint, 0xB0180002B5E3AF0E8FDCF2BBEB680001u128, -2147483647, 0x20); // -- -(2^31-1.5+ulp)
dec_test!(bid128_to_int32_xrnint_356, bid128_to_int32_xrnint, 0xB0180002B5E3AF13FBA450E94E77FFFFu128, -2147483647, 0x20); // -- -(2^31-0.5-ulp)
dec_test!(bid128_to_int32_xrnint_357, bid128_to_int32_xrnint, 0xB0180002B5E3AF13FBA450E94E780000u128, -2147483648, 0x20); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_xrnint_358, bid128_to_int32_xrnint, 0xB0180002B5E3AF13FBA450E94E780001u128, -2147483648, 0x20); // -- -(2^31-0.5+ulp)
dec_test!(bid128_to_int32_xrnint_359, bid128_to_int32_xrnint, 0xB0180002B5E3AF19676BAF16B187FFFFu128, -2147483648, 0x20); // -- -(2^31+0.5-ulp)
dec_test!(bid128_to_int32_xrnint_360, bid128_to_int32_xrnint, 0xB0180002B5E3AF19676BAF16B1880000u128, -2147483648, 0x20); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_xrnint_361, bid128_to_int32_xrnint, 0xB0180002B5E3AF19676BAF16B1880001u128, -2147483648, 0x01); // -- -(2^31+0.5+ulp)
dec_test!(bid128_to_int32_xrnint_362, bid128_to_int32_xrnint, 0xB01800056BC75E2AAD2C50E94E77FFFFu128, -2147483648, 0x01); // -- -(2^32-0.5-ulp)
dec_test!(bid128_to_int32_xrnint_363, bid128_to_int32_xrnint, 0xB01800056BC75E2AAD2C50E94E780000u128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_xrnint_364, bid128_to_int32_xrnint, 0xB01800056BC75E2AAD2C50E94E780001u128, -2147483648, 0x01); // -- -(2^32-0.5+ulp)
dec_test!(bid128_to_int32_xrnint_365, bid128_to_int32_xrnint, 0xB01800056BC75E3018F3AF16B187FFFFu128, -2147483648, 0x01); // -- -(2^32+0.5-ulp)
dec_test!(bid128_to_int32_xrnint_366, bid128_to_int32_xrnint, 0xB01800056BC75E3018F3AF16B1880000u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_xrnint_367, bid128_to_int32_xrnint, 0xB01800056BC75E3018F3AF16B1880001u128, -2147483648, 0x01); // -- -(2^32+0.5+ulp)
dec_test!(bid128_to_int32_xrnint_368, bid128_to_int32_xrnint, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrnint_369, bid128_to_int32_xrnint, 0xB01A0000000000A2E6C09AD3E0D40000u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrnint_370, bid128_to_int32_xrnint, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrnint_371, bid128_to_int32_xrnint, 0xB01A000045639181BA2CDCFB7617FFFFu128, -2147483647, 0x20); // -- -(2^31-1-ulp)
dec_test!(bid128_to_int32_xrnint_372, bid128_to_int32_xrnint, 0xB01A000045639181BA2CDCFB76180000u128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_xrnint_373, bid128_to_int32_xrnint, 0xB01A000045639181BA2CDCFB76180001u128, -2147483647, 0x20); // -- -(2^31-1+ulp)
dec_test!(bid128_to_int32_xrnint_374, bid128_to_int32_xrnint, 0xB01A00004563918244F3FFFFFFFFFFFFu128, -2147483648, 0x20); // -- -(2^31-ulp)
dec_test!(bid128_to_int32_xrnint_375, bid128_to_int32_xrnint, 0xB01A00004563918244F4000000000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_xrnint_376, bid128_to_int32_xrnint, 0xB01A00004563918244F4000000000001u128, -2147483648, 0x20); // -- -(2^31+ulp)
dec_test!(bid128_to_int32_xrnint_377, bid128_to_int32_xrnint, 0xB01A000045639182CFBB230489E7FFFFu128, -2147483648, 0x01); // -- -(2^31+1-ulp)
dec_test!(bid128_to_int32_xrnint_378, bid128_to_int32_xrnint, 0xB01A000045639182CFBB230489E80000u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_xrnint_379, bid128_to_int32_xrnint, 0xB01A000045639182CFBB230489E80001u128, -2147483648, 0x01); // -- -(2^31+1+ulp)
dec_test!(bid128_to_int32_xrnint_380, bid128_to_int32_xrnint, 0xB01A00008AC72303FF20DCFB7617FFFFu128, -2147483648, 0x01); // -- -(2^32-1-ulp)
dec_test!(bid128_to_int32_xrnint_381, bid128_to_int32_xrnint, 0xB01A00008AC72303FF20DCFB76180000u128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_xrnint_382, bid128_to_int32_xrnint, 0xB01A00008AC72303FF20DCFB76180001u128, -2147483648, 0x01); // -- -(2^32-1+ulp)
dec_test!(bid128_to_int32_xrnint_383, bid128_to_int32_xrnint, 0xB01A00008AC7230489E7FFFFFFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-ulp)
dec_test!(bid128_to_int32_xrnint_384, bid128_to_int32_xrnint, 0xB01A00008AC7230489E8000000000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_xrnint_385, bid128_to_int32_xrnint, 0xB01A00008AC7230489E8000000000001u128, -2147483648, 0x01); // -- -(2^32+ulp)
dec_test!(bid128_to_int32_xrnint_386, bid128_to_int32_xrnint, 0xB01A00008AC7230514AF230489E7FFFFu128, -2147483648, 0x01); // -- -(2^32+1-ulp)
dec_test!(bid128_to_int32_xrnint_387, bid128_to_int32_xrnint, 0xB01A00008AC7230514AF230489E80000u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_xrnint_388, bid128_to_int32_xrnint, 0xB01A00008AC7230514AF230489E80001u128, -2147483648, 0x01); // -- -(2^32+1+ulp)
dec_test!(bid128_to_int32_xrnint_389, bid128_to_int32_xrnint, 0xB01E000000000001A055690D9DB7FFFFu128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_390, bid128_to_int32_xrnint, 0xB01E000000000001A055690D9DB80000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_391, bid128_to_int32_xrnint, 0xB01E000000000001A055690D9DB80001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_392, bid128_to_int32_xrnint, 0xB02000000000000029A2241AF62BFFFFu128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_393, bid128_to_int32_xrnint, 0xB02000000000000029A2241AF62C0000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_394, bid128_to_int32_xrnint, 0xB02000000000000029A2241AF62C0001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_395, bid128_to_int32_xrnint, 0xB024000000000000006A94D74F42FFFFu128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_396, bid128_to_int32_xrnint, 0xB024000000000000006A94D74F430000u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_397, bid128_to_int32_xrnint, 0xB024000000000000006A94D74F430001u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_398, bid128_to_int32_xrnint, 0xB02A00000000006C6B935B68D08DA3FFu128, -2147483648, 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int32_xrnint_399, bid128_to_int32_xrnint, 0xB02A00000000006C6B935B68D08DA400u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_xrnint_400, bid128_to_int32_xrnint, 0xB02A00000000006C6B935B68D08DA401u128, -2147483648, 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int32_xrnint_401, bid128_to_int32_xrnint, 0xB02A00000000006C6B935B8019048BFFu128, -2147483648, 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int32_xrnint_402, bid128_to_int32_xrnint, 0xB02A00000000006C6B935B8019048C00u128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_xrnint_403, bid128_to_int32_xrnint, 0xB02A00000000006C6B935B8019048C01u128, -2147483648, 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int32_xrnint_404, bid128_to_int32_xrnint, 0xB02C000000000000000002BBA7F521FFu128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrnint_405, bid128_to_int32_xrnint, 0xB02C000000000000000002BBA7F52200u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrnint_406, bid128_to_int32_xrnint, 0xB02C000000000000000002BBA7F52201u128, -301       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrnint_407, bid128_to_int32_xrnint, 0xB02C00000000000AD78EBC5872141BFFu128, -2147483648, 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int32_xrnint_408, bid128_to_int32_xrnint, 0xB02C00000000000AD78EBC5872141C00u128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_xrnint_409, bid128_to_int32_xrnint, 0xB02C00000000000AD78EBC5872141C01u128, -2147483648, 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int32_xrnint_410, bid128_to_int32_xrnint, 0xB02C00000000000AD78EBC5BF025F1FFu128, -2147483648, 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int32_xrnint_411, bid128_to_int32_xrnint, 0xB02C00000000000AD78EBC5BF025F200u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_xrnint_412, bid128_to_int32_xrnint, 0xB02C00000000000AD78EBC5BF025F201u128, -2147483648, 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int32_xrnint_413, bid128_to_int32_xrnint, 0xB02C00000000000AD78EBC5E4431D5FFu128, -2147483648, 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int32_xrnint_414, bid128_to_int32_xrnint, 0xB02C00000000000AD78EBC5E4431D600u128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_xrnint_415, bid128_to_int32_xrnint, 0xB02C00000000000AD78EBC5E4431D601u128, -2147483648, 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int32_xrnint_416, bid128_to_int32_xrnint, 0xB02E000000000001158E46094F6AC9FFu128, -2147483648, 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int32_xrnint_417, bid128_to_int32_xrnint, 0xB02E000000000001158E46094F6ACA00u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_xrnint_418, bid128_to_int32_xrnint, 0xB02E000000000001158E46094F6ACA01u128, -2147483648, 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int32_xrnint_419, bid128_to_int32_xrnint, 0xB03000000000000000000006FC23ABFFu128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_420, bid128_to_int32_xrnint, 0xB03000000000000000000006FC23AC00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_421, bid128_to_int32_xrnint, 0xB03000000000000000000006FC23AC01u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_422, bid128_to_int32_xrnint, 0xB03200000000000000000000B2D05DFFu128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_423, bid128_to_int32_xrnint, 0xB03200000000000000000000B2D05E00u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_424, bid128_to_int32_xrnint, 0xB03200000000000000000000B2D05E01u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_425, bid128_to_int32_xrnint, 0xB03800000000000000000000002DDA47u128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrnint_426, bid128_to_int32_xrnint, 0xB03800000000000000000000002DDA48u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrnint_427, bid128_to_int32_xrnint, 0xB03800000000000000000000002DDA49u128, -301       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrnint_428, bid128_to_int32_xrnint, 0xB03A00000000000000000000000003E7u128, -1         , 0x20); // -- -(0.999)
dec_test!(bid128_to_int32_xrnint_429, bid128_to_int32_xrnint, 0xB03A00000000000000000000000495D3u128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrnint_430, bid128_to_int32_xrnint, 0xB03A00000000000000000000000495D4u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrnint_431, bid128_to_int32_xrnint, 0xB03A00000000000000000000000495D5u128, -301       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrnint_432, bid128_to_int32_xrnint, 0xB03C0000000000000000000000007561u128, -300       , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int32_xrnint_433, bid128_to_int32_xrnint, 0xB03C0000000000000000000000007562u128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrnint_434, bid128_to_int32_xrnint, 0xB03C0000000000000000000000007563u128, -301       , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int32_xrnint_435, bid128_to_int32_xrnint, 0xB03E0000000000000000000000000005u128, 0          , 0x20); // -- -(0.5)
dec_test!(bid128_to_int32_xrnint_436, bid128_to_int32_xrnint, 0xB03E000000000000000000000000000Fu128, -2         , 0x20); // -- -(1.5)
dec_test!(bid128_to_int32_xrnint_437, bid128_to_int32_xrnint, 0xB03E0000000000000000000000000BB7u128, -300       , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_438, bid128_to_int32_xrnint, 0xB03E0000000000000000000000000BB8u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_439, bid128_to_int32_xrnint, 0xB03E0000000000000000000000000BB9u128, -300       , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_440, bid128_to_int32_xrnint, 0xB03E0000000000000000000000000BBDu128, -300       , 0x20); // -- -(300.5)
dec_test!(bid128_to_int32_xrnint_441, bid128_to_int32_xrnint, 0xB03E00000000000000000004FFFFFFF1u128, -2147483646, 0x20); // -- -(2^31-1.5)
dec_test!(bid128_to_int32_xrnint_442, bid128_to_int32_xrnint, 0xB03E00000000000000000004FFFFFFFBu128, -2147483648, 0x20); // -- -(2^31-0.5)
dec_test!(bid128_to_int32_xrnint_443, bid128_to_int32_xrnint, 0xB03E0000000000000000000500000005u128, -2147483648, 0x20); // -- -(2^31+0.5)
dec_test!(bid128_to_int32_xrnint_444, bid128_to_int32_xrnint, 0xB03E00000000000000000009FFFFFFFBu128, -2147483648, 0x01); // -- -(2^32-0.5)
dec_test!(bid128_to_int32_xrnint_445, bid128_to_int32_xrnint, 0xB03E0000000000000000000A00000005u128, -2147483648, 0x01); // -- -(2^32+0.5)
dec_test!(bid128_to_int32_xrnint_446, bid128_to_int32_xrnint, 0xB03E0000000000000000002E90EDCFF1u128, -2147483648, 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_int32_xrnint_447, bid128_to_int32_xrnint, 0xB03E0000000000000000002E90EDCFFBu128, -2147483648, 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_int32_xrnint_448, bid128_to_int32_xrnint, 0xB03E0000000000000000002E90EDD005u128, -2147483648, 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_int32_xrnint_449, bid128_to_int32_xrnint, 0xB03E0000000000000000002E90EDD00Fu128, -2147483648, 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_int32_xrnint_450, bid128_to_int32_xrnint, 0xB0400000000000000000000000000001u128, -1         , 0x00); // -- -(1)
dec_test!(bid128_to_int32_xrnint_451, bid128_to_int32_xrnint, 0xB040000000000000000000000000012Bu128, -299       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_452, bid128_to_int32_xrnint, 0xB040000000000000000000000000012Cu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_453, bid128_to_int32_xrnint, 0xB040000000000000000000000000012Du128, -301       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_454, bid128_to_int32_xrnint, 0xB040000000000000000000007FFFFFFFu128, -2147483647, 0x00); // -- -(2^31-1)
dec_test!(bid128_to_int32_xrnint_455, bid128_to_int32_xrnint, 0xB0400000000000000000000080000000u128, -2147483648, 0x00); // -- -(2^31)
dec_test!(bid128_to_int32_xrnint_456, bid128_to_int32_xrnint, 0xB0400000000000000000000080000001u128, -2147483648, 0x01); // -- -(2^31+1)
dec_test!(bid128_to_int32_xrnint_457, bid128_to_int32_xrnint, 0xB04000000000000000000000FFFFFFFFu128, -2147483648, 0x01); // -- -(2^32-1)
dec_test!(bid128_to_int32_xrnint_458, bid128_to_int32_xrnint, 0xB0400000000000000000000100000000u128, -2147483648, 0x01); // -- -(2^32)
dec_test!(bid128_to_int32_xrnint_459, bid128_to_int32_xrnint, 0xB0400000000000000000000100000001u128, -2147483648, 0x01); // -- -(2^32+1)
dec_test!(bid128_to_int32_xrnint_460, bid128_to_int32_xrnint, 0xB04000000000000000000004A817C7FFu128, -2147483648, 0x01); // -- -(2e10-1)
dec_test!(bid128_to_int32_xrnint_461, bid128_to_int32_xrnint, 0xB04000000000000000000004A817C801u128, -2147483648, 0x01); // -- -(2e10+1)
dec_test!(bid128_to_int32_xrnint_462, bid128_to_int32_xrnint, 0xB042000000000000000000000000001Du128, -290       , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int32_xrnint_463, bid128_to_int32_xrnint, 0xB042000000000000000000000000001Eu128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_464, bid128_to_int32_xrnint, 0xB042000000000000000000000000001Fu128, -310       , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int32_xrnint_465, bid128_to_int32_xrnint, 0xB04200000000000000000000773593FFu128, -2147483648, 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_int32_xrnint_466, bid128_to_int32_xrnint, 0xB0420000000000000000000077359400u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_xrnint_467, bid128_to_int32_xrnint, 0xB0420000000000000000000077359401u128, -2147483648, 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_int32_xrnint_468, bid128_to_int32_xrnint, 0xB0440000000000000000000000000003u128, -300       , 0x00); // -- -(300)
dec_test!(bid128_to_int32_xrnint_469, bid128_to_int32_xrnint, 0xB0520000000000000000000000000004u128, -2147483648, 0x01); // -- -(4e9)
dec_test!(bid128_to_int32_xrnint_470, bid128_to_int32_xrnint, 0xB0520000000000000000000000000005u128, -2147483648, 0x01); // -- -(5e9)
dec_test!(bid128_to_int32_xrnint_471, bid128_to_int32_xrnint, 0xB0540000000000000000000000000002u128, -2147483648, 0x01); // -- -(2e10)
dec_test!(bid128_to_int32_xrnint_472, bid128_to_int32_xrnint, 0xb7ffff7dffffffff4c81800a1452030au128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_473, bid128_to_int32_xrnint, 0xc0658ef8174407631cf27bf23ccad1e5u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_474, bid128_to_int32_xrnint, 0xc2ca0000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_475, bid128_to_int32_xrnint, 0xc3460000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_476, bid128_to_int32_xrnint, 0xd1000000000000000000000000000000u128, 0          , 0x00);
dec_test!(bid128_to_int32_xrnint_477, bid128_to_int32_xrnint, 0xd90b8f7a65532781298648fd32f0e9e5u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_478, bid128_to_int32_xrnint, 0xf86214e3a06affef004cff06ffbd67b1u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_479, bid128_to_int32_xrnint, 0xfca76effdfffe7eceefb5ffef9effeefu128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_480, bid128_to_int32_xrnint, 0xffdeff5dbbadf9fb3868950a106062a2u128, -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_481, bid128_to_int32_xrnint, "-Infinity"                           , -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_482, bid128_to_int32_xrnint, "Infinity"                            , -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_483, bid128_to_int32_xrnint, "QNaN"                                , -2147483648, 0x01);
dec_test!(bid128_to_int32_xrnint_484, bid128_to_int32_xrnint, "SNaN"                                , -2147483648, 0x01);
