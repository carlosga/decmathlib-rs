/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_abs_001, bid128_abs, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128);
dec_test!(bid128_abs_002, bid128_abs, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128);
dec_test!(bid128_abs_003, bid128_abs, "-0"                                  , 0x30400000000000000000000000000000u128);
dec_test!(bid128_abs_004, bid128_abs,  "0"                                  , 0x30400000000000000000000000000000u128);
dec_test!(bid128_abs_005, bid128_abs, 0x20491165061c532a535089a5c8f9da39u128, 0x20491165061c532a535089a5c8f9da39u128);
dec_test!(bid128_abs_006, bid128_abs, 0x21213a1853ae801da7a83cd1c3bfc8cbu128, 0x21213a1853ae801da7a83cd1c3bfc8cbu128);
dec_test!(bid128_abs_007, bid128_abs, 0x3577621b973eb09d7e1681d2fc2fd1a3u128, 0x3577621b973eb09d7e1681d2fc2fd1a3u128);
dec_test!(bid128_abs_008, bid128_abs, 0x3d780000000000000000000000000000u128, 0x3d780000000000000000000000000000u128);
dec_test!(bid128_abs_009, bid128_abs, 0x3f620000000000000000000000000000u128, 0x3f620000000000000000000000000000u128);
dec_test!(bid128_abs_010, bid128_abs, 0x54f99570af8fbdf89052e356786395d7u128, 0x54f99570af8fbdf89052e356786395d7u128);
dec_test!(bid128_abs_011, bid128_abs, 0x59170c2b2d753371caddbeb2aa9e9d19u128, 0x59170c2b2d753371caddbeb2aa9e9d19u128);
dec_test!(bid128_abs_012, bid128_abs, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128);
dec_test!(bid128_abs_013, bid128_abs, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128);
dec_test!(bid128_abs_014, bid128_abs, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128);
dec_test!(bid128_abs_015, bid128_abs, 0x812c0000000000000000000000000000u128, 0x012c0000000000000000000000000000u128);
dec_test!(bid128_abs_016, bid128_abs, 0xa050785cc4df58144acbb40b269da9e3u128, 0x2050785cc4df58144acbb40b269da9e3u128);
dec_test!(bid128_abs_017, bid128_abs, 0xbc920000000000000000000000000000u128, 0x3c920000000000000000000000000000u128);
dec_test!(bid128_abs_018, bid128_abs, "-Infinity"                           , 0x78000000000000000000000000000000u128);
dec_test!(bid128_abs_019, bid128_abs,  "Infinity"                           , 0x78000000000000000000000000000000u128);
dec_test!(bid128_abs_020, bid128_abs,      "QNaN"                           , 0x7c000000000000000000000000000000u128);
dec_test!(bid128_abs_021, bid128_abs,      "SNaN"                           , 0x7e000000000000000000000000000000u128);
