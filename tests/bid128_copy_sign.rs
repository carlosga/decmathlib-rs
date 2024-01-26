/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_copy_sign_001, bid128_copy_sign, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128);
dec_test!(bid128_copy_sign_002, bid128_copy_sign, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128);
dec_test!(bid128_copy_sign_003, bid128_copy_sign, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128);
dec_test!(bid128_copy_sign_004, bid128_copy_sign, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128);
dec_test!(bid128_copy_sign_005, bid128_copy_sign, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128);
dec_test!(bid128_copy_sign_006, bid128_copy_sign, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128);
dec_test!(bid128_copy_sign_007, bid128_copy_sign, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128);
dec_test!(bid128_copy_sign_008, bid128_copy_sign, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128);
dec_test!(bid128_copy_sign_009, bid128_copy_sign, "-0"                                  , "-0"                                  , 0xb0400000000000000000000000000000u128);
dec_test!(bid128_copy_sign_010, bid128_copy_sign, "-0"                                  , "-Infinity"                           , 0xb0400000000000000000000000000000u128);
dec_test!(bid128_copy_sign_011, bid128_copy_sign, 0x190c0000000000000000000000000000u128, 0xb4ee0000000000000000000000000000u128, 0x990c0000000000000000000000000000u128);
dec_test!(bid128_copy_sign_012, bid128_copy_sign, 0x24e465fbb1822777803774e51267ccf2u128, 0x06620000000000000000000000000000u128, 0x24e465fbb1822777803774e51267ccf2u128);
dec_test!(bid128_copy_sign_013, bid128_copy_sign, 0x2e880000000000000000000000000000u128, 0x8dbfdda66712e3e7fd5ff95b33eeb55cu128, 0xae880000000000000000000000000000u128);
dec_test!(bid128_copy_sign_014, bid128_copy_sign, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128);
dec_test!(bid128_copy_sign_015, bid128_copy_sign, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128);
dec_test!(bid128_copy_sign_016, bid128_copy_sign, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128);
dec_test!(bid128_copy_sign_017, bid128_copy_sign, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128);
dec_test!(bid128_copy_sign_018, bid128_copy_sign, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128);
dec_test!(bid128_copy_sign_019, bid128_copy_sign, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128);
dec_test!(bid128_copy_sign_020, bid128_copy_sign, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128);
dec_test!(bid128_copy_sign_021, bid128_copy_sign, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128);
dec_test!(bid128_copy_sign_022, bid128_copy_sign, 0x885a0000000000000000000000000000u128, 0x55c20000000000000000000000000000u128, 0x085a0000000000000000000000000000u128);
dec_test!(bid128_copy_sign_023, bid128_copy_sign, 0x8c78cd86d1408bbf9c58c58d5f149b91u128, 0x5f580000000000000000000000000000u128, 0x0c78cd86d1408bbf9c58c58d5f149b91u128);
dec_test!(bid128_copy_sign_024, bid128_copy_sign, 0xb7eaa93ab4ec09af34c13cdee30f13a9u128, 0x29efeb164e8e506f4f310fb5b0c9f682u128, 0x37eaa93ab4ec09af34c13cdee30f13a9u128);
dec_test!(bid128_copy_sign_025, bid128_copy_sign, 0xbfcf3787f5c05b56d1475745c9d0a484u128, 0xbcb7ddb5c38dac859dd421ddbc349c64u128, 0xbfcf3787f5c05b56d1475745c9d0a484u128);
dec_test!(bid128_copy_sign_026, bid128_copy_sign, 0xc92c0000000000000000000000000000u128, 0xc64f7d71753ab94157aac934ceffcb9eu128, 0xc92c0000000000000000000000000000u128);
dec_test!(bid128_copy_sign_027, bid128_copy_sign, 0xcf1cf4acfdffb59cd1931d42e614aba5u128, 0xc17d0dc7cab493594e57a66491e7955eu128, 0xcf1cf4acfdffb59cd1931d42e614aba5u128);
dec_test!(bid128_copy_sign_028, bid128_copy_sign, 0xd2e40000000000000000000000000000u128, 0x4780757d59815b86f86c2ca4c77a9fd7u128, 0x52e40000000000000000000000000000u128);
dec_test!(bid128_copy_sign_029, bid128_copy_sign, 0xf69a5c09a7fbd669b109e139b2b3490fu128, 0x7ceafd3f733f8d074820a5f60c181a30u128, 0x769a5c09a7fbd669b109e139b2b3490fu128);
dec_test!(bid128_copy_sign_030, bid128_copy_sign, "-Infinity"                           ,         "0"                           , 0x78000000000000000000000000000000u128);
dec_test!(bid128_copy_sign_031, bid128_copy_sign,  "Infinity"                           ,         "0"                           , 0x78000000000000000000000000000000u128);
dec_test!(bid128_copy_sign_032, bid128_copy_sign, "-Infinity"                           ,  "Infinity"                           , 0x78000000000000000000000000000000u128);
dec_test!(bid128_copy_sign_033, bid128_copy_sign, "-Infinity"                           ,      "QNaN"                           , 0x78000000000000000000000000000000u128);
dec_test!(bid128_copy_sign_034, bid128_copy_sign,  "Infinity"                           ,      "QNaN"                           , 0x78000000000000000000000000000000u128);
dec_test!(bid128_copy_sign_035, bid128_copy_sign,      "QNaN"                           ,      "SNaN"                           , 0x7c000000000000000000000000000000u128);
dec_test!(bid128_copy_sign_036, bid128_copy_sign,      "SNaN"                           , "-Infinity"                           , 0xfe000000000000000000000000000000u128);
