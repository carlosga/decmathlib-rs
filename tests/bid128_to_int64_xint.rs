/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_int64_xint_001, bid128_to_int64_xint, "-0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_xint_002, bid128_to_int64_xint,  "0"                                  , 0                   , 0x00);
dec_test!(bid128_to_int64_xint_003, bid128_to_int64_xint, 0x00000000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xint_004, bid128_to_int64_xint, 0x00000000000000000000008000000810u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_005, bid128_to_int64_xint, 0x0001ed09bead87c0378d8e62ffffffffu128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_006, bid128_to_int64_xint, 0x0001ed09bead87c0378d8e64ffffffffu128, 0                   , 0x00);
dec_test!(bid128_to_int64_xint_007, bid128_to_int64_xint, 0x042a79109ddb140a878aa59ddad35443u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_008, bid128_to_int64_xint, 0x2002000000000000ffff7ffffffffbffu128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_009, bid128_to_int64_xint, 0x285101d9dbffbc06b1394fae0096b900u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_010, bid128_to_int64_xint, 0x2FFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x20); // -- 0.5-ulp
dec_test!(bid128_to_int64_xint_011, bid128_to_int64_xint, 0x2FFCF684DF56C3E01BC6C73200000000u128, 0                   , 0x20); // -- 0.5
dec_test!(bid128_to_int64_xint_012, bid128_to_int64_xint, 0x2FFCF684DF56C3E01BC6C73200000001u128, 0                   , 0x20); // -- 0.5+ulp
dec_test!(bid128_to_int64_xint_013, bid128_to_int64_xint, 0x2FFDEC8B86EF679D76FC433D7FFFFFFFu128, 0                   , 0x20); // -- 0.999-ulp
dec_test!(bid128_to_int64_xint_014, bid128_to_int64_xint, 0x2FFDEC8B86EF679D76FC433D80000000u128, 0                   , 0x20); // -- 0.999
dec_test!(bid128_to_int64_xint_015, bid128_to_int64_xint, 0x2FFDEC8B86EF679D76FC433D80000001u128, 0                   , 0x20); // -- 0.999+ulp
dec_test!(bid128_to_int64_xint_016, bid128_to_int64_xint, 0x2FFE314DC6448D9338C15B09FFFFFFFFu128, 0                   , 0x20); // -- 1-ulp
dec_test!(bid128_to_int64_xint_017, bid128_to_int64_xint, 0x2FFE314DC6448D9338C15B0A00000000u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_xint_018, bid128_to_int64_xint, 0x2FFE314DC6448D9338C15B0A00000001u128, 1                   , 0x20); // -- 1+ulp
dec_test!(bid128_to_int64_xint_019, bid128_to_int64_xint, 0x2FFE49F4A966D45CD522088EFFFFFFFFu128, 1                   , 0x20); // -- 1.5-ulp
dec_test!(bid128_to_int64_xint_020, bid128_to_int64_xint, 0x2FFE49F4A966D45CD522088F00000000u128, 1                   , 0x20); // -- 1.5
dec_test!(bid128_to_int64_xint_021, bid128_to_int64_xint, 0x2FFE49F4A966D45CD522088F00000001u128, 1                   , 0x20); // -- 1.5+ulp
dec_test!(bid128_to_int64_xint_022, bid128_to_int64_xint, 0x300041c848908032e954a1efad5abe7bu128, 13                  , 0x20);
dec_test!(bid128_to_int64_xint_023, bid128_to_int64_xint, 0x300293E952CDA8B9AA44111DFFFFFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_024, bid128_to_int64_xint, 0x300293E952CDA8B9AA44111E00000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_025, bid128_to_int64_xint, 0x300293E952CDA8B9AA44111E00000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_026, bid128_to_int64_xint, 0x300294286EACB8CB0A8CB6B13FFFFFFFu128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xint_027, bid128_to_int64_xint, 0x300294286EACB8CB0A8CB6B140000000u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xint_028, bid128_to_int64_xint, 0x300294286EACB8CB0A8CB6B140000001u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xint_029, bid128_to_int64_xint, 0x30040ECA8847C4129106CE82FFFFFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_030, bid128_to_int64_xint, 0x30040ECA8847C4129106CE8300000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_031, bid128_to_int64_xint, 0x30040ECA8847C4129106CE8300000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_032, bid128_to_int64_xint, 0x300A0003C95A2F0B4856475FDFFFFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_033, bid128_to_int64_xint, 0x300A0003C95A2F0B4856475FE0000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_034, bid128_to_int64_xint, 0x300A0003C95A2F0B4856475FE0000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_035, bid128_to_int64_xint, 0x300C000060EF6B1ABA6F07232FFFFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_036, bid128_to_int64_xint, 0x300C000060EF6B1ABA6F072330000000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_037, bid128_to_int64_xint, 0x300C000060EF6B1ABA6F072330000001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_038, bid128_to_int64_xint, 0x3010C5371912364CE3056C27FFFFFFFFu128, 3999999999          , 0x20); // -- 4e9-ulp
dec_test!(bid128_to_int64_xint_039, bid128_to_int64_xint, 0x3010C5371912364CE3056C2800000000u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_xint_040, bid128_to_int64_xint, 0x3010C5371912364CE3056C2800000001u128, 4000000000          , 0x20); // -- 4e9+ulp
dec_test!(bid128_to_int64_xint_041, bid128_to_int64_xint, 0x3010F684DF56C3E01BC6C731FFFFFFFFu128, 4999999999          , 0x20); // -- 5e9-ulp
dec_test!(bid128_to_int64_xint_042, bid128_to_int64_xint, 0x3010F684DF56C3E01BC6C73200000000u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_xint_043, bid128_to_int64_xint, 0x3010F684DF56C3E01BC6C73200000001u128, 5000000000          , 0x20); // -- 5e9+ulp
dec_test!(bid128_to_int64_xint_044, bid128_to_int64_xint, 0x3012629B8C88FB62ED56E4238E3FFFFFu128, 19999999998         , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_xint_045, bid128_to_int64_xint, 0x3012629B8C88FB62ED56E4238E400000u128, 19999999998         , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_int64_xint_046, bid128_to_int64_xint, 0x3012629B8C88FB62ED56E4238E400001u128, 19999999998         , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_xint_047, bid128_to_int64_xint, 0x3012629B8C8905F96EBAD4C9097FFFFFu128, 19999999998         , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_xint_048, bid128_to_int64_xint, 0x3012629B8C8905F96EBAD4C909800000u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_xint_049, bid128_to_int64_xint, 0x3012629B8C8905F96EBAD4C909800001u128, 19999999999         , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_xint_050, bid128_to_int64_xint, 0x3012629B8C89108FF01EC56E84BFFFFFu128, 19999999999         , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_xint_051, bid128_to_int64_xint, 0x3012629B8C89108FF01EC56E84C00000u128, 19999999999         , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_int64_xint_052, bid128_to_int64_xint, 0x3012629B8C89108FF01EC56E84C00001u128, 19999999999         , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_xint_053, bid128_to_int64_xint, 0x3012629B8C891B267182B613FFFFFFFFu128, 19999999999         , 0x20); // -- 2e10-ulp
dec_test!(bid128_to_int64_xint_054, bid128_to_int64_xint, 0x3012629B8C891B267182B61400000000u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_xint_055, bid128_to_int64_xint, 0x3012629B8C891B267182B61400000001u128, 20000000000         , 0x20); // -- 2e10+ulp
dec_test!(bid128_to_int64_xint_056, bid128_to_int64_xint, 0x3012629B8C8925BCF2E6A6B97B3FFFFFu128, 20000000000         , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_xint_057, bid128_to_int64_xint, 0x3012629B8C8925BCF2E6A6B97B400000u128, 20000000000         , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_int64_xint_058, bid128_to_int64_xint, 0x3012629B8C8925BCF2E6A6B97B400001u128, 20000000000         , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_xint_059, bid128_to_int64_xint, 0x3012629B8C893053744A975EF67FFFFFu128, 20000000000         , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_xint_060, bid128_to_int64_xint, 0x3012629B8C893053744A975EF6800000u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_xint_061, bid128_to_int64_xint, 0x3012629B8C893053744A975EF6800001u128, 20000000001         , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_xint_062, bid128_to_int64_xint, 0x3012629B8C893AE9F5AE880471BFFFFFu128, 20000000001         , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_xint_063, bid128_to_int64_xint, 0x3012629B8C893AE9F5AE880471C00000u128, 20000000001         , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_int64_xint_064, bid128_to_int64_xint, 0x3012629B8C893AE9F5AE880471C00001u128, 20000000001         , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_xint_065, bid128_to_int64_xint, 0x3018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, 35184372088831      , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_int64_xint_066, bid128_to_int64_xint, 0x3018AD78EBC5AC620000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_xint_067, bid128_to_int64_xint, 0x3018AD78EBC5AC620000000000000001u128, 35184372088832      , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_int64_xint_068, bid128_to_int64_xint, 0x3018AD78EBC5AC64B5E3AF16B187FFFFu128, 35184372088832      , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_xint_069, bid128_to_int64_xint, 0x3018AD78EBC5AC64B5E3AF16B1880000u128, 35184372088832      , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_int64_xint_070, bid128_to_int64_xint, 0x3018AD78EBC5AC64B5E3AF16B1880001u128, 35184372088832      , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_xint_071, bid128_to_int64_xint, 0x301A0000000000A2E6C09AD3E0D3FFFFu128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xint_072, bid128_to_int64_xint, 0x301A0000000000A2E6C09AD3E0D40000u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xint_073, bid128_to_int64_xint, 0x301A0000000000A2E6C09AD3E0D40001u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xint_074, bid128_to_int64_xint, 0x301C629B8C891B265CB1A40684E9FFFFu128, 1999999999999998    , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_xint_075, bid128_to_int64_xint, 0x301C629B8C891B265CB1A40684EA0000u128, 1999999999999998    , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_int64_xint_076, bid128_to_int64_xint, 0x301C629B8C891B265CB1A40684EA0001u128, 1999999999999998    , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_xint_077, bid128_to_int64_xint, 0x301C629B8C891B2663A1FF60589BFFFFu128, 1999999999999998    , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_xint_078, bid128_to_int64_xint, 0x301C629B8C891B2663A1FF60589C0000u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_xint_079, bid128_to_int64_xint, 0x301C629B8C891B2663A1FF60589C0001u128, 1999999999999999    , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_xint_080, bid128_to_int64_xint, 0x301C629B8C891B266A925ABA2C4DFFFFu128, 1999999999999999    , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_xint_081, bid128_to_int64_xint, 0x301C629B8C891B266A925ABA2C4E0000u128, 1999999999999999    , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_int64_xint_082, bid128_to_int64_xint, 0x301C629B8C891B266A925ABA2C4E0001u128, 1999999999999999    , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_xint_083, bid128_to_int64_xint, 0x301C629B8C891B267182B613FFFFFFFFu128, 1999999999999999    , 0x20); // -- 2e15-ulp
dec_test!(bid128_to_int64_xint_084, bid128_to_int64_xint, 0x301C629B8C891B267182B61400000000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_xint_085, bid128_to_int64_xint, 0x301C629B8C891B267182B61400000001u128, 2000000000000000    , 0x20); // -- 2e15+ulp
dec_test!(bid128_to_int64_xint_086, bid128_to_int64_xint, 0x301C629B8C891B267873116DD3B1FFFFu128, 2000000000000000    , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_xint_087, bid128_to_int64_xint, 0x301C629B8C891B267873116DD3B20000u128, 2000000000000000    , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_int64_xint_088, bid128_to_int64_xint, 0x301C629B8C891B267873116DD3B20001u128, 2000000000000000    , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_xint_089, bid128_to_int64_xint, 0x301C629B8C891B267F636CC7A763FFFFu128, 2000000000000000    , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_xint_090, bid128_to_int64_xint, 0x301C629B8C891B267F636CC7A7640000u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_xint_091, bid128_to_int64_xint, 0x301C629B8C891B267F636CC7A7640001u128, 2000000000000001    , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_xint_092, bid128_to_int64_xint, 0x301C629B8C891B268653C8217B15FFFFu128, 2000000000000001    , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_xint_093, bid128_to_int64_xint, 0x301C629B8C891B268653C8217B160000u128, 2000000000000001    , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_int64_xint_094, bid128_to_int64_xint, 0x301C629B8C891B268653C8217B160001u128, 2000000000000001    , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_xint_095, bid128_to_int64_xint, 0x301E000000000001A055690D9DB7FFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_096, bid128_to_int64_xint, 0x301E000000000001A055690D9DB80000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_097, bid128_to_int64_xint, 0x301E000000000001A055690D9DB80001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_098, bid128_to_int64_xint, 0x301E002C68AF0BB140B1A2BC2EC4FFFFu128, 35184372088832      , 0x20); // -- 2^45+0.5-ulp
dec_test!(bid128_to_int64_xint_099, bid128_to_int64_xint, 0x301E002C68AF0BB140B1A2BC2EC50000u128, 35184372088832      , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_int64_xint_100, bid128_to_int64_xint, 0x301E002C68AF0BB140B1A2BC2EC50001u128, 35184372088832      , 0x20); // -- 2^45+0.5+ulp
dec_test!(bid128_to_int64_xint_101, bid128_to_int64_xint, 0x302000000000000029A2241AF62BFFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_102, bid128_to_int64_xint, 0x302000000000000029A2241AF62C0000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_103, bid128_to_int64_xint, 0x302000000000000029A2241AF62C0001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_104, bid128_to_int64_xint, 0x3020000470DE4DF81FFFFFFFFFFFFFFFu128, 35184372088831      , 0x20); // -- 2^45-ulp
dec_test!(bid128_to_int64_xint_105, bid128_to_int64_xint, 0x3020000470DE4DF82000000000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_xint_106, bid128_to_int64_xint, 0x3020000470DE4DF82000000000000001u128, 35184372088832      , 0x20); // -- 2^45+ulp
dec_test!(bid128_to_int64_xint_107, bid128_to_int64_xint, 0x302000FC6F7C4045813459C637E07FFFu128, 2000000000000000    , 0x20); // -- 2e15+0.5-ulp
dec_test!(bid128_to_int64_xint_108, bid128_to_int64_xint, 0x302000FC6F7C4045813459C637E08000u128, 2000000000000000    , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_int64_xint_109, bid128_to_int64_xint, 0x302000FC6F7C4045813459C637E08001u128, 2000000000000000    , 0x20); // -- 2e15+0.5+ulp
dec_test!(bid128_to_int64_xint_110, bid128_to_int64_xint, 0x302000FC6F7C40458157E0B8A7A17FFFu128, 2000000000000001    , 0x20); // -- 2e15+1.5-ulp
dec_test!(bid128_to_int64_xint_111, bid128_to_int64_xint, 0x302000FC6F7C40458157E0B8A7A18000u128, 2000000000000001    , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_int64_xint_112, bid128_to_int64_xint, 0x302000FC6F7C40458157E0B8A7A18001u128, 2000000000000001    , 0x20); // -- 2e15+1.5+ulp
dec_test!(bid128_to_int64_xint_113, bid128_to_int64_xint, 0x302200193E5939A08CE4879688D63FFFu128, 1999999999999998    , 0x20); // -- 2e15-1.5-ulp
dec_test!(bid128_to_int64_xint_114, bid128_to_int64_xint, 0x302200193E5939A08CE4879688D64000u128, 1999999999999998    , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_int64_xint_115, bid128_to_int64_xint, 0x302200193E5939A08CE4879688D64001u128, 1999999999999998    , 0x20); // -- 2e15-1.5+ulp
dec_test!(bid128_to_int64_xint_116, bid128_to_int64_xint, 0x302200193E5939A08CE815152D9CBFFFu128, 1999999999999999    , 0x20); // -- 2e15-0.5-ulp
dec_test!(bid128_to_int64_xint_117, bid128_to_int64_xint, 0x302200193E5939A08CE815152D9CC000u128, 1999999999999999    , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_int64_xint_118, bid128_to_int64_xint, 0x302200193E5939A08CE815152D9CC001u128, 1999999999999999    , 0x20); // -- 2e15-0.5+ulp
dec_test!(bid128_to_int64_xint_119, bid128_to_int64_xint, 0x302308b591e0042c62f88832845936d1u128, 5368941578924638780 , 0x20);
dec_test!(bid128_to_int64_xint_120, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFAABC208D63FFFu128, 9223372036854775806 , 0x20); // -- 2^63-1.5-ulp
dec_test!(bid128_to_int64_xint_121, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFAABC208D64000u128, 9223372036854775806 , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_int64_xint_122, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFAABC208D64001u128, 9223372036854775806 , 0x20); // -- 2^63-1.5+ulp
dec_test!(bid128_to_int64_xint_123, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFC72815B397FFFu128, 9223372036854775806 , 0x20); // -- 2^63-1-ulp
dec_test!(bid128_to_int64_xint_124, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFC72815B398000u128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_xint_125, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFC72815B398001u128, 9223372036854775807 , 0x20); // -- 2^63-1+ulp
dec_test!(bid128_to_int64_xint_126, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFE3940AD9CBFFFu128, 9223372036854775807 , 0x20); // -- 2^63-0.5-ulp
dec_test!(bid128_to_int64_xint_127, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFE3940AD9CC000u128, 9223372036854775807 , 0x20); // -- 2^63-0.5
dec_test!(bid128_to_int64_xint_128, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFE3940AD9CC001u128, 9223372036854775807 , 0x20); // -- 2^63-0.5+ulp
dec_test!(bid128_to_int64_xint_129, bid128_to_int64_xint, 0x3023C6BF52633FFFFFFFFFFFFFFFFFFFu128, 9223372036854775807 , 0x20); // -- 2^63-ulp
dec_test!(bid128_to_int64_xint_130, bid128_to_int64_xint, 0x3023C6BF526340000000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_xint_131, bid128_to_int64_xint, 0x3023C6BF526340000000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+ulp
dec_test!(bid128_to_int64_xint_132, bid128_to_int64_xint, 0x3023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x01); // -- 2^63+0.5-ulp
dec_test!(bid128_to_int64_xint_133, bid128_to_int64_xint, 0x3023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_xint_134, bid128_to_int64_xint, 0x3023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x01); // -- 2^63+0.5+ulp
dec_test!(bid128_to_int64_xint_135, bid128_to_int64_xint, 0x3023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x01); // -- 2^63+1-ulp
dec_test!(bid128_to_int64_xint_136, bid128_to_int64_xint, 0x3023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_xint_137, bid128_to_int64_xint, 0x3023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- 2^63+1+ulp
dec_test!(bid128_to_int64_xint_138, bid128_to_int64_xint, 0x3023d8409b85085d0000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_139, bid128_to_int64_xint, 0x3024000000000000006A94D74F42FFFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_140, bid128_to_int64_xint, 0x3024000000000000006A94D74F430000u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_141, bid128_to_int64_xint, 0x3024000000000000006A94D74F430001u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_142, bid128_to_int64_xint, 0x3024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e19-ulp
dec_test!(bid128_to_int64_xint_143, bid128_to_int64_xint, 0x3024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_xint_144, bid128_to_int64_xint, 0x3024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e19+ulp
dec_test!(bid128_to_int64_xint_145, bid128_to_int64_xint, 0x3024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- 1e19+0.5-ulp
dec_test!(bid128_to_int64_xint_146, bid128_to_int64_xint, 0x3024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_xint_147, bid128_to_int64_xint, 0x3024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- 1e19+0.5+ulp
dec_test!(bid128_to_int64_xint_148, bid128_to_int64_xint, 0x302449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- 1.5e19-ulp
dec_test!(bid128_to_int64_xint_149, bid128_to_int64_xint, 0x302449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_xint_150, bid128_to_int64_xint, 0x302449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- 1.5e19+ulp
dec_test!(bid128_to_int64_xint_151, bid128_to_int64_xint, 0x30245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- 2^64-1-ulp
dec_test!(bid128_to_int64_xint_152, bid128_to_int64_xint, 0x30245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_xint_153, bid128_to_int64_xint, 0x30245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- 2^64-1+ulp
dec_test!(bid128_to_int64_xint_154, bid128_to_int64_xint, 0x30245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- 2^64-0.5-ulp
dec_test!(bid128_to_int64_xint_155, bid128_to_int64_xint, 0x30245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_xint_156, bid128_to_int64_xint, 0x30245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- 2^64-0.5+ulp
dec_test!(bid128_to_int64_xint_157, bid128_to_int64_xint, 0x30245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-ulp
dec_test!(bid128_to_int64_xint_158, bid128_to_int64_xint, 0x30245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_xint_159, bid128_to_int64_xint, 0x30245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+ulp
dec_test!(bid128_to_int64_xint_160, bid128_to_int64_xint, 0x30245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- 2^64+0.5-ulp
dec_test!(bid128_to_int64_xint_161, bid128_to_int64_xint, 0x30245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_xint_162, bid128_to_int64_xint, 0x30245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- 2^64+0.5+ulp
dec_test!(bid128_to_int64_xint_163, bid128_to_int64_xint, 0x30245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- 2^64+1-ulp
dec_test!(bid128_to_int64_xint_164, bid128_to_int64_xint, 0x30245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_xint_165, bid128_to_int64_xint, 0x30245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- 2^64+1+ulp
dec_test!(bid128_to_int64_xint_166, bid128_to_int64_xint, 0x3024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2e19-ulp
dec_test!(bid128_to_int64_xint_167, bid128_to_int64_xint, 0x3024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_xint_168, bid128_to_int64_xint, 0x3024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- 2e19+ulp
dec_test!(bid128_to_int64_xint_169, bid128_to_int64_xint, 0x30247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- 2.5e19-ulp
dec_test!(bid128_to_int64_xint_170, bid128_to_int64_xint, 0x30247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_xint_171, bid128_to_int64_xint, 0x30247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- 2.5e19+ulp
dec_test!(bid128_to_int64_xint_172, bid128_to_int64_xint, 0x3026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- 1e20-ulp
dec_test!(bid128_to_int64_xint_173, bid128_to_int64_xint, 0x3026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_xint_174, bid128_to_int64_xint, 0x3026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- 1e20+ulp
dec_test!(bid128_to_int64_xint_175, bid128_to_int64_xint, 0x302A00000000006C6B935B68D08DA3FFu128, 19999999998         , 0x20); // -- 2e10-1.5-ulp
dec_test!(bid128_to_int64_xint_176, bid128_to_int64_xint, 0x302A00000000006C6B935B68D08DA400u128, 19999999998         , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_int64_xint_177, bid128_to_int64_xint, 0x302A00000000006C6B935B68D08DA401u128, 19999999998         , 0x20); // -- 2e10-1.5+ulp
dec_test!(bid128_to_int64_xint_178, bid128_to_int64_xint, 0x302A00000000006C6B935B8019048BFFu128, 19999999999         , 0x20); // -- 2e10-0.5-ulp
dec_test!(bid128_to_int64_xint_179, bid128_to_int64_xint, 0x302A00000000006C6B935B8019048C00u128, 19999999999         , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_int64_xint_180, bid128_to_int64_xint, 0x302A00000000006C6B935B8019048C01u128, 19999999999         , 0x20); // -- 2e10-0.5+ulp
dec_test!(bid128_to_int64_xint_181, bid128_to_int64_xint, 0x302C000000000000000002BBA7F521FFu128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xint_182, bid128_to_int64_xint, 0x302C000000000000000002BBA7F52200u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xint_183, bid128_to_int64_xint, 0x302C000000000000000002BBA7F52201u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xint_184, bid128_to_int64_xint, 0x302C00000000000AD78EBC5872141BFFu128, 19999999998         , 0x20); // -- 2e10-1-ulp
dec_test!(bid128_to_int64_xint_185, bid128_to_int64_xint, 0x302C00000000000AD78EBC5872141C00u128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_xint_186, bid128_to_int64_xint, 0x302C00000000000AD78EBC5872141C01u128, 19999999999         , 0x20); // -- 2e10-1+ulp
dec_test!(bid128_to_int64_xint_187, bid128_to_int64_xint, 0x302C00000000000AD78EBC5BF025F1FFu128, 20000000000         , 0x20); // -- 2e10+0.5-ulp
dec_test!(bid128_to_int64_xint_188, bid128_to_int64_xint, 0x302C00000000000AD78EBC5BF025F200u128, 20000000000         , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_int64_xint_189, bid128_to_int64_xint, 0x302C00000000000AD78EBC5BF025F201u128, 20000000000         , 0x20); // -- 2e10+0.5+ulp
dec_test!(bid128_to_int64_xint_190, bid128_to_int64_xint, 0x302C00000000000AD78EBC5E4431D5FFu128, 20000000001         , 0x20); // -- 2e10+1.5-ulp
dec_test!(bid128_to_int64_xint_191, bid128_to_int64_xint, 0x302C00000000000AD78EBC5E4431D600u128, 20000000001         , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_int64_xint_192, bid128_to_int64_xint, 0x302C00000000000AD78EBC5E4431D601u128, 20000000001         , 0x20); // -- 2e10+1.5+ulp
dec_test!(bid128_to_int64_xint_193, bid128_to_int64_xint, 0x302C000000108B2A2C28028E3FF41BFFu128, 1999999999999998    , 0x20); // -- 2e15-1-ulp
dec_test!(bid128_to_int64_xint_194, bid128_to_int64_xint, 0x302C000000108B2A2C28028E3FF41C00u128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_xint_195, bid128_to_int64_xint, 0x302C000000108B2A2C28028E3FF41C01u128, 1999999999999999    , 0x20); // -- 2e15-1+ulp
dec_test!(bid128_to_int64_xint_196, bid128_to_int64_xint, 0x302E000000000001158E46094F6AC9FFu128, 20000000000         , 0x20); // -- 2e10+1-ulp
dec_test!(bid128_to_int64_xint_197, bid128_to_int64_xint, 0x302E000000000001158E46094F6ACA00u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_xint_198, bid128_to_int64_xint, 0x302E000000000001158E46094F6ACA01u128, 20000000001         , 0x20); // -- 2e10+1+ulp
dec_test!(bid128_to_int64_xint_199, bid128_to_int64_xint, 0x302E00000001A784379D99DB7D9AC9FFu128, 2000000000000000    , 0x20); // -- 2e15+1-ulp
dec_test!(bid128_to_int64_xint_200, bid128_to_int64_xint, 0x302E00000001A784379D99DB7D9ACA00u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_xint_201, bid128_to_int64_xint, 0x302E00000001A784379D99DB7D9ACA01u128, 2000000000000001    , 0x20); // -- 2e15+1+ulp
dec_test!(bid128_to_int64_xint_202, bid128_to_int64_xint, 0x303000000000000000000006FC23ABFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_203, bid128_to_int64_xint, 0x303000000000000000000006FC23AC00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_204, bid128_to_int64_xint, 0x303000000000000000000006FC23AC01u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_205, bid128_to_int64_xint, 0x303200000000000000000000B2D05DFFu128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_206, bid128_to_int64_xint, 0x303200000000000000000000B2D05E00u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_207, bid128_to_int64_xint, 0x303200000000000000000000B2D05E01u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_208, bid128_to_int64_xint, 0x303800000000000000000000002DDA47u128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xint_209, bid128_to_int64_xint, 0x303800000000000000000000002DDA48u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xint_210, bid128_to_int64_xint, 0x303800000000000000000000002DDA49u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xint_211, bid128_to_int64_xint, 0x303A00000000000000000000000003E7u128, 0                   , 0x20); // -- 0.999
dec_test!(bid128_to_int64_xint_212, bid128_to_int64_xint, 0x303A00000000000000000000000495D3u128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xint_213, bid128_to_int64_xint, 0x303A00000000000000000000000495D4u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xint_214, bid128_to_int64_xint, 0x303A00000000000000000000000495D5u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xint_215, bid128_to_int64_xint, 0x303C0000000000000000000000007561u128, 300                 , 0x20); // -- 300.5-ulp
dec_test!(bid128_to_int64_xint_216, bid128_to_int64_xint, 0x303C0000000000000000000000007562u128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xint_217, bid128_to_int64_xint, 0x303C0000000000000000000000007563u128, 300                 , 0x20); // -- 300.5+ulp
dec_test!(bid128_to_int64_xint_218, bid128_to_int64_xint, 0x303E0000000000000000000000000005u128, 0                   , 0x20); // -- 0.5
dec_test!(bid128_to_int64_xint_219, bid128_to_int64_xint, 0x303E000000000000000000000000000Fu128, 1                   , 0x20); // -- 1.5
dec_test!(bid128_to_int64_xint_220, bid128_to_int64_xint, 0x303E0000000000000000000000000BB7u128, 299                 , 0x20); // -- 300-ulp
dec_test!(bid128_to_int64_xint_221, bid128_to_int64_xint, 0x303E0000000000000000000000000BB8u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_222, bid128_to_int64_xint, 0x303E0000000000000000000000000BB9u128, 300                 , 0x20); // -- 300+ulp
dec_test!(bid128_to_int64_xint_223, bid128_to_int64_xint, 0x303E0000000000000000000000000BBDu128, 300                 , 0x20); // -- 300.5
dec_test!(bid128_to_int64_xint_224, bid128_to_int64_xint, 0x303E0000000000000000002E90EDCFF1u128, 19999999998         , 0x20); // -- 2e10-1.5
dec_test!(bid128_to_int64_xint_225, bid128_to_int64_xint, 0x303E0000000000000000002E90EDCFFBu128, 19999999999         , 0x20); // -- 2e10-0.5
dec_test!(bid128_to_int64_xint_226, bid128_to_int64_xint, 0x303E0000000000000000002E90EDD005u128, 20000000000         , 0x20); // -- 2e10+0.5
dec_test!(bid128_to_int64_xint_227, bid128_to_int64_xint, 0x303E0000000000000000002E90EDD00Fu128, 20000000001         , 0x20); // -- 2e10+1.5
dec_test!(bid128_to_int64_xint_228, bid128_to_int64_xint, 0x303E0000000000000001400000000005u128, 35184372088832      , 0x20); // -- 2^45+0.5
dec_test!(bid128_to_int64_xint_229, bid128_to_int64_xint, 0x303E00000000000000470DE4DF81FFF1u128, 1999999999999998    , 0x20); // -- 2e15-1.5
dec_test!(bid128_to_int64_xint_230, bid128_to_int64_xint, 0x303E00000000000000470DE4DF81FFFBu128, 1999999999999999    , 0x20); // -- 2e15-0.5
dec_test!(bid128_to_int64_xint_231, bid128_to_int64_xint, 0x303E00000000000000470DE4DF820005u128, 2000000000000000    , 0x20); // -- 2e15+0.5
dec_test!(bid128_to_int64_xint_232, bid128_to_int64_xint, 0x303E00000000000000470DE4DF82000Fu128, 2000000000000001    , 0x20); // -- 2e15+1.5
dec_test!(bid128_to_int64_xint_233, bid128_to_int64_xint, 0x303E000000000004FFFFFFFFFFFFFFF1u128, 9223372036854775806 , 0x20); // -- 2^63-1.5
dec_test!(bid128_to_int64_xint_234, bid128_to_int64_xint, 0x303E000000000004FFFFFFFFFFFFFFFBu128, 9223372036854775807 , 0x20); // -- 2^63-0.5
dec_test!(bid128_to_int64_xint_235, bid128_to_int64_xint, 0x303E0000000000050000000000000005u128, -9223372036854775808, 0x01); // -- 2^63+0.5
dec_test!(bid128_to_int64_xint_236, bid128_to_int64_xint, 0x303E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- 1e19+0.5
dec_test!(bid128_to_int64_xint_237, bid128_to_int64_xint, 0x303E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- 2^64-0.5
dec_test!(bid128_to_int64_xint_238, bid128_to_int64_xint, 0x303E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- 2^64+0.5
dec_test!(bid128_to_int64_xint_239, bid128_to_int64_xint, 0x30400000000000000000000000000001u128, 1                   , 0x00); // -- 1
dec_test!(bid128_to_int64_xint_240, bid128_to_int64_xint, 0x3040000000000000000000000000012Bu128, 299                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_xint_241, bid128_to_int64_xint, 0x3040000000000000000000000000012Cu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_242, bid128_to_int64_xint, 0x3040000000000000000000000000012Du128, 301                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_xint_243, bid128_to_int64_xint, 0x304000000000000000000004A817C7FFu128, 19999999999         , 0x00); // -- 2e10-1
dec_test!(bid128_to_int64_xint_244, bid128_to_int64_xint, 0x304000000000000000000004A817C801u128, 20000000001         , 0x00); // -- 2e10+1
dec_test!(bid128_to_int64_xint_245, bid128_to_int64_xint, 0x30400000000000000000200000000000u128, 35184372088832      , 0x00); // -- 2^45
dec_test!(bid128_to_int64_xint_246, bid128_to_int64_xint, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-1
dec_test!(bid128_to_int64_xint_247, bid128_to_int64_xint, 0x304000000000000000071AFD498CFFFFu128, 1999999999999999    , 0x00); // -- 2e15-ulp
dec_test!(bid128_to_int64_xint_248, bid128_to_int64_xint, 0x304000000000000000071AFD498D0000u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_xint_249, bid128_to_int64_xint, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+1
dec_test!(bid128_to_int64_xint_250, bid128_to_int64_xint, 0x304000000000000000071AFD498D0001u128, 2000000000000001    , 0x00); // -- 2e15+ulp
dec_test!(bid128_to_int64_xint_251, bid128_to_int64_xint, 0x304000000000000012f195307644245fu128, 1365036197445117023 , 0x00);
dec_test!(bid128_to_int64_xint_252, bid128_to_int64_xint, 0x30400000000000007FFFFFFFFFFFFFFFu128, 9223372036854775807 , 0x00); // -- 2^63-1
dec_test!(bid128_to_int64_xint_253, bid128_to_int64_xint, 0x30400000000000008000000000000000u128, -9223372036854775808, 0x01); // -- 2^63
dec_test!(bid128_to_int64_xint_254, bid128_to_int64_xint, 0x30400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- 2^63+1
dec_test!(bid128_to_int64_xint_255, bid128_to_int64_xint, 0x3040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- 2^64-1
dec_test!(bid128_to_int64_xint_256, bid128_to_int64_xint, 0x30400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- 2^64
dec_test!(bid128_to_int64_xint_257, bid128_to_int64_xint, 0x30400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- 2^64+1
dec_test!(bid128_to_int64_xint_258, bid128_to_int64_xint, 0x3041ED09BEAD87C0378D8E6400000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xint_259, bid128_to_int64_xint, 0x3042000000000000000000000000001Du128, 290                 , 0x00); // -- 300-ulp
dec_test!(bid128_to_int64_xint_260, bid128_to_int64_xint, 0x3042000000000000000000000000001Eu128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_261, bid128_to_int64_xint, 0x3042000000000000000000000000001Fu128, 310                 , 0x00); // -- 300+ulp
dec_test!(bid128_to_int64_xint_262, bid128_to_int64_xint, 0x304200000000000000000000773593FFu128, 19999999990         , 0x00); // -- 2e10-ulp
dec_test!(bid128_to_int64_xint_263, bid128_to_int64_xint, 0x30420000000000000000000077359400u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_xint_264, bid128_to_int64_xint, 0x30420000000000000000000077359401u128, 20000000010         , 0x00); // -- 2e10+ulp
dec_test!(bid128_to_int64_xint_265, bid128_to_int64_xint, 0x30440000000000000000000000000003u128, 300                 , 0x00); // -- 300
dec_test!(bid128_to_int64_xint_266, bid128_to_int64_xint, 0x30520000000000000000000000000004u128, 4000000000          , 0x00); // -- 4e9
dec_test!(bid128_to_int64_xint_267, bid128_to_int64_xint, 0x30520000000000000000000000000005u128, 5000000000          , 0x00); // -- 5e9
dec_test!(bid128_to_int64_xint_268, bid128_to_int64_xint, 0x30540000000000000000000000000002u128, 20000000000         , 0x00); // -- 2e10
dec_test!(bid128_to_int64_xint_269, bid128_to_int64_xint, 0x305E0000000000000000000000000002u128, 2000000000000000    , 0x00); // -- 2e15
dec_test!(bid128_to_int64_xint_270, bid128_to_int64_xint, 0x3064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- 1.5e19
dec_test!(bid128_to_int64_xint_271, bid128_to_int64_xint, 0x30640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- 2.5e19
dec_test!(bid128_to_int64_xint_272, bid128_to_int64_xint, 0x30660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e19
dec_test!(bid128_to_int64_xint_273, bid128_to_int64_xint, 0x30660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- 2e19
dec_test!(bid128_to_int64_xint_274, bid128_to_int64_xint, 0x30680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- 1e20
dec_test!(bid128_to_int64_xint_275, bid128_to_int64_xint, 0x354c0000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xint_276, bid128_to_int64_xint, "-466584.39E0"                        , -466584             , 0x20);
dec_test!(bid128_to_int64_xint_277, bid128_to_int64_xint, "+4747628878.7846E0"                  , 4747628878          , 0x20);
dec_test!(bid128_to_int64_xint_278, bid128_to_int64_xint, 0x4c06b60bd887fca38b6a56816097c0c2u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_279, bid128_to_int64_xint, "-5.9659588676E0"                     , -5                  , 0x20);
dec_test!(bid128_to_int64_xint_280, bid128_to_int64_xint, 0x69113260bdb1cb1569033de6d564f457u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xint_281, bid128_to_int64_xint, "+77989.8577778985E0"                 , 77989               , 0x20);
dec_test!(bid128_to_int64_xint_282, bid128_to_int64_xint, 0x78000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_283, bid128_to_int64_xint, 0x79e0d7586dc22128137a65493953def8u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_284, bid128_to_int64_xint, 0x7c000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_285, bid128_to_int64_xint, 0x7c003fffffffffff38c15b08ffffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_286, bid128_to_int64_xint, 0x7c003fffffffffff38c15b0affffffffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_287, bid128_to_int64_xint, 0x7e000000000000000000000000000000u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_288, bid128_to_int64_xint, "-88.999E0"                           , -88                 , 0x20);
dec_test!(bid128_to_int64_xint_289, bid128_to_int64_xint, "+8.95957795967E0"                    , 8                   , 0x20);
dec_test!(bid128_to_int64_xint_290, bid128_to_int64_xint, "-8999899998.9E0"                     , -8999899998         , 0x20);
dec_test!(bid128_to_int64_xint_291, bid128_to_int64_xint, 0x89ff6f777fb662ceb6587145989a798eu128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_292, bid128_to_int64_xint, 0x8b400000000000000000000000000000u128, 0                   , 0x00);
dec_test!(bid128_to_int64_xint_293, bid128_to_int64_xint, 0x954d56e73f14b8e52ae3a4b01fc9715cu128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_294, bid128_to_int64_xint, 0x972e998f51ec93910f0f9d6eff850178u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_295, bid128_to_int64_xint, "+988998889.9E0"                      , 988998889           , 0x20);
dec_test!(bid128_to_int64_xint_296, bid128_to_int64_xint, "-9"                                  , -9                  , 0x00);
dec_test!(bid128_to_int64_xint_297, bid128_to_int64_xint, "+998.98998999899989E0"               , 998                 , 0x20);
dec_test!(bid128_to_int64_xint_298, bid128_to_int64_xint, 0xa0ec38959ac6ce9a7ea964682bb93b31u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_299, bid128_to_int64_xint, 0xaec4f2ed810502d01c581b3a4ab0a7f6u128, 0                   , 0x20);
dec_test!(bid128_to_int64_xint_300, bid128_to_int64_xint, 0xAFFCF684DF56C3E01BC6C731FFFFFFFFu128, 0                   , 0x20); // -- -(0.5-ulp)
dec_test!(bid128_to_int64_xint_301, bid128_to_int64_xint, 0xAFFCF684DF56C3E01BC6C73200000000u128, 0                   , 0x20); // -- -(0.5)
dec_test!(bid128_to_int64_xint_302, bid128_to_int64_xint, 0xAFFCF684DF56C3E01BC6C73200000001u128, 0                   , 0x20); // -- -(0.5+ulp)
dec_test!(bid128_to_int64_xint_303, bid128_to_int64_xint, 0xAFFDEC8B86EF679D76FC433D7FFFFFFFu128, 0                   , 0x20); // -- -(0.999-ulp)
dec_test!(bid128_to_int64_xint_304, bid128_to_int64_xint, 0xAFFDEC8B86EF679D76FC433D80000000u128, 0                   , 0x20); // -- -(0.999)
dec_test!(bid128_to_int64_xint_305, bid128_to_int64_xint, 0xAFFDEC8B86EF679D76FC433D80000001u128, 0                   , 0x20); // -- -(0.999+ulp)
dec_test!(bid128_to_int64_xint_306, bid128_to_int64_xint, 0xAFFE314DC6448D9338C15B09FFFFFFFFu128, 0                   , 0x20); // -- -(1-ulp)
dec_test!(bid128_to_int64_xint_307, bid128_to_int64_xint, 0xAFFE314DC6448D9338C15B0A00000000u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_xint_308, bid128_to_int64_xint, 0xAFFE314DC6448D9338C15B0A00000001u128, -1                  , 0x20); // -- -(1+ulp)
dec_test!(bid128_to_int64_xint_309, bid128_to_int64_xint, 0xAFFE49F4A966D45CD522088EFFFFFFFFu128, -1                  , 0x20); // -- -(1.5-ulp)
dec_test!(bid128_to_int64_xint_310, bid128_to_int64_xint, 0xAFFE49F4A966D45CD522088F00000000u128, -1                  , 0x20); // -- -(1.5)
dec_test!(bid128_to_int64_xint_311, bid128_to_int64_xint, 0xAFFE49F4A966D45CD522088F00000001u128, -1                  , 0x20); // -- -(1.5+ulp)
dec_test!(bid128_to_int64_xint_312, bid128_to_int64_xint, 0xB00293E952CDA8B9AA44111DFFFFFFFFu128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_313, bid128_to_int64_xint, 0xB00293E952CDA8B9AA44111E00000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_314, bid128_to_int64_xint, 0xB00293E952CDA8B9AA44111E00000001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_315, bid128_to_int64_xint, 0xB00294286EACB8CB0A8CB6B13FFFFFFFu128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xint_316, bid128_to_int64_xint, 0xB00294286EACB8CB0A8CB6B140000000u128, -300                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xint_317, bid128_to_int64_xint, 0xB00294286EACB8CB0A8CB6B140000001u128, -300                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xint_318, bid128_to_int64_xint, 0xB0040ECA8847C4129106CE82FFFFFFFFu128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_319, bid128_to_int64_xint, 0xB0040ECA8847C4129106CE8300000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_320, bid128_to_int64_xint, 0xB0040ECA8847C4129106CE8300000001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_321, bid128_to_int64_xint, 0xB00A0003C95A2F0B4856475FDFFFFFFFu128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_322, bid128_to_int64_xint, 0xB00A0003C95A2F0B4856475FE0000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_323, bid128_to_int64_xint, 0xB00A0003C95A2F0B4856475FE0000001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_324, bid128_to_int64_xint, 0xB00C000060EF6B1ABA6F07232FFFFFFFu128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_325, bid128_to_int64_xint, 0xB00C000060EF6B1ABA6F072330000000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_326, bid128_to_int64_xint, 0xB00C000060EF6B1ABA6F072330000001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_327, bid128_to_int64_xint, 0xB010C5371912364CE3056C27FFFFFFFFu128, -3999999999         , 0x20); // -- -(4e9-ulp)
dec_test!(bid128_to_int64_xint_328, bid128_to_int64_xint, 0xB010C5371912364CE3056C2800000000u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_xint_329, bid128_to_int64_xint, 0xB010C5371912364CE3056C2800000001u128, -4000000000         , 0x20); // -- -(4e9+ulp)
dec_test!(bid128_to_int64_xint_330, bid128_to_int64_xint, 0xB010F684DF56C3E01BC6C731FFFFFFFFu128, -4999999999         , 0x20); // -- -(5e9-ulp)
dec_test!(bid128_to_int64_xint_331, bid128_to_int64_xint, 0xB010F684DF56C3E01BC6C73200000000u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_xint_332, bid128_to_int64_xint, 0xB010F684DF56C3E01BC6C73200000001u128, -5000000000         , 0x20); // -- -(5e9+ulp)
dec_test!(bid128_to_int64_xint_333, bid128_to_int64_xint, 0xB012629B8C88FB62ED56E4238E3FFFFFu128, -19999999998        , 0x20); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_xint_334, bid128_to_int64_xint, 0xB012629B8C88FB62ED56E4238E400000u128, -19999999998        , 0x20); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_xint_335, bid128_to_int64_xint, 0xB012629B8C88FB62ED56E4238E400001u128, -19999999998        , 0x20); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_xint_336, bid128_to_int64_xint, 0xB012629B8C8905F96EBAD4C9097FFFFFu128, -19999999998        , 0x20); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_xint_337, bid128_to_int64_xint, 0xB012629B8C8905F96EBAD4C909800000u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_xint_338, bid128_to_int64_xint, 0xB012629B8C8905F96EBAD4C909800001u128, -19999999999        , 0x20); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_xint_339, bid128_to_int64_xint, 0xB012629B8C89108FF01EC56E84BFFFFFu128, -19999999999        , 0x20); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_xint_340, bid128_to_int64_xint, 0xB012629B8C89108FF01EC56E84C00000u128, -19999999999        , 0x20); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_xint_341, bid128_to_int64_xint, 0xB012629B8C89108FF01EC56E84C00001u128, -19999999999        , 0x20); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_xint_342, bid128_to_int64_xint, 0xB012629B8C891B267182B613FFFFFFFFu128, -19999999999        , 0x20); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_xint_343, bid128_to_int64_xint, 0xB012629B8C891B267182B61400000000u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_xint_344, bid128_to_int64_xint, 0xB012629B8C891B267182B61400000001u128, -20000000000        , 0x20); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_xint_345, bid128_to_int64_xint, 0xB012629B8C8925BCF2E6A6B97B3FFFFFu128, -20000000000        , 0x20); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_xint_346, bid128_to_int64_xint, 0xB012629B8C8925BCF2E6A6B97B400000u128, -20000000000        , 0x20); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_xint_347, bid128_to_int64_xint, 0xB012629B8C8925BCF2E6A6B97B400001u128, -20000000000        , 0x20); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_xint_348, bid128_to_int64_xint, 0xB012629B8C893053744A975EF67FFFFFu128, -20000000000        , 0x20); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_xint_349, bid128_to_int64_xint, 0xB012629B8C893053744A975EF6800000u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_xint_350, bid128_to_int64_xint, 0xB012629B8C893053744A975EF6800001u128, -20000000001        , 0x20); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_xint_351, bid128_to_int64_xint, 0xB012629B8C893AE9F5AE880471BFFFFFu128, -20000000001        , 0x20); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_xint_352, bid128_to_int64_xint, 0xB012629B8C893AE9F5AE880471C00000u128, -20000000001        , 0x20); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_xint_353, bid128_to_int64_xint, 0xB012629B8C893AE9F5AE880471C00001u128, -20000000001        , 0x20); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_xint_354, bid128_to_int64_xint, 0xb0145004e2271980ff77f5af2dd3d73fu128, -162297967182       , 0x20);
dec_test!(bid128_to_int64_xint_355, bid128_to_int64_xint, 0xB018AD78EBC5AC61FFFFFFFFFFFFFFFFu128, -35184372088831     , 0x20); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_xint_356, bid128_to_int64_xint, 0xB018AD78EBC5AC620000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_xint_357, bid128_to_int64_xint, 0xB018AD78EBC5AC620000000000000001u128, -35184372088832     , 0x20); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_xint_358, bid128_to_int64_xint, 0xB018AD78EBC5AC64B5E3AF16B187FFFFu128, -35184372088832     , 0x20); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_xint_359, bid128_to_int64_xint, 0xB018AD78EBC5AC64B5E3AF16B1880000u128, -35184372088832     , 0x20); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_xint_360, bid128_to_int64_xint, 0xB018AD78EBC5AC64B5E3AF16B1880001u128, -35184372088832     , 0x20); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_xint_361, bid128_to_int64_xint, 0xB01A0000000000A2E6C09AD3E0D3FFFFu128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xint_362, bid128_to_int64_xint, 0xB01A0000000000A2E6C09AD3E0D40000u128, -300                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xint_363, bid128_to_int64_xint, 0xB01A0000000000A2E6C09AD3E0D40001u128, -300                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xint_364, bid128_to_int64_xint, 0xB01C629B8C891B265CB1A40684E9FFFFu128, -1999999999999998   , 0x20); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_xint_365, bid128_to_int64_xint, 0xB01C629B8C891B265CB1A40684EA0000u128, -1999999999999998   , 0x20); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_xint_366, bid128_to_int64_xint, 0xB01C629B8C891B265CB1A40684EA0001u128, -1999999999999998   , 0x20); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_xint_367, bid128_to_int64_xint, 0xB01C629B8C891B2663A1FF60589BFFFFu128, -1999999999999998   , 0x20); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_xint_368, bid128_to_int64_xint, 0xB01C629B8C891B2663A1FF60589C0000u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_xint_369, bid128_to_int64_xint, 0xB01C629B8C891B2663A1FF60589C0001u128, -1999999999999999   , 0x20); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_xint_370, bid128_to_int64_xint, 0xB01C629B8C891B266A925ABA2C4DFFFFu128, -1999999999999999   , 0x20); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_xint_371, bid128_to_int64_xint, 0xB01C629B8C891B266A925ABA2C4E0000u128, -1999999999999999   , 0x20); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_xint_372, bid128_to_int64_xint, 0xB01C629B8C891B266A925ABA2C4E0001u128, -1999999999999999   , 0x20); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_xint_373, bid128_to_int64_xint, 0xB01C629B8C891B267182B613FFFFFFFFu128, -1999999999999999   , 0x20); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_xint_374, bid128_to_int64_xint, 0xB01C629B8C891B267182B61400000000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_xint_375, bid128_to_int64_xint, 0xB01C629B8C891B267182B61400000001u128, -2000000000000000   , 0x20); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_xint_376, bid128_to_int64_xint, 0xB01C629B8C891B267873116DD3B1FFFFu128, -2000000000000000   , 0x20); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_xint_377, bid128_to_int64_xint, 0xB01C629B8C891B267873116DD3B20000u128, -2000000000000000   , 0x20); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_xint_378, bid128_to_int64_xint, 0xB01C629B8C891B267873116DD3B20001u128, -2000000000000000   , 0x20); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_xint_379, bid128_to_int64_xint, 0xB01C629B8C891B267F636CC7A763FFFFu128, -2000000000000000   , 0x20); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_xint_380, bid128_to_int64_xint, 0xB01C629B8C891B267F636CC7A7640000u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_xint_381, bid128_to_int64_xint, 0xB01C629B8C891B267F636CC7A7640001u128, -2000000000000001   , 0x20); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_xint_382, bid128_to_int64_xint, 0xB01C629B8C891B268653C8217B15FFFFu128, -2000000000000001   , 0x20); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_xint_383, bid128_to_int64_xint, 0xB01C629B8C891B268653C8217B160000u128, -2000000000000001   , 0x20); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_xint_384, bid128_to_int64_xint, 0xB01C629B8C891B268653C8217B160001u128, -2000000000000001   , 0x20); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_xint_385, bid128_to_int64_xint, 0xB01E000000000001A055690D9DB7FFFFu128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_386, bid128_to_int64_xint, 0xB01E000000000001A055690D9DB80000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_387, bid128_to_int64_xint, 0xB01E000000000001A055690D9DB80001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_388, bid128_to_int64_xint, 0xB01E002C68AF0BB140B1A2BC2EC4FFFFu128, -35184372088832     , 0x20); // -- -(2^45+0.5-ulp)
dec_test!(bid128_to_int64_xint_389, bid128_to_int64_xint, 0xB01E002C68AF0BB140B1A2BC2EC50000u128, -35184372088832     , 0x20); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_xint_390, bid128_to_int64_xint, 0xB01E002C68AF0BB140B1A2BC2EC50001u128, -35184372088832     , 0x20); // -- -(2^45+0.5+ulp)
dec_test!(bid128_to_int64_xint_391, bid128_to_int64_xint, 0xB02000000000000029A2241AF62BFFFFu128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_392, bid128_to_int64_xint, 0xB02000000000000029A2241AF62C0000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_393, bid128_to_int64_xint, 0xB02000000000000029A2241AF62C0001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_394, bid128_to_int64_xint, 0xB020000470DE4DF81FFFFFFFFFFFFFFFu128, -35184372088831     , 0x20); // -- -(2^45-ulp)
dec_test!(bid128_to_int64_xint_395, bid128_to_int64_xint, 0xB020000470DE4DF82000000000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_xint_396, bid128_to_int64_xint, 0xB020000470DE4DF82000000000000001u128, -35184372088832     , 0x20); // -- -(2^45+ulp)
dec_test!(bid128_to_int64_xint_397, bid128_to_int64_xint, 0xB02000FC6F7C4045813459C637E07FFFu128, -2000000000000000   , 0x20); // -- -(2e15+0.5-ulp)
dec_test!(bid128_to_int64_xint_398, bid128_to_int64_xint, 0xB02000FC6F7C4045813459C637E08000u128, -2000000000000000   , 0x20); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_xint_399, bid128_to_int64_xint, 0xB02000FC6F7C4045813459C637E08001u128, -2000000000000000   , 0x20); // -- -(2e15+0.5+ulp)
dec_test!(bid128_to_int64_xint_400, bid128_to_int64_xint, 0xB02000FC6F7C40458157E0B8A7A17FFFu128, -2000000000000001   , 0x20); // -- -(2e15+1.5-ulp)
dec_test!(bid128_to_int64_xint_401, bid128_to_int64_xint, 0xB02000FC6F7C40458157E0B8A7A18000u128, -2000000000000001   , 0x20); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_xint_402, bid128_to_int64_xint, 0xB02000FC6F7C40458157E0B8A7A18001u128, -2000000000000001   , 0x20); // -- -(2e15+1.5+ulp)
dec_test!(bid128_to_int64_xint_403, bid128_to_int64_xint, 0xB02200193E5939A08CE4879688D63FFFu128, -1999999999999998   , 0x20); // -- -(2e15-1.5-ulp)
dec_test!(bid128_to_int64_xint_404, bid128_to_int64_xint, 0xB02200193E5939A08CE4879688D64000u128, -1999999999999998   , 0x20); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_xint_405, bid128_to_int64_xint, 0xB02200193E5939A08CE4879688D64001u128, -1999999999999998   , 0x20); // -- -(2e15-1.5+ulp)
dec_test!(bid128_to_int64_xint_406, bid128_to_int64_xint, 0xB02200193E5939A08CE815152D9CBFFFu128, -1999999999999999   , 0x20); // -- -(2e15-0.5-ulp)
dec_test!(bid128_to_int64_xint_407, bid128_to_int64_xint, 0xB02200193E5939A08CE815152D9CC000u128, -1999999999999999   , 0x20); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_xint_408, bid128_to_int64_xint, 0xB02200193E5939A08CE815152D9CC001u128, -1999999999999999   , 0x20); // -- -(2e15-0.5+ulp)
dec_test!(bid128_to_int64_xint_409, bid128_to_int64_xint, 0xb0233ed2d05927dd5d10088042c9e02cu128, -6466508648753923220, 0x20);
dec_test!(bid128_to_int64_xint_410, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFAABC208D63FFFu128, -9223372036854775806, 0x20); // -- -(2^63-1.5-ulp)
dec_test!(bid128_to_int64_xint_411, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFAABC208D64000u128, -9223372036854775806, 0x20); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_xint_412, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFAABC208D64001u128, -9223372036854775806, 0x20); // -- -(2^63-1.5+ulp)
dec_test!(bid128_to_int64_xint_413, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFC72815B397FFFu128, -9223372036854775806, 0x20); // -- -(2^63-1-ulp)
dec_test!(bid128_to_int64_xint_414, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFC72815B398000u128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_xint_415, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFC72815B398001u128, -9223372036854775807, 0x20); // -- -(2^63-1+ulp)
dec_test!(bid128_to_int64_xint_416, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFE3940AD9CBFFFu128, -9223372036854775807, 0x20); // -- -(2^63-0.5-ulp)
dec_test!(bid128_to_int64_xint_417, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFE3940AD9CC000u128, -9223372036854775807, 0x20); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_xint_418, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFE3940AD9CC001u128, -9223372036854775807, 0x20); // -- -(2^63-0.5+ulp)
dec_test!(bid128_to_int64_xint_419, bid128_to_int64_xint, 0xB023C6BF52633FFFFFFFFFFFFFFFFFFFu128, -9223372036854775807, 0x20); // -- -(2^63-ulp)
dec_test!(bid128_to_int64_xint_420, bid128_to_int64_xint, 0xB023C6BF526340000000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_xint_421, bid128_to_int64_xint, 0xB023C6BF526340000000000000000001u128, -9223372036854775808, 0x20); // -- -(2^63+ulp)
dec_test!(bid128_to_int64_xint_422, bid128_to_int64_xint, 0xB023C6BF526340000001C6BF52633FFFu128, -9223372036854775808, 0x20); // -- -(2^63+0.5-ulp)
dec_test!(bid128_to_int64_xint_423, bid128_to_int64_xint, 0xB023C6BF526340000001C6BF52634000u128, -9223372036854775808, 0x20); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_xint_424, bid128_to_int64_xint, 0xB023C6BF526340000001C6BF52634001u128, -9223372036854775808, 0x20); // -- -(2^63+0.5+ulp)
dec_test!(bid128_to_int64_xint_425, bid128_to_int64_xint, 0xB023C6BF5263400000038D7EA4C67FFFu128, -9223372036854775808, 0x20); // -- -(2^63+1-ulp)
dec_test!(bid128_to_int64_xint_426, bid128_to_int64_xint, 0xB023C6BF5263400000038D7EA4C68000u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_xint_427, bid128_to_int64_xint, 0xB023C6BF5263400000038D7EA4C68001u128, -9223372036854775808, 0x01); // -- -(2^63+1+ulp)
dec_test!(bid128_to_int64_xint_428, bid128_to_int64_xint, 0xB024000000000000006A94D74F42FFFFu128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_429, bid128_to_int64_xint, 0xB024000000000000006A94D74F430000u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_430, bid128_to_int64_xint, 0xB024000000000000006A94D74F430001u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_431, bid128_to_int64_xint, 0xb0242e3bb2ad99d7148952241404808bu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_432, bid128_to_int64_xint, 0xB024314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e19-ulp)
dec_test!(bid128_to_int64_xint_433, bid128_to_int64_xint, 0xB024314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_xint_434, bid128_to_int64_xint, 0xB024314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e19+ulp)
dec_test!(bid128_to_int64_xint_435, bid128_to_int64_xint, 0xB024314DC6448D9338C18883883D1FFFu128, -9223372036854775808, 0x01); // -- -(1e19+0.5-ulp)
dec_test!(bid128_to_int64_xint_436, bid128_to_int64_xint, 0xB024314DC6448D9338C18883883D2000u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_xint_437, bid128_to_int64_xint, 0xB024314DC6448D9338C18883883D2001u128, -9223372036854775808, 0x01); // -- -(1e19+0.5+ulp)
dec_test!(bid128_to_int64_xint_438, bid128_to_int64_xint, 0xB02449F4A966D45CD522088EFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1.5e19-ulp)
dec_test!(bid128_to_int64_xint_439, bid128_to_int64_xint, 0xB02449F4A966D45CD522088F00000000u128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_xint_440, bid128_to_int64_xint, 0xB02449F4A966D45CD522088F00000001u128, -9223372036854775808, 0x01); // -- -(1.5e19+ulp)
dec_test!(bid128_to_int64_xint_441, bid128_to_int64_xint, 0xB0245AF3107A3FFFFFFFA50CEF85BFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1-ulp)
dec_test!(bid128_to_int64_xint_442, bid128_to_int64_xint, 0xB0245AF3107A3FFFFFFFA50CEF85C000u128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_xint_443, bid128_to_int64_xint, 0xB0245AF3107A3FFFFFFFA50CEF85C001u128, -9223372036854775808, 0x01); // -- -(2^64-1+ulp)
dec_test!(bid128_to_int64_xint_444, bid128_to_int64_xint, 0xB0245AF3107A3FFFFFFFD28677C2DFFFu128, -9223372036854775808, 0x01); // -- -(2^64-0.5-ulp)
dec_test!(bid128_to_int64_xint_445, bid128_to_int64_xint, 0xB0245AF3107A3FFFFFFFD28677C2E000u128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_xint_446, bid128_to_int64_xint, 0xB0245AF3107A3FFFFFFFD28677C2E001u128, -9223372036854775808, 0x01); // -- -(2^64-0.5+ulp)
dec_test!(bid128_to_int64_xint_447, bid128_to_int64_xint, 0xB0245AF3107A3FFFFFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-ulp)
dec_test!(bid128_to_int64_xint_448, bid128_to_int64_xint, 0xB0245AF3107A40000000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_xint_449, bid128_to_int64_xint, 0xB0245AF3107A40000000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+ulp)
dec_test!(bid128_to_int64_xint_450, bid128_to_int64_xint, 0xB0245AF3107A400000002D79883D1FFFu128, -9223372036854775808, 0x01); // -- -(2^64+0.5-ulp)
dec_test!(bid128_to_int64_xint_451, bid128_to_int64_xint, 0xB0245AF3107A400000002D79883D2000u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_xint_452, bid128_to_int64_xint, 0xB0245AF3107A400000002D79883D2001u128, -9223372036854775808, 0x01); // -- -(2^64+0.5+ulp)
dec_test!(bid128_to_int64_xint_453, bid128_to_int64_xint, 0xB0245AF3107A400000005AF3107A3FFFu128, -9223372036854775808, 0x01); // -- -(2^64+1-ulp)
dec_test!(bid128_to_int64_xint_454, bid128_to_int64_xint, 0xB0245AF3107A400000005AF3107A4000u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_xint_455, bid128_to_int64_xint, 0xB0245AF3107A400000005AF3107A4001u128, -9223372036854775808, 0x01); // -- -(2^64+1+ulp)
dec_test!(bid128_to_int64_xint_456, bid128_to_int64_xint, 0xB024629B8C891B267182B613FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2e19-ulp)
dec_test!(bid128_to_int64_xint_457, bid128_to_int64_xint, 0xB024629B8C891B267182B61400000000u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_xint_458, bid128_to_int64_xint, 0xB024629B8C891B267182B61400000001u128, -9223372036854775808, 0x01); // -- -(2e19+ulp)
dec_test!(bid128_to_int64_xint_459, bid128_to_int64_xint, 0xB0247B426FAB61F00DE36398FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2.5e19-ulp)
dec_test!(bid128_to_int64_xint_460, bid128_to_int64_xint, 0xB0247B426FAB61F00DE3639900000000u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_xint_461, bid128_to_int64_xint, 0xB0247B426FAB61F00DE3639900000001u128, -9223372036854775808, 0x01); // -- -(2.5e19+ulp)
dec_test!(bid128_to_int64_xint_462, bid128_to_int64_xint, 0xB026314DC6448D9338C15B09FFFFFFFFu128, -9223372036854775808, 0x01); // -- -(1e20-ulp)
dec_test!(bid128_to_int64_xint_463, bid128_to_int64_xint, 0xB026314DC6448D9338C15B0A00000000u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_xint_464, bid128_to_int64_xint, 0xB026314DC6448D9338C15B0A00000001u128, -9223372036854775808, 0x01); // -- -(1e20+ulp)
dec_test!(bid128_to_int64_xint_465, bid128_to_int64_xint, 0xB02A00000000006C6B935B68D08DA3FFu128, -19999999998        , 0x20); // -- -(2e10-1.5-ulp)
dec_test!(bid128_to_int64_xint_466, bid128_to_int64_xint, 0xB02A00000000006C6B935B68D08DA400u128, -19999999998        , 0x20); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_xint_467, bid128_to_int64_xint, 0xB02A00000000006C6B935B68D08DA401u128, -19999999998        , 0x20); // -- -(2e10-1.5+ulp)
dec_test!(bid128_to_int64_xint_468, bid128_to_int64_xint, 0xB02A00000000006C6B935B8019048BFFu128, -19999999999        , 0x20); // -- -(2e10-0.5-ulp)
dec_test!(bid128_to_int64_xint_469, bid128_to_int64_xint, 0xB02A00000000006C6B935B8019048C00u128, -19999999999        , 0x20); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_xint_470, bid128_to_int64_xint, 0xB02A00000000006C6B935B8019048C01u128, -19999999999        , 0x20); // -- -(2e10-0.5+ulp)
dec_test!(bid128_to_int64_xint_471, bid128_to_int64_xint, 0xB02C000000000000000002BBA7F521FFu128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xint_472, bid128_to_int64_xint, 0xB02C000000000000000002BBA7F52200u128, -300                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xint_473, bid128_to_int64_xint, 0xB02C000000000000000002BBA7F52201u128, -300                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xint_474, bid128_to_int64_xint, 0xB02C00000000000AD78EBC5872141BFFu128, -19999999998        , 0x20); // -- -(2e10-1-ulp)
dec_test!(bid128_to_int64_xint_475, bid128_to_int64_xint, 0xB02C00000000000AD78EBC5872141C00u128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_xint_476, bid128_to_int64_xint, 0xB02C00000000000AD78EBC5872141C01u128, -19999999999        , 0x20); // -- -(2e10-1+ulp)
dec_test!(bid128_to_int64_xint_477, bid128_to_int64_xint, 0xB02C00000000000AD78EBC5BF025F1FFu128, -20000000000        , 0x20); // -- -(2e10+0.5-ulp)
dec_test!(bid128_to_int64_xint_478, bid128_to_int64_xint, 0xB02C00000000000AD78EBC5BF025F200u128, -20000000000        , 0x20); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_xint_479, bid128_to_int64_xint, 0xB02C00000000000AD78EBC5BF025F201u128, -20000000000        , 0x20); // -- -(2e10+0.5+ulp)
dec_test!(bid128_to_int64_xint_480, bid128_to_int64_xint, 0xB02C00000000000AD78EBC5E4431D5FFu128, -20000000001        , 0x20); // -- -(2e10+1.5-ulp)
dec_test!(bid128_to_int64_xint_481, bid128_to_int64_xint, 0xB02C00000000000AD78EBC5E4431D600u128, -20000000001        , 0x20); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_xint_482, bid128_to_int64_xint, 0xB02C00000000000AD78EBC5E4431D601u128, -20000000001        , 0x20); // -- -(2e10+1.5+ulp)
dec_test!(bid128_to_int64_xint_483, bid128_to_int64_xint, 0xB02C000000108B2A2C28028E3FF41BFFu128, -1999999999999998   , 0x20); // -- -(2e15-1-ulp)
dec_test!(bid128_to_int64_xint_484, bid128_to_int64_xint, 0xB02C000000108B2A2C28028E3FF41C00u128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_xint_485, bid128_to_int64_xint, 0xB02C000000108B2A2C28028E3FF41C01u128, -1999999999999999   , 0x20); // -- -(2e15-1+ulp)
dec_test!(bid128_to_int64_xint_486, bid128_to_int64_xint, 0xB02E000000000001158E46094F6AC9FFu128, -20000000000        , 0x20); // -- -(2e10+1-ulp)
dec_test!(bid128_to_int64_xint_487, bid128_to_int64_xint, 0xB02E000000000001158E46094F6ACA00u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_xint_488, bid128_to_int64_xint, 0xB02E000000000001158E46094F6ACA01u128, -20000000001        , 0x20); // -- -(2e10+1+ulp)
dec_test!(bid128_to_int64_xint_489, bid128_to_int64_xint, 0xB02E00000001A784379D99DB7D9AC9FFu128, -2000000000000000   , 0x20); // -- -(2e15+1-ulp)
dec_test!(bid128_to_int64_xint_490, bid128_to_int64_xint, 0xB02E00000001A784379D99DB7D9ACA00u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_xint_491, bid128_to_int64_xint, 0xB02E00000001A784379D99DB7D9ACA01u128, -2000000000000001   , 0x20); // -- -(2e15+1+ulp)
dec_test!(bid128_to_int64_xint_492, bid128_to_int64_xint, 0xB03000000000000000000006FC23ABFFu128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_493, bid128_to_int64_xint, 0xB03000000000000000000006FC23AC00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_494, bid128_to_int64_xint, 0xB03000000000000000000006FC23AC01u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_495, bid128_to_int64_xint, 0xB03200000000000000000000B2D05DFFu128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_496, bid128_to_int64_xint, 0xB03200000000000000000000B2D05E00u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_497, bid128_to_int64_xint, 0xB03200000000000000000000B2D05E01u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_498, bid128_to_int64_xint, 0xB03800000000000000000000002DDA47u128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xint_499, bid128_to_int64_xint, 0xB03800000000000000000000002DDA48u128, -300                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xint_500, bid128_to_int64_xint, 0xB03800000000000000000000002DDA49u128, -300                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xint_501, bid128_to_int64_xint, 0xB03A00000000000000000000000003E7u128, 0                   , 0x20); // -- -(0.999)
dec_test!(bid128_to_int64_xint_502, bid128_to_int64_xint, 0xB03A00000000000000000000000495D3u128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xint_503, bid128_to_int64_xint, 0xB03A00000000000000000000000495D4u128, -300                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xint_504, bid128_to_int64_xint, 0xB03A00000000000000000000000495D5u128, -300                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xint_505, bid128_to_int64_xint, 0xB03C0000000000000000000000007561u128, -300                , 0x20); // -- -(300.5-ulp)
dec_test!(bid128_to_int64_xint_506, bid128_to_int64_xint, 0xB03C0000000000000000000000007562u128, -300                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xint_507, bid128_to_int64_xint, 0xB03C0000000000000000000000007563u128, -300                , 0x20); // -- -(300.5+ulp)
dec_test!(bid128_to_int64_xint_508, bid128_to_int64_xint, 0xB03E0000000000000000000000000005u128, 0                   , 0x20); // -- -(0.5)
dec_test!(bid128_to_int64_xint_509, bid128_to_int64_xint, 0xB03E000000000000000000000000000Fu128, -1                  , 0x20); // -- -(1.5)
dec_test!(bid128_to_int64_xint_510, bid128_to_int64_xint, 0xB03E0000000000000000000000000BB7u128, -299                , 0x20); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_511, bid128_to_int64_xint, 0xB03E0000000000000000000000000BB8u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_512, bid128_to_int64_xint, 0xB03E0000000000000000000000000BB9u128, -300                , 0x20); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_513, bid128_to_int64_xint, 0xB03E0000000000000000000000000BBDu128, -300                , 0x20); // -- -(300.5)
dec_test!(bid128_to_int64_xint_514, bid128_to_int64_xint, 0xB03E0000000000000000002E90EDCFF1u128, -19999999998        , 0x20); // -- -(2e10-1.5)
dec_test!(bid128_to_int64_xint_515, bid128_to_int64_xint, 0xB03E0000000000000000002E90EDCFFBu128, -19999999999        , 0x20); // -- -(2e10-0.5)
dec_test!(bid128_to_int64_xint_516, bid128_to_int64_xint, 0xB03E0000000000000000002E90EDD005u128, -20000000000        , 0x20); // -- -(2e10+0.5)
dec_test!(bid128_to_int64_xint_517, bid128_to_int64_xint, 0xB03E0000000000000000002E90EDD00Fu128, -20000000001        , 0x20); // -- -(2e10+1.5)
dec_test!(bid128_to_int64_xint_518, bid128_to_int64_xint, 0xB03E0000000000000001400000000005u128, -35184372088832     , 0x20); // -- -(2^45+0.5)
dec_test!(bid128_to_int64_xint_519, bid128_to_int64_xint, 0xB03E00000000000000470DE4DF81FFF1u128, -1999999999999998   , 0x20); // -- -(2e15-1.5)
dec_test!(bid128_to_int64_xint_520, bid128_to_int64_xint, 0xB03E00000000000000470DE4DF81FFFBu128, -1999999999999999   , 0x20); // -- -(2e15-0.5)
dec_test!(bid128_to_int64_xint_521, bid128_to_int64_xint, 0xB03E00000000000000470DE4DF820005u128, -2000000000000000   , 0x20); // -- -(2e15+0.5)
dec_test!(bid128_to_int64_xint_522, bid128_to_int64_xint, 0xB03E00000000000000470DE4DF82000Fu128, -2000000000000001   , 0x20); // -- -(2e15+1.5)
dec_test!(bid128_to_int64_xint_523, bid128_to_int64_xint, 0xB03E000000000004FFFFFFFFFFFFFFF1u128, -9223372036854775806, 0x20); // -- -(2^63-1.5)
dec_test!(bid128_to_int64_xint_524, bid128_to_int64_xint, 0xB03E000000000004FFFFFFFFFFFFFFFBu128, -9223372036854775807, 0x20); // -- -(2^63-0.5)
dec_test!(bid128_to_int64_xint_525, bid128_to_int64_xint, 0xB03E0000000000050000000000000005u128, -9223372036854775808, 0x20); // -- -(2^63+0.5)
dec_test!(bid128_to_int64_xint_526, bid128_to_int64_xint, 0xB03E0000000000056BC75E2D63100005u128, -9223372036854775808, 0x01); // -- -(1e19+0.5)
dec_test!(bid128_to_int64_xint_527, bid128_to_int64_xint, 0xB03E000000000009FFFFFFFFFFFFFFFBu128, -9223372036854775808, 0x01); // -- -(2^64-0.5)
dec_test!(bid128_to_int64_xint_528, bid128_to_int64_xint, 0xB03E00000000000A0000000000000005u128, -9223372036854775808, 0x01); // -- -(2^64+0.5)
dec_test!(bid128_to_int64_xint_529, bid128_to_int64_xint, 0xB0400000000000000000000000000001u128, -1                  , 0x00); // -- -(1)
dec_test!(bid128_to_int64_xint_530, bid128_to_int64_xint, 0xB040000000000000000000000000012Bu128, -299                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_531, bid128_to_int64_xint, 0xB040000000000000000000000000012Cu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_532, bid128_to_int64_xint, 0xB040000000000000000000000000012Du128, -301                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_533, bid128_to_int64_xint, 0xB04000000000000000000004A817C7FFu128, -19999999999        , 0x00); // -- -(2e10-1)
dec_test!(bid128_to_int64_xint_534, bid128_to_int64_xint, 0xB04000000000000000000004A817C801u128, -20000000001        , 0x00); // -- -(2e10+1)
dec_test!(bid128_to_int64_xint_535, bid128_to_int64_xint, 0xB0400000000000000000200000000000u128, -35184372088832     , 0x00); // -- -(2^45)
dec_test!(bid128_to_int64_xint_536, bid128_to_int64_xint, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-1)
dec_test!(bid128_to_int64_xint_537, bid128_to_int64_xint, 0xB04000000000000000071AFD498CFFFFu128, -1999999999999999   , 0x00); // -- -(2e15-ulp)
dec_test!(bid128_to_int64_xint_538, bid128_to_int64_xint, 0xB04000000000000000071AFD498D0000u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_xint_539, bid128_to_int64_xint, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+1)
dec_test!(bid128_to_int64_xint_540, bid128_to_int64_xint, 0xB04000000000000000071AFD498D0001u128, -2000000000000001   , 0x00); // -- -(2e15+ulp)
dec_test!(bid128_to_int64_xint_541, bid128_to_int64_xint, 0xB0400000000000007FFFFFFFFFFFFFFFu128, -9223372036854775807, 0x00); // -- -(2^63-1)
dec_test!(bid128_to_int64_xint_542, bid128_to_int64_xint, 0xB0400000000000008000000000000000u128, -9223372036854775808, 0x00); // -- -(2^63)
dec_test!(bid128_to_int64_xint_543, bid128_to_int64_xint, 0xB0400000000000008000000000000001u128, -9223372036854775808, 0x01); // -- -(2^63+1)
dec_test!(bid128_to_int64_xint_544, bid128_to_int64_xint, 0xB040000000000000FFFFFFFFFFFFFFFFu128, -9223372036854775808, 0x01); // -- -(2^64-1)
dec_test!(bid128_to_int64_xint_545, bid128_to_int64_xint, 0xB0400000000000010000000000000000u128, -9223372036854775808, 0x01); // -- -(2^64)
dec_test!(bid128_to_int64_xint_546, bid128_to_int64_xint, 0xB0400000000000010000000000000001u128, -9223372036854775808, 0x01); // -- -(2^64+1)
dec_test!(bid128_to_int64_xint_547, bid128_to_int64_xint, 0xB042000000000000000000000000001Du128, -290                , 0x00); // -- -(300-ulp)
dec_test!(bid128_to_int64_xint_548, bid128_to_int64_xint, 0xB042000000000000000000000000001Eu128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_549, bid128_to_int64_xint, 0xB042000000000000000000000000001Fu128, -310                , 0x00); // -- -(300+ulp)
dec_test!(bid128_to_int64_xint_550, bid128_to_int64_xint, 0xB04200000000000000000000773593FFu128, -19999999990        , 0x00); // -- -(2e10-ulp)
dec_test!(bid128_to_int64_xint_551, bid128_to_int64_xint, 0xB0420000000000000000000077359400u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_xint_552, bid128_to_int64_xint, 0xB0420000000000000000000077359401u128, -20000000010        , 0x00); // -- -(2e10+ulp)
dec_test!(bid128_to_int64_xint_553, bid128_to_int64_xint, 0xB0440000000000000000000000000003u128, -300                , 0x00); // -- -(300)
dec_test!(bid128_to_int64_xint_554, bid128_to_int64_xint, 0xB0520000000000000000000000000004u128, -4000000000         , 0x00); // -- -(4e9)
dec_test!(bid128_to_int64_xint_555, bid128_to_int64_xint, 0xB0520000000000000000000000000005u128, -5000000000         , 0x00); // -- -(5e9)
dec_test!(bid128_to_int64_xint_556, bid128_to_int64_xint, 0xB0540000000000000000000000000002u128, -20000000000        , 0x00); // -- -(2e10)
dec_test!(bid128_to_int64_xint_557, bid128_to_int64_xint, 0xB05E0000000000000000000000000002u128, -2000000000000000   , 0x00); // -- -(2e15)
dec_test!(bid128_to_int64_xint_558, bid128_to_int64_xint, 0xB064000000000000000000000000000Fu128, -9223372036854775808, 0x01); // -- -(1.5e19)
dec_test!(bid128_to_int64_xint_559, bid128_to_int64_xint, 0xB0640000000000000000000000000019u128, -9223372036854775808, 0x01); // -- -(2.5e19)
dec_test!(bid128_to_int64_xint_560, bid128_to_int64_xint, 0xB0660000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e19)
dec_test!(bid128_to_int64_xint_561, bid128_to_int64_xint, 0xB0660000000000000000000000000002u128, -9223372036854775808, 0x01); // -- -(2e19)
dec_test!(bid128_to_int64_xint_562, bid128_to_int64_xint, 0xB0680000000000000000000000000001u128, -9223372036854775808, 0x01); // -- -(1e20)
dec_test!(bid128_to_int64_xint_563, bid128_to_int64_xint, 0xcb6faccaeee016c78e7ba486cc7060eeu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_564, bid128_to_int64_xint, 0xcdfb22efe510b88407fb56aa3753e252u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_565, bid128_to_int64_xint, 0xf8d74f58fb53517f00c4624f16f84b75u128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_566, bid128_to_int64_xint, 0xfc000a1213a577a2892e3e8426eeb56au128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_567, bid128_to_int64_xint, 0xfdf7ff7ffdffbff9270a519cc2bc50bdu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_568, bid128_to_int64_xint, 0xffaeffffffffaddeffe7ffcbffddcfffu128, -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_569, bid128_to_int64_xint, "-Infinity"                           , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_570, bid128_to_int64_xint, "Infinity"                            , -9223372036854775808, 0x01);
dec_test!(bid128_to_int64_xint_571, bid128_to_int64_xint, "QNaN"                                , -9223372036854775808, 0x01);
