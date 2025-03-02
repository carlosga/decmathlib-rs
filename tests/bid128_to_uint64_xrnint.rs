/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_uint64_xrnint_001, bid128_to_uint64_xrnint, 0                                     , 0                    , 0x00);
dec_test!(bid128_to_uint64_xrnint_002, bid128_to_uint64_xrnint, 0x00000000000000000000000000000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xrnint_003, bid128_to_uint64_xrnint, 0x00000000000000000000000000000020u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_004, bid128_to_uint64_xrnint, 0x0000000000000000040410e1e0400190u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_005, bid128_to_uint64_xrnint, 0x0001ed09bead87c0378d8e62ffffffffu128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_006, bid128_to_uint64_xrnint, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xrnint_007, bid128_to_uint64_xrnint, 0x09503cd77e46d4a0d751c3ec02aa773fu128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_008, bid128_to_uint64_xrnint, "+101000111010100.10E0"               , 101000111010100      , 0x20);
dec_test!(bid128_to_uint64_xrnint_009, bid128_to_uint64_xrnint, 0x10be0000000000000000000000000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xrnint_010, bid128_to_uint64_xrnint, 0x14bd1e0fd03a9d29047599ce25bc9369u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_011, bid128_to_uint64_xrnint, 0x15f2b42fc60cd8e59faab5545495aa46u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_012, bid128_to_uint64_xrnint, 0x177630c2db73b005b61833b53af4ddcbu128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_013, bid128_to_uint64_xrnint, 0x1c202a896acbaec7934698fa73e6d328u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_014, bid128_to_uint64_xrnint, 0x1d27b5f7b69a5f3c2efc102cd216a7cbu128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_015, bid128_to_uint64_xrnint, 0x26de93d972142942dc03bcec5f9aa514u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_016, bid128_to_uint64_xrnint, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                    , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_uint64_xrnint_017, bid128_to_uint64_xrnint, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0                    , 0x20); // -- 0.5
dec_test!(bid128_to_uint64_xrnint_018, bid128_to_uint64_xrnint, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1                    , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_uint64_xrnint_019, bid128_to_uint64_xrnint, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1                    , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_uint64_xrnint_020, bid128_to_uint64_xrnint, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1                    , 0x20); // -- 0.999
dec_test!(bid128_to_uint64_xrnint_021, bid128_to_uint64_xrnint, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1                    , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_uint64_xrnint_022, bid128_to_uint64_xrnint, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1                    , 0x20); // -- 1-ulp
dec_test!(bid128_to_uint64_xrnint_023, bid128_to_uint64_xrnint, 0x2FFE314DC6448D9338C15B0A00000000u128, 1                    , 0x00); // -- 1
dec_test!(bid128_to_uint64_xrnint_024, bid128_to_uint64_xrnint, 0x2FFE314DC6448D9338C15B0A00000001u128, 1                    , 0x20); // -- 1+ulp
dec_test!(bid128_to_uint64_xrnint_025, bid128_to_uint64_xrnint, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1                    , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint64_xrnint_026, bid128_to_uint64_xrnint, 0x2FFE49F4A966D45CD522088F00000000u128, 2                    , 0x20); // -- 1.5
dec_test!(bid128_to_uint64_xrnint_027, bid128_to_uint64_xrnint, 0x2FFE49F4A966D45CD522088F00000001u128, 2                    , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint64_xrnint_028, bid128_to_uint64_xrnint, 0x30000108000a01084000800400000000u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_029, bid128_to_uint64_xrnint, 0x300004201890e0101fbda7d5feffd9fdu128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_030, bid128_to_uint64_xrnint, 0x30007633018090193ffffff7baed6cceu128, 24                   , 0x20);
dec_test!(bid128_to_uint64_xrnint_031, bid128_to_uint64_xrnint, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_032, bid128_to_uint64_xrnint, 0x300293E952CDA8B9AA44111E00000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_033, bid128_to_uint64_xrnint, 0x300293E952CDA8B9AA44111E00000001u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_034, bid128_to_uint64_xrnint, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xrnint_035, bid128_to_uint64_xrnint, 0x300294286EACB8CB0A8CB6B140000000u128, 300                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xrnint_036, bid128_to_uint64_xrnint, 0x300294286EACB8CB0A8CB6B140000001u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xrnint_037, bid128_to_uint64_xrnint, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_038, bid128_to_uint64_xrnint, 0x30040ECA8847C4129106CE8300000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_039, bid128_to_uint64_xrnint, 0x30040ECA8847C4129106CE8300000001u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_040, bid128_to_uint64_xrnint, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_041, bid128_to_uint64_xrnint, 0x300A0003C95A2F0B4856475FE0000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_042, bid128_to_uint64_xrnint, 0x300A0003C95A2F0B4856475FE0000001u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_043, bid128_to_uint64_xrnint, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_044, bid128_to_uint64_xrnint, 0x300C000060EF6B1ABA6F072330000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_045, bid128_to_uint64_xrnint, 0x300C000060EF6B1ABA6F072330000001u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_046, bid128_to_uint64_xrnint, 0x300ca1bf3db1f8809f1897301a5e8c72u128, 32806196             , 0x20);
dec_test!(bid128_to_uint64_xrnint_047, bid128_to_uint64_xrnint, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000           , 0x20); // -- 4e9-ulp
dec_test!(bid128_to_uint64_xrnint_048, bid128_to_uint64_xrnint, 0x3010C5371912364CE3056C2800000000u128, 4000000000           , 0x00); // -- 4e9
dec_test!(bid128_to_uint64_xrnint_049, bid128_to_uint64_xrnint, 0x3010C5371912364CE3056C2800000001u128, 4000000000           , 0x20); // -- 4e9+ulp
dec_test!(bid128_to_uint64_xrnint_050, bid128_to_uint64_xrnint, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 5000000000           , 0x20); // -- 5e9-ulp
dec_test!(bid128_to_uint64_xrnint_051, bid128_to_uint64_xrnint, 0x3010F684DF56C3E01BC6C73200000000u128, 5000000000           , 0x00); // -- 5e9
dec_test!(bid128_to_uint64_xrnint_052, bid128_to_uint64_xrnint, 0x3010F684DF56C3E01BC6C73200000001u128, 5000000000           , 0x20); // -- 5e9+ulp
dec_test!(bid128_to_uint64_xrnint_053, bid128_to_uint64_xrnint, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 19999999998          , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint64_xrnint_054, bid128_to_uint64_xrnint, 0x3012629B8C88FB62ED56E4238E400000u128, 19999999998          , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_uint64_xrnint_055, bid128_to_uint64_xrnint, 0x3012629B8C88FB62ED56E4238E400001u128, 19999999999          , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint64_xrnint_056, bid128_to_uint64_xrnint, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 19999999999          , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_uint64_xrnint_057, bid128_to_uint64_xrnint, 0x3012629B8C8905F96EBAD4C909800000u128, 19999999999          , 0x00); // -- 2e10-1
dec_test!(bid128_to_uint64_xrnint_058, bid128_to_uint64_xrnint, 0x3012629B8C8905F96EBAD4C909800001u128, 19999999999          , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_uint64_xrnint_059, bid128_to_uint64_xrnint, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 19999999999          , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint64_xrnint_060, bid128_to_uint64_xrnint, 0x3012629B8C89108FF01EC56E84C00000u128, 20000000000          , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_uint64_xrnint_061, bid128_to_uint64_xrnint, 0x3012629B8C89108FF01EC56E84C00001u128, 20000000000          , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint64_xrnint_062, bid128_to_uint64_xrnint, 0x3012629B8C891B267182B613FFFFFFFFu128, 20000000000          , 0x20); // -- 2e10-ulp
dec_test!(bid128_to_uint64_xrnint_063, bid128_to_uint64_xrnint, 0x3012629B8C891B267182B61400000000u128, 20000000000          , 0x00); // -- 2e10
dec_test!(bid128_to_uint64_xrnint_064, bid128_to_uint64_xrnint, 0x3012629B8C891B267182B61400000001u128, 20000000000          , 0x20); // -- 2e10+ulp
dec_test!(bid128_to_uint64_xrnint_065, bid128_to_uint64_xrnint, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 20000000000          , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint64_xrnint_066, bid128_to_uint64_xrnint, 0x3012629B8C8925BCF2E6A6B97B400000u128, 20000000000          , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_uint64_xrnint_067, bid128_to_uint64_xrnint, 0x3012629B8C8925BCF2E6A6B97B400001u128, 20000000001          , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint64_xrnint_068, bid128_to_uint64_xrnint, 0x3012629B8C893053744A975EF67FFFFFu128, 20000000001          , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_uint64_xrnint_069, bid128_to_uint64_xrnint, 0x3012629B8C893053744A975EF6800000u128, 20000000001          , 0x00); // -- 2e10+1
dec_test!(bid128_to_uint64_xrnint_070, bid128_to_uint64_xrnint, 0x3012629B8C893053744A975EF6800001u128, 20000000001          , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_uint64_xrnint_071, bid128_to_uint64_xrnint, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 20000000001          , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint64_xrnint_072, bid128_to_uint64_xrnint, 0x3012629B8C893AE9F5AE880471C00000u128, 20000000002          , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_uint64_xrnint_073, bid128_to_uint64_xrnint, 0x3012629B8C893AE9F5AE880471C00001u128, 20000000002          , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint64_xrnint_074, bid128_to_uint64_xrnint, 0x3018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 35184372088832       , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_uint64_xrnint_075, bid128_to_uint64_xrnint, 0x3018AD78EBC5AC620000000000000000u128, 35184372088832       , 0x00); // -- 2^45
dec_test!(bid128_to_uint64_xrnint_076, bid128_to_uint64_xrnint, 0x3018AD78EBC5AC620000000000000001u128, 35184372088832       , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_uint64_xrnint_077, bid128_to_uint64_xrnint, 0x3018AD78EBC5AC64B5E3AF16B187FFFFu128, 35184372088832       , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_uint64_xrnint_078, bid128_to_uint64_xrnint, 0x3018AD78EBC5AC64B5E3AF16B1880000u128, 35184372088832       , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_uint64_xrnint_079, bid128_to_uint64_xrnint, 0x3018AD78EBC5AC64B5E3AF16B1880001u128, 35184372088833       , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_uint64_xrnint_080, bid128_to_uint64_xrnint, 0x301a00000000000084100010002044a0u128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_081, bid128_to_uint64_xrnint, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xrnint_082, bid128_to_uint64_xrnint, 0x301A0000000000A2E6C09AD3E0D40000u128, 300                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xrnint_083, bid128_to_uint64_xrnint, 0x301A0000000000A2E6C09AD3E0D40001u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xrnint_084, bid128_to_uint64_xrnint, 0x301C629B8C891B265CB1A40684E9FFFFu128, 1999999999999998     , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_uint64_xrnint_085, bid128_to_uint64_xrnint, 0x301C629B8C891B265CB1A40684EA0000u128, 1999999999999998     , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_uint64_xrnint_086, bid128_to_uint64_xrnint, 0x301C629B8C891B265CB1A40684EA0001u128, 1999999999999999     , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_uint64_xrnint_087, bid128_to_uint64_xrnint, 0x301C629B8C891B2663A1FF60589BFFFFu128, 1999999999999999     , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_uint64_xrnint_088, bid128_to_uint64_xrnint, 0x301C629B8C891B2663A1FF60589C0000u128, 1999999999999999     , 0x00); // -- 2e15-1
dec_test!(bid128_to_uint64_xrnint_089, bid128_to_uint64_xrnint, 0x301C629B8C891B2663A1FF60589C0001u128, 1999999999999999     , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_uint64_xrnint_090, bid128_to_uint64_xrnint, 0x301C629B8C891B266A925ABA2C4DFFFFu128, 1999999999999999     , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_uint64_xrnint_091, bid128_to_uint64_xrnint, 0x301C629B8C891B266A925ABA2C4E0000u128, 2000000000000000     , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_uint64_xrnint_092, bid128_to_uint64_xrnint, 0x301C629B8C891B266A925ABA2C4E0001u128, 2000000000000000     , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_uint64_xrnint_093, bid128_to_uint64_xrnint, 0x301C629B8C891B267182B613FFFFFFFFu128, 2000000000000000     , 0x20); // -- 2e15-ulp
dec_test!(bid128_to_uint64_xrnint_094, bid128_to_uint64_xrnint, 0x301C629B8C891B267182B61400000000u128, 2000000000000000     , 0x00); // -- 2e15
dec_test!(bid128_to_uint64_xrnint_095, bid128_to_uint64_xrnint, 0x301C629B8C891B267182B61400000001u128, 2000000000000000     , 0x20); // -- 2e15+ulp
dec_test!(bid128_to_uint64_xrnint_096, bid128_to_uint64_xrnint, 0x301C629B8C891B267873116DD3B1FFFFu128, 2000000000000000     , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_uint64_xrnint_097, bid128_to_uint64_xrnint, 0x301C629B8C891B267873116DD3B20000u128, 2000000000000000     , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_uint64_xrnint_098, bid128_to_uint64_xrnint, 0x301C629B8C891B267873116DD3B20001u128, 2000000000000001     , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_uint64_xrnint_099, bid128_to_uint64_xrnint, 0x301C629B8C891B267F636CC7A763FFFFu128, 2000000000000001     , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_uint64_xrnint_100, bid128_to_uint64_xrnint, 0x301C629B8C891B267F636CC7A7640000u128, 2000000000000001     , 0x00); // -- 2e15+1
dec_test!(bid128_to_uint64_xrnint_101, bid128_to_uint64_xrnint, 0x301C629B8C891B267F636CC7A7640001u128, 2000000000000001     , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_uint64_xrnint_102, bid128_to_uint64_xrnint, 0x301C629B8C891B268653C8217B15FFFFu128, 2000000000000001     , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_uint64_xrnint_103, bid128_to_uint64_xrnint, 0x301C629B8C891B268653C8217B160000u128, 2000000000000002     , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_uint64_xrnint_104, bid128_to_uint64_xrnint, 0x301C629B8C891B268653C8217B160001u128, 2000000000000002     , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_uint64_xrnint_105, bid128_to_uint64_xrnint, 0x301c88d1a000434ddeafdbfe1bbfafb7u128, 2775015909981515     , 0x20);
dec_test!(bid128_to_uint64_xrnint_106, bid128_to_uint64_xrnint, 0x301E000000000001A055690D9DB7FFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_107, bid128_to_uint64_xrnint, 0x301E000000000001A055690D9DB80000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_108, bid128_to_uint64_xrnint, 0x301E000000000001A055690D9DB80001u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_109, bid128_to_uint64_xrnint, 0x301E002C68AF0BB140B1A2BC2EC4FFFFu128, 35184372088832       , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_uint64_xrnint_110, bid128_to_uint64_xrnint, 0x301E002C68AF0BB140B1A2BC2EC50000u128, 35184372088832       , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_uint64_xrnint_111, bid128_to_uint64_xrnint, 0x301E002C68AF0BB140B1A2BC2EC50001u128, 35184372088833       , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_uint64_xrnint_112, bid128_to_uint64_xrnint, 0x30200000000000000010524000000000u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_113, bid128_to_uint64_xrnint, 0x302000000000000029A2241AF62BFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_114, bid128_to_uint64_xrnint, 0x302000000000000029A2241AF62C0000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_115, bid128_to_uint64_xrnint, 0x302000000000000029A2241AF62C0001u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_116, bid128_to_uint64_xrnint, 0x3020000470DE4DF81FFFFFFFFFFFFFFFu128, 35184372088832       , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_uint64_xrnint_117, bid128_to_uint64_xrnint, 0x3020000470DE4DF82000000000000000u128, 35184372088832       , 0x00); // -- 2^45
dec_test!(bid128_to_uint64_xrnint_118, bid128_to_uint64_xrnint, 0x3020000470DE4DF82000000000000001u128, 35184372088832       , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_uint64_xrnint_119, bid128_to_uint64_xrnint, 0x302000FC6F7C4045813459C637E07FFFu128, 2000000000000000     , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_uint64_xrnint_120, bid128_to_uint64_xrnint, 0x302000FC6F7C4045813459C637E08000u128, 2000000000000000     , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_uint64_xrnint_121, bid128_to_uint64_xrnint, 0x302000FC6F7C4045813459C637E08001u128, 2000000000000001     , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_uint64_xrnint_122, bid128_to_uint64_xrnint, 0x302000FC6F7C40458157E0B8A7A17FFFu128, 2000000000000001     , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_uint64_xrnint_123, bid128_to_uint64_xrnint, 0x302000FC6F7C40458157E0B8A7A18000u128, 2000000000000002     , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_uint64_xrnint_124, bid128_to_uint64_xrnint, 0x302000FC6F7C40458157E0B8A7A18001u128, 2000000000000002     , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_uint64_xrnint_125, bid128_to_uint64_xrnint, 0x302200193E5939A08CE4879688D63FFFu128, 1999999999999998     , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_uint64_xrnint_126, bid128_to_uint64_xrnint, 0x302200193E5939A08CE4879688D64000u128, 1999999999999998     , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_uint64_xrnint_127, bid128_to_uint64_xrnint, 0x302200193E5939A08CE4879688D64001u128, 1999999999999999     , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_uint64_xrnint_128, bid128_to_uint64_xrnint, 0x302200193E5939A08CE815152D9CBFFFu128, 1999999999999999     , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_uint64_xrnint_129, bid128_to_uint64_xrnint, 0x302200193E5939A08CE815152D9CC000u128, 2000000000000000     , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_uint64_xrnint_130, bid128_to_uint64_xrnint, 0x302200193E5939A08CE815152D9CC001u128, 2000000000000000     , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_uint64_xrnint_131, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775806  , 0x20); // -- 2^63-1.5-ulp
dec_test!(bid128_to_uint64_xrnint_132, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775806  , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_uint64_xrnint_133, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775807  , 0x20); // -- 2^63-1.5+ulp
dec_test!(bid128_to_uint64_xrnint_134, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775807  , 0x20); // -- 2^63-1-ulp
dec_test!(bid128_to_uint64_xrnint_135, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFC72815B398000u128, 9223372036854775807  , 0x00); // -- 2^63-1
dec_test!(bid128_to_uint64_xrnint_136, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFC72815B398001u128, 9223372036854775807  , 0x20); // -- 2^63-1+ulp
dec_test!(bid128_to_uint64_xrnint_137, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775807  , 0x20); // -- 2^63-0.5-ulp
dec_test!(bid128_to_uint64_xrnint_138, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFE3940AD9CC000u128, 9223372036854775808  , 0x20); // -- 2^63-0.5
dec_test!(bid128_to_uint64_xrnint_139, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFE3940AD9CC001u128, 9223372036854775808  , 0x20); // -- 2^63-0.5+ulp
dec_test!(bid128_to_uint64_xrnint_140, bid128_to_uint64_xrnint, 0x3023C6BF52633FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x20); // -- 2^63-ulp
dec_test!(bid128_to_uint64_xrnint_141, bid128_to_uint64_xrnint, 0x3023C6BF526340000000000000000000u128, 9223372036854775808  , 0x00); // -- 2^63
dec_test!(bid128_to_uint64_xrnint_142, bid128_to_uint64_xrnint, 0x3023C6BF526340000000000000000001u128, 9223372036854775808  , 0x20); // -- 2^63+ulp
dec_test!(bid128_to_uint64_xrnint_143, bid128_to_uint64_xrnint, 0x3023C6BF526340000001C6BF52633FFFu128, 9223372036854775808  , 0x20); // -- 2^63+0.5-ulp
dec_test!(bid128_to_uint64_xrnint_144, bid128_to_uint64_xrnint, 0x3023C6BF526340000001C6BF52634000u128, 9223372036854775808  , 0x20); // -- 2^63+0.5
dec_test!(bid128_to_uint64_xrnint_145, bid128_to_uint64_xrnint, 0x3023C6BF526340000001C6BF52634001u128, 9223372036854775809  , 0x20); // -- 2^63+0.5+ulp
dec_test!(bid128_to_uint64_xrnint_146, bid128_to_uint64_xrnint, 0x3023C6BF5263400000038D7EA4C67FFFu128, 9223372036854775809  , 0x20); // -- 2^63+1-ulp
dec_test!(bid128_to_uint64_xrnint_147, bid128_to_uint64_xrnint, 0x3023C6BF5263400000038D7EA4C68000u128, 9223372036854775809  , 0x00); // -- 2^63+1
dec_test!(bid128_to_uint64_xrnint_148, bid128_to_uint64_xrnint, 0x3023C6BF5263400000038D7EA4C68001u128, 9223372036854775809  , 0x20); // -- 2^63+1+ulp
dec_test!(bid128_to_uint64_xrnint_149, bid128_to_uint64_xrnint, 0x3024000000000000006A94D74F42FFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_150, bid128_to_uint64_xrnint, 0x3024000000000000006A94D74F430000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_151, bid128_to_uint64_xrnint, 0x3024000000000000006A94D74F430001u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_152, bid128_to_uint64_xrnint, 0x30242af2723fd83dc0e9d531fc953d2du128, 8710697771564387091  , 0x20);
dec_test!(bid128_to_uint64_xrnint_153, bid128_to_uint64_xrnint, 0x3024314DC6448D9338C15B09FFFFFFFFu128, 10000000000000000000 , 0x20); // -- 1e19-ulp
dec_test!(bid128_to_uint64_xrnint_154, bid128_to_uint64_xrnint, 0x3024314DC6448D9338C15B0A00000000u128, 10000000000000000000 , 0x00); // -- 1e19
dec_test!(bid128_to_uint64_xrnint_155, bid128_to_uint64_xrnint, 0x3024314DC6448D9338C15B0A00000001u128, 10000000000000000000 , 0x20); // -- 1e19+ulp
dec_test!(bid128_to_uint64_xrnint_156, bid128_to_uint64_xrnint, 0x3024314DC6448D9338C18883883D1FFFu128, 10000000000000000000 , 0x20); // -- 1e19+0.5-ulp
dec_test!(bid128_to_uint64_xrnint_157, bid128_to_uint64_xrnint, 0x3024314DC6448D9338C18883883D2000u128, 10000000000000000000 , 0x20); // -- 1e19+0.5
dec_test!(bid128_to_uint64_xrnint_158, bid128_to_uint64_xrnint, 0x3024314DC6448D9338C18883883D2001u128, 10000000000000000001 , 0x20); // -- 1e19+0.5+ulp
dec_test!(bid128_to_uint64_xrnint_159, bid128_to_uint64_xrnint, 0x302449F4A966D45CD522088EFFFFFFFFu128, 15000000000000000000 , 0x20); // -- 1.5e19-ulp
dec_test!(bid128_to_uint64_xrnint_160, bid128_to_uint64_xrnint, 0x302449F4A966D45CD522088F00000000u128, 15000000000000000000 , 0x00); // -- 1.5e19
dec_test!(bid128_to_uint64_xrnint_161, bid128_to_uint64_xrnint, 0x302449F4A966D45CD522088F00000001u128, 15000000000000000000 , 0x20); // -- 1.5e19+ulp
dec_test!(bid128_to_uint64_xrnint_162, bid128_to_uint64_xrnint, 0x30245AF3107A3FFFFFFFA50CEF85BFFFu128, 18446744073709551615 , 0x20); // -- 2^64-1-ulp
dec_test!(bid128_to_uint64_xrnint_163, bid128_to_uint64_xrnint, 0x30245AF3107A3FFFFFFFA50CEF85C000u128, 18446744073709551615 , 0x00); // -- 2^64-1
dec_test!(bid128_to_uint64_xrnint_164, bid128_to_uint64_xrnint, 0x30245AF3107A3FFFFFFFA50CEF85C001u128, 18446744073709551615 , 0x20); // -- 2^64-1+ulp
dec_test!(bid128_to_uint64_xrnint_165, bid128_to_uint64_xrnint, 0x30245AF3107A3FFFFFFFD28677C2DFFFu128, 18446744073709551615 , 0x20); // -- 2^64-0.5-ulp
dec_test!(bid128_to_uint64_xrnint_166, bid128_to_uint64_xrnint, 0x30245AF3107A3FFFFFFFD28677C2E000u128, 9223372036854775808  , 0x01); // -- 2^64-0.5
dec_test!(bid128_to_uint64_xrnint_167, bid128_to_uint64_xrnint, 0x30245AF3107A3FFFFFFFD28677C2E001u128, 9223372036854775808  , 0x01); // -- 2^64-0.5+ulp
dec_test!(bid128_to_uint64_xrnint_168, bid128_to_uint64_xrnint, 0x30245AF3107A3FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- 2^64-ulp
dec_test!(bid128_to_uint64_xrnint_169, bid128_to_uint64_xrnint, 0x30245AF3107A40000000000000000000u128, 9223372036854775808  , 0x01); // -- 2^64
dec_test!(bid128_to_uint64_xrnint_170, bid128_to_uint64_xrnint, 0x30245AF3107A40000000000000000001u128, 9223372036854775808  , 0x01); // -- 2^64+ulp
dec_test!(bid128_to_uint64_xrnint_171, bid128_to_uint64_xrnint, 0x30245AF3107A400000002D79883D1FFFu128, 9223372036854775808  , 0x01); // -- 2^64+0.5-ulp
dec_test!(bid128_to_uint64_xrnint_172, bid128_to_uint64_xrnint, 0x30245AF3107A400000002D79883D2000u128, 9223372036854775808  , 0x01); // -- 2^64+0.5
dec_test!(bid128_to_uint64_xrnint_173, bid128_to_uint64_xrnint, 0x30245AF3107A400000002D79883D2001u128, 9223372036854775808  , 0x01); // -- 2^64+0.5+ulp
dec_test!(bid128_to_uint64_xrnint_174, bid128_to_uint64_xrnint, 0x30245AF3107A400000005AF3107A3FFFu128, 9223372036854775808  , 0x01); // -- 2^64+1-ulp
dec_test!(bid128_to_uint64_xrnint_175, bid128_to_uint64_xrnint, 0x30245AF3107A400000005AF3107A4000u128, 9223372036854775808  , 0x01); // -- 2^64+1
dec_test!(bid128_to_uint64_xrnint_176, bid128_to_uint64_xrnint, 0x30245AF3107A400000005AF3107A4001u128, 9223372036854775808  , 0x01); // -- 2^64+1+ulp
dec_test!(bid128_to_uint64_xrnint_177, bid128_to_uint64_xrnint, 0x3024629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- 2e19-ulp
dec_test!(bid128_to_uint64_xrnint_178, bid128_to_uint64_xrnint, 0x3024629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- 2e19
dec_test!(bid128_to_uint64_xrnint_179, bid128_to_uint64_xrnint, 0x3024629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- 2e19+ulp
dec_test!(bid128_to_uint64_xrnint_180, bid128_to_uint64_xrnint, 0x30247B426FAB61F00DE36398FFFFFFFFu128, 9223372036854775808  , 0x01); // -- 2.5e19-ulp
dec_test!(bid128_to_uint64_xrnint_181, bid128_to_uint64_xrnint, 0x30247B426FAB61F00DE3639900000000u128, 9223372036854775808  , 0x01); // -- 2.5e19
dec_test!(bid128_to_uint64_xrnint_182, bid128_to_uint64_xrnint, 0x30247B426FAB61F00DE3639900000001u128, 9223372036854775808  , 0x01); // -- 2.5e19+ulp
dec_test!(bid128_to_uint64_xrnint_183, bid128_to_uint64_xrnint, 0x3024be403efcefe16a74460324453390u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_184, bid128_to_uint64_xrnint, 0x3026314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- 1e20-ulp
dec_test!(bid128_to_uint64_xrnint_185, bid128_to_uint64_xrnint, 0x3026314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- 1e20
dec_test!(bid128_to_uint64_xrnint_186, bid128_to_uint64_xrnint, 0x3026314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- 1e20+ulp
dec_test!(bid128_to_uint64_xrnint_187, bid128_to_uint64_xrnint, 0x302A00000000006C6B935B68D08DA3FFu128, 19999999998          , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint64_xrnint_188, bid128_to_uint64_xrnint, 0x302A00000000006C6B935B68D08DA400u128, 19999999998          , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_uint64_xrnint_189, bid128_to_uint64_xrnint, 0x302A00000000006C6B935B68D08DA401u128, 19999999999          , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint64_xrnint_190, bid128_to_uint64_xrnint, 0x302A00000000006C6B935B8019048BFFu128, 19999999999          , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint64_xrnint_191, bid128_to_uint64_xrnint, 0x302A00000000006C6B935B8019048C00u128, 20000000000          , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_uint64_xrnint_192, bid128_to_uint64_xrnint, 0x302A00000000006C6B935B8019048C01u128, 20000000000          , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint64_xrnint_193, bid128_to_uint64_xrnint, 0x302C000000000000000002BBA7F521FFu128, 300                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xrnint_194, bid128_to_uint64_xrnint, 0x302C000000000000000002BBA7F52200u128, 300                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xrnint_195, bid128_to_uint64_xrnint, 0x302C000000000000000002BBA7F52201u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xrnint_196, bid128_to_uint64_xrnint, 0x302C00000000000AD78EBC5872141BFFu128, 19999999999          , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_uint64_xrnint_197, bid128_to_uint64_xrnint, 0x302C00000000000AD78EBC5872141C00u128, 19999999999          , 0x00); // -- 2e10-1
dec_test!(bid128_to_uint64_xrnint_198, bid128_to_uint64_xrnint, 0x302C00000000000AD78EBC5872141C01u128, 19999999999          , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_uint64_xrnint_199, bid128_to_uint64_xrnint, 0x302C00000000000AD78EBC5BF025F1FFu128, 20000000000          , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint64_xrnint_200, bid128_to_uint64_xrnint, 0x302C00000000000AD78EBC5BF025F200u128, 20000000000          , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_uint64_xrnint_201, bid128_to_uint64_xrnint, 0x302C00000000000AD78EBC5BF025F201u128, 20000000001          , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint64_xrnint_202, bid128_to_uint64_xrnint, 0x302C00000000000AD78EBC5E4431D5FFu128, 20000000001          , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint64_xrnint_203, bid128_to_uint64_xrnint, 0x302C00000000000AD78EBC5E4431D600u128, 20000000002          , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_uint64_xrnint_204, bid128_to_uint64_xrnint, 0x302C00000000000AD78EBC5E4431D601u128, 20000000002          , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint64_xrnint_205, bid128_to_uint64_xrnint, 0x302C000000108B2A2C28028E3FF41BFFu128, 1999999999999999     , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_uint64_xrnint_206, bid128_to_uint64_xrnint, 0x302C000000108B2A2C28028E3FF41C00u128, 1999999999999999     , 0x00); // -- 2e15-1
dec_test!(bid128_to_uint64_xrnint_207, bid128_to_uint64_xrnint, 0x302C000000108B2A2C28028E3FF41C01u128, 1999999999999999     , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_uint64_xrnint_208, bid128_to_uint64_xrnint, 0x302E000000000001158E46094F6AC9FFu128, 20000000001          , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_uint64_xrnint_209, bid128_to_uint64_xrnint, 0x302E000000000001158E46094F6ACA00u128, 20000000001          , 0x00); // -- 2e10+1
dec_test!(bid128_to_uint64_xrnint_210, bid128_to_uint64_xrnint, 0x302E000000000001158E46094F6ACA01u128, 20000000001          , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_uint64_xrnint_211, bid128_to_uint64_xrnint, 0x302E00000001A784379D99DB7D9AC9FFu128, 2000000000000001     , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_uint64_xrnint_212, bid128_to_uint64_xrnint, 0x302E00000001A784379D99DB7D9ACA00u128, 2000000000000001     , 0x00); // -- 2e15+1
dec_test!(bid128_to_uint64_xrnint_213, bid128_to_uint64_xrnint, 0x302E00000001A784379D99DB7D9ACA01u128, 2000000000000001     , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_uint64_xrnint_214, bid128_to_uint64_xrnint, 0x303000000000000000000006FC23ABFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_215, bid128_to_uint64_xrnint, 0x303000000000000000000006FC23AC00u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_216, bid128_to_uint64_xrnint, 0x303000000000000000000006FC23AC01u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_217, bid128_to_uint64_xrnint, 0x303200000000000000000000B2D05DFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_218, bid128_to_uint64_xrnint, 0x303200000000000000000000B2D05E00u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_219, bid128_to_uint64_xrnint, 0x303200000000000000000000B2D05E01u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_220, bid128_to_uint64_xrnint, 0x303800000000000000000000002DDA47u128, 300                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xrnint_221, bid128_to_uint64_xrnint, 0x303800000000000000000000002DDA48u128, 300                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xrnint_222, bid128_to_uint64_xrnint, 0x303800000000000000000000002DDA49u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xrnint_223, bid128_to_uint64_xrnint, 0x303A00000000000000000000000003E7u128, 1                    , 0x20); // -- 0.999
dec_test!(bid128_to_uint64_xrnint_224, bid128_to_uint64_xrnint, 0x303A00000000000000000000000495D3u128, 300                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xrnint_225, bid128_to_uint64_xrnint, 0x303A00000000000000000000000495D4u128, 300                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xrnint_226, bid128_to_uint64_xrnint, 0x303A00000000000000000000000495D5u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xrnint_227, bid128_to_uint64_xrnint, 0x303C0000000000000000000000007561u128, 300                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xrnint_228, bid128_to_uint64_xrnint, 0x303C0000000000000000000000007562u128, 300                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xrnint_229, bid128_to_uint64_xrnint, 0x303C0000000000000000000000007563u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xrnint_230, bid128_to_uint64_xrnint, 0x303E0000000000000000000000000005u128, 0                    , 0x20); // -- 0.5
dec_test!(bid128_to_uint64_xrnint_231, bid128_to_uint64_xrnint, 0x303E000000000000000000000000000Fu128, 2                    , 0x20); // -- 1.5
dec_test!(bid128_to_uint64_xrnint_232, bid128_to_uint64_xrnint, 0x303E0000000000000000000000000BB7u128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_233, bid128_to_uint64_xrnint, 0x303E0000000000000000000000000BB8u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_234, bid128_to_uint64_xrnint, 0x303E0000000000000000000000000BB9u128, 300                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_235, bid128_to_uint64_xrnint, 0x303E0000000000000000000000000BBDu128, 300                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xrnint_236, bid128_to_uint64_xrnint, 0x303E0000000000000000002E90EDCFF1u128, 19999999998          , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_uint64_xrnint_237, bid128_to_uint64_xrnint, 0x303E0000000000000000002E90EDCFFBu128, 20000000000          , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_uint64_xrnint_238, bid128_to_uint64_xrnint, 0x303E0000000000000000002E90EDD005u128, 20000000000          , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_uint64_xrnint_239, bid128_to_uint64_xrnint, 0x303E0000000000000000002E90EDD00Fu128, 20000000002          , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_uint64_xrnint_240, bid128_to_uint64_xrnint, 0x303E0000000000000001400000000005u128, 35184372088832       , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_uint64_xrnint_241, bid128_to_uint64_xrnint, 0x303E00000000000000470DE4DF81FFF1u128, 1999999999999998     , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_uint64_xrnint_242, bid128_to_uint64_xrnint, 0x303E00000000000000470DE4DF81FFFBu128, 2000000000000000     , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_uint64_xrnint_243, bid128_to_uint64_xrnint, 0x303E00000000000000470DE4DF820005u128, 2000000000000000     , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_uint64_xrnint_244, bid128_to_uint64_xrnint, 0x303E00000000000000470DE4DF82000Fu128, 2000000000000002     , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_uint64_xrnint_245, bid128_to_uint64_xrnint, 0x303E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775806  , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_uint64_xrnint_246, bid128_to_uint64_xrnint, 0x303E000000000004FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x20); // -- 2^63-0.5
dec_test!(bid128_to_uint64_xrnint_247, bid128_to_uint64_xrnint, 0x303E0000000000050000000000000005u128, 9223372036854775808  , 0x20); // -- 2^63+0.5
dec_test!(bid128_to_uint64_xrnint_248, bid128_to_uint64_xrnint, 0x303E0000000000056BC75E2D63100005u128, 10000000000000000000 , 0x20); // -- 1e19+0.5
dec_test!(bid128_to_uint64_xrnint_249, bid128_to_uint64_xrnint, 0x303E000000000009FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x01); // -- 2^64-0.5
dec_test!(bid128_to_uint64_xrnint_250, bid128_to_uint64_xrnint, 0x303E00000000000A0000000000000005u128, 9223372036854775808  , 0x01); // -- 2^64+0.5
dec_test!(bid128_to_uint64_xrnint_251, bid128_to_uint64_xrnint, 0x30400000000000000000000000000001u128, 1                    , 0x00); // -- 1
dec_test!(bid128_to_uint64_xrnint_252, bid128_to_uint64_xrnint, 0x3040000000000000000000000000012Bu128, 299                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_253, bid128_to_uint64_xrnint, 0x3040000000000000000000000000012Cu128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_254, bid128_to_uint64_xrnint, 0x3040000000000000000000000000012Du128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_255, bid128_to_uint64_xrnint, 0x304000000000000000000004A817C7FFu128, 19999999999          , 0x00); // -- 2e10-1
dec_test!(bid128_to_uint64_xrnint_256, bid128_to_uint64_xrnint, 0x304000000000000000000004A817C801u128, 20000000001          , 0x00); // -- 2e10+1
dec_test!(bid128_to_uint64_xrnint_257, bid128_to_uint64_xrnint, 0x30400000000000000000200000000000u128, 35184372088832       , 0x00); // -- 2^45
dec_test!(bid128_to_uint64_xrnint_258, bid128_to_uint64_xrnint, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999     , 0x00); // -- 2e15-1
dec_test!(bid128_to_uint64_xrnint_259, bid128_to_uint64_xrnint, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999     , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_uint64_xrnint_260, bid128_to_uint64_xrnint, 0x304000000000000000071AFD498D0000u128, 2000000000000000     , 0x00); // -- 2e15
dec_test!(bid128_to_uint64_xrnint_261, bid128_to_uint64_xrnint, 0x304000000000000000071AFD498D0001u128, 2000000000000001     , 0x00); // -- 2e15+1
dec_test!(bid128_to_uint64_xrnint_262, bid128_to_uint64_xrnint, 0x304000000000000000071AFD498D0001u128, 2000000000000001     , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_uint64_xrnint_263, bid128_to_uint64_xrnint, 0x30400000000000000b0448a4bfc6a849u128, 793839306753353801   , 0x00);
dec_test!(bid128_to_uint64_xrnint_264, bid128_to_uint64_xrnint, 0x30400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775807  , 0x00); // -- 2^63-1
dec_test!(bid128_to_uint64_xrnint_265, bid128_to_uint64_xrnint, 0x30400000000000008000000000000000u128, 9223372036854775808  , 0x00); // -- 2^63
dec_test!(bid128_to_uint64_xrnint_266, bid128_to_uint64_xrnint, 0x30400000000000008000000000000001u128, 9223372036854775809  , 0x00); // -- 2^63+1
dec_test!(bid128_to_uint64_xrnint_267, bid128_to_uint64_xrnint, 0x3040000000000000FFFFFFFFFFFFFFFFu128, 18446744073709551615 , 0x00); // -- 2^64-1
dec_test!(bid128_to_uint64_xrnint_268, bid128_to_uint64_xrnint, 0x30400000000000010000000000000000u128, 9223372036854775808  , 0x01); // -- 2^64
dec_test!(bid128_to_uint64_xrnint_269, bid128_to_uint64_xrnint, 0x30400000000000010000000000000001u128, 9223372036854775808  , 0x01); // -- 2^64+1
dec_test!(bid128_to_uint64_xrnint_270, bid128_to_uint64_xrnint, 0x30400000000000017ffffffffffff7ffu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_271, bid128_to_uint64_xrnint, 0x3040000000000001e3ddcfbbbb0be7ffu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_272, bid128_to_uint64_xrnint, 0x3041ED09BEAD87C0378D8E6400000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xrnint_273, bid128_to_uint64_xrnint, 0x3042000000000000000000000000001Du128, 290                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_xrnint_274, bid128_to_uint64_xrnint, 0x3042000000000000000000000000001Eu128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_275, bid128_to_uint64_xrnint, 0x3042000000000000000000000000001Fu128, 310                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_xrnint_276, bid128_to_uint64_xrnint, 0x304200000000000000000000773593FFu128, 19999999990          , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_uint64_xrnint_277, bid128_to_uint64_xrnint, 0x30420000000000000000000077359400u128, 20000000000          , 0x00); // -- 2e10
dec_test!(bid128_to_uint64_xrnint_278, bid128_to_uint64_xrnint, 0x30420000000000000000000077359401u128, 20000000010          , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_uint64_xrnint_279, bid128_to_uint64_xrnint, 0x304200000000000083ffc3df3dbbfcfdu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_280, bid128_to_uint64_xrnint, 0x30440000000000000000000000000003u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xrnint_281, bid128_to_uint64_xrnint, 0x30500000000000000000000401200000u128, 1719874355200000000  , 0x00);
dec_test!(bid128_to_uint64_xrnint_282, bid128_to_uint64_xrnint, 0x30520000000000000000000000000004u128, 4000000000           , 0x00); // -- 4e9
dec_test!(bid128_to_uint64_xrnint_283, bid128_to_uint64_xrnint, 0x30520000000000000000000000000005u128, 5000000000           , 0x00); // -- 5e9
dec_test!(bid128_to_uint64_xrnint_284, bid128_to_uint64_xrnint, 0x30540000000000000000000000000002u128, 20000000000          , 0x00); // -- 2e10
dec_test!(bid128_to_uint64_xrnint_285, bid128_to_uint64_xrnint, 0x305c7d0196141b2f42a989810e25be1au128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xrnint_286, bid128_to_uint64_xrnint, 0x305E0000000000000000000000000002u128, 2000000000000000     , 0x00); // -- 2e15
dec_test!(bid128_to_uint64_xrnint_287, bid128_to_uint64_xrnint, 0x3064000000000000000000000000000Fu128, 15000000000000000000 , 0x00); // -- 1.5e19
dec_test!(bid128_to_uint64_xrnint_288, bid128_to_uint64_xrnint, 0x30640000000000000000000000000019u128, 9223372036854775808  , 0x01); // -- 2.5e19
dec_test!(bid128_to_uint64_xrnint_289, bid128_to_uint64_xrnint, 0x30660000000000000000000000000001u128, 10000000000000000000 , 0x00); // -- 1e19
dec_test!(bid128_to_uint64_xrnint_290, bid128_to_uint64_xrnint, 0x30660000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- 2e19
dec_test!(bid128_to_uint64_xrnint_291, bid128_to_uint64_xrnint, 0x30680000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- 1e20
dec_test!(bid128_to_uint64_xrnint_292, bid128_to_uint64_xrnint, 0x34ba0000000000000000000000000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xrnint_293, bid128_to_uint64_xrnint, 0x45f98804228a38582bc15f10714df8b9u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xrnint_294, bid128_to_uint64_xrnint, "5.5"                                 , 6                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_295, bid128_to_uint64_xrnint, 0x5b5536cfab05199bb428ebf84ed3fa67u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xrnint_296, bid128_to_uint64_xrnint, 0x78000000000000000000000000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_297, bid128_to_uint64_xrnint, 0x7af80b34a7f40cd0fcf5b37e7f3de3bdu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_298, bid128_to_uint64_xrnint, 0x7c000000000000000000000000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_299, bid128_to_uint64_xrnint, 0x7c003fffffffffff38c15b08ffffffffu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_300, bid128_to_uint64_xrnint, 0x7c003fffffffffff38c15b0affffffffu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_301, bid128_to_uint64_xrnint, 0x7e000000000000000000000000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_302, bid128_to_uint64_xrnint, 0x8657af280cd75cae69ef7f1b4112e075u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_303, bid128_to_uint64_xrnint, 0x89bfc67e8cd54589367913c1270e4318u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_304, bid128_to_uint64_xrnint, 0x914d0c8d7cd516035f80ba851d9841a1u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_305, bid128_to_uint64_xrnint, 0x9bb8eee450181616bc29a8d331a562dfu128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_306, bid128_to_uint64_xrnint, 0xa2743547f4a9031e1af80aaa76d5a81cu128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xrnint_307, bid128_to_uint64_xrnint, 0xac6e0000000000000000000000000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xrnint_308, bid128_to_uint64_xrnint, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                    , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_309, bid128_to_uint64_xrnint, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0                    , 0x20); // -- -(0.5)
dec_test!(bid128_to_uint64_xrnint_310, bid128_to_uint64_xrnint, 0xAFFCF684DF56C3E01BC6C73200000001u128, 9223372036854775808  , 0x01); // -- -(0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_311, bid128_to_uint64_xrnint, 0xaffd7dbbddfdef9f821f4ecf8f63eab4u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_312, bid128_to_uint64_xrnint, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 9223372036854775808  , 0x01); // -- -(0.999-ulp)
dec_test!(bid128_to_uint64_xrnint_313, bid128_to_uint64_xrnint, 0xAFFDEC8B86EF679D76FC433D80000000u128, 9223372036854775808  , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint64_xrnint_314, bid128_to_uint64_xrnint, 0xAFFDEC8B86EF679D76FC433D80000001u128, 9223372036854775808  , 0x01); // -- -(0.999+ulp)
dec_test!(bid128_to_uint64_xrnint_315, bid128_to_uint64_xrnint, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1-ulp)
dec_test!(bid128_to_uint64_xrnint_316, bid128_to_uint64_xrnint, 0xAFFE314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- -(1)
dec_test!(bid128_to_uint64_xrnint_317, bid128_to_uint64_xrnint, 0xAFFE314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint64_xrnint_318, bid128_to_uint64_xrnint, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_319, bid128_to_uint64_xrnint, 0xAFFE49F4A966D45CD522088F00000000u128, 9223372036854775808  , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint64_xrnint_320, bid128_to_uint64_xrnint, 0xAFFE49F4A966D45CD522088F00000001u128, 9223372036854775808  , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_321, bid128_to_uint64_xrnint, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_322, bid128_to_uint64_xrnint, 0xB00293E952CDA8B9AA44111E00000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_323, bid128_to_uint64_xrnint, 0xB00293E952CDA8B9AA44111E00000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_324, bid128_to_uint64_xrnint, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xrnint_325, bid128_to_uint64_xrnint, 0xB00294286EACB8CB0A8CB6B140000000u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xrnint_326, bid128_to_uint64_xrnint, 0xB00294286EACB8CB0A8CB6B140000001u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xrnint_327, bid128_to_uint64_xrnint, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_328, bid128_to_uint64_xrnint, 0xB0040ECA8847C4129106CE8300000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_329, bid128_to_uint64_xrnint, 0xB0040ECA8847C4129106CE8300000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_330, bid128_to_uint64_xrnint, 0xb005c6d8bb5bcc85fffff7ffffefb7ffu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_331, bid128_to_uint64_xrnint, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_332, bid128_to_uint64_xrnint, 0xB00A0003C95A2F0B4856475FE0000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_333, bid128_to_uint64_xrnint, 0xB00A0003C95A2F0B4856475FE0000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_334, bid128_to_uint64_xrnint, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_335, bid128_to_uint64_xrnint, 0xB00C000060EF6B1ABA6F072330000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_336, bid128_to_uint64_xrnint, 0xB00C000060EF6B1ABA6F072330000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_337, bid128_to_uint64_xrnint, 0xB010C5371912364CE3056C27FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_uint64_xrnint_338, bid128_to_uint64_xrnint, 0xB010C5371912364CE3056C2800000000u128, 9223372036854775808  , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint64_xrnint_339, bid128_to_uint64_xrnint, 0xB010C5371912364CE3056C2800000001u128, 9223372036854775808  , 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_uint64_xrnint_340, bid128_to_uint64_xrnint, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_uint64_xrnint_341, bid128_to_uint64_xrnint, 0xB010F684DF56C3E01BC6C73200000000u128, 9223372036854775808  , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint64_xrnint_342, bid128_to_uint64_xrnint, 0xB010F684DF56C3E01BC6C73200000001u128, 9223372036854775808  , 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_uint64_xrnint_343, bid128_to_uint64_xrnint, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_344, bid128_to_uint64_xrnint, 0xB012629B8C88FB62ED56E4238E400000u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint64_xrnint_345, bid128_to_uint64_xrnint, 0xB012629B8C88FB62ED56E4238E400001u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_346, bid128_to_uint64_xrnint, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint64_xrnint_347, bid128_to_uint64_xrnint, 0xB012629B8C8905F96EBAD4C909800000u128, 9223372036854775808  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint64_xrnint_348, bid128_to_uint64_xrnint, 0xB012629B8C8905F96EBAD4C909800001u128, 9223372036854775808  , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint64_xrnint_349, bid128_to_uint64_xrnint, 0xB012629B8C89108FF01EC56E84BFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_350, bid128_to_uint64_xrnint, 0xB012629B8C89108FF01EC56E84C00000u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint64_xrnint_351, bid128_to_uint64_xrnint, 0xB012629B8C89108FF01EC56E84C00001u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_352, bid128_to_uint64_xrnint, 0xB012629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint64_xrnint_353, bid128_to_uint64_xrnint, 0xB012629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint64_xrnint_354, bid128_to_uint64_xrnint, 0xB012629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint64_xrnint_355, bid128_to_uint64_xrnint, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_356, bid128_to_uint64_xrnint, 0xB012629B8C8925BCF2E6A6B97B400000u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint64_xrnint_357, bid128_to_uint64_xrnint, 0xB012629B8C8925BCF2E6A6B97B400001u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_358, bid128_to_uint64_xrnint, 0xB012629B8C893053744A975EF67FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint64_xrnint_359, bid128_to_uint64_xrnint, 0xB012629B8C893053744A975EF6800000u128, 9223372036854775808  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint64_xrnint_360, bid128_to_uint64_xrnint, 0xB012629B8C893053744A975EF6800001u128, 9223372036854775808  , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint64_xrnint_361, bid128_to_uint64_xrnint, 0xB012629B8C893AE9F5AE880471BFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_362, bid128_to_uint64_xrnint, 0xB012629B8C893AE9F5AE880471C00000u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint64_xrnint_363, bid128_to_uint64_xrnint, 0xB012629B8C893AE9F5AE880471C00001u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_364, bid128_to_uint64_xrnint, 0xB018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^45-ulp)
dec_test!(bid128_to_uint64_xrnint_365, bid128_to_uint64_xrnint, 0xB018AD78EBC5AC620000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^45)
dec_test!(bid128_to_uint64_xrnint_366, bid128_to_uint64_xrnint, 0xB018AD78EBC5AC620000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^45+ulp)
dec_test!(bid128_to_uint64_xrnint_367, bid128_to_uint64_xrnint, 0xB018AD78EBC5AC64B5E3AF16B187FFFFu128, 9223372036854775808  , 0x01); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_368, bid128_to_uint64_xrnint, 0xB018AD78EBC5AC64B5E3AF16B1880000u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5)
dec_test!(bid128_to_uint64_xrnint_369, bid128_to_uint64_xrnint, 0xB018AD78EBC5AC64B5E3AF16B1880001u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_370, bid128_to_uint64_xrnint, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xrnint_371, bid128_to_uint64_xrnint, 0xB01A0000000000A2E6C09AD3E0D40000u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xrnint_372, bid128_to_uint64_xrnint, 0xB01A0000000000A2E6C09AD3E0D40001u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xrnint_373, bid128_to_uint64_xrnint, 0xB01C629B8C891B265CB1A40684E9FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_374, bid128_to_uint64_xrnint, 0xB01C629B8C891B265CB1A40684EA0000u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5)
dec_test!(bid128_to_uint64_xrnint_375, bid128_to_uint64_xrnint, 0xB01C629B8C891B265CB1A40684EA0001u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_376, bid128_to_uint64_xrnint, 0xB01C629B8C891B2663A1FF60589BFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1-ulp)
dec_test!(bid128_to_uint64_xrnint_377, bid128_to_uint64_xrnint, 0xB01C629B8C891B2663A1FF60589C0000u128, 9223372036854775808  , 0x01); // -- -(2e15-1)
dec_test!(bid128_to_uint64_xrnint_378, bid128_to_uint64_xrnint, 0xB01C629B8C891B2663A1FF60589C0001u128, 9223372036854775808  , 0x01); // -- -(2e15-1+ulp)
dec_test!(bid128_to_uint64_xrnint_379, bid128_to_uint64_xrnint, 0xB01C629B8C891B266A925ABA2C4DFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_380, bid128_to_uint64_xrnint, 0xB01C629B8C891B266A925ABA2C4E0000u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5)
dec_test!(bid128_to_uint64_xrnint_381, bid128_to_uint64_xrnint, 0xB01C629B8C891B266A925ABA2C4E0001u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_382, bid128_to_uint64_xrnint, 0xB01C629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-ulp)
dec_test!(bid128_to_uint64_xrnint_383, bid128_to_uint64_xrnint, 0xB01C629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- -(2e15)
dec_test!(bid128_to_uint64_xrnint_384, bid128_to_uint64_xrnint, 0xB01C629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- -(2e15+ulp)
dec_test!(bid128_to_uint64_xrnint_385, bid128_to_uint64_xrnint, 0xB01C629B8C891B267873116DD3B1FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_386, bid128_to_uint64_xrnint, 0xB01C629B8C891B267873116DD3B20000u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5)
dec_test!(bid128_to_uint64_xrnint_387, bid128_to_uint64_xrnint, 0xB01C629B8C891B267873116DD3B20001u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_388, bid128_to_uint64_xrnint, 0xB01C629B8C891B267F636CC7A763FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15+1-ulp)
dec_test!(bid128_to_uint64_xrnint_389, bid128_to_uint64_xrnint, 0xB01C629B8C891B267F636CC7A7640000u128, 9223372036854775808  , 0x01); // -- -(2e15+1)
dec_test!(bid128_to_uint64_xrnint_390, bid128_to_uint64_xrnint, 0xB01C629B8C891B267F636CC7A7640001u128, 9223372036854775808  , 0x01); // -- -(2e15+1+ulp)
dec_test!(bid128_to_uint64_xrnint_391, bid128_to_uint64_xrnint, 0xB01C629B8C891B268653C8217B15FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_392, bid128_to_uint64_xrnint, 0xB01C629B8C891B268653C8217B160000u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5)
dec_test!(bid128_to_uint64_xrnint_393, bid128_to_uint64_xrnint, 0xB01C629B8C891B268653C8217B160001u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_394, bid128_to_uint64_xrnint, 0xB01E000000000001A055690D9DB7FFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_395, bid128_to_uint64_xrnint, 0xB01E000000000001A055690D9DB80000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_396, bid128_to_uint64_xrnint, 0xB01E000000000001A055690D9DB80001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_397, bid128_to_uint64_xrnint, 0xB01E002C68AF0BB140B1A2BC2EC4FFFFu128, 9223372036854775808  , 0x01); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_398, bid128_to_uint64_xrnint, 0xB01E002C68AF0BB140B1A2BC2EC50000u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5)
dec_test!(bid128_to_uint64_xrnint_399, bid128_to_uint64_xrnint, 0xB01E002C68AF0BB140B1A2BC2EC50001u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_400, bid128_to_uint64_xrnint, 0xB02000000000000029A2241AF62BFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_401, bid128_to_uint64_xrnint, 0xB02000000000000029A2241AF62C0000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_402, bid128_to_uint64_xrnint, 0xB02000000000000029A2241AF62C0001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_403, bid128_to_uint64_xrnint, 0xB020000470DE4DF81FFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^45-ulp)
dec_test!(bid128_to_uint64_xrnint_404, bid128_to_uint64_xrnint, 0xB020000470DE4DF82000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^45)
dec_test!(bid128_to_uint64_xrnint_405, bid128_to_uint64_xrnint, 0xB020000470DE4DF82000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^45+ulp)
dec_test!(bid128_to_uint64_xrnint_406, bid128_to_uint64_xrnint, 0xB02000FC6F7C4045813459C637E07FFFu128, 9223372036854775808  , 0x01); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_407, bid128_to_uint64_xrnint, 0xB02000FC6F7C4045813459C637E08000u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5)
dec_test!(bid128_to_uint64_xrnint_408, bid128_to_uint64_xrnint, 0xB02000FC6F7C4045813459C637E08001u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_409, bid128_to_uint64_xrnint, 0xB02000FC6F7C40458157E0B8A7A17FFFu128, 9223372036854775808  , 0x01); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_410, bid128_to_uint64_xrnint, 0xB02000FC6F7C40458157E0B8A7A18000u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5)
dec_test!(bid128_to_uint64_xrnint_411, bid128_to_uint64_xrnint, 0xB02000FC6F7C40458157E0B8A7A18001u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_412, bid128_to_uint64_xrnint, 0xB02200193E5939A08CE4879688D63FFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_413, bid128_to_uint64_xrnint, 0xB02200193E5939A08CE4879688D64000u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5)
dec_test!(bid128_to_uint64_xrnint_414, bid128_to_uint64_xrnint, 0xB02200193E5939A08CE4879688D64001u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_415, bid128_to_uint64_xrnint, 0xB02200193E5939A08CE815152D9CBFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_416, bid128_to_uint64_xrnint, 0xB02200193E5939A08CE815152D9CC000u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5)
dec_test!(bid128_to_uint64_xrnint_417, bid128_to_uint64_xrnint, 0xB02200193E5939A08CE815152D9CC001u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_418, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775808  , 0x01); // -- -(2^63-1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_419, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775808  , 0x01); // -- -(2^63-1.5)
dec_test!(bid128_to_uint64_xrnint_420, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775808  , 0x01); // -- -(2^63-1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_421, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775808  , 0x01); // -- -(2^63-1-ulp)
dec_test!(bid128_to_uint64_xrnint_422, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFC72815B398000u128, 9223372036854775808  , 0x01); // -- -(2^63-1)
dec_test!(bid128_to_uint64_xrnint_423, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFC72815B398001u128, 9223372036854775808  , 0x01); // -- -(2^63-1+ulp)
dec_test!(bid128_to_uint64_xrnint_424, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775808  , 0x01); // -- -(2^63-0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_425, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFE3940AD9CC000u128, 9223372036854775808  , 0x01); // -- -(2^63-0.5)
dec_test!(bid128_to_uint64_xrnint_426, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFE3940AD9CC001u128, 9223372036854775808  , 0x01); // -- -(2^63-0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_427, bid128_to_uint64_xrnint, 0xB023C6BF52633FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^63-ulp)
dec_test!(bid128_to_uint64_xrnint_428, bid128_to_uint64_xrnint, 0xB023C6BF526340000000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^63)
dec_test!(bid128_to_uint64_xrnint_429, bid128_to_uint64_xrnint, 0xB023C6BF526340000000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^63+ulp)
dec_test!(bid128_to_uint64_xrnint_430, bid128_to_uint64_xrnint, 0xB023C6BF526340000001C6BF52633FFFu128, 9223372036854775808  , 0x01); // -- -(2^63+0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_431, bid128_to_uint64_xrnint, 0xB023C6BF526340000001C6BF52634000u128, 9223372036854775808  , 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_uint64_xrnint_432, bid128_to_uint64_xrnint, 0xB023C6BF526340000001C6BF52634001u128, 9223372036854775808  , 0x01); // -- -(2^63+0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_433, bid128_to_uint64_xrnint, 0xB023C6BF5263400000038D7EA4C67FFFu128, 9223372036854775808  , 0x01); // -- -(2^63+1-ulp)
dec_test!(bid128_to_uint64_xrnint_434, bid128_to_uint64_xrnint, 0xB023C6BF5263400000038D7EA4C68000u128, 9223372036854775808  , 0x01); // -- -(2^63+1)
dec_test!(bid128_to_uint64_xrnint_435, bid128_to_uint64_xrnint, 0xB023C6BF5263400000038D7EA4C68001u128, 9223372036854775808  , 0x01); // -- -(2^63+1+ulp)
dec_test!(bid128_to_uint64_xrnint_436, bid128_to_uint64_xrnint, 0xB024000000000000006A94D74F42FFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_437, bid128_to_uint64_xrnint, 0xB024000000000000006A94D74F430000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_438, bid128_to_uint64_xrnint, 0xB024000000000000006A94D74F430001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_439, bid128_to_uint64_xrnint, 0xB024314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1e19-ulp)
dec_test!(bid128_to_uint64_xrnint_440, bid128_to_uint64_xrnint, 0xB024314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- -(1e19)
dec_test!(bid128_to_uint64_xrnint_441, bid128_to_uint64_xrnint, 0xB024314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- -(1e19+ulp)
dec_test!(bid128_to_uint64_xrnint_442, bid128_to_uint64_xrnint, 0xB024314DC6448D9338C18883883D1FFFu128, 9223372036854775808  , 0x01); // -- -(1e19+0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_443, bid128_to_uint64_xrnint, 0xB024314DC6448D9338C18883883D2000u128, 9223372036854775808  , 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_uint64_xrnint_444, bid128_to_uint64_xrnint, 0xB024314DC6448D9338C18883883D2001u128, 9223372036854775808  , 0x01); // -- -(1e19+0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_445, bid128_to_uint64_xrnint, 0xB02449F4A966D45CD522088EFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1.5e19-ulp)
dec_test!(bid128_to_uint64_xrnint_446, bid128_to_uint64_xrnint, 0xB02449F4A966D45CD522088F00000000u128, 9223372036854775808  , 0x01); // -- -(1.5e19)
dec_test!(bid128_to_uint64_xrnint_447, bid128_to_uint64_xrnint, 0xB02449F4A966D45CD522088F00000001u128, 9223372036854775808  , 0x01); // -- -(1.5e19+ulp)
dec_test!(bid128_to_uint64_xrnint_448, bid128_to_uint64_xrnint, 0xB0245AF3107A3FFFFFFFA50CEF85BFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-1-ulp)
dec_test!(bid128_to_uint64_xrnint_449, bid128_to_uint64_xrnint, 0xB0245AF3107A3FFFFFFFA50CEF85C000u128, 9223372036854775808  , 0x01); // -- -(2^64-1)
dec_test!(bid128_to_uint64_xrnint_450, bid128_to_uint64_xrnint, 0xB0245AF3107A3FFFFFFFA50CEF85C001u128, 9223372036854775808  , 0x01); // -- -(2^64-1+ulp)
dec_test!(bid128_to_uint64_xrnint_451, bid128_to_uint64_xrnint, 0xB0245AF3107A3FFFFFFFD28677C2DFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_452, bid128_to_uint64_xrnint, 0xB0245AF3107A3FFFFFFFD28677C2E000u128, 9223372036854775808  , 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_uint64_xrnint_453, bid128_to_uint64_xrnint, 0xB0245AF3107A3FFFFFFFD28677C2E001u128, 9223372036854775808  , 0x01); // -- -(2^64-0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_454, bid128_to_uint64_xrnint, 0xB0245AF3107A3FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-ulp)
dec_test!(bid128_to_uint64_xrnint_455, bid128_to_uint64_xrnint, 0xB0245AF3107A40000000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^64)
dec_test!(bid128_to_uint64_xrnint_456, bid128_to_uint64_xrnint, 0xB0245AF3107A40000000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^64+ulp)
dec_test!(bid128_to_uint64_xrnint_457, bid128_to_uint64_xrnint, 0xB0245AF3107A400000002D79883D1FFFu128, 9223372036854775808  , 0x01); // -- -(2^64+0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_458, bid128_to_uint64_xrnint, 0xB0245AF3107A400000002D79883D2000u128, 9223372036854775808  , 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_uint64_xrnint_459, bid128_to_uint64_xrnint, 0xB0245AF3107A400000002D79883D2001u128, 9223372036854775808  , 0x01); // -- -(2^64+0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_460, bid128_to_uint64_xrnint, 0xB0245AF3107A400000005AF3107A3FFFu128, 9223372036854775808  , 0x01); // -- -(2^64+1-ulp)
dec_test!(bid128_to_uint64_xrnint_461, bid128_to_uint64_xrnint, 0xB0245AF3107A400000005AF3107A4000u128, 9223372036854775808  , 0x01); // -- -(2^64+1)
dec_test!(bid128_to_uint64_xrnint_462, bid128_to_uint64_xrnint, 0xB0245AF3107A400000005AF3107A4001u128, 9223372036854775808  , 0x01); // -- -(2^64+1+ulp)
dec_test!(bid128_to_uint64_xrnint_463, bid128_to_uint64_xrnint, 0xB024629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e19-ulp)
dec_test!(bid128_to_uint64_xrnint_464, bid128_to_uint64_xrnint, 0xB024629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- -(2e19)
dec_test!(bid128_to_uint64_xrnint_465, bid128_to_uint64_xrnint, 0xB024629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- -(2e19+ulp)
dec_test!(bid128_to_uint64_xrnint_466, bid128_to_uint64_xrnint, 0xB0247B426FAB61F00DE36398FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2.5e19-ulp)
dec_test!(bid128_to_uint64_xrnint_467, bid128_to_uint64_xrnint, 0xB0247B426FAB61F00DE3639900000000u128, 9223372036854775808  , 0x01); // -- -(2.5e19)
dec_test!(bid128_to_uint64_xrnint_468, bid128_to_uint64_xrnint, 0xB0247B426FAB61F00DE3639900000001u128, 9223372036854775808  , 0x01); // -- -(2.5e19+ulp)
dec_test!(bid128_to_uint64_xrnint_469, bid128_to_uint64_xrnint, 0xB026314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1e20-ulp)
dec_test!(bid128_to_uint64_xrnint_470, bid128_to_uint64_xrnint, 0xB026314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- -(1e20)
dec_test!(bid128_to_uint64_xrnint_471, bid128_to_uint64_xrnint, 0xB026314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- -(1e20+ulp)
dec_test!(bid128_to_uint64_xrnint_472, bid128_to_uint64_xrnint, 0xb0280468c10200730020002000021000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_473, bid128_to_uint64_xrnint, 0xB02A00000000006C6B935B68D08DA3FFu128, 9223372036854775808  , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_474, bid128_to_uint64_xrnint, 0xB02A00000000006C6B935B68D08DA400u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint64_xrnint_475, bid128_to_uint64_xrnint, 0xB02A00000000006C6B935B68D08DA401u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_476, bid128_to_uint64_xrnint, 0xB02A00000000006C6B935B8019048BFFu128, 9223372036854775808  , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_477, bid128_to_uint64_xrnint, 0xB02A00000000006C6B935B8019048C00u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint64_xrnint_478, bid128_to_uint64_xrnint, 0xB02A00000000006C6B935B8019048C01u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_479, bid128_to_uint64_xrnint, 0xB02C000000000000000002BBA7F521FFu128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xrnint_480, bid128_to_uint64_xrnint, 0xB02C000000000000000002BBA7F52200u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xrnint_481, bid128_to_uint64_xrnint, 0xB02C000000000000000002BBA7F52201u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xrnint_482, bid128_to_uint64_xrnint, 0xB02C00000000000AD78EBC5872141BFFu128, 9223372036854775808  , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint64_xrnint_483, bid128_to_uint64_xrnint, 0xB02C00000000000AD78EBC5872141C00u128, 9223372036854775808  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint64_xrnint_484, bid128_to_uint64_xrnint, 0xB02C00000000000AD78EBC5872141C01u128, 9223372036854775808  , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint64_xrnint_485, bid128_to_uint64_xrnint, 0xB02C00000000000AD78EBC5BF025F1FFu128, 9223372036854775808  , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint64_xrnint_486, bid128_to_uint64_xrnint, 0xB02C00000000000AD78EBC5BF025F200u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint64_xrnint_487, bid128_to_uint64_xrnint, 0xB02C00000000000AD78EBC5BF025F201u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint64_xrnint_488, bid128_to_uint64_xrnint, 0xB02C00000000000AD78EBC5E4431D5FFu128, 9223372036854775808  , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint64_xrnint_489, bid128_to_uint64_xrnint, 0xB02C00000000000AD78EBC5E4431D600u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint64_xrnint_490, bid128_to_uint64_xrnint, 0xB02C00000000000AD78EBC5E4431D601u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint64_xrnint_491, bid128_to_uint64_xrnint, 0xB02C000000108B2A2C28028E3FF41BFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1-ulp)
dec_test!(bid128_to_uint64_xrnint_492, bid128_to_uint64_xrnint, 0xB02C000000108B2A2C28028E3FF41C00u128, 9223372036854775808  , 0x01); // -- -(2e15-1)
dec_test!(bid128_to_uint64_xrnint_493, bid128_to_uint64_xrnint, 0xB02C000000108B2A2C28028E3FF41C01u128, 9223372036854775808  , 0x01); // -- -(2e15-1+ulp)
dec_test!(bid128_to_uint64_xrnint_494, bid128_to_uint64_xrnint, 0xB02E000000000001158E46094F6AC9FFu128, 9223372036854775808  , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint64_xrnint_495, bid128_to_uint64_xrnint, 0xB02E000000000001158E46094F6ACA00u128, 9223372036854775808  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint64_xrnint_496, bid128_to_uint64_xrnint, 0xB02E000000000001158E46094F6ACA01u128, 9223372036854775808  , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint64_xrnint_497, bid128_to_uint64_xrnint, 0xB02E00000001A784379D99DB7D9AC9FFu128, 9223372036854775808  , 0x01); // -- -(2e15+1-ulp)
dec_test!(bid128_to_uint64_xrnint_498, bid128_to_uint64_xrnint, 0xB02E00000001A784379D99DB7D9ACA00u128, 9223372036854775808  , 0x01); // -- -(2e15+1)
dec_test!(bid128_to_uint64_xrnint_499, bid128_to_uint64_xrnint, 0xB02E00000001A784379D99DB7D9ACA01u128, 9223372036854775808  , 0x01); // -- -(2e15+1+ulp)
dec_test!(bid128_to_uint64_xrnint_500, bid128_to_uint64_xrnint, 0xB03000000000000000000006FC23ABFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_501, bid128_to_uint64_xrnint, 0xB03000000000000000000006FC23AC00u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_502, bid128_to_uint64_xrnint, 0xB03000000000000000000006FC23AC01u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_503, bid128_to_uint64_xrnint, 0xB03200000000000000000000B2D05DFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_504, bid128_to_uint64_xrnint, 0xB03200000000000000000000B2D05E00u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_505, bid128_to_uint64_xrnint, 0xB03200000000000000000000B2D05E01u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_506, bid128_to_uint64_xrnint, 0xB03800000000000000000000002DDA47u128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xrnint_507, bid128_to_uint64_xrnint, 0xB03800000000000000000000002DDA48u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xrnint_508, bid128_to_uint64_xrnint, 0xB03800000000000000000000002DDA49u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xrnint_509, bid128_to_uint64_xrnint, 0xB03A00000000000000000000000003E7u128, 9223372036854775808  , 0x01); // -- -(0.999)
dec_test!(bid128_to_uint64_xrnint_510, bid128_to_uint64_xrnint, 0xB03A00000000000000000000000495D3u128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xrnint_511, bid128_to_uint64_xrnint, 0xB03A00000000000000000000000495D4u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xrnint_512, bid128_to_uint64_xrnint, 0xB03A00000000000000000000000495D5u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xrnint_513, bid128_to_uint64_xrnint, 0xB03C0000000000000000000000007561u128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xrnint_514, bid128_to_uint64_xrnint, 0xB03C0000000000000000000000007562u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xrnint_515, bid128_to_uint64_xrnint, 0xB03C0000000000000000000000007563u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xrnint_516, bid128_to_uint64_xrnint, 0xB03E0000000000000000000000000005u128, 0                    , 0x20); // -- -(0.5)
dec_test!(bid128_to_uint64_xrnint_517, bid128_to_uint64_xrnint, 0xB03E000000000000000000000000000Fu128, 9223372036854775808  , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint64_xrnint_518, bid128_to_uint64_xrnint, 0xB03E0000000000000000000000000BB7u128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_519, bid128_to_uint64_xrnint, 0xB03E0000000000000000000000000BB8u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_520, bid128_to_uint64_xrnint, 0xB03E0000000000000000000000000BB9u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_521, bid128_to_uint64_xrnint, 0xB03E0000000000000000000000000BBDu128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xrnint_522, bid128_to_uint64_xrnint, 0xB03E0000000000000000002E90EDCFF1u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint64_xrnint_523, bid128_to_uint64_xrnint, 0xB03E0000000000000000002E90EDCFFBu128, 9223372036854775808  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint64_xrnint_524, bid128_to_uint64_xrnint, 0xB03E0000000000000000002E90EDD005u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint64_xrnint_525, bid128_to_uint64_xrnint, 0xB03E0000000000000000002E90EDD00Fu128, 9223372036854775808  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint64_xrnint_526, bid128_to_uint64_xrnint, 0xB03E0000000000000001400000000005u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5)
dec_test!(bid128_to_uint64_xrnint_527, bid128_to_uint64_xrnint, 0xB03E00000000000000470DE4DF81FFF1u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5)
dec_test!(bid128_to_uint64_xrnint_528, bid128_to_uint64_xrnint, 0xB03E00000000000000470DE4DF81FFFBu128, 9223372036854775808  , 0x01); // -- -(2e15-0.5)
dec_test!(bid128_to_uint64_xrnint_529, bid128_to_uint64_xrnint, 0xB03E00000000000000470DE4DF820005u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5)
dec_test!(bid128_to_uint64_xrnint_530, bid128_to_uint64_xrnint, 0xB03E00000000000000470DE4DF82000Fu128, 9223372036854775808  , 0x01); // -- -(2e15+1.5)
dec_test!(bid128_to_uint64_xrnint_531, bid128_to_uint64_xrnint, 0xB03E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775808  , 0x01); // -- -(2^63-1.5)
dec_test!(bid128_to_uint64_xrnint_532, bid128_to_uint64_xrnint, 0xB03E000000000004FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x01); // -- -(2^63-0.5)
dec_test!(bid128_to_uint64_xrnint_533, bid128_to_uint64_xrnint, 0xB03E0000000000050000000000000005u128, 9223372036854775808  , 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_uint64_xrnint_534, bid128_to_uint64_xrnint, 0xB03E0000000000056BC75E2D63100005u128, 9223372036854775808  , 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_uint64_xrnint_535, bid128_to_uint64_xrnint, 0xB03E000000000009FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_uint64_xrnint_536, bid128_to_uint64_xrnint, 0xB03E00000000000A0000000000000005u128, 9223372036854775808  , 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_uint64_xrnint_537, bid128_to_uint64_xrnint, 0xB0400000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- -(1)
dec_test!(bid128_to_uint64_xrnint_538, bid128_to_uint64_xrnint, 0xB040000000000000000000000000012Bu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_539, bid128_to_uint64_xrnint, 0xB040000000000000000000000000012Cu128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_540, bid128_to_uint64_xrnint, 0xB040000000000000000000000000012Du128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_541, bid128_to_uint64_xrnint, 0xB04000000000000000000004A817C7FFu128, 9223372036854775808  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint64_xrnint_542, bid128_to_uint64_xrnint, 0xB04000000000000000000004A817C801u128, 9223372036854775808  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint64_xrnint_543, bid128_to_uint64_xrnint, 0xB0400000000000000000200000000000u128, 9223372036854775808  , 0x01); // -- -(2^45)
dec_test!(bid128_to_uint64_xrnint_544, bid128_to_uint64_xrnint, 0xB04000000000000000071AFD498CFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1)
dec_test!(bid128_to_uint64_xrnint_545, bid128_to_uint64_xrnint, 0xB04000000000000000071AFD498CFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-ulp)
dec_test!(bid128_to_uint64_xrnint_546, bid128_to_uint64_xrnint, 0xB04000000000000000071AFD498D0000u128, 9223372036854775808  , 0x01); // -- -(2e15)
dec_test!(bid128_to_uint64_xrnint_547, bid128_to_uint64_xrnint, 0xB04000000000000000071AFD498D0001u128, 9223372036854775808  , 0x01); // -- -(2e15+1)
dec_test!(bid128_to_uint64_xrnint_548, bid128_to_uint64_xrnint, 0xB04000000000000000071AFD498D0001u128, 9223372036854775808  , 0x01); // -- -(2e15+ulp)
dec_test!(bid128_to_uint64_xrnint_549, bid128_to_uint64_xrnint, 0xB0400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^63-1)
dec_test!(bid128_to_uint64_xrnint_550, bid128_to_uint64_xrnint, 0xB0400000000000008000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^63)
dec_test!(bid128_to_uint64_xrnint_551, bid128_to_uint64_xrnint, 0xB0400000000000008000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^63+1)
dec_test!(bid128_to_uint64_xrnint_552, bid128_to_uint64_xrnint, 0xB040000000000000FFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-1)
dec_test!(bid128_to_uint64_xrnint_553, bid128_to_uint64_xrnint, 0xB0400000000000010000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^64)
dec_test!(bid128_to_uint64_xrnint_554, bid128_to_uint64_xrnint, 0xB0400000000000010000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^64+1)
dec_test!(bid128_to_uint64_xrnint_555, bid128_to_uint64_xrnint, 0xB042000000000000000000000000001Du128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xrnint_556, bid128_to_uint64_xrnint, 0xB042000000000000000000000000001Eu128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_557, bid128_to_uint64_xrnint, 0xB042000000000000000000000000001Fu128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xrnint_558, bid128_to_uint64_xrnint, 0xB04200000000000000000000773593FFu128, 9223372036854775808  , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint64_xrnint_559, bid128_to_uint64_xrnint, 0xB0420000000000000000000077359400u128, 9223372036854775808  , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint64_xrnint_560, bid128_to_uint64_xrnint, 0xB0420000000000000000000077359401u128, 9223372036854775808  , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint64_xrnint_561, bid128_to_uint64_xrnint, 0xB0440000000000000000000000000003u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xrnint_562, bid128_to_uint64_xrnint, 0xB0520000000000000000000000000004u128, 9223372036854775808  , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint64_xrnint_563, bid128_to_uint64_xrnint, 0xB0520000000000000000000000000005u128, 9223372036854775808  , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint64_xrnint_564, bid128_to_uint64_xrnint, 0xB0540000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint64_xrnint_565, bid128_to_uint64_xrnint, 0xB05E0000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- -(2e15)
dec_test!(bid128_to_uint64_xrnint_566, bid128_to_uint64_xrnint, 0xB064000000000000000000000000000Fu128, 9223372036854775808  , 0x01); // -- -(1.5e19)
dec_test!(bid128_to_uint64_xrnint_567, bid128_to_uint64_xrnint, 0xB0640000000000000000000000000019u128, 9223372036854775808  , 0x01); // -- -(2.5e19)
dec_test!(bid128_to_uint64_xrnint_568, bid128_to_uint64_xrnint, 0xB0660000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- -(1e19)
dec_test!(bid128_to_uint64_xrnint_569, bid128_to_uint64_xrnint, 0xB0660000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- -(2e19)
dec_test!(bid128_to_uint64_xrnint_570, bid128_to_uint64_xrnint, 0xB0680000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- -(1e20)
dec_test!(bid128_to_uint64_xrnint_571, bid128_to_uint64_xrnint, 0xb344fab8a18bfd6df3a2cd02b0d8c4ddu128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xrnint_572, bid128_to_uint64_xrnint, 0xce9c1612010283c9842217081a055491u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_573, bid128_to_uint64_xrnint, 0xf236c9c916900ccdd3c013af0c55839au128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xrnint_574, bid128_to_uint64_xrnint, 0xf812d8630b9c5e82a8b03f0fc861645cu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_575, bid128_to_uint64_xrnint, 0xfc000000000000000000000000000000u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xrnint_576, bid128_to_uint64_xrnint, 0xfc0019003fbb2c35627b30b61c33f5ffu128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xrnint_577, bid128_to_uint64_xrnint, 0xfcf5b1804ba8bcd733beb5408ab2064au128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_578, bid128_to_uint64_xrnint, 0xfe002d0e4aee9999edf6252866e5f201u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xrnint_579, bid128_to_uint64_xrnint, 0xfedf77bd47bef4f1ec6310ad202025a9u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xrnint_580, bid128_to_uint64_xrnint, "Infinity"                            , 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xrnint_581, bid128_to_uint64_xrnint, "SNaN"                                , 0x8000000000000000u64, 0x01);
