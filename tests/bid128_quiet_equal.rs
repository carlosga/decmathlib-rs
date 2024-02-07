/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_quiet_equal_001, bid128_quiet_equal, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_002, bid128_quiet_equal, 0x00000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_003, bid128_quiet_equal, 0x00000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_004, bid128_quiet_equal, 0x00000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_005, bid128_quiet_equal, 0x00000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_006, bid128_quiet_equal, 0x00000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_007, bid128_quiet_equal, 0x0000000000000000ffffffffffffdfffu128, 0x0000000000000000ffffffffffffdfffu128, true , 0x00);
dec_test!(bid128_quiet_equal_008, bid128_quiet_equal, 0x00000001028000024202004882880109u128, 0x0040108000002000dfffff7effffb7dfu128, false, 0x00);
dec_test!(bid128_quiet_equal_009, bid128_quiet_equal, 0x0000100000104000f4fe7bf7f4f4e9d7u128, 0x000002000000000098014d4381200061u128, false, 0x00);
dec_test!(bid128_quiet_equal_010, bid128_quiet_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_equal_011, bid128_quiet_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_012, bid128_quiet_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_013, bid128_quiet_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_014, bid128_quiet_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_015, bid128_quiet_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_equal_016, bid128_quiet_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_017, bid128_quiet_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_018, bid128_quiet_equal, "-0"                                  , "SNaN"                                , false, 0x01);
dec_test!(bid128_quiet_equal_019, bid128_quiet_equal, 0x13eafeb08cf2b0eea88565fef0a9db68u128, 0x2825c5e8c106a2f5f7fffbff7ffffeffu128, false, 0x00);
dec_test!(bid128_quiet_equal_020, bid128_quiet_equal, 0x29bfc0ccbe61a69d5a431b9c8d34f2c9u128, 0x00606021041d0018dd983e4140352304u128, false, 0x00);
dec_test!(bid128_quiet_equal_021, bid128_quiet_equal, 0x2a9b19d76c0b139321b8008859c221c4u128, 0xfb34fc8e85bbbe9529a2d6daed92f0c5u128, false, 0x00);
dec_test!(bid128_quiet_equal_022, bid128_quiet_equal, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_023, bid128_quiet_equal, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000003u128, false, 0x00);
dec_test!(bid128_quiet_equal_024, bid128_quiet_equal, 0x303e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_025, bid128_quiet_equal, 0x303e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_026, bid128_quiet_equal, 0x303e0000000000020000000000000000u128, 0x303e0000000000010000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_027, bid128_quiet_equal, 0x303e0000000000020000000000000000u128, 0x303e0000000000030000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_028, bid128_quiet_equal, 0x303ea05e84ee29bc16cfd0dba062ec02u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_quiet_equal_029, bid128_quiet_equal, 0x30400000001faa9fc5dcff096007ffffu128, 0x306600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_equal_030, bid128_quiet_equal, 0x30400000001faa9fc5dcff0960080000u128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_equal_031, bid128_quiet_equal, 0x30400000001faa9fc5dcff0960080001u128, 0x306600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_equal_032, bid128_quiet_equal, 0x3040000000fd54fe2ee7f84b003fffffu128, 0x306800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_equal_033, bid128_quiet_equal, 0x3040000000fd54fe2ee7f84b00400000u128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_equal_034, bid128_quiet_equal, 0x3040000000fd54fe2ee7f84b00400001u128, 0x306800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_equal_035, bid128_quiet_equal, 0x3040a6274bbdd0fadd61999e07ac0251u128, 0x3082a6274bbdd0fadd61999e07ac0250u128, false, 0x00);
dec_test!(bid128_quiet_equal_036, bid128_quiet_equal, 0x3040c612062576589dd46a73a100695eu128, 0x3074c612062576589dd46a73a100695du128, false, 0x00);
dec_test!(bid128_quiet_equal_037, bid128_quiet_equal, 0x3040c612062576589dda322d47eb47ffu128, 0x3074c612062576589dda322d47eb47feu128, false, 0x00);
dec_test!(bid128_quiet_equal_038, bid128_quiet_equal, 0x3041622d6fbc91e01277c0caded5b8a3u128, 0x3067622d6fbc91e01277c0caded5b8a2u128, false, 0x00);
dec_test!(bid128_quiet_equal_039, bid128_quiet_equal, 0x3041622d6fbc91e0127820b72d18cacfu128, 0x3067622d6fbc91e0127820b72d18caceu128, false, 0x00);
dec_test!(bid128_quiet_equal_040, bid128_quiet_equal, 0x30417361cb863de627fa3c5af3bb4c09u128, 0x30737361cb863de627fa3c5af3bb4c08u128, false, 0x00);
// -- ##### File bid128_compare.c, function bid128_quiet_equal(), line 151 ################################
// -- Here when exp_y-exp_x>19, carry CY1=0.
dec_test!(bid128_quiet_equal_041, bid128_quiet_equal, 0x3042314DC6448D9338C15B0A00000000u128, 0x3042314DC6448D9338C15B0A00000000u128, true , 0x00);
dec_test!(bid128_quiet_equal_042, bid128_quiet_equal, 0x3042314DC6448D9338C15B0A00000000u128, 0x304404EE2D6D415B85ACEF8100000000u128, true , 0x00);
// -- ##### File bid128_compare.c, function bid128_quiet_equal(), line 151 ################################
// -- Here when exp_y-exp_x>19, carry CY1=1.
// -- The same point for case when exp_y-exp_x>19, carry CY2=0.
dec_test!(bid128_quiet_equal_043, bid128_quiet_equal, 0x3042314DC6448D9338C15B0A00000000u128, 0x307000000000021E19E0C9BAB2400000u128, false, 0x00);
dec_test!(bid128_quiet_equal_044, bid128_quiet_equal, 0x3042314DC6448D9338C15B0A00000000u128, 0x3082000000000000000000000000000Au128, true , 0x00);
dec_test!(bid128_quiet_equal_045, bid128_quiet_equal, 0x3042314DC6448D9338C15B0A00000000u128, 0x30840000000000000000000000000001u128, true , 0x00);
// -- ##### File bid128_compare.c, function bid128_quiet_equal(), line 151 ################################
// -- Here when exp_y-exp_x>19, carry CY2=1, case when in macro __add_carry_in_out (called from
// -- __mul_128x128_to_256 macro), carry CY2 appeared in S=X1+Y (not in X1=X+CI).
dec_test!(bid128_quiet_equal_046, bid128_quiet_equal, 0x3042314DC6448D9338C15B0A00000000u128, 0x30845313A5E419B7ffffffffffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_047, bid128_quiet_equal, 0x3047ec3daf9417fe642eaec0d36a73d5u128, 0x307fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_048, bid128_quiet_equal, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_049, bid128_quiet_equal, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_050, bid128_quiet_equal, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff096007ffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_051, bid128_quiet_equal, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080000u128, true , 0x00);
dec_test!(bid128_quiet_equal_052, bid128_quiet_equal, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080001u128, false, 0x00);
dec_test!(bid128_quiet_equal_053, bid128_quiet_equal, 0x3067622d6fbc91e01277c0caded5b8a2u128, 0x3041622d6fbc91e01277c0caded5b8a3u128, false, 0x00);
dec_test!(bid128_quiet_equal_054, bid128_quiet_equal, 0x3067622d6fbc91e0127820b72d18caceu128, 0x3041622d6fbc91e0127820b72d18cacfu128, false, 0x00);
dec_test!(bid128_quiet_equal_055, bid128_quiet_equal, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b003fffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_056, bid128_quiet_equal, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400000u128, true , 0x00);
dec_test!(bid128_quiet_equal_057, bid128_quiet_equal, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400001u128, false, 0x00);
// -- ##### File bid128_compare.c, function bid128_quiet_equal(), line 151 ################################
// -- Here when exp_y-exp_x>19, carry CY2=1, case when in macro __add_carry_in_out (called from
// -- __mul_128x128_to_256 macro), carry CY2 appeared in X1=X+CI (not in S=X1+Y), hence CY1=1, X=0xF..F.
// -- NOT FOUND!!!   (The block is maybe unattainable but the code in the block is correct.)
// -- ##### File bid128_compare.c, function bid128_quiet_equal(), line 163 ################################
// -- Here when exp_y-exp_x<=19, in __add_128_64 macro, (B64)+(A128).w[0] gives carry=0.
dec_test!(bid128_quiet_equal_058a, bid128_quiet_equal, 0x306C314DC6448D9338C15B0A00000000u128, 0x308519799812DEA11197F27F0F6E885Cu128, false, 0x00);
// -- ##### File bid128_compare.c, function bid128_quiet_equal(), line 163 ################################
// -- Here when exp_y-exp_x<=19, in __add_128_64 macro, (B64)+(A128).w[0] gives carry=1.
dec_test!(bid128_quiet_equal_058, bid128_quiet_equal, 0x306C314DC6448D9338C15B0A00000000u128, 0x308519799812DEA112B16C17224D296Eu128, false, 0x00);
dec_test!(bid128_quiet_equal_059, bid128_quiet_equal, 0x30737361cb863de627fa3c5af3bb4c08u128, 0x30417361cb863de627fa3c5af3bb4c09u128, false, 0x00);
dec_test!(bid128_quiet_equal_060, bid128_quiet_equal, 0x3074c612062576589dd46a73a100695du128, 0x3040c612062576589dd46a73a100695eu128, false, 0x00);
dec_test!(bid128_quiet_equal_061, bid128_quiet_equal, 0x3074c612062576589dda322d47eb47feu128, 0x3040c612062576589dda322d47eb47ffu128, false, 0x00);
dec_test!(bid128_quiet_equal_062, bid128_quiet_equal, 0x307fec3c64797fe80000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_equal_063, bid128_quiet_equal, 0x307fec4450b72ff30000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_equal_064, bid128_quiet_equal, 0x30820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_065, bid128_quiet_equal, 0x30820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_066, bid128_quiet_equal, 0x3082a6274bbdd0fadd61999e07ac0250u128, 0x3040a6274bbdd0fadd61999e07ac0251u128, false, 0x00);
dec_test!(bid128_quiet_equal_067, bid128_quiet_equal, 0x36ff097bb57ad97a9101b83330481093u128, 0x26a60000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_068, bid128_quiet_equal, 0x39e3a16cd407e3c142ecfe03a2adfaf3u128, 0x123f76c748daeeff3988979f0f80136cu128, false, 0x00);
dec_test!(bid128_quiet_equal_069, bid128_quiet_equal, 0x3cde01f2a15129e9d550c601ce532c8bu128, 0xd9fffdbfffdbfef5fffffdf7fbefffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_070, bid128_quiet_equal, 0x3f160000000000000000000000000000u128, 0xd6679fa9084c03e449ed8e354744584au128, false, 0x00);
dec_test!(bid128_quiet_equal_071, bid128_quiet_equal, 0x78000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, false, 0x00);
// -- ##### File bid128_compare.c, function bid128_quiet_equal(), line 64 #################################
// -- Here when x=inf, y=inf, but bitwise x!=y
dec_test!(bid128_quiet_equal_072, bid128_quiet_equal, 0x78000000000000000000000000000001u128, 0x78000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_equal_073, bid128_quiet_equal, 0x78000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_equal_074, bid128_quiet_equal, 0x78000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_075, bid128_quiet_equal, 0x78000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_076, bid128_quiet_equal, 0x7a9fd79c78ff3ce0dbf3dc3e7c1ccf65u128, 0xac83468d0d150bb133ea21589480c46cu128, false, 0x00);
dec_test!(bid128_quiet_equal_077, bid128_quiet_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_078, bid128_quiet_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_079, bid128_quiet_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_080, bid128_quiet_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_081, bid128_quiet_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_082, bid128_quiet_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_083, bid128_quiet_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_084, bid128_quiet_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_equal_085, bid128_quiet_equal, 0x80000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_086, bid128_quiet_equal, 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_087, bid128_quiet_equal, 0x80000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_088, bid128_quiet_equal, 0x80000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_089, bid128_quiet_equal, 0x80000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_090, bid128_quiet_equal, 0x80000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_091, bid128_quiet_equal, 0x80dd09102018d470992510210d60904au128, 0xffdf8fefbffbff6cffedffeefdffdfffu128, false, 0x01);
dec_test!(bid128_quiet_equal_092, bid128_quiet_equal, 0x8859d596f889650c19ef6c04d3156c95u128, 0x7e000a3a5ca3ccbbea9d0efa4f2a9740u128, false, 0x01);
dec_test!(bid128_quiet_equal_093, bid128_quiet_equal, 0x97979adf7638436cb2e950f93e6d45fau128, 0x4d6d5a81bdd44f3ddee17f32bfcb9862u128, false, 0x00);
dec_test!(bid128_quiet_equal_094, bid128_quiet_equal, 0xa4dc52e0591e0e430b9c6c8ea68f4ee2u128, 0x78000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_095, bid128_quiet_equal, 0xb03e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_096, bid128_quiet_equal, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_097, bid128_quiet_equal, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000003u128, false, 0x00);
dec_test!(bid128_quiet_equal_098, bid128_quiet_equal, 0xb03e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_equal_099, bid128_quiet_equal, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000010000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_100, bid128_quiet_equal, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000030000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_101, bid128_quiet_equal, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_102, bid128_quiet_equal, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_equal_103, bid128_quiet_equal, 0xb07fec3c64797fe80000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_equal_104, bid128_quiet_equal, 0xb07fec4450b72ff30000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_equal_105, bid128_quiet_equal, 0xb0820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_106, bid128_quiet_equal, 0xb0820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_107, bid128_quiet_equal, 0xc02a10fff39ef71039b9cb90aa0544fcu128, 0x41d37d29bdeabf6e38bcbf073226ecbcu128, false, 0x00);
dec_test!(bid128_quiet_equal_108, bid128_quiet_equal, 0xcd83df8a3886cf4d53c0cc10b3b3b457u128, 0x88ec8ba89e5e93add40d163ec5c81fd0u128, false, 0x00);
dec_test!(bid128_quiet_equal_109, bid128_quiet_equal, 0xd89e72da4917665d84232245b5e3113fu128, 0x5069cbf34f7418d3714e9a9ad7ba7745u128, false, 0x00);
dec_test!(bid128_quiet_equal_110, bid128_quiet_equal, 0xde3d3f733ac4ab0362c5136e5b52331fu128, 0x9af92a693403e3e561579b50bc48856au128, false, 0x00);
dec_test!(bid128_quiet_equal_111, bid128_quiet_equal, 0xee8ffc67bfdcd6452000bc0032042425u128, 0xe90b11c56431463a0216455541858d12u128, true , 0x00);
dec_test!(bid128_quiet_equal_112, bid128_quiet_equal, 0xefb1ff1252bd9a71b62e6584e6ead4d7u128, 0x4500c8058002e08070b8b80ad877344cu128, false, 0x00);
dec_test!(bid128_quiet_equal_113, bid128_quiet_equal, 0xf8000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_114, bid128_quiet_equal, 0xf8000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_115, bid128_quiet_equal, 0xf8000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_equal_116, bid128_quiet_equal, 0xf8000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_equal_117, bid128_quiet_equal, 0xfbdfffdfffffffff08100f4400212804u128, 0x78b44225c7cd73f81bebf9534706ac74u128, false, 0x00);
dec_test!(bid128_quiet_equal_118, bid128_quiet_equal, "-Infinity"                           ,      "SNaN"                           , false, 0x01);
dec_test!(bid128_quiet_equal_119, bid128_quiet_equal,  "Infinity"                           ,      "SNaN"                           , false, 0x01);
dec_test!(bid128_quiet_equal_120, bid128_quiet_equal,      "QNaN"                           ,         "0"                           , false, 0x00);
dec_test!(bid128_quiet_equal_121, bid128_quiet_equal,      "SNaN"                           ,        "-0"                           , false, 0x01);
dec_test!(bid128_quiet_equal_122, bid128_quiet_equal,      "SNaN"                           , "-Infinity"                           , false, 0x01);
