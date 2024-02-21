/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_uint64_xceil_001, bid128_to_uint64_xceil, 0                                     , 0                    , 0x00);
dec_test!(bid128_to_uint64_xceil_002, bid128_to_uint64_xceil, 0x00000000000000000000000000000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xceil_003, bid128_to_uint64_xceil, 0x0000000000000000ffffffffffffffffu128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_004, bid128_to_uint64_xceil, 0x0001ed09bead87c0378d8e62ffffffffu128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_005, bid128_to_uint64_xceil, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xceil_006, bid128_to_uint64_xceil, 0x0055a08860808120dfffedff8ce69f5fu128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_007, bid128_to_uint64_xceil, "+0.0E0"                              , 0                    , 0x00);
dec_test!(bid128_to_uint64_xceil_008, bid128_to_uint64_xceil, 0x02000000000000000000000040220228u128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_009, bid128_to_uint64_xceil, 0x089499c102c4c52e546f41be0f804c47u128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_010, bid128_to_uint64_xceil, 0x0e0460b266cfb52d202b4af818fb27ffu128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_011, bid128_to_uint64_xceil, 0x0eb64bdd6acc0d045d3b78f4bddd3e2du128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_012, bid128_to_uint64_xceil, "+11.1000000E0"                       , 12                   , 0x20);
dec_test!(bid128_to_uint64_xceil_013, bid128_to_uint64_xceil, 0x142f3516b76237abcca99f0cd8412258u128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_014, bid128_to_uint64_xceil, 0x160d0691f4137c8e0c008e028f55f9b4u128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_015, bid128_to_uint64_xceil, 0x1794a1c05bd50fb0a2b6db1e45b1f4c9u128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_016, bid128_to_uint64_xceil, 0x233a3a46e7da5997af83347e3e9a1d7bu128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_017, bid128_to_uint64_xceil, 0x283388393d569c3c8c438dd65126d612u128, 1                    , 0x20);
dec_test!(bid128_to_uint64_xceil_018, bid128_to_uint64_xceil, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 1                    , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_uint64_xceil_019, bid128_to_uint64_xceil, 0x2FFCF684DF56C3E01BC6C73200000000u128, 1                    , 0x20); // -- 0.5
dec_test!(bid128_to_uint64_xceil_020, bid128_to_uint64_xceil, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1                    , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_uint64_xceil_021, bid128_to_uint64_xceil, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1                    , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_uint64_xceil_022, bid128_to_uint64_xceil, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1                    , 0x20); // -- 0.999
dec_test!(bid128_to_uint64_xceil_023, bid128_to_uint64_xceil, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1                    , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_uint64_xceil_024, bid128_to_uint64_xceil, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1                    , 0x20); // -- 1-ulp
dec_test!(bid128_to_uint64_xceil_025, bid128_to_uint64_xceil, 0x2FFE314DC6448D9338C15B0A00000000u128, 1                    , 0x00); // -- 1
dec_test!(bid128_to_uint64_xceil_026, bid128_to_uint64_xceil, 0x2FFE314DC6448D9338C15B0A00000001u128, 2                    , 0x20); // -- 1+ulp
dec_test!(bid128_to_uint64_xceil_027, bid128_to_uint64_xceil, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 2                    , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_uint64_xceil_028, bid128_to_uint64_xceil, 0x2FFE49F4A966D45CD522088F00000000u128, 2                    , 0x20); // -- 1.5
dec_test!(bid128_to_uint64_xceil_029, bid128_to_uint64_xceil, 0x2FFE49F4A966D45CD522088F00000001u128, 2                    , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_uint64_xceil_030, bid128_to_uint64_xceil, 0x3001000400040800ffef7ffffdffffffu128, 52                   , 0x20);
dec_test!(bid128_to_uint64_xceil_031, bid128_to_uint64_xceil, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_032, bid128_to_uint64_xceil, 0x300293E952CDA8B9AA44111E00000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_033, bid128_to_uint64_xceil, 0x300293E952CDA8B9AA44111E00000001u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_034, bid128_to_uint64_xceil, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 301                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xceil_035, bid128_to_uint64_xceil, 0x300294286EACB8CB0A8CB6B140000000u128, 301                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xceil_036, bid128_to_uint64_xceil, 0x300294286EACB8CB0A8CB6B140000001u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xceil_037, bid128_to_uint64_xceil, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_038, bid128_to_uint64_xceil, 0x30040ECA8847C4129106CE8300000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_039, bid128_to_uint64_xceil, 0x30040ECA8847C4129106CE8300000001u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_040, bid128_to_uint64_xceil, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_041, bid128_to_uint64_xceil, 0x300A0003C95A2F0B4856475FE0000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_042, bid128_to_uint64_xceil, 0x300A0003C95A2F0B4856475FE0000001u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_043, bid128_to_uint64_xceil, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_044, bid128_to_uint64_xceil, 0x300C000060EF6B1ABA6F072330000000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_045, bid128_to_uint64_xceil, 0x300C000060EF6B1ABA6F072330000001u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_046, bid128_to_uint64_xceil, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000           , 0x20); // -- 4e9-ulp
dec_test!(bid128_to_uint64_xceil_047, bid128_to_uint64_xceil, 0x3010C5371912364CE3056C2800000000u128, 4000000000           , 0x00); // -- 4e9
dec_test!(bid128_to_uint64_xceil_048, bid128_to_uint64_xceil, 0x3010C5371912364CE3056C2800000001u128, 4000000001           , 0x20); // -- 4e9+ulp
dec_test!(bid128_to_uint64_xceil_049, bid128_to_uint64_xceil, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 5000000000           , 0x20); // -- 5e9-ulp
dec_test!(bid128_to_uint64_xceil_050, bid128_to_uint64_xceil, 0x3010F684DF56C3E01BC6C73200000000u128, 5000000000           , 0x00); // -- 5e9
dec_test!(bid128_to_uint64_xceil_051, bid128_to_uint64_xceil, 0x3010F684DF56C3E01BC6C73200000001u128, 5000000001           , 0x20); // -- 5e9+ulp
dec_test!(bid128_to_uint64_xceil_052, bid128_to_uint64_xceil, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 19999999999          , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint64_xceil_053, bid128_to_uint64_xceil, 0x3012629B8C88FB62ED56E4238E400000u128, 19999999999          , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_uint64_xceil_054, bid128_to_uint64_xceil, 0x3012629B8C88FB62ED56E4238E400001u128, 19999999999          , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint64_xceil_055, bid128_to_uint64_xceil, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 19999999999          , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_uint64_xceil_056, bid128_to_uint64_xceil, 0x3012629B8C8905F96EBAD4C909800000u128, 19999999999          , 0x00); // -- 2e10-1
dec_test!(bid128_to_uint64_xceil_057, bid128_to_uint64_xceil, 0x3012629B8C8905F96EBAD4C909800001u128, 20000000000          , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_uint64_xceil_058, bid128_to_uint64_xceil, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 20000000000          , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint64_xceil_059, bid128_to_uint64_xceil, 0x3012629B8C89108FF01EC56E84C00000u128, 20000000000          , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_uint64_xceil_060, bid128_to_uint64_xceil, 0x3012629B8C89108FF01EC56E84C00001u128, 20000000000          , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint64_xceil_061, bid128_to_uint64_xceil, 0x3012629B8C891B267182B613FFFFFFFFu128, 20000000000          , 0x20); // -- 2e10-ulp
dec_test!(bid128_to_uint64_xceil_062, bid128_to_uint64_xceil, 0x3012629B8C891B267182B61400000000u128, 20000000000          , 0x00); // -- 2e10
dec_test!(bid128_to_uint64_xceil_063, bid128_to_uint64_xceil, 0x3012629B8C891B267182B61400000001u128, 20000000001          , 0x20); // -- 2e10+ulp
dec_test!(bid128_to_uint64_xceil_064, bid128_to_uint64_xceil, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 20000000001          , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint64_xceil_065, bid128_to_uint64_xceil, 0x3012629B8C8925BCF2E6A6B97B400000u128, 20000000001          , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_uint64_xceil_066, bid128_to_uint64_xceil, 0x3012629B8C8925BCF2E6A6B97B400001u128, 20000000001          , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint64_xceil_067, bid128_to_uint64_xceil, 0x3012629B8C893053744A975EF67FFFFFu128, 20000000001          , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_uint64_xceil_068, bid128_to_uint64_xceil, 0x3012629B8C893053744A975EF6800000u128, 20000000001          , 0x00); // -- 2e10+1
dec_test!(bid128_to_uint64_xceil_069, bid128_to_uint64_xceil, 0x3012629B8C893053744A975EF6800001u128, 20000000002          , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_uint64_xceil_070, bid128_to_uint64_xceil, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 20000000002          , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint64_xceil_071, bid128_to_uint64_xceil, 0x3012629B8C893AE9F5AE880471C00000u128, 20000000002          , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_uint64_xceil_072, bid128_to_uint64_xceil, 0x3012629B8C893AE9F5AE880471C00001u128, 20000000002          , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint64_xceil_073, bid128_to_uint64_xceil, 0x3018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 35184372088832       , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_uint64_xceil_074, bid128_to_uint64_xceil, 0x3018AD78EBC5AC620000000000000000u128, 35184372088832       , 0x00); // -- 2^45
dec_test!(bid128_to_uint64_xceil_075, bid128_to_uint64_xceil, 0x3018AD78EBC5AC620000000000000001u128, 35184372088833       , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_uint64_xceil_076, bid128_to_uint64_xceil, 0x3018AD78EBC5AC64B5E3AF16B187FFFFu128, 35184372088833       , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_uint64_xceil_077, bid128_to_uint64_xceil, 0x3018AD78EBC5AC64B5E3AF16B1880000u128, 35184372088833       , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_uint64_xceil_078, bid128_to_uint64_xceil, 0x3018AD78EBC5AC64B5E3AF16B1880001u128, 35184372088833       , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_uint64_xceil_079, bid128_to_uint64_xceil, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 301                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xceil_080, bid128_to_uint64_xceil, 0x301A0000000000A2E6C09AD3E0D40000u128, 301                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xceil_081, bid128_to_uint64_xceil, 0x301A0000000000A2E6C09AD3E0D40001u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xceil_082, bid128_to_uint64_xceil, 0x301C629B8C891B265CB1A40684E9FFFFu128, 1999999999999999     , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_uint64_xceil_083, bid128_to_uint64_xceil, 0x301C629B8C891B265CB1A40684EA0000u128, 1999999999999999     , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_uint64_xceil_084, bid128_to_uint64_xceil, 0x301C629B8C891B265CB1A40684EA0001u128, 1999999999999999     , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_uint64_xceil_085, bid128_to_uint64_xceil, 0x301C629B8C891B2663A1FF60589BFFFFu128, 1999999999999999     , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_uint64_xceil_086, bid128_to_uint64_xceil, 0x301C629B8C891B2663A1FF60589C0000u128, 1999999999999999     , 0x00); // -- 2e15-1
dec_test!(bid128_to_uint64_xceil_087, bid128_to_uint64_xceil, 0x301C629B8C891B2663A1FF60589C0001u128, 2000000000000000     , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_uint64_xceil_088, bid128_to_uint64_xceil, 0x301C629B8C891B266A925ABA2C4DFFFFu128, 2000000000000000     , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_uint64_xceil_089, bid128_to_uint64_xceil, 0x301C629B8C891B266A925ABA2C4E0000u128, 2000000000000000     , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_uint64_xceil_090, bid128_to_uint64_xceil, 0x301C629B8C891B266A925ABA2C4E0001u128, 2000000000000000     , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_uint64_xceil_091, bid128_to_uint64_xceil, 0x301C629B8C891B267182B613FFFFFFFFu128, 2000000000000000     , 0x20); // -- 2e15-ulp
dec_test!(bid128_to_uint64_xceil_092, bid128_to_uint64_xceil, 0x301C629B8C891B267182B61400000000u128, 2000000000000000     , 0x00); // -- 2e15
dec_test!(bid128_to_uint64_xceil_093, bid128_to_uint64_xceil, 0x301C629B8C891B267182B61400000001u128, 2000000000000001     , 0x20); // -- 2e15+ulp
dec_test!(bid128_to_uint64_xceil_094, bid128_to_uint64_xceil, 0x301C629B8C891B267873116DD3B1FFFFu128, 2000000000000001     , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_uint64_xceil_095, bid128_to_uint64_xceil, 0x301C629B8C891B267873116DD3B20000u128, 2000000000000001     , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_uint64_xceil_096, bid128_to_uint64_xceil, 0x301C629B8C891B267873116DD3B20001u128, 2000000000000001     , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_uint64_xceil_097, bid128_to_uint64_xceil, 0x301C629B8C891B267F636CC7A763FFFFu128, 2000000000000001     , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_uint64_xceil_098, bid128_to_uint64_xceil, 0x301C629B8C891B267F636CC7A7640000u128, 2000000000000001     , 0x00); // -- 2e15+1
dec_test!(bid128_to_uint64_xceil_099, bid128_to_uint64_xceil, 0x301C629B8C891B267F636CC7A7640001u128, 2000000000000002     , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_uint64_xceil_100, bid128_to_uint64_xceil, 0x301C629B8C891B268653C8217B15FFFFu128, 2000000000000002     , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_uint64_xceil_101, bid128_to_uint64_xceil, 0x301C629B8C891B268653C8217B160000u128, 2000000000000002     , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_uint64_xceil_102, bid128_to_uint64_xceil, 0x301C629B8C891B268653C8217B160001u128, 2000000000000002     , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_uint64_xceil_103, bid128_to_uint64_xceil, 0x301E000000000001A055690D9DB7FFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_104, bid128_to_uint64_xceil, 0x301E000000000001A055690D9DB80000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_105, bid128_to_uint64_xceil, 0x301E000000000001A055690D9DB80001u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_106, bid128_to_uint64_xceil, 0x301E002C68AF0BB140B1A2BC2EC4FFFFu128, 35184372088833       , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_uint64_xceil_107, bid128_to_uint64_xceil, 0x301E002C68AF0BB140B1A2BC2EC50000u128, 35184372088833       , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_uint64_xceil_108, bid128_to_uint64_xceil, 0x301E002C68AF0BB140B1A2BC2EC50001u128, 35184372088833       , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_uint64_xceil_109, bid128_to_uint64_xceil, 0x302000000000000029A2241AF62BFFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_110, bid128_to_uint64_xceil, 0x302000000000000029A2241AF62C0000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_111, bid128_to_uint64_xceil, 0x302000000000000029A2241AF62C0001u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_112, bid128_to_uint64_xceil, 0x3020000470DE4DF81FFFFFFFFFFFFFFFu128, 35184372088832       , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_uint64_xceil_113, bid128_to_uint64_xceil, 0x3020000470DE4DF82000000000000000u128, 35184372088832       , 0x00); // -- 2^45
dec_test!(bid128_to_uint64_xceil_114, bid128_to_uint64_xceil, 0x3020000470DE4DF82000000000000001u128, 35184372088833       , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_uint64_xceil_115, bid128_to_uint64_xceil, 0x302000FC6F7C4045813459C637E07FFFu128, 2000000000000001     , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_uint64_xceil_116, bid128_to_uint64_xceil, 0x302000FC6F7C4045813459C637E08000u128, 2000000000000001     , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_uint64_xceil_117, bid128_to_uint64_xceil, 0x302000FC6F7C4045813459C637E08001u128, 2000000000000001     , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_uint64_xceil_118, bid128_to_uint64_xceil, 0x302000FC6F7C40458157E0B8A7A17FFFu128, 2000000000000002     , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_uint64_xceil_119, bid128_to_uint64_xceil, 0x302000FC6F7C40458157E0B8A7A18000u128, 2000000000000002     , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_uint64_xceil_120, bid128_to_uint64_xceil, 0x302000FC6F7C40458157E0B8A7A18001u128, 2000000000000002     , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_uint64_xceil_121, bid128_to_uint64_xceil, 0x302200193E5939A08CE4879688D63FFFu128, 1999999999999999     , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_uint64_xceil_122, bid128_to_uint64_xceil, 0x302200193E5939A08CE4879688D64000u128, 1999999999999999     , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_uint64_xceil_123, bid128_to_uint64_xceil, 0x302200193E5939A08CE4879688D64001u128, 1999999999999999     , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_uint64_xceil_124, bid128_to_uint64_xceil, 0x302200193E5939A08CE815152D9CBFFFu128, 2000000000000000     , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_uint64_xceil_125, bid128_to_uint64_xceil, 0x302200193E5939A08CE815152D9CC000u128, 2000000000000000     , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_uint64_xceil_126, bid128_to_uint64_xceil, 0x302200193E5939A08CE815152D9CC001u128, 2000000000000000     , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_uint64_xceil_127, bid128_to_uint64_xceil, 0x302296a903040ce28711e6504d221ba4u128, 3055751933364241074  , 0x20);
dec_test!(bid128_to_uint64_xceil_128, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775807  , 0x20); // -- 2^63-1.5-ulp
dec_test!(bid128_to_uint64_xceil_129, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775807  , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_uint64_xceil_130, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775807  , 0x20); // -- 2^63-1.5+ulp
dec_test!(bid128_to_uint64_xceil_131, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775807  , 0x20); // -- 2^63-1-ulp
dec_test!(bid128_to_uint64_xceil_132, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFC72815B398000u128, 9223372036854775807  , 0x00); // -- 2^63-1
dec_test!(bid128_to_uint64_xceil_133, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFC72815B398001u128, 9223372036854775808  , 0x20); // -- 2^63-1+ulp
dec_test!(bid128_to_uint64_xceil_134, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775808  , 0x20); // -- 2^63-0.5-ulp
dec_test!(bid128_to_uint64_xceil_135, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFE3940AD9CC000u128, 9223372036854775808  , 0x20); // -- 2^63-0.5
dec_test!(bid128_to_uint64_xceil_136, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFE3940AD9CC001u128, 9223372036854775808  , 0x20); // -- 2^63-0.5+ulp
dec_test!(bid128_to_uint64_xceil_137, bid128_to_uint64_xceil, 0x3023C6BF52633FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x20); // -- 2^63-ulp
dec_test!(bid128_to_uint64_xceil_138, bid128_to_uint64_xceil, 0x3023C6BF526340000000000000000000u128, 9223372036854775808  , 0x00); // -- 2^63
dec_test!(bid128_to_uint64_xceil_139, bid128_to_uint64_xceil, 0x3023C6BF526340000000000000000001u128, 9223372036854775809  , 0x20); // -- 2^63+ulp
dec_test!(bid128_to_uint64_xceil_140, bid128_to_uint64_xceil, 0x3023C6BF526340000001C6BF52633FFFu128, 9223372036854775809  , 0x20); // -- 2^63+0.5-ulp
dec_test!(bid128_to_uint64_xceil_141, bid128_to_uint64_xceil, 0x3023C6BF526340000001C6BF52634000u128, 9223372036854775809  , 0x20); // -- 2^63+0.5
dec_test!(bid128_to_uint64_xceil_142, bid128_to_uint64_xceil, 0x3023C6BF526340000001C6BF52634001u128, 9223372036854775809  , 0x20); // -- 2^63+0.5+ulp
dec_test!(bid128_to_uint64_xceil_143, bid128_to_uint64_xceil, 0x3023C6BF5263400000038D7EA4C67FFFu128, 9223372036854775809  , 0x20); // -- 2^63+1-ulp
dec_test!(bid128_to_uint64_xceil_144, bid128_to_uint64_xceil, 0x3023C6BF5263400000038D7EA4C68000u128, 9223372036854775809  , 0x00); // -- 2^63+1
dec_test!(bid128_to_uint64_xceil_145, bid128_to_uint64_xceil, 0x3023C6BF5263400000038D7EA4C68001u128, 9223372036854775810  , 0x20); // -- 2^63+1+ulp
dec_test!(bid128_to_uint64_xceil_146, bid128_to_uint64_xceil, 0x3024000000000000006A94D74F42FFFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_147, bid128_to_uint64_xceil, 0x3024000000000000006A94D74F430000u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_148, bid128_to_uint64_xceil, 0x3024000000000000006A94D74F430001u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_149, bid128_to_uint64_xceil, 0x3024314DC6448D9338C15B09FFFFFFFFu128, 10000000000000000000 , 0x20); // -- 1e19-ulp
dec_test!(bid128_to_uint64_xceil_150, bid128_to_uint64_xceil, 0x3024314DC6448D9338C15B0A00000000u128, 10000000000000000000 , 0x00); // -- 1e19
dec_test!(bid128_to_uint64_xceil_151, bid128_to_uint64_xceil, 0x3024314DC6448D9338C15B0A00000001u128, 10000000000000000001 , 0x20); // -- 1e19+ulp
dec_test!(bid128_to_uint64_xceil_152, bid128_to_uint64_xceil, 0x3024314DC6448D9338C18883883D1FFFu128, 10000000000000000001 , 0x20); // -- 1e19+0.5-ulp
dec_test!(bid128_to_uint64_xceil_153, bid128_to_uint64_xceil, 0x3024314DC6448D9338C18883883D2000u128, 10000000000000000001 , 0x20); // -- 1e19+0.5
dec_test!(bid128_to_uint64_xceil_154, bid128_to_uint64_xceil, 0x3024314DC6448D9338C18883883D2001u128, 10000000000000000001 , 0x20); // -- 1e19+0.5+ulp
dec_test!(bid128_to_uint64_xceil_155, bid128_to_uint64_xceil, 0x302449F4A966D45CD522088EFFFFFFFFu128, 15000000000000000000 , 0x20); // -- 1.5e19-ulp
dec_test!(bid128_to_uint64_xceil_156, bid128_to_uint64_xceil, 0x302449F4A966D45CD522088F00000000u128, 15000000000000000000 , 0x00); // -- 1.5e19
dec_test!(bid128_to_uint64_xceil_157, bid128_to_uint64_xceil, 0x302449F4A966D45CD522088F00000001u128, 15000000000000000001 , 0x20); // -- 1.5e19+ulp
dec_test!(bid128_to_uint64_xceil_158, bid128_to_uint64_xceil, 0x30245AF3107A3FFFFFFFA50CEF85BFFFu128, 18446744073709551615 , 0x20); // -- 2^64-1-ulp
dec_test!(bid128_to_uint64_xceil_159, bid128_to_uint64_xceil, 0x30245AF3107A3FFFFFFFA50CEF85C000u128, 18446744073709551615 , 0x00); // -- 2^64-1
dec_test!(bid128_to_uint64_xceil_160, bid128_to_uint64_xceil, 0x30245AF3107A3FFFFFFFA50CEF85C001u128, 9223372036854775808  , 0x01); // -- 2^64-1+ulp
dec_test!(bid128_to_uint64_xceil_161, bid128_to_uint64_xceil, 0x30245AF3107A3FFFFFFFD28677C2DFFFu128, 9223372036854775808  , 0x01); // -- 2^64-0.5-ulp
dec_test!(bid128_to_uint64_xceil_162, bid128_to_uint64_xceil, 0x30245AF3107A3FFFFFFFD28677C2E000u128, 9223372036854775808  , 0x01); // -- 2^64-0.5
dec_test!(bid128_to_uint64_xceil_163, bid128_to_uint64_xceil, 0x30245AF3107A3FFFFFFFD28677C2E001u128, 9223372036854775808  , 0x01); // -- 2^64-0.5+ulp
dec_test!(bid128_to_uint64_xceil_164, bid128_to_uint64_xceil, 0x30245AF3107A3FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- 2^64-ulp
dec_test!(bid128_to_uint64_xceil_165, bid128_to_uint64_xceil, 0x30245AF3107A40000000000000000000u128, 9223372036854775808  , 0x01); // -- 2^64
dec_test!(bid128_to_uint64_xceil_166, bid128_to_uint64_xceil, 0x30245AF3107A40000000000000000001u128, 9223372036854775808  , 0x01); // -- 2^64+ulp
dec_test!(bid128_to_uint64_xceil_167, bid128_to_uint64_xceil, 0x30245AF3107A400000002D79883D1FFFu128, 9223372036854775808  , 0x01); // -- 2^64+0.5-ulp
dec_test!(bid128_to_uint64_xceil_168, bid128_to_uint64_xceil, 0x30245AF3107A400000002D79883D2000u128, 9223372036854775808  , 0x01); // -- 2^64+0.5
dec_test!(bid128_to_uint64_xceil_169, bid128_to_uint64_xceil, 0x30245AF3107A400000002D79883D2001u128, 9223372036854775808  , 0x01); // -- 2^64+0.5+ulp
dec_test!(bid128_to_uint64_xceil_170, bid128_to_uint64_xceil, 0x30245AF3107A400000005AF3107A3FFFu128, 9223372036854775808  , 0x01); // -- 2^64+1-ulp
dec_test!(bid128_to_uint64_xceil_171, bid128_to_uint64_xceil, 0x30245AF3107A400000005AF3107A4000u128, 9223372036854775808  , 0x01); // -- 2^64+1
dec_test!(bid128_to_uint64_xceil_172, bid128_to_uint64_xceil, 0x30245AF3107A400000005AF3107A4001u128, 9223372036854775808  , 0x01); // -- 2^64+1+ulp
dec_test!(bid128_to_uint64_xceil_173, bid128_to_uint64_xceil, 0x3024629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- 2e19-ulp
dec_test!(bid128_to_uint64_xceil_174, bid128_to_uint64_xceil, 0x3024629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- 2e19
dec_test!(bid128_to_uint64_xceil_175, bid128_to_uint64_xceil, 0x3024629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- 2e19+ulp
dec_test!(bid128_to_uint64_xceil_176, bid128_to_uint64_xceil, 0x30247B426FAB61F00DE36398FFFFFFFFu128, 9223372036854775808  , 0x01); // -- 2.5e19-ulp
dec_test!(bid128_to_uint64_xceil_177, bid128_to_uint64_xceil, 0x30247B426FAB61F00DE3639900000000u128, 9223372036854775808  , 0x01); // -- 2.5e19
dec_test!(bid128_to_uint64_xceil_178, bid128_to_uint64_xceil, 0x30247B426FAB61F00DE3639900000001u128, 9223372036854775808  , 0x01); // -- 2.5e19+ulp
dec_test!(bid128_to_uint64_xceil_179, bid128_to_uint64_xceil, 0x3024b0020a408691effffff3ffd77e7fu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_180, bid128_to_uint64_xceil, 0x3026314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- 1e20-ulp
dec_test!(bid128_to_uint64_xceil_181, bid128_to_uint64_xceil, 0x3026314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- 1e20
dec_test!(bid128_to_uint64_xceil_182, bid128_to_uint64_xceil, 0x3026314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- 1e20+ulp
dec_test!(bid128_to_uint64_xceil_183, bid128_to_uint64_xceil, 0x302A00000000006C6B935B68D08DA3FFu128, 19999999999          , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_uint64_xceil_184, bid128_to_uint64_xceil, 0x302A00000000006C6B935B68D08DA400u128, 19999999999          , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_uint64_xceil_185, bid128_to_uint64_xceil, 0x302A00000000006C6B935B68D08DA401u128, 19999999999          , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_uint64_xceil_186, bid128_to_uint64_xceil, 0x302A00000000006C6B935B8019048BFFu128, 20000000000          , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_uint64_xceil_187, bid128_to_uint64_xceil, 0x302A00000000006C6B935B8019048C00u128, 20000000000          , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_uint64_xceil_188, bid128_to_uint64_xceil, 0x302A00000000006C6B935B8019048C01u128, 20000000000          , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_uint64_xceil_189, bid128_to_uint64_xceil, 0x302C000000000000000002BBA7F521FFu128, 301                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xceil_190, bid128_to_uint64_xceil, 0x302C000000000000000002BBA7F52200u128, 301                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xceil_191, bid128_to_uint64_xceil, 0x302C000000000000000002BBA7F52201u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xceil_192, bid128_to_uint64_xceil, 0x302C00000000000AD78EBC5872141BFFu128, 19999999999          , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_uint64_xceil_193, bid128_to_uint64_xceil, 0x302C00000000000AD78EBC5872141C00u128, 19999999999          , 0x00); // -- 2e10-1
dec_test!(bid128_to_uint64_xceil_194, bid128_to_uint64_xceil, 0x302C00000000000AD78EBC5872141C01u128, 20000000000          , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_uint64_xceil_195, bid128_to_uint64_xceil, 0x302C00000000000AD78EBC5BF025F1FFu128, 20000000001          , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_uint64_xceil_196, bid128_to_uint64_xceil, 0x302C00000000000AD78EBC5BF025F200u128, 20000000001          , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_uint64_xceil_197, bid128_to_uint64_xceil, 0x302C00000000000AD78EBC5BF025F201u128, 20000000001          , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_uint64_xceil_198, bid128_to_uint64_xceil, 0x302C00000000000AD78EBC5E4431D5FFu128, 20000000002          , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_uint64_xceil_199, bid128_to_uint64_xceil, 0x302C00000000000AD78EBC5E4431D600u128, 20000000002          , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_uint64_xceil_200, bid128_to_uint64_xceil, 0x302C00000000000AD78EBC5E4431D601u128, 20000000002          , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_uint64_xceil_201, bid128_to_uint64_xceil, 0x302C000000108B2A2C28028E3FF41BFFu128, 1999999999999999     , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_uint64_xceil_202, bid128_to_uint64_xceil, 0x302C000000108B2A2C28028E3FF41C00u128, 1999999999999999     , 0x00); // -- 2e15-1
dec_test!(bid128_to_uint64_xceil_203, bid128_to_uint64_xceil, 0x302C000000108B2A2C28028E3FF41C01u128, 2000000000000000     , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_uint64_xceil_204, bid128_to_uint64_xceil, 0x302E000000000001158E46094F6AC9FFu128, 20000000001          , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_uint64_xceil_205, bid128_to_uint64_xceil, 0x302E000000000001158E46094F6ACA00u128, 20000000001          , 0x00); // -- 2e10+1
dec_test!(bid128_to_uint64_xceil_206, bid128_to_uint64_xceil, 0x302E000000000001158E46094F6ACA01u128, 20000000002          , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_uint64_xceil_207, bid128_to_uint64_xceil, 0x302E00000001A784379D99DB7D9AC9FFu128, 2000000000000001     , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_uint64_xceil_208, bid128_to_uint64_xceil, 0x302E00000001A784379D99DB7D9ACA00u128, 2000000000000001     , 0x00); // -- 2e15+1
dec_test!(bid128_to_uint64_xceil_209, bid128_to_uint64_xceil, 0x302E00000001A784379D99DB7D9ACA01u128, 2000000000000002     , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_uint64_xceil_210, bid128_to_uint64_xceil, 0x303000000000000000000006FC23ABFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_211, bid128_to_uint64_xceil, 0x303000000000000000000006FC23AC00u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_212, bid128_to_uint64_xceil, 0x303000000000000000000006FC23AC01u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_213, bid128_to_uint64_xceil, 0x303200000000000000000000B2D05DFFu128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_214, bid128_to_uint64_xceil, 0x303200000000000000000000B2D05E00u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_215, bid128_to_uint64_xceil, 0x303200000000000000000000B2D05E01u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_216, bid128_to_uint64_xceil, 0x303800000000000000000000002DDA47u128, 301                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xceil_217, bid128_to_uint64_xceil, 0x303800000000000000000000002DDA48u128, 301                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xceil_218, bid128_to_uint64_xceil, 0x303800000000000000000000002DDA49u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xceil_219, bid128_to_uint64_xceil, 0x303A00000000000000000000000003E7u128, 1                    , 0x20); // -- 0.999
dec_test!(bid128_to_uint64_xceil_220, bid128_to_uint64_xceil, 0x303A00000000000000000000000495D3u128, 301                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xceil_221, bid128_to_uint64_xceil, 0x303A00000000000000000000000495D4u128, 301                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xceil_222, bid128_to_uint64_xceil, 0x303A00000000000000000000000495D5u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xceil_223, bid128_to_uint64_xceil, 0x303C0000000000000000000000007561u128, 301                  , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_uint64_xceil_224, bid128_to_uint64_xceil, 0x303C0000000000000000000000007562u128, 301                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xceil_225, bid128_to_uint64_xceil, 0x303C0000000000000000000000007563u128, 301                  , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_uint64_xceil_226, bid128_to_uint64_xceil, 0x303E0000000000000000000000000005u128, 1                    , 0x20); // -- 0.5
dec_test!(bid128_to_uint64_xceil_227, bid128_to_uint64_xceil, 0x303E000000000000000000000000000Fu128, 2                    , 0x20); // -- 1.5
dec_test!(bid128_to_uint64_xceil_228, bid128_to_uint64_xceil, 0x303E0000000000000000000000000BB7u128, 300                  , 0x20); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_229, bid128_to_uint64_xceil, 0x303E0000000000000000000000000BB8u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_230, bid128_to_uint64_xceil, 0x303E0000000000000000000000000BB9u128, 301                  , 0x20); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_231, bid128_to_uint64_xceil, 0x303E0000000000000000000000000BBDu128, 301                  , 0x20); // -- 300.5
dec_test!(bid128_to_uint64_xceil_232, bid128_to_uint64_xceil, 0x303E0000000000000000002E90EDCFF1u128, 19999999999          , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_uint64_xceil_233, bid128_to_uint64_xceil, 0x303E0000000000000000002E90EDCFFBu128, 20000000000          , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_uint64_xceil_234, bid128_to_uint64_xceil, 0x303E0000000000000000002E90EDD005u128, 20000000001          , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_uint64_xceil_235, bid128_to_uint64_xceil, 0x303E0000000000000000002E90EDD00Fu128, 20000000002          , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_uint64_xceil_236, bid128_to_uint64_xceil, 0x303E0000000000000001400000000005u128, 35184372088833       , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_uint64_xceil_237, bid128_to_uint64_xceil, 0x303E00000000000000470DE4DF81FFF1u128, 1999999999999999     , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_uint64_xceil_238, bid128_to_uint64_xceil, 0x303E00000000000000470DE4DF81FFFBu128, 2000000000000000     , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_uint64_xceil_239, bid128_to_uint64_xceil, 0x303E00000000000000470DE4DF820005u128, 2000000000000001     , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_uint64_xceil_240, bid128_to_uint64_xceil, 0x303E00000000000000470DE4DF82000Fu128, 2000000000000002     , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_uint64_xceil_241, bid128_to_uint64_xceil, 0x303E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775807  , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_uint64_xceil_242, bid128_to_uint64_xceil, 0x303E000000000004FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x20); // -- 2^63-0.5
dec_test!(bid128_to_uint64_xceil_243, bid128_to_uint64_xceil, 0x303E0000000000050000000000000005u128, 9223372036854775809  , 0x20); // -- 2^63+0.5
dec_test!(bid128_to_uint64_xceil_244, bid128_to_uint64_xceil, 0x303E0000000000056BC75E2D63100005u128, 10000000000000000001 , 0x20); // -- 1e19+0.5
dec_test!(bid128_to_uint64_xceil_245, bid128_to_uint64_xceil, 0x303E000000000009FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x01); // -- 2^64-0.5
dec_test!(bid128_to_uint64_xceil_246, bid128_to_uint64_xceil, 0x303E00000000000A0000000000000005u128, 9223372036854775808  , 0x01); // -- 2^64+0.5
dec_test!(bid128_to_uint64_xceil_247, bid128_to_uint64_xceil, 0x30400000000000000000000000000001u128, 1                    , 0x00); // -- 1
dec_test!(bid128_to_uint64_xceil_248, bid128_to_uint64_xceil, 0x3040000000000000000000000000012Bu128, 299                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_249, bid128_to_uint64_xceil, 0x3040000000000000000000000000012Cu128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_250, bid128_to_uint64_xceil, 0x3040000000000000000000000000012Du128, 301                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_251, bid128_to_uint64_xceil, 0x304000000000000000000004A817C7FFu128, 19999999999          , 0x00); // -- 2e10-1
dec_test!(bid128_to_uint64_xceil_252, bid128_to_uint64_xceil, 0x304000000000000000000004A817C801u128, 20000000001          , 0x00); // -- 2e10+1
dec_test!(bid128_to_uint64_xceil_253, bid128_to_uint64_xceil, 0x30400000000000000000200000000000u128, 35184372088832       , 0x00); // -- 2^45
dec_test!(bid128_to_uint64_xceil_254, bid128_to_uint64_xceil, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999     , 0x00); // -- 2e15-1
dec_test!(bid128_to_uint64_xceil_255, bid128_to_uint64_xceil, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999     , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_uint64_xceil_256, bid128_to_uint64_xceil, 0x304000000000000000071AFD498D0000u128, 2000000000000000     , 0x00); // -- 2e15
dec_test!(bid128_to_uint64_xceil_257, bid128_to_uint64_xceil, 0x304000000000000000071AFD498D0001u128, 2000000000000001     , 0x00); // -- 2e15+1
dec_test!(bid128_to_uint64_xceil_258, bid128_to_uint64_xceil, 0x304000000000000000071AFD498D0001u128, 2000000000000001     , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_uint64_xceil_259, bid128_to_uint64_xceil, 0x30400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775807  , 0x00); // -- 2^63-1
dec_test!(bid128_to_uint64_xceil_260, bid128_to_uint64_xceil, 0x30400000000000008000000000000000u128, 9223372036854775808  , 0x00); // -- 2^63
dec_test!(bid128_to_uint64_xceil_261, bid128_to_uint64_xceil, 0x30400000000000008000000000000001u128, 9223372036854775809  , 0x00); // -- 2^63+1
dec_test!(bid128_to_uint64_xceil_262, bid128_to_uint64_xceil, 0x3040000000000000a828e71e40526000u128, 12117188914554494976 , 0x00);
dec_test!(bid128_to_uint64_xceil_263, bid128_to_uint64_xceil, 0x3040000000000000FFFFFFFFFFFFFFFFu128, 18446744073709551615 , 0x00); // -- 2^64-1
dec_test!(bid128_to_uint64_xceil_264, bid128_to_uint64_xceil, 0x30400000000000010000000000000000u128, 9223372036854775808  , 0x01); // -- 2^64
dec_test!(bid128_to_uint64_xceil_265, bid128_to_uint64_xceil, 0x30400000000000010000000000000001u128, 9223372036854775808  , 0x01); // -- 2^64+1
dec_test!(bid128_to_uint64_xceil_266, bid128_to_uint64_xceil, 0x30400000000000045b1f1bcddfcfa7bdu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_267, bid128_to_uint64_xceil, 0x3041ED09BEAD87C0378D8E6400000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xceil_268, bid128_to_uint64_xceil, 0x3042000000000000000000000000001Du128, 290                  , 0x00); // -- 300-ulp
dec_test!(bid128_to_uint64_xceil_269, bid128_to_uint64_xceil, 0x3042000000000000000000000000001Eu128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_270, bid128_to_uint64_xceil, 0x3042000000000000000000000000001Fu128, 310                  , 0x00); // -- 300+ulp
dec_test!(bid128_to_uint64_xceil_271, bid128_to_uint64_xceil, 0x304200000000000000000000773593FFu128, 19999999990          , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_uint64_xceil_272, bid128_to_uint64_xceil, 0x30420000000000000000000077359400u128, 20000000000          , 0x00); // -- 2e10
dec_test!(bid128_to_uint64_xceil_273, bid128_to_uint64_xceil, 0x30420000000000000000000077359401u128, 20000000010          , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_uint64_xceil_274, bid128_to_uint64_xceil, 0x30420000000000000002110003010840u128, 5816417014928000     , 0x00);
dec_test!(bid128_to_uint64_xceil_275, bid128_to_uint64_xceil, 0x30440000000000000000000000000003u128, 300                  , 0x00); // -- 300
dec_test!(bid128_to_uint64_xceil_276, bid128_to_uint64_xceil, 0x30520000000000000000000000000004u128, 4000000000           , 0x00); // -- 4e9
dec_test!(bid128_to_uint64_xceil_277, bid128_to_uint64_xceil, 0x30520000000000000000000000000005u128, 5000000000           , 0x00); // -- 5e9
dec_test!(bid128_to_uint64_xceil_278, bid128_to_uint64_xceil, 0x30540000000000000000000000000002u128, 20000000000          , 0x00); // -- 2e10
dec_test!(bid128_to_uint64_xceil_279, bid128_to_uint64_xceil, 0x305E0000000000000000000000000002u128, 2000000000000000     , 0x00); // -- 2e15
dec_test!(bid128_to_uint64_xceil_280, bid128_to_uint64_xceil, 0x3064000000000000000000000000000Fu128, 15000000000000000000 , 0x00); // -- 1.5e19
dec_test!(bid128_to_uint64_xceil_281, bid128_to_uint64_xceil, 0x30640000000000000000000000000019u128, 9223372036854775808  , 0x01); // -- 2.5e19
dec_test!(bid128_to_uint64_xceil_282, bid128_to_uint64_xceil, 0x30660000000000000000000000000001u128, 10000000000000000000 , 0x00); // -- 1e19
dec_test!(bid128_to_uint64_xceil_283, bid128_to_uint64_xceil, 0x30660000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- 2e19
dec_test!(bid128_to_uint64_xceil_284, bid128_to_uint64_xceil, 0x30680000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- 1e20
dec_test!(bid128_to_uint64_xceil_285, bid128_to_uint64_xceil, 0x30c0b20d1475c6012641dafc3800111au128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xceil_286, bid128_to_uint64_xceil, 0x30e7d7ba456ba07fb88c0d9c6dd25a00u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xceil_287, bid128_to_uint64_xceil, 0x34d019b16bca9412af53e8c37d049b5eu128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xceil_288, bid128_to_uint64_xceil, 0x3b1a0000000000000000000000000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xceil_289, bid128_to_uint64_xceil, 0x43802776a60f6889853d27aba79d43e0u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xceil_290, bid128_to_uint64_xceil, 0x59264c702c8dfa2a53de37658c568604u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xceil_291, bid128_to_uint64_xceil, "+66.747E0"                           , 67                   , 0x20);
dec_test!(bid128_to_uint64_xceil_292, bid128_to_uint64_xceil, 0x66daefcef29f6fb50020701100040893u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xceil_293, bid128_to_uint64_xceil, "+693468453.825E0"                    , 693468454            , 0x20);
dec_test!(bid128_to_uint64_xceil_294, bid128_to_uint64_xceil, 0x78000000000000000000000000000000u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xceil_295, bid128_to_uint64_xceil, 0x78000000000000000000000000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_296, bid128_to_uint64_xceil, 0x7acbd2a1b3f5acb11fffd0726dd68d14u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_297, bid128_to_uint64_xceil, 0x7c000000000000000000000000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_298, bid128_to_uint64_xceil, 0x7c003fffffffffff38c15b08ffffffffu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_299, bid128_to_uint64_xceil, 0x7c003fffffffffff38c15b0affffffffu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_300, bid128_to_uint64_xceil, 0x7c540e44930f5c11fffdffff7ceffefbu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_301, bid128_to_uint64_xceil, 0x7e000000000000000000000000000000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_302, bid128_to_uint64_xceil, 0x8f5ae105235255a21d8b2f93513a581au128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xceil_303, bid128_to_uint64_xceil, 0xa025ddc4e000a0e50820000000020250u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xceil_304, bid128_to_uint64_xceil, 0xa75feb5b943d6d49a4f7d60d578c4080u128, 0                    , 0x20);
dec_test!(bid128_to_uint64_xceil_305, bid128_to_uint64_xceil, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                    , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_uint64_xceil_306, bid128_to_uint64_xceil, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0                    , 0x20); // -- -(0.5)
dec_test!(bid128_to_uint64_xceil_307, bid128_to_uint64_xceil, 0xAFFCF684DF56C3E01BC6C73200000001u128, 0                    , 0x20); // -- -(0.5+ulp)
dec_test!(bid128_to_uint64_xceil_308, bid128_to_uint64_xceil, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 0                    , 0x20); // -- -(0.999-ulp)
dec_test!(bid128_to_uint64_xceil_309, bid128_to_uint64_xceil, 0xAFFDEC8B86EF679D76FC433D80000000u128, 0                    , 0x20); // -- -(0.999)
dec_test!(bid128_to_uint64_xceil_310, bid128_to_uint64_xceil, 0xAFFDEC8B86EF679D76FC433D80000001u128, 0                    , 0x20); // -- -(0.999+ulp)
dec_test!(bid128_to_uint64_xceil_311, bid128_to_uint64_xceil, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 0                    , 0x20); // -- -(1-ulp)
dec_test!(bid128_to_uint64_xceil_312, bid128_to_uint64_xceil, 0xAFFE314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- -(1)
dec_test!(bid128_to_uint64_xceil_313, bid128_to_uint64_xceil, 0xAFFE314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- -(1+ulp)
dec_test!(bid128_to_uint64_xceil_314, bid128_to_uint64_xceil, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1.5-ulp)
dec_test!(bid128_to_uint64_xceil_315, bid128_to_uint64_xceil, 0xAFFE49F4A966D45CD522088F00000000u128, 9223372036854775808  , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint64_xceil_316, bid128_to_uint64_xceil, 0xAFFE49F4A966D45CD522088F00000001u128, 9223372036854775808  , 0x01); // -- -(1.5+ulp)
dec_test!(bid128_to_uint64_xceil_317, bid128_to_uint64_xceil, 0xb00088840701200820010100000098b2u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_318, bid128_to_uint64_xceil, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_319, bid128_to_uint64_xceil, 0xB00293E952CDA8B9AA44111E00000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_320, bid128_to_uint64_xceil, 0xB00293E952CDA8B9AA44111E00000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_321, bid128_to_uint64_xceil, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xceil_322, bid128_to_uint64_xceil, 0xB00294286EACB8CB0A8CB6B140000000u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xceil_323, bid128_to_uint64_xceil, 0xB00294286EACB8CB0A8CB6B140000001u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xceil_324, bid128_to_uint64_xceil, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_325, bid128_to_uint64_xceil, 0xB0040ECA8847C4129106CE8300000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_326, bid128_to_uint64_xceil, 0xB0040ECA8847C4129106CE8300000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_327, bid128_to_uint64_xceil, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_328, bid128_to_uint64_xceil, 0xB00A0003C95A2F0B4856475FE0000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_329, bid128_to_uint64_xceil, 0xB00A0003C95A2F0B4856475FE0000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_330, bid128_to_uint64_xceil, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_331, bid128_to_uint64_xceil, 0xB00C000060EF6B1ABA6F072330000000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_332, bid128_to_uint64_xceil, 0xB00C000060EF6B1ABA6F072330000001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_333, bid128_to_uint64_xceil, 0xB010C5371912364CE3056C27FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(4e9-ulp)
dec_test!(bid128_to_uint64_xceil_334, bid128_to_uint64_xceil, 0xB010C5371912364CE3056C2800000000u128, 9223372036854775808  , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint64_xceil_335, bid128_to_uint64_xceil, 0xB010C5371912364CE3056C2800000001u128, 9223372036854775808  , 0x01); // -- -(4e9+ulp)
dec_test!(bid128_to_uint64_xceil_336, bid128_to_uint64_xceil, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(5e9-ulp)
dec_test!(bid128_to_uint64_xceil_337, bid128_to_uint64_xceil, 0xB010F684DF56C3E01BC6C73200000000u128, 9223372036854775808  , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint64_xceil_338, bid128_to_uint64_xceil, 0xB010F684DF56C3E01BC6C73200000001u128, 9223372036854775808  , 0x01); // -- -(5e9+ulp)
dec_test!(bid128_to_uint64_xceil_339, bid128_to_uint64_xceil, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint64_xceil_340, bid128_to_uint64_xceil, 0xB012629B8C88FB62ED56E4238E400000u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint64_xceil_341, bid128_to_uint64_xceil, 0xB012629B8C88FB62ED56E4238E400001u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint64_xceil_342, bid128_to_uint64_xceil, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint64_xceil_343, bid128_to_uint64_xceil, 0xB012629B8C8905F96EBAD4C909800000u128, 9223372036854775808  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint64_xceil_344, bid128_to_uint64_xceil, 0xB012629B8C8905F96EBAD4C909800001u128, 9223372036854775808  , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint64_xceil_345, bid128_to_uint64_xceil, 0xB012629B8C89108FF01EC56E84BFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint64_xceil_346, bid128_to_uint64_xceil, 0xB012629B8C89108FF01EC56E84C00000u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint64_xceil_347, bid128_to_uint64_xceil, 0xB012629B8C89108FF01EC56E84C00001u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint64_xceil_348, bid128_to_uint64_xceil, 0xB012629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint64_xceil_349, bid128_to_uint64_xceil, 0xB012629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint64_xceil_350, bid128_to_uint64_xceil, 0xB012629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint64_xceil_351, bid128_to_uint64_xceil, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint64_xceil_352, bid128_to_uint64_xceil, 0xB012629B8C8925BCF2E6A6B97B400000u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint64_xceil_353, bid128_to_uint64_xceil, 0xB012629B8C8925BCF2E6A6B97B400001u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint64_xceil_354, bid128_to_uint64_xceil, 0xB012629B8C893053744A975EF67FFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint64_xceil_355, bid128_to_uint64_xceil, 0xB012629B8C893053744A975EF6800000u128, 9223372036854775808  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint64_xceil_356, bid128_to_uint64_xceil, 0xB012629B8C893053744A975EF6800001u128, 9223372036854775808  , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint64_xceil_357, bid128_to_uint64_xceil, 0xB012629B8C893AE9F5AE880471BFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint64_xceil_358, bid128_to_uint64_xceil, 0xB012629B8C893AE9F5AE880471C00000u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint64_xceil_359, bid128_to_uint64_xceil, 0xB012629B8C893AE9F5AE880471C00001u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint64_xceil_360, bid128_to_uint64_xceil, 0xB018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^45-ulp)
dec_test!(bid128_to_uint64_xceil_361, bid128_to_uint64_xceil, 0xB018AD78EBC5AC620000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^45)
dec_test!(bid128_to_uint64_xceil_362, bid128_to_uint64_xceil, 0xB018AD78EBC5AC620000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^45+ulp)
dec_test!(bid128_to_uint64_xceil_363, bid128_to_uint64_xceil, 0xB018AD78EBC5AC64B5E3AF16B187FFFFu128, 9223372036854775808  , 0x01); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_uint64_xceil_364, bid128_to_uint64_xceil, 0xB018AD78EBC5AC64B5E3AF16B1880000u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5)
dec_test!(bid128_to_uint64_xceil_365, bid128_to_uint64_xceil, 0xB018AD78EBC5AC64B5E3AF16B1880001u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_uint64_xceil_366, bid128_to_uint64_xceil, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xceil_367, bid128_to_uint64_xceil, 0xB01A0000000000A2E6C09AD3E0D40000u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xceil_368, bid128_to_uint64_xceil, 0xB01A0000000000A2E6C09AD3E0D40001u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xceil_369, bid128_to_uint64_xceil, 0xB01C629B8C891B265CB1A40684E9FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_uint64_xceil_370, bid128_to_uint64_xceil, 0xB01C629B8C891B265CB1A40684EA0000u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5)
dec_test!(bid128_to_uint64_xceil_371, bid128_to_uint64_xceil, 0xB01C629B8C891B265CB1A40684EA0001u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_uint64_xceil_372, bid128_to_uint64_xceil, 0xB01C629B8C891B2663A1FF60589BFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1-ulp)
dec_test!(bid128_to_uint64_xceil_373, bid128_to_uint64_xceil, 0xB01C629B8C891B2663A1FF60589C0000u128, 9223372036854775808  , 0x01); // -- -(2e15-1)
dec_test!(bid128_to_uint64_xceil_374, bid128_to_uint64_xceil, 0xB01C629B8C891B2663A1FF60589C0001u128, 9223372036854775808  , 0x01); // -- -(2e15-1+ulp)
dec_test!(bid128_to_uint64_xceil_375, bid128_to_uint64_xceil, 0xB01C629B8C891B266A925ABA2C4DFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_uint64_xceil_376, bid128_to_uint64_xceil, 0xB01C629B8C891B266A925ABA2C4E0000u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5)
dec_test!(bid128_to_uint64_xceil_377, bid128_to_uint64_xceil, 0xB01C629B8C891B266A925ABA2C4E0001u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_uint64_xceil_378, bid128_to_uint64_xceil, 0xB01C629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-ulp)
dec_test!(bid128_to_uint64_xceil_379, bid128_to_uint64_xceil, 0xB01C629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- -(2e15)
dec_test!(bid128_to_uint64_xceil_380, bid128_to_uint64_xceil, 0xB01C629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- -(2e15+ulp)
dec_test!(bid128_to_uint64_xceil_381, bid128_to_uint64_xceil, 0xB01C629B8C891B267873116DD3B1FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_uint64_xceil_382, bid128_to_uint64_xceil, 0xB01C629B8C891B267873116DD3B20000u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5)
dec_test!(bid128_to_uint64_xceil_383, bid128_to_uint64_xceil, 0xB01C629B8C891B267873116DD3B20001u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_uint64_xceil_384, bid128_to_uint64_xceil, 0xB01C629B8C891B267F636CC7A763FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15+1-ulp)
dec_test!(bid128_to_uint64_xceil_385, bid128_to_uint64_xceil, 0xB01C629B8C891B267F636CC7A7640000u128, 9223372036854775808  , 0x01); // -- -(2e15+1)
dec_test!(bid128_to_uint64_xceil_386, bid128_to_uint64_xceil, 0xB01C629B8C891B267F636CC7A7640001u128, 9223372036854775808  , 0x01); // -- -(2e15+1+ulp)
dec_test!(bid128_to_uint64_xceil_387, bid128_to_uint64_xceil, 0xB01C629B8C891B268653C8217B15FFFFu128, 9223372036854775808  , 0x01); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_uint64_xceil_388, bid128_to_uint64_xceil, 0xB01C629B8C891B268653C8217B160000u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5)
dec_test!(bid128_to_uint64_xceil_389, bid128_to_uint64_xceil, 0xB01C629B8C891B268653C8217B160001u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_uint64_xceil_390, bid128_to_uint64_xceil, 0xB01E000000000001A055690D9DB7FFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_391, bid128_to_uint64_xceil, 0xB01E000000000001A055690D9DB80000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_392, bid128_to_uint64_xceil, 0xB01E000000000001A055690D9DB80001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_393, bid128_to_uint64_xceil, 0xB01E002C68AF0BB140B1A2BC2EC4FFFFu128, 9223372036854775808  , 0x01); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_uint64_xceil_394, bid128_to_uint64_xceil, 0xB01E002C68AF0BB140B1A2BC2EC50000u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5)
dec_test!(bid128_to_uint64_xceil_395, bid128_to_uint64_xceil, 0xB01E002C68AF0BB140B1A2BC2EC50001u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_uint64_xceil_396, bid128_to_uint64_xceil, 0xB02000000000000029A2241AF62BFFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_397, bid128_to_uint64_xceil, 0xB02000000000000029A2241AF62C0000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_398, bid128_to_uint64_xceil, 0xB02000000000000029A2241AF62C0001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_399, bid128_to_uint64_xceil, 0xB020000470DE4DF81FFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^45-ulp)
dec_test!(bid128_to_uint64_xceil_400, bid128_to_uint64_xceil, 0xB020000470DE4DF82000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^45)
dec_test!(bid128_to_uint64_xceil_401, bid128_to_uint64_xceil, 0xB020000470DE4DF82000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^45+ulp)
dec_test!(bid128_to_uint64_xceil_402, bid128_to_uint64_xceil, 0xB02000FC6F7C4045813459C637E07FFFu128, 9223372036854775808  , 0x01); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_uint64_xceil_403, bid128_to_uint64_xceil, 0xB02000FC6F7C4045813459C637E08000u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5)
dec_test!(bid128_to_uint64_xceil_404, bid128_to_uint64_xceil, 0xB02000FC6F7C4045813459C637E08001u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_uint64_xceil_405, bid128_to_uint64_xceil, 0xB02000FC6F7C40458157E0B8A7A17FFFu128, 9223372036854775808  , 0x01); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_uint64_xceil_406, bid128_to_uint64_xceil, 0xB02000FC6F7C40458157E0B8A7A18000u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5)
dec_test!(bid128_to_uint64_xceil_407, bid128_to_uint64_xceil, 0xB02000FC6F7C40458157E0B8A7A18001u128, 9223372036854775808  , 0x01); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_uint64_xceil_408, bid128_to_uint64_xceil, 0xB02200193E5939A08CE4879688D63FFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_uint64_xceil_409, bid128_to_uint64_xceil, 0xB02200193E5939A08CE4879688D64000u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5)
dec_test!(bid128_to_uint64_xceil_410, bid128_to_uint64_xceil, 0xB02200193E5939A08CE4879688D64001u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_uint64_xceil_411, bid128_to_uint64_xceil, 0xB02200193E5939A08CE815152D9CBFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_uint64_xceil_412, bid128_to_uint64_xceil, 0xB02200193E5939A08CE815152D9CC000u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5)
dec_test!(bid128_to_uint64_xceil_413, bid128_to_uint64_xceil, 0xB02200193E5939A08CE815152D9CC001u128, 9223372036854775808  , 0x01); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_uint64_xceil_414, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775808  , 0x01); // -- -(2^63-1.5-ulp)
dec_test!(bid128_to_uint64_xceil_415, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775808  , 0x01); // -- -(2^63-1.5)
dec_test!(bid128_to_uint64_xceil_416, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775808  , 0x01); // -- -(2^63-1.5+ulp)
dec_test!(bid128_to_uint64_xceil_417, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775808  , 0x01); // -- -(2^63-1-ulp)
dec_test!(bid128_to_uint64_xceil_418, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFC72815B398000u128, 9223372036854775808  , 0x01); // -- -(2^63-1)
dec_test!(bid128_to_uint64_xceil_419, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFC72815B398001u128, 9223372036854775808  , 0x01); // -- -(2^63-1+ulp)
dec_test!(bid128_to_uint64_xceil_420, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775808  , 0x01); // -- -(2^63-0.5-ulp)
dec_test!(bid128_to_uint64_xceil_421, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFE3940AD9CC000u128, 9223372036854775808  , 0x01); // -- -(2^63-0.5)
dec_test!(bid128_to_uint64_xceil_422, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFE3940AD9CC001u128, 9223372036854775808  , 0x01); // -- -(2^63-0.5+ulp)
dec_test!(bid128_to_uint64_xceil_423, bid128_to_uint64_xceil, 0xB023C6BF52633FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^63-ulp)
dec_test!(bid128_to_uint64_xceil_424, bid128_to_uint64_xceil, 0xB023C6BF526340000000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^63)
dec_test!(bid128_to_uint64_xceil_425, bid128_to_uint64_xceil, 0xB023C6BF526340000000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^63+ulp)
dec_test!(bid128_to_uint64_xceil_426, bid128_to_uint64_xceil, 0xB023C6BF526340000001C6BF52633FFFu128, 9223372036854775808  , 0x01); // -- -(2^63+0.5-ulp)
dec_test!(bid128_to_uint64_xceil_427, bid128_to_uint64_xceil, 0xB023C6BF526340000001C6BF52634000u128, 9223372036854775808  , 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_uint64_xceil_428, bid128_to_uint64_xceil, 0xB023C6BF526340000001C6BF52634001u128, 9223372036854775808  , 0x01); // -- -(2^63+0.5+ulp)
dec_test!(bid128_to_uint64_xceil_429, bid128_to_uint64_xceil, 0xB023C6BF5263400000038D7EA4C67FFFu128, 9223372036854775808  , 0x01); // -- -(2^63+1-ulp)
dec_test!(bid128_to_uint64_xceil_430, bid128_to_uint64_xceil, 0xB023C6BF5263400000038D7EA4C68000u128, 9223372036854775808  , 0x01); // -- -(2^63+1)
dec_test!(bid128_to_uint64_xceil_431, bid128_to_uint64_xceil, 0xB023C6BF5263400000038D7EA4C68001u128, 9223372036854775808  , 0x01); // -- -(2^63+1+ulp)
dec_test!(bid128_to_uint64_xceil_432, bid128_to_uint64_xceil, 0xB024000000000000006A94D74F42FFFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_433, bid128_to_uint64_xceil, 0xB024000000000000006A94D74F430000u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_434, bid128_to_uint64_xceil, 0xB024000000000000006A94D74F430001u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_435, bid128_to_uint64_xceil, 0xB024314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1e19-ulp)
dec_test!(bid128_to_uint64_xceil_436, bid128_to_uint64_xceil, 0xB024314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- -(1e19)
dec_test!(bid128_to_uint64_xceil_437, bid128_to_uint64_xceil, 0xB024314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- -(1e19+ulp)
dec_test!(bid128_to_uint64_xceil_438, bid128_to_uint64_xceil, 0xB024314DC6448D9338C18883883D1FFFu128, 9223372036854775808  , 0x01); // -- -(1e19+0.5-ulp)
dec_test!(bid128_to_uint64_xceil_439, bid128_to_uint64_xceil, 0xB024314DC6448D9338C18883883D2000u128, 9223372036854775808  , 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_uint64_xceil_440, bid128_to_uint64_xceil, 0xB024314DC6448D9338C18883883D2001u128, 9223372036854775808  , 0x01); // -- -(1e19+0.5+ulp)
dec_test!(bid128_to_uint64_xceil_441, bid128_to_uint64_xceil, 0xB02449F4A966D45CD522088EFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1.5e19-ulp)
dec_test!(bid128_to_uint64_xceil_442, bid128_to_uint64_xceil, 0xB02449F4A966D45CD522088F00000000u128, 9223372036854775808  , 0x01); // -- -(1.5e19)
dec_test!(bid128_to_uint64_xceil_443, bid128_to_uint64_xceil, 0xB02449F4A966D45CD522088F00000001u128, 9223372036854775808  , 0x01); // -- -(1.5e19+ulp)
dec_test!(bid128_to_uint64_xceil_444, bid128_to_uint64_xceil, 0xB0245AF3107A3FFFFFFFA50CEF85BFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-1-ulp)
dec_test!(bid128_to_uint64_xceil_445, bid128_to_uint64_xceil, 0xB0245AF3107A3FFFFFFFA50CEF85C000u128, 9223372036854775808  , 0x01); // -- -(2^64-1)
dec_test!(bid128_to_uint64_xceil_446, bid128_to_uint64_xceil, 0xB0245AF3107A3FFFFFFFA50CEF85C001u128, 9223372036854775808  , 0x01); // -- -(2^64-1+ulp)
dec_test!(bid128_to_uint64_xceil_447, bid128_to_uint64_xceil, 0xB0245AF3107A3FFFFFFFD28677C2DFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-0.5-ulp)
dec_test!(bid128_to_uint64_xceil_448, bid128_to_uint64_xceil, 0xB0245AF3107A3FFFFFFFD28677C2E000u128, 9223372036854775808  , 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_uint64_xceil_449, bid128_to_uint64_xceil, 0xB0245AF3107A3FFFFFFFD28677C2E001u128, 9223372036854775808  , 0x01); // -- -(2^64-0.5+ulp)
dec_test!(bid128_to_uint64_xceil_450, bid128_to_uint64_xceil, 0xB0245AF3107A3FFFFFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-ulp)
dec_test!(bid128_to_uint64_xceil_451, bid128_to_uint64_xceil, 0xB0245AF3107A40000000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^64)
dec_test!(bid128_to_uint64_xceil_452, bid128_to_uint64_xceil, 0xB0245AF3107A40000000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^64+ulp)
dec_test!(bid128_to_uint64_xceil_453, bid128_to_uint64_xceil, 0xB0245AF3107A400000002D79883D1FFFu128, 9223372036854775808  , 0x01); // -- -(2^64+0.5-ulp)
dec_test!(bid128_to_uint64_xceil_454, bid128_to_uint64_xceil, 0xB0245AF3107A400000002D79883D2000u128, 9223372036854775808  , 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_uint64_xceil_455, bid128_to_uint64_xceil, 0xB0245AF3107A400000002D79883D2001u128, 9223372036854775808  , 0x01); // -- -(2^64+0.5+ulp)
dec_test!(bid128_to_uint64_xceil_456, bid128_to_uint64_xceil, 0xB0245AF3107A400000005AF3107A3FFFu128, 9223372036854775808  , 0x01); // -- -(2^64+1-ulp)
dec_test!(bid128_to_uint64_xceil_457, bid128_to_uint64_xceil, 0xB0245AF3107A400000005AF3107A4000u128, 9223372036854775808  , 0x01); // -- -(2^64+1)
dec_test!(bid128_to_uint64_xceil_458, bid128_to_uint64_xceil, 0xB0245AF3107A400000005AF3107A4001u128, 9223372036854775808  , 0x01); // -- -(2^64+1+ulp)
dec_test!(bid128_to_uint64_xceil_459, bid128_to_uint64_xceil, 0xB024629B8C891B267182B613FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2e19-ulp)
dec_test!(bid128_to_uint64_xceil_460, bid128_to_uint64_xceil, 0xB024629B8C891B267182B61400000000u128, 9223372036854775808  , 0x01); // -- -(2e19)
dec_test!(bid128_to_uint64_xceil_461, bid128_to_uint64_xceil, 0xB024629B8C891B267182B61400000001u128, 9223372036854775808  , 0x01); // -- -(2e19+ulp)
dec_test!(bid128_to_uint64_xceil_462, bid128_to_uint64_xceil, 0xb0246dcc187a49e91fad2dbae07bc819u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_463, bid128_to_uint64_xceil, 0xB0247B426FAB61F00DE36398FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2.5e19-ulp)
dec_test!(bid128_to_uint64_xceil_464, bid128_to_uint64_xceil, 0xB0247B426FAB61F00DE3639900000000u128, 9223372036854775808  , 0x01); // -- -(2.5e19)
dec_test!(bid128_to_uint64_xceil_465, bid128_to_uint64_xceil, 0xB0247B426FAB61F00DE3639900000001u128, 9223372036854775808  , 0x01); // -- -(2.5e19+ulp)
dec_test!(bid128_to_uint64_xceil_466, bid128_to_uint64_xceil, 0xB026314DC6448D9338C15B09FFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(1e20-ulp)
dec_test!(bid128_to_uint64_xceil_467, bid128_to_uint64_xceil, 0xB026314DC6448D9338C15B0A00000000u128, 9223372036854775808  , 0x01); // -- -(1e20)
dec_test!(bid128_to_uint64_xceil_468, bid128_to_uint64_xceil, 0xB026314DC6448D9338C15B0A00000001u128, 9223372036854775808  , 0x01); // -- -(1e20+ulp)
dec_test!(bid128_to_uint64_xceil_469, bid128_to_uint64_xceil, 0xB02A00000000006C6B935B68D08DA3FFu128, 9223372036854775808  , 0x01); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_uint64_xceil_470, bid128_to_uint64_xceil, 0xB02A00000000006C6B935B68D08DA400u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint64_xceil_471, bid128_to_uint64_xceil, 0xB02A00000000006C6B935B68D08DA401u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_uint64_xceil_472, bid128_to_uint64_xceil, 0xB02A00000000006C6B935B8019048BFFu128, 9223372036854775808  , 0x01); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_uint64_xceil_473, bid128_to_uint64_xceil, 0xB02A00000000006C6B935B8019048C00u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint64_xceil_474, bid128_to_uint64_xceil, 0xB02A00000000006C6B935B8019048C01u128, 9223372036854775808  , 0x01); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_uint64_xceil_475, bid128_to_uint64_xceil, 0xB02C000000000000000002BBA7F521FFu128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xceil_476, bid128_to_uint64_xceil, 0xB02C000000000000000002BBA7F52200u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xceil_477, bid128_to_uint64_xceil, 0xB02C000000000000000002BBA7F52201u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xceil_478, bid128_to_uint64_xceil, 0xB02C00000000000AD78EBC5872141BFFu128, 9223372036854775808  , 0x01); // -- -(2e10-1-ulp)
dec_test!(bid128_to_uint64_xceil_479, bid128_to_uint64_xceil, 0xB02C00000000000AD78EBC5872141C00u128, 9223372036854775808  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint64_xceil_480, bid128_to_uint64_xceil, 0xB02C00000000000AD78EBC5872141C01u128, 9223372036854775808  , 0x01); // -- -(2e10-1+ulp)
dec_test!(bid128_to_uint64_xceil_481, bid128_to_uint64_xceil, 0xB02C00000000000AD78EBC5BF025F1FFu128, 9223372036854775808  , 0x01); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_uint64_xceil_482, bid128_to_uint64_xceil, 0xB02C00000000000AD78EBC5BF025F200u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint64_xceil_483, bid128_to_uint64_xceil, 0xB02C00000000000AD78EBC5BF025F201u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_uint64_xceil_484, bid128_to_uint64_xceil, 0xB02C00000000000AD78EBC5E4431D5FFu128, 9223372036854775808  , 0x01); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_uint64_xceil_485, bid128_to_uint64_xceil, 0xB02C00000000000AD78EBC5E4431D600u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint64_xceil_486, bid128_to_uint64_xceil, 0xB02C00000000000AD78EBC5E4431D601u128, 9223372036854775808  , 0x01); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_uint64_xceil_487, bid128_to_uint64_xceil, 0xB02C000000108B2A2C28028E3FF41BFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1-ulp)
dec_test!(bid128_to_uint64_xceil_488, bid128_to_uint64_xceil, 0xB02C000000108B2A2C28028E3FF41C00u128, 9223372036854775808  , 0x01); // -- -(2e15-1)
dec_test!(bid128_to_uint64_xceil_489, bid128_to_uint64_xceil, 0xB02C000000108B2A2C28028E3FF41C01u128, 9223372036854775808  , 0x01); // -- -(2e15-1+ulp)
dec_test!(bid128_to_uint64_xceil_490, bid128_to_uint64_xceil, 0xB02E000000000001158E46094F6AC9FFu128, 9223372036854775808  , 0x01); // -- -(2e10+1-ulp)
dec_test!(bid128_to_uint64_xceil_491, bid128_to_uint64_xceil, 0xB02E000000000001158E46094F6ACA00u128, 9223372036854775808  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint64_xceil_492, bid128_to_uint64_xceil, 0xB02E000000000001158E46094F6ACA01u128, 9223372036854775808  , 0x01); // -- -(2e10+1+ulp)
dec_test!(bid128_to_uint64_xceil_493, bid128_to_uint64_xceil, 0xB02E00000001A784379D99DB7D9AC9FFu128, 9223372036854775808  , 0x01); // -- -(2e15+1-ulp)
dec_test!(bid128_to_uint64_xceil_494, bid128_to_uint64_xceil, 0xB02E00000001A784379D99DB7D9ACA00u128, 9223372036854775808  , 0x01); // -- -(2e15+1)
dec_test!(bid128_to_uint64_xceil_495, bid128_to_uint64_xceil, 0xB02E00000001A784379D99DB7D9ACA01u128, 9223372036854775808  , 0x01); // -- -(2e15+1+ulp)
dec_test!(bid128_to_uint64_xceil_496, bid128_to_uint64_xceil, 0xB03000000000000000000006FC23ABFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_497, bid128_to_uint64_xceil, 0xB03000000000000000000006FC23AC00u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_498, bid128_to_uint64_xceil, 0xB03000000000000000000006FC23AC01u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_499, bid128_to_uint64_xceil, 0xB03200000000000000000000B2D05DFFu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_500, bid128_to_uint64_xceil, 0xB03200000000000000000000B2D05E00u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_501, bid128_to_uint64_xceil, 0xB03200000000000000000000B2D05E01u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_502, bid128_to_uint64_xceil, 0xB03800000000000000000000002DDA47u128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xceil_503, bid128_to_uint64_xceil, 0xB03800000000000000000000002DDA48u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xceil_504, bid128_to_uint64_xceil, 0xB03800000000000000000000002DDA49u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xceil_505, bid128_to_uint64_xceil, 0xB03A00000000000000000000000003E7u128, 0                    , 0x20); // -- -(0.999)
dec_test!(bid128_to_uint64_xceil_506, bid128_to_uint64_xceil, 0xB03A00000000000000000000000495D3u128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xceil_507, bid128_to_uint64_xceil, 0xB03A00000000000000000000000495D4u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xceil_508, bid128_to_uint64_xceil, 0xB03A00000000000000000000000495D5u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xceil_509, bid128_to_uint64_xceil, 0xB03C0000000000000000000000007561u128, 9223372036854775808  , 0x01); // -- -(300.5-ulp)
dec_test!(bid128_to_uint64_xceil_510, bid128_to_uint64_xceil, 0xB03C0000000000000000000000007562u128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xceil_511, bid128_to_uint64_xceil, 0xB03C0000000000000000000000007563u128, 9223372036854775808  , 0x01); // -- -(300.5+ulp)
dec_test!(bid128_to_uint64_xceil_512, bid128_to_uint64_xceil, 0xB03E0000000000000000000000000005u128, 0                    , 0x20); // -- -(0.5)
dec_test!(bid128_to_uint64_xceil_513, bid128_to_uint64_xceil, 0xB03E000000000000000000000000000Fu128, 9223372036854775808  , 0x01); // -- -(1.5)
dec_test!(bid128_to_uint64_xceil_514, bid128_to_uint64_xceil, 0xB03E0000000000000000000000000BB7u128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_515, bid128_to_uint64_xceil, 0xB03E0000000000000000000000000BB8u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_516, bid128_to_uint64_xceil, 0xB03E0000000000000000000000000BB9u128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_517, bid128_to_uint64_xceil, 0xB03E0000000000000000000000000BBDu128, 9223372036854775808  , 0x01); // -- -(300.5)
dec_test!(bid128_to_uint64_xceil_518, bid128_to_uint64_xceil, 0xB03E0000000000000000002E90EDCFF1u128, 9223372036854775808  , 0x01); // -- -(2e10-1.5)
dec_test!(bid128_to_uint64_xceil_519, bid128_to_uint64_xceil, 0xB03E0000000000000000002E90EDCFFBu128, 9223372036854775808  , 0x01); // -- -(2e10-0.5)
dec_test!(bid128_to_uint64_xceil_520, bid128_to_uint64_xceil, 0xB03E0000000000000000002E90EDD005u128, 9223372036854775808  , 0x01); // -- -(2e10+0.5)
dec_test!(bid128_to_uint64_xceil_521, bid128_to_uint64_xceil, 0xB03E0000000000000000002E90EDD00Fu128, 9223372036854775808  , 0x01); // -- -(2e10+1.5)
dec_test!(bid128_to_uint64_xceil_522, bid128_to_uint64_xceil, 0xB03E0000000000000001400000000005u128, 9223372036854775808  , 0x01); // -- -(2^45+0.5)
dec_test!(bid128_to_uint64_xceil_523, bid128_to_uint64_xceil, 0xB03E00000000000000470DE4DF81FFF1u128, 9223372036854775808  , 0x01); // -- -(2e15-1.5)
dec_test!(bid128_to_uint64_xceil_524, bid128_to_uint64_xceil, 0xB03E00000000000000470DE4DF81FFFBu128, 9223372036854775808  , 0x01); // -- -(2e15-0.5)
dec_test!(bid128_to_uint64_xceil_525, bid128_to_uint64_xceil, 0xB03E00000000000000470DE4DF820005u128, 9223372036854775808  , 0x01); // -- -(2e15+0.5)
dec_test!(bid128_to_uint64_xceil_526, bid128_to_uint64_xceil, 0xB03E00000000000000470DE4DF82000Fu128, 9223372036854775808  , 0x01); // -- -(2e15+1.5)
dec_test!(bid128_to_uint64_xceil_527, bid128_to_uint64_xceil, 0xB03E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775808  , 0x01); // -- -(2^63-1.5)
dec_test!(bid128_to_uint64_xceil_528, bid128_to_uint64_xceil, 0xB03E000000000004FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x01); // -- -(2^63-0.5)
dec_test!(bid128_to_uint64_xceil_529, bid128_to_uint64_xceil, 0xB03E0000000000050000000000000005u128, 9223372036854775808  , 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_uint64_xceil_530, bid128_to_uint64_xceil, 0xB03E0000000000056BC75E2D63100005u128, 9223372036854775808  , 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_uint64_xceil_531, bid128_to_uint64_xceil, 0xB03E000000000009FFFFFFFFFFFFFFFBu128, 9223372036854775808  , 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_uint64_xceil_532, bid128_to_uint64_xceil, 0xB03E00000000000A0000000000000005u128, 9223372036854775808  , 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_uint64_xceil_533, bid128_to_uint64_xceil, 0xB0400000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- -(1)
dec_test!(bid128_to_uint64_xceil_534, bid128_to_uint64_xceil, 0xB040000000000000000000000000012Bu128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_535, bid128_to_uint64_xceil, 0xB040000000000000000000000000012Cu128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_536, bid128_to_uint64_xceil, 0xB040000000000000000000000000012Du128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_537, bid128_to_uint64_xceil, 0xB04000000000000000000004A817C7FFu128, 9223372036854775808  , 0x01); // -- -(2e10-1)
dec_test!(bid128_to_uint64_xceil_538, bid128_to_uint64_xceil, 0xB04000000000000000000004A817C801u128, 9223372036854775808  , 0x01); // -- -(2e10+1)
dec_test!(bid128_to_uint64_xceil_539, bid128_to_uint64_xceil, 0xB0400000000000000000200000000000u128, 9223372036854775808  , 0x01); // -- -(2^45)
dec_test!(bid128_to_uint64_xceil_540, bid128_to_uint64_xceil, 0xB04000000000000000071AFD498CFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-1)
dec_test!(bid128_to_uint64_xceil_541, bid128_to_uint64_xceil, 0xB04000000000000000071AFD498CFFFFu128, 9223372036854775808  , 0x01); // -- -(2e15-ulp)
dec_test!(bid128_to_uint64_xceil_542, bid128_to_uint64_xceil, 0xB04000000000000000071AFD498D0000u128, 9223372036854775808  , 0x01); // -- -(2e15)
dec_test!(bid128_to_uint64_xceil_543, bid128_to_uint64_xceil, 0xB04000000000000000071AFD498D0001u128, 9223372036854775808  , 0x01); // -- -(2e15+1)
dec_test!(bid128_to_uint64_xceil_544, bid128_to_uint64_xceil, 0xB04000000000000000071AFD498D0001u128, 9223372036854775808  , 0x01); // -- -(2e15+ulp)
dec_test!(bid128_to_uint64_xceil_545, bid128_to_uint64_xceil, 0xB0400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^63-1)
dec_test!(bid128_to_uint64_xceil_546, bid128_to_uint64_xceil, 0xB0400000000000008000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^63)
dec_test!(bid128_to_uint64_xceil_547, bid128_to_uint64_xceil, 0xB0400000000000008000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^63+1)
dec_test!(bid128_to_uint64_xceil_548, bid128_to_uint64_xceil, 0xB040000000000000FFFFFFFFFFFFFFFFu128, 9223372036854775808  , 0x01); // -- -(2^64-1)
dec_test!(bid128_to_uint64_xceil_549, bid128_to_uint64_xceil, 0xB0400000000000010000000000000000u128, 9223372036854775808  , 0x01); // -- -(2^64)
dec_test!(bid128_to_uint64_xceil_550, bid128_to_uint64_xceil, 0xB0400000000000010000000000000001u128, 9223372036854775808  , 0x01); // -- -(2^64+1)
dec_test!(bid128_to_uint64_xceil_551, bid128_to_uint64_xceil, 0xB042000000000000000000000000001Du128, 9223372036854775808  , 0x01); // -- -(300-ulp)
dec_test!(bid128_to_uint64_xceil_552, bid128_to_uint64_xceil, 0xB042000000000000000000000000001Eu128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_553, bid128_to_uint64_xceil, 0xB042000000000000000000000000001Fu128, 9223372036854775808  , 0x01); // -- -(300+ulp)
dec_test!(bid128_to_uint64_xceil_554, bid128_to_uint64_xceil, 0xB04200000000000000000000773593FFu128, 9223372036854775808  , 0x01); // -- -(2e10-ulp)
dec_test!(bid128_to_uint64_xceil_555, bid128_to_uint64_xceil, 0xB0420000000000000000000077359400u128, 9223372036854775808  , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint64_xceil_556, bid128_to_uint64_xceil, 0xB0420000000000000000000077359401u128, 9223372036854775808  , 0x01); // -- -(2e10+ulp)
dec_test!(bid128_to_uint64_xceil_557, bid128_to_uint64_xceil, 0xB0440000000000000000000000000003u128, 9223372036854775808  , 0x01); // -- -(300)
dec_test!(bid128_to_uint64_xceil_558, bid128_to_uint64_xceil, 0xB0520000000000000000000000000004u128, 9223372036854775808  , 0x01); // -- -(4e9)
dec_test!(bid128_to_uint64_xceil_559, bid128_to_uint64_xceil, 0xB0520000000000000000000000000005u128, 9223372036854775808  , 0x01); // -- -(5e9)
dec_test!(bid128_to_uint64_xceil_560, bid128_to_uint64_xceil, 0xB0540000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- -(2e10)
dec_test!(bid128_to_uint64_xceil_561, bid128_to_uint64_xceil, 0xB05E0000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- -(2e15)
dec_test!(bid128_to_uint64_xceil_562, bid128_to_uint64_xceil, 0xB064000000000000000000000000000Fu128, 9223372036854775808  , 0x01); // -- -(1.5e19)
dec_test!(bid128_to_uint64_xceil_563, bid128_to_uint64_xceil, 0xB0640000000000000000000000000019u128, 9223372036854775808  , 0x01); // -- -(2.5e19)
dec_test!(bid128_to_uint64_xceil_564, bid128_to_uint64_xceil, 0xB0660000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- -(1e19)
dec_test!(bid128_to_uint64_xceil_565, bid128_to_uint64_xceil, 0xB0660000000000000000000000000002u128, 9223372036854775808  , 0x01); // -- -(2e19)
dec_test!(bid128_to_uint64_xceil_566, bid128_to_uint64_xceil, 0xB0680000000000000000000000000001u128, 9223372036854775808  , 0x01); // -- -(1e20)
dec_test!(bid128_to_uint64_xceil_567, bid128_to_uint64_xceil, 0xb47e4d159d7770e934429211edfa338bu128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_568, bid128_to_uint64_xceil, 0xbdae728ede1c9207459dfefbe69b9b06u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xceil_569, bid128_to_uint64_xceil, 0xc1bfc444135ab0c42187e8ef63ab9048u128, 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xceil_570, bid128_to_uint64_xceil, 0xc9e80000000000000000000000000000u128, 0                    , 0x00);
dec_test!(bid128_to_uint64_xceil_571, bid128_to_uint64_xceil, 0xfa4b7fbfcffbb7d940720602886c0130u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_572, bid128_to_uint64_xceil, 0xffe2ffefdfeffb9f809c090020081000u128, 9223372036854775808  , 0x01);
dec_test!(bid128_to_uint64_xceil_573, bid128_to_uint64_xceil, "-Infinity"                           , 0x8000000000000000u64, 0x01);
dec_test!(bid128_to_uint64_xceil_574, bid128_to_uint64_xceil, "SNaN"                                , 0x8000000000000000u64, 0x01);
