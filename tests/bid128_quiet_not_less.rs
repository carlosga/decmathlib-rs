/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_quiet_not_less_001, bid128_quiet_not_less, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_not_less_002, bid128_quiet_not_less, 0x00000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_less_003, bid128_quiet_not_less, 0x00000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_004, bid128_quiet_not_less, 0x00000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_not_less_005, bid128_quiet_not_less, 0x00000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_006, bid128_quiet_not_less, 0x00000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_less_007, bid128_quiet_not_less, 0x000000000000000066c2758e22c22602u128, 0x00000000000000000000400000040000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_008, bid128_quiet_not_less, 0x0000000000000000ffffffefbfffffffu128, 0x87020b1b16689b01e97be068a409fa51u128, true , 0x00);
dec_test!(bid128_quiet_not_less_009, bid128_quiet_not_less, 0x00000000104000004800022a00000002u128, 0x257f0b1d8c85560c9d4dbb63c2d7d060u128, false, 0x00);
dec_test!(bid128_quiet_not_less_010, bid128_quiet_not_less, 0x0000000084408420dfdfffbffbdfffffu128, 0x00186902a808e49202100000e8c10000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_011, bid128_quiet_not_less, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_012, bid128_quiet_not_less, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_013, bid128_quiet_not_less, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_014, bid128_quiet_not_less, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_015, bid128_quiet_not_less, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false, 0x00);
dec_test!(bid128_quiet_not_less_016, bid128_quiet_not_less, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_017, bid128_quiet_not_less, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_018, bid128_quiet_not_less, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_019, bid128_quiet_not_less, 0x00868002000440000000000000000000u128, 0x00c2049000000020c200ba0012402968u128, false, 0x00);
dec_test!(bid128_quiet_not_less_020, bid128_quiet_not_less, 0x0128e573906e13ee3a5fa8389980eb3eu128, 0x0110028060000412effddafdb7f5afbfu128, true , 0x00);
dec_test!(bid128_quiet_not_less_021, bid128_quiet_not_less, 0x018233483740cbbade254d52fd848fd4u128, 0x3c282c472d43840234769acb4d16cb02u128, false, 0x00);
dec_test!(bid128_quiet_not_less_022, bid128_quiet_not_less, 0                                     , "SNaN"                                , true , 0x01);
dec_test!(bid128_quiet_not_less_023, bid128_quiet_not_less, 0x1001f1500020948273d6fd39effbbdcfu128, 0xffb9c7eb4d4e823a0000000004800000u128, true , 0x01);
dec_test!(bid128_quiet_not_less_024, bid128_quiet_not_less, "1.0"                                 , 1                                     , true , 0x00);
dec_test!(bid128_quiet_not_less_025, bid128_quiet_not_less, 0x10cd9661aff01a4cedb813a25701d5dau128, 0x7b8a4cb7def99f99888c010a11021740u128, false, 0x00);
dec_test!(bid128_quiet_not_less_026, bid128_quiet_not_less, 1                                     , "1.0"                                 , true , 0x00);
dec_test!(bid128_quiet_not_less_027, bid128_quiet_not_less, 0x2421c83dbe7fb22dd10321d67230eb7cu128, 0x5d7994ab146429aa30de00eef1a808d8u128, false, 0x00);
dec_test!(bid128_quiet_not_less_028, bid128_quiet_not_less, 0x2838d47280dd0911474498db19237031u128, 0x0000300008000000fe7b3926791b353cu128, true , 0x00);
dec_test!(bid128_quiet_not_less_029, bid128_quiet_not_less, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_less_030, bid128_quiet_not_less, 0x303e0000000000000000000000000002u128, 0x303e0000000000000000000000000003u128, false, 0x00);
dec_test!(bid128_quiet_not_less_031, bid128_quiet_not_less, 0x303e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_not_less_032, bid128_quiet_not_less, 0x303e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_less_033, bid128_quiet_not_less, 0x303e0000000000020000000000000000u128, 0x303e0000000000010000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_034, bid128_quiet_not_less, 0x303e0000000000020000000000000000u128, 0x303e0000000000030000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_035, bid128_quiet_not_less, 0x30400000001faa9fc5dcff096007ffffu128, 0x306600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_not_less_036, bid128_quiet_not_less, 0x30400000001faa9fc5dcff0960080000u128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_not_less_037, bid128_quiet_not_less, 0x30400000001faa9fc5dcff0960080001u128, 0x306600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_not_less_038, bid128_quiet_not_less, 0x3040000000fd54fe2ee7f84b003fffffu128, 0x306800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_not_less_039, bid128_quiet_not_less, 0x3040000000fd54fe2ee7f84b00400000u128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_not_less_040, bid128_quiet_not_less, 0x3040000000fd54fe2ee7f84b00400001u128, 0x306800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_not_less_041, bid128_quiet_not_less, 0x3040a6274bbdd0fadd61999e07ac0251u128, 0x3082a6274bbdd0fadd61999e07ac0250u128, false, 0x00);
dec_test!(bid128_quiet_not_less_042, bid128_quiet_not_less, 0x3040c612062576589dd46a73a100695eu128, 0x3074c612062576589dd46a73a100695du128, false, 0x00);
dec_test!(bid128_quiet_not_less_043, bid128_quiet_not_less, 0x3040c612062576589dda322d47eb47ffu128, 0x3074c612062576589dda322d47eb47feu128, false, 0x00);
dec_test!(bid128_quiet_not_less_044, bid128_quiet_not_less, 0x3041622d6fbc91e01277c0caded5b8a3u128, 0x3067622d6fbc91e01277c0caded5b8a2u128, false, 0x00);
dec_test!(bid128_quiet_not_less_045, bid128_quiet_not_less, 0x3041622d6fbc91e0127820b72d18cacfu128, 0x3067622d6fbc91e0127820b72d18caceu128, false, 0x00);
dec_test!(bid128_quiet_not_less_046, bid128_quiet_not_less, 0x30417361cb863de627fa3c5af3bb4c09u128, 0x30737361cb863de627fa3c5af3bb4c08u128, false, 0x00);
dec_test!(bid128_quiet_not_less_047, bid128_quiet_not_less, 0x3047ec3daf9417fe642eaec0d36a73d5u128, 0x307fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_048, bid128_quiet_not_less, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec3c64797fe80000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_049, bid128_quiet_not_less, 0x3047ec3daf9417fe642eaec0d36a73d7u128, 0x307fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_050, bid128_quiet_not_less, 0x3047ec4450b72ff30000000000000000u128, 0x307fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_051, bid128_quiet_not_less, 0x3047ec4450b72ff30000000000000001u128, 0x307fec4450b72ff30000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_052, bid128_quiet_not_less, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff096007ffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_053, bid128_quiet_not_less, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_054, bid128_quiet_not_less, 0x306600000000000000000000003a6a15u128, 0x30400000001faa9fc5dcff0960080001u128, false, 0x00);
dec_test!(bid128_quiet_not_less_055, bid128_quiet_not_less, 0x306600000000000000000000003a6a15u128, 0x306800000000000000000000003a6a16u128, false, 0x00);
dec_test!(bid128_quiet_not_less_056, bid128_quiet_not_less, 0x3067622d6fbc91e01277c0caded5b8a2u128, 0x3041622d6fbc91e01277c0caded5b8a3u128, true , 0x00);
dec_test!(bid128_quiet_not_less_057, bid128_quiet_not_less, 0x3067622d6fbc91e0127820b72d18caceu128, 0x3041622d6fbc91e0127820b72d18cacfu128, true , 0x00);
dec_test!(bid128_quiet_not_less_058, bid128_quiet_not_less, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b003fffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_059, bid128_quiet_not_less, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_060, bid128_quiet_not_less, 0x306800000000000000000000002ebb44u128, 0x3040000000fd54fe2ee7f84b00400001u128, false, 0x00);
dec_test!(bid128_quiet_not_less_061, bid128_quiet_not_less, 0x30737361cb863de627fa3c5af3bb4c08u128, 0x30417361cb863de627fa3c5af3bb4c09u128, true , 0x00);
dec_test!(bid128_quiet_not_less_062, bid128_quiet_not_less, 0x3074c612062576589dd46a73a100695du128, 0x3040c612062576589dd46a73a100695eu128, true , 0x00);
dec_test!(bid128_quiet_not_less_063, bid128_quiet_not_less, 0x3074c612062576589dda322d47eb47feu128, 0x3040c612062576589dda322d47eb47ffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_064, bid128_quiet_not_less, 0x307fec3c64797fe80000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_not_less_065, bid128_quiet_not_less, 0x307fec4450b72ff30000000000000000u128, 0x3047ec3daf9417fe642eaec0d36a73d7u128, true , 0x00);
dec_test!(bid128_quiet_not_less_066, bid128_quiet_not_less, 0x307fec4450b72ff30000000000000000u128, 0x3047ec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_067, bid128_quiet_not_less, 0x307fec4450b72ff30000000000000000u128, 0x3047ec4450b72ff30000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_less_068, bid128_quiet_not_less, 0x30820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_less_069, bid128_quiet_not_less, 0x30820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_less_070, bid128_quiet_not_less, 0x3082a6274bbdd0fadd61999e07ac0250u128, 0x3040a6274bbdd0fadd61999e07ac0251u128, true , 0x00);
dec_test!(bid128_quiet_not_less_071, bid128_quiet_not_less, 0x356c945f41531fb1ed8303675c002e6fu128, 0x7e0013507e2cc63d7f040040a47ac851u128, true , 0x01);
dec_test!(bid128_quiet_not_less_072, bid128_quiet_not_less, 0x55ae9cb1f3a1e13f4079358c07c9787au128, 0x115d2b11b57e886e29b92870feda1884u128, true , 0x00);
dec_test!(bid128_quiet_not_less_073, bid128_quiet_not_less, 0x5f32efe0a6ae1c648383e95b5a73642du128, 0xf8000000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_074, bid128_quiet_not_less, 0x69c7a99b78ef9445813042c9ecfcaf9eu128, 0xc5d3a5b087fe52f19cd7f4558800b786u128, true , 0x00);
dec_test!(bid128_quiet_not_less_075, bid128_quiet_not_less, 0x6ae504f51ba2a4b5f8c079644d6f58f7u128, 0x66eb80e197d6928562e9868005d00874u128, true , 0x00);
dec_test!(bid128_quiet_not_less_076, bid128_quiet_not_less, 0x78000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_less_077, bid128_quiet_not_less, 0x78000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_less_078, bid128_quiet_not_less, 0x78000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_less_079, bid128_quiet_not_less, 0x78000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_less_080, bid128_quiet_not_less, 0x78d1d012a99203f60100004800581091u128, 0x2432ca4068421214f7effbffffeffef3u128, true , 0x00);
dec_test!(bid128_quiet_not_less_081, bid128_quiet_not_less, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_082, bid128_quiet_not_less, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_083, bid128_quiet_not_less, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_084, bid128_quiet_not_less, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_085, bid128_quiet_not_less, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_086, bid128_quiet_not_less, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_087, bid128_quiet_not_less, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_088, bid128_quiet_not_less, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true , 0x00);
dec_test!(bid128_quiet_not_less_089, bid128_quiet_not_less, 0x80000000000000000000000000000000u128, 0x00000000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_not_less_090, bid128_quiet_not_less, 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_less_091, bid128_quiet_not_less, 0x80000000000000000000000000000001u128, 0x00000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_092, bid128_quiet_not_less, 0x80000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_not_less_093, bid128_quiet_not_less, 0x80000000000000000000000000000001u128, 0x80000000000000000000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_094, bid128_quiet_not_less, 0x80000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_less_095, bid128_quiet_not_less, 0x832e9f34d2eb9177c1c38581695f8531u128, 0x333aad9a11ca35b7e4c5272df78be98au128, false, 0x00);
dec_test!(bid128_quiet_not_less_096, bid128_quiet_not_less, 0x87d28e665b030e7a3859164bea673916u128, 0xcbc0562a01ca02b84036c403c9beddb6u128, true , 0x00);
dec_test!(bid128_quiet_not_less_097, bid128_quiet_not_less, 0x8f960000000000000000000000000000u128, 0x00e80000000000000000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_098, bid128_quiet_not_less, 0xa2268299e896ef177c54d3d7191e3e42u128, 0xa1e716be49371fd45011600014049e20u128, false, 0x00);
dec_test!(bid128_quiet_not_less_099, bid128_quiet_not_less, 0xa7e62c15ea1fe53d48c754d460998143u128, 0x685f3143c924d2aaa26da1d6bb424067u128, false, 0x00);
dec_test!(bid128_quiet_not_less_100, bid128_quiet_not_less, 0xb03e0000000000000000000000000002u128, 0x30820000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_not_less_101, bid128_quiet_not_less, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000001u128, false, 0x00);
dec_test!(bid128_quiet_not_less_102, bid128_quiet_not_less, 0xb03e0000000000000000000000000002u128, 0xb03e0000000000000000000000000003u128, true , 0x00);
dec_test!(bid128_quiet_not_less_103, bid128_quiet_not_less, 0xb03e0000000000000000000000000002u128, 0xb0820000000000000000000000000001u128, true , 0x00);
dec_test!(bid128_quiet_not_less_104, bid128_quiet_not_less, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000010000000000000000u128, false, 0x00);
dec_test!(bid128_quiet_not_less_105, bid128_quiet_not_less, 0xb03e0000000000020000000000000000u128, 0xb03e0000000000030000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_106, bid128_quiet_not_less, 0xb0400000001faa9fc5dcff096007ffffu128, 0xb06600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_not_less_107, bid128_quiet_not_less, 0xb0400000001faa9fc5dcff0960080000u128, 0xb06600000000000000000000003a6a15u128, true , 0x00);
dec_test!(bid128_quiet_not_less_108, bid128_quiet_not_less, 0xb0400000001faa9fc5dcff0960080001u128, 0xb06600000000000000000000003a6a15u128, false, 0x00);
dec_test!(bid128_quiet_not_less_109, bid128_quiet_not_less, 0xb040000000fd54fe2ee7f84b003fffffu128, 0xb06800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_not_less_110, bid128_quiet_not_less, 0xb040000000fd54fe2ee7f84b00400000u128, 0xb06800000000000000000000002ebb44u128, true , 0x00);
dec_test!(bid128_quiet_not_less_111, bid128_quiet_not_less, 0xb040000000fd54fe2ee7f84b00400001u128, 0xb06800000000000000000000002ebb44u128, false, 0x00);
dec_test!(bid128_quiet_not_less_112, bid128_quiet_not_less, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec3c64797fe80000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_113, bid128_quiet_not_less, 0xb047ec3daf9417fe642eaec0d36a73d7u128, 0xb07fec4450b72ff30000000000000000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_114, bid128_quiet_not_less, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff096007ffffu128, false, 0x00);
dec_test!(bid128_quiet_not_less_115, bid128_quiet_not_less, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff0960080000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_116, bid128_quiet_not_less, 0xb06600000000000000000000003a6a15u128, 0xb0400000001faa9fc5dcff0960080001u128, true , 0x00);
dec_test!(bid128_quiet_not_less_117, bid128_quiet_not_less, 0xb06600000000000000000000003a6a15u128, 0xb06800000000000000000000003a6a16u128, true , 0x00);
dec_test!(bid128_quiet_not_less_118, bid128_quiet_not_less, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b003fffffu128, false, 0x00);
dec_test!(bid128_quiet_not_less_119, bid128_quiet_not_less, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b00400000u128, true , 0x00);
dec_test!(bid128_quiet_not_less_120, bid128_quiet_not_less, 0xb06800000000000000000000002ebb44u128, 0xb040000000fd54fe2ee7f84b00400001u128, true , 0x00);
dec_test!(bid128_quiet_not_less_121, bid128_quiet_not_less, 0xb07fec3c64797fe80000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_not_less_122, bid128_quiet_not_less, 0xb07fec4450b72ff30000000000000000u128, 0xb047ec3daf9417fe642eaec0d36a73d7u128, false, 0x00);
dec_test!(bid128_quiet_not_less_123, bid128_quiet_not_less, 0xb0820000000000000000000000000001u128, 0x303e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_not_less_124, bid128_quiet_not_less, 0xb0820000000000000000000000000001u128, 0xb03e0000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_not_less_125, bid128_quiet_not_less, 0xb0ae39e4d139516bd8f895ea21181b4du128, 0xc781e13fca61fc898b83aa907bef34a6u128, true , 0x00);
dec_test!(bid128_quiet_not_less_126, bid128_quiet_not_less, 0xb5877c4e6dc8bf87225dd57923e12119u128, 0x845c53f38c0fa9dd0ac2971128a02fffu128, false, 0x00);
dec_test!(bid128_quiet_not_less_127, bid128_quiet_not_less, 0xdeef5feffbf7cdffffbeffefffffff7eu128, 0xb1d8229607a6153ac877e9d480a48b5du128, false, 0x00);
dec_test!(bid128_quiet_not_less_128, bid128_quiet_not_less, 0xf8000000000000000000000000000001u128, 0x00000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_not_less_129, bid128_quiet_not_less, 0xf8000000000000000000000000000001u128, 0x78000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_not_less_130, bid128_quiet_not_less, 0xf8000000000000000000000000000001u128, 0x80000000000000000000000000000002u128, false, 0x00);
dec_test!(bid128_quiet_not_less_131, bid128_quiet_not_less, 0xf8000000000000000000000000000001u128, 0xf8000000000000000000000000000002u128, true , 0x00);
dec_test!(bid128_quiet_not_less_132, bid128_quiet_not_less, 0xfbf40e7fecfbfbd7a6f0b499a7b41250u128, 0x642a517b20149ee9020004870179800eu128, false, 0x00);
dec_test!(bid128_quiet_not_less_133, bid128_quiet_not_less, 0xfe0000a25a358a5315cd3fe6584ed659u128, 0x080af97b41db836379246038125dc16eu128, true , 0x01);
dec_test!(bid128_quiet_not_less_134, bid128_quiet_not_less, "-Infinity"                           , "-Infinity"                           , true , 0x00);
dec_test!(bid128_quiet_not_less_135, bid128_quiet_not_less, "QNaN"                                , "SNaN"                                , true , 0x01);
dec_test!(bid128_quiet_not_less_136, bid128_quiet_not_less, "SNaN"                                , "-0"                                  , true , 0x01);
dec_test!(bid128_quiet_not_less_137, bid128_quiet_not_less, "SNaN"                                , 0                                     , true , 0x01);
dec_test!(bid128_quiet_not_less_138, bid128_quiet_not_less, "SNaN"                                , "Infinity"                            , true , 0x01);
dec_test!(bid128_quiet_not_less_139, bid128_quiet_not_less, "SNaN"                                , "QNaN"                                , true , 0x01);