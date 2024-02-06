/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int64_rninta_001, bid128_to_int64_rninta, "-0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_002, bid128_to_int64_rninta,  "0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_003, bid128_to_int64_rninta, 0x00000000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_004, bid128_to_int64_rninta, 0x00000000000000000000146802882012u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_005, bid128_to_int64_rninta, 0x0000000000000000bffffffffbfe7fffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_006, bid128_to_int64_rninta, 0x0000000040010000da2608680574088cu128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_007, bid128_to_int64_rninta, 0x0001ed09bead87c0378d8e62ffffffffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_008, bid128_to_int64_rninta, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_009, bid128_to_int64_rninta, "0.1"                                 , 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_010, bid128_to_int64_rninta, "0.5"                                 , 1                   , 0x00);
dec_test!(bid128_to_int64_rninta_011, bid128_to_int64_rninta, "1152921504606846976"                 , 1152921504606846976 , 0x00);
dec_test!(bid128_to_int64_rninta_012, bid128_to_int64_rninta, 0x19022808200008645100784068310120u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_013, bid128_to_int64_rninta, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_int64_rninta_014, bid128_to_int64_rninta, 0x2FFCF684DF56C3E01BC6C73200000000u128, 1                   , 0x00); // -- 0.5
dec_test!(bid128_to_int64_rninta_015, bid128_to_int64_rninta, 0x2FFCF684DF56C3E01BC6C73200000001u128, 1                   , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_int64_rninta_016, bid128_to_int64_rninta, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 1                   , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_int64_rninta_017, bid128_to_int64_rninta, 0x2FFDEC8B86EF679D76FC433D80000000u128, 1                   , 0x00); // -- 0.999
dec_test!(bid128_to_int64_rninta_018, bid128_to_int64_rninta, 0x2FFDEC8B86EF679D76FC433D80000001u128, 1                   , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_int64_rninta_019, bid128_to_int64_rninta, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 1                   , 0x00); // -- 1-ulp
dec_test!(bid128_to_int64_rninta_020, bid128_to_int64_rninta, 0x2FFE314DC6448D9338C15B0A00000000u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_rninta_021, bid128_to_int64_rninta, 0x2FFE314DC6448D9338C15B0A00000001u128, 1                   , 0x00); // -- 1+ulp
dec_test!(bid128_to_int64_rninta_022, bid128_to_int64_rninta, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1                   , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_int64_rninta_023, bid128_to_int64_rninta, 0x2FFE49F4A966D45CD522088F00000000u128, 2                   , 0x00); // -- 1.5
dec_test!(bid128_to_int64_rninta_024, bid128_to_int64_rninta, 0x2FFE49F4A966D45CD522088F00000001u128, 2                   , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_int64_rninta_025, bid128_to_int64_rninta, 0x3000040a809450460400d18400d00000u128, 1                   , 0x00);
dec_test!(bid128_to_int64_rninta_026, bid128_to_int64_rninta, 0x3002001002000004dd07f11639f7596bu128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_027, bid128_to_int64_rninta, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_028, bid128_to_int64_rninta, 0x300293E952CDA8B9AA44111E00000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_029, bid128_to_int64_rninta, 0x300293E952CDA8B9AA44111E00000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_030, bid128_to_int64_rninta, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rninta_031, bid128_to_int64_rninta, 0x300294286EACB8CB0A8CB6B140000000u128, 301                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rninta_032, bid128_to_int64_rninta, 0x300294286EACB8CB0A8CB6B140000001u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rninta_033, bid128_to_int64_rninta, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_034, bid128_to_int64_rninta, 0x30040ECA8847C4129106CE8300000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_035, bid128_to_int64_rninta, 0x30040ECA8847C4129106CE8300000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_036, bid128_to_int64_rninta, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_037, bid128_to_int64_rninta, 0x300A0003C95A2F0B4856475FE0000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_038, bid128_to_int64_rninta, 0x300A0003C95A2F0B4856475FE0000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_039, bid128_to_int64_rninta, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_040, bid128_to_int64_rninta, 0x300C000060EF6B1ABA6F072330000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_041, bid128_to_int64_rninta, 0x300C000060EF6B1ABA6F072330000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_042, bid128_to_int64_rninta, 0x3010C5371912364CE3056C27FFFFFFFFu128, 4000000000          , 0x00); // -- 4e9-ulp
dec_test!(bid128_to_int64_rninta_043, bid128_to_int64_rninta, 0x3010C5371912364CE3056C2800000000u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_rninta_044, bid128_to_int64_rninta, 0x3010C5371912364CE3056C2800000001u128, 4000000000          , 0x00); // -- 4e9+ulp
dec_test!(bid128_to_int64_rninta_045, bid128_to_int64_rninta, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 5000000000          , 0x00); // -- 5e9-ulp
dec_test!(bid128_to_int64_rninta_046, bid128_to_int64_rninta, 0x3010F684DF56C3E01BC6C73200000000u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_rninta_047, bid128_to_int64_rninta, 0x3010F684DF56C3E01BC6C73200000001u128, 5000000000          , 0x00); // -- 5e9+ulp
dec_test!(bid128_to_int64_rninta_048, bid128_to_int64_rninta, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 19999999998         , 0x00); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_rninta_049, bid128_to_int64_rninta, 0x3012629B8C88FB62ED56E4238E400000u128, 19999999999         , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_int64_rninta_050, bid128_to_int64_rninta, 0x3012629B8C88FB62ED56E4238E400001u128, 19999999999         , 0x00); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_rninta_051, bid128_to_int64_rninta, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 19999999999         , 0x00); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_rninta_052, bid128_to_int64_rninta, 0x3012629B8C8905F96EBAD4C909800000u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_rninta_053, bid128_to_int64_rninta, 0x3012629B8C8905F96EBAD4C909800001u128, 19999999999         , 0x00); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_rninta_054, bid128_to_int64_rninta, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 19999999999         , 0x00); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_rninta_055, bid128_to_int64_rninta, 0x3012629B8C89108FF01EC56E84C00000u128, 20000000000         , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_int64_rninta_056, bid128_to_int64_rninta, 0x3012629B8C89108FF01EC56E84C00001u128, 20000000000         , 0x00); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_rninta_057, bid128_to_int64_rninta, 0x3012629B8C891B267182B613FFFFFFFFu128, 20000000000         , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_int64_rninta_058, bid128_to_int64_rninta, 0x3012629B8C891B267182B61400000000u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_rninta_059, bid128_to_int64_rninta, 0x3012629B8C891B267182B61400000001u128, 20000000000         , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_int64_rninta_060, bid128_to_int64_rninta, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 20000000000         , 0x00); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_rninta_061, bid128_to_int64_rninta, 0x3012629B8C8925BCF2E6A6B97B400000u128, 20000000001         , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_int64_rninta_062, bid128_to_int64_rninta, 0x3012629B8C8925BCF2E6A6B97B400001u128, 20000000001         , 0x00); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_rninta_063, bid128_to_int64_rninta, 0x3012629B8C893053744A975EF67FFFFFu128, 20000000001         , 0x00); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_rninta_064, bid128_to_int64_rninta, 0x3012629B8C893053744A975EF6800000u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_rninta_065, bid128_to_int64_rninta, 0x3012629B8C893053744A975EF6800001u128, 20000000001         , 0x00); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_rninta_066, bid128_to_int64_rninta, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 20000000001         , 0x00); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_rninta_067, bid128_to_int64_rninta, 0x3012629B8C893AE9F5AE880471C00000u128, 20000000002         , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_int64_rninta_068, bid128_to_int64_rninta, 0x3012629B8C893AE9F5AE880471C00001u128, 20000000002         , 0x00); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_rninta_069, bid128_to_int64_rninta, 0x30162020004ea902db716b8df7bef3f6u128, 651572503612        , 0x00);
dec_test!(bid128_to_int64_rninta_070, bid128_to_int64_rninta, 0x3018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 35184372088832      , 0x00); // -- 2^45-ulp
dec_test!(bid128_to_int64_rninta_071, bid128_to_int64_rninta, 0x3018AD78EBC5AC620000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_rninta_072, bid128_to_int64_rninta, 0x3018AD78EBC5AC620000000000000001u128, 35184372088832      , 0x00); // -- 2^45+ulp
dec_test!(bid128_to_int64_rninta_073, bid128_to_int64_rninta, 0x3018AD78EBC5AC64B5E3AF16B187FFFFu128, 35184372088832      , 0x00); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_rninta_074, bid128_to_int64_rninta, 0x3018AD78EBC5AC64B5E3AF16B1880000u128, 35184372088833      , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_int64_rninta_075, bid128_to_int64_rninta, 0x3018AD78EBC5AC64B5E3AF16B1880001u128, 35184372088833      , 0x00); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_rninta_076, bid128_to_int64_rninta, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rninta_077, bid128_to_int64_rninta, 0x301A0000000000A2E6C09AD3E0D40000u128, 301                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rninta_078, bid128_to_int64_rninta, 0x301A0000000000A2E6C09AD3E0D40001u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rninta_079, bid128_to_int64_rninta, 0x301C629B8C891B265CB1A40684E9FFFFu128, 1999999999999998    , 0x00); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_rninta_080, bid128_to_int64_rninta, 0x301C629B8C891B265CB1A40684EA0000u128, 1999999999999999    , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_int64_rninta_081, bid128_to_int64_rninta, 0x301C629B8C891B265CB1A40684EA0001u128, 1999999999999999    , 0x00); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_rninta_082, bid128_to_int64_rninta, 0x301C629B8C891B2663A1FF60589BFFFFu128, 1999999999999999    , 0x00); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_rninta_083, bid128_to_int64_rninta, 0x301C629B8C891B2663A1FF60589C0000u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_rninta_084, bid128_to_int64_rninta, 0x301C629B8C891B2663A1FF60589C0001u128, 1999999999999999    , 0x00); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_rninta_085, bid128_to_int64_rninta, 0x301C629B8C891B266A925ABA2C4DFFFFu128, 1999999999999999    , 0x00); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_rninta_086, bid128_to_int64_rninta, 0x301C629B8C891B266A925ABA2C4E0000u128, 2000000000000000    , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_int64_rninta_087, bid128_to_int64_rninta, 0x301C629B8C891B266A925ABA2C4E0001u128, 2000000000000000    , 0x00); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_rninta_088, bid128_to_int64_rninta, 0x301C629B8C891B267182B613FFFFFFFFu128, 2000000000000000    , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_int64_rninta_089, bid128_to_int64_rninta, 0x301C629B8C891B267182B61400000000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_rninta_090, bid128_to_int64_rninta, 0x301C629B8C891B267182B61400000001u128, 2000000000000000    , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_int64_rninta_091, bid128_to_int64_rninta, 0x301C629B8C891B267873116DD3B1FFFFu128, 2000000000000000    , 0x00); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_rninta_092, bid128_to_int64_rninta, 0x301C629B8C891B267873116DD3B20000u128, 2000000000000001    , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_int64_rninta_093, bid128_to_int64_rninta, 0x301C629B8C891B267873116DD3B20001u128, 2000000000000001    , 0x00); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_rninta_094, bid128_to_int64_rninta, 0x301C629B8C891B267F636CC7A763FFFFu128, 2000000000000001    , 0x00); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_rninta_095, bid128_to_int64_rninta, 0x301C629B8C891B267F636CC7A7640000u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_rninta_096, bid128_to_int64_rninta, 0x301C629B8C891B267F636CC7A7640001u128, 2000000000000001    , 0x00); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_rninta_097, bid128_to_int64_rninta, 0x301C629B8C891B268653C8217B15FFFFu128, 2000000000000001    , 0x00); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_rninta_098, bid128_to_int64_rninta, 0x301C629B8C891B268653C8217B160000u128, 2000000000000002    , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_int64_rninta_099, bid128_to_int64_rninta, 0x301C629B8C891B268653C8217B160001u128, 2000000000000002    , 0x00); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_rninta_100, bid128_to_int64_rninta, 0x301E000000000001A055690D9DB7FFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_101, bid128_to_int64_rninta, 0x301E000000000001A055690D9DB80000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_102, bid128_to_int64_rninta, 0x301E000000000001A055690D9DB80001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_103, bid128_to_int64_rninta, 0x301E002C68AF0BB140B1A2BC2EC4FFFFu128, 35184372088832      , 0x00); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_rninta_104, bid128_to_int64_rninta, 0x301E002C68AF0BB140B1A2BC2EC50000u128, 35184372088833      , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_int64_rninta_105, bid128_to_int64_rninta, 0x301E002C68AF0BB140B1A2BC2EC50001u128, 35184372088833      , 0x00); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_rninta_106, bid128_to_int64_rninta, 0x302000000000000029A2241AF62BFFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_107, bid128_to_int64_rninta, 0x302000000000000029A2241AF62C0000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_108, bid128_to_int64_rninta, 0x302000000000000029A2241AF62C0001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_109, bid128_to_int64_rninta, 0x3020000470DE4DF81FFFFFFFFFFFFFFFu128, 35184372088832      , 0x00); // -- 2^45-ulp
dec_test!(bid128_to_int64_rninta_110, bid128_to_int64_rninta, 0x3020000470DE4DF82000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_rninta_111, bid128_to_int64_rninta, 0x3020000470DE4DF82000000000000001u128, 35184372088832      , 0x00); // -- 2^45+ulp
dec_test!(bid128_to_int64_rninta_112, bid128_to_int64_rninta, 0x302000FC6F7C4045813459C637E07FFFu128, 2000000000000000    , 0x00); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_rninta_113, bid128_to_int64_rninta, 0x302000FC6F7C4045813459C637E08000u128, 2000000000000001    , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_int64_rninta_114, bid128_to_int64_rninta, 0x302000FC6F7C4045813459C637E08001u128, 2000000000000001    , 0x00); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_rninta_115, bid128_to_int64_rninta, 0x302000FC6F7C40458157E0B8A7A17FFFu128, 2000000000000001    , 0x00); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_rninta_116, bid128_to_int64_rninta, 0x302000FC6F7C40458157E0B8A7A18000u128, 2000000000000002    , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_int64_rninta_117, bid128_to_int64_rninta, 0x302000FC6F7C40458157E0B8A7A18001u128, 2000000000000002    , 0x00); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_rninta_118, bid128_to_int64_rninta, 0x302200193E5939A08CE4879688D63FFFu128, 1999999999999998    , 0x00); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_rninta_119, bid128_to_int64_rninta, 0x302200193E5939A08CE4879688D64000u128, 1999999999999999    , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_int64_rninta_120, bid128_to_int64_rninta, 0x302200193E5939A08CE4879688D64001u128, 1999999999999999    , 0x00); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_rninta_121, bid128_to_int64_rninta, 0x302200193E5939A08CE815152D9CBFFFu128, 1999999999999999    , 0x00); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_rninta_122, bid128_to_int64_rninta, 0x302200193E5939A08CE815152D9CC000u128, 2000000000000000    , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_int64_rninta_123, bid128_to_int64_rninta, 0x302200193E5939A08CE815152D9CC001u128, 2000000000000000    , 0x00); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_rninta_124, bid128_to_int64_rninta, 0x3023818c995a18c00800000000000400u128, 7819867100284595699 , 0x00);
dec_test!(bid128_to_int64_rninta_125, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775806 , 0x00); // -- 2^63-1.5-ulp
dec_test!(bid128_to_int64_rninta_126, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775807 , 0x00); // -- 2^63-1.5
dec_test!(bid128_to_int64_rninta_127, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775807 , 0x00); // -- 2^63-1.5+ulp
dec_test!(bid128_to_int64_rninta_128, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775807 , 0x00); // -- 2^63-1-ulp
dec_test!(bid128_to_int64_rninta_129, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFC72815B398000u128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_rninta_130, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFC72815B398001u128, 9223372036854775807 , 0x00); // -- 2^63-1+ulp
dec_test!(bid128_to_int64_rninta_131, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775807 , 0x00); // -- 2^63-0.5-ulp
dec_test!(bid128_to_int64_rninta_132, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFE3940AD9CC000u128, -9223372036854775808, 0x01); // -- 2^63-0.5
dec_test!(bid128_to_int64_rninta_133, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFE3940AD9CC001u128, -9223372036854775808, 0x01); // -- 2^63-0.5+ulp
dec_test!(bid128_to_int64_rninta_134, bid128_to_int64_rninta, 0x3023C6BF52633FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^63-ulp
dec_test!(bid128_to_int64_rninta_135, bid128_to_int64_rninta, 0x3023C6BF526340000000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_rninta_136, bid128_to_int64_rninta, 0x3023C6BF526340000000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+ulp
dec_test!(bid128_to_int64_rninta_137, bid128_to_int64_rninta, 0x3023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x01); // -- 2^63+0.5-ulp
dec_test!(bid128_to_int64_rninta_138, bid128_to_int64_rninta, 0x3023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_rninta_139, bid128_to_int64_rninta, 0x3023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- 2^63+0.5+ulp
dec_test!(bid128_to_int64_rninta_140, bid128_to_int64_rninta, 0x3023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- 2^63+1-ulp
dec_test!(bid128_to_int64_rninta_141, bid128_to_int64_rninta, 0x3023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_rninta_142, bid128_to_int64_rninta, 0x3023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- 2^63+1+ulp
dec_test!(bid128_to_int64_rninta_143, bid128_to_int64_rninta, 0x3024000000000000006A94D74F42FFFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_144, bid128_to_int64_rninta, 0x3024000000000000006A94D74F430000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_145, bid128_to_int64_rninta, 0x3024000000000000006A94D74F430001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_146, bid128_to_int64_rninta, 0x30242e6403d866e29415341a986ea96bu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_147, bid128_to_int64_rninta, 0x3024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e19-ulp
dec_test!(bid128_to_int64_rninta_148, bid128_to_int64_rninta, 0x3024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_rninta_149, bid128_to_int64_rninta, 0x3024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e19+ulp
dec_test!(bid128_to_int64_rninta_150, bid128_to_int64_rninta, 0x3024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- 1e19+0.5-ulp
dec_test!(bid128_to_int64_rninta_151, bid128_to_int64_rninta, 0x3024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_rninta_152, bid128_to_int64_rninta, 0x3024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- 1e19+0.5+ulp
dec_test!(bid128_to_int64_rninta_153, bid128_to_int64_rninta, 0x302449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- 1.5e19-ulp
dec_test!(bid128_to_int64_rninta_154, bid128_to_int64_rninta, 0x302449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_rninta_155, bid128_to_int64_rninta, 0x302449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- 1.5e19+ulp
dec_test!(bid128_to_int64_rninta_156, bid128_to_int64_rninta, 0x30245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- 2^64-1-ulp
dec_test!(bid128_to_int64_rninta_157, bid128_to_int64_rninta, 0x30245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_rninta_158, bid128_to_int64_rninta, 0x30245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- 2^64-1+ulp
dec_test!(bid128_to_int64_rninta_159, bid128_to_int64_rninta, 0x30245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- 2^64-0.5-ulp
dec_test!(bid128_to_int64_rninta_160, bid128_to_int64_rninta, 0x30245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_rninta_161, bid128_to_int64_rninta, 0x30245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- 2^64-0.5+ulp
dec_test!(bid128_to_int64_rninta_162, bid128_to_int64_rninta, 0x30245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-ulp
dec_test!(bid128_to_int64_rninta_163, bid128_to_int64_rninta, 0x30245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_rninta_164, bid128_to_int64_rninta, 0x30245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+ulp
dec_test!(bid128_to_int64_rninta_165, bid128_to_int64_rninta, 0x30245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- 2^64+0.5-ulp
dec_test!(bid128_to_int64_rninta_166, bid128_to_int64_rninta, 0x30245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_rninta_167, bid128_to_int64_rninta, 0x30245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- 2^64+0.5+ulp
dec_test!(bid128_to_int64_rninta_168, bid128_to_int64_rninta, 0x30245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- 2^64+1-ulp
dec_test!(bid128_to_int64_rninta_169, bid128_to_int64_rninta, 0x30245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_rninta_170, bid128_to_int64_rninta, 0x30245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- 2^64+1+ulp
dec_test!(bid128_to_int64_rninta_171, bid128_to_int64_rninta, 0x3024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2e19-ulp
dec_test!(bid128_to_int64_rninta_172, bid128_to_int64_rninta, 0x3024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_rninta_173, bid128_to_int64_rninta, 0x3024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- 2e19+ulp
dec_test!(bid128_to_int64_rninta_174, bid128_to_int64_rninta, 0x30247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2.5e19-ulp
dec_test!(bid128_to_int64_rninta_175, bid128_to_int64_rninta, 0x30247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_rninta_176, bid128_to_int64_rninta, 0x30247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- 2.5e19+ulp
dec_test!(bid128_to_int64_rninta_177, bid128_to_int64_rninta, 0x3026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e20-ulp
dec_test!(bid128_to_int64_rninta_178, bid128_to_int64_rninta, 0x3026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_rninta_179, bid128_to_int64_rninta, 0x3026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e20+ulp
dec_test!(bid128_to_int64_rninta_180, bid128_to_int64_rninta, 0x302A00000000006C6B935B68D08DA3FFu128, 19999999998         , 0x00); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_rninta_181, bid128_to_int64_rninta, 0x302A00000000006C6B935B68D08DA400u128, 19999999999         , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_int64_rninta_182, bid128_to_int64_rninta, 0x302A00000000006C6B935B68D08DA401u128, 19999999999         , 0x00); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_rninta_183, bid128_to_int64_rninta, 0x302A00000000006C6B935B8019048BFFu128, 19999999999         , 0x00); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_rninta_184, bid128_to_int64_rninta, 0x302A00000000006C6B935B8019048C00u128, 20000000000         , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_int64_rninta_185, bid128_to_int64_rninta, 0x302A00000000006C6B935B8019048C01u128, 20000000000         , 0x00); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_rninta_186, bid128_to_int64_rninta, 0x302C000000000000000002BBA7F521FFu128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rninta_187, bid128_to_int64_rninta, 0x302C000000000000000002BBA7F52200u128, 301                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rninta_188, bid128_to_int64_rninta, 0x302C000000000000000002BBA7F52201u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rninta_189, bid128_to_int64_rninta, 0x302C00000000000AD78EBC5872141BFFu128, 19999999999         , 0x00); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_rninta_190, bid128_to_int64_rninta, 0x302C00000000000AD78EBC5872141C00u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_rninta_191, bid128_to_int64_rninta, 0x302C00000000000AD78EBC5872141C01u128, 19999999999         , 0x00); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_rninta_192, bid128_to_int64_rninta, 0x302C00000000000AD78EBC5BF025F1FFu128, 20000000000         , 0x00); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_rninta_193, bid128_to_int64_rninta, 0x302C00000000000AD78EBC5BF025F200u128, 20000000001         , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_int64_rninta_194, bid128_to_int64_rninta, 0x302C00000000000AD78EBC5BF025F201u128, 20000000001         , 0x00); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_rninta_195, bid128_to_int64_rninta, 0x302C00000000000AD78EBC5E4431D5FFu128, 20000000001         , 0x00); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_rninta_196, bid128_to_int64_rninta, 0x302C00000000000AD78EBC5E4431D600u128, 20000000002         , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_int64_rninta_197, bid128_to_int64_rninta, 0x302C00000000000AD78EBC5E4431D601u128, 20000000002         , 0x00); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_rninta_198, bid128_to_int64_rninta, 0x302C000000108B2A2C28028E3FF41BFFu128, 1999999999999999    , 0x00); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_rninta_199, bid128_to_int64_rninta, 0x302C000000108B2A2C28028E3FF41C00u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_rninta_200, bid128_to_int64_rninta, 0x302C000000108B2A2C28028E3FF41C01u128, 1999999999999999    , 0x00); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_rninta_201, bid128_to_int64_rninta, 0x302E000000000001158E46094F6AC9FFu128, 20000000001         , 0x00); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_rninta_202, bid128_to_int64_rninta, 0x302E000000000001158E46094F6ACA00u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_rninta_203, bid128_to_int64_rninta, 0x302E000000000001158E46094F6ACA01u128, 20000000001         , 0x00); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_rninta_204, bid128_to_int64_rninta, 0x302E00000001A784379D99DB7D9AC9FFu128, 2000000000000001    , 0x00); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_rninta_205, bid128_to_int64_rninta, 0x302E00000001A784379D99DB7D9ACA00u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_rninta_206, bid128_to_int64_rninta, 0x302E00000001A784379D99DB7D9ACA01u128, 2000000000000001    , 0x00); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_rninta_207, bid128_to_int64_rninta, 0x303000000000000000000006FC23ABFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_208, bid128_to_int64_rninta, 0x303000000000000000000006FC23AC00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_209, bid128_to_int64_rninta, 0x303000000000000000000006FC23AC01u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_210, bid128_to_int64_rninta, 0x303200000000000000000000B2D05DFFu128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_211, bid128_to_int64_rninta, 0x303200000000000000000000B2D05E00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_212, bid128_to_int64_rninta, 0x303200000000000000000000B2D05E01u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_213, bid128_to_int64_rninta, 0x303800000000000000000000002DDA47u128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rninta_214, bid128_to_int64_rninta, 0x303800000000000000000000002DDA48u128, 301                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rninta_215, bid128_to_int64_rninta, 0x303800000000000000000000002DDA49u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rninta_216, bid128_to_int64_rninta, 0x303A00000000000000000000000003E7u128, 1                   , 0x00); // -- 0.999
dec_test!(bid128_to_int64_rninta_217, bid128_to_int64_rninta, 0x303A00000000000000000000000495D3u128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rninta_218, bid128_to_int64_rninta, 0x303A00000000000000000000000495D4u128, 301                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rninta_219, bid128_to_int64_rninta, 0x303A00000000000000000000000495D5u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rninta_220, bid128_to_int64_rninta, 0x303C0000000000000000000000007561u128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_rninta_221, bid128_to_int64_rninta, 0x303C0000000000000000000000007562u128, 301                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rninta_222, bid128_to_int64_rninta, 0x303C0000000000000000000000007563u128, 301                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_rninta_223, bid128_to_int64_rninta, 0x303E0000000000000000000000000005u128, 1                   , 0x00); // -- 0.5
dec_test!(bid128_to_int64_rninta_224, bid128_to_int64_rninta, 0x303E000000000000000000000000000Fu128, 2                   , 0x00); // -- 1.5
dec_test!(bid128_to_int64_rninta_225, bid128_to_int64_rninta, 0x303E0000000000000000000000000BB7u128, 300                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_226, bid128_to_int64_rninta, 0x303E0000000000000000000000000BB8u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_227, bid128_to_int64_rninta, 0x303E0000000000000000000000000BB9u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_228, bid128_to_int64_rninta, 0x303E0000000000000000000000000BBDu128, 301                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_rninta_229, bid128_to_int64_rninta, 0x303E0000000000000000002E90EDCFF1u128, 19999999999         , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_int64_rninta_230, bid128_to_int64_rninta, 0x303E0000000000000000002E90EDCFFBu128, 20000000000         , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_int64_rninta_231, bid128_to_int64_rninta, 0x303E0000000000000000002E90EDD005u128, 20000000001         , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_int64_rninta_232, bid128_to_int64_rninta, 0x303E0000000000000000002E90EDD00Fu128, 20000000002         , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_int64_rninta_233, bid128_to_int64_rninta, 0x303E0000000000000001400000000005u128, 35184372088833      , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_int64_rninta_234, bid128_to_int64_rninta, 0x303E00000000000000470DE4DF81FFF1u128, 1999999999999999    , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_int64_rninta_235, bid128_to_int64_rninta, 0x303E00000000000000470DE4DF81FFFBu128, 2000000000000000    , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_int64_rninta_236, bid128_to_int64_rninta, 0x303E00000000000000470DE4DF820005u128, 2000000000000001    , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_int64_rninta_237, bid128_to_int64_rninta, 0x303E00000000000000470DE4DF82000Fu128, 2000000000000002    , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_int64_rninta_238, bid128_to_int64_rninta, 0x303E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775807 , 0x00); // -- 2^63-1.5
dec_test!(bid128_to_int64_rninta_239, bid128_to_int64_rninta, 0x303E000000000004FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- 2^63-0.5
dec_test!(bid128_to_int64_rninta_240, bid128_to_int64_rninta, 0x303E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_rninta_241, bid128_to_int64_rninta, 0x303E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_rninta_242, bid128_to_int64_rninta, 0x303E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_rninta_243, bid128_to_int64_rninta, 0x303E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_rninta_244, bid128_to_int64_rninta, 0x30400000000000000000000000000001u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_rninta_245, bid128_to_int64_rninta, 0x3040000000000000000000000000012Bu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_246, bid128_to_int64_rninta, 0x3040000000000000000000000000012Cu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_247, bid128_to_int64_rninta, 0x3040000000000000000000000000012Du128, 301                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_248, bid128_to_int64_rninta, 0x30400000000000000000000200000000u128, 8589934592          , 0x00);
dec_test!(bid128_to_int64_rninta_249, bid128_to_int64_rninta, 0x304000000000000000000004A817C7FFu128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_rninta_250, bid128_to_int64_rninta, 0x304000000000000000000004A817C801u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_rninta_251, bid128_to_int64_rninta, 0x30400000000000000000200000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_rninta_252, bid128_to_int64_rninta, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_rninta_253, bid128_to_int64_rninta, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_int64_rninta_254, bid128_to_int64_rninta, 0x304000000000000000071AFD498D0000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_rninta_255, bid128_to_int64_rninta, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_rninta_256, bid128_to_int64_rninta, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_int64_rninta_257, bid128_to_int64_rninta, 0x30400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_rninta_258, bid128_to_int64_rninta, 0x30400000000000008000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_rninta_259, bid128_to_int64_rninta, 0x30400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_rninta_260, bid128_to_int64_rninta, 0x3040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_rninta_261, bid128_to_int64_rninta, 0x30400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_rninta_262, bid128_to_int64_rninta, 0x30400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_rninta_263, bid128_to_int64_rninta, 0x3041ED09BEAD87C0378D8E6400000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_264, bid128_to_int64_rninta, 0x3042000000000000000000000000001Du128, 290                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_rninta_265, bid128_to_int64_rninta, 0x3042000000000000000000000000001Eu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_266, bid128_to_int64_rninta, 0x3042000000000000000000000000001Fu128, 310                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_rninta_267, bid128_to_int64_rninta, 0x304200000000000000000000773593FFu128, 19999999990         , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_int64_rninta_268, bid128_to_int64_rninta, 0x30420000000000000000000077359400u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_rninta_269, bid128_to_int64_rninta, 0x30420000000000000000000077359401u128, 20000000010         , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_int64_rninta_270, bid128_to_int64_rninta, 0x30440000000000000000000000000003u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_rninta_271, bid128_to_int64_rninta, 0x30520000000000000000000000000004u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_rninta_272, bid128_to_int64_rninta, 0x30520000000000000000000000000005u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_rninta_273, bid128_to_int64_rninta, 0x30540000000000000000000000000002u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_rninta_274, bid128_to_int64_rninta, 0x305E0000000000000000000000000002u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_rninta_275, bid128_to_int64_rninta, 0x3064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_rninta_276, bid128_to_int64_rninta, 0x30640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_rninta_277, bid128_to_int64_rninta, 0x30660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_rninta_278, bid128_to_int64_rninta, 0x30660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_rninta_279, bid128_to_int64_rninta, 0x30680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_rninta_280, bid128_to_int64_rninta, 0x3b3973298eaf846a91a7e6ba2090264au128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_281, bid128_to_int64_rninta, 0x4585216f0958d51080c804232410b8c0u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_282, bid128_to_int64_rninta, 0x50059cf74c04f6262215f71ae83e5868u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_283, bid128_to_int64_rninta, 0x55e732345ce43363e9d6daa3869460aau128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_284, bid128_to_int64_rninta, 0x78000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_285, bid128_to_int64_rninta, 0x7b89080098369ed0693fbef9ff36bbffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_286, bid128_to_int64_rninta, 0x7c000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_287, bid128_to_int64_rninta, 0x7c003fffffffffff38c15b08ffffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_288, bid128_to_int64_rninta, 0x7c003fffffffffff38c15b0affffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_289, bid128_to_int64_rninta, 0x7d0e712b68b25e1b8933e228b54b2e08u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_290, bid128_to_int64_rninta, 0x7e000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_291, bid128_to_int64_rninta, 0x7e3eb56bebfdfffd5799842a17181b47u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_292, bid128_to_int64_rninta, "-9"                                  , -9                  , 0x00);
dec_test!(bid128_to_int64_rninta_293, bid128_to_int64_rninta, 0xaf248e8bc039c9417c99950ac6726ed8u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_294, bid128_to_int64_rninta, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_int64_rninta_295, bid128_to_int64_rninta, 0xAFFCF684DF56C3E01BC6C73200000000u128, -1                  , 0x00); // -- -(0.5)
dec_test!(bid128_to_int64_rninta_296, bid128_to_int64_rninta, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1                  , 0x00); // -- -(0.5+ulp)
dec_test!(bid128_to_int64_rninta_297, bid128_to_int64_rninta, 0xaffd7e2fe5dd75ffefbeef1f4c7e6bb7u128, -1                  , 0x00);
dec_test!(bid128_to_int64_rninta_298, bid128_to_int64_rninta, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1                  , 0x00); // -- -(0.999-ulp)
dec_test!(bid128_to_int64_rninta_299, bid128_to_int64_rninta, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1                  , 0x00); // -- -(0.999)
dec_test!(bid128_to_int64_rninta_300, bid128_to_int64_rninta, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1                  , 0x00); // -- -(0.999+ulp)
dec_test!(bid128_to_int64_rninta_301, bid128_to_int64_rninta, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1                  , 0x00); // -- -(1-ulp)
dec_test!(bid128_to_int64_rninta_302, bid128_to_int64_rninta, 0xAFFE314DC6448D9338C15B0A00000000u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_rninta_303, bid128_to_int64_rninta, 0xAFFE314DC6448D9338C15B0A00000001u128, -1                  , 0x00); // -- -(1+ulp)
dec_test!(bid128_to_int64_rninta_304, bid128_to_int64_rninta, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1                  , 0x00); // -- -(1.5-ulp)
dec_test!(bid128_to_int64_rninta_305, bid128_to_int64_rninta, 0xAFFE49F4A966D45CD522088F00000000u128, -2                  , 0x00); // -- -(1.5)
dec_test!(bid128_to_int64_rninta_306, bid128_to_int64_rninta, 0xAFFE49F4A966D45CD522088F00000001u128, -2                  , 0x00); // -- -(1.5+ulp)
dec_test!(bid128_to_int64_rninta_307, bid128_to_int64_rninta, 0xb0001a0c205ca026e65cd8d7119849ffu128, -5                  , 0x00);
dec_test!(bid128_to_int64_rninta_308, bid128_to_int64_rninta, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_309, bid128_to_int64_rninta, 0xB00293E952CDA8B9AA44111E00000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_310, bid128_to_int64_rninta, 0xB00293E952CDA8B9AA44111E00000001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_311, bid128_to_int64_rninta, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rninta_312, bid128_to_int64_rninta, 0xB00294286EACB8CB0A8CB6B140000000u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rninta_313, bid128_to_int64_rninta, 0xB00294286EACB8CB0A8CB6B140000001u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rninta_314, bid128_to_int64_rninta, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_315, bid128_to_int64_rninta, 0xB0040ECA8847C4129106CE8300000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_316, bid128_to_int64_rninta, 0xB0040ECA8847C4129106CE8300000001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_317, bid128_to_int64_rninta, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_318, bid128_to_int64_rninta, 0xB00A0003C95A2F0B4856475FE0000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_319, bid128_to_int64_rninta, 0xB00A0003C95A2F0B4856475FE0000001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_320, bid128_to_int64_rninta, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_321, bid128_to_int64_rninta, 0xB00C000060EF6B1ABA6F072330000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_322, bid128_to_int64_rninta, 0xB00C000060EF6B1ABA6F072330000001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_323, bid128_to_int64_rninta, 0xB010C5371912364CE3056C27FFFFFFFFu128, -4000000000         , 0x00); // -- -(4e9-ulp)
dec_test!(bid128_to_int64_rninta_324, bid128_to_int64_rninta, 0xB010C5371912364CE3056C2800000000u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_rninta_325, bid128_to_int64_rninta, 0xB010C5371912364CE3056C2800000001u128, -4000000000         , 0x00); // -- -(4e9+ulp)
dec_test!(bid128_to_int64_rninta_326, bid128_to_int64_rninta, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -5000000000         , 0x00); // -- -(5e9-ulp)
dec_test!(bid128_to_int64_rninta_327, bid128_to_int64_rninta, 0xB010F684DF56C3E01BC6C73200000000u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_rninta_328, bid128_to_int64_rninta, 0xB010F684DF56C3E01BC6C73200000001u128, -5000000000         , 0x00); // -- -(5e9+ulp)
dec_test!(bid128_to_int64_rninta_329, bid128_to_int64_rninta, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -19999999998        , 0x00); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_rninta_330, bid128_to_int64_rninta, 0xB012629B8C88FB62ED56E4238E400000u128, -19999999999        , 0x00); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_rninta_331, bid128_to_int64_rninta, 0xB012629B8C88FB62ED56E4238E400001u128, -19999999999        , 0x00); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_rninta_332, bid128_to_int64_rninta, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -19999999999        , 0x00); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_rninta_333, bid128_to_int64_rninta, 0xB012629B8C8905F96EBAD4C909800000u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_rninta_334, bid128_to_int64_rninta, 0xB012629B8C8905F96EBAD4C909800001u128, -19999999999        , 0x00); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_rninta_335, bid128_to_int64_rninta, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -19999999999        , 0x00); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_rninta_336, bid128_to_int64_rninta, 0xB012629B8C89108FF01EC56E84C00000u128, -20000000000        , 0x00); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_rninta_337, bid128_to_int64_rninta, 0xB012629B8C89108FF01EC56E84C00001u128, -20000000000        , 0x00); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_rninta_338, bid128_to_int64_rninta, 0xB012629B8C891B267182B613FFFFFFFFu128, -20000000000        , 0x00); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_rninta_339, bid128_to_int64_rninta, 0xB012629B8C891B267182B61400000000u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_rninta_340, bid128_to_int64_rninta, 0xB012629B8C891B267182B61400000001u128, -20000000000        , 0x00); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_rninta_341, bid128_to_int64_rninta, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -20000000000        , 0x00); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_rninta_342, bid128_to_int64_rninta, 0xB012629B8C8925BCF2E6A6B97B400000u128, -20000000001        , 0x00); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_rninta_343, bid128_to_int64_rninta, 0xB012629B8C8925BCF2E6A6B97B400001u128, -20000000001        , 0x00); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_rninta_344, bid128_to_int64_rninta, 0xB012629B8C893053744A975EF67FFFFFu128, -20000000001        , 0x00); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_rninta_345, bid128_to_int64_rninta, 0xB012629B8C893053744A975EF6800000u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_rninta_346, bid128_to_int64_rninta, 0xB012629B8C893053744A975EF6800001u128, -20000000001        , 0x00); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_rninta_347, bid128_to_int64_rninta, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -20000000001        , 0x00); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_rninta_348, bid128_to_int64_rninta, 0xB012629B8C893AE9F5AE880471C00000u128, -20000000002        , 0x00); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_rninta_349, bid128_to_int64_rninta, 0xB012629B8C893AE9F5AE880471C00001u128, -20000000002        , 0x00); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_rninta_350, bid128_to_int64_rninta, 0xB018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, -35184372088832     , 0x00); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_rninta_351, bid128_to_int64_rninta, 0xB018AD78EBC5AC620000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_rninta_352, bid128_to_int64_rninta, 0xB018AD78EBC5AC620000000000000001u128, -35184372088832     , 0x00); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_rninta_353, bid128_to_int64_rninta, 0xB018AD78EBC5AC64B5E3AF16B187FFFFu128, -35184372088832     , 0x00); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_rninta_354, bid128_to_int64_rninta, 0xB018AD78EBC5AC64B5E3AF16B1880000u128, -35184372088833     , 0x00); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_rninta_355, bid128_to_int64_rninta, 0xB018AD78EBC5AC64B5E3AF16B1880001u128, -35184372088833     , 0x00); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_rninta_356, bid128_to_int64_rninta, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rninta_357, bid128_to_int64_rninta, 0xB01A0000000000A2E6C09AD3E0D40000u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rninta_358, bid128_to_int64_rninta, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rninta_359, bid128_to_int64_rninta, 0xB01C629B8C891B265CB1A40684E9FFFFu128, -1999999999999998   , 0x00); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_rninta_360, bid128_to_int64_rninta, 0xB01C629B8C891B265CB1A40684EA0000u128, -1999999999999999   , 0x00); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_rninta_361, bid128_to_int64_rninta, 0xB01C629B8C891B265CB1A40684EA0001u128, -1999999999999999   , 0x00); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_rninta_362, bid128_to_int64_rninta, 0xB01C629B8C891B2663A1FF60589BFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_rninta_363, bid128_to_int64_rninta, 0xB01C629B8C891B2663A1FF60589C0000u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_rninta_364, bid128_to_int64_rninta, 0xB01C629B8C891B2663A1FF60589C0001u128, -1999999999999999   , 0x00); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_rninta_365, bid128_to_int64_rninta, 0xB01C629B8C891B266A925ABA2C4DFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_rninta_366, bid128_to_int64_rninta, 0xB01C629B8C891B266A925ABA2C4E0000u128, -2000000000000000   , 0x00); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_rninta_367, bid128_to_int64_rninta, 0xB01C629B8C891B266A925ABA2C4E0001u128, -2000000000000000   , 0x00); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_rninta_368, bid128_to_int64_rninta, 0xB01C629B8C891B267182B613FFFFFFFFu128, -2000000000000000   , 0x00); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_rninta_369, bid128_to_int64_rninta, 0xB01C629B8C891B267182B61400000000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_rninta_370, bid128_to_int64_rninta, 0xB01C629B8C891B267182B61400000001u128, -2000000000000000   , 0x00); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_rninta_371, bid128_to_int64_rninta, 0xB01C629B8C891B267873116DD3B1FFFFu128, -2000000000000000   , 0x00); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_rninta_372, bid128_to_int64_rninta, 0xB01C629B8C891B267873116DD3B20000u128, -2000000000000001   , 0x00); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_rninta_373, bid128_to_int64_rninta, 0xB01C629B8C891B267873116DD3B20001u128, -2000000000000001   , 0x00); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_rninta_374, bid128_to_int64_rninta, 0xB01C629B8C891B267F636CC7A763FFFFu128, -2000000000000001   , 0x00); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_rninta_375, bid128_to_int64_rninta, 0xB01C629B8C891B267F636CC7A7640000u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_rninta_376, bid128_to_int64_rninta, 0xB01C629B8C891B267F636CC7A7640001u128, -2000000000000001   , 0x00); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_rninta_377, bid128_to_int64_rninta, 0xB01C629B8C891B268653C8217B15FFFFu128, -2000000000000001   , 0x00); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_rninta_378, bid128_to_int64_rninta, 0xB01C629B8C891B268653C8217B160000u128, -2000000000000002   , 0x00); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_rninta_379, bid128_to_int64_rninta, 0xB01C629B8C891B268653C8217B160001u128, -2000000000000002   , 0x00); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_rninta_380, bid128_to_int64_rninta, 0xB01E000000000001A055690D9DB7FFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_381, bid128_to_int64_rninta, 0xB01E000000000001A055690D9DB80000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_382, bid128_to_int64_rninta, 0xB01E000000000001A055690D9DB80001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_383, bid128_to_int64_rninta, 0xB01E002C68AF0BB140B1A2BC2EC4FFFFu128, -35184372088832     , 0x00); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_rninta_384, bid128_to_int64_rninta, 0xB01E002C68AF0BB140B1A2BC2EC50000u128, -35184372088833     , 0x00); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_rninta_385, bid128_to_int64_rninta, 0xB01E002C68AF0BB140B1A2BC2EC50001u128, -35184372088833     , 0x00); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_rninta_386, bid128_to_int64_rninta, 0xB02000000000000029A2241AF62BFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_387, bid128_to_int64_rninta, 0xB02000000000000029A2241AF62C0000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_388, bid128_to_int64_rninta, 0xB02000000000000029A2241AF62C0001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_389, bid128_to_int64_rninta, 0xB020000470DE4DF81FFFFFFFFFFFFFFFu128, -35184372088832     , 0x00); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_rninta_390, bid128_to_int64_rninta, 0xB020000470DE4DF82000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_rninta_391, bid128_to_int64_rninta, 0xB020000470DE4DF82000000000000001u128, -35184372088832     , 0x00); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_rninta_392, bid128_to_int64_rninta, 0xB02000FC6F7C4045813459C637E07FFFu128, -2000000000000000   , 0x00); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_rninta_393, bid128_to_int64_rninta, 0xB02000FC6F7C4045813459C637E08000u128, -2000000000000001   , 0x00); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_rninta_394, bid128_to_int64_rninta, 0xB02000FC6F7C4045813459C637E08001u128, -2000000000000001   , 0x00); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_rninta_395, bid128_to_int64_rninta, 0xB02000FC6F7C40458157E0B8A7A17FFFu128, -2000000000000001   , 0x00); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_rninta_396, bid128_to_int64_rninta, 0xB02000FC6F7C40458157E0B8A7A18000u128, -2000000000000002   , 0x00); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_rninta_397, bid128_to_int64_rninta, 0xB02000FC6F7C40458157E0B8A7A18001u128, -2000000000000002   , 0x00); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_rninta_398, bid128_to_int64_rninta, 0xb020c47709db8c6effffcffddfffffffu128, -398478348443794664 , 0x00);
dec_test!(bid128_to_int64_rninta_399, bid128_to_int64_rninta, 0xB02200193E5939A08CE4879688D63FFFu128, -1999999999999998   , 0x00); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_rninta_400, bid128_to_int64_rninta, 0xB02200193E5939A08CE4879688D64000u128, -1999999999999999   , 0x00); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_rninta_401, bid128_to_int64_rninta, 0xB02200193E5939A08CE4879688D64001u128, -1999999999999999   , 0x00); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_rninta_402, bid128_to_int64_rninta, 0xB02200193E5939A08CE815152D9CBFFFu128, -1999999999999999   , 0x00); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_rninta_403, bid128_to_int64_rninta, 0xB02200193E5939A08CE815152D9CC000u128, -2000000000000000   , 0x00); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_rninta_404, bid128_to_int64_rninta, 0xB02200193E5939A08CE815152D9CC001u128, -2000000000000000   , 0x00); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_rninta_405, bid128_to_int64_rninta, 0xb022ba38cfdbda29dc32cae713420290u128, -3777029292562045093, 0x00);
dec_test!(bid128_to_int64_rninta_406, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFAABC208D63FFFu128, -9223372036854775806, 0x00); // -- -(2^63-1.5-ulp)
dec_test!(bid128_to_int64_rninta_407, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFAABC208D64000u128, -9223372036854775807, 0x00); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_rninta_408, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFAABC208D64001u128, -9223372036854775807, 0x00); // -- -(2^63-1.5+ulp)
dec_test!(bid128_to_int64_rninta_409, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFC72815B397FFFu128, -9223372036854775807, 0x00); // -- -(2^63-1-ulp)
dec_test!(bid128_to_int64_rninta_410, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFC72815B398000u128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_rninta_411, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFC72815B398001u128, -9223372036854775807, 0x00); // -- -(2^63-1+ulp)
dec_test!(bid128_to_int64_rninta_412, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFE3940AD9CBFFFu128, -9223372036854775807, 0x00); // -- -(2^63-0.5-ulp)
dec_test!(bid128_to_int64_rninta_413, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFE3940AD9CC000u128, -9223372036854775808, 0x00); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_rninta_414, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFE3940AD9CC001u128, -9223372036854775808, 0x00); // -- -(2^63-0.5+ulp)
dec_test!(bid128_to_int64_rninta_415, bid128_to_int64_rninta, 0xB023C6BF52633FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x00); // -- -(2^63-ulp)
dec_test!(bid128_to_int64_rninta_416, bid128_to_int64_rninta, 0xB023C6BF526340000000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_rninta_417, bid128_to_int64_rninta, 0xB023C6BF526340000000000000000001u128, -9223372036854775808, 0x00); // -- -(2^63+ulp)
dec_test!(bid128_to_int64_rninta_418, bid128_to_int64_rninta, 0xB023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x00); // -- -(2^63+0.5-ulp)
dec_test!(bid128_to_int64_rninta_419, bid128_to_int64_rninta, 0xB023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_rninta_420, bid128_to_int64_rninta, 0xB023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- -(2^63+0.5+ulp)
dec_test!(bid128_to_int64_rninta_421, bid128_to_int64_rninta, 0xB023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- -(2^63+1-ulp)
dec_test!(bid128_to_int64_rninta_422, bid128_to_int64_rninta, 0xB023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_rninta_423, bid128_to_int64_rninta, 0xB023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- -(2^63+1+ulp)
dec_test!(bid128_to_int64_rninta_424, bid128_to_int64_rninta, 0xb023e25670f7a43bca7c3f24bb7eacf5u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_425, bid128_to_int64_rninta, 0xB024000000000000006A94D74F42FFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_426, bid128_to_int64_rninta, 0xB024000000000000006A94D74F430000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_427, bid128_to_int64_rninta, 0xB024000000000000006A94D74F430001u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_428, bid128_to_int64_rninta, 0xB024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e19-ulp)
dec_test!(bid128_to_int64_rninta_429, bid128_to_int64_rninta, 0xB024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_rninta_430, bid128_to_int64_rninta, 0xB024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e19+ulp)
dec_test!(bid128_to_int64_rninta_431, bid128_to_int64_rninta, 0xB024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- -(1e19+0.5-ulp)
dec_test!(bid128_to_int64_rninta_432, bid128_to_int64_rninta, 0xB024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_rninta_433, bid128_to_int64_rninta, 0xB024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- -(1e19+0.5+ulp)
dec_test!(bid128_to_int64_rninta_434, bid128_to_int64_rninta, 0xB02449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1.5e19-ulp)
dec_test!(bid128_to_int64_rninta_435, bid128_to_int64_rninta, 0xB02449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_rninta_436, bid128_to_int64_rninta, 0xB02449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- -(1.5e19+ulp)
dec_test!(bid128_to_int64_rninta_437, bid128_to_int64_rninta, 0xB0245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1-ulp)
dec_test!(bid128_to_int64_rninta_438, bid128_to_int64_rninta, 0xB0245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_rninta_439, bid128_to_int64_rninta, 0xB0245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- -(2^64-1+ulp)
dec_test!(bid128_to_int64_rninta_440, bid128_to_int64_rninta, 0xB0245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- -(2^64-0.5-ulp)
dec_test!(bid128_to_int64_rninta_441, bid128_to_int64_rninta, 0xB0245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_rninta_442, bid128_to_int64_rninta, 0xB0245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- -(2^64-0.5+ulp)
dec_test!(bid128_to_int64_rninta_443, bid128_to_int64_rninta, 0xB0245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-ulp)
dec_test!(bid128_to_int64_rninta_444, bid128_to_int64_rninta, 0xB0245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_rninta_445, bid128_to_int64_rninta, 0xB0245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+ulp)
dec_test!(bid128_to_int64_rninta_446, bid128_to_int64_rninta, 0xB0245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- -(2^64+0.5-ulp)
dec_test!(bid128_to_int64_rninta_447, bid128_to_int64_rninta, 0xB0245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_rninta_448, bid128_to_int64_rninta, 0xB0245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- -(2^64+0.5+ulp)
dec_test!(bid128_to_int64_rninta_449, bid128_to_int64_rninta, 0xB0245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- -(2^64+1-ulp)
dec_test!(bid128_to_int64_rninta_450, bid128_to_int64_rninta, 0xB0245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_rninta_451, bid128_to_int64_rninta, 0xB0245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- -(2^64+1+ulp)
dec_test!(bid128_to_int64_rninta_452, bid128_to_int64_rninta, 0xB024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2e19-ulp)
dec_test!(bid128_to_int64_rninta_453, bid128_to_int64_rninta, 0xB024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_rninta_454, bid128_to_int64_rninta, 0xB024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- -(2e19+ulp)
dec_test!(bid128_to_int64_rninta_455, bid128_to_int64_rninta, 0xB0247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2.5e19-ulp)
dec_test!(bid128_to_int64_rninta_456, bid128_to_int64_rninta, 0xB0247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_rninta_457, bid128_to_int64_rninta, 0xB0247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- -(2.5e19+ulp)
dec_test!(bid128_to_int64_rninta_458, bid128_to_int64_rninta, 0xB026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e20-ulp)
dec_test!(bid128_to_int64_rninta_459, bid128_to_int64_rninta, 0xB026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_rninta_460, bid128_to_int64_rninta, 0xB026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e20+ulp)
dec_test!(bid128_to_int64_rninta_461, bid128_to_int64_rninta, 0xB02A00000000006C6B935B68D08DA3FFu128, -19999999998        , 0x00); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_rninta_462, bid128_to_int64_rninta, 0xB02A00000000006C6B935B68D08DA400u128, -19999999999        , 0x00); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_rninta_463, bid128_to_int64_rninta, 0xB02A00000000006C6B935B68D08DA401u128, -19999999999        , 0x00); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_rninta_464, bid128_to_int64_rninta, 0xB02A00000000006C6B935B8019048BFFu128, -19999999999        , 0x00); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_rninta_465, bid128_to_int64_rninta, 0xB02A00000000006C6B935B8019048C00u128, -20000000000        , 0x00); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_rninta_466, bid128_to_int64_rninta, 0xB02A00000000006C6B935B8019048C01u128, -20000000000        , 0x00); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_rninta_467, bid128_to_int64_rninta, 0xB02C000000000000000002BBA7F521FFu128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rninta_468, bid128_to_int64_rninta, 0xB02C000000000000000002BBA7F52200u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rninta_469, bid128_to_int64_rninta, 0xB02C000000000000000002BBA7F52201u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rninta_470, bid128_to_int64_rninta, 0xB02C00000000000AD78EBC5872141BFFu128, -19999999999        , 0x00); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_rninta_471, bid128_to_int64_rninta, 0xB02C00000000000AD78EBC5872141C00u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_rninta_472, bid128_to_int64_rninta, 0xB02C00000000000AD78EBC5872141C01u128, -19999999999        , 0x00); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_rninta_473, bid128_to_int64_rninta, 0xB02C00000000000AD78EBC5BF025F1FFu128, -20000000000        , 0x00); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_rninta_474, bid128_to_int64_rninta, 0xB02C00000000000AD78EBC5BF025F200u128, -20000000001        , 0x00); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_rninta_475, bid128_to_int64_rninta, 0xB02C00000000000AD78EBC5BF025F201u128, -20000000001        , 0x00); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_rninta_476, bid128_to_int64_rninta, 0xB02C00000000000AD78EBC5E4431D5FFu128, -20000000001        , 0x00); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_rninta_477, bid128_to_int64_rninta, 0xB02C00000000000AD78EBC5E4431D600u128, -20000000002        , 0x00); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_rninta_478, bid128_to_int64_rninta, 0xB02C00000000000AD78EBC5E4431D601u128, -20000000002        , 0x00); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_rninta_479, bid128_to_int64_rninta, 0xB02C000000108B2A2C28028E3FF41BFFu128, -1999999999999999   , 0x00); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_rninta_480, bid128_to_int64_rninta, 0xB02C000000108B2A2C28028E3FF41C00u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_rninta_481, bid128_to_int64_rninta, 0xB02C000000108B2A2C28028E3FF41C01u128, -1999999999999999   , 0x00); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_rninta_482, bid128_to_int64_rninta, 0xB02E000000000001158E46094F6AC9FFu128, -20000000001        , 0x00); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_rninta_483, bid128_to_int64_rninta, 0xB02E000000000001158E46094F6ACA00u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_rninta_484, bid128_to_int64_rninta, 0xB02E000000000001158E46094F6ACA01u128, -20000000001        , 0x00); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_rninta_485, bid128_to_int64_rninta, 0xB02E00000001A784379D99DB7D9AC9FFu128, -2000000000000001   , 0x00); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_rninta_486, bid128_to_int64_rninta, 0xB02E00000001A784379D99DB7D9ACA00u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_rninta_487, bid128_to_int64_rninta, 0xB02E00000001A784379D99DB7D9ACA01u128, -2000000000000001   , 0x00); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_rninta_488, bid128_to_int64_rninta, 0xB03000000000000000000006FC23ABFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_489, bid128_to_int64_rninta, 0xB03000000000000000000006FC23AC00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_490, bid128_to_int64_rninta, 0xB03000000000000000000006FC23AC01u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_491, bid128_to_int64_rninta, 0xB03200000000000000000000B2D05DFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_492, bid128_to_int64_rninta, 0xB03200000000000000000000B2D05E00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_493, bid128_to_int64_rninta, 0xB03200000000000000000000B2D05E01u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_494, bid128_to_int64_rninta, 0xB03800000000000000000000002DDA47u128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rninta_495, bid128_to_int64_rninta, 0xB03800000000000000000000002DDA48u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rninta_496, bid128_to_int64_rninta, 0xB03800000000000000000000002DDA49u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rninta_497, bid128_to_int64_rninta, 0xB03A00000000000000000000000003E7u128, -1                  , 0x00); // -- -(0.999)
dec_test!(bid128_to_int64_rninta_498, bid128_to_int64_rninta, 0xB03A00000000000000000000000495D3u128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rninta_499, bid128_to_int64_rninta, 0xB03A00000000000000000000000495D4u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rninta_500, bid128_to_int64_rninta, 0xB03A00000000000000000000000495D5u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rninta_501, bid128_to_int64_rninta, 0xB03C0000000000000000000000007561u128, -300                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_rninta_502, bid128_to_int64_rninta, 0xB03C0000000000000000000000007562u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rninta_503, bid128_to_int64_rninta, 0xB03C0000000000000000000000007563u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_rninta_504, bid128_to_int64_rninta, 0xB03E0000000000000000000000000005u128, -1                  , 0x00); // -- -(0.5)
dec_test!(bid128_to_int64_rninta_505, bid128_to_int64_rninta, 0xB03E000000000000000000000000000Fu128, -2                  , 0x00); // -- -(1.5)
dec_test!(bid128_to_int64_rninta_506, bid128_to_int64_rninta, 0xB03E0000000000000000000000000BB7u128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_507, bid128_to_int64_rninta, 0xB03E0000000000000000000000000BB8u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_508, bid128_to_int64_rninta, 0xB03E0000000000000000000000000BB9u128, -300                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_509, bid128_to_int64_rninta, 0xB03E0000000000000000000000000BBDu128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_rninta_510, bid128_to_int64_rninta, 0xB03E0000000000000000002E90EDCFF1u128, -19999999999        , 0x00); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_rninta_511, bid128_to_int64_rninta, 0xB03E0000000000000000002E90EDCFFBu128, -20000000000        , 0x00); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_rninta_512, bid128_to_int64_rninta, 0xB03E0000000000000000002E90EDD005u128, -20000000001        , 0x00); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_rninta_513, bid128_to_int64_rninta, 0xB03E0000000000000000002E90EDD00Fu128, -20000000002        , 0x00); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_rninta_514, bid128_to_int64_rninta, 0xB03E0000000000000001400000000005u128, -35184372088833     , 0x00); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_rninta_515, bid128_to_int64_rninta, 0xB03E00000000000000470DE4DF81FFF1u128, -1999999999999999   , 0x00); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_rninta_516, bid128_to_int64_rninta, 0xB03E00000000000000470DE4DF81FFFBu128, -2000000000000000   , 0x00); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_rninta_517, bid128_to_int64_rninta, 0xB03E00000000000000470DE4DF820005u128, -2000000000000001   , 0x00); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_rninta_518, bid128_to_int64_rninta, 0xB03E00000000000000470DE4DF82000Fu128, -2000000000000002   , 0x00); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_rninta_519, bid128_to_int64_rninta, 0xB03E000000000004FFFFFFFFFFFFFFF1u128, -9223372036854775807, 0x00); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_rninta_520, bid128_to_int64_rninta, 0xB03E000000000004FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x00); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_rninta_521, bid128_to_int64_rninta, 0xB03E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_rninta_522, bid128_to_int64_rninta, 0xB03E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_rninta_523, bid128_to_int64_rninta, 0xB03E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_rninta_524, bid128_to_int64_rninta, 0xB03E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_rninta_525, bid128_to_int64_rninta, 0xB0400000000000000000000000000001u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_rninta_526, bid128_to_int64_rninta, 0xB040000000000000000000000000012Bu128, -299                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_527, bid128_to_int64_rninta, 0xB040000000000000000000000000012Cu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_528, bid128_to_int64_rninta, 0xB040000000000000000000000000012Du128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_529, bid128_to_int64_rninta, 0xB04000000000000000000004A817C7FFu128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_rninta_530, bid128_to_int64_rninta, 0xB04000000000000000000004A817C801u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_rninta_531, bid128_to_int64_rninta, 0xB0400000000000000000200000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_rninta_532, bid128_to_int64_rninta, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_rninta_533, bid128_to_int64_rninta, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_rninta_534, bid128_to_int64_rninta, 0xB04000000000000000071AFD498D0000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_rninta_535, bid128_to_int64_rninta, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_rninta_536, bid128_to_int64_rninta, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_rninta_537, bid128_to_int64_rninta, 0xB0400000000000007FFFFFFFFFFFFFFFu128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_rninta_538, bid128_to_int64_rninta, 0xB0400000000000008000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_rninta_539, bid128_to_int64_rninta, 0xB0400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_rninta_540, bid128_to_int64_rninta, 0xB040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_rninta_541, bid128_to_int64_rninta, 0xB0400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_rninta_542, bid128_to_int64_rninta, 0xB0400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_rninta_543, bid128_to_int64_rninta, 0xB042000000000000000000000000001Du128, -290                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_rninta_544, bid128_to_int64_rninta, 0xB042000000000000000000000000001Eu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_545, bid128_to_int64_rninta, 0xB042000000000000000000000000001Fu128, -310                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_rninta_546, bid128_to_int64_rninta, 0xB04200000000000000000000773593FFu128, -19999999990        , 0x00); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_rninta_547, bid128_to_int64_rninta, 0xB0420000000000000000000077359400u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_rninta_548, bid128_to_int64_rninta, 0xB0420000000000000000000077359401u128, -20000000010        , 0x00); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_rninta_549, bid128_to_int64_rninta, 0xB0440000000000000000000000000003u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_rninta_550, bid128_to_int64_rninta, 0xB0520000000000000000000000000004u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_rninta_551, bid128_to_int64_rninta, 0xB0520000000000000000000000000005u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_rninta_552, bid128_to_int64_rninta, 0xB0540000000000000000000000000002u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_rninta_553, bid128_to_int64_rninta, 0xB05E0000000000000000000000000002u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_rninta_554, bid128_to_int64_rninta, 0xB064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_rninta_555, bid128_to_int64_rninta, 0xB0640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_rninta_556, bid128_to_int64_rninta, 0xB0660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_rninta_557, bid128_to_int64_rninta, 0xB0660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_rninta_558, bid128_to_int64_rninta, 0xB0680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_rninta_559, bid128_to_int64_rninta, 0xb177ae1f0fbaeae6735166a237469c3fu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_560, bid128_to_int64_rninta, 0xb8560000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_561, bid128_to_int64_rninta, 0xc1b604fbb3a0bead5f22bf982b7183b4u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_562, bid128_to_int64_rninta, 0xc1c20507539924fd6ea354666ca07f6eu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_563, bid128_to_int64_rninta, 0xd42bb0c423ed7de2a5d91135f6842bfcu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_564, bid128_to_int64_rninta, 0xda6c0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_565, bid128_to_int64_rninta, 0xf7f7fffaff7ffb4bf7eff5ffdfffdbf7u128, 0                   , 0x00);
dec_test!(bid128_to_int64_rninta_566, bid128_to_int64_rninta, 0xfbbfedfbfdbfeffd0000008800100802u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_567, bid128_to_int64_rninta, 0xfc0014f646c840686332e0a57568e7c1u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_568, bid128_to_int64_rninta, "-Infinity"                           , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_569, bid128_to_int64_rninta, "Infinity"                            , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_570, bid128_to_int64_rninta, "QNaN"                                , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_rninta_571, bid128_to_int64_rninta, "SNaN"                                , -9223372036854775808, 0x01);
