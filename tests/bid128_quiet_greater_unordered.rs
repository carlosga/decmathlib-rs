/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_quiet_greater_unordered_001, bid128_quiet_greater_unordered, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_002, bid128_quiet_greater_unordered, 0x00000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_003, bid128_quiet_greater_unordered, 0x00000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_004, bid128_quiet_greater_unordered, 0x00000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_005, bid128_quiet_greater_unordered, 0x00000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_006, bid128_quiet_greater_unordered, 0x00000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_007, bid128_quiet_greater_unordered, 0x000002000000014c802a40148841204du128, 0x00000000000000005f7ff6fffffbffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_008, bid128_quiet_greater_unordered, 0x0000400208002801ffffdfdfff47df7fu128, 0x001050a24930c4fc9f5fe85a4e8480aau128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_009, bid128_quiet_greater_unordered, 0x0000800000021000fffffffeffffffffu128, 0xfff97fffff3f7bedeffd68b27fffddfcu128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_010, bid128_quiet_greater_unordered, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_011, bid128_quiet_greater_unordered, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_012, bid128_quiet_greater_unordered, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_013, bid128_quiet_greater_unordered, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_014, bid128_quiet_greater_unordered, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_015, bid128_quiet_greater_unordered, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_016, bid128_quiet_greater_unordered, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_017, bid128_quiet_greater_unordered, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_018, bid128_quiet_greater_unordered, 0x00030200200200843df7ff6ef8ffffafu128, 0x00400000000021001020074212044001u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_019, bid128_quiet_greater_unordered, 0x0140413328540800ae3058b322086b46u128, 0xbd6ba29617933f4fbda9791f324bbecfu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_020, bid128_quiet_greater_unordered, "0E+368"                              , "0E+368"                              , false, 0x00);
dec_test!(bid128_quiet_greater_unordered_021, bid128_quiet_greater_unordered, "-0"                                  , "SNaN"                                , true , 0x01);
dec_test!(bid128_quiet_greater_unordered_022, bid128_quiet_greater_unordered, "1.0"                                 , 1                                     , false, 0x00);
dec_test!(bid128_quiet_greater_unordered_023, bid128_quiet_greater_unordered, 1                                     , "1.0"                                 , false, 0x00);
dec_test!(bid128_quiet_greater_unordered_024, bid128_quiet_greater_unordered, 0x156553d01d6545cc0502088164022260u128, 0x15389e38d74c664334b6020146811667u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_025, bid128_quiet_greater_unordered, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_026, bid128_quiet_greater_unordered, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000003u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_027, bid128_quiet_greater_unordered, 0x303e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_028, bid128_quiet_greater_unordered, 0x303e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_029, bid128_quiet_greater_unordered, 0x303e0000000000020000000000000000u128, 0x303e0000000000010000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_030, bid128_quiet_greater_unordered, 0x303e0000000000020000000000000000u128, 0x303e0000000000030000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_031, bid128_quiet_greater_unordered, 0x30400000001faa9fc5dcff096007ffffu128, 0x306600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_032, bid128_quiet_greater_unordered, 0x30400000001faa9fc5dcff0960080000u128, 0x306600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_033, bid128_quiet_greater_unordered, 0x30400000001faa9fc5dcff0960080001u128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_034, bid128_quiet_greater_unordered, 0x3040000000fd54fe2ee7f84b003fffffu128, 0x306800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_035, bid128_quiet_greater_unordered, 0x3040000000fd54fe2ee7f84b00400000u128, 0x306800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_036, bid128_quiet_greater_unordered, 0x3040000000fd54fe2ee7f84b00400001u128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_037, bid128_quiet_greater_unordered, 0x3040a6274bbdd0fadd61999e07ac0251u128, 0x3082a6274bbdd0fadd61999e07ac0250u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_038, bid128_quiet_greater_unordered, 0x3040c612062576589dd46a73a100695eu128, 0x3074c612062576589dd46a73a100695du128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_039, bid128_quiet_greater_unordered, 0x3040c612062576589dda322d47eb47ffu128, 0x3074c612062576589dda322d47eb47feu128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_040, bid128_quiet_greater_unordered, 0x3041622d6fbc91e01277c0caded5b8a3u128, 0x3067622d6fbc91e01277c0caded5b8a2u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_041, bid128_quiet_greater_unordered, 0x3041622d6fbc91e0127820b72d18cacfu128, 0x3067622d6fbc91e0127820b72d18caceu128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_042, bid128_quiet_greater_unordered, 0x30417361cb863de627fa3c5af3bb4c09u128, 0x30737361cb863de627fa3c5af3bb4c08u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_043, bid128_quiet_greater_unordered, 0x3047ec3daf9417fe642eaec0d36a73d5u128, 0x307fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_044, bid128_quiet_greater_unordered, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_045, bid128_quiet_greater_unordered, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_046, bid128_quiet_greater_unordered, 0x3047ec4450b72ff30000000000000000u128, 0x307fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_047, bid128_quiet_greater_unordered, 0x3047ec4450b72ff30000000000000001u128, 0x307fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_048, bid128_quiet_greater_unordered, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff096007ffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_049, bid128_quiet_greater_unordered, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_050, bid128_quiet_greater_unordered, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080001u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_051, bid128_quiet_greater_unordered, 0x306600000000000000000000003a6a15u128, 0x306800000000000000000000003a6a16u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_052, bid128_quiet_greater_unordered, 0x3067622d6fbc91e01277c0caded5b8a2u128, 0x3041622d6fbc91e01277c0caded5b8a3u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_053, bid128_quiet_greater_unordered, 0x3067622d6fbc91e0127820b72d18caceu128, 0x3041622d6fbc91e0127820b72d18cacfu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_054, bid128_quiet_greater_unordered, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b003fffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_055, bid128_quiet_greater_unordered, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_056, bid128_quiet_greater_unordered, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400001u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_057, bid128_quiet_greater_unordered, 0x30737361cb863de627fa3c5af3bb4c08u128, 0x30417361cb863de627fa3c5af3bb4c09u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_058, bid128_quiet_greater_unordered, 0x3074c612062576589dd46a73a100695du128, 0x3040c612062576589dd46a73a100695eu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_059, bid128_quiet_greater_unordered, 0x3074c612062576589dda322d47eb47feu128, 0x3040c612062576589dda322d47eb47ffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_060, bid128_quiet_greater_unordered, 0x307fec3c64797fe80000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_061, bid128_quiet_greater_unordered, 0x307fec4450b72ff30000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_062, bid128_quiet_greater_unordered, 0x307fec4450b72ff30000000000000000u128, 0x3047ec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_063, bid128_quiet_greater_unordered, 0x307fec4450b72ff30000000000000000u128, 0x3047ec4450b72ff30000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_064, bid128_quiet_greater_unordered, 0x30820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_065, bid128_quiet_greater_unordered, 0x30820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_066, bid128_quiet_greater_unordered, 0x3082a6274bbdd0fadd61999e07ac0250u128, 0x3040a6274bbdd0fadd61999e07ac0251u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_067, bid128_quiet_greater_unordered, 0x78000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_068, bid128_quiet_greater_unordered, 0x78000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_069, bid128_quiet_greater_unordered, 0x78000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_070, bid128_quiet_greater_unordered, 0x78000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_071, bid128_quiet_greater_unordered, 0x79fa89141f1ee82bfffbbfffffffffffu128, 0xb407a50b94a8b8bcc23f8ea54ed80059u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_072, bid128_quiet_greater_unordered, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_073, bid128_quiet_greater_unordered, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_074, bid128_quiet_greater_unordered, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_075, bid128_quiet_greater_unordered, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_076, bid128_quiet_greater_unordered, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_077, bid128_quiet_greater_unordered, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_078, bid128_quiet_greater_unordered, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_079, bid128_quiet_greater_unordered, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_080, bid128_quiet_greater_unordered, 0x7e000000000000000000000000000000u128, 0x40caa49458329028ea7098011fd879a8u128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_081, bid128_quiet_greater_unordered, 0x7e00071214c1fd1e71b168890a16d39cu128, 0xcf2d16a36b3c1bf215456649b9d8febeu128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_082, bid128_quiet_greater_unordered, 0x80000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_083, bid128_quiet_greater_unordered, 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_084, bid128_quiet_greater_unordered, 0x80000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_085, bid128_quiet_greater_unordered, 0x80000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_086, bid128_quiet_greater_unordered, 0x80000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_087, bid128_quiet_greater_unordered, 0x80000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_088, bid128_quiet_greater_unordered, 0x84010020fb2509080e297d7d07364da3u128, 0xdc04d6b2b42757787e29887eff505870u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_089, bid128_quiet_greater_unordered, 0x9076a3856ca8c1c84049080380801100u128, 0xfadf3cede96feda9ffffffffffffffffu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_090, bid128_quiet_greater_unordered, 0x940340890201bd50f9e19d039a42853du128, 0x8f5953c82be405b8560478137f005d17u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_091, bid128_quiet_greater_unordered, 0x95eb90a2b780c87697dcaba60aff1781u128, 0x7e002a4407f2758acd0b71e2a57e99b4u128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_092, bid128_quiet_greater_unordered, 0x994c8d74e4a4cbd50dc04f5bd28d7d5eu128, 0x5a235feb0337a830d144cf6a6d70bb60u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_093, bid128_quiet_greater_unordered, 0x9d640000000000000000000000000000u128, 0x7e0014ddd982a84c8418812ce79edb0bu128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_094, bid128_quiet_greater_unordered, 0xab822028425094019544dd28f1df700cu128, 0xab6f2417518e5fbd607ee92050f35374u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_095, bid128_quiet_greater_unordered, 0xabfe0000000000000000000000000000u128, 0x7e0019668c79cc5e737566ebe9d5c8dbu128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_096, bid128_quiet_greater_unordered, 0xadffbffff6ffefe218002005d540e17cu128, 0xb367d5e6b90fcb9eeeffb7dbffffff7fu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_097, bid128_quiet_greater_unordered, 0xb03e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_098, bid128_quiet_greater_unordered, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_099, bid128_quiet_greater_unordered, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000003u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_100, bid128_quiet_greater_unordered, 0xb03e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_101, bid128_quiet_greater_unordered, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000010000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_102, bid128_quiet_greater_unordered, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000030000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_103, bid128_quiet_greater_unordered, 0xb0400000001faa9fc5dcff096007ffffu128, 0xb06600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_104, bid128_quiet_greater_unordered, 0xb0400000001faa9fc5dcff0960080000u128, 0xb06600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_105, bid128_quiet_greater_unordered, 0xb0400000001faa9fc5dcff0960080001u128, 0xb06600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_106, bid128_quiet_greater_unordered, 0xb040000000fd54fe2ee7f84b003fffffu128, 0xb06800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_107, bid128_quiet_greater_unordered, 0xb040000000fd54fe2ee7f84b00400000u128, 0xb06800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_108, bid128_quiet_greater_unordered, 0xb040000000fd54fe2ee7f84b00400001u128, 0xb06800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_109, bid128_quiet_greater_unordered, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_110, bid128_quiet_greater_unordered, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_111, bid128_quiet_greater_unordered, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff096007ffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_112, bid128_quiet_greater_unordered, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff0960080000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_113, bid128_quiet_greater_unordered, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff0960080001u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_114, bid128_quiet_greater_unordered, 0xb06600000000000000000000003a6a15u128, 0xb06800000000000000000000003a6a16u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_115, bid128_quiet_greater_unordered, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b003fffffu128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_116, bid128_quiet_greater_unordered, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b00400000u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_117, bid128_quiet_greater_unordered, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b00400001u128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_118, bid128_quiet_greater_unordered, 0xb07fec3c64797fe80000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_119, bid128_quiet_greater_unordered, 0xb07fec4450b72ff30000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_120, bid128_quiet_greater_unordered, 0xb0820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_121, bid128_quiet_greater_unordered, 0xb0820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_122, bid128_quiet_greater_unordered, 0xbbef9f5f7efe6ff5d7128aae0b9acfb4u128, 0x63eeef41975fffda480d05cab1540eaau128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_123, bid128_quiet_greater_unordered, 0xcded591d41046f7dcb79dbdfefaffdd5u128, 0xa67ec49c9f7de6938dcd030d878245f1u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_124, bid128_quiet_greater_unordered, 0xd36b64dc4bfad8d8a1876c6674892402u128, 0x7e000ccf1142c2c23ad13c3824e5a2dfu128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_125, bid128_quiet_greater_unordered, 0xe3fafe5fe28e445c4410020004080022u128, 0x778fdaee0befb1564d4875834d1a4134u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_126, bid128_quiet_greater_unordered, 0xf5ff6bf3f5e6bfff6efe70fd7cefe997u128, 0xbdbc95aa883ea7bbca232119e2e268fdu128, true , 0x00);
dec_test!(bid128_quiet_greater_unordered_127, bid128_quiet_greater_unordered, 0xf8000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_128, bid128_quiet_greater_unordered, 0xf8000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_129, bid128_quiet_greater_unordered, 0xf8000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_130, bid128_quiet_greater_unordered, 0xf8000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_131, bid128_quiet_greater_unordered, 0xfaffdeea96fd57a7d242081902028108u128, 0xafd4b0f9f706e324c5a664740f162c77u128, false, 0x00);
dec_test!(bid128_quiet_greater_unordered_132, bid128_quiet_greater_unordered, 0xfc0030e0fc7447bf13d81f9879658c50u128, 0xfe000000000000000000000000000000u128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_133, bid128_quiet_greater_unordered, 0xfe000c773adf703e1717f0e85dacc77au128, 0x50c00000000000000000000000000000u128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_134, bid128_quiet_greater_unordered, 0xfe0024cdddba647a38a5033463233d34u128, 0x982e06e3202201e6ec63345675eaae51u128, true , 0x01);
dec_test!(bid128_quiet_greater_unordered_135, bid128_quiet_greater_unordered, "Infinity"                            , "SNaN"                                , true , 0x01);
dec_test!(bid128_quiet_greater_unordered_136, bid128_quiet_greater_unordered, "QNaN"                                , 0                                     , true , 0x00);
dec_test!(bid128_quiet_greater_unordered_137, bid128_quiet_greater_unordered, "QNaN"                                , "Infinity"                            , true , 0x00);
dec_test!(bid128_quiet_greater_unordered_138, bid128_quiet_greater_unordered, "QNaN"                                , "SNaN"                                , true , 0x01);
dec_test!(bid128_quiet_greater_unordered_139, bid128_quiet_greater_unordered, "SNaN"                                , "-Infinity"                           , true , 0x01);
dec_test!(bid128_quiet_greater_unordered_140, bid128_quiet_greater_unordered, "SNaN"                                , "SNaN"                                , true , 0x01);
