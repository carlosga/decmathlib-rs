/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_quiet_not_equal_001, bid128_quiet_not_equal, "-0"                                  , "0"                                   , false, 0x00);
dec_test!(bid128_quiet_not_equal_002, bid128_quiet_not_equal, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_003, bid128_quiet_not_equal, 0x00000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_004, bid128_quiet_not_equal, 0x00000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_005, bid128_quiet_not_equal, 0x00000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_006, bid128_quiet_not_equal, 0x00000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_007, bid128_quiet_not_equal, 0x00000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_008, bid128_quiet_not_equal, 0x0000000000000000c46001c1602024bcu128, 0xce775a40b746d5f28000001090010000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_009, bid128_quiet_not_equal, 0x00000000020000009002000220200080u128, 0x0000000040000000ffbfff7fbff7ffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_010, bid128_quiet_not_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_not_equal_011, bid128_quiet_not_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_012, bid128_quiet_not_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_013, bid128_quiet_not_equal, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_014, bid128_quiet_not_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_015, bid128_quiet_not_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_not_equal_016, bid128_quiet_not_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_017, bid128_quiet_not_equal, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_018, bid128_quiet_not_equal, 0x004000004002e0000040608004000810u128, 0x000000000000048affff7fdffffffffdu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_019, bid128_quiet_not_equal, "-0"                                  , "-Infinity"                           , true , 0x00);
dec_test!(bid128_quiet_not_equal_020, bid128_quiet_not_equal,  "0"                                  ,      "SNaN"                           , true , 0x01);
dec_test!(bid128_quiet_not_equal_021, bid128_quiet_not_equal, 0x1402c6a3193d4fe6d8188d154c210ad6u128, 0x7e000eef19887283c4ecaff995b2a0c3u128, true , 0x01);
dec_test!(bid128_quiet_not_equal_022, bid128_quiet_not_equal, 0x2056609e22108222f57fb7ed79efefe7u128, 0xebfe7e7aebdc6fdb7ffffcfedffd1ffeu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_023, bid128_quiet_not_equal, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_024, bid128_quiet_not_equal, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000003u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_025, bid128_quiet_not_equal, 0x303e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_026, bid128_quiet_not_equal, 0x303e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_027, bid128_quiet_not_equal, 0x303e0000000000020000000000000000u128, 0x303e0000000000010000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_028, bid128_quiet_not_equal, 0x303e0000000000020000000000000000u128, 0x303e0000000000030000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_029, bid128_quiet_not_equal, 0x30400000001faa9fc5dcff096007ffffu128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_030, bid128_quiet_not_equal, 0x30400000001faa9fc5dcff0960080000u128, 0x306600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_not_equal_031, bid128_quiet_not_equal, 0x30400000001faa9fc5dcff0960080001u128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_032, bid128_quiet_not_equal, 0x3040000000fd54fe2ee7f84b003fffffu128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_033, bid128_quiet_not_equal, 0x3040000000fd54fe2ee7f84b00400000u128, 0x306800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_not_equal_034, bid128_quiet_not_equal, 0x3040000000fd54fe2ee7f84b00400001u128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_035, bid128_quiet_not_equal, 0x3040a6274bbdd0fadd61999e07ac0251u128, 0x3082a6274bbdd0fadd61999e07ac0250u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_036, bid128_quiet_not_equal, 0x3040c612062576589dd46a73a100695eu128, 0x3074c612062576589dd46a73a100695du128, true , 0x00);
dec_test!(bid128_quiet_not_equal_037, bid128_quiet_not_equal, 0x3040c612062576589dda322d47eb47ffu128, 0x3074c612062576589dda322d47eb47feu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_038, bid128_quiet_not_equal, 0x3041622d6fbc91e01277c0caded5b8a3u128, 0x3067622d6fbc91e01277c0caded5b8a2u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_039, bid128_quiet_not_equal, 0x3041622d6fbc91e0127820b72d18cacfu128, 0x3067622d6fbc91e0127820b72d18caceu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_040, bid128_quiet_not_equal, 0x30417361cb863de627fa3c5af3bb4c09u128, 0x30737361cb863de627fa3c5af3bb4c08u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_041, bid128_quiet_not_equal, 0x3047ec3daf9417fe642eaec0d36a73d5u128, 0x307fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_042, bid128_quiet_not_equal, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_043, bid128_quiet_not_equal, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_044, bid128_quiet_not_equal, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff096007ffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_045, bid128_quiet_not_equal, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080000u128, false, 0x00);
dec_test!(bid128_quiet_not_equal_046, bid128_quiet_not_equal, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_047, bid128_quiet_not_equal, 0x3067622d6fbc91e01277c0caded5b8a2u128, 0x3041622d6fbc91e01277c0caded5b8a3u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_048, bid128_quiet_not_equal, 0x3067622d6fbc91e0127820b72d18caceu128, 0x3041622d6fbc91e0127820b72d18cacfu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_049, bid128_quiet_not_equal, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b003fffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_050, bid128_quiet_not_equal, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400000u128, false, 0x00);
dec_test!(bid128_quiet_not_equal_051, bid128_quiet_not_equal, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_052, bid128_quiet_not_equal, 0x30737361cb863de627fa3c5af3bb4c08u128, 0x30417361cb863de627fa3c5af3bb4c09u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_053, bid128_quiet_not_equal, 0x3074c612062576589dd46a73a100695du128, 0x3040c612062576589dd46a73a100695eu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_054, bid128_quiet_not_equal, 0x3074c612062576589dda322d47eb47feu128, 0x3040c612062576589dda322d47eb47ffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_055, bid128_quiet_not_equal, 0x307fec3c64797fe80000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_056, bid128_quiet_not_equal, 0x307fec4450b72ff30000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_057, bid128_quiet_not_equal, 0x30820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_058, bid128_quiet_not_equal, 0x30820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_059, bid128_quiet_not_equal, 0x3082a6274bbdd0fadd61999e07ac0250u128, 0x3040a6274bbdd0fadd61999e07ac0251u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_060, bid128_quiet_not_equal, 0x34b05a0857e63aeee5e31e5c368634b1u128, 0xfe0019a01ff9d2db07cf5a2328410c03u128, true , 0x01);
dec_test!(bid128_quiet_not_equal_061, bid128_quiet_not_equal, 0x527602c93aa7115c438e6c8ed77c63b6u128, 0x36bee6672ce9c9aa2fbe0a6e382420bfu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_062, bid128_quiet_not_equal, 0x59232823e16662be9266f90b62e30785u128, 0x984e0000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_063, bid128_quiet_not_equal, 0x78000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_064, bid128_quiet_not_equal, 0x78000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_not_equal_065, bid128_quiet_not_equal, 0x78000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_066, bid128_quiet_not_equal, 0x78000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_067, bid128_quiet_not_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_068, bid128_quiet_not_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_069, bid128_quiet_not_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_070, bid128_quiet_not_equal, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_071, bid128_quiet_not_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_072, bid128_quiet_not_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_073, bid128_quiet_not_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_074, bid128_quiet_not_equal, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_075, bid128_quiet_not_equal, 0x7e000000000000000000000000000000u128, 0x0b560000000000000000000000000000u128, true , 0x01);
dec_test!(bid128_quiet_not_equal_076, bid128_quiet_not_equal, 0x7e000000000000000000000000000000u128, 0xa2a947a41960371cd9e19928c5e84264u128, true , 0x01);
dec_test!(bid128_quiet_not_equal_077, bid128_quiet_not_equal, 0x80000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_078, bid128_quiet_not_equal, 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_079, bid128_quiet_not_equal, 0x80000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_080, bid128_quiet_not_equal, 0x80000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_081, bid128_quiet_not_equal, 0x80000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_082, bid128_quiet_not_equal, 0x80000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_083, bid128_quiet_not_equal, 0x84ecd158238681a8516b42210e25039du128, 0xfe000000000000000000000000000000u128, true , 0x01);
dec_test!(bid128_quiet_not_equal_084, bid128_quiet_not_equal, 0x8f125f17cdaeaa38d84b5a2cb597de42u128, 0x0f320000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_085, bid128_quiet_not_equal, 0x9123549d405a4648a7f7f6eff7ddc772u128, 0x7bc15e9b17226af9675dfdedfeaeefcau128, true , 0x00);
dec_test!(bid128_quiet_not_equal_086, bid128_quiet_not_equal, 0x929133fad2422275ea724c3ceadd69b6u128, 0x81458a91916a68e2b1c7db53dcad1e9fu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_087, bid128_quiet_not_equal, 0x9abcf7d6719f86c07a5427954bd7a638u128, 0xfe0006eaac310b592b4914638f9c7b67u128, true , 0x01);
dec_test!(bid128_quiet_not_equal_088, bid128_quiet_not_equal, 0xad1ac63fc590d5ef0d2647cf162f4255u128, 0xa4a7909220dacbce942f13314b218165u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_089, bid128_quiet_not_equal, 0xb03e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_090, bid128_quiet_not_equal, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_091, bid128_quiet_not_equal, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000003u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_092, bid128_quiet_not_equal, 0xb03e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_093, bid128_quiet_not_equal, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000010000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_094, bid128_quiet_not_equal, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000030000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_095, bid128_quiet_not_equal, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_096, bid128_quiet_not_equal, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_097, bid128_quiet_not_equal, 0xb07fec3c64797fe80000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_098, bid128_quiet_not_equal, 0xb07fec4450b72ff30000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_099, bid128_quiet_not_equal, 0xb0820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_100, bid128_quiet_not_equal, 0xb0820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_101, bid128_quiet_not_equal, 0xb9f5f57ff7eff6f7a3a62fbdd76dfd49u128, 0x6d17439a8e5b145ff6f84f58ec25cb64u128, false, 0x00);
dec_test!(bid128_quiet_not_equal_102, bid128_quiet_not_equal, 0xbb9828df5e0a51cbc7dedaff2bede9dfu128, 0x8b4fb4314dd4a5391015ed9daf27fc2fu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_103, bid128_quiet_not_equal, 0xc4780000000000000000000000000000u128, 0x4a5bcab0f8edec186fc2509fced1b4e3u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_104, bid128_quiet_not_equal, 0xce7ed527b9a990e456073fc49c5947b2u128, 0x83f064bc3c29ffd58661eae7c5a54c3au128, true , 0x00);
dec_test!(bid128_quiet_not_equal_105, bid128_quiet_not_equal, 0xceaab518af7af6cc65a81cdb9cda88e8u128, 0x80b71dffd19ea26fab1848ece4858f78u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_106, bid128_quiet_not_equal, 0xcecc4a2c797aeeb0fc44030cbb7a9a63u128, 0x48f8218baa01a30f84cf75ddd2fb0e1du128, true , 0x00);
dec_test!(bid128_quiet_not_equal_107, bid128_quiet_not_equal, 0xd26e0000000000000000000000000000u128, 0x1ca412f710ed6e306d8690724c10255du128, true , 0x00);
dec_test!(bid128_quiet_not_equal_108, bid128_quiet_not_equal, 0xd2b40000000000000000000000000000u128, 0x7e001bdb5646c9583a7b9a68ba57b2fdu128, true , 0x01);
dec_test!(bid128_quiet_not_equal_109, bid128_quiet_not_equal, 0xdcba0000000000000000000000000000u128, 0x7e0004fc36a8161319696eaaa9ce1f76u128, true , 0x01);
dec_test!(bid128_quiet_not_equal_110, bid128_quiet_not_equal, 0xf8000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_111, bid128_quiet_not_equal, 0xf8000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_112, bid128_quiet_not_equal, 0xf8000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_equal_113, bid128_quiet_not_equal, 0xf8000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_not_equal_114, bid128_quiet_not_equal, 0xf9dbb109ba197ef20000000000021400u128, 0xfbfdbfb9ab3ffcdb96040801488033f2u128, false, 0x00);
dec_test!(bid128_quiet_not_equal_115, bid128_quiet_not_equal, 0xfb3b4271f03f6747d028f34b102fbcb5u128, 0x6fefd8cef7f77ff9feef7dffff7fffffu128, true , 0x00);
dec_test!(bid128_quiet_not_equal_116, bid128_quiet_not_equal, 0xfe0005f89d8a148bcec4f97767abe0afu128, 0xc10c8aed9af5ac13c4fd8ac5d61ceb65u128, true , 0x01);
dec_test!(bid128_quiet_not_equal_117, bid128_quiet_not_equal, 0xfe002bc74aa2a9abfc30e0ba5cf9de3du128, 0x0442c4856f10bc1d28855220bff25b9au128, true , 0x01);
dec_test!(bid128_quiet_not_equal_118, bid128_quiet_not_equal, 0xffffffffffffffff0040040000800870u128, 0x62a8800004000044062b102206021a09u128, true , 0x01);
dec_test!(bid128_quiet_not_equal_119, bid128_quiet_not_equal, "-Infinity"                           ,        "-0"                           , true , 0x00);
dec_test!(bid128_quiet_not_equal_120, bid128_quiet_not_equal,  "Infinity"                           ,        "-0"                           , true , 0x00);
dec_test!(bid128_quiet_not_equal_121, bid128_quiet_not_equal,  "Infinity"                           ,         "0"                           , true , 0x00);
dec_test!(bid128_quiet_not_equal_122, bid128_quiet_not_equal, "-Infinity"                           , "-Infinity"                           , false, 0x00);
dec_test!(bid128_quiet_not_equal_123, bid128_quiet_not_equal, "-Infinity"                           ,      "SNaN"                           , true , 0x01);
dec_test!(bid128_quiet_not_equal_124, bid128_quiet_not_equal,  "Infinity"                           ,      "SNaN"                           , true , 0x01);
dec_test!(bid128_quiet_not_equal_125, bid128_quiet_not_equal,      "QNaN"                           ,        "-0"                           , true , 0x00);
dec_test!(bid128_quiet_not_equal_126, bid128_quiet_not_equal,      "QNaN"                           ,         "0"                           , true , 0x00);
dec_test!(bid128_quiet_not_equal_127, bid128_quiet_not_equal,      "QNaN"                           ,  "Infinity"                           , true , 0x00);
dec_test!(bid128_quiet_not_equal_128, bid128_quiet_not_equal,      "QNaN"                           ,      "QNaN"                           , true , 0x00);
dec_test!(bid128_quiet_not_equal_129, bid128_quiet_not_equal,      "SNaN"                           ,        "-0"                           , true , 0x01);
dec_test!(bid128_quiet_not_equal_130, bid128_quiet_not_equal,      "SNaN"                           ,         "0"                           , true , 0x01);
dec_test!(bid128_quiet_not_equal_131, bid128_quiet_not_equal,      "SNaN"                           ,  "Infinity"                           , true , 0x01);
dec_test!(bid128_quiet_not_equal_132, bid128_quiet_not_equal,      "SNaN"                           ,      "QNaN"                           , true , 0x01);