/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_quiet_greater_equal_001, bid128_quiet_greater_equal, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_002, bid128_quiet_greater_equal, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_003, bid128_quiet_greater_equal, 0x00000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_004, bid128_quiet_greater_equal, 0x00000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_005, bid128_quiet_greater_equal, 0x00000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_006, bid128_quiet_greater_equal, 0x00000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_007, bid128_quiet_greater_equal, 0x00000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_008, bid128_quiet_greater_equal, 0x0000000000000000dedc0646b2cdc378u128, 0x0000000000080000ef9cffdf7f6f7fbfu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_009, bid128_quiet_greater_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_010, bid128_quiet_greater_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_011, bid128_quiet_greater_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_012, bid128_quiet_greater_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_013, bid128_quiet_greater_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_014, bid128_quiet_greater_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_015, bid128_quiet_greater_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_016, bid128_quiet_greater_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_017, bid128_quiet_greater_equal, "-0"                                  ,  "0"                                  , true , 0x00);
dec_test!(bid128_quiet_greater_equal_018, bid128_quiet_greater_equal,  "0"                                  , "-0"                                  , true , 0x00);
dec_test!(bid128_quiet_greater_equal_019, bid128_quiet_greater_equal, 0x0040000000040008eed861be4b942b33u128, 0x00000000004000007fdfffffffffbbdfu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_020, bid128_quiet_greater_equal, 0x021d6190180865d0ffe7ff7eeef3ad76u128, 0x02540624385827122cb7163af7ee0939u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_021, bid128_quiet_greater_equal, 0x02602effba0526d499fb99923a16b2ebu128, 0x3205deaa2ba86da1e60ccb02f8bb7517u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_022, bid128_quiet_greater_equal, 0x0900000141610280d4adee151fc17ebfu128, 0x59c655f8e6da985b4435241d66cc2b50u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_023, bid128_quiet_greater_equal, 0x0effa7c81f16a6ef4bf08c566cc1009au128, 0x23a3c5156d4ce69c6cecc44e902aff62u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_024, bid128_quiet_greater_equal,  "-0"                                 , "-Infinity"                           , true , 0x00);
dec_test!(bid128_quiet_greater_equal_025, bid128_quiet_greater_equal,  "-0"                                 ,      "SNaN"                           , false, 0x01);
dec_test!(bid128_quiet_greater_equal_026, bid128_quiet_greater_equal, "1.0"                                 ,         "1"                           , true , 0x00);
dec_test!(bid128_quiet_greater_equal_027, bid128_quiet_greater_equal,   "1"                                 ,       "1.0"                           , true , 0x00);
dec_test!(bid128_quiet_greater_equal_028, bid128_quiet_greater_equal, 0x2679d9c31333fe0f3a070a117fb40f81u128, 0x9a69767071ac26ed80811281030a7120u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_029, bid128_quiet_greater_equal, 0x2cfac81c79e9e03e82f2cd0bb6fb5014u128, 0x7e000000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_quiet_greater_equal_030, bid128_quiet_greater_equal, 0x2e5f56c87e7c693f09915cbb57003441u128, 0x7e00023c76ba7e9bc1388ef6450ebc10u128, false, 0x01);
dec_test!(bid128_quiet_greater_equal_031, bid128_quiet_greater_equal, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_032, bid128_quiet_greater_equal, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000003u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_033, bid128_quiet_greater_equal, 0x303e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_034, bid128_quiet_greater_equal, 0x303e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_035, bid128_quiet_greater_equal, 0x303e0000000000020000000000000000u128, 0x303e0000000000010000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_036, bid128_quiet_greater_equal, 0x303e0000000000020000000000000000u128, 0x303e0000000000030000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_037, bid128_quiet_greater_equal, 0x30400000001faa9fc5dcff096007ffffu128, 0x306600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_038, bid128_quiet_greater_equal, 0x30400000001faa9fc5dcff0960080000u128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_039, bid128_quiet_greater_equal, 0x30400000001faa9fc5dcff0960080001u128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_040, bid128_quiet_greater_equal, 0x3040000000fd54fe2ee7f84b003fffffu128, 0x306800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_041, bid128_quiet_greater_equal, 0x3040000000fd54fe2ee7f84b00400000u128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_042, bid128_quiet_greater_equal, 0x3040000000fd54fe2ee7f84b00400001u128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_043, bid128_quiet_greater_equal, 0x3040a6274bbdd0fadd61999e07ac0251u128, 0x3082a6274bbdd0fadd61999e07ac0250u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_044, bid128_quiet_greater_equal, 0x3040c612062576589dd46a73a100695eu128, 0x3074c612062576589dd46a73a100695du128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_045, bid128_quiet_greater_equal, 0x3040c612062576589dda322d47eb47ffu128, 0x3074c612062576589dda322d47eb47feu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_046, bid128_quiet_greater_equal, 0x3041622d6fbc91e01277c0caded5b8a3u128, 0x3067622d6fbc91e01277c0caded5b8a2u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_047, bid128_quiet_greater_equal, 0x3041622d6fbc91e0127820b72d18cacfu128, 0x3067622d6fbc91e0127820b72d18caceu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_048, bid128_quiet_greater_equal, 0x30417361cb863de627fa3c5af3bb4c09u128, 0x30737361cb863de627fa3c5af3bb4c08u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_049, bid128_quiet_greater_equal, 0x3047ec3daf9417fe642eaec0d36a73d5u128, 0x307fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_050, bid128_quiet_greater_equal, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_051, bid128_quiet_greater_equal, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_052, bid128_quiet_greater_equal, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff096007ffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_053, bid128_quiet_greater_equal, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_054, bid128_quiet_greater_equal, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080001u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_055, bid128_quiet_greater_equal, 0x306600000000000000000000003a6a15u128, 0x306800000000000000000000003a6a16u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_056, bid128_quiet_greater_equal, 0x3067622d6fbc91e01277c0caded5b8a2u128, 0x3041622d6fbc91e01277c0caded5b8a3u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_057, bid128_quiet_greater_equal, 0x3067622d6fbc91e0127820b72d18caceu128, 0x3041622d6fbc91e0127820b72d18cacfu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_058, bid128_quiet_greater_equal, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b003fffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_059, bid128_quiet_greater_equal, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_060, bid128_quiet_greater_equal, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400001u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_061, bid128_quiet_greater_equal, 0x30737361cb863de627fa3c5af3bb4c08u128, 0x30417361cb863de627fa3c5af3bb4c09u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_062, bid128_quiet_greater_equal, 0x3074c612062576589dd46a73a100695du128, 0x3040c612062576589dd46a73a100695eu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_063, bid128_quiet_greater_equal, 0x3074c612062576589dda322d47eb47feu128, 0x3040c612062576589dda322d47eb47ffu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_064, bid128_quiet_greater_equal, 0x307fec3c64797fe80000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_065, bid128_quiet_greater_equal, 0x307fec4450b72ff30000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_066, bid128_quiet_greater_equal, 0x30820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_067, bid128_quiet_greater_equal, 0x30820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_068, bid128_quiet_greater_equal, 0x3082a6274bbdd0fadd61999e07ac0250u128, 0x3040a6274bbdd0fadd61999e07ac0251u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_069, bid128_quiet_greater_equal, 0x400000000000000038b6b28d657c531eu128, 0x40100000040000001800000000000010u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_070, bid128_quiet_greater_equal, 0x45c54e2c27baef9214d77e0ee1f808f0u128, 0x08bd088818102bc10000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_071, bid128_quiet_greater_equal, 0x4fb87ed44b5e43d45919f2c08cb84d68u128, 0x1847729e4d9783e67be0dd15cb9f9563u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_072, bid128_quiet_greater_equal, 0x50408020c400140820d5004258cc2d10u128, 0x503cdca2004e0110400020100c064388u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_073, bid128_quiet_greater_equal, "-589.877959968857E0"                 , "-0"                                  , false, 0x00);
dec_test!(bid128_quiet_greater_equal_074, bid128_quiet_greater_equal, 0x6ddf270aacd87608c408220397400100u128, 0x40a4860005208804f6f3957ffbc6c2e7u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_075, bid128_quiet_greater_equal, 0x78000000000000000000000000000000u128, 0xd7a1c5c5a282a420e1e067a97f712785u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_076, bid128_quiet_greater_equal, 0x78000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_077, bid128_quiet_greater_equal, 0x78000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_078, bid128_quiet_greater_equal, 0x78000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_079, bid128_quiet_greater_equal, 0x78000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_080, bid128_quiet_greater_equal, 0x7a4db95ff5c4f5bc0000000000000000u128, 0xa99913a67020b5f8960c20aaf6bcc77bu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_081, bid128_quiet_greater_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_082, bid128_quiet_greater_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_083, bid128_quiet_greater_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_084, bid128_quiet_greater_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_085, bid128_quiet_greater_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_086, bid128_quiet_greater_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_087, bid128_quiet_greater_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_088, bid128_quiet_greater_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_089, bid128_quiet_greater_equal, 0x80000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_090, bid128_quiet_greater_equal, 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_091, bid128_quiet_greater_equal, 0x80000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_092, bid128_quiet_greater_equal, 0x80000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_093, bid128_quiet_greater_equal, 0x80000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_094, bid128_quiet_greater_equal, 0x80000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_095, bid128_quiet_greater_equal, 0x9289c3a1997d378bdee215345c2e6e1cu128, 0xa0c00000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_096, bid128_quiet_greater_equal, 0x96461f718877f10e7f8e6392049d3e68u128, 0x944c0000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_097, bid128_quiet_greater_equal, 0xaa540c37e20b231eebbbdae11d1cf9d0u128, 0xfbdd6df3f57abb0f02081d010692c4d3u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_098, bid128_quiet_greater_equal, 0xacf68f8a36d72a61a5a0e3464cfe7dc5u128, 0x55700000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_099, bid128_quiet_greater_equal, 0xb03e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_100, bid128_quiet_greater_equal, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_101, bid128_quiet_greater_equal, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000003u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_102, bid128_quiet_greater_equal, 0xb03e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_103, bid128_quiet_greater_equal, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000010000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_104, bid128_quiet_greater_equal, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000030000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_105, bid128_quiet_greater_equal, 0xb0400000001faa9fc5dcff096007ffffu128, 0xb06600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_106, bid128_quiet_greater_equal, 0xb0400000001faa9fc5dcff0960080000u128, 0xb06600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_107, bid128_quiet_greater_equal, 0xb0400000001faa9fc5dcff0960080001u128, 0xb06600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_108, bid128_quiet_greater_equal, 0xb040000000fd54fe2ee7f84b003fffffu128, 0xb06800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_109, bid128_quiet_greater_equal, 0xb040000000fd54fe2ee7f84b00400000u128, 0xb06800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_110, bid128_quiet_greater_equal, 0xb040000000fd54fe2ee7f84b00400001u128, 0xb06800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_111, bid128_quiet_greater_equal, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_112, bid128_quiet_greater_equal, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_113, bid128_quiet_greater_equal, 0xb047ec4450b72ff30000000000000000u128, 0xb07fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_114, bid128_quiet_greater_equal, 0xb047ec4450b72ff30000000000000001u128, 0xb07fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_115, bid128_quiet_greater_equal, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff096007ffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_116, bid128_quiet_greater_equal, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff0960080000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_117, bid128_quiet_greater_equal, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff0960080001u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_118, bid128_quiet_greater_equal, 0xb06600000000000000000000003a6a15u128, 0xb06800000000000000000000003a6a16u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_119, bid128_quiet_greater_equal, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b003fffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_120, bid128_quiet_greater_equal, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b00400000u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_121, bid128_quiet_greater_equal, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b00400001u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_122, bid128_quiet_greater_equal, 0xb07fec3c64797fe80000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_123, bid128_quiet_greater_equal, 0xb07fec4450b72ff30000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_124, bid128_quiet_greater_equal, 0xb07fec4450b72ff30000000000000000u128, 0xb047ec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_125, bid128_quiet_greater_equal, 0xb07fec4450b72ff30000000000000000u128, 0xb047ec4450b72ff30000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_126, bid128_quiet_greater_equal, 0xb0820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_127, bid128_quiet_greater_equal, 0xb0820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_128, bid128_quiet_greater_equal, 0xd57bd57af7c657ac7d22a00647cd1ecdu128, 0xe9effd7e9ffbfb7df8dd3ec8fbcf76e5u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_129, bid128_quiet_greater_equal, 0xd88fb2ed67c5a206e2c34c3746329b09u128, 0xc3c60000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_130, bid128_quiet_greater_equal, 0xdb86fde797c7ee96ba7db11e9df4dbc5u128, 0x964378bd032801d20103d6939fe2d7e5u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_131, bid128_quiet_greater_equal, 0xf21883668b612842fffed7fbdfffffffu128, 0x6c66fdf441dfe7fb61b81d40f8ded76cu128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_132, bid128_quiet_greater_equal, 0xf53fe7eff6f7b9f5d002289202b65c59u128, 0xffffffffffffffff75a516a7ed6a9e5fu128, false, 0x01);
dec_test!(bid128_quiet_greater_equal_133, bid128_quiet_greater_equal, 0xf8000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_134, bid128_quiet_greater_equal, 0xf8000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_135, bid128_quiet_greater_equal, 0xf8000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_136, bid128_quiet_greater_equal, 0xf8000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_equal_137, bid128_quiet_greater_equal, 0xf942aea3731d9b1b07efbf1925913095u128, 0x28482a244a079402a115bafaefacca5cu128, false, 0x00);
dec_test!(bid128_quiet_greater_equal_138, bid128_quiet_greater_equal, 0xfe00142f348bd52cf08f9fe47091c056u128, 0x44660000000000000000000000000000u128, false, 0x01);
dec_test!(bid128_quiet_greater_equal_139, bid128_quiet_greater_equal, "-Infinity"                           , "-Infinity"                           , true , 0x00);
dec_test!(bid128_quiet_greater_equal_140, bid128_quiet_greater_equal,  "Infinity"                           ,      "QNaN"                           , false, 0x00);
dec_test!(bid128_quiet_greater_equal_141, bid128_quiet_greater_equal,      "QNaN"                           ,      "QNaN"                           , false, 0x00);
dec_test!(bid128_quiet_greater_equal_142, bid128_quiet_greater_equal,      "SNaN"                           ,      "QNaN"                           , false, 0x01);
dec_test!(bid128_quiet_greater_equal_143, bid128_quiet_greater_equal,      "SNaN"                           ,      "SNaN"                           , false, 0x01);
