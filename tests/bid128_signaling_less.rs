/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_signaling_less_001, bid128_signaling_less, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_signaling_less_002, bid128_signaling_less, 0x00000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_signaling_less_003, bid128_signaling_less, 0x00000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_004, bid128_signaling_less, 0x00000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_signaling_less_005, bid128_signaling_less, 0x00000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_006, bid128_signaling_less, 0x00000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_signaling_less_007, bid128_signaling_less, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_signaling_less_008, bid128_signaling_less, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_signaling_less_009, bid128_signaling_less, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_010, bid128_signaling_less, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_011, bid128_signaling_less, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_signaling_less_012, bid128_signaling_less, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_signaling_less_013, bid128_signaling_less, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_014, bid128_signaling_less, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_015, bid128_signaling_less, 0x0004000000000000000000003b9dd75fu128, 0x00060000000000000000000005f62f23u128, false, 0x00);
dec_test!(bid128_signaling_less_016, bid128_signaling_less, 0x0004000000000000000000003b9dd75fu128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, true , 0x00);
dec_test!(bid128_signaling_less_017, bid128_signaling_less, 0x0004000000000000000000003b9dd75fu128, 0x80060000000000000000000005f62f23u128, false, 0x00);
dec_test!(bid128_signaling_less_018, bid128_signaling_less, 0x0004000000000000000000003b9dd75fu128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_019, bid128_signaling_less, 0x00060000000000000000000005f62f23u128, 0x00060000000000000000000005f62f23u128, false, 0x00);
dec_test!(bid128_signaling_less_020, bid128_signaling_less, 0x00060000000000000000000005f62f23u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, true , 0x00);
dec_test!(bid128_signaling_less_021, bid128_signaling_less, 0x00060000000000000000000005f62f23u128, 0x80060000000000000000000005f62f23u128, false, 0x00);
dec_test!(bid128_signaling_less_022, bid128_signaling_less, 0x00060000000000000000000005f62f23u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_023, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_signaling_less_024, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, true , 0x00);
dec_test!(bid128_signaling_less_025, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0x60000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_026, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0x6003b75d7734cd9e1234567890123456u128, false, 0x00);
dec_test!(bid128_signaling_less_027, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0x69dbb75d7734cd9e1234567890123456u128, false, 0x00);
dec_test!(bid128_signaling_less_028, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0x78000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_signaling_less_029, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0x7c000001000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_030, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0x7c003fffffffffffffffffffffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_031, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0x7e100001000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_032, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_033, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0xf8000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_signaling_less_034, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0xfc000001000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_035, bid128_signaling_less, 0x10000000000000000000000000000000u128, 0xfe200000000000100000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_036, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_signaling_less_037, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x10000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_038, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, true , 0x00);
dec_test!(bid128_signaling_less_039, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x5fffed09bead87c0378d8e63ffffffffu128, true , 0x00);
dec_test!(bid128_signaling_less_040, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x60000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_041, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x6003b75d7734cd9e1234567890123456u128, false, 0x00);
dec_test!(bid128_signaling_less_042, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x69dbb75d7734cd9e1234567890123456u128, false, 0x00);
dec_test!(bid128_signaling_less_043, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x78000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_044, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x78000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_signaling_less_045, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_046, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x7c000001000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_047, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x7c003fffffffffffffffffffffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_048, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0x7e100001000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_049, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_050, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0xb8000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_051, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0xdfffed09bead87c0378d8e63ffffffffu128, false, 0x00);
dec_test!(bid128_signaling_less_052, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0xf8000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_053, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0xf8000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_signaling_less_054, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0xfc000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_055, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0xfc000001000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_056, bid128_signaling_less, 0x302e000000000000000000001e1a7589u128, 0xfe200000000000100000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_057, bid128_signaling_less, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_signaling_less_058, bid128_signaling_less, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000003u128, true , 0x00);
dec_test!(bid128_signaling_less_059, bid128_signaling_less, 0x303e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_signaling_less_060, bid128_signaling_less, 0x303e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_signaling_less_061, bid128_signaling_less, 0x303e0000000000020000000000000000u128, 0x303e0000000000010000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_062, bid128_signaling_less, 0x303e0000000000020000000000000000u128, 0x303e0000000000030000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_063, bid128_signaling_less, 0x30400000001faa9fc5dcff096007ffffu128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_signaling_less_064, bid128_signaling_less, 0x30400000001faa9fc5dcff0960080000u128, 0x306600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_signaling_less_065, bid128_signaling_less, 0x30400000001faa9fc5dcff0960080001u128, 0x306600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_signaling_less_066, bid128_signaling_less, 0x3040000000fd54fe2ee7f84b003fffffu128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_signaling_less_067, bid128_signaling_less, 0x3040000000fd54fe2ee7f84b00400000u128, 0x306800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_signaling_less_068, bid128_signaling_less, 0x3040000000fd54fe2ee7f84b00400001u128, 0x306800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_signaling_less_069, bid128_signaling_less, 0x3040a6274bbdd0fadd61999e07ac0251u128, 0x3082a6274bbdd0fadd61999e07ac0250u128, true , 0x00);
dec_test!(bid128_signaling_less_070, bid128_signaling_less, 0x3040c612062576589dd46a73a100695eu128, 0x3074c612062576589dd46a73a100695du128, true , 0x00);
dec_test!(bid128_signaling_less_071, bid128_signaling_less, 0x3040c612062576589dda322d47eb47ffu128, 0x3074c612062576589dda322d47eb47feu128, true , 0x00);
dec_test!(bid128_signaling_less_072, bid128_signaling_less, 0x3041622d6fbc91e01277c0caded5b8a3u128, 0x3067622d6fbc91e01277c0caded5b8a2u128, true , 0x00);
dec_test!(bid128_signaling_less_073, bid128_signaling_less, 0x3041622d6fbc91e0127820b72d18cacfu128, 0x3067622d6fbc91e0127820b72d18caceu128, true , 0x00);
dec_test!(bid128_signaling_less_074, bid128_signaling_less, 0x30417361cb863de627fa3c5af3bb4c09u128, 0x30737361cb863de627fa3c5af3bb4c08u128, true , 0x00);
dec_test!(bid128_signaling_less_075, bid128_signaling_less, 0x3047ec3daf9417fe642eaec0d36a73d5u128, 0x307fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_076, bid128_signaling_less, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_077, bid128_signaling_less, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_078, bid128_signaling_less, 0x3047ec4450b72ff30000000000000000u128, 0x307fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_079, bid128_signaling_less, 0x3047ec4450b72ff30000000000000001u128, 0x307fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_080, bid128_signaling_less, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff096007ffffu128, false, 0x00);
dec_test!(bid128_signaling_less_081, bid128_signaling_less, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080000u128, false, 0x00);
dec_test!(bid128_signaling_less_082, bid128_signaling_less, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080001u128, true , 0x00);
dec_test!(bid128_signaling_less_083, bid128_signaling_less, 0x306600000000000000000000003a6a15u128, 0x306800000000000000000000003a6a16u128, true , 0x00);
dec_test!(bid128_signaling_less_084, bid128_signaling_less, 0x3067622d6fbc91e01277c0caded5b8a2u128, 0x3041622d6fbc91e01277c0caded5b8a3u128, false, 0x00);
dec_test!(bid128_signaling_less_085, bid128_signaling_less, 0x3067622d6fbc91e0127820b72d18caceu128, 0x3041622d6fbc91e0127820b72d18cacfu128, false, 0x00);
dec_test!(bid128_signaling_less_086, bid128_signaling_less, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b003fffffu128, false, 0x00);
dec_test!(bid128_signaling_less_087, bid128_signaling_less, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400000u128, false, 0x00);
dec_test!(bid128_signaling_less_088, bid128_signaling_less, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400001u128, true , 0x00);
dec_test!(bid128_signaling_less_089, bid128_signaling_less, 0x30737361cb863de627fa3c5af3bb4c08u128, 0x30417361cb863de627fa3c5af3bb4c09u128, false, 0x00);
dec_test!(bid128_signaling_less_090, bid128_signaling_less, 0x3074c612062576589dd46a73a100695du128, 0x3040c612062576589dd46a73a100695eu128, false, 0x00);
dec_test!(bid128_signaling_less_091, bid128_signaling_less, 0x3074c612062576589dda322d47eb47feu128, 0x3040c612062576589dda322d47eb47ffu128, false, 0x00);
dec_test!(bid128_signaling_less_092, bid128_signaling_less, 0x307fec3c64797fe80000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_signaling_less_093, bid128_signaling_less, 0x307fec4450b72ff30000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_signaling_less_094, bid128_signaling_less, 0x307fec4450b72ff30000000000000000u128, 0x3047ec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_095, bid128_signaling_less, 0x307fec4450b72ff30000000000000000u128, 0x3047ec4450b72ff30000000000000001u128, false, 0x00);
dec_test!(bid128_signaling_less_096, bid128_signaling_less, 0x30820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_signaling_less_097, bid128_signaling_less, 0x30820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_signaling_less_098, bid128_signaling_less, 0x3082a6274bbdd0fadd61999e07ac0250u128, 0x3040a6274bbdd0fadd61999e07ac0251u128, false, 0x00);
dec_test!(bid128_signaling_less_099, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x10000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_100, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_101, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x5fffed09bead87c0378d8e63ffffffffu128, false, 0x00);
dec_test!(bid128_signaling_less_102, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x78000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_103, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_104, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_105, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0xb8000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_106, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0xdfffed09bead87c0378d8e63ffffffffu128, false, 0x00);
dec_test!(bid128_signaling_less_107, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0xf8000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_108, bid128_signaling_less, 0x5fffed09bead87c0378d8e63ffffffffu128, 0xfc000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_109, bid128_signaling_less, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_110, bid128_signaling_less, 0x78000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_111, bid128_signaling_less, 0x78000000000000000000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_112, bid128_signaling_less, 0x78000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_113, bid128_signaling_less, 0x78000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_114, bid128_signaling_less, 0x78000000000000000000000000000000u128, 0xfe000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_115, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_signaling_less_116, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0x10000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_117, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_118, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0x69dbb75d7734cd9e1234567890123456u128, false, 0x00);
dec_test!(bid128_signaling_less_119, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0x78000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_120, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_signaling_less_121, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_122, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_123, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_signaling_less_124, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_125, bid128_signaling_less, 0x78000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_signaling_less_126, bid128_signaling_less, 0x7c000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_127, bid128_signaling_less, 0x7c000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_128, bid128_signaling_less, 0x7c000000000000000000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_129, bid128_signaling_less, 0x7c000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_130, bid128_signaling_less, 0x7c000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_131, bid128_signaling_less, 0x7c000000000000000000000000000000u128, 0xfe000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_132, bid128_signaling_less, 0x7c000001000000000000000000000000u128, 0x10000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_133, bid128_signaling_less, 0x7c000001000000000000000000000000u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
dec_test!(bid128_signaling_less_134, bid128_signaling_less, 0x7c000001000000000000000000000000u128, 0x69dbb75d7734cd9e1234567890123456u128, false, 0x01);
dec_test!(bid128_signaling_less_135, bid128_signaling_less, 0x7c000001000000000000000000000000u128, 0x78000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_136, bid128_signaling_less, 0x7c000001000000000000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_137, bid128_signaling_less, 0x7c000001000000000000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_138, bid128_signaling_less, 0x7c000001000000000000000000000000u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
dec_test!(bid128_signaling_less_139, bid128_signaling_less, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_140, bid128_signaling_less, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_141, bid128_signaling_less, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_142, bid128_signaling_less, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_143, bid128_signaling_less, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_144, bid128_signaling_less, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_145, bid128_signaling_less, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_146, bid128_signaling_less, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x01);
dec_test!(bid128_signaling_less_147, bid128_signaling_less, 0x7c003fffffffffffffffffffffffffffu128, 0x10000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_148, bid128_signaling_less, 0x7c003fffffffffffffffffffffffffffu128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
dec_test!(bid128_signaling_less_149, bid128_signaling_less, 0x7c003fffffffffffffffffffffffffffu128, 0x69dbb75d7734cd9e1234567890123456u128, false, 0x01);
dec_test!(bid128_signaling_less_150, bid128_signaling_less, 0x7c003fffffffffffffffffffffffffffu128, 0x78000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_151, bid128_signaling_less, 0x7c003fffffffffffffffffffffffffffu128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_152, bid128_signaling_less, 0x7c003fffffffffffffffffffffffffffu128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_153, bid128_signaling_less, 0x7c003fffffffffffffffffffffffffffu128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
dec_test!(bid128_signaling_less_154, bid128_signaling_less, 0x7e000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_155, bid128_signaling_less, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_156, bid128_signaling_less, 0x7e000000000000000000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_157, bid128_signaling_less, 0x7e000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_158, bid128_signaling_less, 0x7e000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_159, bid128_signaling_less, 0x7e000000000000000000000000000000u128, 0xfe000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_160, bid128_signaling_less, 0x7e100001000000000000000000000000u128, 0x10000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_161, bid128_signaling_less, 0x7e100001000000000000000000000000u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
dec_test!(bid128_signaling_less_162, bid128_signaling_less, 0x7e100001000000000000000000000000u128, 0x69dbb75d7734cd9e1234567890123456u128, false, 0x01);
dec_test!(bid128_signaling_less_163, bid128_signaling_less, 0x7e100001000000000000000000000000u128, 0x78000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_164, bid128_signaling_less, 0x7e100001000000000000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_165, bid128_signaling_less, 0x7e100001000000000000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_166, bid128_signaling_less, 0x7e100001000000000000000000000000u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
dec_test!(bid128_signaling_less_167, bid128_signaling_less, 0x80000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_signaling_less_168, bid128_signaling_less, 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_signaling_less_169, bid128_signaling_less, 0x80000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_170, bid128_signaling_less, 0x80000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_signaling_less_171, bid128_signaling_less, 0x80000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_172, bid128_signaling_less, 0x80000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_signaling_less_173, bid128_signaling_less, 0x8004000000000000000000003b9dd75fu128, 0x00060000000000000000000005f62f23u128, true , 0x00);
dec_test!(bid128_signaling_less_174, bid128_signaling_less, 0x8004000000000000000000003b9dd75fu128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, true , 0x00);
dec_test!(bid128_signaling_less_175, bid128_signaling_less, 0x8004000000000000000000003b9dd75fu128, 0x80060000000000000000000005f62f23u128, true , 0x00);
dec_test!(bid128_signaling_less_176, bid128_signaling_less, 0x8004000000000000000000003b9dd75fu128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_177, bid128_signaling_less, 0x80060000000000000000000005f62f23u128, 0x00060000000000000000000005f62f23u128, true , 0x00);
dec_test!(bid128_signaling_less_178, bid128_signaling_less, 0x80060000000000000000000005f62f23u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, true , 0x00);
dec_test!(bid128_signaling_less_179, bid128_signaling_less, 0x80060000000000000000000005f62f23u128, 0x80060000000000000000000005f62f23u128, false, 0x00);
dec_test!(bid128_signaling_less_180, bid128_signaling_less, 0x80060000000000000000000005f62f23u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x00);
dec_test!(bid128_signaling_less_181, bid128_signaling_less, 0xb03e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_signaling_less_182, bid128_signaling_less, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_signaling_less_183, bid128_signaling_less, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000003u128, false, 0x00);
dec_test!(bid128_signaling_less_184, bid128_signaling_less, 0xb03e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_signaling_less_185, bid128_signaling_less, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000010000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_186, bid128_signaling_less, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000030000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_187, bid128_signaling_less, 0xb0400000001faa9fc5dcff096007ffffu128, 0xb06600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_signaling_less_188, bid128_signaling_less, 0xb0400000001faa9fc5dcff0960080000u128, 0xb06600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_signaling_less_189, bid128_signaling_less, 0xb0400000001faa9fc5dcff0960080001u128, 0xb06600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_signaling_less_190, bid128_signaling_less, 0xb040000000fd54fe2ee7f84b003fffffu128, 0xb06800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_signaling_less_191, bid128_signaling_less, 0xb040000000fd54fe2ee7f84b00400000u128, 0xb06800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_signaling_less_192, bid128_signaling_less, 0xb040000000fd54fe2ee7f84b00400001u128, 0xb06800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_signaling_less_193, bid128_signaling_less, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_194, bid128_signaling_less, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_195, bid128_signaling_less, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff096007ffffu128, true , 0x00);
dec_test!(bid128_signaling_less_196, bid128_signaling_less, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff0960080000u128, false, 0x00);
dec_test!(bid128_signaling_less_197, bid128_signaling_less, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff0960080001u128, false, 0x00);
dec_test!(bid128_signaling_less_198, bid128_signaling_less, 0xb06600000000000000000000003a6a15u128, 0xb06800000000000000000000003a6a16u128, false, 0x00);
dec_test!(bid128_signaling_less_199, bid128_signaling_less, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b003fffffu128, true , 0x00);
dec_test!(bid128_signaling_less_200, bid128_signaling_less, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b00400000u128, false, 0x00);
dec_test!(bid128_signaling_less_201, bid128_signaling_less, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b00400001u128, false, 0x00);
dec_test!(bid128_signaling_less_202, bid128_signaling_less, 0xb07fec3c64797fe80000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_signaling_less_203, bid128_signaling_less, 0xb07fec4450b72ff30000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_signaling_less_204, bid128_signaling_less, 0xb0820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_signaling_less_205, bid128_signaling_less, 0xb0820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_signaling_less_206, bid128_signaling_less, 0xf8000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_207, bid128_signaling_less, 0xf8000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_208, bid128_signaling_less, 0xf8000000000000000000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_209, bid128_signaling_less, 0xf8000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_signaling_less_210, bid128_signaling_less, 0xf8000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_211, bid128_signaling_less, 0xf8000000000000000000000000000000u128, 0xfe000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_212, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_signaling_less_213, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0x10000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_214, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, true , 0x00);
dec_test!(bid128_signaling_less_215, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0x69dbb75d7734cd9e1234567890123456u128, true , 0x00);
dec_test!(bid128_signaling_less_216, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0x78000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_signaling_less_217, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_signaling_less_218, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_219, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_220, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_signaling_less_221, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, true , 0x00);
dec_test!(bid128_signaling_less_222, bid128_signaling_less, 0xf8000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_signaling_less_223, bid128_signaling_less, 0xfc000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_224, bid128_signaling_less, 0xfc000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_225, bid128_signaling_less, 0xfc000000000000000000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_226, bid128_signaling_less, 0xfc000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_227, bid128_signaling_less, 0xfc000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_228, bid128_signaling_less, 0xfc000000000000000000000000000000u128, 0xfe000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_229, bid128_signaling_less, 0xfc000001000000000000000000000000u128, 0x10000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_230, bid128_signaling_less, 0xfc000001000000000000000000000000u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
dec_test!(bid128_signaling_less_231, bid128_signaling_less, 0xfc000001000000000000000000000000u128, 0x69dbb75d7734cd9e1234567890123456u128, false, 0x01);
dec_test!(bid128_signaling_less_232, bid128_signaling_less, 0xfc000001000000000000000000000000u128, 0x78000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_233, bid128_signaling_less, 0xfc000001000000000000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_234, bid128_signaling_less, 0xfc000001000000000000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_235, bid128_signaling_less, 0xfc000001000000000000000000000000u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
dec_test!(bid128_signaling_less_236, bid128_signaling_less, 0xfe000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_237, bid128_signaling_less, 0xfe000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_238, bid128_signaling_less, 0xfe000000000000000000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_239, bid128_signaling_less, 0xfe000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_240, bid128_signaling_less, 0xfe000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_241, bid128_signaling_less, 0xfe000000000000000000000000000000u128, 0xfe000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_242, bid128_signaling_less, 0xfe200000000000100000000000000000u128, 0x10000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_243, bid128_signaling_less, 0xfe200000000000100000000000000000u128, 0x2ffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
dec_test!(bid128_signaling_less_244, bid128_signaling_less, 0xfe200000000000100000000000000000u128, 0x69dbb75d7734cd9e1234567890123456u128, false, 0x01);
dec_test!(bid128_signaling_less_245, bid128_signaling_less, 0xfe200000000000100000000000000000u128, 0x78000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_246, bid128_signaling_less, 0xfe200000000000100000000000000000u128, 0x7c000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_247, bid128_signaling_less, 0xfe200000000000100000000000000000u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_signaling_less_248, bid128_signaling_less, 0xfe200000000000100000000000000000u128, 0xaffcf90255f4f60e8f4e3d819a000001u128, false, 0x01);
