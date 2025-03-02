/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_is_canonical_001, bid128_is_canonical, 0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_is_canonical_002, bid128_is_canonical, 0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_is_canonical_003, bid128_is_canonical, "-0"                                  , true);
dec_test!(bid128_is_canonical_004, bid128_is_canonical,  "0"                                  , true);
dec_test!(bid128_is_canonical_005, bid128_is_canonical, 0x171dc93a3a721a43b58542fdde27b555u128, true);
dec_test!(bid128_is_canonical_006, bid128_is_canonical, 0x23155fa1ce3b63e7054decfe550b4e45u128, true);
dec_test!(bid128_is_canonical_007, bid128_is_canonical, 0x3a2e024cc9d0b53e05182193ee969dbdu128, true);
dec_test!(bid128_is_canonical_008, bid128_is_canonical, 0x4089bca971360a7d5aeff4e6bf411c39u128, true);
dec_test!(bid128_is_canonical_009, bid128_is_canonical, 0x5dbb7d927efefd2ff33f8f17fefffdffu128, true);
dec_test!(bid128_is_canonical_010, bid128_is_canonical, 0x6085008102161490ffdffeeff7ffbfffu128, false);
dec_test!(bid128_is_canonical_011, bid128_is_canonical, 0x7c0013e87ada0359835044d68d872147u128, true);
dec_test!(bid128_is_canonical_012, bid128_is_canonical, 0x7c00314dc6448d9338c15b09ffffffffu128, true);
dec_test!(bid128_is_canonical_013, bid128_is_canonical, 0x7c003fffffffffff38c15b08ffffffffu128, false);
dec_test!(bid128_is_canonical_014, bid128_is_canonical, 0x7c003fffffffffff38c15b0affffffffu128, false);
dec_test!(bid128_is_canonical_015, bid128_is_canonical, 0x816d4d6ec11d63025b6461da887f2291u128, true);
dec_test!(bid128_is_canonical_016, bid128_is_canonical, 0x94ac069b0f87925be19b1e3d5736e4b3u128, true);
dec_test!(bid128_is_canonical_017, bid128_is_canonical, 0xa1d3aac268268694dcad90cc10d00253u128, true);
dec_test!(bid128_is_canonical_018, bid128_is_canonical, 0xbe362574fcd0f5abf3a57ff131f5cfdbu128, true);
dec_test!(bid128_is_canonical_019, bid128_is_canonical, 0xc9816c991a0af8f1270fa52aebd97dccu128, true);
dec_test!(bid128_is_canonical_020, bid128_is_canonical, 0xd8c92b6df1c4160755a964589af7ae58u128, true);
dec_test!(bid128_is_canonical_021, bid128_is_canonical, 0xf8000000000000000000000000000000u128, true);
dec_test!(bid128_is_canonical_022, bid128_is_canonical, 0xf8000000010000000000000000000000u128, false);
dec_test!(bid128_is_canonical_023, bid128_is_canonical, 0xf8100000000000000000000000000000u128, false);
dec_test!(bid128_is_canonical_024, bid128_is_canonical, 0xfa79d291c68723e9bf36ffd4dbefc63fu128, false);
dec_test!(bid128_is_canonical_025, bid128_is_canonical, 0xfe00381d0020a920ff6f3fff9ff3cd7eu128, false);
dec_test!(bid128_is_canonical_026, bid128_is_canonical, 0xffffffffffffffff1000000000000000u128, false);
dec_test!(bid128_is_canonical_027, bid128_is_canonical, "-Infinity"                           , true);
dec_test!(bid128_is_canonical_028, bid128_is_canonical,  "Infinity"                           , true);
dec_test!(bid128_is_canonical_029, bid128_is_canonical,      "QNaN"                           , true);
dec_test!(bid128_is_canonical_030, bid128_is_canonical,      "SNaN"                           , true);
