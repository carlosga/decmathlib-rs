/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_uint64_ceil_001, bid128_to_uint64_ceil, "-0"                                  , 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_002, bid128_to_uint64_ceil, 0x00000000000000000a00400202000840u128, 1                    , 0x00);
dec_test!(bid128_to_uint64_ceil_003, bid128_to_uint64_ceil, 0x0001ed09bead87c0378d8e62ffffffffu128, 1                    , 0x00);
dec_test!(bid128_to_uint64_ceil_004, bid128_to_uint64_ceil, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_005, bid128_to_uint64_ceil, 0x00420000000000000000000000000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_006, bid128_to_uint64_ceil, 0x0ef813273386a4c23a074fe19f41f24eu128, 1                    , 0x00);
dec_test!(bid128_to_uint64_ceil_007, bid128_to_uint64_ceil, "1.0"                                 , 1                    , 0x00);
dec_test!(bid128_to_uint64_ceil_008, bid128_to_uint64_ceil, 0x10f80000000000000000000000000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_009, bid128_to_uint64_ceil, 0x15bf2793b8abf833f63523aa1d31bbc7u128, 1                    , 0x00);
dec_test!(bid128_to_uint64_ceil_010, bid128_to_uint64_ceil, 0x28e400d40a40011d3637e78d5c7a9757u128, 1                    , 0x00);
dec_test!(bid128_to_uint64_ceil_011, bid128_to_uint64_ceil, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 1                    , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_uint64_ceil_012, bid128_to_uint64_ceil, 0x2FFCF684DF56C3E01BC6C73200000000u128, 1                    , 0x00); // -- 0.5
dec_test!(bid128_to_uint64_ceil_013, bid128_to_uint64_ceil, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1                    , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_uint64_ceil_014, bid128_to_uint64_ceil, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1                    , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_uint64_ceil_015, bid128_to_uint64_ceil, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1                    , 0x00); // -- 0.999
dec_test!(bid128_to_uint64_ceil_016, bid128_to_uint64_ceil, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1                    , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_uint64_ceil_017, bid128_to_uint64_ceil, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1                    , 0x00); // -- 1-ulp
dec_test!(bid128_to_uint64_ceil_018, bid128_to_uint64_ceil, 0x2FFE314DC6448D9338C15B0A00000000u128, 1                    , 0x00); // -- 1
dec_test!(bid128_to_uint64_ceil_019, bid128_to_uint64_ceil, 0x2FFE314DC6448D9338C15B0A00000001u128, 2                    , 0x00); // -- 1+ulp
dec_test!(bid128_to_uint64_ceil_020, bid128_to_uint64_ceil, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 2                    , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_uint64_ceil_021, bid128_to_uint64_ceil, 0x2FFE49F4A966D45CD522088F00000000u128, 2                    , 0x00); // -- 1.5
dec_test!(bid128_to_uint64_ceil_022, bid128_to_uint64_ceil, 0x2FFE49F4A966D45CD522088F00000001u128, 2                    , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_uint64_ceil_023, bid128_to_uint64_ceil, 0x30022804044600063f96ff8d85eb6af7u128, 82                   , 0x00);
dec_test!(bid128_to_uint64_ceil_024, bid128_to_uint64_ceil, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_025, bid128_to_uint64_ceil, 0x300293E952CDA8B9AA44111E00000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_026, bid128_to_uint64_ceil, 0x300293E952CDA8B9AA44111E00000001u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_027, bid128_to_uint64_ceil, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 301                  , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint64_ceil_028, bid128_to_uint64_ceil, 0x300294286EACB8CB0A8CB6B140000000u128, 301                  , 0x00); // -- 300.5
dec_test!(bid128_to_uint64_ceil_029, bid128_to_uint64_ceil, 0x300294286EACB8CB0A8CB6B140000001u128, 301                  , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint64_ceil_030, bid128_to_uint64_ceil, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_031, bid128_to_uint64_ceil, 0x30040ECA8847C4129106CE8300000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_032, bid128_to_uint64_ceil, 0x30040ECA8847C4129106CE8300000001u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_033, bid128_to_uint64_ceil, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_034, bid128_to_uint64_ceil, 0x300A0003C95A2F0B4856475FE0000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_035, bid128_to_uint64_ceil, 0x300A0003C95A2F0B4856475FE0000001u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_036, bid128_to_uint64_ceil, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_037, bid128_to_uint64_ceil, 0x300C000060EF6B1ABA6F072330000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_038, bid128_to_uint64_ceil, 0x300C000060EF6B1ABA6F072330000001u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_039, bid128_to_uint64_ceil, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000           , 0x00); // -- 4e9-ulp
dec_test!(bid128_to_uint64_ceil_040, bid128_to_uint64_ceil, 0x3010C5371912364CE3056C2800000000u128, 4000000000           , 0x00); // -- 4e9
dec_test!(bid128_to_uint64_ceil_041, bid128_to_uint64_ceil, 0x3010C5371912364CE3056C2800000001u128, 4000000001           , 0x00); // -- 4e9+ulp
dec_test!(bid128_to_uint64_ceil_042, bid128_to_uint64_ceil, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 5000000000           , 0x00); // -- 5e9-ulp
dec_test!(bid128_to_uint64_ceil_043, bid128_to_uint64_ceil, 0x3010F684DF56C3E01BC6C73200000000u128, 5000000000           , 0x00); // -- 5e9
dec_test!(bid128_to_uint64_ceil_044, bid128_to_uint64_ceil, 0x3010F684DF56C3E01BC6C73200000001u128, 5000000001           , 0x00); // -- 5e9+ulp
dec_test!(bid128_to_uint64_ceil_045, bid128_to_uint64_ceil, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 19999999999          , 0x00); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint64_ceil_046, bid128_to_uint64_ceil, 0x3012629B8C88FB62ED56E4238E400000u128, 19999999999          , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_uint64_ceil_047, bid128_to_uint64_ceil, 0x3012629B8C88FB62ED56E4238E400001u128, 19999999999          , 0x00); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint64_ceil_048, bid128_to_uint64_ceil, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 19999999999          , 0x00); // -- 2e10-1-ulp
dec_test!(bid128_to_uint64_ceil_049, bid128_to_uint64_ceil, 0x3012629B8C8905F96EBAD4C909800000u128, 19999999999          , 0x00); // -- 2e10-1
dec_test!(bid128_to_uint64_ceil_050, bid128_to_uint64_ceil, 0x3012629B8C8905F96EBAD4C909800001u128, 20000000000          , 0x00); // -- 2e10-1+ulp
dec_test!(bid128_to_uint64_ceil_051, bid128_to_uint64_ceil, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 20000000000          , 0x00); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint64_ceil_052, bid128_to_uint64_ceil, 0x3012629B8C89108FF01EC56E84C00000u128, 20000000000          , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_uint64_ceil_053, bid128_to_uint64_ceil, 0x3012629B8C89108FF01EC56E84C00001u128, 20000000000          , 0x00); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint64_ceil_054, bid128_to_uint64_ceil, 0x3012629B8C891B267182B613FFFFFFFFu128, 20000000000          , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_uint64_ceil_055, bid128_to_uint64_ceil, 0x3012629B8C891B267182B61400000000u128, 20000000000          , 0x00); // -- 2e10
dec_test!(bid128_to_uint64_ceil_056, bid128_to_uint64_ceil, 0x3012629B8C891B267182B61400000001u128, 20000000001          , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_uint64_ceil_057, bid128_to_uint64_ceil, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 20000000001          , 0x00); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint64_ceil_058, bid128_to_uint64_ceil, 0x3012629B8C8925BCF2E6A6B97B400000u128, 20000000001          , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_uint64_ceil_059, bid128_to_uint64_ceil, 0x3012629B8C8925BCF2E6A6B97B400001u128, 20000000001          , 0x00); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint64_ceil_060, bid128_to_uint64_ceil, 0x3012629B8C893053744A975EF67FFFFFu128, 20000000001          , 0x00); // -- 2e10+1-ulp
dec_test!(bid128_to_uint64_ceil_061, bid128_to_uint64_ceil, 0x3012629B8C893053744A975EF6800000u128, 20000000001          , 0x00); // -- 2e10+1
dec_test!(bid128_to_uint64_ceil_062, bid128_to_uint64_ceil, 0x3012629B8C893053744A975EF6800001u128, 20000000002          , 0x00); // -- 2e10+1+ulp
dec_test!(bid128_to_uint64_ceil_063, bid128_to_uint64_ceil, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 20000000002          , 0x00); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint64_ceil_064, bid128_to_uint64_ceil, 0x3012629B8C893AE9F5AE880471C00000u128, 20000000002          , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_uint64_ceil_065, bid128_to_uint64_ceil, 0x3012629B8C893AE9F5AE880471C00001u128, 20000000002          , 0x00); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint64_ceil_066, bid128_to_uint64_ceil, 0x3018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 35184372088832       , 0x00); // -- 2^45-ulp
dec_test!(bid128_to_uint64_ceil_067, bid128_to_uint64_ceil, 0x3018AD78EBC5AC620000000000000000u128, 35184372088832       , 0x00); // -- 2^45
dec_test!(bid128_to_uint64_ceil_068, bid128_to_uint64_ceil, 0x3018AD78EBC5AC620000000000000001u128, 35184372088833       , 0x00); // -- 2^45+ulp
dec_test!(bid128_to_uint64_ceil_069, bid128_to_uint64_ceil, 0x3018AD78EBC5AC64B5E3AF16B187FFFFu128, 35184372088833       , 0x00); // -- 2^45+0.5-ulp
dec_test!(bid128_to_uint64_ceil_070, bid128_to_uint64_ceil, 0x3018AD78EBC5AC64B5E3AF16B1880000u128, 35184372088833       , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_uint64_ceil_071, bid128_to_uint64_ceil, 0x3018AD78EBC5AC64B5E3AF16B1880001u128, 35184372088833       , 0x00); // -- 2^45+0.5+ulp
dec_test!(bid128_to_uint64_ceil_072, bid128_to_uint64_ceil, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 301                  , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint64_ceil_073, bid128_to_uint64_ceil, 0x301A0000000000A2E6C09AD3E0D40000u128, 301                  , 0x00); // -- 300.5
dec_test!(bid128_to_uint64_ceil_074, bid128_to_uint64_ceil, 0x301A0000000000A2E6C09AD3E0D40001u128, 301                  , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint64_ceil_075, bid128_to_uint64_ceil, 0x301C629B8C891B265CB1A40684E9FFFFu128, 1999999999999999     , 0x00); // -- 2e15-1.5-ulp
dec_test!(bid128_to_uint64_ceil_076, bid128_to_uint64_ceil, 0x301C629B8C891B265CB1A40684EA0000u128, 1999999999999999     , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_uint64_ceil_077, bid128_to_uint64_ceil, 0x301C629B8C891B265CB1A40684EA0001u128, 1999999999999999     , 0x00); // -- 2e15-1.5+ulp
dec_test!(bid128_to_uint64_ceil_078, bid128_to_uint64_ceil, 0x301C629B8C891B2663A1FF60589BFFFFu128, 1999999999999999     , 0x00); // -- 2e15-1-ulp
dec_test!(bid128_to_uint64_ceil_079, bid128_to_uint64_ceil, 0x301C629B8C891B2663A1FF60589C0000u128, 1999999999999999     , 0x00); // -- 2e15-1
dec_test!(bid128_to_uint64_ceil_080, bid128_to_uint64_ceil, 0x301C629B8C891B2663A1FF60589C0001u128, 2000000000000000     , 0x00); // -- 2e15-1+ulp
dec_test!(bid128_to_uint64_ceil_081, bid128_to_uint64_ceil, 0x301C629B8C891B266A925ABA2C4DFFFFu128, 2000000000000000     , 0x00); // -- 2e15-0.5-ulp
dec_test!(bid128_to_uint64_ceil_082, bid128_to_uint64_ceil, 0x301C629B8C891B266A925ABA2C4E0000u128, 2000000000000000     , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_uint64_ceil_083, bid128_to_uint64_ceil, 0x301C629B8C891B266A925ABA2C4E0001u128, 2000000000000000     , 0x00); // -- 2e15-0.5+ulp
dec_test!(bid128_to_uint64_ceil_084, bid128_to_uint64_ceil, 0x301C629B8C891B267182B613FFFFFFFFu128, 2000000000000000     , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_uint64_ceil_085, bid128_to_uint64_ceil, 0x301C629B8C891B267182B61400000000u128, 2000000000000000     , 0x00); // -- 2e15
dec_test!(bid128_to_uint64_ceil_086, bid128_to_uint64_ceil, 0x301C629B8C891B267182B61400000001u128, 2000000000000001     , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_uint64_ceil_087, bid128_to_uint64_ceil, 0x301C629B8C891B267873116DD3B1FFFFu128, 2000000000000001     , 0x00); // -- 2e15+0.5-ulp
dec_test!(bid128_to_uint64_ceil_088, bid128_to_uint64_ceil, 0x301C629B8C891B267873116DD3B20000u128, 2000000000000001     , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_uint64_ceil_089, bid128_to_uint64_ceil, 0x301C629B8C891B267873116DD3B20001u128, 2000000000000001     , 0x00); // -- 2e15+0.5+ulp
dec_test!(bid128_to_uint64_ceil_090, bid128_to_uint64_ceil, 0x301C629B8C891B267F636CC7A763FFFFu128, 2000000000000001     , 0x00); // -- 2e15+1-ulp
dec_test!(bid128_to_uint64_ceil_091, bid128_to_uint64_ceil, 0x301C629B8C891B267F636CC7A7640000u128, 2000000000000001     , 0x00); // -- 2e15+1
dec_test!(bid128_to_uint64_ceil_092, bid128_to_uint64_ceil, 0x301C629B8C891B267F636CC7A7640001u128, 2000000000000002     , 0x00); // -- 2e15+1+ulp
dec_test!(bid128_to_uint64_ceil_093, bid128_to_uint64_ceil, 0x301C629B8C891B268653C8217B15FFFFu128, 2000000000000002     , 0x00); // -- 2e15+1.5-ulp
dec_test!(bid128_to_uint64_ceil_094, bid128_to_uint64_ceil, 0x301C629B8C891B268653C8217B160000u128, 2000000000000002     , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_uint64_ceil_095, bid128_to_uint64_ceil, 0x301C629B8C891B268653C8217B160001u128, 2000000000000002     , 0x00); // -- 2e15+1.5+ulp
dec_test!(bid128_to_uint64_ceil_096, bid128_to_uint64_ceil, 0x301E000000000001A055690D9DB7FFFFu128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_097, bid128_to_uint64_ceil, 0x301E000000000001A055690D9DB80000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_098, bid128_to_uint64_ceil, 0x301E000000000001A055690D9DB80001u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_099, bid128_to_uint64_ceil, 0x301E002C68AF0BB140B1A2BC2EC4FFFFu128, 35184372088833       , 0x00); // -- 2^45+0.5-ulp
dec_test!(bid128_to_uint64_ceil_100, bid128_to_uint64_ceil, 0x301E002C68AF0BB140B1A2BC2EC50000u128, 35184372088833       , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_uint64_ceil_101, bid128_to_uint64_ceil, 0x301E002C68AF0BB140B1A2BC2EC50001u128, 35184372088833       , 0x00); // -- 2^45+0.5+ulp
dec_test!(bid128_to_uint64_ceil_102, bid128_to_uint64_ceil, 0x302000000000000029A2241AF62BFFFFu128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_103, bid128_to_uint64_ceil, 0x302000000000000029A2241AF62C0000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_104, bid128_to_uint64_ceil, 0x302000000000000029A2241AF62C0001u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_105, bid128_to_uint64_ceil, 0x3020000470DE4DF81FFFFFFFFFFFFFFFu128, 35184372088832       , 0x00); // -- 2^45-ulp
dec_test!(bid128_to_uint64_ceil_106, bid128_to_uint64_ceil, 0x3020000470DE4DF82000000000000000u128, 35184372088832       , 0x00); // -- 2^45
dec_test!(bid128_to_uint64_ceil_107, bid128_to_uint64_ceil, 0x3020000470DE4DF82000000000000001u128, 35184372088833       , 0x00); // -- 2^45+ulp
dec_test!(bid128_to_uint64_ceil_108, bid128_to_uint64_ceil, 0x302000FC6F7C4045813459C637E07FFFu128, 2000000000000001     , 0x00); // -- 2e15+0.5-ulp
dec_test!(bid128_to_uint64_ceil_109, bid128_to_uint64_ceil, 0x302000FC6F7C4045813459C637E08000u128, 2000000000000001     , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_uint64_ceil_110, bid128_to_uint64_ceil, 0x302000FC6F7C4045813459C637E08001u128, 2000000000000001     , 0x00); // -- 2e15+0.5+ulp
dec_test!(bid128_to_uint64_ceil_111, bid128_to_uint64_ceil, 0x302000FC6F7C40458157E0B8A7A17FFFu128, 2000000000000002     , 0x00); // -- 2e15+1.5-ulp
dec_test!(bid128_to_uint64_ceil_112, bid128_to_uint64_ceil, 0x302000FC6F7C40458157E0B8A7A18000u128, 2000000000000002     , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_uint64_ceil_113, bid128_to_uint64_ceil, 0x302000FC6F7C40458157E0B8A7A18001u128, 2000000000000002     , 0x00); // -- 2e15+1.5+ulp
dec_test!(bid128_to_uint64_ceil_114, bid128_to_uint64_ceil, 0x302200193E5939A08CE4879688D63FFFu128, 1999999999999999     , 0x00); // -- 2e15-1.5-ulp
dec_test!(bid128_to_uint64_ceil_115, bid128_to_uint64_ceil, 0x302200193E5939A08CE4879688D64000u128, 1999999999999999     , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_uint64_ceil_116, bid128_to_uint64_ceil, 0x302200193E5939A08CE4879688D64001u128, 1999999999999999     , 0x00); // -- 2e15-1.5+ulp
dec_test!(bid128_to_uint64_ceil_117, bid128_to_uint64_ceil, 0x302200193E5939A08CE815152D9CBFFFu128, 2000000000000000     , 0x00); // -- 2e15-0.5-ulp
dec_test!(bid128_to_uint64_ceil_118, bid128_to_uint64_ceil, 0x302200193E5939A08CE815152D9CC000u128, 2000000000000000     , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_uint64_ceil_119, bid128_to_uint64_ceil, 0x302200193E5939A08CE815152D9CC001u128, 2000000000000000     , 0x00); // -- 2e15-0.5+ulp
dec_test!(bid128_to_uint64_ceil_120, bid128_to_uint64_ceil, 0x30235814c7ca909afffffffffeffffffu128, 6978795299309309942  , 0x00);
dec_test!(bid128_to_uint64_ceil_121, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775807  , 0x00); // -- 2^63-1.5-ulp
dec_test!(bid128_to_uint64_ceil_122, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775807  , 0x00); // -- 2^63-1.5
dec_test!(bid128_to_uint64_ceil_123, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775807  , 0x00); // -- 2^63-1.5+ulp
dec_test!(bid128_to_uint64_ceil_124, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775807  , 0x00); // -- 2^63-1-ulp
dec_test!(bid128_to_uint64_ceil_125, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFC72815B398000u128, 9223372036854775807  , 0x00); // -- 2^63-1
dec_test!(bid128_to_uint64_ceil_126, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFC72815B398001u128, 9223372036854775808  , 0x00); // -- 2^63-1+ulp
dec_test!(bid128_to_uint64_ceil_127, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775808  , 0x00); // -- 2^63-0.5-ulp
dec_test!(bid128_to_uint64_ceil_128, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFE3940AD9CC000u128, 9223372036854775808  , 0x00); // -- 2^63-0.5
dec_test!(bid128_to_uint64_ceil_129, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFE3940AD9CC001u128, 9223372036854775808  , 0x00); // -- 2^63-0.5+ulp
dec_test!(bid128_to_uint64_ceil_130, bid128_to_uint64_ceil, 0x3023C6BF52633FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x00); // -- 2^63-ulp
dec_test!(bid128_to_uint64_ceil_131, bid128_to_uint64_ceil, 0x3023C6BF526340000000000000000000u128, 9223372036854775808  , 0x00); // -- 2^63
dec_test!(bid128_to_uint64_ceil_132, bid128_to_uint64_ceil, 0x3023C6BF526340000000000000000001u128, 9223372036854775809  , 0x00); // -- 2^63+ulp
dec_test!(bid128_to_uint64_ceil_133, bid128_to_uint64_ceil, 0x3023C6BF526340000001C6BF52633FFFu128, 9223372036854775809  , 0x00); // -- 2^63+0.5-ulp
dec_test!(bid128_to_uint64_ceil_134, bid128_to_uint64_ceil, 0x3023C6BF526340000001C6BF52634000u128, 9223372036854775809  , 0x00); // -- 2^63+0.5
dec_test!(bid128_to_uint64_ceil_135, bid128_to_uint64_ceil, 0x3023C6BF526340000001C6BF52634001u128, 9223372036854775809  , 0x00); // -- 2^63+0.5+ulp
dec_test!(bid128_to_uint64_ceil_136, bid128_to_uint64_ceil, 0x3023C6BF5263400000038D7EA4C67FFFu128, 9223372036854775809  , 0x00); // -- 2^63+1-ulp
dec_test!(bid128_to_uint64_ceil_137, bid128_to_uint64_ceil, 0x3023C6BF5263400000038D7EA4C68000u128, 9223372036854775809  , 0x00); // -- 2^63+1
dec_test!(bid128_to_uint64_ceil_138, bid128_to_uint64_ceil, 0x3023C6BF5263400000038D7EA4C68001u128, 9223372036854775810  , 0x00); // -- 2^63+1+ulp
dec_test!(bid128_to_uint64_ceil_139, bid128_to_uint64_ceil, 0x3024000000000000006A94D74F42FFFFu128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_140, bid128_to_uint64_ceil, 0x3024000000000000006A94D74F430000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_141, bid128_to_uint64_ceil, 0x3024000000000000006A94D74F430001u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_142, bid128_to_uint64_ceil, 0x3024314DC6448D9338C15B09FFFFFFFFu128, 10000000000000000000 , 0x00); // -- 1e19-ulp
dec_test!(bid128_to_uint64_ceil_143, bid128_to_uint64_ceil, 0x3024314DC6448D9338C15B0A00000000u128, 10000000000000000000 , 0x00); // -- 1e19
dec_test!(bid128_to_uint64_ceil_144, bid128_to_uint64_ceil, 0x3024314DC6448D9338C15B0A00000001u128, 10000000000000000001 , 0x00); // -- 1e19+ulp
dec_test!(bid128_to_uint64_ceil_145, bid128_to_uint64_ceil, 0x3024314DC6448D9338C18883883D1FFFu128, 10000000000000000001 , 0x00); // -- 1e19+0.5-ulp
dec_test!(bid128_to_uint64_ceil_146, bid128_to_uint64_ceil, 0x3024314DC6448D9338C18883883D2000u128, 10000000000000000001 , 0x00); // -- 1e19+0.5
dec_test!(bid128_to_uint64_ceil_147, bid128_to_uint64_ceil, 0x3024314DC6448D9338C18883883D2001u128, 10000000000000000001 , 0x00); // -- 1e19+0.5+ulp
dec_test!(bid128_to_uint64_ceil_148, bid128_to_uint64_ceil, 0x302449F4A966D45CD522088EFFFFFFFFu128, 15000000000000000000 , 0x00); // -- 1.5e19-ulp
dec_test!(bid128_to_uint64_ceil_149, bid128_to_uint64_ceil, 0x302449F4A966D45CD522088F00000000u128, 15000000000000000000 , 0x00); // -- 1.5e19
dec_test!(bid128_to_uint64_ceil_150, bid128_to_uint64_ceil, 0x302449F4A966D45CD522088F00000001u128, 15000000000000000001 , 0x00); // -- 1.5e19+ulp
dec_test!(bid128_to_uint64_ceil_151, bid128_to_uint64_ceil, 0x30245AF3107A3FFFFFFFA50CEF85BFFFu128, 18446744073709551615 , 0x00); // -- 2^64-1-ulp
dec_test!(bid128_to_uint64_ceil_152, bid128_to_uint64_ceil, 0x30245AF3107A3FFFFFFFA50CEF85C000u128, 18446744073709551615 , 0x00); // -- 2^64-1
dec_test!(bid128_to_uint64_ceil_153, bid128_to_uint64_ceil, 0x30245AF3107A3FFFFFFFA50CEF85C001u128, 9223372036854775808  , 0x01); // -- 2^64-1+ulp
dec_test!(bid128_to_uint64_ceil_154, bid128_to_uint64_ceil, 0x30245AF3107A3FFFFFFFD28677C2DFFFu128, 9223372036854775808  , 0x01); // -- 2^64-0.5-ulp
dec_test!(bid128_to_uint64_ceil_155, bid128_to_uint64_ceil, 0x30245AF3107A3FFFFFFFD28677C2E000u128, 9223372036854775808  , 0x01); // -- 2^64-0.5
dec_test!(bid128_to_uint64_ceil_156, bid128_to_uint64_ceil, 0x30245AF3107A3FFFFFFFD28677C2E001u128, 9223372036854775808  , 0x01); // -- 2^64-0.5+ulp
dec_test!(bid128_to_uint64_ceil_157, bid128_to_uint64_ceil, 0x30245AF3107A3FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- 2^64-ulp
dec_test!(bid128_to_uint64_ceil_158, bid128_to_uint64_ceil, 0x30245AF3107A40000000000000000000u128, 9223372036854775808  , 0x01); // -- 2^64
dec_test!(bid128_to_uint64_ceil_159, bid128_to_uint64_ceil, 0x30245AF3107A40000000000000000001u128, 9223372036854775808  , 0x01); // -- 2^64+ulp
dec_test!(bid128_to_uint64_ceil_160, bid128_to_uint64_ceil, 0x30245AF3107A400000002D79883D1FFFu128, 9223372036854775808  , 0x01); // -- 2^64+0.5-ulp
dec_test!(bid128_to_uint64_ceil_161, bid128_to_uint64_ceil, 0x30245AF3107A400000002D79883D2000u128, 9223372036854775808  , 0x01); // -- 2^64+0.5
dec_test!(bid128_to_uint64_ceil_162, bid128_to_uint64_ceil, 0x30245AF3107A400000002D79883D2001u128, 9223372036854775808  , 0x01); // -- 2^64+0.5+ulp
dec_test!(bid128_to_uint64_ceil_163, bid128_to_uint64_ceil, 0x30245AF3107A400000005AF3107A3FFFu128, 9223372036854775808  , 0x01); // -- 2^64+1-ulp
dec_test!(bid128_to_uint64_ceil_164, bid128_to_uint64_ceil, 0x30245AF3107A400000005AF3107A4000u128, 9223372036854775808  , 0x01); // -- 2^64+1
dec_test!(bid128_to_uint64_ceil_165, bid128_to_uint64_ceil, 0x30245AF3107A400000005AF3107A4001u128, 9223372036854775808  , 0x01); // -- 2^64+1+ulp
dec_test!(bid128_to_uint64_ceil_166, bid128_to_uint64_ceil, 0x3024629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- 2e19-ulp
dec_test!(bid128_to_uint64_ceil_167, bid128_to_uint64_ceil, 0x3024629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- 2e19
dec_test!(bid128_to_uint64_ceil_168, bid128_to_uint64_ceil, 0x3024629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- 2e19+ulp
dec_test!(bid128_to_uint64_ceil_169, bid128_to_uint64_ceil, 0x30247B426FAB61F00DE36398FFFFFFFFu128, 9223372036854775808  , 0x01); // -- 2.5e19-ulp
dec_test!(bid128_to_uint64_ceil_170, bid128_to_uint64_ceil, 0x30247B426FAB61F00DE3639900000000u128, 9223372036854775808  , 0x01); // -- 2.5e19
dec_test!(bid128_to_uint64_ceil_171, bid128_to_uint64_ceil, 0x30247B426FAB61F00DE3639900000001u128, 9223372036854775808  , 0x01); // -- 2.5e19+ulp
dec_test!(bid128_to_uint64_ceil_172, bid128_to_uint64_ceil, 0x30251efa8718e0435dbcf90b07eccdaeu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_173, bid128_to_uint64_ceil, 0x3026314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- 1e20-ulp
dec_test!(bid128_to_uint64_ceil_174, bid128_to_uint64_ceil, 0x3026314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- 1e20
dec_test!(bid128_to_uint64_ceil_175, bid128_to_uint64_ceil, 0x3026314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- 1e20+ulp
dec_test!(bid128_to_uint64_ceil_176, bid128_to_uint64_ceil, 0x302A00000000006C6B935B68D08DA3FFu128, 19999999999          , 0x00); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint64_ceil_177, bid128_to_uint64_ceil, 0x302A00000000006C6B935B68D08DA400u128, 19999999999          , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_uint64_ceil_178, bid128_to_uint64_ceil, 0x302A00000000006C6B935B68D08DA401u128, 19999999999          , 0x00); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint64_ceil_179, bid128_to_uint64_ceil, 0x302A00000000006C6B935B8019048BFFu128, 20000000000          , 0x00); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint64_ceil_180, bid128_to_uint64_ceil, 0x302A00000000006C6B935B8019048C00u128, 20000000000          , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_uint64_ceil_181, bid128_to_uint64_ceil, 0x302A00000000006C6B935B8019048C01u128, 20000000000          , 0x00); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint64_ceil_182, bid128_to_uint64_ceil, 0x302C000000000000000002BBA7F521FFu128, 301                  , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint64_ceil_183, bid128_to_uint64_ceil, 0x302C000000000000000002BBA7F52200u128, 301                  , 0x00); // -- 300.5
dec_test!(bid128_to_uint64_ceil_184, bid128_to_uint64_ceil, 0x302C000000000000000002BBA7F52201u128, 301                  , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint64_ceil_185, bid128_to_uint64_ceil, 0x302C00000000000AD78EBC5872141BFFu128, 19999999999          , 0x00); // -- 2e10-1-ulp
dec_test!(bid128_to_uint64_ceil_186, bid128_to_uint64_ceil, 0x302C00000000000AD78EBC5872141C00u128, 19999999999          , 0x00); // -- 2e10-1
dec_test!(bid128_to_uint64_ceil_187, bid128_to_uint64_ceil, 0x302C00000000000AD78EBC5872141C01u128, 20000000000          , 0x00); // -- 2e10-1+ulp
dec_test!(bid128_to_uint64_ceil_188, bid128_to_uint64_ceil, 0x302C00000000000AD78EBC5BF025F1FFu128, 20000000001          , 0x00); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint64_ceil_189, bid128_to_uint64_ceil, 0x302C00000000000AD78EBC5BF025F200u128, 20000000001          , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_uint64_ceil_190, bid128_to_uint64_ceil, 0x302C00000000000AD78EBC5BF025F201u128, 20000000001          , 0x00); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint64_ceil_191, bid128_to_uint64_ceil, 0x302C00000000000AD78EBC5E4431D5FFu128, 20000000002          , 0x00); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint64_ceil_192, bid128_to_uint64_ceil, 0x302C00000000000AD78EBC5E4431D600u128, 20000000002          , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_uint64_ceil_193, bid128_to_uint64_ceil, 0x302C00000000000AD78EBC5E4431D601u128, 20000000002          , 0x00); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint64_ceil_194, bid128_to_uint64_ceil, 0x302C000000108B2A2C28028E3FF41BFFu128, 1999999999999999     , 0x00); // -- 2e15-1-ulp
dec_test!(bid128_to_uint64_ceil_195, bid128_to_uint64_ceil, 0x302C000000108B2A2C28028E3FF41C00u128, 1999999999999999     , 0x00); // -- 2e15-1
dec_test!(bid128_to_uint64_ceil_196, bid128_to_uint64_ceil, 0x302C000000108B2A2C28028E3FF41C01u128, 2000000000000000     , 0x00); // -- 2e15-1+ulp
dec_test!(bid128_to_uint64_ceil_197, bid128_to_uint64_ceil, 0x302E000000000001158E46094F6AC9FFu128, 20000000001          , 0x00); // -- 2e10+1-ulp
dec_test!(bid128_to_uint64_ceil_198, bid128_to_uint64_ceil, 0x302E000000000001158E46094F6ACA00u128, 20000000001          , 0x00); // -- 2e10+1
dec_test!(bid128_to_uint64_ceil_199, bid128_to_uint64_ceil, 0x302E000000000001158E46094F6ACA01u128, 20000000002          , 0x00); // -- 2e10+1+ulp
dec_test!(bid128_to_uint64_ceil_200, bid128_to_uint64_ceil, 0x302E00000001A784379D99DB7D9AC9FFu128, 2000000000000001     , 0x00); // -- 2e15+1-ulp
dec_test!(bid128_to_uint64_ceil_201, bid128_to_uint64_ceil, 0x302E00000001A784379D99DB7D9ACA00u128, 2000000000000001     , 0x00); // -- 2e15+1
dec_test!(bid128_to_uint64_ceil_202, bid128_to_uint64_ceil, 0x302E00000001A784379D99DB7D9ACA01u128, 2000000000000002     , 0x00); // -- 2e15+1+ulp
dec_test!(bid128_to_uint64_ceil_203, bid128_to_uint64_ceil, 0x303000000000000000000006FC23ABFFu128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_204, bid128_to_uint64_ceil, 0x303000000000000000000006FC23AC00u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_205, bid128_to_uint64_ceil, 0x303000000000000000000006FC23AC01u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_206, bid128_to_uint64_ceil, 0x303200000000000000000000B2D05DFFu128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_207, bid128_to_uint64_ceil, 0x303200000000000000000000B2D05E00u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_208, bid128_to_uint64_ceil, 0x303200000000000000000000B2D05E01u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_209, bid128_to_uint64_ceil, 0x303800000000000000000000002DDA47u128, 301                  , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint64_ceil_210, bid128_to_uint64_ceil, 0x303800000000000000000000002DDA48u128, 301                  , 0x00); // -- 300.5
dec_test!(bid128_to_uint64_ceil_211, bid128_to_uint64_ceil, 0x303800000000000000000000002DDA49u128, 301                  , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint64_ceil_212, bid128_to_uint64_ceil, 0x303A00000000000000000000000003E7u128, 1                    , 0x00); // -- 0.999
dec_test!(bid128_to_uint64_ceil_213, bid128_to_uint64_ceil, 0x303A00000000000000000000000495D3u128, 301                  , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint64_ceil_214, bid128_to_uint64_ceil, 0x303A00000000000000000000000495D4u128, 301                  , 0x00); // -- 300.5
dec_test!(bid128_to_uint64_ceil_215, bid128_to_uint64_ceil, 0x303A00000000000000000000000495D5u128, 301                  , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint64_ceil_216, bid128_to_uint64_ceil, 0x303C0000000000000000000000007561u128, 301                  , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_uint64_ceil_217, bid128_to_uint64_ceil, 0x303C0000000000000000000000007562u128, 301                  , 0x00); // -- 300.5
dec_test!(bid128_to_uint64_ceil_218, bid128_to_uint64_ceil, 0x303C0000000000000000000000007563u128, 301                  , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_uint64_ceil_219, bid128_to_uint64_ceil, 0x303E0000000000000000000000000005u128, 1                    , 0x00); // -- 0.5
dec_test!(bid128_to_uint64_ceil_220, bid128_to_uint64_ceil, 0x303E000000000000000000000000000Fu128, 2                    , 0x00); // -- 1.5
dec_test!(bid128_to_uint64_ceil_221, bid128_to_uint64_ceil, 0x303E0000000000000000000000000BB7u128, 300                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_222, bid128_to_uint64_ceil, 0x303E0000000000000000000000000BB8u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_223, bid128_to_uint64_ceil, 0x303E0000000000000000000000000BB9u128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_224, bid128_to_uint64_ceil, 0x303E0000000000000000000000000BBDu128, 301                  , 0x00); // -- 300.5
dec_test!(bid128_to_uint64_ceil_225, bid128_to_uint64_ceil, 0x303E0000000000000000002E90EDCFF1u128, 19999999999          , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_uint64_ceil_226, bid128_to_uint64_ceil, 0x303E0000000000000000002E90EDCFFBu128, 20000000000          , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_uint64_ceil_227, bid128_to_uint64_ceil, 0x303E0000000000000000002E90EDD005u128, 20000000001          , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_uint64_ceil_228, bid128_to_uint64_ceil, 0x303E0000000000000000002E90EDD00Fu128, 20000000002          , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_uint64_ceil_229, bid128_to_uint64_ceil, 0x303E0000000000000001400000000005u128, 35184372088833       , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_uint64_ceil_230, bid128_to_uint64_ceil, 0x303E00000000000000470DE4DF81FFF1u128, 1999999999999999     , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_uint64_ceil_231, bid128_to_uint64_ceil, 0x303E00000000000000470DE4DF81FFFBu128, 2000000000000000     , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_uint64_ceil_232, bid128_to_uint64_ceil, 0x303E00000000000000470DE4DF820005u128, 2000000000000001     , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_uint64_ceil_233, bid128_to_uint64_ceil, 0x303E00000000000000470DE4DF82000Fu128, 2000000000000002     , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_uint64_ceil_234, bid128_to_uint64_ceil, 0x303E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775807  , 0x00); // -- 2^63-1.5
dec_test!(bid128_to_uint64_ceil_235, bid128_to_uint64_ceil, 0x303E000000000004FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x00); // -- 2^63-0.5
dec_test!(bid128_to_uint64_ceil_236, bid128_to_uint64_ceil, 0x303E0000000000050000000000000005u128, 9223372036854775809  , 0x00); // -- 2^63+0.5
dec_test!(bid128_to_uint64_ceil_237, bid128_to_uint64_ceil, 0x303E0000000000056BC75E2D63100005u128, 10000000000000000001 , 0x00); // -- 1e19+0.5
dec_test!(bid128_to_uint64_ceil_238, bid128_to_uint64_ceil, 0x303E000000000009FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x01); // -- 2^64-0.5
dec_test!(bid128_to_uint64_ceil_239, bid128_to_uint64_ceil, 0x303E00000000000A0000000000000005u128, 9223372036854775808  , 0x01); // -- 2^64+0.5
dec_test!(bid128_to_uint64_ceil_240, bid128_to_uint64_ceil, 0x30400000000000000000000000000001u128, 1                    , 0x00); // -- 1
dec_test!(bid128_to_uint64_ceil_241, bid128_to_uint64_ceil, 0x3040000000000000000000000000012Bu128, 299                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_242, bid128_to_uint64_ceil, 0x3040000000000000000000000000012Cu128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_243, bid128_to_uint64_ceil, 0x3040000000000000000000000000012Du128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_244, bid128_to_uint64_ceil, 0x304000000000000000000004A817C7FFu128, 19999999999          , 0x00); // -- 2e10-1
dec_test!(bid128_to_uint64_ceil_245, bid128_to_uint64_ceil, 0x304000000000000000000004A817C801u128, 20000000001          , 0x00); // -- 2e10+1
dec_test!(bid128_to_uint64_ceil_246, bid128_to_uint64_ceil, 0x30400000000000000000200000000000u128, 35184372088832       , 0x00); // -- 2^45
dec_test!(bid128_to_uint64_ceil_247, bid128_to_uint64_ceil, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999     , 0x00); // -- 2e15-1
dec_test!(bid128_to_uint64_ceil_248, bid128_to_uint64_ceil, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999     , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_uint64_ceil_249, bid128_to_uint64_ceil, 0x304000000000000000071AFD498D0000u128, 2000000000000000     , 0x00); // -- 2e15
dec_test!(bid128_to_uint64_ceil_250, bid128_to_uint64_ceil, 0x304000000000000000071AFD498D0001u128, 2000000000000001     , 0x00); // -- 2e15+1
dec_test!(bid128_to_uint64_ceil_251, bid128_to_uint64_ceil, 0x304000000000000000071AFD498D0001u128, 2000000000000001     , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_uint64_ceil_252, bid128_to_uint64_ceil, 0x30400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775807  , 0x00); // -- 2^63-1
dec_test!(bid128_to_uint64_ceil_253, bid128_to_uint64_ceil, 0x30400000000000008000000000000000u128, 9223372036854775808  , 0x00); // -- 2^63
dec_test!(bid128_to_uint64_ceil_254, bid128_to_uint64_ceil, 0x30400000000000008000000000000001u128, 9223372036854775809  , 0x00); // -- 2^63+1
dec_test!(bid128_to_uint64_ceil_255, bid128_to_uint64_ceil, 0x3040000000000000c8081a8a00200615u128, 14413799787409180181 , 0x00);
dec_test!(bid128_to_uint64_ceil_256, bid128_to_uint64_ceil, 0x3040000000000000FFFFFFFFFFFFFFFFu128, 18446744073709551615 , 0x00); // -- 2^64-1
dec_test!(bid128_to_uint64_ceil_257, bid128_to_uint64_ceil, 0x30400000000000010000000000000000u128, 9223372036854775808  , 0x01); // -- 2^64
dec_test!(bid128_to_uint64_ceil_258, bid128_to_uint64_ceil, 0x30400000000000010000000000000001u128, 9223372036854775808  , 0x01); // -- 2^64+1
dec_test!(bid128_to_uint64_ceil_259, bid128_to_uint64_ceil, 0x3040000000000001427421855c060bb7u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_260, bid128_to_uint64_ceil, 0x3041ED09BEAD87C0378D8E6400000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_261, bid128_to_uint64_ceil, 0x3042000000000000000000000000001Du128, 290                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_ceil_262, bid128_to_uint64_ceil, 0x3042000000000000000000000000001Eu128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_263, bid128_to_uint64_ceil, 0x3042000000000000000000000000001Fu128, 310                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_ceil_264, bid128_to_uint64_ceil, 0x304200000000000000000000773593FFu128, 19999999990          , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_uint64_ceil_265, bid128_to_uint64_ceil, 0x30420000000000000000000077359400u128, 20000000000          , 0x00); // -- 2e10
dec_test!(bid128_to_uint64_ceil_266, bid128_to_uint64_ceil, 0x30420000000000000000000077359401u128, 20000000010          , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_uint64_ceil_267, bid128_to_uint64_ceil, 0x30440000000000000000000000000003u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_ceil_268, bid128_to_uint64_ceil, 0x3044000000000000079daace9339febau128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_269, bid128_to_uint64_ceil, 0x30520000000000000000000000000004u128, 4000000000           , 0x00); // -- 4e9
dec_test!(bid128_to_uint64_ceil_270, bid128_to_uint64_ceil, 0x30520000000000000000000000000005u128, 5000000000           , 0x00); // -- 5e9
dec_test!(bid128_to_uint64_ceil_271, bid128_to_uint64_ceil, 0x30540000000000000000000000000002u128, 20000000000          , 0x00); // -- 2e10
dec_test!(bid128_to_uint64_ceil_272, bid128_to_uint64_ceil, 0x305E0000000000000000000000000002u128, 2000000000000000     , 0x00); // -- 2e15
dec_test!(bid128_to_uint64_ceil_273, bid128_to_uint64_ceil, 0x3064000000000000000000000000000Fu128, 15000000000000000000 , 0x00); // -- 1.5e19
dec_test!(bid128_to_uint64_ceil_274, bid128_to_uint64_ceil, 0x30640000000000000000000000000019u128, 9223372036854775808  , 0x01); // -- 2.5e19
dec_test!(bid128_to_uint64_ceil_275, bid128_to_uint64_ceil, 0x30660000000000000000000000000001u128, 10000000000000000000 , 0x00); // -- 1e19
dec_test!(bid128_to_uint64_ceil_276, bid128_to_uint64_ceil, 0x30660000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- 2e19
dec_test!(bid128_to_uint64_ceil_277, bid128_to_uint64_ceil, 0x30680000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- 1e20
dec_test!(bid128_to_uint64_ceil_278, bid128_to_uint64_ceil, 0x3175e6cf9a5b5f66c7f922c6c8d7d2a2u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_ceil_279, bid128_to_uint64_ceil, 0x460e79e2adf9dc789890b1e0341e9144u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_ceil_280, bid128_to_uint64_ceil, 0x4bdc2e1af54b480984bc63a496961020u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_ceil_281, bid128_to_uint64_ceil, 0x4fcaa517c408305dc2a26ec011e55363u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_282, bid128_to_uint64_ceil, "5.5"                                 , 6                    , 0x00);
dec_test!(bid128_to_uint64_ceil_283, bid128_to_uint64_ceil, 0x78000000000000000000000000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_284, bid128_to_uint64_ceil, 0x79fffdafffff3f7f8044000400000004u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_285, bid128_to_uint64_ceil, 0x7c000000000000000000000000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_286, bid128_to_uint64_ceil, 0x7c003fffffffffff38c15b08ffffffffu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_287, bid128_to_uint64_ceil, 0x7c003fffffffffff38c15b0affffffffu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_288, bid128_to_uint64_ceil, 0x7df7ffeeefffbfffdfb7effff725fffdu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_289, bid128_to_uint64_ceil, 0x7e000000000000000000000000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_290, bid128_to_uint64_ceil, 0x8504912002100002dfffbffdfb9eefb7u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_291, bid128_to_uint64_ceil, "+877.5868765976E0"                   , 878                  , 0x00);
dec_test!(bid128_to_uint64_ceil_292, bid128_to_uint64_ceil, 0x8a763f5d86ef5be687600e556d7e2bc6u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_293, bid128_to_uint64_ceil, 0x9cb42ee99e0113980a02b4c241c1a472u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_294, bid128_to_uint64_ceil, 0xa7ddc3954c23d30d8fcbddd2953a1308u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_295, bid128_to_uint64_ceil, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                    , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_uint64_ceil_296, bid128_to_uint64_ceil, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0                    , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint64_ceil_297, bid128_to_uint64_ceil, 0xAFFCF684DF56C3E01BC6C73200000001u128, 0                    , 0x00); // -- -(0.5+ulp)
dec_test!(bid128_to_uint64_ceil_298, bid128_to_uint64_ceil, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 0                    , 0x00); // -- -(0.999-ulp)
dec_test!(bid128_to_uint64_ceil_299, bid128_to_uint64_ceil, 0xAFFDEC8B86EF679D76FC433D80000000u128, 0                    , 0x00); // -- -(0.999)
dec_test!(bid128_to_uint64_ceil_300, bid128_to_uint64_ceil, 0xAFFDEC8B86EF679D76FC433D80000001u128, 0                    , 0x00); // -- -(0.999+ulp)
dec_test!(bid128_to_uint64_ceil_301, bid128_to_uint64_ceil, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 0                    , 0x00); // -- -(1-ulp)
dec_test!(bid128_to_uint64_ceil_302, bid128_to_uint64_ceil, 0xAFFE314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- -(1)
dec_test!(bid128_to_uint64_ceil_303, bid128_to_uint64_ceil, 0xAFFE314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint64_ceil_304, bid128_to_uint64_ceil, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint64_ceil_305, bid128_to_uint64_ceil, 0xAFFE49F4A966D45CD522088F00000000u128, 9223372036854775808  , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint64_ceil_306, bid128_to_uint64_ceil, 0xAFFE49F4A966D45CD522088F00000001u128, 9223372036854775808  , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint64_ceil_307, bid128_to_uint64_ceil, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_308, bid128_to_uint64_ceil, 0xB00293E952CDA8B9AA44111E00000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_309, bid128_to_uint64_ceil, 0xB00293E952CDA8B9AA44111E00000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_310, bid128_to_uint64_ceil, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_ceil_311, bid128_to_uint64_ceil, 0xB00294286EACB8CB0A8CB6B140000000u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_ceil_312, bid128_to_uint64_ceil, 0xB00294286EACB8CB0A8CB6B140000001u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_ceil_313, bid128_to_uint64_ceil, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_314, bid128_to_uint64_ceil, 0xB0040ECA8847C4129106CE8300000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_315, bid128_to_uint64_ceil, 0xB0040ECA8847C4129106CE8300000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_316, bid128_to_uint64_ceil, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_317, bid128_to_uint64_ceil, 0xB00A0003C95A2F0B4856475FE0000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_318, bid128_to_uint64_ceil, 0xB00A0003C95A2F0B4856475FE0000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_319, bid128_to_uint64_ceil, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_320, bid128_to_uint64_ceil, 0xB00C000060EF6B1ABA6F072330000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_321, bid128_to_uint64_ceil, 0xB00C000060EF6B1ABA6F072330000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_322, bid128_to_uint64_ceil, 0xB010C5371912364CE3056C27FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_uint64_ceil_323, bid128_to_uint64_ceil, 0xB010C5371912364CE3056C2800000000u128, 9223372036854775808  , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint64_ceil_324, bid128_to_uint64_ceil, 0xB010C5371912364CE3056C2800000001u128, 9223372036854775808  , 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_uint64_ceil_325, bid128_to_uint64_ceil, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_uint64_ceil_326, bid128_to_uint64_ceil, 0xB010F684DF56C3E01BC6C73200000000u128, 9223372036854775808  , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint64_ceil_327, bid128_to_uint64_ceil, 0xB010F684DF56C3E01BC6C73200000001u128, 9223372036854775808  , 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_uint64_ceil_328, bid128_to_uint64_ceil, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint64_ceil_329, bid128_to_uint64_ceil, 0xB012629B8C88FB62ED56E4238E400000u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint64_ceil_330, bid128_to_uint64_ceil, 0xB012629B8C88FB62ED56E4238E400001u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint64_ceil_331, bid128_to_uint64_ceil, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint64_ceil_332, bid128_to_uint64_ceil, 0xB012629B8C8905F96EBAD4C909800000u128, 9223372036854775808  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint64_ceil_333, bid128_to_uint64_ceil, 0xB012629B8C8905F96EBAD4C909800001u128, 9223372036854775808  , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint64_ceil_334, bid128_to_uint64_ceil, 0xB012629B8C89108FF01EC56E84BFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint64_ceil_335, bid128_to_uint64_ceil, 0xB012629B8C89108FF01EC56E84C00000u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint64_ceil_336, bid128_to_uint64_ceil, 0xB012629B8C89108FF01EC56E84C00001u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint64_ceil_337, bid128_to_uint64_ceil, 0xB012629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint64_ceil_338, bid128_to_uint64_ceil, 0xB012629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint64_ceil_339, bid128_to_uint64_ceil, 0xB012629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint64_ceil_340, bid128_to_uint64_ceil, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint64_ceil_341, bid128_to_uint64_ceil, 0xB012629B8C8925BCF2E6A6B97B400000u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint64_ceil_342, bid128_to_uint64_ceil, 0xB012629B8C8925BCF2E6A6B97B400001u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint64_ceil_343, bid128_to_uint64_ceil, 0xB012629B8C893053744A975EF67FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint64_ceil_344, bid128_to_uint64_ceil, 0xB012629B8C893053744A975EF6800000u128, 9223372036854775808  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint64_ceil_345, bid128_to_uint64_ceil, 0xB012629B8C893053744A975EF6800001u128, 9223372036854775808  , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint64_ceil_346, bid128_to_uint64_ceil, 0xB012629B8C893AE9F5AE880471BFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint64_ceil_347, bid128_to_uint64_ceil, 0xB012629B8C893AE9F5AE880471C00000u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint64_ceil_348, bid128_to_uint64_ceil, 0xB012629B8C893AE9F5AE880471C00001u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint64_ceil_349, bid128_to_uint64_ceil, 0xB018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^45-ulp)
dec_test!(bid128_to_uint64_ceil_350, bid128_to_uint64_ceil, 0xB018AD78EBC5AC620000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^45)
dec_test!(bid128_to_uint64_ceil_351, bid128_to_uint64_ceil, 0xB018AD78EBC5AC620000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^45+ulp)
dec_test!(bid128_to_uint64_ceil_352, bid128_to_uint64_ceil, 0xB018AD78EBC5AC64B5E3AF16B187FFFFu128, 9223372036854775808  , 0x01); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_uint64_ceil_353, bid128_to_uint64_ceil, 0xB018AD78EBC5AC64B5E3AF16B1880000u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5)
dec_test!(bid128_to_uint64_ceil_354, bid128_to_uint64_ceil, 0xB018AD78EBC5AC64B5E3AF16B1880001u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_uint64_ceil_355, bid128_to_uint64_ceil, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_ceil_356, bid128_to_uint64_ceil, 0xB01A0000000000A2E6C09AD3E0D40000u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_ceil_357, bid128_to_uint64_ceil, 0xB01A0000000000A2E6C09AD3E0D40001u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_ceil_358, bid128_to_uint64_ceil, 0xB01C629B8C891B265CB1A40684E9FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_uint64_ceil_359, bid128_to_uint64_ceil, 0xB01C629B8C891B265CB1A40684EA0000u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5)
dec_test!(bid128_to_uint64_ceil_360, bid128_to_uint64_ceil, 0xB01C629B8C891B265CB1A40684EA0001u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_uint64_ceil_361, bid128_to_uint64_ceil, 0xB01C629B8C891B2663A1FF60589BFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1-ulp)
dec_test!(bid128_to_uint64_ceil_362, bid128_to_uint64_ceil, 0xB01C629B8C891B2663A1FF60589C0000u128, 9223372036854775808  , 0x01); // -- -(2e15-1)
dec_test!(bid128_to_uint64_ceil_363, bid128_to_uint64_ceil, 0xB01C629B8C891B2663A1FF60589C0001u128, 9223372036854775808  , 0x01); // -- -(2e15-1+ulp)
dec_test!(bid128_to_uint64_ceil_364, bid128_to_uint64_ceil, 0xB01C629B8C891B266A925ABA2C4DFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_uint64_ceil_365, bid128_to_uint64_ceil, 0xB01C629B8C891B266A925ABA2C4E0000u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5)
dec_test!(bid128_to_uint64_ceil_366, bid128_to_uint64_ceil, 0xB01C629B8C891B266A925ABA2C4E0001u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_uint64_ceil_367, bid128_to_uint64_ceil, 0xB01C629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-ulp)
dec_test!(bid128_to_uint64_ceil_368, bid128_to_uint64_ceil, 0xB01C629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- -(2e15)
dec_test!(bid128_to_uint64_ceil_369, bid128_to_uint64_ceil, 0xB01C629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- -(2e15+ulp)
dec_test!(bid128_to_uint64_ceil_370, bid128_to_uint64_ceil, 0xB01C629B8C891B267873116DD3B1FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_uint64_ceil_371, bid128_to_uint64_ceil, 0xB01C629B8C891B267873116DD3B20000u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5)
dec_test!(bid128_to_uint64_ceil_372, bid128_to_uint64_ceil, 0xB01C629B8C891B267873116DD3B20001u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_uint64_ceil_373, bid128_to_uint64_ceil, 0xB01C629B8C891B267F636CC7A763FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15+1-ulp)
dec_test!(bid128_to_uint64_ceil_374, bid128_to_uint64_ceil, 0xB01C629B8C891B267F636CC7A7640000u128, 9223372036854775808  , 0x01); // -- -(2e15+1)
dec_test!(bid128_to_uint64_ceil_375, bid128_to_uint64_ceil, 0xB01C629B8C891B267F636CC7A7640001u128, 9223372036854775808  , 0x01); // -- -(2e15+1+ulp)
dec_test!(bid128_to_uint64_ceil_376, bid128_to_uint64_ceil, 0xB01C629B8C891B268653C8217B15FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_uint64_ceil_377, bid128_to_uint64_ceil, 0xB01C629B8C891B268653C8217B160000u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5)
dec_test!(bid128_to_uint64_ceil_378, bid128_to_uint64_ceil, 0xB01C629B8C891B268653C8217B160001u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_uint64_ceil_379, bid128_to_uint64_ceil, 0xB01E000000000001A055690D9DB7FFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_380, bid128_to_uint64_ceil, 0xB01E000000000001A055690D9DB80000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_381, bid128_to_uint64_ceil, 0xB01E000000000001A055690D9DB80001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_382, bid128_to_uint64_ceil, 0xB01E002C68AF0BB140B1A2BC2EC4FFFFu128, 9223372036854775808  , 0x01); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_uint64_ceil_383, bid128_to_uint64_ceil, 0xB01E002C68AF0BB140B1A2BC2EC50000u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5)
dec_test!(bid128_to_uint64_ceil_384, bid128_to_uint64_ceil, 0xB01E002C68AF0BB140B1A2BC2EC50001u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_uint64_ceil_385, bid128_to_uint64_ceil, 0xB02000000000000029A2241AF62BFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_386, bid128_to_uint64_ceil, 0xB02000000000000029A2241AF62C0000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_387, bid128_to_uint64_ceil, 0xB02000000000000029A2241AF62C0001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_388, bid128_to_uint64_ceil, 0xB020000470DE4DF81FFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^45-ulp)
dec_test!(bid128_to_uint64_ceil_389, bid128_to_uint64_ceil, 0xB020000470DE4DF82000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^45)
dec_test!(bid128_to_uint64_ceil_390, bid128_to_uint64_ceil, 0xB020000470DE4DF82000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^45+ulp)
dec_test!(bid128_to_uint64_ceil_391, bid128_to_uint64_ceil, 0xB02000FC6F7C4045813459C637E07FFFu128, 9223372036854775808  , 0x01); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_uint64_ceil_392, bid128_to_uint64_ceil, 0xB02000FC6F7C4045813459C637E08000u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5)
dec_test!(bid128_to_uint64_ceil_393, bid128_to_uint64_ceil, 0xB02000FC6F7C4045813459C637E08001u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_uint64_ceil_394, bid128_to_uint64_ceil, 0xB02000FC6F7C40458157E0B8A7A17FFFu128, 9223372036854775808  , 0x01); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_uint64_ceil_395, bid128_to_uint64_ceil, 0xB02000FC6F7C40458157E0B8A7A18000u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5)
dec_test!(bid128_to_uint64_ceil_396, bid128_to_uint64_ceil, 0xB02000FC6F7C40458157E0B8A7A18001u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_uint64_ceil_397, bid128_to_uint64_ceil, 0xB02200193E5939A08CE4879688D63FFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_uint64_ceil_398, bid128_to_uint64_ceil, 0xB02200193E5939A08CE4879688D64000u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5)
dec_test!(bid128_to_uint64_ceil_399, bid128_to_uint64_ceil, 0xB02200193E5939A08CE4879688D64001u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_uint64_ceil_400, bid128_to_uint64_ceil, 0xB02200193E5939A08CE815152D9CBFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_uint64_ceil_401, bid128_to_uint64_ceil, 0xB02200193E5939A08CE815152D9CC000u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5)
dec_test!(bid128_to_uint64_ceil_402, bid128_to_uint64_ceil, 0xB02200193E5939A08CE815152D9CC001u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_uint64_ceil_403, bid128_to_uint64_ceil, 0xb0238236842f62920cec9a530c0bacc4u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_404, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775808  , 0x01); // -- -(2^63-1.5-ulp)
dec_test!(bid128_to_uint64_ceil_405, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775808  , 0x01); // -- -(2^63-1.5)
dec_test!(bid128_to_uint64_ceil_406, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775808  , 0x01); // -- -(2^63-1.5+ulp)
dec_test!(bid128_to_uint64_ceil_407, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775808  , 0x01); // -- -(2^63-1-ulp)
dec_test!(bid128_to_uint64_ceil_408, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFC72815B398000u128, 9223372036854775808  , 0x01); // -- -(2^63-1)
dec_test!(bid128_to_uint64_ceil_409, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFC72815B398001u128, 9223372036854775808  , 0x01); // -- -(2^63-1+ulp)
dec_test!(bid128_to_uint64_ceil_410, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775808  , 0x01); // -- -(2^63-0.5-ulp)
dec_test!(bid128_to_uint64_ceil_411, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFE3940AD9CC000u128, 9223372036854775808  , 0x01); // -- -(2^63-0.5)
dec_test!(bid128_to_uint64_ceil_412, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFE3940AD9CC001u128, 9223372036854775808  , 0x01); // -- -(2^63-0.5+ulp)
dec_test!(bid128_to_uint64_ceil_413, bid128_to_uint64_ceil, 0xB023C6BF52633FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^63-ulp)
dec_test!(bid128_to_uint64_ceil_414, bid128_to_uint64_ceil, 0xB023C6BF526340000000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^63)
dec_test!(bid128_to_uint64_ceil_415, bid128_to_uint64_ceil, 0xB023C6BF526340000000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^63+ulp)
dec_test!(bid128_to_uint64_ceil_416, bid128_to_uint64_ceil, 0xB023C6BF526340000001C6BF52633FFFu128, 9223372036854775808  , 0x01); // -- -(2^63+0.5-ulp)
dec_test!(bid128_to_uint64_ceil_417, bid128_to_uint64_ceil, 0xB023C6BF526340000001C6BF52634000u128, 9223372036854775808  , 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_uint64_ceil_418, bid128_to_uint64_ceil, 0xB023C6BF526340000001C6BF52634001u128, 9223372036854775808  , 0x01); // -- -(2^63+0.5+ulp)
dec_test!(bid128_to_uint64_ceil_419, bid128_to_uint64_ceil, 0xB023C6BF5263400000038D7EA4C67FFFu128, 9223372036854775808  , 0x01); // -- -(2^63+1-ulp)
dec_test!(bid128_to_uint64_ceil_420, bid128_to_uint64_ceil, 0xB023C6BF5263400000038D7EA4C68000u128, 9223372036854775808  , 0x01); // -- -(2^63+1)
dec_test!(bid128_to_uint64_ceil_421, bid128_to_uint64_ceil, 0xB023C6BF5263400000038D7EA4C68001u128, 9223372036854775808  , 0x01); // -- -(2^63+1+ulp)
dec_test!(bid128_to_uint64_ceil_422, bid128_to_uint64_ceil, 0xB024000000000000006A94D74F42FFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_423, bid128_to_uint64_ceil, 0xB024000000000000006A94D74F430000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_424, bid128_to_uint64_ceil, 0xB024000000000000006A94D74F430001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_425, bid128_to_uint64_ceil, 0xB024314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1e19-ulp)
dec_test!(bid128_to_uint64_ceil_426, bid128_to_uint64_ceil, 0xB024314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- -(1e19)
dec_test!(bid128_to_uint64_ceil_427, bid128_to_uint64_ceil, 0xB024314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- -(1e19+ulp)
dec_test!(bid128_to_uint64_ceil_428, bid128_to_uint64_ceil, 0xB024314DC6448D9338C18883883D1FFFu128, 9223372036854775808  , 0x01); // -- -(1e19+0.5-ulp)
dec_test!(bid128_to_uint64_ceil_429, bid128_to_uint64_ceil, 0xB024314DC6448D9338C18883883D2000u128, 9223372036854775808  , 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_uint64_ceil_430, bid128_to_uint64_ceil, 0xB024314DC6448D9338C18883883D2001u128, 9223372036854775808  , 0x01); // -- -(1e19+0.5+ulp)
dec_test!(bid128_to_uint64_ceil_431, bid128_to_uint64_ceil, 0xB02449F4A966D45CD522088EFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1.5e19-ulp)
dec_test!(bid128_to_uint64_ceil_432, bid128_to_uint64_ceil, 0xB02449F4A966D45CD522088F00000000u128, 9223372036854775808  , 0x01); // -- -(1.5e19)
dec_test!(bid128_to_uint64_ceil_433, bid128_to_uint64_ceil, 0xB02449F4A966D45CD522088F00000001u128, 9223372036854775808  , 0x01); // -- -(1.5e19+ulp)
dec_test!(bid128_to_uint64_ceil_434, bid128_to_uint64_ceil, 0xB0245AF3107A3FFFFFFFA50CEF85BFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-1-ulp)
dec_test!(bid128_to_uint64_ceil_435, bid128_to_uint64_ceil, 0xB0245AF3107A3FFFFFFFA50CEF85C000u128, 9223372036854775808  , 0x01); // -- -(2^64-1)
dec_test!(bid128_to_uint64_ceil_436, bid128_to_uint64_ceil, 0xB0245AF3107A3FFFFFFFA50CEF85C001u128, 9223372036854775808  , 0x01); // -- -(2^64-1+ulp)
dec_test!(bid128_to_uint64_ceil_437, bid128_to_uint64_ceil, 0xB0245AF3107A3FFFFFFFD28677C2DFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-0.5-ulp)
dec_test!(bid128_to_uint64_ceil_438, bid128_to_uint64_ceil, 0xB0245AF3107A3FFFFFFFD28677C2E000u128, 9223372036854775808  , 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_uint64_ceil_439, bid128_to_uint64_ceil, 0xB0245AF3107A3FFFFFFFD28677C2E001u128, 9223372036854775808  , 0x01); // -- -(2^64-0.5+ulp)
dec_test!(bid128_to_uint64_ceil_440, bid128_to_uint64_ceil, 0xB0245AF3107A3FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-ulp)
dec_test!(bid128_to_uint64_ceil_441, bid128_to_uint64_ceil, 0xB0245AF3107A40000000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^64)
dec_test!(bid128_to_uint64_ceil_442, bid128_to_uint64_ceil, 0xB0245AF3107A40000000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^64+ulp)
dec_test!(bid128_to_uint64_ceil_443, bid128_to_uint64_ceil, 0xB0245AF3107A400000002D79883D1FFFu128, 9223372036854775808  , 0x01); // -- -(2^64+0.5-ulp)
dec_test!(bid128_to_uint64_ceil_444, bid128_to_uint64_ceil, 0xB0245AF3107A400000002D79883D2000u128, 9223372036854775808  , 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_uint64_ceil_445, bid128_to_uint64_ceil, 0xB0245AF3107A400000002D79883D2001u128, 9223372036854775808  , 0x01); // -- -(2^64+0.5+ulp)
dec_test!(bid128_to_uint64_ceil_446, bid128_to_uint64_ceil, 0xB0245AF3107A400000005AF3107A3FFFu128, 9223372036854775808  , 0x01); // -- -(2^64+1-ulp)
dec_test!(bid128_to_uint64_ceil_447, bid128_to_uint64_ceil, 0xB0245AF3107A400000005AF3107A4000u128, 9223372036854775808  , 0x01); // -- -(2^64+1)
dec_test!(bid128_to_uint64_ceil_448, bid128_to_uint64_ceil, 0xB0245AF3107A400000005AF3107A4001u128, 9223372036854775808  , 0x01); // -- -(2^64+1+ulp)
dec_test!(bid128_to_uint64_ceil_449, bid128_to_uint64_ceil, 0xB024629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e19-ulp)
dec_test!(bid128_to_uint64_ceil_450, bid128_to_uint64_ceil, 0xB024629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- -(2e19)
dec_test!(bid128_to_uint64_ceil_451, bid128_to_uint64_ceil, 0xB024629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- -(2e19+ulp)
dec_test!(bid128_to_uint64_ceil_452, bid128_to_uint64_ceil, 0xB0247B426FAB61F00DE36398FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2.5e19-ulp)
dec_test!(bid128_to_uint64_ceil_453, bid128_to_uint64_ceil, 0xB0247B426FAB61F00DE3639900000000u128, 9223372036854775808  , 0x01); // -- -(2.5e19)
dec_test!(bid128_to_uint64_ceil_454, bid128_to_uint64_ceil, 0xB0247B426FAB61F00DE3639900000001u128, 9223372036854775808  , 0x01); // -- -(2.5e19+ulp)
dec_test!(bid128_to_uint64_ceil_455, bid128_to_uint64_ceil, 0xb024c185a836495796fdc57cdbb294c8u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_456, bid128_to_uint64_ceil, 0xB026314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1e20-ulp)
dec_test!(bid128_to_uint64_ceil_457, bid128_to_uint64_ceil, 0xB026314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- -(1e20)
dec_test!(bid128_to_uint64_ceil_458, bid128_to_uint64_ceil, 0xB026314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- -(1e20+ulp)
dec_test!(bid128_to_uint64_ceil_459, bid128_to_uint64_ceil, 0xB02A00000000006C6B935B68D08DA3FFu128, 9223372036854775808  , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint64_ceil_460, bid128_to_uint64_ceil, 0xB02A00000000006C6B935B68D08DA400u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint64_ceil_461, bid128_to_uint64_ceil, 0xB02A00000000006C6B935B68D08DA401u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint64_ceil_462, bid128_to_uint64_ceil, 0xB02A00000000006C6B935B8019048BFFu128, 9223372036854775808  , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint64_ceil_463, bid128_to_uint64_ceil, 0xB02A00000000006C6B935B8019048C00u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint64_ceil_464, bid128_to_uint64_ceil, 0xB02A00000000006C6B935B8019048C01u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint64_ceil_465, bid128_to_uint64_ceil, 0xB02C000000000000000002BBA7F521FFu128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_ceil_466, bid128_to_uint64_ceil, 0xB02C000000000000000002BBA7F52200u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_ceil_467, bid128_to_uint64_ceil, 0xB02C000000000000000002BBA7F52201u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_ceil_468, bid128_to_uint64_ceil, 0xB02C00000000000AD78EBC5872141BFFu128, 9223372036854775808  , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint64_ceil_469, bid128_to_uint64_ceil, 0xB02C00000000000AD78EBC5872141C00u128, 9223372036854775808  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint64_ceil_470, bid128_to_uint64_ceil, 0xB02C00000000000AD78EBC5872141C01u128, 9223372036854775808  , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint64_ceil_471, bid128_to_uint64_ceil, 0xB02C00000000000AD78EBC5BF025F1FFu128, 9223372036854775808  , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint64_ceil_472, bid128_to_uint64_ceil, 0xB02C00000000000AD78EBC5BF025F200u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint64_ceil_473, bid128_to_uint64_ceil, 0xB02C00000000000AD78EBC5BF025F201u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint64_ceil_474, bid128_to_uint64_ceil, 0xB02C00000000000AD78EBC5E4431D5FFu128, 9223372036854775808  , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint64_ceil_475, bid128_to_uint64_ceil, 0xB02C00000000000AD78EBC5E4431D600u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint64_ceil_476, bid128_to_uint64_ceil, 0xB02C00000000000AD78EBC5E4431D601u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint64_ceil_477, bid128_to_uint64_ceil, 0xB02C000000108B2A2C28028E3FF41BFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1-ulp)
dec_test!(bid128_to_uint64_ceil_478, bid128_to_uint64_ceil, 0xB02C000000108B2A2C28028E3FF41C00u128, 9223372036854775808  , 0x01); // -- -(2e15-1)
dec_test!(bid128_to_uint64_ceil_479, bid128_to_uint64_ceil, 0xB02C000000108B2A2C28028E3FF41C01u128, 9223372036854775808  , 0x01); // -- -(2e15-1+ulp)
dec_test!(bid128_to_uint64_ceil_480, bid128_to_uint64_ceil, 0xB02E000000000001158E46094F6AC9FFu128, 9223372036854775808  , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint64_ceil_481, bid128_to_uint64_ceil, 0xB02E000000000001158E46094F6ACA00u128, 9223372036854775808  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint64_ceil_482, bid128_to_uint64_ceil, 0xB02E000000000001158E46094F6ACA01u128, 9223372036854775808  , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint64_ceil_483, bid128_to_uint64_ceil, 0xB02E00000001A784379D99DB7D9AC9FFu128, 9223372036854775808  , 0x01); // -- -(2e15+1-ulp)
dec_test!(bid128_to_uint64_ceil_484, bid128_to_uint64_ceil, 0xB02E00000001A784379D99DB7D9ACA00u128, 9223372036854775808  , 0x01); // -- -(2e15+1)
dec_test!(bid128_to_uint64_ceil_485, bid128_to_uint64_ceil, 0xB02E00000001A784379D99DB7D9ACA01u128, 9223372036854775808  , 0x01); // -- -(2e15+1+ulp)
dec_test!(bid128_to_uint64_ceil_486, bid128_to_uint64_ceil, 0xB03000000000000000000006FC23ABFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_487, bid128_to_uint64_ceil, 0xB03000000000000000000006FC23AC00u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_488, bid128_to_uint64_ceil, 0xB03000000000000000000006FC23AC01u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_489, bid128_to_uint64_ceil, 0xB03200000000000000000000B2D05DFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_490, bid128_to_uint64_ceil, 0xB03200000000000000000000B2D05E00u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_491, bid128_to_uint64_ceil, 0xB03200000000000000000000B2D05E01u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_492, bid128_to_uint64_ceil, 0xB03800000000000000000000002DDA47u128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_ceil_493, bid128_to_uint64_ceil, 0xB03800000000000000000000002DDA48u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_ceil_494, bid128_to_uint64_ceil, 0xB03800000000000000000000002DDA49u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_ceil_495, bid128_to_uint64_ceil, 0xB03A00000000000000000000000003E7u128, 0                    , 0x00); // -- -(0.999)
dec_test!(bid128_to_uint64_ceil_496, bid128_to_uint64_ceil, 0xB03A00000000000000000000000495D3u128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_ceil_497, bid128_to_uint64_ceil, 0xB03A00000000000000000000000495D4u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_ceil_498, bid128_to_uint64_ceil, 0xB03A00000000000000000000000495D5u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_ceil_499, bid128_to_uint64_ceil, 0xB03C0000000000000000000000007561u128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_ceil_500, bid128_to_uint64_ceil, 0xB03C0000000000000000000000007562u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_ceil_501, bid128_to_uint64_ceil, 0xB03C0000000000000000000000007563u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_ceil_502, bid128_to_uint64_ceil, 0xB03E0000000000000000000000000005u128, 0                    , 0x00); // -- -(0.5)
dec_test!(bid128_to_uint64_ceil_503, bid128_to_uint64_ceil, 0xB03E000000000000000000000000000Fu128, 9223372036854775808  , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint64_ceil_504, bid128_to_uint64_ceil, 0xB03E0000000000000000000000000BB7u128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_505, bid128_to_uint64_ceil, 0xB03E0000000000000000000000000BB8u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_506, bid128_to_uint64_ceil, 0xB03E0000000000000000000000000BB9u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_507, bid128_to_uint64_ceil, 0xB03E0000000000000000000000000BBDu128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_ceil_508, bid128_to_uint64_ceil, 0xB03E0000000000000000002E90EDCFF1u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint64_ceil_509, bid128_to_uint64_ceil, 0xB03E0000000000000000002E90EDCFFBu128, 9223372036854775808  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint64_ceil_510, bid128_to_uint64_ceil, 0xB03E0000000000000000002E90EDD005u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint64_ceil_511, bid128_to_uint64_ceil, 0xB03E0000000000000000002E90EDD00Fu128, 9223372036854775808  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint64_ceil_512, bid128_to_uint64_ceil, 0xB03E0000000000000001400000000005u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5)
dec_test!(bid128_to_uint64_ceil_513, bid128_to_uint64_ceil, 0xB03E00000000000000470DE4DF81FFF1u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5)
dec_test!(bid128_to_uint64_ceil_514, bid128_to_uint64_ceil, 0xB03E00000000000000470DE4DF81FFFBu128, 9223372036854775808  , 0x01); // -- -(2e15-0.5)
dec_test!(bid128_to_uint64_ceil_515, bid128_to_uint64_ceil, 0xB03E00000000000000470DE4DF820005u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5)
dec_test!(bid128_to_uint64_ceil_516, bid128_to_uint64_ceil, 0xB03E00000000000000470DE4DF82000Fu128, 9223372036854775808  , 0x01); // -- -(2e15+1.5)
dec_test!(bid128_to_uint64_ceil_517, bid128_to_uint64_ceil, 0xB03E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775808  , 0x01); // -- -(2^63-1.5)
dec_test!(bid128_to_uint64_ceil_518, bid128_to_uint64_ceil, 0xB03E000000000004FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x01); // -- -(2^63-0.5)
dec_test!(bid128_to_uint64_ceil_519, bid128_to_uint64_ceil, 0xB03E0000000000050000000000000005u128, 9223372036854775808  , 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_uint64_ceil_520, bid128_to_uint64_ceil, 0xB03E0000000000056BC75E2D63100005u128, 9223372036854775808  , 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_uint64_ceil_521, bid128_to_uint64_ceil, 0xB03E000000000009FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_uint64_ceil_522, bid128_to_uint64_ceil, 0xB03E00000000000A0000000000000005u128, 9223372036854775808  , 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_uint64_ceil_523, bid128_to_uint64_ceil, 0xB0400000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- -(1)
dec_test!(bid128_to_uint64_ceil_524, bid128_to_uint64_ceil, 0xB040000000000000000000000000012Bu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_525, bid128_to_uint64_ceil, 0xB040000000000000000000000000012Cu128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_526, bid128_to_uint64_ceil, 0xB040000000000000000000000000012Du128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_527, bid128_to_uint64_ceil, 0xB04000000000000000000004A817C7FFu128, 9223372036854775808  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint64_ceil_528, bid128_to_uint64_ceil, 0xB04000000000000000000004A817C801u128, 9223372036854775808  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint64_ceil_529, bid128_to_uint64_ceil, 0xB0400000000000000000200000000000u128, 9223372036854775808  , 0x01); // -- -(2^45)
dec_test!(bid128_to_uint64_ceil_530, bid128_to_uint64_ceil, 0xB04000000000000000071AFD498CFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1)
dec_test!(bid128_to_uint64_ceil_531, bid128_to_uint64_ceil, 0xB04000000000000000071AFD498CFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-ulp)
dec_test!(bid128_to_uint64_ceil_532, bid128_to_uint64_ceil, 0xB04000000000000000071AFD498D0000u128, 9223372036854775808  , 0x01); // -- -(2e15)
dec_test!(bid128_to_uint64_ceil_533, bid128_to_uint64_ceil, 0xB04000000000000000071AFD498D0001u128, 9223372036854775808  , 0x01); // -- -(2e15+1)
dec_test!(bid128_to_uint64_ceil_534, bid128_to_uint64_ceil, 0xB04000000000000000071AFD498D0001u128, 9223372036854775808  , 0x01); // -- -(2e15+ulp)
dec_test!(bid128_to_uint64_ceil_535, bid128_to_uint64_ceil, 0xB0400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^63-1)
dec_test!(bid128_to_uint64_ceil_536, bid128_to_uint64_ceil, 0xB0400000000000008000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^63)
dec_test!(bid128_to_uint64_ceil_537, bid128_to_uint64_ceil, 0xB0400000000000008000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^63+1)
dec_test!(bid128_to_uint64_ceil_538, bid128_to_uint64_ceil, 0xB040000000000000FFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-1)
dec_test!(bid128_to_uint64_ceil_539, bid128_to_uint64_ceil, 0xB0400000000000010000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^64)
dec_test!(bid128_to_uint64_ceil_540, bid128_to_uint64_ceil, 0xB0400000000000010000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^64+1)
dec_test!(bid128_to_uint64_ceil_541, bid128_to_uint64_ceil, 0xB042000000000000000000000000001Du128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_ceil_542, bid128_to_uint64_ceil, 0xB042000000000000000000000000001Eu128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_543, bid128_to_uint64_ceil, 0xB042000000000000000000000000001Fu128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_ceil_544, bid128_to_uint64_ceil, 0xB04200000000000000000000773593FFu128, 9223372036854775808  , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint64_ceil_545, bid128_to_uint64_ceil, 0xB0420000000000000000000077359400u128, 9223372036854775808  , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint64_ceil_546, bid128_to_uint64_ceil, 0xB0420000000000000000000077359401u128, 9223372036854775808  , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint64_ceil_547, bid128_to_uint64_ceil, 0xB0440000000000000000000000000003u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_ceil_548, bid128_to_uint64_ceil, 0xB0520000000000000000000000000004u128, 9223372036854775808  , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint64_ceil_549, bid128_to_uint64_ceil, 0xB0520000000000000000000000000005u128, 9223372036854775808  , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint64_ceil_550, bid128_to_uint64_ceil, 0xB0540000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint64_ceil_551, bid128_to_uint64_ceil, 0xB05E0000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- -(2e15)
dec_test!(bid128_to_uint64_ceil_552, bid128_to_uint64_ceil, 0xB064000000000000000000000000000Fu128, 9223372036854775808  , 0x01); // -- -(1.5e19)
dec_test!(bid128_to_uint64_ceil_553, bid128_to_uint64_ceil, 0xB0640000000000000000000000000019u128, 9223372036854775808  , 0x01); // -- -(2.5e19)
dec_test!(bid128_to_uint64_ceil_554, bid128_to_uint64_ceil, 0xB0660000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- -(1e19)
dec_test!(bid128_to_uint64_ceil_555, bid128_to_uint64_ceil, 0xB0660000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- -(2e19)
dec_test!(bid128_to_uint64_ceil_556, bid128_to_uint64_ceil, 0xB0680000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- -(1e20)
dec_test!(bid128_to_uint64_ceil_557, bid128_to_uint64_ceil, 0xc1080000000000000000001000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_558, bid128_to_uint64_ceil, 0xc1af51d64433ae735e41cf3d037d765fu128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_ceil_559, bid128_to_uint64_ceil, 0xde02b4a550ecd95301fb94e82648c3e2u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_ceil_560, bid128_to_uint64_ceil, 0xdef9dfbd7f8b5cfffefdfffff777fffdu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_561, bid128_to_uint64_ceil, 0xf77061e824bd9f78ffffffffffffffffu128, 0                    , 0x00);
dec_test!(bid128_to_uint64_ceil_562, bid128_to_uint64_ceil, 0xfbffdfe7f7bdeebf00000000000c0102u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_563, bid128_to_uint64_ceil, 0xffaa4f779f773e5fa41f121c57118638u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_ceil_564, bid128_to_uint64_ceil, "-Infinity"                           , 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_ceil_565, bid128_to_uint64_ceil, "Infinity"                            , 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_ceil_566, bid128_to_uint64_ceil, "QNaN"                                , 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_ceil_567, bid128_to_uint64_ceil, "SNaN"                                , 0x8000000000000000u64, 0x01);
