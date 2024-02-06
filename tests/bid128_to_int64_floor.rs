/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int64_floor_001, bid128_to_int64_floor, "-0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_floor_002, bid128_to_int64_floor,  "0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_floor_003, bid128_to_int64_floor, 0x00000000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_004, bid128_to_int64_floor, 0x00000000000000000000002000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_005, bid128_to_int64_floor, 0x00000000000000000804100000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_006, bid128_to_int64_floor, 0x0000000000000000afd59ea57edef575u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_007, bid128_to_int64_floor, 0x0001ed09bead87c0378d8e62ffffffffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_008, bid128_to_int64_floor, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_009, bid128_to_int64_floor, "1.0"                                 , 1                   , 0x00);
dec_test!(bid128_to_int64_floor_010, bid128_to_int64_floor, "1152921504606846976"                 , 1152921504606846976 , 0x00);
dec_test!(bid128_to_int64_floor_011, bid128_to_int64_floor, 0x11cd4f45a75a0e18056e2e9019475f83u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_012, bid128_to_int64_floor, 0x1b18eec278dc29519ebed123c533d962u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_013, bid128_to_int64_floor, 0x1d1e0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_014, bid128_to_int64_floor, 0x240652287b972a5392e84ab8f917df43u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_015, bid128_to_int64_floor, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x00); // -- 0.5-ulp
dec_test!(bid128_to_int64_floor_016, bid128_to_int64_floor, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0                   , 0x00); // -- 0.5
dec_test!(bid128_to_int64_floor_017, bid128_to_int64_floor, 0x2FFCF684DF56C3E01BC6C73200000001u128, 0                   , 0x00); // -- 0.5+ulp
dec_test!(bid128_to_int64_floor_018, bid128_to_int64_floor, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 0                   , 0x00); // -- 0.999-ulp
dec_test!(bid128_to_int64_floor_019, bid128_to_int64_floor, 0x2FFDEC8B86EF679D76FC433D80000000u128, 0                   , 0x00); // -- 0.999
dec_test!(bid128_to_int64_floor_020, bid128_to_int64_floor, 0x2FFDEC8B86EF679D76FC433D80000001u128, 0                   , 0x00); // -- 0.999+ulp
dec_test!(bid128_to_int64_floor_021, bid128_to_int64_floor, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 0                   , 0x00); // -- 1-ulp
dec_test!(bid128_to_int64_floor_022, bid128_to_int64_floor, 0x2FFE314DC6448D9338C15B0A00000000u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_floor_023, bid128_to_int64_floor, 0x2FFE314DC6448D9338C15B0A00000001u128, 1                   , 0x00); // -- 1+ulp
dec_test!(bid128_to_int64_floor_024, bid128_to_int64_floor, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1                   , 0x00); // -- 1.5-ulp
dec_test!(bid128_to_int64_floor_025, bid128_to_int64_floor, 0x2FFE49F4A966D45CD522088F00000000u128, 1                   , 0x00); // -- 1.5
dec_test!(bid128_to_int64_floor_026, bid128_to_int64_floor, 0x2FFE49F4A966D45CD522088F00000001u128, 1                   , 0x00); // -- 1.5+ulp
dec_test!(bid128_to_int64_floor_027, bid128_to_int64_floor, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_028, bid128_to_int64_floor, 0x300293E952CDA8B9AA44111E00000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_029, bid128_to_int64_floor, 0x300293E952CDA8B9AA44111E00000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_030, bid128_to_int64_floor, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_floor_031, bid128_to_int64_floor, 0x300294286EACB8CB0A8CB6B140000000u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_floor_032, bid128_to_int64_floor, 0x300294286EACB8CB0A8CB6B140000001u128, 300                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_floor_033, bid128_to_int64_floor, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_034, bid128_to_int64_floor, 0x30040ECA8847C4129106CE8300000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_035, bid128_to_int64_floor, 0x30040ECA8847C4129106CE8300000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_036, bid128_to_int64_floor, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_037, bid128_to_int64_floor, 0x300A0003C95A2F0B4856475FE0000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_038, bid128_to_int64_floor, 0x300A0003C95A2F0B4856475FE0000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_039, bid128_to_int64_floor, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_040, bid128_to_int64_floor, 0x300C000060EF6B1ABA6F072330000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_041, bid128_to_int64_floor, 0x300C000060EF6B1ABA6F072330000001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_042, bid128_to_int64_floor, 0x3010C5371912364CE3056C27FFFFFFFFu128, 3999999999          , 0x00); // -- 4e9-ulp
dec_test!(bid128_to_int64_floor_043, bid128_to_int64_floor, 0x3010C5371912364CE3056C2800000000u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_floor_044, bid128_to_int64_floor, 0x3010C5371912364CE3056C2800000001u128, 4000000000          , 0x00); // -- 4e9+ulp
dec_test!(bid128_to_int64_floor_045, bid128_to_int64_floor, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 4999999999          , 0x00); // -- 5e9-ulp
dec_test!(bid128_to_int64_floor_046, bid128_to_int64_floor, 0x3010F684DF56C3E01BC6C73200000000u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_floor_047, bid128_to_int64_floor, 0x3010F684DF56C3E01BC6C73200000001u128, 5000000000          , 0x00); // -- 5e9+ulp
dec_test!(bid128_to_int64_floor_048, bid128_to_int64_floor, 0x3012328809041443fbf57ab87f88cd77u128, 10248983005         , 0x00);
dec_test!(bid128_to_int64_floor_049, bid128_to_int64_floor, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 19999999998         , 0x00); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_floor_050, bid128_to_int64_floor, 0x3012629B8C88FB62ED56E4238E400000u128, 19999999998         , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_int64_floor_051, bid128_to_int64_floor, 0x3012629B8C88FB62ED56E4238E400001u128, 19999999998         , 0x00); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_floor_052, bid128_to_int64_floor, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 19999999998         , 0x00); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_floor_053, bid128_to_int64_floor, 0x3012629B8C8905F96EBAD4C909800000u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_floor_054, bid128_to_int64_floor, 0x3012629B8C8905F96EBAD4C909800001u128, 19999999999         , 0x00); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_floor_055, bid128_to_int64_floor, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 19999999999         , 0x00); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_floor_056, bid128_to_int64_floor, 0x3012629B8C89108FF01EC56E84C00000u128, 19999999999         , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_int64_floor_057, bid128_to_int64_floor, 0x3012629B8C89108FF01EC56E84C00001u128, 19999999999         , 0x00); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_floor_058, bid128_to_int64_floor, 0x3012629B8C891B267182B613FFFFFFFFu128, 19999999999         , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_int64_floor_059, bid128_to_int64_floor, 0x3012629B8C891B267182B61400000000u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_floor_060, bid128_to_int64_floor, 0x3012629B8C891B267182B61400000001u128, 20000000000         , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_int64_floor_061, bid128_to_int64_floor, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 20000000000         , 0x00); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_floor_062, bid128_to_int64_floor, 0x3012629B8C8925BCF2E6A6B97B400000u128, 20000000000         , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_int64_floor_063, bid128_to_int64_floor, 0x3012629B8C8925BCF2E6A6B97B400001u128, 20000000000         , 0x00); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_floor_064, bid128_to_int64_floor, 0x3012629B8C893053744A975EF67FFFFFu128, 20000000000         , 0x00); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_floor_065, bid128_to_int64_floor, 0x3012629B8C893053744A975EF6800000u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_floor_066, bid128_to_int64_floor, 0x3012629B8C893053744A975EF6800001u128, 20000000001         , 0x00); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_floor_067, bid128_to_int64_floor, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 20000000001         , 0x00); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_floor_068, bid128_to_int64_floor, 0x3012629B8C893AE9F5AE880471C00000u128, 20000000001         , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_int64_floor_069, bid128_to_int64_floor, 0x3012629B8C893AE9F5AE880471C00001u128, 20000000001         , 0x00); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_floor_070, bid128_to_int64_floor, 0x3018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 35184372088831      , 0x00); // -- 2^45-ulp
dec_test!(bid128_to_int64_floor_071, bid128_to_int64_floor, 0x3018AD78EBC5AC620000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_floor_072, bid128_to_int64_floor, 0x3018AD78EBC5AC620000000000000001u128, 35184372088832      , 0x00); // -- 2^45+ulp
dec_test!(bid128_to_int64_floor_073, bid128_to_int64_floor, 0x3018AD78EBC5AC64B5E3AF16B187FFFFu128, 35184372088832      , 0x00); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_floor_074, bid128_to_int64_floor, 0x3018AD78EBC5AC64B5E3AF16B1880000u128, 35184372088832      , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_int64_floor_075, bid128_to_int64_floor, 0x3018AD78EBC5AC64B5E3AF16B1880001u128, 35184372088832      , 0x00); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_floor_076, bid128_to_int64_floor, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_floor_077, bid128_to_int64_floor, 0x301A0000000000A2E6C09AD3E0D40000u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_floor_078, bid128_to_int64_floor, 0x301A0000000000A2E6C09AD3E0D40001u128, 300                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_floor_079, bid128_to_int64_floor, 0x301C629B8C891B265CB1A40684E9FFFFu128, 1999999999999998    , 0x00); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_floor_080, bid128_to_int64_floor, 0x301C629B8C891B265CB1A40684EA0000u128, 1999999999999998    , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_int64_floor_081, bid128_to_int64_floor, 0x301C629B8C891B265CB1A40684EA0001u128, 1999999999999998    , 0x00); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_floor_082, bid128_to_int64_floor, 0x301C629B8C891B2663A1FF60589BFFFFu128, 1999999999999998    , 0x00); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_floor_083, bid128_to_int64_floor, 0x301C629B8C891B2663A1FF60589C0000u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_floor_084, bid128_to_int64_floor, 0x301C629B8C891B2663A1FF60589C0001u128, 1999999999999999    , 0x00); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_floor_085, bid128_to_int64_floor, 0x301C629B8C891B266A925ABA2C4DFFFFu128, 1999999999999999    , 0x00); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_floor_086, bid128_to_int64_floor, 0x301C629B8C891B266A925ABA2C4E0000u128, 1999999999999999    , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_int64_floor_087, bid128_to_int64_floor, 0x301C629B8C891B266A925ABA2C4E0001u128, 1999999999999999    , 0x00); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_floor_088, bid128_to_int64_floor, 0x301C629B8C891B267182B613FFFFFFFFu128, 1999999999999999    , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_int64_floor_089, bid128_to_int64_floor, 0x301C629B8C891B267182B61400000000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_floor_090, bid128_to_int64_floor, 0x301C629B8C891B267182B61400000001u128, 2000000000000000    , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_int64_floor_091, bid128_to_int64_floor, 0x301C629B8C891B267873116DD3B1FFFFu128, 2000000000000000    , 0x00); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_floor_092, bid128_to_int64_floor, 0x301C629B8C891B267873116DD3B20000u128, 2000000000000000    , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_int64_floor_093, bid128_to_int64_floor, 0x301C629B8C891B267873116DD3B20001u128, 2000000000000000    , 0x00); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_floor_094, bid128_to_int64_floor, 0x301C629B8C891B267F636CC7A763FFFFu128, 2000000000000000    , 0x00); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_floor_095, bid128_to_int64_floor, 0x301C629B8C891B267F636CC7A7640000u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_floor_096, bid128_to_int64_floor, 0x301C629B8C891B267F636CC7A7640001u128, 2000000000000001    , 0x00); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_floor_097, bid128_to_int64_floor, 0x301C629B8C891B268653C8217B15FFFFu128, 2000000000000001    , 0x00); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_floor_098, bid128_to_int64_floor, 0x301C629B8C891B268653C8217B160000u128, 2000000000000001    , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_int64_floor_099, bid128_to_int64_floor, 0x301C629B8C891B268653C8217B160001u128, 2000000000000001    , 0x00); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_floor_100, bid128_to_int64_floor, 0x301E000000000001A055690D9DB7FFFFu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_101, bid128_to_int64_floor, 0x301E000000000001A055690D9DB80000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_102, bid128_to_int64_floor, 0x301E000000000001A055690D9DB80001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_103, bid128_to_int64_floor, 0x301E002C68AF0BB140B1A2BC2EC4FFFFu128, 35184372088832      , 0x00); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_floor_104, bid128_to_int64_floor, 0x301E002C68AF0BB140B1A2BC2EC50000u128, 35184372088832      , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_int64_floor_105, bid128_to_int64_floor, 0x301E002C68AF0BB140B1A2BC2EC50001u128, 35184372088832      , 0x00); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_floor_106, bid128_to_int64_floor, 0x302000000000000029A2241AF62BFFFFu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_107, bid128_to_int64_floor, 0x302000000000000029A2241AF62C0000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_108, bid128_to_int64_floor, 0x302000000000000029A2241AF62C0001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_109, bid128_to_int64_floor, 0x3020000470DE4DF81FFFFFFFFFFFFFFFu128, 35184372088831      , 0x00); // -- 2^45-ulp
dec_test!(bid128_to_int64_floor_110, bid128_to_int64_floor, 0x3020000470DE4DF82000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_floor_111, bid128_to_int64_floor, 0x3020000470DE4DF82000000000000001u128, 35184372088832      , 0x00); // -- 2^45+ulp
dec_test!(bid128_to_int64_floor_112, bid128_to_int64_floor, 0x302000FC6F7C4045813459C637E07FFFu128, 2000000000000000    , 0x00); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_floor_113, bid128_to_int64_floor, 0x302000FC6F7C4045813459C637E08000u128, 2000000000000000    , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_int64_floor_114, bid128_to_int64_floor, 0x302000FC6F7C4045813459C637E08001u128, 2000000000000000    , 0x00); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_floor_115, bid128_to_int64_floor, 0x302000FC6F7C40458157E0B8A7A17FFFu128, 2000000000000001    , 0x00); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_floor_116, bid128_to_int64_floor, 0x302000FC6F7C40458157E0B8A7A18000u128, 2000000000000001    , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_int64_floor_117, bid128_to_int64_floor, 0x302000FC6F7C40458157E0B8A7A18001u128, 2000000000000001    , 0x00); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_floor_118, bid128_to_int64_floor, 0x302200193E5939A08CE4879688D63FFFu128, 1999999999999998    , 0x00); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_floor_119, bid128_to_int64_floor, 0x302200193E5939A08CE4879688D64000u128, 1999999999999998    , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_int64_floor_120, bid128_to_int64_floor, 0x302200193E5939A08CE4879688D64001u128, 1999999999999998    , 0x00); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_floor_121, bid128_to_int64_floor, 0x302200193E5939A08CE815152D9CBFFFu128, 1999999999999999    , 0x00); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_floor_122, bid128_to_int64_floor, 0x302200193E5939A08CE815152D9CC000u128, 1999999999999999    , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_int64_floor_123, bid128_to_int64_floor, 0x302200193E5939A08CE815152D9CC001u128, 1999999999999999    , 0x00); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_floor_124, bid128_to_int64_floor, 0x30227426399e455fc2f4ea96abf32844u128, 2355788016182584905 , 0x00);
dec_test!(bid128_to_int64_floor_125, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775806 , 0x00); // -- 2^63-1.5-ulp
dec_test!(bid128_to_int64_floor_126, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775806 , 0x00); // -- 2^63-1.5
dec_test!(bid128_to_int64_floor_127, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775806 , 0x00); // -- 2^63-1.5+ulp
dec_test!(bid128_to_int64_floor_128, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775806 , 0x00); // -- 2^63-1-ulp
dec_test!(bid128_to_int64_floor_129, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFC72815B398000u128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_floor_130, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFC72815B398001u128, 9223372036854775807 , 0x00); // -- 2^63-1+ulp
dec_test!(bid128_to_int64_floor_131, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775807 , 0x00); // -- 2^63-0.5-ulp
dec_test!(bid128_to_int64_floor_132, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFE3940AD9CC000u128, 9223372036854775807 , 0x00); // -- 2^63-0.5
dec_test!(bid128_to_int64_floor_133, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFE3940AD9CC001u128, 9223372036854775807 , 0x00); // -- 2^63-0.5+ulp
dec_test!(bid128_to_int64_floor_134, bid128_to_int64_floor, 0x3023C6BF52633FFFFFFFFFFFFFFFFFFFu128, 9223372036854775807 , 0x00); // -- 2^63-ulp
dec_test!(bid128_to_int64_floor_135, bid128_to_int64_floor, 0x3023C6BF526340000000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_floor_136, bid128_to_int64_floor, 0x3023C6BF526340000000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+ulp
dec_test!(bid128_to_int64_floor_137, bid128_to_int64_floor, 0x3023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x01); // -- 2^63+0.5-ulp
dec_test!(bid128_to_int64_floor_138, bid128_to_int64_floor, 0x3023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_floor_139, bid128_to_int64_floor, 0x3023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- 2^63+0.5+ulp
dec_test!(bid128_to_int64_floor_140, bid128_to_int64_floor, 0x3023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- 2^63+1-ulp
dec_test!(bid128_to_int64_floor_141, bid128_to_int64_floor, 0x3023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_floor_142, bid128_to_int64_floor, 0x3023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- 2^63+1+ulp
dec_test!(bid128_to_int64_floor_143, bid128_to_int64_floor, 0x3023cbe5f7f0933ff1c66c03b14ff837u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_144, bid128_to_int64_floor, 0x3024000000000000006A94D74F42FFFFu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_145, bid128_to_int64_floor, 0x3024000000000000006A94D74F430000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_146, bid128_to_int64_floor, 0x3024000000000000006A94D74F430001u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_147, bid128_to_int64_floor, 0x3024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e19-ulp
dec_test!(bid128_to_int64_floor_148, bid128_to_int64_floor, 0x3024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_floor_149, bid128_to_int64_floor, 0x3024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e19+ulp
dec_test!(bid128_to_int64_floor_150, bid128_to_int64_floor, 0x3024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- 1e19+0.5-ulp
dec_test!(bid128_to_int64_floor_151, bid128_to_int64_floor, 0x3024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_floor_152, bid128_to_int64_floor, 0x3024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- 1e19+0.5+ulp
dec_test!(bid128_to_int64_floor_153, bid128_to_int64_floor, 0x302449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- 1.5e19-ulp
dec_test!(bid128_to_int64_floor_154, bid128_to_int64_floor, 0x302449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_floor_155, bid128_to_int64_floor, 0x302449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- 1.5e19+ulp
dec_test!(bid128_to_int64_floor_156, bid128_to_int64_floor, 0x30245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- 2^64-1-ulp
dec_test!(bid128_to_int64_floor_157, bid128_to_int64_floor, 0x30245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_floor_158, bid128_to_int64_floor, 0x30245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- 2^64-1+ulp
dec_test!(bid128_to_int64_floor_159, bid128_to_int64_floor, 0x30245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- 2^64-0.5-ulp
dec_test!(bid128_to_int64_floor_160, bid128_to_int64_floor, 0x30245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_floor_161, bid128_to_int64_floor, 0x30245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- 2^64-0.5+ulp
dec_test!(bid128_to_int64_floor_162, bid128_to_int64_floor, 0x30245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-ulp
dec_test!(bid128_to_int64_floor_163, bid128_to_int64_floor, 0x30245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_floor_164, bid128_to_int64_floor, 0x30245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+ulp
dec_test!(bid128_to_int64_floor_165, bid128_to_int64_floor, 0x30245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- 2^64+0.5-ulp
dec_test!(bid128_to_int64_floor_166, bid128_to_int64_floor, 0x30245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_floor_167, bid128_to_int64_floor, 0x30245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- 2^64+0.5+ulp
dec_test!(bid128_to_int64_floor_168, bid128_to_int64_floor, 0x30245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- 2^64+1-ulp
dec_test!(bid128_to_int64_floor_169, bid128_to_int64_floor, 0x30245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_floor_170, bid128_to_int64_floor, 0x30245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- 2^64+1+ulp
dec_test!(bid128_to_int64_floor_171, bid128_to_int64_floor, 0x3024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2e19-ulp
dec_test!(bid128_to_int64_floor_172, bid128_to_int64_floor, 0x3024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_floor_173, bid128_to_int64_floor, 0x3024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- 2e19+ulp
dec_test!(bid128_to_int64_floor_174, bid128_to_int64_floor, 0x30247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2.5e19-ulp
dec_test!(bid128_to_int64_floor_175, bid128_to_int64_floor, 0x30247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_floor_176, bid128_to_int64_floor, 0x30247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- 2.5e19+ulp
dec_test!(bid128_to_int64_floor_177, bid128_to_int64_floor, 0x3026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e20-ulp
dec_test!(bid128_to_int64_floor_178, bid128_to_int64_floor, 0x3026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_floor_179, bid128_to_int64_floor, 0x3026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e20+ulp
dec_test!(bid128_to_int64_floor_180, bid128_to_int64_floor, 0x302A00000000006C6B935B68D08DA3FFu128, 19999999998         , 0x00); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_floor_181, bid128_to_int64_floor, 0x302A00000000006C6B935B68D08DA400u128, 19999999998         , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_int64_floor_182, bid128_to_int64_floor, 0x302A00000000006C6B935B68D08DA401u128, 19999999998         , 0x00); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_floor_183, bid128_to_int64_floor, 0x302A00000000006C6B935B8019048BFFu128, 19999999999         , 0x00); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_floor_184, bid128_to_int64_floor, 0x302A00000000006C6B935B8019048C00u128, 19999999999         , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_int64_floor_185, bid128_to_int64_floor, 0x302A00000000006C6B935B8019048C01u128, 19999999999         , 0x00); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_floor_186, bid128_to_int64_floor, 0x302C000000000000000002BBA7F521FFu128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_floor_187, bid128_to_int64_floor, 0x302C000000000000000002BBA7F52200u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_floor_188, bid128_to_int64_floor, 0x302C000000000000000002BBA7F52201u128, 300                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_floor_189, bid128_to_int64_floor, 0x302C00000000000AD78EBC5872141BFFu128, 19999999998         , 0x00); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_floor_190, bid128_to_int64_floor, 0x302C00000000000AD78EBC5872141C00u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_floor_191, bid128_to_int64_floor, 0x302C00000000000AD78EBC5872141C01u128, 19999999999         , 0x00); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_floor_192, bid128_to_int64_floor, 0x302C00000000000AD78EBC5BF025F1FFu128, 20000000000         , 0x00); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_floor_193, bid128_to_int64_floor, 0x302C00000000000AD78EBC5BF025F200u128, 20000000000         , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_int64_floor_194, bid128_to_int64_floor, 0x302C00000000000AD78EBC5BF025F201u128, 20000000000         , 0x00); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_floor_195, bid128_to_int64_floor, 0x302C00000000000AD78EBC5E4431D5FFu128, 20000000001         , 0x00); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_floor_196, bid128_to_int64_floor, 0x302C00000000000AD78EBC5E4431D600u128, 20000000001         , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_int64_floor_197, bid128_to_int64_floor, 0x302C00000000000AD78EBC5E4431D601u128, 20000000001         , 0x00); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_floor_198, bid128_to_int64_floor, 0x302C000000108B2A2C28028E3FF41BFFu128, 1999999999999998    , 0x00); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_floor_199, bid128_to_int64_floor, 0x302C000000108B2A2C28028E3FF41C00u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_floor_200, bid128_to_int64_floor, 0x302C000000108B2A2C28028E3FF41C01u128, 1999999999999999    , 0x00); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_floor_201, bid128_to_int64_floor, 0x302E000000000001158E46094F6AC9FFu128, 20000000000         , 0x00); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_floor_202, bid128_to_int64_floor, 0x302E000000000001158E46094F6ACA00u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_floor_203, bid128_to_int64_floor, 0x302E000000000001158E46094F6ACA01u128, 20000000001         , 0x00); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_floor_204, bid128_to_int64_floor, 0x302E00000001A784379D99DB7D9AC9FFu128, 2000000000000000    , 0x00); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_floor_205, bid128_to_int64_floor, 0x302E00000001A784379D99DB7D9ACA00u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_floor_206, bid128_to_int64_floor, 0x302E00000001A784379D99DB7D9ACA01u128, 2000000000000001    , 0x00); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_floor_207, bid128_to_int64_floor, 0x303000000000000000000006FC23ABFFu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_208, bid128_to_int64_floor, 0x303000000000000000000006FC23AC00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_209, bid128_to_int64_floor, 0x303000000000000000000006FC23AC01u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_210, bid128_to_int64_floor, 0x303200000000000000000000B2D05DFFu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_211, bid128_to_int64_floor, 0x303200000000000000000000B2D05E00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_212, bid128_to_int64_floor, 0x303200000000000000000000B2D05E01u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_213, bid128_to_int64_floor, 0x303800000000000000000000002DDA47u128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_floor_214, bid128_to_int64_floor, 0x303800000000000000000000002DDA48u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_floor_215, bid128_to_int64_floor, 0x303800000000000000000000002DDA49u128, 300                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_floor_216, bid128_to_int64_floor, 0x303A00000000000000000000000003E7u128, 0                   , 0x00); // -- 0.999
dec_test!(bid128_to_int64_floor_217, bid128_to_int64_floor, 0x303A00000000000000000000000495D3u128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_floor_218, bid128_to_int64_floor, 0x303A00000000000000000000000495D4u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_floor_219, bid128_to_int64_floor, 0x303A00000000000000000000000495D5u128, 300                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_floor_220, bid128_to_int64_floor, 0x303C0000000000000000000000007561u128, 300                 , 0x00); // -- 300.5-ulp
dec_test!(bid128_to_int64_floor_221, bid128_to_int64_floor, 0x303C0000000000000000000000007562u128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_floor_222, bid128_to_int64_floor, 0x303C0000000000000000000000007563u128, 300                 , 0x00); // -- 300.5+ulp
dec_test!(bid128_to_int64_floor_223, bid128_to_int64_floor, 0x303E0000000000000000000000000005u128, 0                   , 0x00); // -- 0.5
dec_test!(bid128_to_int64_floor_224, bid128_to_int64_floor, 0x303E000000000000000000000000000Fu128, 1                   , 0x00); // -- 1.5
dec_test!(bid128_to_int64_floor_225, bid128_to_int64_floor, 0x303E0000000000000000000000000BB7u128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_226, bid128_to_int64_floor, 0x303E0000000000000000000000000BB8u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_227, bid128_to_int64_floor, 0x303E0000000000000000000000000BB9u128, 300                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_228, bid128_to_int64_floor, 0x303E0000000000000000000000000BBDu128, 300                 , 0x00); // -- 300.5
dec_test!(bid128_to_int64_floor_229, bid128_to_int64_floor, 0x303E0000000000000000002E90EDCFF1u128, 19999999998         , 0x00); // -- 2e10-1.5
dec_test!(bid128_to_int64_floor_230, bid128_to_int64_floor, 0x303E0000000000000000002E90EDCFFBu128, 19999999999         , 0x00); // -- 2e10-0.5
dec_test!(bid128_to_int64_floor_231, bid128_to_int64_floor, 0x303E0000000000000000002E90EDD005u128, 20000000000         , 0x00); // -- 2e10+0.5
dec_test!(bid128_to_int64_floor_232, bid128_to_int64_floor, 0x303E0000000000000000002E90EDD00Fu128, 20000000001         , 0x00); // -- 2e10+1.5
dec_test!(bid128_to_int64_floor_233, bid128_to_int64_floor, 0x303E0000000000000001400000000005u128, 35184372088832      , 0x00); // -- 2^45+0.5
dec_test!(bid128_to_int64_floor_234, bid128_to_int64_floor, 0x303E00000000000000470DE4DF81FFF1u128, 1999999999999998    , 0x00); // -- 2e15-1.5
dec_test!(bid128_to_int64_floor_235, bid128_to_int64_floor, 0x303E00000000000000470DE4DF81FFFBu128, 1999999999999999    , 0x00); // -- 2e15-0.5
dec_test!(bid128_to_int64_floor_236, bid128_to_int64_floor, 0x303E00000000000000470DE4DF820005u128, 2000000000000000    , 0x00); // -- 2e15+0.5
dec_test!(bid128_to_int64_floor_237, bid128_to_int64_floor, 0x303E00000000000000470DE4DF82000Fu128, 2000000000000001    , 0x00); // -- 2e15+1.5
dec_test!(bid128_to_int64_floor_238, bid128_to_int64_floor, 0x303E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775806 , 0x00); // -- 2^63-1.5
dec_test!(bid128_to_int64_floor_239, bid128_to_int64_floor, 0x303E000000000004FFFFFFFFFFFFFFFBu128, 9223372036854775807 , 0x00); // -- 2^63-0.5
dec_test!(bid128_to_int64_floor_240, bid128_to_int64_floor, 0x303E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_floor_241, bid128_to_int64_floor, 0x303E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_floor_242, bid128_to_int64_floor, 0x303E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_floor_243, bid128_to_int64_floor, 0x303E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_floor_244, bid128_to_int64_floor, 0x30400000000000000000000000000001u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_floor_245, bid128_to_int64_floor, 0x3040000000000000000000000000012Bu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_246, bid128_to_int64_floor, 0x3040000000000000000000000000012Cu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_247, bid128_to_int64_floor, 0x3040000000000000000000000000012Du128, 301                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_248, bid128_to_int64_floor, 0x304000000000000000000004A817C7FFu128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_floor_249, bid128_to_int64_floor, 0x304000000000000000000004A817C801u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_floor_250, bid128_to_int64_floor, 0x30400000000000000000200000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_floor_251, bid128_to_int64_floor, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_floor_252, bid128_to_int64_floor, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_int64_floor_253, bid128_to_int64_floor, 0x304000000000000000071AFD498D0000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_floor_254, bid128_to_int64_floor, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_floor_255, bid128_to_int64_floor, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_int64_floor_256, bid128_to_int64_floor, 0x30400000000000000100000009040080u128, 72057594189185152   , 0x00);
dec_test!(bid128_to_int64_floor_257, bid128_to_int64_floor, 0x30400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_floor_258, bid128_to_int64_floor, 0x30400000000000008000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_floor_259, bid128_to_int64_floor, 0x30400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_floor_260, bid128_to_int64_floor, 0x3040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_floor_261, bid128_to_int64_floor, 0x30400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_floor_262, bid128_to_int64_floor, 0x30400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_floor_263, bid128_to_int64_floor, 0x3041ED09BEAD87C0378D8E6400000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_264, bid128_to_int64_floor, 0x3042000000000000000000000000001Du128, 290                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_floor_265, bid128_to_int64_floor, 0x3042000000000000000000000000001Eu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_266, bid128_to_int64_floor, 0x3042000000000000000000000000001Fu128, 310                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_floor_267, bid128_to_int64_floor, 0x304200000000000000000000773593FFu128, 19999999990         , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_int64_floor_268, bid128_to_int64_floor, 0x30420000000000000000000077359400u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_floor_269, bid128_to_int64_floor, 0x30420000000000000000000077359401u128, 20000000010         , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_int64_floor_270, bid128_to_int64_floor, 0x30440000000000000000000000000003u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_floor_271, bid128_to_int64_floor, 0x30520000000000000000000000000004u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_floor_272, bid128_to_int64_floor, 0x30520000000000000000000000000005u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_floor_273, bid128_to_int64_floor, 0x30540000000000000000000000000002u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_floor_274, bid128_to_int64_floor, 0x305E0000000000000000000000000002u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_floor_275, bid128_to_int64_floor, 0x3064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_floor_276, bid128_to_int64_floor, 0x30640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_floor_277, bid128_to_int64_floor, 0x30660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_floor_278, bid128_to_int64_floor, 0x30660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_floor_279, bid128_to_int64_floor, 0x30680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_floor_280, bid128_to_int64_floor, 0x353e0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_281, bid128_to_int64_floor, "5.5"                                 , 5                   , 0x00);
dec_test!(bid128_to_int64_floor_282, bid128_to_int64_floor, 0x57e60000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_283, bid128_to_int64_floor, 0x5c6daa990c7e9d2a4b6e971cbb177379u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_284, bid128_to_int64_floor, 0x768d57ff75df7663e3b5a513648c8939u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_285, bid128_to_int64_floor, 0x78000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_286, bid128_to_int64_floor, 0x78000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_287, bid128_to_int64_floor, 0x79d0a2f7d9c54c7885513e2b9f892bc5u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_288, bid128_to_int64_floor, 0x7c000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_289, bid128_to_int64_floor, 0x7c000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_290, bid128_to_int64_floor, 0x7c003fffffffffff38c15b08ffffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_291, bid128_to_int64_floor, 0x7c003fffffffffff38c15b0affffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_292, bid128_to_int64_floor, 0x7dffdfbcfe9fffff7ef77dc7f9cffa7du128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_293, bid128_to_int64_floor, 0x7e000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_294, bid128_to_int64_floor, 0x8040100000040206ae3068b36208685eu128, -1                  , 0x00);
dec_test!(bid128_to_int64_floor_295, bid128_to_int64_floor, 0x976a832080679c989646ac6640a2a10fu128, -1                  , 0x00);
dec_test!(bid128_to_int64_floor_296, bid128_to_int64_floor, "-9"                                  , -9                  , 0x00);
dec_test!(bid128_to_int64_floor_297, bid128_to_int64_floor, 0xa0ee9059eb92cd6bfe13087e90c3a73du128, -1                  , 0x00);
dec_test!(bid128_to_int64_floor_298, bid128_to_int64_floor, 0xa49b29393479a403143d85612bb2e11bu128, -1                  , 0x00);
dec_test!(bid128_to_int64_floor_299, bid128_to_int64_floor, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, -1                  , 0x00); // -- -(0.5-ulp)
dec_test!(bid128_to_int64_floor_300, bid128_to_int64_floor, 0xAFFCF684DF56C3E01BC6C73200000000u128, -1                  , 0x00); // -- -(0.5)
dec_test!(bid128_to_int64_floor_301, bid128_to_int64_floor, 0xAFFCF684DF56C3E01BC6C73200000001u128, -1                  , 0x00); // -- -(0.5+ulp)
dec_test!(bid128_to_int64_floor_302, bid128_to_int64_floor, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, -1                  , 0x00); // -- -(0.999-ulp)
dec_test!(bid128_to_int64_floor_303, bid128_to_int64_floor, 0xAFFDEC8B86EF679D76FC433D80000000u128, -1                  , 0x00); // -- -(0.999)
dec_test!(bid128_to_int64_floor_304, bid128_to_int64_floor, 0xAFFDEC8B86EF679D76FC433D80000001u128, -1                  , 0x00); // -- -(0.999+ulp)
dec_test!(bid128_to_int64_floor_305, bid128_to_int64_floor, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, -1                  , 0x00); // -- -(1-ulp)
dec_test!(bid128_to_int64_floor_306, bid128_to_int64_floor, 0xAFFE314DC6448D9338C15B0A00000000u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_floor_307, bid128_to_int64_floor, 0xAFFE314DC6448D9338C15B0A00000001u128, -2                  , 0x00); // -- -(1+ulp)
dec_test!(bid128_to_int64_floor_308, bid128_to_int64_floor, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -2                  , 0x00); // -- -(1.5-ulp)
dec_test!(bid128_to_int64_floor_309, bid128_to_int64_floor, 0xAFFE49F4A966D45CD522088F00000000u128, -2                  , 0x00); // -- -(1.5)
dec_test!(bid128_to_int64_floor_310, bid128_to_int64_floor, 0xAFFE49F4A966D45CD522088F00000001u128, -2                  , 0x00); // -- -(1.5+ulp)
dec_test!(bid128_to_int64_floor_311, bid128_to_int64_floor, 0xafff7ff98545f3ca0d80004690400008u128, -8                  , 0x00);
dec_test!(bid128_to_int64_floor_312, bid128_to_int64_floor, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_313, bid128_to_int64_floor, 0xB00293E952CDA8B9AA44111E00000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_314, bid128_to_int64_floor, 0xB00293E952CDA8B9AA44111E00000001u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_315, bid128_to_int64_floor, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -301                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_floor_316, bid128_to_int64_floor, 0xB00294286EACB8CB0A8CB6B140000000u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_floor_317, bid128_to_int64_floor, 0xB00294286EACB8CB0A8CB6B140000001u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_floor_318, bid128_to_int64_floor, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_319, bid128_to_int64_floor, 0xB0040ECA8847C4129106CE8300000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_320, bid128_to_int64_floor, 0xB0040ECA8847C4129106CE8300000001u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_321, bid128_to_int64_floor, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_322, bid128_to_int64_floor, 0xB00A0003C95A2F0B4856475FE0000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_323, bid128_to_int64_floor, 0xB00A0003C95A2F0B4856475FE0000001u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_324, bid128_to_int64_floor, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_325, bid128_to_int64_floor, 0xB00C000060EF6B1ABA6F072330000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_326, bid128_to_int64_floor, 0xB00C000060EF6B1ABA6F072330000001u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_327, bid128_to_int64_floor, 0xB010C5371912364CE3056C27FFFFFFFFu128, -4000000000         , 0x00); // -- -(4e9-ulp)
dec_test!(bid128_to_int64_floor_328, bid128_to_int64_floor, 0xB010C5371912364CE3056C2800000000u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_floor_329, bid128_to_int64_floor, 0xB010C5371912364CE3056C2800000001u128, -4000000001         , 0x00); // -- -(4e9+ulp)
dec_test!(bid128_to_int64_floor_330, bid128_to_int64_floor, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -5000000000         , 0x00); // -- -(5e9-ulp)
dec_test!(bid128_to_int64_floor_331, bid128_to_int64_floor, 0xB010F684DF56C3E01BC6C73200000000u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_floor_332, bid128_to_int64_floor, 0xB010F684DF56C3E01BC6C73200000001u128, -5000000001         , 0x00); // -- -(5e9+ulp)
dec_test!(bid128_to_int64_floor_333, bid128_to_int64_floor, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -19999999999        , 0x00); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_floor_334, bid128_to_int64_floor, 0xB012629B8C88FB62ED56E4238E400000u128, -19999999999        , 0x00); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_floor_335, bid128_to_int64_floor, 0xB012629B8C88FB62ED56E4238E400001u128, -19999999999        , 0x00); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_floor_336, bid128_to_int64_floor, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -19999999999        , 0x00); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_floor_337, bid128_to_int64_floor, 0xB012629B8C8905F96EBAD4C909800000u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_floor_338, bid128_to_int64_floor, 0xB012629B8C8905F96EBAD4C909800001u128, -20000000000        , 0x00); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_floor_339, bid128_to_int64_floor, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -20000000000        , 0x00); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_floor_340, bid128_to_int64_floor, 0xB012629B8C89108FF01EC56E84C00000u128, -20000000000        , 0x00); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_floor_341, bid128_to_int64_floor, 0xB012629B8C89108FF01EC56E84C00001u128, -20000000000        , 0x00); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_floor_342, bid128_to_int64_floor, 0xB012629B8C891B267182B613FFFFFFFFu128, -20000000000        , 0x00); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_floor_343, bid128_to_int64_floor, 0xB012629B8C891B267182B61400000000u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_floor_344, bid128_to_int64_floor, 0xB012629B8C891B267182B61400000001u128, -20000000001        , 0x00); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_floor_345, bid128_to_int64_floor, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -20000000001        , 0x00); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_floor_346, bid128_to_int64_floor, 0xB012629B8C8925BCF2E6A6B97B400000u128, -20000000001        , 0x00); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_floor_347, bid128_to_int64_floor, 0xB012629B8C8925BCF2E6A6B97B400001u128, -20000000001        , 0x00); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_floor_348, bid128_to_int64_floor, 0xB012629B8C893053744A975EF67FFFFFu128, -20000000001        , 0x00); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_floor_349, bid128_to_int64_floor, 0xB012629B8C893053744A975EF6800000u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_floor_350, bid128_to_int64_floor, 0xB012629B8C893053744A975EF6800001u128, -20000000002        , 0x00); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_floor_351, bid128_to_int64_floor, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -20000000002        , 0x00); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_floor_352, bid128_to_int64_floor, 0xB012629B8C893AE9F5AE880471C00000u128, -20000000002        , 0x00); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_floor_353, bid128_to_int64_floor, 0xB012629B8C893AE9F5AE880471C00001u128, -20000000002        , 0x00); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_floor_354, bid128_to_int64_floor, 0xb015e792ed14b858f4736de2399e740fu128, -988917416171       , 0x00);
dec_test!(bid128_to_int64_floor_355, bid128_to_int64_floor, 0xB018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, -35184372088832     , 0x00); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_floor_356, bid128_to_int64_floor, 0xB018AD78EBC5AC620000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_floor_357, bid128_to_int64_floor, 0xB018AD78EBC5AC620000000000000001u128, -35184372088833     , 0x00); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_floor_358, bid128_to_int64_floor, 0xB018AD78EBC5AC64B5E3AF16B187FFFFu128, -35184372088833     , 0x00); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_floor_359, bid128_to_int64_floor, 0xB018AD78EBC5AC64B5E3AF16B1880000u128, -35184372088833     , 0x00); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_floor_360, bid128_to_int64_floor, 0xB018AD78EBC5AC64B5E3AF16B1880001u128, -35184372088833     , 0x00); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_floor_361, bid128_to_int64_floor, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -301                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_floor_362, bid128_to_int64_floor, 0xB01A0000000000A2E6C09AD3E0D40000u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_floor_363, bid128_to_int64_floor, 0xB01A0000000000A2E6C09AD3E0D40001u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_floor_364, bid128_to_int64_floor, 0xB01C629B8C891B265CB1A40684E9FFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_floor_365, bid128_to_int64_floor, 0xB01C629B8C891B265CB1A40684EA0000u128, -1999999999999999   , 0x00); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_floor_366, bid128_to_int64_floor, 0xB01C629B8C891B265CB1A40684EA0001u128, -1999999999999999   , 0x00); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_floor_367, bid128_to_int64_floor, 0xB01C629B8C891B2663A1FF60589BFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_floor_368, bid128_to_int64_floor, 0xB01C629B8C891B2663A1FF60589C0000u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_floor_369, bid128_to_int64_floor, 0xB01C629B8C891B2663A1FF60589C0001u128, -2000000000000000   , 0x00); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_floor_370, bid128_to_int64_floor, 0xB01C629B8C891B266A925ABA2C4DFFFFu128, -2000000000000000   , 0x00); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_floor_371, bid128_to_int64_floor, 0xB01C629B8C891B266A925ABA2C4E0000u128, -2000000000000000   , 0x00); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_floor_372, bid128_to_int64_floor, 0xB01C629B8C891B266A925ABA2C4E0001u128, -2000000000000000   , 0x00); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_floor_373, bid128_to_int64_floor, 0xB01C629B8C891B267182B613FFFFFFFFu128, -2000000000000000   , 0x00); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_floor_374, bid128_to_int64_floor, 0xB01C629B8C891B267182B61400000000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_floor_375, bid128_to_int64_floor, 0xB01C629B8C891B267182B61400000001u128, -2000000000000001   , 0x00); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_floor_376, bid128_to_int64_floor, 0xB01C629B8C891B267873116DD3B1FFFFu128, -2000000000000001   , 0x00); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_floor_377, bid128_to_int64_floor, 0xB01C629B8C891B267873116DD3B20000u128, -2000000000000001   , 0x00); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_floor_378, bid128_to_int64_floor, 0xB01C629B8C891B267873116DD3B20001u128, -2000000000000001   , 0x00); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_floor_379, bid128_to_int64_floor, 0xB01C629B8C891B267F636CC7A763FFFFu128, -2000000000000001   , 0x00); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_floor_380, bid128_to_int64_floor, 0xB01C629B8C891B267F636CC7A7640000u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_floor_381, bid128_to_int64_floor, 0xB01C629B8C891B267F636CC7A7640001u128, -2000000000000002   , 0x00); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_floor_382, bid128_to_int64_floor, 0xB01C629B8C891B268653C8217B15FFFFu128, -2000000000000002   , 0x00); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_floor_383, bid128_to_int64_floor, 0xB01C629B8C891B268653C8217B160000u128, -2000000000000002   , 0x00); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_floor_384, bid128_to_int64_floor, 0xB01C629B8C891B268653C8217B160001u128, -2000000000000002   , 0x00); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_floor_385, bid128_to_int64_floor, 0xB01E000000000001A055690D9DB7FFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_386, bid128_to_int64_floor, 0xB01E000000000001A055690D9DB80000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_387, bid128_to_int64_floor, 0xB01E000000000001A055690D9DB80001u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_388, bid128_to_int64_floor, 0xB01E002C68AF0BB140B1A2BC2EC4FFFFu128, -35184372088833     , 0x00); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_floor_389, bid128_to_int64_floor, 0xB01E002C68AF0BB140B1A2BC2EC50000u128, -35184372088833     , 0x00); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_floor_390, bid128_to_int64_floor, 0xB01E002C68AF0BB140B1A2BC2EC50001u128, -35184372088833     , 0x00); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_floor_391, bid128_to_int64_floor, 0xB02000000000000029A2241AF62BFFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_392, bid128_to_int64_floor, 0xB02000000000000029A2241AF62C0000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_393, bid128_to_int64_floor, 0xB02000000000000029A2241AF62C0001u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_394, bid128_to_int64_floor, 0xB020000470DE4DF81FFFFFFFFFFFFFFFu128, -35184372088832     , 0x00); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_floor_395, bid128_to_int64_floor, 0xB020000470DE4DF82000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_floor_396, bid128_to_int64_floor, 0xB020000470DE4DF82000000000000001u128, -35184372088833     , 0x00); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_floor_397, bid128_to_int64_floor, 0xB02000FC6F7C4045813459C637E07FFFu128, -2000000000000001   , 0x00); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_floor_398, bid128_to_int64_floor, 0xB02000FC6F7C4045813459C637E08000u128, -2000000000000001   , 0x00); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_floor_399, bid128_to_int64_floor, 0xB02000FC6F7C4045813459C637E08001u128, -2000000000000001   , 0x00); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_floor_400, bid128_to_int64_floor, 0xB02000FC6F7C40458157E0B8A7A17FFFu128, -2000000000000002   , 0x00); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_floor_401, bid128_to_int64_floor, 0xB02000FC6F7C40458157E0B8A7A18000u128, -2000000000000002   , 0x00); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_floor_402, bid128_to_int64_floor, 0xB02000FC6F7C40458157E0B8A7A18001u128, -2000000000000002   , 0x00); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_floor_403, bid128_to_int64_floor, 0xB02200193E5939A08CE4879688D63FFFu128, -1999999999999999   , 0x00); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_floor_404, bid128_to_int64_floor, 0xB02200193E5939A08CE4879688D64000u128, -1999999999999999   , 0x00); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_floor_405, bid128_to_int64_floor, 0xB02200193E5939A08CE4879688D64001u128, -1999999999999999   , 0x00); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_floor_406, bid128_to_int64_floor, 0xB02200193E5939A08CE815152D9CBFFFu128, -2000000000000000   , 0x00); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_floor_407, bid128_to_int64_floor, 0xB02200193E5939A08CE815152D9CC000u128, -2000000000000000   , 0x00); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_floor_408, bid128_to_int64_floor, 0xB02200193E5939A08CE815152D9CC001u128, -2000000000000000   , 0x00); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_floor_409, bid128_to_int64_floor, 0xb023b54c2ac54b422a0c10410042dc10u128, -8869447574031061027, 0x00);
dec_test!(bid128_to_int64_floor_410, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFAABC208D63FFFu128, -9223372036854775807, 0x00); // -- -(2^63-1.5-ulp)
dec_test!(bid128_to_int64_floor_411, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFAABC208D64000u128, -9223372036854775807, 0x00); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_floor_412, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFAABC208D64001u128, -9223372036854775807, 0x00); // -- -(2^63-1.5+ulp)
dec_test!(bid128_to_int64_floor_413, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFC72815B397FFFu128, -9223372036854775807, 0x00); // -- -(2^63-1-ulp)
dec_test!(bid128_to_int64_floor_414, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFC72815B398000u128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_floor_415, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFC72815B398001u128, -9223372036854775808, 0x00); // -- -(2^63-1+ulp)
dec_test!(bid128_to_int64_floor_416, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFE3940AD9CBFFFu128, -9223372036854775808, 0x00); // -- -(2^63-0.5-ulp)
dec_test!(bid128_to_int64_floor_417, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFE3940AD9CC000u128, -9223372036854775808, 0x00); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_floor_418, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFE3940AD9CC001u128, -9223372036854775808, 0x00); // -- -(2^63-0.5+ulp)
dec_test!(bid128_to_int64_floor_419, bid128_to_int64_floor, 0xB023C6BF52633FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x00); // -- -(2^63-ulp)
dec_test!(bid128_to_int64_floor_420, bid128_to_int64_floor, 0xB023C6BF526340000000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_floor_421, bid128_to_int64_floor, 0xB023C6BF526340000000000000000001u128, -9223372036854775808, 0x01); // -- -(2^63+ulp)
dec_test!(bid128_to_int64_floor_422, bid128_to_int64_floor, 0xB023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x01); // -- -(2^63+0.5-ulp)
dec_test!(bid128_to_int64_floor_423, bid128_to_int64_floor, 0xB023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_floor_424, bid128_to_int64_floor, 0xB023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- -(2^63+0.5+ulp)
dec_test!(bid128_to_int64_floor_425, bid128_to_int64_floor, 0xB023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- -(2^63+1-ulp)
dec_test!(bid128_to_int64_floor_426, bid128_to_int64_floor, 0xB023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_floor_427, bid128_to_int64_floor, 0xB023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- -(2^63+1+ulp)
dec_test!(bid128_to_int64_floor_428, bid128_to_int64_floor, 0xb023e3babba2257f00d7819973b0ce1du128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_429, bid128_to_int64_floor, 0xB024000000000000006A94D74F42FFFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_430, bid128_to_int64_floor, 0xB024000000000000006A94D74F430000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_431, bid128_to_int64_floor, 0xB024000000000000006A94D74F430001u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_432, bid128_to_int64_floor, 0xB024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e19-ulp)
dec_test!(bid128_to_int64_floor_433, bid128_to_int64_floor, 0xB024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_floor_434, bid128_to_int64_floor, 0xB024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e19+ulp)
dec_test!(bid128_to_int64_floor_435, bid128_to_int64_floor, 0xB024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- -(1e19+0.5-ulp)
dec_test!(bid128_to_int64_floor_436, bid128_to_int64_floor, 0xB024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_floor_437, bid128_to_int64_floor, 0xB024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- -(1e19+0.5+ulp)
dec_test!(bid128_to_int64_floor_438, bid128_to_int64_floor, 0xB02449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1.5e19-ulp)
dec_test!(bid128_to_int64_floor_439, bid128_to_int64_floor, 0xB02449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_floor_440, bid128_to_int64_floor, 0xB02449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- -(1.5e19+ulp)
dec_test!(bid128_to_int64_floor_441, bid128_to_int64_floor, 0xB0245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1-ulp)
dec_test!(bid128_to_int64_floor_442, bid128_to_int64_floor, 0xB0245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_floor_443, bid128_to_int64_floor, 0xB0245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- -(2^64-1+ulp)
dec_test!(bid128_to_int64_floor_444, bid128_to_int64_floor, 0xB0245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- -(2^64-0.5-ulp)
dec_test!(bid128_to_int64_floor_445, bid128_to_int64_floor, 0xB0245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_floor_446, bid128_to_int64_floor, 0xB0245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- -(2^64-0.5+ulp)
dec_test!(bid128_to_int64_floor_447, bid128_to_int64_floor, 0xB0245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-ulp)
dec_test!(bid128_to_int64_floor_448, bid128_to_int64_floor, 0xB0245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_floor_449, bid128_to_int64_floor, 0xB0245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+ulp)
dec_test!(bid128_to_int64_floor_450, bid128_to_int64_floor, 0xB0245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- -(2^64+0.5-ulp)
dec_test!(bid128_to_int64_floor_451, bid128_to_int64_floor, 0xB0245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_floor_452, bid128_to_int64_floor, 0xB0245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- -(2^64+0.5+ulp)
dec_test!(bid128_to_int64_floor_453, bid128_to_int64_floor, 0xB0245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- -(2^64+1-ulp)
dec_test!(bid128_to_int64_floor_454, bid128_to_int64_floor, 0xB0245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_floor_455, bid128_to_int64_floor, 0xB0245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- -(2^64+1+ulp)
dec_test!(bid128_to_int64_floor_456, bid128_to_int64_floor, 0xB024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2e19-ulp)
dec_test!(bid128_to_int64_floor_457, bid128_to_int64_floor, 0xB024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_floor_458, bid128_to_int64_floor, 0xB024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- -(2e19+ulp)
dec_test!(bid128_to_int64_floor_459, bid128_to_int64_floor, 0xB0247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2.5e19-ulp)
dec_test!(bid128_to_int64_floor_460, bid128_to_int64_floor, 0xB0247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_floor_461, bid128_to_int64_floor, 0xB0247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- -(2.5e19+ulp)
dec_test!(bid128_to_int64_floor_462, bid128_to_int64_floor, 0xB026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e20-ulp)
dec_test!(bid128_to_int64_floor_463, bid128_to_int64_floor, 0xB026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_floor_464, bid128_to_int64_floor, 0xB026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e20+ulp)
dec_test!(bid128_to_int64_floor_465, bid128_to_int64_floor, 0xB02A00000000006C6B935B68D08DA3FFu128, -19999999999        , 0x00); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_floor_466, bid128_to_int64_floor, 0xB02A00000000006C6B935B68D08DA400u128, -19999999999        , 0x00); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_floor_467, bid128_to_int64_floor, 0xB02A00000000006C6B935B68D08DA401u128, -19999999999        , 0x00); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_floor_468, bid128_to_int64_floor, 0xB02A00000000006C6B935B8019048BFFu128, -20000000000        , 0x00); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_floor_469, bid128_to_int64_floor, 0xB02A00000000006C6B935B8019048C00u128, -20000000000        , 0x00); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_floor_470, bid128_to_int64_floor, 0xB02A00000000006C6B935B8019048C01u128, -20000000000        , 0x00); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_floor_471, bid128_to_int64_floor, 0xB02C000000000000000002BBA7F521FFu128, -301                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_floor_472, bid128_to_int64_floor, 0xB02C000000000000000002BBA7F52200u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_floor_473, bid128_to_int64_floor, 0xB02C000000000000000002BBA7F52201u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_floor_474, bid128_to_int64_floor, 0xB02C00000000000AD78EBC5872141BFFu128, -19999999999        , 0x00); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_floor_475, bid128_to_int64_floor, 0xB02C00000000000AD78EBC5872141C00u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_floor_476, bid128_to_int64_floor, 0xB02C00000000000AD78EBC5872141C01u128, -20000000000        , 0x00); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_floor_477, bid128_to_int64_floor, 0xB02C00000000000AD78EBC5BF025F1FFu128, -20000000001        , 0x00); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_floor_478, bid128_to_int64_floor, 0xB02C00000000000AD78EBC5BF025F200u128, -20000000001        , 0x00); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_floor_479, bid128_to_int64_floor, 0xB02C00000000000AD78EBC5BF025F201u128, -20000000001        , 0x00); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_floor_480, bid128_to_int64_floor, 0xB02C00000000000AD78EBC5E4431D5FFu128, -20000000002        , 0x00); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_floor_481, bid128_to_int64_floor, 0xB02C00000000000AD78EBC5E4431D600u128, -20000000002        , 0x00); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_floor_482, bid128_to_int64_floor, 0xB02C00000000000AD78EBC5E4431D601u128, -20000000002        , 0x00); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_floor_483, bid128_to_int64_floor, 0xB02C000000108B2A2C28028E3FF41BFFu128, -1999999999999999   , 0x00); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_floor_484, bid128_to_int64_floor, 0xB02C000000108B2A2C28028E3FF41C00u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_floor_485, bid128_to_int64_floor, 0xB02C000000108B2A2C28028E3FF41C01u128, -2000000000000000   , 0x00); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_floor_486, bid128_to_int64_floor, 0xB02E000000000001158E46094F6AC9FFu128, -20000000001        , 0x00); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_floor_487, bid128_to_int64_floor, 0xB02E000000000001158E46094F6ACA00u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_floor_488, bid128_to_int64_floor, 0xB02E000000000001158E46094F6ACA01u128, -20000000002        , 0x00); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_floor_489, bid128_to_int64_floor, 0xB02E00000001A784379D99DB7D9AC9FFu128, -2000000000000001   , 0x00); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_floor_490, bid128_to_int64_floor, 0xB02E00000001A784379D99DB7D9ACA00u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_floor_491, bid128_to_int64_floor, 0xB02E00000001A784379D99DB7D9ACA01u128, -2000000000000002   , 0x00); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_floor_492, bid128_to_int64_floor, 0xB03000000000000000000006FC23ABFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_493, bid128_to_int64_floor, 0xB03000000000000000000006FC23AC00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_494, bid128_to_int64_floor, 0xB03000000000000000000006FC23AC01u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_495, bid128_to_int64_floor, 0xB03200000000000000000000B2D05DFFu128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_496, bid128_to_int64_floor, 0xB03200000000000000000000B2D05E00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_497, bid128_to_int64_floor, 0xB03200000000000000000000B2D05E01u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_498, bid128_to_int64_floor, 0xB03800000000000000000000002DDA47u128, -301                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_floor_499, bid128_to_int64_floor, 0xB03800000000000000000000002DDA48u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_floor_500, bid128_to_int64_floor, 0xB03800000000000000000000002DDA49u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_floor_501, bid128_to_int64_floor, 0xB03A00000000000000000000000003E7u128, -1                  , 0x00); // -- -(0.999)
dec_test!(bid128_to_int64_floor_502, bid128_to_int64_floor, 0xB03A00000000000000000000000495D3u128, -301                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_floor_503, bid128_to_int64_floor, 0xB03A00000000000000000000000495D4u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_floor_504, bid128_to_int64_floor, 0xB03A00000000000000000000000495D5u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_floor_505, bid128_to_int64_floor, 0xB03C0000000000000000000000007561u128, -301                , 0x00); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_floor_506, bid128_to_int64_floor, 0xB03C0000000000000000000000007562u128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_floor_507, bid128_to_int64_floor, 0xB03C0000000000000000000000007563u128, -301                , 0x00); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_floor_508, bid128_to_int64_floor, 0xB03E0000000000000000000000000005u128, -1                  , 0x00); // -- -(0.5)
dec_test!(bid128_to_int64_floor_509, bid128_to_int64_floor, 0xB03E000000000000000000000000000Fu128, -2                  , 0x00); // -- -(1.5)
dec_test!(bid128_to_int64_floor_510, bid128_to_int64_floor, 0xB03E0000000000000000000000000BB7u128, -300                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_511, bid128_to_int64_floor, 0xB03E0000000000000000000000000BB8u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_512, bid128_to_int64_floor, 0xB03E0000000000000000000000000BB9u128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_513, bid128_to_int64_floor, 0xB03E0000000000000000000000000BBDu128, -301                , 0x00); // -- -(300.5)
dec_test!(bid128_to_int64_floor_514, bid128_to_int64_floor, 0xB03E0000000000000000002E90EDCFF1u128, -19999999999        , 0x00); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_floor_515, bid128_to_int64_floor, 0xB03E0000000000000000002E90EDCFFBu128, -20000000000        , 0x00); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_floor_516, bid128_to_int64_floor, 0xB03E0000000000000000002E90EDD005u128, -20000000001        , 0x00); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_floor_517, bid128_to_int64_floor, 0xB03E0000000000000000002E90EDD00Fu128, -20000000002        , 0x00); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_floor_518, bid128_to_int64_floor, 0xB03E0000000000000001400000000005u128, -35184372088833     , 0x00); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_floor_519, bid128_to_int64_floor, 0xB03E00000000000000470DE4DF81FFF1u128, -1999999999999999   , 0x00); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_floor_520, bid128_to_int64_floor, 0xB03E00000000000000470DE4DF81FFFBu128, -2000000000000000   , 0x00); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_floor_521, bid128_to_int64_floor, 0xB03E00000000000000470DE4DF820005u128, -2000000000000001   , 0x00); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_floor_522, bid128_to_int64_floor, 0xB03E00000000000000470DE4DF82000Fu128, -2000000000000002   , 0x00); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_floor_523, bid128_to_int64_floor, 0xB03E000000000004FFFFFFFFFFFFFFF1u128, -9223372036854775807, 0x00); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_floor_524, bid128_to_int64_floor, 0xB03E000000000004FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x00); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_floor_525, bid128_to_int64_floor, 0xB03E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_floor_526, bid128_to_int64_floor, 0xB03E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_floor_527, bid128_to_int64_floor, 0xB03E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_floor_528, bid128_to_int64_floor, 0xB03E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_floor_529, bid128_to_int64_floor, 0xB0400000000000000000000000000001u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_floor_530, bid128_to_int64_floor, 0xB040000000000000000000000000012Bu128, -299                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_531, bid128_to_int64_floor, 0xB040000000000000000000000000012Cu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_532, bid128_to_int64_floor, 0xB040000000000000000000000000012Du128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_533, bid128_to_int64_floor, 0xB04000000000000000000004A817C7FFu128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_floor_534, bid128_to_int64_floor, 0xB04000000000000000000004A817C801u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_floor_535, bid128_to_int64_floor, 0xB0400000000000000000200000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_floor_536, bid128_to_int64_floor, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_floor_537, bid128_to_int64_floor, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_floor_538, bid128_to_int64_floor, 0xB04000000000000000071AFD498D0000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_floor_539, bid128_to_int64_floor, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_floor_540, bid128_to_int64_floor, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_floor_541, bid128_to_int64_floor, 0xB0400000000000007FFFFFFFFFFFFFFFu128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_floor_542, bid128_to_int64_floor, 0xB0400000000000008000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_floor_543, bid128_to_int64_floor, 0xB0400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_floor_544, bid128_to_int64_floor, 0xB040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_floor_545, bid128_to_int64_floor, 0xB0400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_floor_546, bid128_to_int64_floor, 0xB0400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_floor_547, bid128_to_int64_floor, 0xB042000000000000000000000000001Du128, -290                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_floor_548, bid128_to_int64_floor, 0xB042000000000000000000000000001Eu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_549, bid128_to_int64_floor, 0xB042000000000000000000000000001Fu128, -310                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_floor_550, bid128_to_int64_floor, 0xB04200000000000000000000773593FFu128, -19999999990        , 0x00); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_floor_551, bid128_to_int64_floor, 0xB0420000000000000000000077359400u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_floor_552, bid128_to_int64_floor, 0xB0420000000000000000000077359401u128, -20000000010        , 0x00); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_floor_553, bid128_to_int64_floor, 0xB0440000000000000000000000000003u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_floor_554, bid128_to_int64_floor, 0xB0520000000000000000000000000004u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_floor_555, bid128_to_int64_floor, 0xB0520000000000000000000000000005u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_floor_556, bid128_to_int64_floor, 0xB0540000000000000000000000000002u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_floor_557, bid128_to_int64_floor, 0xB05E0000000000000000000000000002u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_floor_558, bid128_to_int64_floor, 0xB064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_floor_559, bid128_to_int64_floor, 0xB0640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_floor_560, bid128_to_int64_floor, 0xB0660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_floor_561, bid128_to_int64_floor, 0xB0660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_floor_562, bid128_to_int64_floor, 0xB0680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_floor_563, bid128_to_int64_floor, 0xcf6a234572acfddf60e6cecb3bc03259u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_564, bid128_to_int64_floor, 0xd3ce237ccd4ba18c58b7811bb7b5db0eu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_565, bid128_to_int64_floor, 0xd51c0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_floor_566, bid128_to_int64_floor, 0xd9430a17fe1c9bdef1725e5a5b636742u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_567, bid128_to_int64_floor, 0xde0c2e5b9aa9a10ff139c18087a5a22du128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_568, bid128_to_int64_floor, 0xf8000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_569, bid128_to_int64_floor, 0xfac96549d4ec7a0d8c57479fbe2d5f69u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_570, bid128_to_int64_floor, 0xffcddb0ecf7d7e690000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_571, bid128_to_int64_floor, "Infinity"                              , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_floor_572, bid128_to_int64_floor, "SNaN"                                  , -9223372036854775808, 0x01);
