/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_scalbln_001, bid128_scalbln, 0, 0x00000000000000000000000000000001u128, 0                   , 0x00000000000000000000000000000001u128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_002, bid128_scalbln, 0, 0x00000000000000000000000000000001u128, 0                   , 0x00000000000000000000000000000001u128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_003, bid128_scalbln, 0, 0x00000000000000000000000000000001u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.1000000000 longintsize=32
dec_test!(bid128_scalbln_004, bid128_scalbln, 0, 0x00000000000000000000000000000001u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.1000000000 longintsize=64
dec_test!(bid128_scalbln_005, bid128_scalbln, 0, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.5000000000 longintsize=32
dec_test!(bid128_scalbln_006, bid128_scalbln, 0, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.5000000000 longintsize=64
dec_test!(bid128_scalbln_007, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_008, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_009, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x2FFBED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_010, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x2FFBED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_011, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x2FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_012, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x2FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_013, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 2147483647          , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.00000 longintsize=32
dec_test!(bid128_scalbln_014, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 2147483647          , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.00000 longintsize=64
dec_test!(bid128_scalbln_015, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -2147483648         , 0x00000000000000000000000000000000u128, 0x30); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_016, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -2147483648         , 0x00000000000000000000000000000000u128, 0x30); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_017, bid128_scalbln, 0, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_018, bid128_scalbln, 0, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_019, bid128_scalbln, 0, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_020, bid128_scalbln, 0, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_021, bid128_scalbln, 0, 0x69dbb75d7734cd9e1234567890123456u128, 0                   , 0x276e0000000000000000000000000000u128, 0x00); //  longintsize=32
dec_test!(bid128_scalbln_022, bid128_scalbln, 0, 0x69dbb75d7734cd9e1234567890123456u128, 0                   , 0x276e0000000000000000000000000000u128, 0x00); //  longintsize=64
dec_test!(bid128_scalbln_023, bid128_scalbln, 0, 0x69dbb75d7734cd9e1234567890123456u128, 1                   , 0x27700000000000000000000000000000u128, 0x00); //  longintsize=32
dec_test!(bid128_scalbln_024, bid128_scalbln, 0, 0x69dbb75d7734cd9e1234567890123456u128, 1                   , 0x27700000000000000000000000000000u128, 0x00); //  longintsize=64
// dec_test!(bid128_scalbln_025, bid128_scalbln, 0, 0x78000000000000000000000000000000u128, 0                   , 0x78000000000000000000000000000000u128, 0x00); // longintsize=32
dec_test!(bid128_scalbln_026, bid128_scalbln, 0, 0x78000000000000000000000000000000u128, 0                   , 0x78000000000000000000000000000000u128, 0x00); // longintsize=64
// dec_test!(bid128_scalbln_027, bid128_scalbln, 0, 0x78000000000000000000000000000000u128, -1                  , 0x78000000000000000000000000000000u128, 0x00); // longintsize=32
// dec_test!(bid128_scalbln_028, bid128_scalbln, 0, 0x78000000000000000000000000000000u128, 1                   , 0x78000000000000000000000000000000u128, 0x00); // longintsize=32
dec_test!(bid128_scalbln_029, bid128_scalbln, 0, 0x78000000000000000000000000000000u128, -1                  , 0x78000000000000000000000000000000u128, 0x00); // longintsize=64
dec_test!(bid128_scalbln_030, bid128_scalbln, 0, 0x78000000000000000000000000000000u128, 1                   , 0x78000000000000000000000000000000u128, 0x00); // longintsize=64
// dec_test!(bid128_scalbln_031, bid128_scalbln, 0, 0x7c000000000000000000000000000000u128, 0                   , 0x7c000000000000000000000000000000u128, 0x00); // longintsize=32
dec_test!(bid128_scalbln_032, bid128_scalbln, 0, 0x7c000000000000000000000000000000u128, 0                   , 0x7c000000000000000000000000000000u128, 0x00); // longintsize=64
// dec_test!(bid128_scalbln_033, bid128_scalbln, 0, 0x7e000000000000000000000000000000u128, 0                   , 0x7c000000000000000000000000000000u128, 0x01); // longintsize=32
dec_test!(bid128_scalbln_034, bid128_scalbln, 0, 0x7e000000000000000000000000000000u128, 0                   , 0x7c000000000000000000000000000000u128, 0x01); // longintsize=64
dec_test!(bid128_scalbln_035, bid128_scalbln, 1, 0x00000000000000000000000000000001u128, 0                   , 0x00000000000000000000000000000001u128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_036, bid128_scalbln, 1, 0x00000000000000000000000000000001u128, 0                   , 0x00000000000000000000000000000001u128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_037, bid128_scalbln, 1, 0x00000000000000000000000000000001u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.1000000000 longintsize=32
dec_test!(bid128_scalbln_038, bid128_scalbln, 1, 0x00000000000000000000000000000001u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.1000000000 longintsize=64
dec_test!(bid128_scalbln_039, bid128_scalbln, 1, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.5000000000 longintsize=32
dec_test!(bid128_scalbln_040, bid128_scalbln, 1, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.5000000000 longintsize=64
dec_test!(bid128_scalbln_041, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_042, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_043, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x2FFBED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_044, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x2FFBED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_045, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x2FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_046, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x2FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_047, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 2147483647          , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.00000 longintsize=32
dec_test!(bid128_scalbln_048, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 2147483647          , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.00000 longintsize=64
dec_test!(bid128_scalbln_049, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -2147483648         , 0x00000000000000000000000000000000u128, 0x30); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_050, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -2147483648         , 0x00000000000000000000000000000000u128, 0x30); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_051, bid128_scalbln, 1, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_052, bid128_scalbln, 1, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_053, bid128_scalbln, 1, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_054, bid128_scalbln, 1, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_055, bid128_scalbln, 2, 0x00000000000000000000000000000001u128, 0                   , 0x00000000000000000000000000000001u128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_056, bid128_scalbln, 2, 0x00000000000000000000000000000001u128, 0                   , 0x00000000000000000000000000000001u128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_057, bid128_scalbln, 2, 0x00000000000000000000000000000001u128, -1                  , 0x00000000000000000000000000000001u128, 0x30); // ulp=-.9000000000 longintsize=32
dec_test!(bid128_scalbln_058, bid128_scalbln, 2, 0x00000000000000000000000000000001u128, -1                  , 0x00000000000000000000000000000001u128, 0x30); // ulp=-.9000000000 longintsize=64
dec_test!(bid128_scalbln_059, bid128_scalbln, 2, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000001u128, 0x30); // ulp=-.500000000 longintsize=32
dec_test!(bid128_scalbln_060, bid128_scalbln, 2, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000001u128, 0x30); // ulp=-.500000000 longintsize=64
dec_test!(bid128_scalbln_061, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_062, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_063, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x2FFBED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_064, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x2FFBED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_065, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x2FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_066, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x2FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_067, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 2147483647          , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.00000 longintsize=32
dec_test!(bid128_scalbln_068, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 2147483647          , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.00000 longintsize=64
dec_test!(bid128_scalbln_069, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -2147483648         , 0x00000000000000000000000000000001u128, 0x30); // ulp=-1.000000000 longintsize=32
dec_test!(bid128_scalbln_070, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -2147483648         , 0x00000000000000000000000000000001u128, 0x30); // ulp=-1.000000000 longintsize=64
dec_test!(bid128_scalbln_071, bid128_scalbln, 2, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_072, bid128_scalbln, 2, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_073, bid128_scalbln, 2, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_074, bid128_scalbln, 2, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_075, bid128_scalbln, 3, 0x00000000000000000000000000000001u128, 0                   , 0x00000000000000000000000000000001u128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_076, bid128_scalbln, 3, 0x00000000000000000000000000000001u128, 0                   , 0x00000000000000000000000000000001u128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_077, bid128_scalbln, 3, 0x00000000000000000000000000000001u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.1000000000 longintsize=32
dec_test!(bid128_scalbln_078, bid128_scalbln, 3, 0x00000000000000000000000000000001u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.1000000000 longintsize=64
dec_test!(bid128_scalbln_079, bid128_scalbln, 3, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.5000000000 longintsize=32
dec_test!(bid128_scalbln_080, bid128_scalbln, 3, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000000u128, 0x30); // ulp=.5000000000 longintsize=64
dec_test!(bid128_scalbln_081, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_082, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_083, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x2FFBED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_084, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -1                  , 0x2FFBED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_085, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x2FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_086, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x2FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_087, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 2147483647          , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.00000 longintsize=32
dec_test!(bid128_scalbln_088, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 2147483647          , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.00000 longintsize=64
dec_test!(bid128_scalbln_089, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -2147483648         , 0x00000000000000000000000000000000u128, 0x30); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_090, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -2147483648         , 0x00000000000000000000000000000000u128, 0x30); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_091, bid128_scalbln, 3, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_092, bid128_scalbln, 3, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x00); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_093, bid128_scalbln, 3, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.0000000000 longintsize=32
dec_test!(bid128_scalbln_094, bid128_scalbln, 3, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 1                   , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_095, bid128_scalbln, 4, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000001u128, 0x30); // ulp=-.500000000 longintsize=32
dec_test!(bid128_scalbln_096, bid128_scalbln, 4, 0x00000000000000000000000000000005u128, -1                  , 0x00000000000000000000000000000001u128, 0x30); // ulp=-.500000000 longintsize=64
dec_test!(bid128_scalbln_097, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 9223372036854775807 , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.00000 longintsize=64
dec_test!(bid128_scalbln_098, bid128_scalbln, 0, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808, 0x00000000000000000000000000000000u128, 0x30); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_099, bid128_scalbln, 0, 0x69dbb75d7734cd9e1234567890123456u128, -1                  , 0x276c0000000000000000000000000000u128, 0x00); // longintsize=64
dec_test!(bid128_scalbln_100, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 9223372036854775807 , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.00000 longintsize=64
dec_test!(bid128_scalbln_101, bid128_scalbln, 1, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808, 0x00000000000000000000000000000000u128, 0x30); // ulp=0.0000000000 longintsize=64
dec_test!(bid128_scalbln_102, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 9223372036854775807 , 0x78000000000000000000000000000000u128, 0x28); // ulp=0.00000 longintsize=64
dec_test!(bid128_scalbln_103, bid128_scalbln, 2, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808, 0x00000000000000000000000000000001u128, 0x30); // ulp=-1.000000000 longintsize=64
dec_test!(bid128_scalbln_104, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, 9223372036854775807 , 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x28); // ulp=0.00000 longintsize=64
dec_test!(bid128_scalbln_105, bid128_scalbln, 3, 0x2FFDED09BEAD87C0378D8E63FFFFFFFFu128, -9223372036854775808, 0x00000000000000000000000000000000u128, 0x30); // ulp=0.0000000000 longintsize=64
// bid128_scalbln 0 [69dbb75d7734cd9e1234567890123456] -1 [276c0000000000000000000000000000] 00 longintsize=32