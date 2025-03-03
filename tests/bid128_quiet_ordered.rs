/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_quiet_ordered_001, bid128_quiet_ordered, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_002, bid128_quiet_ordered, 0x00000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_003, bid128_quiet_ordered, 0x00000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_004, bid128_quiet_ordered, 0x00000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_005, bid128_quiet_ordered, 0x00000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_006, bid128_quiet_ordered, 0x00000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_007, bid128_quiet_ordered, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_ordered_008, bid128_quiet_ordered, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_ordered_009, bid128_quiet_ordered, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_010, bid128_quiet_ordered, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_011, bid128_quiet_ordered, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_ordered_012, bid128_quiet_ordered, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_ordered_013, bid128_quiet_ordered, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_014, bid128_quiet_ordered, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_015, bid128_quiet_ordered, "-0"                                  , "-0"                                  , true , 0x00);
dec_test!(bid128_quiet_ordered_016, bid128_quiet_ordered, 0                                     , 0                                     , true , 0x00);
dec_test!(bid128_quiet_ordered_017, bid128_quiet_ordered, "-0"                                  , "Infinity"                            , true , 0x00);
dec_test!(bid128_quiet_ordered_018, bid128_quiet_ordered, 0                                     , "SNaN"                                , false, 0x01);
dec_test!(bid128_quiet_ordered_019, bid128_quiet_ordered, 0x108e550dc1f8afd798cf9cc71e726953u128, 0x9a836740ba27c879f94b7d812099255eu128, true , 0x00);
dec_test!(bid128_quiet_ordered_020, bid128_quiet_ordered, 0x1447e04dfb1b1f4ef66bd2c0c4ae5fc7u128, 0x8434fa94bfe03e6bf838103d2482d7e0u128, true , 0x00);
dec_test!(bid128_quiet_ordered_021, bid128_quiet_ordered, 0x2e6c01b4174e06ef54672d9a4c75f5cdu128, 0xfe00154091a1131bd93923f4deb92e2au128, false, 0x01);
dec_test!(bid128_quiet_ordered_022, bid128_quiet_ordered, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_023, bid128_quiet_ordered, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000003u128, true , 0x00);
dec_test!(bid128_quiet_ordered_024, bid128_quiet_ordered, 0x303e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_025, bid128_quiet_ordered, 0x303e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_026, bid128_quiet_ordered, 0x303e0000000000020000000000000000u128, 0x303e0000000000010000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_027, bid128_quiet_ordered, 0x303e0000000000020000000000000000u128, 0x303e0000000000030000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_028, bid128_quiet_ordered, 0x30400000001faa9fc5dcff096007ffffu128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_ordered_029, bid128_quiet_ordered, 0x30400000001faa9fc5dcff0960080000u128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_ordered_030, bid128_quiet_ordered, 0x30400000001faa9fc5dcff0960080001u128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_ordered_031, bid128_quiet_ordered, 0x3040000000fd54fe2ee7f84b003fffffu128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_ordered_032, bid128_quiet_ordered, 0x3040000000fd54fe2ee7f84b00400000u128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_ordered_033, bid128_quiet_ordered, 0x3040000000fd54fe2ee7f84b00400001u128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_ordered_034, bid128_quiet_ordered, 0x3040a6274bbdd0fadd61999e07ac0251u128, 0x3082a6274bbdd0fadd61999e07ac0250u128, true , 0x00);
dec_test!(bid128_quiet_ordered_035, bid128_quiet_ordered, 0x3040c612062576589dd46a73a100695eu128, 0x3074c612062576589dd46a73a100695du128, true , 0x00);
dec_test!(bid128_quiet_ordered_036, bid128_quiet_ordered, 0x3040c612062576589dda322d47eb47ffu128, 0x3074c612062576589dda322d47eb47feu128, true , 0x00);
dec_test!(bid128_quiet_ordered_037, bid128_quiet_ordered, 0x3041622d6fbc91e01277c0caded5b8a3u128, 0x3067622d6fbc91e01277c0caded5b8a2u128, true , 0x00);
dec_test!(bid128_quiet_ordered_038, bid128_quiet_ordered, 0x3041622d6fbc91e0127820b72d18cacfu128, 0x3067622d6fbc91e0127820b72d18caceu128, true , 0x00);
dec_test!(bid128_quiet_ordered_039, bid128_quiet_ordered, 0x30417361cb863de627fa3c5af3bb4c09u128, 0x30737361cb863de627fa3c5af3bb4c08u128, true , 0x00);
dec_test!(bid128_quiet_ordered_040, bid128_quiet_ordered, 0x3047ec3daf9417fe642eaec0d36a73d5u128, 0x307fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_041, bid128_quiet_ordered, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_042, bid128_quiet_ordered, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_043, bid128_quiet_ordered, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff096007ffffu128, true , 0x00);
dec_test!(bid128_quiet_ordered_044, bid128_quiet_ordered, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_045, bid128_quiet_ordered, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_046, bid128_quiet_ordered, 0x3067622d6fbc91e01277c0caded5b8a2u128, 0x3041622d6fbc91e01277c0caded5b8a3u128, true , 0x00);
dec_test!(bid128_quiet_ordered_047, bid128_quiet_ordered, 0x3067622d6fbc91e0127820b72d18caceu128, 0x3041622d6fbc91e0127820b72d18cacfu128, true , 0x00);
dec_test!(bid128_quiet_ordered_048, bid128_quiet_ordered, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b003fffffu128, true , 0x00);
dec_test!(bid128_quiet_ordered_049, bid128_quiet_ordered, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_050, bid128_quiet_ordered, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_051, bid128_quiet_ordered, 0x30737361cb863de627fa3c5af3bb4c08u128, 0x30417361cb863de627fa3c5af3bb4c09u128, true , 0x00);
dec_test!(bid128_quiet_ordered_052, bid128_quiet_ordered, 0x3074c612062576589dd46a73a100695du128, 0x3040c612062576589dd46a73a100695eu128, true , 0x00);
dec_test!(bid128_quiet_ordered_053, bid128_quiet_ordered, 0x3074c612062576589dda322d47eb47feu128, 0x3040c612062576589dda322d47eb47ffu128, true , 0x00);
dec_test!(bid128_quiet_ordered_054, bid128_quiet_ordered, 0x307fec3c64797fe80000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_ordered_055, bid128_quiet_ordered, 0x307fec4450b72ff30000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_ordered_056, bid128_quiet_ordered, 0x30820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_057, bid128_quiet_ordered, 0x30820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_058, bid128_quiet_ordered, 0x3082a6274bbdd0fadd61999e07ac0250u128, 0x3040a6274bbdd0fadd61999e07ac0251u128, true , 0x00);
dec_test!(bid128_quiet_ordered_059, bid128_quiet_ordered, 0x78000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_060, bid128_quiet_ordered, 0x78000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_061, bid128_quiet_ordered, 0x78000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_062, bid128_quiet_ordered, 0x78000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_063, bid128_quiet_ordered, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_064, bid128_quiet_ordered, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_065, bid128_quiet_ordered, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_066, bid128_quiet_ordered, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_067, bid128_quiet_ordered, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_068, bid128_quiet_ordered, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_069, bid128_quiet_ordered, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_070, bid128_quiet_ordered, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_ordered_071, bid128_quiet_ordered, 0x7c1b30570dab9076feffee7fff786db6u128, 0xdf259b6242e67b67a5c0030e1a7cc2f9u128, false, 0x00);
dec_test!(bid128_quiet_ordered_072, bid128_quiet_ordered, 0x7e000000000000000000000000000000u128, 0x3b3aeffb1440309b8643bd765f1a6103u128, false, 0x01);
dec_test!(bid128_quiet_ordered_073, bid128_quiet_ordered, 0x7e000e93b851c7ae4768a1775218d71bu128, 0x86b53a0929c1327ac1fc7520c866591bu128, false, 0x01);
dec_test!(bid128_quiet_ordered_074, bid128_quiet_ordered, 0x7e001990e7ca59a448693799e4b2cfb2u128, 0x1e0a0000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_quiet_ordered_075, bid128_quiet_ordered, 0x7e002dbd9fa86dadce27d5e5d5b5a732u128, 0x80bf5a585cc5354d779786d7880a8f1eu128, false, 0x01);
dec_test!(bid128_quiet_ordered_076, bid128_quiet_ordered, 0x80000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_077, bid128_quiet_ordered, 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_078, bid128_quiet_ordered, 0x80000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_079, bid128_quiet_ordered, 0x80000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_080, bid128_quiet_ordered, 0x80000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_081, bid128_quiet_ordered, 0x80000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_082, bid128_quiet_ordered, 0xb03e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_083, bid128_quiet_ordered, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_084, bid128_quiet_ordered, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000003u128, true , 0x00);
dec_test!(bid128_quiet_ordered_085, bid128_quiet_ordered, 0xb03e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_ordered_086, bid128_quiet_ordered, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000010000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_087, bid128_quiet_ordered, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000030000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_088, bid128_quiet_ordered, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_089, bid128_quiet_ordered, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_ordered_090, bid128_quiet_ordered, 0xb07fec3c64797fe80000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_ordered_091, bid128_quiet_ordered, 0xb07fec4450b72ff30000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_ordered_092, bid128_quiet_ordered, 0xb0820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_093, bid128_quiet_ordered, 0xb0820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_094, bid128_quiet_ordered, 0xc28c502f0218518445c3a769d8f1d387u128, 0x0e2d8d2c531aae338d2784773d354381u128, true , 0x00);
dec_test!(bid128_quiet_ordered_095, bid128_quiet_ordered, 0xc87ac24f38e5d8b1144c6157a826d0f3u128, 0x8826b7b469d1fae75a265b9999da00b9u128, true , 0x00);
dec_test!(bid128_quiet_ordered_096, bid128_quiet_ordered, 0xcaf69b3bb2944c14fffffffeffffffffu128, 0x7f06f6f04bcdbe2b0802251381002840u128, false, 0x01);
dec_test!(bid128_quiet_ordered_097, bid128_quiet_ordered, 0xcf657a7c815ae5e9423740ccec9d67d4u128, 0xcf6b410baae3fcf8a1efa6e8d53818beu128, true , 0x00);
dec_test!(bid128_quiet_ordered_098, bid128_quiet_ordered, 0xf8000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_099, bid128_quiet_ordered, 0xf8000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_100, bid128_quiet_ordered, 0xf8000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_101, bid128_quiet_ordered, 0xf8000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_ordered_102, bid128_quiet_ordered, 0xfe001d9ca92da6f4125795b05171711bu128, 0xd531bfe76efa200af73e253e32f36347u128, false, 0x01);
dec_test!(bid128_quiet_ordered_103, bid128_quiet_ordered, "-Infinity"                           , 0                                     , true , 0x00);
dec_test!(bid128_quiet_ordered_104, bid128_quiet_ordered, "Infinity"                            , 0                                     , true , 0x00);
dec_test!(bid128_quiet_ordered_105, bid128_quiet_ordered, "-Infinity"                           , "Infinity"                            , true , 0x00);
dec_test!(bid128_quiet_ordered_106, bid128_quiet_ordered, "Infinity"                            , "-Infinity"                           , true , 0x00);
dec_test!(bid128_quiet_ordered_107, bid128_quiet_ordered, "QNaN"                                , "SNaN"                                , false, 0x01);
