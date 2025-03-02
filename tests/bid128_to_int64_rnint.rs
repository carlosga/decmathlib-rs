/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int64_rnint_001, bid128_to_int64_rnint, "-0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_002, bid128_to_int64_rnint,  "0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_003, bid128_to_int64_rnint, 0x00000000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_004, bid128_to_int64_rnint, 0x0001ed09bead87c0378d8e62ffffffffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_005, bid128_to_int64_rnint, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_006, bid128_to_int64_rnint, 0x04000000000000003afbdf757637fd75u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_007, bid128_to_int64_rnint, "0.5"                                 , 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_008, bid128_to_int64_rnint, 0x0671104ab504d12688fca30a35062742u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_009, bid128_to_int64_rnint, 0x08000000000000000000000010001800u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_010, bid128_to_int64_rnint, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_int64_rnint_011, bid128_to_int64_rnint, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0                   , 0x00); // -- 0.5
dec_test!(bid128_to_int64_rnint_012, bid128_to_int64_rnint, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1                   , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_int64_rnint_013, bid128_to_int64_rnint, 0x2ffcfdfa6894ddef4ac820bf6e100b68u128, 1                   , 0x00);
dec_test!(bid128_to_int64_rnint_014, bid128_to_int64_rnint, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1                   , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_int64_rnint_015, bid128_to_int64_rnint, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1                   , 0x00); // -- 0.999
dec_test!(bid128_to_int64_rnint_016, bid128_to_int64_rnint, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1                   , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_int64_rnint_017, bid128_to_int64_rnint, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1                   , 0x00); // -- 1-ulp
dec_test!(bid128_to_int64_rnint_018, bid128_to_int64_rnint, 0x2FFE314DC6448D9338C15B0A00000000u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_rnint_019, bid128_to_int64_rnint, 0x2FFE314DC6448D9338C15B0A00000001u128, 1                   , 0x00); // -- 1+ulp
dec_test!(bid128_to_int64_rnint_020, bid128_to_int64_rnint, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1                   , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_int64_rnint_021, bid128_to_int64_rnint, 0x2FFE49F4A966D45CD522088F00000000u128, 2                   , 0x00); // -- 1.5
dec_test!(bid128_to_int64_rnint_022, bid128_to_int64_rnint, 0x2FFE49F4A966D45CD522088F00000001u128, 2                   , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_int64_rnint_023, bid128_to_int64_rnint, 0x30000090000002400020010000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_024, bid128_to_int64_rnint, 0x3000200000800000fcf79dfdedffb9fdu128, 6                   , 0x00);
dec_test!(bid128_to_int64_rnint_025, bid128_to_int64_rnint, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_026, bid128_to_int64_rnint, 0x300293E952CDA8B9AA44111E00000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_027, bid128_to_int64_rnint, 0x300293E952CDA8B9AA44111E00000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_028, bid128_to_int64_rnint, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rnint_029, bid128_to_int64_rnint, 0x300294286EACB8CB0A8CB6B140000000u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rnint_030, bid128_to_int64_rnint, 0x300294286EACB8CB0A8CB6B140000001u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rnint_031, bid128_to_int64_rnint, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_032, bid128_to_int64_rnint, 0x30040ECA8847C4129106CE8300000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_033, bid128_to_int64_rnint, 0x30040ECA8847C4129106CE8300000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_034, bid128_to_int64_rnint, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_035, bid128_to_int64_rnint, 0x300A0003C95A2F0B4856475FE0000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_036, bid128_to_int64_rnint, 0x300A0003C95A2F0B4856475FE0000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_037, bid128_to_int64_rnint, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_038, bid128_to_int64_rnint, 0x300C000060EF6B1ABA6F072330000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_039, bid128_to_int64_rnint, 0x300C000060EF6B1ABA6F072330000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_040, bid128_to_int64_rnint, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000          , 0x00); // -- 4e9-ulp
dec_test!(bid128_to_int64_rnint_041, bid128_to_int64_rnint, 0x3010C5371912364CE3056C2800000000u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_rnint_042, bid128_to_int64_rnint, 0x3010C5371912364CE3056C2800000001u128, 4000000000          , 0x00); // -- 4e9+ulp
dec_test!(bid128_to_int64_rnint_043, bid128_to_int64_rnint, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 5000000000          , 0x00); // -- 5e9-ulp
dec_test!(bid128_to_int64_rnint_044, bid128_to_int64_rnint, 0x3010F684DF56C3E01BC6C73200000000u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_rnint_045, bid128_to_int64_rnint, 0x3010F684DF56C3E01BC6C73200000001u128, 5000000000          , 0x00); // -- 5e9+ulp
dec_test!(bid128_to_int64_rnint_046, bid128_to_int64_rnint, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 19999999998         , 0x00); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_rnint_047, bid128_to_int64_rnint, 0x3012629B8C88FB62ED56E4238E400000u128, 19999999998         , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_int64_rnint_048, bid128_to_int64_rnint, 0x3012629B8C88FB62ED56E4238E400001u128, 19999999999         , 0x00); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_rnint_049, bid128_to_int64_rnint, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 19999999999         , 0x00); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_rnint_050, bid128_to_int64_rnint, 0x3012629B8C8905F96EBAD4C909800000u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_rnint_051, bid128_to_int64_rnint, 0x3012629B8C8905F96EBAD4C909800001u128, 19999999999         , 0x00); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_rnint_052, bid128_to_int64_rnint, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 19999999999         , 0x00); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_rnint_053, bid128_to_int64_rnint, 0x3012629B8C89108FF01EC56E84C00000u128, 20000000000         , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_int64_rnint_054, bid128_to_int64_rnint, 0x3012629B8C89108FF01EC56E84C00001u128, 20000000000         , 0x00); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_rnint_055, bid128_to_int64_rnint, 0x3012629B8C891B267182B613FFFFFFFFu128, 20000000000         , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_int64_rnint_056, bid128_to_int64_rnint, 0x3012629B8C891B267182B61400000000u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_rnint_057, bid128_to_int64_rnint, 0x3012629B8C891B267182B61400000001u128, 20000000000         , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_int64_rnint_058, bid128_to_int64_rnint, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 20000000000         , 0x00); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_rnint_059, bid128_to_int64_rnint, 0x3012629B8C8925BCF2E6A6B97B400000u128, 20000000000         , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_int64_rnint_060, bid128_to_int64_rnint, 0x3012629B8C8925BCF2E6A6B97B400001u128, 20000000001         , 0x00); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_rnint_061, bid128_to_int64_rnint, 0x3012629B8C893053744A975EF67FFFFFu128, 20000000001         , 0x00); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_rnint_062, bid128_to_int64_rnint, 0x3012629B8C893053744A975EF6800000u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_rnint_063, bid128_to_int64_rnint, 0x3012629B8C893053744A975EF6800001u128, 20000000001         , 0x00); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_rnint_064, bid128_to_int64_rnint, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 20000000001         , 0x00); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_rnint_065, bid128_to_int64_rnint, 0x3012629B8C893AE9F5AE880471C00000u128, 20000000002         , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_int64_rnint_066, bid128_to_int64_rnint, 0x3012629B8C893AE9F5AE880471C00001u128, 20000000002         , 0x00); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_rnint_067, bid128_to_int64_rnint, 0x3016045834312040d33fe76f5ffbdcdfu128, 88117869326         , 0x00);
dec_test!(bid128_to_int64_rnint_068, bid128_to_int64_rnint, 0x3018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 35184372088832      , 0x00); // -- 2^45-ulp
dec_test!(bid128_to_int64_rnint_069, bid128_to_int64_rnint, 0x3018AD78EBC5AC620000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_rnint_070, bid128_to_int64_rnint, 0x3018AD78EBC5AC620000000000000001u128, 35184372088832      , 0x00); // -- 2^45+ulp
dec_test!(bid128_to_int64_rnint_071, bid128_to_int64_rnint, 0x3018AD78EBC5AC64B5E3AF16B187FFFFu128, 35184372088832      , 0x00); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_rnint_072, bid128_to_int64_rnint, 0x3018AD78EBC5AC64B5E3AF16B1880000u128, 35184372088832      , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_int64_rnint_073, bid128_to_int64_rnint, 0x3018AD78EBC5AC64B5E3AF16B1880001u128, 35184372088833      , 0x00); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_rnint_074, bid128_to_int64_rnint, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rnint_075, bid128_to_int64_rnint, 0x301A0000000000A2E6C09AD3E0D40000u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rnint_076, bid128_to_int64_rnint, 0x301A0000000000A2E6C09AD3E0D40001u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rnint_077, bid128_to_int64_rnint, 0x301C629B8C891B265CB1A40684E9FFFFu128, 1999999999999998    , 0x00); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_rnint_078, bid128_to_int64_rnint, 0x301C629B8C891B265CB1A40684EA0000u128, 1999999999999998    , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_int64_rnint_079, bid128_to_int64_rnint, 0x301C629B8C891B265CB1A40684EA0001u128, 1999999999999999    , 0x00); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_rnint_080, bid128_to_int64_rnint, 0x301C629B8C891B2663A1FF60589BFFFFu128, 1999999999999999    , 0x00); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_rnint_081, bid128_to_int64_rnint, 0x301C629B8C891B2663A1FF60589C0000u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_rnint_082, bid128_to_int64_rnint, 0x301C629B8C891B2663A1FF60589C0001u128, 1999999999999999    , 0x00); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_rnint_083, bid128_to_int64_rnint, 0x301C629B8C891B266A925ABA2C4DFFFFu128, 1999999999999999    , 0x00); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_rnint_084, bid128_to_int64_rnint, 0x301C629B8C891B266A925ABA2C4E0000u128, 2000000000000000    , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_int64_rnint_085, bid128_to_int64_rnint, 0x301C629B8C891B266A925ABA2C4E0001u128, 2000000000000000    , 0x00); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_rnint_086, bid128_to_int64_rnint, 0x301C629B8C891B267182B613FFFFFFFFu128, 2000000000000000    , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_int64_rnint_087, bid128_to_int64_rnint, 0x301C629B8C891B267182B61400000000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_rnint_088, bid128_to_int64_rnint, 0x301C629B8C891B267182B61400000001u128, 2000000000000000    , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_int64_rnint_089, bid128_to_int64_rnint, 0x301C629B8C891B267873116DD3B1FFFFu128, 2000000000000000    , 0x00); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_rnint_090, bid128_to_int64_rnint, 0x301C629B8C891B267873116DD3B20000u128, 2000000000000000    , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_int64_rnint_091, bid128_to_int64_rnint, 0x301C629B8C891B267873116DD3B20001u128, 2000000000000001    , 0x00); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_rnint_092, bid128_to_int64_rnint, 0x301C629B8C891B267F636CC7A763FFFFu128, 2000000000000001    , 0x00); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_rnint_093, bid128_to_int64_rnint, 0x301C629B8C891B267F636CC7A7640000u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_rnint_094, bid128_to_int64_rnint, 0x301C629B8C891B267F636CC7A7640001u128, 2000000000000001    , 0x00); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_rnint_095, bid128_to_int64_rnint, 0x301C629B8C891B268653C8217B15FFFFu128, 2000000000000001    , 0x00); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_rnint_096, bid128_to_int64_rnint, 0x301C629B8C891B268653C8217B160000u128, 2000000000000002    , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_int64_rnint_097, bid128_to_int64_rnint, 0x301C629B8C891B268653C8217B160001u128, 2000000000000002    , 0x00); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_rnint_098, bid128_to_int64_rnint, 0x301E000000000001A055690D9DB7FFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_099, bid128_to_int64_rnint, 0x301E000000000001A055690D9DB80000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_100, bid128_to_int64_rnint, 0x301E000000000001A055690D9DB80001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_101, bid128_to_int64_rnint, 0x301E002C68AF0BB140B1A2BC2EC4FFFFu128, 35184372088832      , 0x00); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_rnint_102, bid128_to_int64_rnint, 0x301E002C68AF0BB140B1A2BC2EC50000u128, 35184372088832      , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_int64_rnint_103, bid128_to_int64_rnint, 0x301E002C68AF0BB140B1A2BC2EC50001u128, 35184372088833      , 0x00); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_rnint_104, bid128_to_int64_rnint, 0x302000000000000029A2241AF62BFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_105, bid128_to_int64_rnint, 0x302000000000000029A2241AF62C0000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_106, bid128_to_int64_rnint, 0x302000000000000029A2241AF62C0001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_107, bid128_to_int64_rnint, 0x3020000470DE4DF81FFFFFFFFFFFFFFFu128, 35184372088832      , 0x00); // -- 2^45-ulp
dec_test!(bid128_to_int64_rnint_108, bid128_to_int64_rnint, 0x3020000470DE4DF82000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_rnint_109, bid128_to_int64_rnint, 0x3020000470DE4DF82000000000000001u128, 35184372088832      , 0x00); // -- 2^45+ulp
dec_test!(bid128_to_int64_rnint_110, bid128_to_int64_rnint, 0x302000FC6F7C4045813459C637E07FFFu128, 2000000000000000    , 0x00); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_rnint_111, bid128_to_int64_rnint, 0x302000FC6F7C4045813459C637E08000u128, 2000000000000000    , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_int64_rnint_112, bid128_to_int64_rnint, 0x302000FC6F7C4045813459C637E08001u128, 2000000000000001    , 0x00); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_rnint_113, bid128_to_int64_rnint, 0x302000FC6F7C40458157E0B8A7A17FFFu128, 2000000000000001    , 0x00); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_rnint_114, bid128_to_int64_rnint, 0x302000FC6F7C40458157E0B8A7A18000u128, 2000000000000002    , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_int64_rnint_115, bid128_to_int64_rnint, 0x302000FC6F7C40458157E0B8A7A18001u128, 2000000000000002    , 0x00); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_rnint_116, bid128_to_int64_rnint, 0x30203094024c2800e001a044020a00a2u128, 98528214006468404   , 0x00);
dec_test!(bid128_to_int64_rnint_117, bid128_to_int64_rnint, 0x302200193E5939A08CE4879688D63FFFu128, 1999999999999998    , 0x00); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_rnint_118, bid128_to_int64_rnint, 0x302200193E5939A08CE4879688D64000u128, 1999999999999998    , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_int64_rnint_119, bid128_to_int64_rnint, 0x302200193E5939A08CE4879688D64001u128, 1999999999999999    , 0x00); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_rnint_120, bid128_to_int64_rnint, 0x302200193E5939A08CE815152D9CBFFFu128, 1999999999999999    , 0x00); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_rnint_121, bid128_to_int64_rnint, 0x302200193E5939A08CE815152D9CC000u128, 2000000000000000    , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_int64_rnint_122, bid128_to_int64_rnint, 0x302200193E5939A08CE815152D9CC001u128, 2000000000000000    , 0x00); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_rnint_123, bid128_to_int64_rnint, 0x30238c3652a32db137cc05298519897du128, 8036138098863321296 , 0x00);
dec_test!(bid128_to_int64_rnint_124, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775806 , 0x00); // -- 2^63-1.5-ulp
dec_test!(bid128_to_int64_rnint_125, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775806 , 0x00); // -- 2^63-1.5
dec_test!(bid128_to_int64_rnint_126, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775807 , 0x00); // -- 2^63-1.5+ulp
dec_test!(bid128_to_int64_rnint_127, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775807 , 0x00); // -- 2^63-1-ulp
dec_test!(bid128_to_int64_rnint_128, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFC72815B398000u128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_rnint_129, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFC72815B398001u128, 9223372036854775807 , 0x00); // -- 2^63-1+ulp
dec_test!(bid128_to_int64_rnint_130, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775807 , 0x00); // -- 2^63-0.5-ulp
dec_test!(bid128_to_int64_rnint_131, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFE3940AD9CC000u128, -9223372036854775808, 0x01); // -- 2^63-0.5
dec_test!(bid128_to_int64_rnint_132, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFE3940AD9CC001u128, -9223372036854775808, 0x01); // -- 2^63-0.5+ulp
dec_test!(bid128_to_int64_rnint_133, bid128_to_int64_rnint, 0x3023C6BF52633FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^63-ulp
dec_test!(bid128_to_int64_rnint_134, bid128_to_int64_rnint, 0x3023C6BF526340000000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_rnint_135, bid128_to_int64_rnint, 0x3023C6BF526340000000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+ulp
dec_test!(bid128_to_int64_rnint_136, bid128_to_int64_rnint, 0x3023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x01); // -- 2^63+0.5-ulp
dec_test!(bid128_to_int64_rnint_137, bid128_to_int64_rnint, 0x3023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_rnint_138, bid128_to_int64_rnint, 0x3023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- 2^63+0.5+ulp
dec_test!(bid128_to_int64_rnint_139, bid128_to_int64_rnint, 0x3023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- 2^63+1-ulp
dec_test!(bid128_to_int64_rnint_140, bid128_to_int64_rnint, 0x3023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_rnint_141, bid128_to_int64_rnint, 0x3023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- 2^63+1+ulp
dec_test!(bid128_to_int64_rnint_142, bid128_to_int64_rnint, 0x3023d6a9330e0ee6ffffdffffeffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_143, bid128_to_int64_rnint, 0x3024000000000000006A94D74F42FFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_144, bid128_to_int64_rnint, 0x3024000000000000006A94D74F430000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_145, bid128_to_int64_rnint, 0x3024000000000000006A94D74F430001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_146, bid128_to_int64_rnint, 0x3024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e19-ulp
dec_test!(bid128_to_int64_rnint_147, bid128_to_int64_rnint, 0x3024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_rnint_148, bid128_to_int64_rnint, 0x3024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e19+ulp
dec_test!(bid128_to_int64_rnint_149, bid128_to_int64_rnint, 0x3024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- 1e19+0.5-ulp
dec_test!(bid128_to_int64_rnint_150, bid128_to_int64_rnint, 0x3024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_rnint_151, bid128_to_int64_rnint, 0x3024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- 1e19+0.5+ulp
dec_test!(bid128_to_int64_rnint_152, bid128_to_int64_rnint, 0x302449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- 1.5e19-ulp
dec_test!(bid128_to_int64_rnint_153, bid128_to_int64_rnint, 0x302449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_rnint_154, bid128_to_int64_rnint, 0x302449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- 1.5e19+ulp
dec_test!(bid128_to_int64_rnint_155, bid128_to_int64_rnint, 0x30245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- 2^64-1-ulp
dec_test!(bid128_to_int64_rnint_156, bid128_to_int64_rnint, 0x30245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_rnint_157, bid128_to_int64_rnint, 0x30245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- 2^64-1+ulp
dec_test!(bid128_to_int64_rnint_158, bid128_to_int64_rnint, 0x30245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- 2^64-0.5-ulp
dec_test!(bid128_to_int64_rnint_159, bid128_to_int64_rnint, 0x30245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_rnint_160, bid128_to_int64_rnint, 0x30245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- 2^64-0.5+ulp
dec_test!(bid128_to_int64_rnint_161, bid128_to_int64_rnint, 0x30245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-ulp
dec_test!(bid128_to_int64_rnint_162, bid128_to_int64_rnint, 0x30245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_rnint_163, bid128_to_int64_rnint, 0x30245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+ulp
dec_test!(bid128_to_int64_rnint_164, bid128_to_int64_rnint, 0x30245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- 2^64+0.5-ulp
dec_test!(bid128_to_int64_rnint_165, bid128_to_int64_rnint, 0x30245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_rnint_166, bid128_to_int64_rnint, 0x30245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- 2^64+0.5+ulp
dec_test!(bid128_to_int64_rnint_167, bid128_to_int64_rnint, 0x30245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- 2^64+1-ulp
dec_test!(bid128_to_int64_rnint_168, bid128_to_int64_rnint, 0x30245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_rnint_169, bid128_to_int64_rnint, 0x30245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- 2^64+1+ulp
dec_test!(bid128_to_int64_rnint_170, bid128_to_int64_rnint, 0x3024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2e19-ulp
dec_test!(bid128_to_int64_rnint_171, bid128_to_int64_rnint, 0x3024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_rnint_172, bid128_to_int64_rnint, 0x3024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- 2e19+ulp
dec_test!(bid128_to_int64_rnint_173, bid128_to_int64_rnint, 0x30247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2.5e19-ulp
dec_test!(bid128_to_int64_rnint_174, bid128_to_int64_rnint, 0x30247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_rnint_175, bid128_to_int64_rnint, 0x30247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- 2.5e19+ulp
dec_test!(bid128_to_int64_rnint_176, bid128_to_int64_rnint, 0x3026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e20-ulp
dec_test!(bid128_to_int64_rnint_177, bid128_to_int64_rnint, 0x3026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_rnint_178, bid128_to_int64_rnint, 0x3026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e20+ulp
dec_test!(bid128_to_int64_rnint_179, bid128_to_int64_rnint, 0x302A00000000006C6B935B68D08DA3FFu128, 19999999998         , 0x00); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_rnint_180, bid128_to_int64_rnint, 0x302A00000000006C6B935B68D08DA400u128, 19999999998         , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_int64_rnint_181, bid128_to_int64_rnint, 0x302A00000000006C6B935B68D08DA401u128, 19999999999         , 0x00); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_rnint_182, bid128_to_int64_rnint, 0x302A00000000006C6B935B8019048BFFu128, 19999999999         , 0x00); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_rnint_183, bid128_to_int64_rnint, 0x302A00000000006C6B935B8019048C00u128, 20000000000         , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_int64_rnint_184, bid128_to_int64_rnint, 0x302A00000000006C6B935B8019048C01u128, 20000000000         , 0x00); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_rnint_185, bid128_to_int64_rnint, 0x302C000000000000000002BBA7F521FFu128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rnint_186, bid128_to_int64_rnint, 0x302C000000000000000002BBA7F52200u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rnint_187, bid128_to_int64_rnint, 0x302C000000000000000002BBA7F52201u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rnint_188, bid128_to_int64_rnint, 0x302C00000000000AD78EBC5872141BFFu128, 19999999999         , 0x00); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_rnint_189, bid128_to_int64_rnint, 0x302C00000000000AD78EBC5872141C00u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_rnint_190, bid128_to_int64_rnint, 0x302C00000000000AD78EBC5872141C01u128, 19999999999         , 0x00); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_rnint_191, bid128_to_int64_rnint, 0x302C00000000000AD78EBC5BF025F1FFu128, 20000000000         , 0x00); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_rnint_192, bid128_to_int64_rnint, 0x302C00000000000AD78EBC5BF025F200u128, 20000000000         , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_int64_rnint_193, bid128_to_int64_rnint, 0x302C00000000000AD78EBC5BF025F201u128, 20000000001         , 0x00); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_rnint_194, bid128_to_int64_rnint, 0x302C00000000000AD78EBC5E4431D5FFu128, 20000000001         , 0x00); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_rnint_195, bid128_to_int64_rnint, 0x302C00000000000AD78EBC5E4431D600u128, 20000000002         , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_int64_rnint_196, bid128_to_int64_rnint, 0x302C00000000000AD78EBC5E4431D601u128, 20000000002         , 0x00); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_rnint_197, bid128_to_int64_rnint, 0x302C000000108B2A2C28028E3FF41BFFu128, 1999999999999999    , 0x00); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_rnint_198, bid128_to_int64_rnint, 0x302C000000108B2A2C28028E3FF41C00u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_rnint_199, bid128_to_int64_rnint, 0x302C000000108B2A2C28028E3FF41C01u128, 1999999999999999    , 0x00); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_rnint_200, bid128_to_int64_rnint, 0x302E000000000001158E46094F6AC9FFu128, 20000000001         , 0x00); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_rnint_201, bid128_to_int64_rnint, 0x302E000000000001158E46094F6ACA00u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_rnint_202, bid128_to_int64_rnint, 0x302E000000000001158E46094F6ACA01u128, 20000000001         , 0x00); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_rnint_203, bid128_to_int64_rnint, 0x302E00000001A784379D99DB7D9AC9FFu128, 2000000000000001    , 0x00); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_rnint_204, bid128_to_int64_rnint, 0x302E00000001A784379D99DB7D9ACA00u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_rnint_205, bid128_to_int64_rnint, 0x302E00000001A784379D99DB7D9ACA01u128, 2000000000000001    , 0x00); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_rnint_206, bid128_to_int64_rnint, 0x303000000000000000000006FC23ABFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_207, bid128_to_int64_rnint, 0x303000000000000000000006FC23AC00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_208, bid128_to_int64_rnint, 0x303000000000000000000006FC23AC01u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_209, bid128_to_int64_rnint, 0x303200000000000000000000B2D05DFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_210, bid128_to_int64_rnint, 0x303200000000000000000000B2D05E00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_211, bid128_to_int64_rnint, 0x303200000000000000000000B2D05E01u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_212, bid128_to_int64_rnint, 0x303800000000000000000000002DDA47u128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rnint_213, bid128_to_int64_rnint, 0x303800000000000000000000002DDA48u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rnint_214, bid128_to_int64_rnint, 0x303800000000000000000000002DDA49u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rnint_215, bid128_to_int64_rnint, 0x303A00000000000000000000000003E7u128, 1                   , 0x00); // -- 0.999
dec_test!(bid128_to_int64_rnint_216, bid128_to_int64_rnint, 0x303A00000000000000000000000495D3u128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rnint_217, bid128_to_int64_rnint, 0x303A00000000000000000000000495D4u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rnint_218, bid128_to_int64_rnint, 0x303A00000000000000000000000495D5u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rnint_219, bid128_to_int64_rnint, 0x303C0000000000000000000000007561u128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rnint_220, bid128_to_int64_rnint, 0x303C0000000000000000000000007562u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rnint_221, bid128_to_int64_rnint, 0x303C0000000000000000000000007563u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rnint_222, bid128_to_int64_rnint, 0x303E0000000000000000000000000005u128, 0                   , 0x00); // -- 0.5
dec_test!(bid128_to_int64_rnint_223, bid128_to_int64_rnint, 0x303E000000000000000000000000000Fu128, 2                   , 0x00); // -- 1.5
dec_test!(bid128_to_int64_rnint_224, bid128_to_int64_rnint, 0x303E0000000000000000000000000BB7u128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_225, bid128_to_int64_rnint, 0x303E0000000000000000000000000BB8u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_226, bid128_to_int64_rnint, 0x303E0000000000000000000000000BB9u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_227, bid128_to_int64_rnint, 0x303E0000000000000000000000000BBDu128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rnint_228, bid128_to_int64_rnint, 0x303E0000000000000000002E90EDCFF1u128, 19999999998         , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_int64_rnint_229, bid128_to_int64_rnint, 0x303E0000000000000000002E90EDCFFBu128, 20000000000         , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_int64_rnint_230, bid128_to_int64_rnint, 0x303E0000000000000000002E90EDD005u128, 20000000000         , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_int64_rnint_231, bid128_to_int64_rnint, 0x303E0000000000000000002E90EDD00Fu128, 20000000002         , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_int64_rnint_232, bid128_to_int64_rnint, 0x303E0000000000000001400000000005u128, 35184372088832      , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_int64_rnint_233, bid128_to_int64_rnint, 0x303E00000000000000470DE4DF81FFF1u128, 1999999999999998    , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_int64_rnint_234, bid128_to_int64_rnint, 0x303E00000000000000470DE4DF81FFFBu128, 2000000000000000    , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_int64_rnint_235, bid128_to_int64_rnint, 0x303E00000000000000470DE4DF820005u128, 2000000000000000    , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_int64_rnint_236, bid128_to_int64_rnint, 0x303E00000000000000470DE4DF82000Fu128, 2000000000000002    , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_int64_rnint_237, bid128_to_int64_rnint, 0x303E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775806 , 0x00); // -- 2^63-1.5
dec_test!(bid128_to_int64_rnint_238, bid128_to_int64_rnint, 0x303E000000000004FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- 2^63-0.5
dec_test!(bid128_to_int64_rnint_239, bid128_to_int64_rnint, 0x303E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_rnint_240, bid128_to_int64_rnint, 0x303E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_rnint_241, bid128_to_int64_rnint, 0x303E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_rnint_242, bid128_to_int64_rnint, 0x303E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_rnint_243, bid128_to_int64_rnint, 0x30400000000000000000000000000001u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_rnint_244, bid128_to_int64_rnint, 0x3040000000000000000000000000012Bu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_245, bid128_to_int64_rnint, 0x3040000000000000000000000000012Cu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_246, bid128_to_int64_rnint, 0x3040000000000000000000000000012Du128, 301                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_247, bid128_to_int64_rnint, 0x30400000000000000000000301002000u128, 12901687296         , 0x00);
dec_test!(bid128_to_int64_rnint_248, bid128_to_int64_rnint, 0x304000000000000000000004A817C7FFu128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_rnint_249, bid128_to_int64_rnint, 0x304000000000000000000004A817C801u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_rnint_250, bid128_to_int64_rnint, 0x30400000000000000000200000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_rnint_251, bid128_to_int64_rnint, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_rnint_252, bid128_to_int64_rnint, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_int64_rnint_253, bid128_to_int64_rnint, 0x304000000000000000071AFD498D0000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_rnint_254, bid128_to_int64_rnint, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_rnint_255, bid128_to_int64_rnint, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_int64_rnint_256, bid128_to_int64_rnint, 0x30400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_rnint_257, bid128_to_int64_rnint, 0x30400000000000008000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_rnint_258, bid128_to_int64_rnint, 0x30400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_rnint_259, bid128_to_int64_rnint, 0x30400000000000008000c00200000400u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_260, bid128_to_int64_rnint, 0x3040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_rnint_261, bid128_to_int64_rnint, 0x30400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_rnint_262, bid128_to_int64_rnint, 0x30400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_rnint_263, bid128_to_int64_rnint, 0x3041ED09BEAD87C0378D8E6400000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_264, bid128_to_int64_rnint, 0x3042000000000000000000000000001Du128, 290                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rnint_265, bid128_to_int64_rnint, 0x3042000000000000000000000000001Eu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_266, bid128_to_int64_rnint, 0x3042000000000000000000000000001Fu128, 310                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rnint_267, bid128_to_int64_rnint, 0x304200000000000000000000773593FFu128, 19999999990         , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_int64_rnint_268, bid128_to_int64_rnint, 0x30420000000000000000000077359400u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_rnint_269, bid128_to_int64_rnint, 0x30420000000000000000000077359401u128, 20000000010         , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_int64_rnint_270, bid128_to_int64_rnint, 0x30440000000000000000000000000003u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rnint_271, bid128_to_int64_rnint, 0x30520000000000000000000000000004u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_rnint_272, bid128_to_int64_rnint, 0x30520000000000000000000000000005u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_rnint_273, bid128_to_int64_rnint, 0x30540000000000000000000000000002u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_rnint_274, bid128_to_int64_rnint, 0x305E0000000000000000000000000002u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_rnint_275, bid128_to_int64_rnint, 0x3064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_rnint_276, bid128_to_int64_rnint, 0x30640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_rnint_277, bid128_to_int64_rnint, 0x30660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_rnint_278, bid128_to_int64_rnint, 0x30660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_rnint_279, bid128_to_int64_rnint, 0x30680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_rnint_280, bid128_to_int64_rnint, 0x36b0c7b00d24f0494ddc84f22c6b4570u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_281, bid128_to_int64_rnint, 0x438f05162c6d6d4c6bf427ae950bce88u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_282, bid128_to_int64_rnint, 0x43ebb89d88797a9d62cb7c75ab88bae1u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_283, bid128_to_int64_rnint, 0x49173fd9fb37ed111637144320ec2a77u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_284, bid128_to_int64_rnint, 0x500329262f4cd9f739f1cc85e630d2a2u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_285, bid128_to_int64_rnint, "5.5"                                 , 6                   , 0x00);
dec_test!(bid128_to_int64_rnint_286, bid128_to_int64_rnint, 0x56f943d1681e501081edac7a982c7dfcu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_287, bid128_to_int64_rnint, 0x5913566d53157b9dd918e032cc45f239u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_288, bid128_to_int64_rnint, 0x78000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_289, bid128_to_int64_rnint, 0x78000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_290, bid128_to_int64_rnint, 0x7bdffbefddb3fb7fbffe37ff5eddc89bu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_291, bid128_to_int64_rnint, 0x7c000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_292, bid128_to_int64_rnint, 0x7c003fffffffffff38c15b08ffffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_293, bid128_to_int64_rnint, 0x7c003fffffffffff38c15b0affffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_294, bid128_to_int64_rnint, 0x7dff4dfb65effddea000800040200280u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_295, bid128_to_int64_rnint, 0x7e000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_296, bid128_to_int64_rnint, 0x8040050008112080ffff7effffff77ffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_297, bid128_to_int64_rnint, "+889.988999988999998E0"              , 890                 , 0x00);
dec_test!(bid128_to_int64_rnint_298, bid128_to_int64_rnint, 0x918aabf124f3341d7b8d0a6f0811905cu128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_299, bid128_to_int64_rnint, "-9"                                  , -9                  , 0x00);
dec_test!(bid128_to_int64_rnint_300, bid128_to_int64_rnint, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_int64_rnint_301, bid128_to_int64_rnint, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0                   , 0x00); // -- -(0.5)
dec_test!(bid128_to_int64_rnint_302, bid128_to_int64_rnint, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1                  , 0x00); // -- -(0.5+ulp)
dec_test!(bid128_to_int64_rnint_303, bid128_to_int64_rnint, 0xaffda6a3fd7cce7e09120383ad0c10a3u128, -1                  , 0x00);
dec_test!(bid128_to_int64_rnint_304, bid128_to_int64_rnint, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1                  , 0x00); // -- -(0.999-ulp)
dec_test!(bid128_to_int64_rnint_305, bid128_to_int64_rnint, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1                  , 0x00); // -- -(0.999)
dec_test!(bid128_to_int64_rnint_306, bid128_to_int64_rnint, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1                  , 0x00); // -- -(0.999+ulp)
dec_test!(bid128_to_int64_rnint_307, bid128_to_int64_rnint, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1                  , 0x00); // -- -(1-ulp)
dec_test!(bid128_to_int64_rnint_308, bid128_to_int64_rnint, 0xAFFE314DC6448D9338C15B0A00000000u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_rnint_309, bid128_to_int64_rnint, 0xAFFE314DC6448D9338C15B0A00000001u128, -1                  , 0x00); // -- -(1+ulp)
dec_test!(bid128_to_int64_rnint_310, bid128_to_int64_rnint, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1                  , 0x00); // -- -(1.5-ulp)
dec_test!(bid128_to_int64_rnint_311, bid128_to_int64_rnint, 0xAFFE49F4A966D45CD522088F00000000u128, -2                  , 0x00); // -- -(1.5)
dec_test!(bid128_to_int64_rnint_312, bid128_to_int64_rnint, 0xAFFE49F4A966D45CD522088F00000001u128, -2                  , 0x00); // -- -(1.5+ulp)
dec_test!(bid128_to_int64_rnint_313, bid128_to_int64_rnint, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_314, bid128_to_int64_rnint, 0xB00293E952CDA8B9AA44111E00000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_315, bid128_to_int64_rnint, 0xB00293E952CDA8B9AA44111E00000001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_316, bid128_to_int64_rnint, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rnint_317, bid128_to_int64_rnint, 0xB00294286EACB8CB0A8CB6B140000000u128, -300                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rnint_318, bid128_to_int64_rnint, 0xB00294286EACB8CB0A8CB6B140000001u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rnint_319, bid128_to_int64_rnint, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_320, bid128_to_int64_rnint, 0xB0040ECA8847C4129106CE8300000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_321, bid128_to_int64_rnint, 0xB0040ECA8847C4129106CE8300000001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_322, bid128_to_int64_rnint, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_323, bid128_to_int64_rnint, 0xB00A0003C95A2F0B4856475FE0000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_324, bid128_to_int64_rnint, 0xB00A0003C95A2F0B4856475FE0000001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_325, bid128_to_int64_rnint, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_326, bid128_to_int64_rnint, 0xB00C000060EF6B1ABA6F072330000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_327, bid128_to_int64_rnint, 0xB00C000060EF6B1ABA6F072330000001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_328, bid128_to_int64_rnint, 0xB010C5371912364CE3056C27FFFFFFFFu128, -4000000000         , 0x00); // -- -(4e9-ulp)
dec_test!(bid128_to_int64_rnint_329, bid128_to_int64_rnint, 0xB010C5371912364CE3056C2800000000u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_rnint_330, bid128_to_int64_rnint, 0xB010C5371912364CE3056C2800000001u128, -4000000000         , 0x00); // -- -(4e9+ulp)
dec_test!(bid128_to_int64_rnint_331, bid128_to_int64_rnint, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -5000000000         , 0x00); // -- -(5e9-ulp)
dec_test!(bid128_to_int64_rnint_332, bid128_to_int64_rnint, 0xB010F684DF56C3E01BC6C73200000000u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_rnint_333, bid128_to_int64_rnint, 0xB010F684DF56C3E01BC6C73200000001u128, -5000000000         , 0x00); // -- -(5e9+ulp)
dec_test!(bid128_to_int64_rnint_334, bid128_to_int64_rnint, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -19999999998        , 0x00); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_rnint_335, bid128_to_int64_rnint, 0xB012629B8C88FB62ED56E4238E400000u128, -19999999998        , 0x00); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_rnint_336, bid128_to_int64_rnint, 0xB012629B8C88FB62ED56E4238E400001u128, -19999999999        , 0x00); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_rnint_337, bid128_to_int64_rnint, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -19999999999        , 0x00); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_rnint_338, bid128_to_int64_rnint, 0xB012629B8C8905F96EBAD4C909800000u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_rnint_339, bid128_to_int64_rnint, 0xB012629B8C8905F96EBAD4C909800001u128, -19999999999        , 0x00); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_rnint_340, bid128_to_int64_rnint, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -19999999999        , 0x00); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_rnint_341, bid128_to_int64_rnint, 0xB012629B8C89108FF01EC56E84C00000u128, -20000000000        , 0x00); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_rnint_342, bid128_to_int64_rnint, 0xB012629B8C89108FF01EC56E84C00001u128, -20000000000        , 0x00); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_rnint_343, bid128_to_int64_rnint, 0xB012629B8C891B267182B613FFFFFFFFu128, -20000000000        , 0x00); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_rnint_344, bid128_to_int64_rnint, 0xB012629B8C891B267182B61400000000u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_rnint_345, bid128_to_int64_rnint, 0xB012629B8C891B267182B61400000001u128, -20000000000        , 0x00); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_rnint_346, bid128_to_int64_rnint, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -20000000000        , 0x00); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_rnint_347, bid128_to_int64_rnint, 0xB012629B8C8925BCF2E6A6B97B400000u128, -20000000000        , 0x00); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_rnint_348, bid128_to_int64_rnint, 0xB012629B8C8925BCF2E6A6B97B400001u128, -20000000001        , 0x00); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_rnint_349, bid128_to_int64_rnint, 0xB012629B8C893053744A975EF67FFFFFu128, -20000000001        , 0x00); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_rnint_350, bid128_to_int64_rnint, 0xB012629B8C893053744A975EF6800000u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_rnint_351, bid128_to_int64_rnint, 0xB012629B8C893053744A975EF6800001u128, -20000000001        , 0x00); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_rnint_352, bid128_to_int64_rnint, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -20000000001        , 0x00); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_rnint_353, bid128_to_int64_rnint, 0xB012629B8C893AE9F5AE880471C00000u128, -20000000002        , 0x00); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_rnint_354, bid128_to_int64_rnint, 0xB012629B8C893AE9F5AE880471C00001u128, -20000000002        , 0x00); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_rnint_355, bid128_to_int64_rnint, 0xb013836206c179d47f3be86fff7d5feeu128, -78570589673        , 0x00);
dec_test!(bid128_to_int64_rnint_356, bid128_to_int64_rnint, 0xB018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, -35184372088832     , 0x00); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_rnint_357, bid128_to_int64_rnint, 0xB018AD78EBC5AC620000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_rnint_358, bid128_to_int64_rnint, 0xB018AD78EBC5AC620000000000000001u128, -35184372088832     , 0x00); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_rnint_359, bid128_to_int64_rnint, 0xB018AD78EBC5AC64B5E3AF16B187FFFFu128, -35184372088832     , 0x00); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_rnint_360, bid128_to_int64_rnint, 0xB018AD78EBC5AC64B5E3AF16B1880000u128, -35184372088832     , 0x00); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_rnint_361, bid128_to_int64_rnint, 0xB018AD78EBC5AC64B5E3AF16B1880001u128, -35184372088833     , 0x00); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_rnint_362, bid128_to_int64_rnint, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rnint_363, bid128_to_int64_rnint, 0xB01A0000000000A2E6C09AD3E0D40000u128, -300                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rnint_364, bid128_to_int64_rnint, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rnint_365, bid128_to_int64_rnint, 0xB01C629B8C891B265CB1A40684E9FFFFu128, -1999999999999998   , 0x00); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_rnint_366, bid128_to_int64_rnint, 0xB01C629B8C891B265CB1A40684EA0000u128, -1999999999999998   , 0x00); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_rnint_367, bid128_to_int64_rnint, 0xB01C629B8C891B265CB1A40684EA0001u128, -1999999999999999   , 0x00); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_rnint_368, bid128_to_int64_rnint, 0xB01C629B8C891B2663A1FF60589BFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_rnint_369, bid128_to_int64_rnint, 0xB01C629B8C891B2663A1FF60589C0000u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_rnint_370, bid128_to_int64_rnint, 0xB01C629B8C891B2663A1FF60589C0001u128, -1999999999999999   , 0x00); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_rnint_371, bid128_to_int64_rnint, 0xB01C629B8C891B266A925ABA2C4DFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_rnint_372, bid128_to_int64_rnint, 0xB01C629B8C891B266A925ABA2C4E0000u128, -2000000000000000   , 0x00); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_rnint_373, bid128_to_int64_rnint, 0xB01C629B8C891B266A925ABA2C4E0001u128, -2000000000000000   , 0x00); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_rnint_374, bid128_to_int64_rnint, 0xB01C629B8C891B267182B613FFFFFFFFu128, -2000000000000000   , 0x00); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_rnint_375, bid128_to_int64_rnint, 0xB01C629B8C891B267182B61400000000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_rnint_376, bid128_to_int64_rnint, 0xB01C629B8C891B267182B61400000001u128, -2000000000000000   , 0x00); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_rnint_377, bid128_to_int64_rnint, 0xB01C629B8C891B267873116DD3B1FFFFu128, -2000000000000000   , 0x00); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_rnint_378, bid128_to_int64_rnint, 0xB01C629B8C891B267873116DD3B20000u128, -2000000000000000   , 0x00); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_rnint_379, bid128_to_int64_rnint, 0xB01C629B8C891B267873116DD3B20001u128, -2000000000000001   , 0x00); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_rnint_380, bid128_to_int64_rnint, 0xB01C629B8C891B267F636CC7A763FFFFu128, -2000000000000001   , 0x00); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_rnint_381, bid128_to_int64_rnint, 0xB01C629B8C891B267F636CC7A7640000u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_rnint_382, bid128_to_int64_rnint, 0xB01C629B8C891B267F636CC7A7640001u128, -2000000000000001   , 0x00); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_rnint_383, bid128_to_int64_rnint, 0xB01C629B8C891B268653C8217B15FFFFu128, -2000000000000001   , 0x00); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_rnint_384, bid128_to_int64_rnint, 0xB01C629B8C891B268653C8217B160000u128, -2000000000000002   , 0x00); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_rnint_385, bid128_to_int64_rnint, 0xB01C629B8C891B268653C8217B160001u128, -2000000000000002   , 0x00); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_rnint_386, bid128_to_int64_rnint, 0xB01E000000000001A055690D9DB7FFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_387, bid128_to_int64_rnint, 0xB01E000000000001A055690D9DB80000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_388, bid128_to_int64_rnint, 0xB01E000000000001A055690D9DB80001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_389, bid128_to_int64_rnint, 0xB01E002C68AF0BB140B1A2BC2EC4FFFFu128, -35184372088832     , 0x00); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_rnint_390, bid128_to_int64_rnint, 0xB01E002C68AF0BB140B1A2BC2EC50000u128, -35184372088832     , 0x00); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_rnint_391, bid128_to_int64_rnint, 0xB01E002C68AF0BB140B1A2BC2EC50001u128, -35184372088833     , 0x00); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_rnint_392, bid128_to_int64_rnint, 0xB02000000000000029A2241AF62BFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_393, bid128_to_int64_rnint, 0xB02000000000000029A2241AF62C0000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_394, bid128_to_int64_rnint, 0xB02000000000000029A2241AF62C0001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_395, bid128_to_int64_rnint, 0xB020000470DE4DF81FFFFFFFFFFFFFFFu128, -35184372088832     , 0x00); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_rnint_396, bid128_to_int64_rnint, 0xB020000470DE4DF82000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_rnint_397, bid128_to_int64_rnint, 0xB020000470DE4DF82000000000000001u128, -35184372088832     , 0x00); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_rnint_398, bid128_to_int64_rnint, 0xB02000FC6F7C4045813459C637E07FFFu128, -2000000000000000   , 0x00); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_rnint_399, bid128_to_int64_rnint, 0xB02000FC6F7C4045813459C637E08000u128, -2000000000000000   , 0x00); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_rnint_400, bid128_to_int64_rnint, 0xB02000FC6F7C4045813459C637E08001u128, -2000000000000001   , 0x00); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_rnint_401, bid128_to_int64_rnint, 0xB02000FC6F7C40458157E0B8A7A17FFFu128, -2000000000000001   , 0x00); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_rnint_402, bid128_to_int64_rnint, 0xB02000FC6F7C40458157E0B8A7A18000u128, -2000000000000002   , 0x00); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_rnint_403, bid128_to_int64_rnint, 0xB02000FC6F7C40458157E0B8A7A18001u128, -2000000000000002   , 0x00); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_rnint_404, bid128_to_int64_rnint, 0xB02200193E5939A08CE4879688D63FFFu128, -1999999999999998   , 0x00); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_rnint_405, bid128_to_int64_rnint, 0xB02200193E5939A08CE4879688D64000u128, -1999999999999998   , 0x00); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_rnint_406, bid128_to_int64_rnint, 0xB02200193E5939A08CE4879688D64001u128, -1999999999999999   , 0x00); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_rnint_407, bid128_to_int64_rnint, 0xB02200193E5939A08CE815152D9CBFFFu128, -1999999999999999   , 0x00); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_rnint_408, bid128_to_int64_rnint, 0xB02200193E5939A08CE815152D9CC000u128, -2000000000000000   , 0x00); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_rnint_409, bid128_to_int64_rnint, 0xB02200193E5939A08CE815152D9CC001u128, -2000000000000000   , 0x00); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_rnint_410, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFAABC208D63FFFu128, -9223372036854775806, 0x00); // -- -(2^63-1.5-ulp)
dec_test!(bid128_to_int64_rnint_411, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFAABC208D64000u128, -9223372036854775806, 0x00); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_rnint_412, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFAABC208D64001u128, -9223372036854775807, 0x00); // -- -(2^63-1.5+ulp)
dec_test!(bid128_to_int64_rnint_413, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFC72815B397FFFu128, -9223372036854775807, 0x00); // -- -(2^63-1-ulp)
dec_test!(bid128_to_int64_rnint_414, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFC72815B398000u128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_rnint_415, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFC72815B398001u128, -9223372036854775807, 0x00); // -- -(2^63-1+ulp)
dec_test!(bid128_to_int64_rnint_416, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFE3940AD9CBFFFu128, -9223372036854775807, 0x00); // -- -(2^63-0.5-ulp)
dec_test!(bid128_to_int64_rnint_417, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFE3940AD9CC000u128, -9223372036854775808, 0x00); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_rnint_418, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFE3940AD9CC001u128, -9223372036854775808, 0x00); // -- -(2^63-0.5+ulp)
dec_test!(bid128_to_int64_rnint_419, bid128_to_int64_rnint, 0xB023C6BF52633FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x00); // -- -(2^63-ulp)
dec_test!(bid128_to_int64_rnint_420, bid128_to_int64_rnint, 0xB023C6BF526340000000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_rnint_421, bid128_to_int64_rnint, 0xB023C6BF526340000000000000000001u128, -9223372036854775808, 0x00); // -- -(2^63+ulp)
dec_test!(bid128_to_int64_rnint_422, bid128_to_int64_rnint, 0xB023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x00); // -- -(2^63+0.5-ulp)
dec_test!(bid128_to_int64_rnint_423, bid128_to_int64_rnint, 0xB023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x00); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_rnint_424, bid128_to_int64_rnint, 0xB023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- -(2^63+0.5+ulp)
dec_test!(bid128_to_int64_rnint_425, bid128_to_int64_rnint, 0xB023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- -(2^63+1-ulp)
dec_test!(bid128_to_int64_rnint_426, bid128_to_int64_rnint, 0xB023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_rnint_427, bid128_to_int64_rnint, 0xB023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- -(2^63+1+ulp)
dec_test!(bid128_to_int64_rnint_428, bid128_to_int64_rnint, 0xB024000000000000006A94D74F42FFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_429, bid128_to_int64_rnint, 0xB024000000000000006A94D74F430000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_430, bid128_to_int64_rnint, 0xB024000000000000006A94D74F430001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_431, bid128_to_int64_rnint, 0xb0243143100501687dfe7deaf79d7fefu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_432, bid128_to_int64_rnint, 0xB024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e19-ulp)
dec_test!(bid128_to_int64_rnint_433, bid128_to_int64_rnint, 0xB024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_rnint_434, bid128_to_int64_rnint, 0xB024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e19+ulp)
dec_test!(bid128_to_int64_rnint_435, bid128_to_int64_rnint, 0xB024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- -(1e19+0.5-ulp)
dec_test!(bid128_to_int64_rnint_436, bid128_to_int64_rnint, 0xB024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_rnint_437, bid128_to_int64_rnint, 0xB024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- -(1e19+0.5+ulp)
dec_test!(bid128_to_int64_rnint_438, bid128_to_int64_rnint, 0xB02449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1.5e19-ulp)
dec_test!(bid128_to_int64_rnint_439, bid128_to_int64_rnint, 0xB02449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_rnint_440, bid128_to_int64_rnint, 0xB02449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- -(1.5e19+ulp)
dec_test!(bid128_to_int64_rnint_441, bid128_to_int64_rnint, 0xB0245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1-ulp)
dec_test!(bid128_to_int64_rnint_442, bid128_to_int64_rnint, 0xB0245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_rnint_443, bid128_to_int64_rnint, 0xB0245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- -(2^64-1+ulp)
dec_test!(bid128_to_int64_rnint_444, bid128_to_int64_rnint, 0xB0245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- -(2^64-0.5-ulp)
dec_test!(bid128_to_int64_rnint_445, bid128_to_int64_rnint, 0xB0245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_rnint_446, bid128_to_int64_rnint, 0xB0245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- -(2^64-0.5+ulp)
dec_test!(bid128_to_int64_rnint_447, bid128_to_int64_rnint, 0xB0245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-ulp)
dec_test!(bid128_to_int64_rnint_448, bid128_to_int64_rnint, 0xB0245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_rnint_449, bid128_to_int64_rnint, 0xB0245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+ulp)
dec_test!(bid128_to_int64_rnint_450, bid128_to_int64_rnint, 0xB0245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- -(2^64+0.5-ulp)
dec_test!(bid128_to_int64_rnint_451, bid128_to_int64_rnint, 0xB0245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_rnint_452, bid128_to_int64_rnint, 0xB0245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- -(2^64+0.5+ulp)
dec_test!(bid128_to_int64_rnint_453, bid128_to_int64_rnint, 0xB0245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- -(2^64+1-ulp)
dec_test!(bid128_to_int64_rnint_454, bid128_to_int64_rnint, 0xB0245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_rnint_455, bid128_to_int64_rnint, 0xB0245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- -(2^64+1+ulp)
dec_test!(bid128_to_int64_rnint_456, bid128_to_int64_rnint, 0xB024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2e19-ulp)
dec_test!(bid128_to_int64_rnint_457, bid128_to_int64_rnint, 0xB024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_rnint_458, bid128_to_int64_rnint, 0xB024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- -(2e19+ulp)
dec_test!(bid128_to_int64_rnint_459, bid128_to_int64_rnint, 0xB0247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2.5e19-ulp)
dec_test!(bid128_to_int64_rnint_460, bid128_to_int64_rnint, 0xB0247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_rnint_461, bid128_to_int64_rnint, 0xB0247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- -(2.5e19+ulp)
dec_test!(bid128_to_int64_rnint_462, bid128_to_int64_rnint, 0xB026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e20-ulp)
dec_test!(bid128_to_int64_rnint_463, bid128_to_int64_rnint, 0xB026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_rnint_464, bid128_to_int64_rnint, 0xB026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e20+ulp)
dec_test!(bid128_to_int64_rnint_465, bid128_to_int64_rnint, 0xB02A00000000006C6B935B68D08DA3FFu128, -19999999998        , 0x00); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_rnint_466, bid128_to_int64_rnint, 0xB02A00000000006C6B935B68D08DA400u128, -19999999998        , 0x00); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_rnint_467, bid128_to_int64_rnint, 0xB02A00000000006C6B935B68D08DA401u128, -19999999999        , 0x00); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_rnint_468, bid128_to_int64_rnint, 0xB02A00000000006C6B935B8019048BFFu128, -19999999999        , 0x00); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_rnint_469, bid128_to_int64_rnint, 0xB02A00000000006C6B935B8019048C00u128, -20000000000        , 0x00); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_rnint_470, bid128_to_int64_rnint, 0xB02A00000000006C6B935B8019048C01u128, -20000000000        , 0x00); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_rnint_471, bid128_to_int64_rnint, 0xB02C000000000000000002BBA7F521FFu128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rnint_472, bid128_to_int64_rnint, 0xB02C000000000000000002BBA7F52200u128, -300                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rnint_473, bid128_to_int64_rnint, 0xB02C000000000000000002BBA7F52201u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rnint_474, bid128_to_int64_rnint, 0xB02C00000000000AD78EBC5872141BFFu128, -19999999999        , 0x00); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_rnint_475, bid128_to_int64_rnint, 0xB02C00000000000AD78EBC5872141C00u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_rnint_476, bid128_to_int64_rnint, 0xB02C00000000000AD78EBC5872141C01u128, -19999999999        , 0x00); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_rnint_477, bid128_to_int64_rnint, 0xB02C00000000000AD78EBC5BF025F1FFu128, -20000000000        , 0x00); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_rnint_478, bid128_to_int64_rnint, 0xB02C00000000000AD78EBC5BF025F200u128, -20000000000        , 0x00); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_rnint_479, bid128_to_int64_rnint, 0xB02C00000000000AD78EBC5BF025F201u128, -20000000001        , 0x00); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_rnint_480, bid128_to_int64_rnint, 0xB02C00000000000AD78EBC5E4431D5FFu128, -20000000001        , 0x00); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_rnint_481, bid128_to_int64_rnint, 0xB02C00000000000AD78EBC5E4431D600u128, -20000000002        , 0x00); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_rnint_482, bid128_to_int64_rnint, 0xB02C00000000000AD78EBC5E4431D601u128, -20000000002        , 0x00); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_rnint_483, bid128_to_int64_rnint, 0xB02C000000108B2A2C28028E3FF41BFFu128, -1999999999999999   , 0x00); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_rnint_484, bid128_to_int64_rnint, 0xB02C000000108B2A2C28028E3FF41C00u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_rnint_485, bid128_to_int64_rnint, 0xB02C000000108B2A2C28028E3FF41C01u128, -1999999999999999   , 0x00); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_rnint_486, bid128_to_int64_rnint, 0xB02E000000000001158E46094F6AC9FFu128, -20000000001        , 0x00); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_rnint_487, bid128_to_int64_rnint, 0xB02E000000000001158E46094F6ACA00u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_rnint_488, bid128_to_int64_rnint, 0xB02E000000000001158E46094F6ACA01u128, -20000000001        , 0x00); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_rnint_489, bid128_to_int64_rnint, 0xB02E00000001A784379D99DB7D9AC9FFu128, -2000000000000001   , 0x00); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_rnint_490, bid128_to_int64_rnint, 0xB02E00000001A784379D99DB7D9ACA00u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_rnint_491, bid128_to_int64_rnint, 0xB02E00000001A784379D99DB7D9ACA01u128, -2000000000000001   , 0x00); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_rnint_492, bid128_to_int64_rnint, 0xB03000000000000000000006FC23ABFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_493, bid128_to_int64_rnint, 0xB03000000000000000000006FC23AC00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_494, bid128_to_int64_rnint, 0xB03000000000000000000006FC23AC01u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_495, bid128_to_int64_rnint, 0xB03200000000000000000000B2D05DFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_496, bid128_to_int64_rnint, 0xB03200000000000000000000B2D05E00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_497, bid128_to_int64_rnint, 0xB03200000000000000000000B2D05E01u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_498, bid128_to_int64_rnint, 0xB03800000000000000000000002DDA47u128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rnint_499, bid128_to_int64_rnint, 0xB03800000000000000000000002DDA48u128, -300                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rnint_500, bid128_to_int64_rnint, 0xB03800000000000000000000002DDA49u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rnint_501, bid128_to_int64_rnint, 0xB03A00000000000000000000000003E7u128, -1                  , 0x00); // -- -(0.999)
dec_test!(bid128_to_int64_rnint_502, bid128_to_int64_rnint, 0xB03A00000000000000000000000495D3u128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rnint_503, bid128_to_int64_rnint, 0xB03A00000000000000000000000495D4u128, -300                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rnint_504, bid128_to_int64_rnint, 0xB03A00000000000000000000000495D5u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rnint_505, bid128_to_int64_rnint, 0xB03C0000000000000000000000007561u128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rnint_506, bid128_to_int64_rnint, 0xB03C0000000000000000000000007562u128, -300                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rnint_507, bid128_to_int64_rnint, 0xB03C0000000000000000000000007563u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rnint_508, bid128_to_int64_rnint, 0xB03E0000000000000000000000000005u128, 0                   , 0x00); // -- -(0.5)
dec_test!(bid128_to_int64_rnint_509, bid128_to_int64_rnint, 0xB03E000000000000000000000000000Fu128, -2                  , 0x00); // -- -(1.5)
dec_test!(bid128_to_int64_rnint_510, bid128_to_int64_rnint, 0xB03E0000000000000000000000000BB7u128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_511, bid128_to_int64_rnint, 0xB03E0000000000000000000000000BB8u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_512, bid128_to_int64_rnint, 0xB03E0000000000000000000000000BB9u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_513, bid128_to_int64_rnint, 0xB03E0000000000000000000000000BBDu128, -300                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rnint_514, bid128_to_int64_rnint, 0xB03E0000000000000000002E90EDCFF1u128, -19999999998        , 0x00); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_rnint_515, bid128_to_int64_rnint, 0xB03E0000000000000000002E90EDCFFBu128, -20000000000        , 0x00); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_rnint_516, bid128_to_int64_rnint, 0xB03E0000000000000000002E90EDD005u128, -20000000000        , 0x00); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_rnint_517, bid128_to_int64_rnint, 0xB03E0000000000000000002E90EDD00Fu128, -20000000002        , 0x00); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_rnint_518, bid128_to_int64_rnint, 0xB03E0000000000000001400000000005u128, -35184372088832     , 0x00); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_rnint_519, bid128_to_int64_rnint, 0xB03E00000000000000470DE4DF81FFF1u128, -1999999999999998   , 0x00); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_rnint_520, bid128_to_int64_rnint, 0xB03E00000000000000470DE4DF81FFFBu128, -2000000000000000   , 0x00); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_rnint_521, bid128_to_int64_rnint, 0xB03E00000000000000470DE4DF820005u128, -2000000000000000   , 0x00); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_rnint_522, bid128_to_int64_rnint, 0xB03E00000000000000470DE4DF82000Fu128, -2000000000000002   , 0x00); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_rnint_523, bid128_to_int64_rnint, 0xB03E000000000004FFFFFFFFFFFFFFF1u128, -9223372036854775806, 0x00); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_rnint_524, bid128_to_int64_rnint, 0xB03E000000000004FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x00); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_rnint_525, bid128_to_int64_rnint, 0xB03E0000000000050000000000000005u128, -9223372036854775808, 0x00); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_rnint_526, bid128_to_int64_rnint, 0xB03E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_rnint_527, bid128_to_int64_rnint, 0xB03E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_rnint_528, bid128_to_int64_rnint, 0xB03E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_rnint_529, bid128_to_int64_rnint, 0xB0400000000000000000000000000001u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_rnint_530, bid128_to_int64_rnint, 0xB040000000000000000000000000012Bu128, -299                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_531, bid128_to_int64_rnint, 0xB040000000000000000000000000012Cu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_532, bid128_to_int64_rnint, 0xB040000000000000000000000000012Du128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_533, bid128_to_int64_rnint, 0xB04000000000000000000004A817C7FFu128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_rnint_534, bid128_to_int64_rnint, 0xB04000000000000000000004A817C801u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_rnint_535, bid128_to_int64_rnint, 0xB0400000000000000000200000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_rnint_536, bid128_to_int64_rnint, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_rnint_537, bid128_to_int64_rnint, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_rnint_538, bid128_to_int64_rnint, 0xB04000000000000000071AFD498D0000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_rnint_539, bid128_to_int64_rnint, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_rnint_540, bid128_to_int64_rnint, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_rnint_541, bid128_to_int64_rnint, 0xB0400000000000007FFFFFFFFFFFFFFFu128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_rnint_542, bid128_to_int64_rnint, 0xB0400000000000008000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_rnint_543, bid128_to_int64_rnint, 0xB0400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_rnint_544, bid128_to_int64_rnint, 0xB040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_rnint_545, bid128_to_int64_rnint, 0xB0400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_rnint_546, bid128_to_int64_rnint, 0xB0400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_rnint_547, bid128_to_int64_rnint, 0xB042000000000000000000000000001Du128, -290                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rnint_548, bid128_to_int64_rnint, 0xB042000000000000000000000000001Eu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_549, bid128_to_int64_rnint, 0xB042000000000000000000000000001Fu128, -310                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rnint_550, bid128_to_int64_rnint, 0xB04200000000000000000000773593FFu128, -19999999990        , 0x00); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_rnint_551, bid128_to_int64_rnint, 0xB0420000000000000000000077359400u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_rnint_552, bid128_to_int64_rnint, 0xB0420000000000000000000077359401u128, -20000000010        , 0x00); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_rnint_553, bid128_to_int64_rnint, 0xB0440000000000000000000000000003u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rnint_554, bid128_to_int64_rnint, 0xB0520000000000000000000000000004u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_rnint_555, bid128_to_int64_rnint, 0xB0520000000000000000000000000005u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_rnint_556, bid128_to_int64_rnint, 0xB0540000000000000000000000000002u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_rnint_557, bid128_to_int64_rnint, 0xB05E0000000000000000000000000002u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_rnint_558, bid128_to_int64_rnint, 0xB064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_rnint_559, bid128_to_int64_rnint, 0xB0640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_rnint_560, bid128_to_int64_rnint, 0xB0660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_rnint_561, bid128_to_int64_rnint, 0xB0660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_rnint_562, bid128_to_int64_rnint, 0xB0680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_rnint_563, bid128_to_int64_rnint, 0xc74ccd978165f2c27777b28ece449d3eu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_564, bid128_to_int64_rnint, 0xcfa2322b95767fe5a170edfcafc7f7e2u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_565, bid128_to_int64_rnint, 0xef30869f086e838e7616a72b8e385908u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rnint_566, bid128_to_int64_rnint, 0xfbffff7eefff7fde01284080e4621016u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_567, bid128_to_int64_rnint, 0xffffff6ffffffdff8b82e784551402b7u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_568, bid128_to_int64_rnint, "Infinity"                            , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rnint_569, bid128_to_int64_rnint, "SNaN"                                , -9223372036854775808, 0x01);
