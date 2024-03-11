/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid_dpd_to_bid128_001, bid_dpd_to_bid128, 0, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_002, bid_dpd_to_bid128, 0, 0x00000000000000000000000000000001u128, 0x00000000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_003, bid_dpd_to_bid128, 0, 0x00000000000000000000000000000010u128, 0x0000000000000000000000000000000au128, 0x00);
dec_test!(bid_dpd_to_bid128_004, bid_dpd_to_bid128, 0, 0x000000000000000000000000000049c6u128, 0x0000000000000000000000000000303au128, 0x00);
dec_test!(bid_dpd_to_bid128_005, bid_dpd_to_bid128, 0, 0x00000000000000000049c5de08d4d2e8u128, 0x0000000000000000002bdc545d6b4b88u128, 0x00);
dec_test!(bid_dpd_to_bid128_006, bid_dpd_to_bid128, 0, 0x00000a395bcf049c5de08d4d2e7078a3u128, 0x000006163e665beb7ca6a2e1a64244cbu128, 0x00);
dec_test!(bid_dpd_to_bid128_007, bid_dpd_to_bid128, 0, 0x00001e8ff7d63e0618f9a649ec13de30u128, 0x00001231276e5fb21234567890123456u128, 0x00);
dec_test!(bid_dpd_to_bid128_008, bid_dpd_to_bid128, 0, 0x0000c000000000000000000008008003u128, 0x00060000000000000000000005f62f23u128, 0x00);
dec_test!(bid_dpd_to_bid128_009, bid_dpd_to_bid128, 0, 0x0001c000000000000000000000000000u128, 0x000e0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_010, bid_dpd_to_bid128, 0, 0x0007c000000000000000000000000000u128, 0x003e0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_011, bid_dpd_to_bid128, 0, 0x0400134b9c1e28e56f3c127177823534u128, 0x00003cde6fff9732de825cd07e96aff2u128, 0x00);
dec_test!(bid_dpd_to_bid128_012, bid_dpd_to_bid128, 0, 0x0800186243fc2493c62c9171ab3bce45u128, 0x000071b33671d6160123456789012345u128, 0x00);
dec_test!(bid_dpd_to_bid128_013, bid_dpd_to_bid128, 0, 0x0851aa829c4095c20000000100008800u128, 0x028c7ca57309efe67e0288d4addb7df0u128, 0x00);
dec_test!(bid_dpd_to_bid128_014, bid_dpd_to_bid128, 0, 0x20edc000000000000000000000000000u128, 0x276e0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_015, bid_dpd_to_bid128, 0, 0x21a0c00000000000000534b9c1e28e56u128, 0x2d06000000000000000462d53c8abac0u128, 0x00);
dec_test!(bid_dpd_to_bid128_016, bid_dpd_to_bid128, 0, 0x21a1000000000000000534b9c1e28e56u128, 0x2d08000000000000000462d53c8abac0u128, 0x00);
dec_test!(bid_dpd_to_bid128_017, bid_dpd_to_bid128, 0, 0x21a48000000000000000000000000000u128, 0x2d240000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_018, bid_dpd_to_bid128, 0, 0x21a4c000000000000000000000000000u128, 0x2d260000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_019, bid_dpd_to_bid128, 0, 0x21a4c00000000000000534b9c1e28e56u128, 0x2d26000000000000000462d53c8abac0u128, 0x00);
dec_test!(bid_dpd_to_bid128_020, bid_dpd_to_bid128, 0, 0x21a7c000000000000000000000000000u128, 0x2d3e0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_021, bid_dpd_to_bid128, 0, 0x22044000000000000004000000000000u128, 0x302200000000000000038d7ea4c68000u128, 0x00);
dec_test!(bid_dpd_to_bid128_022, bid_dpd_to_bid128, 0, 0x22048000000000000000800000000000u128, 0x302400000000000000005af3107a4000u128, 0x00);
dec_test!(bid_dpd_to_bid128_023, bid_dpd_to_bid128, 0, 0x2204c000000000000000100000000000u128, 0x3026000000000000000009184e72a000u128, 0x00);
dec_test!(bid_dpd_to_bid128_024, bid_dpd_to_bid128, 0, 0x22050000000000000000010000000000u128, 0x3028000000000000000000e8d4a51000u128, 0x00);
dec_test!(bid_dpd_to_bid128_025, bid_dpd_to_bid128, 0, 0x22054000000000000000002000000000u128, 0x302a000000000000000000174876e800u128, 0x00);
dec_test!(bid_dpd_to_bid128_026, bid_dpd_to_bid128, 0, 0x22058000000000000000000400000000u128, 0x302c00000000000000000002540be400u128, 0x00);
dec_test!(bid_dpd_to_bid128_027, bid_dpd_to_bid128, 0, 0x2205c000000000000000000040000000u128, 0x302e000000000000000000003b9aca00u128, 0x00);
dec_test!(bid_dpd_to_bid128_028, bid_dpd_to_bid128, 0, 0x22060000000000000000000008000000u128, 0x30300000000000000000000005f5e100u128, 0x00);
dec_test!(bid_dpd_to_bid128_029, bid_dpd_to_bid128, 0, 0x22064000000000000000000001000000u128, 0x30320000000000000000000000989680u128, 0x00);
dec_test!(bid_dpd_to_bid128_030, bid_dpd_to_bid128, 0, 0x22068000000000000000000000100000u128, 0x303400000000000000000000000f4240u128, 0x00);
dec_test!(bid_dpd_to_bid128_031, bid_dpd_to_bid128, 0, 0x2206c000000000000000000000020000u128, 0x303600000000000000000000000186a0u128, 0x00);
dec_test!(bid_dpd_to_bid128_032, bid_dpd_to_bid128, 0, 0x22070000000000000000000000004000u128, 0x30380000000000000000000000002710u128, 0x00);
dec_test!(bid_dpd_to_bid128_033, bid_dpd_to_bid128, 0, 0x22074000000000000000000000000400u128, 0x303a00000000000000000000000003e8u128, 0x00);
dec_test!(bid_dpd_to_bid128_034, bid_dpd_to_bid128, 0, 0x22078000000000000000000000000080u128, 0x303c0000000000000000000000000064u128, 0x00);
dec_test!(bid_dpd_to_bid128_035, bid_dpd_to_bid128, 0, 0x2207c000000000000000000000000010u128, 0x303e000000000000000000000000000au128, 0x00);
dec_test!(bid_dpd_to_bid128_036, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000001u128, 0x30400000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_037, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000002u128, 0x30400000000000000000000000000002u128, 0x00);
dec_test!(bid_dpd_to_bid128_038, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000004u128, 0x30400000000000000000000000000004u128, 0x00);
dec_test!(bid_dpd_to_bid128_039, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000008u128, 0x30400000000000000000000000000008u128, 0x00);
dec_test!(bid_dpd_to_bid128_040, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000016u128, 0x30400000000000000000000000000010u128, 0x00);
dec_test!(bid_dpd_to_bid128_041, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000032u128, 0x30400000000000000000000000000020u128, 0x00);
dec_test!(bid_dpd_to_bid128_042, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000064u128, 0x30400000000000000000000000000040u128, 0x00);
dec_test!(bid_dpd_to_bid128_043, bid_dpd_to_bid128, 0, 0x220800000000000000000000000000a8u128, 0x30400000000000000000000000000080u128, 0x00);
dec_test!(bid_dpd_to_bid128_044, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000156u128, 0x30400000000000000000000000000100u128, 0x00);
dec_test!(bid_dpd_to_bid128_045, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000292u128, 0x30400000000000000000000000000200u128, 0x00);
dec_test!(bid_dpd_to_bid128_046, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000424u128, 0x30400000000000000000000000000400u128, 0x00);
dec_test!(bid_dpd_to_bid128_047, bid_dpd_to_bid128, 0, 0x22080000000000000000000000000848u128, 0x30400000000000000000000000000800u128, 0x00);
dec_test!(bid_dpd_to_bid128_048, bid_dpd_to_bid128, 0, 0x2208000000000000000000000000107au128, 0x30400000000000000000000000001000u128, 0x00);
dec_test!(bid_dpd_to_bid128_049, bid_dpd_to_bid128, 0, 0x220800000000000000000000000020bau128, 0x30400000000000000000000000002000u128, 0x00);
dec_test!(bid_dpd_to_bid128_050, bid_dpd_to_bid128, 0, 0x220800000000000000000000000059cau128, 0x30400000000000000000000000004000u128, 0x00);
dec_test!(bid_dpd_to_bid128_051, bid_dpd_to_bid128, 0, 0x2208000000000000000000000000cbe8u128, 0x30400000000000000000000000008000u128, 0x00);
dec_test!(bid_dpd_to_bid128_052, bid_dpd_to_bid128, 0, 0x220800000000000000000000000196b6u128, 0x30400000000000000000000000010000u128, 0x00);
dec_test!(bid_dpd_to_bid128_053, bid_dpd_to_bid128, 0, 0x2208000000000000000000000002c472u128, 0x30400000000000000000000000020000u128, 0x00);
dec_test!(bid_dpd_to_bid128_054, bid_dpd_to_bid128, 0, 0x220800000000000000000000000588c4u128, 0x30400000000000000000000000040000u128, 0x00);
dec_test!(bid_dpd_to_bid128_055, bid_dpd_to_bid128, 0, 0x220800000000000000000000000a914eu128, 0x30400000000000000000000000080000u128, 0x00);
dec_test!(bid_dpd_to_bid128_056, bid_dpd_to_bid128, 0, 0x220800000000000000000000001122f6u128, 0x30400000000000000000000000100000u128, 0x00);
dec_test!(bid_dpd_to_bid128_057, bid_dpd_to_bid128, 0, 0x2208000000000000000000000021ecd2u128, 0x30400000000000000000000000200000u128, 0x00);
dec_test!(bid_dpd_to_bid128_058, bid_dpd_to_bid128, 0, 0x22080000000000000000000000436984u128, 0x30400000000000000000000000400000u128, 0x00);
dec_test!(bid_dpd_to_bid128_059, bid_dpd_to_bid128, 0, 0x22080000000000000000000000873b08u128, 0x30400000000000000000000000800000u128, 0x00);
dec_test!(bid_dpd_to_bid128_060, bid_dpd_to_bid128, 0, 0x220800000000000000000000016fdd16u128, 0x30400000000000000000000001000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_061, bid_dpd_to_bid128, 0, 0x220800000000000000000000033b5232u128, 0x30400000000000000000000002000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_062, bid_dpd_to_bid128, 0, 0x2208000000000000000000000672226cu128, 0x30400000000000000000000004000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_063, bid_dpd_to_bid128, 0, 0x2208000000000000000000000b445fa8u128, 0x30400000000000000000000008000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_064, bid_dpd_to_bid128, 0, 0x2208000000000000000000001688d656u128, 0x30400000000000000000000010000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_065, bid_dpd_to_bid128, 0, 0x2208000000000000000000002b61f19cu128, 0x30400000000000000000000020000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_066, bid_dpd_to_bid128, 0, 0x220800000000000000000000473f062cu128, 0x30400000000000000000000040000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_067, bid_dpd_to_bid128, 0, 0x2208000000000000000000008c78af48u128, 0x30400000000000000000000080000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_068, bid_dpd_to_bid128, 0, 0x22080000000000000000000115afb57au128, 0x30400000000000000000000100000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_069, bid_dpd_to_bid128, 0, 0x2208000000000000000000022cfaf2bau128, 0x30400000000000000000000200000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_070, bid_dpd_to_bid128, 0, 0x220800000000000000000005cf9cbccau128, 0x30400000000000000000000400000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_071, bid_dpd_to_bid128, 0, 0x22080000000000000000000d1d9ee1e8u128, 0x30400000000000000000000800000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_072, bid_dpd_to_bid128, 0, 0x22080000000000000000001a3999dbb6u128, 0x30400000000000000000001000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_073, bid_dpd_to_bid128, 0, 0x22080000000000000000002de3877672u128, 0x30400000000000000000002000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_074, bid_dpd_to_bid128, 0, 0x22080000000000000000005d37de32ccu128, 0x30400000000000000000004000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_075, bid_dpd_to_bid128, 0, 0x2208000000000000000000b27d54746eu128, 0x30400000000000000000008000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_076, bid_dpd_to_bid128, 0, 0x220800000000000000000117e91c9ff6u128, 0x30400000000000000000010000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_077, bid_dpd_to_bid128, 0, 0x220800000000000000000237c23556d2u128, 0x30400000000000000000020000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_078, bid_dpd_to_bid128, 0, 0x220800000000000000000477846a4484u128, 0x30400000000000000000040000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_079, bid_dpd_to_bid128, 0, 0x2208000000000000000008fe83b08908u128, 0x30400000000000000000080000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_080, bid_dpd_to_bid128, 0, 0x2208000000000000000017ae8ea11216u128, 0x30400000000000000000100000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_081, bid_dpd_to_bid128, 0, 0x2208000000000000000035329f21393cu128, 0x30400000000000000000200000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_082, bid_dpd_to_bid128, 0, 0x22080000000000000000707a3c43df64u128, 0x30400000000000000000400000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_083, bid_dpd_to_bid128, 0, 0x22080000000000000000c0ede4e755a8u128, 0x30400000000000000000800000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_084, bid_dpd_to_bid128, 0, 0x220800000000000000010b9d3fce4356u128, 0x30400000000000000001000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_085, bid_dpd_to_bid128, 0, 0x22080000000000000002e2abddd88592u128, 0x30400000000000000002000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_086, bid_dpd_to_bid128, 0, 0x22080000000000000004a51ff8c53324u128, 0x30400000000000000004000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_087, bid_dpd_to_bid128, 0, 0x2208000000000000000951f7d1dd2d48u128, 0x30400000000000000008000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_088, bid_dpd_to_bid128, 0, 0x2208000000000000001283b7f277c27au128, 0x30400000000000000010000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_089, bid_dpd_to_bid128, 0, 0x220800000000000000240737d54f019eu128, 0x30400000000000000020000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_090, bid_dpd_to_bid128, 0, 0x220800000000000000601477a8982e8eu128, 0x30400000000000000040000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_091, bid_dpd_to_bid128, 0, 0x220800000000000000d828fec187b7aeu128, 0x30400000000000000080000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_092, bid_dpd_to_bid128, 0, 0x220800000000000001c857b6837eb7bcu128, 0x30400000000000000100000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_093, bid_dpd_to_bid128, 0, 0x2208000000000000031095338759757cu128, 0x30400000000000000200000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_094, bid_dpd_to_bid128, 0, 0x22080000000000000539307d8d1e47c4u128, 0x30400000000000000400000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_095, bid_dpd_to_bid128, 0, 0x22080000000000000bda60f498388e4eu128, 0x30400000000000000800000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_096, bid_dpd_to_bid128, 0, 0x22080000000000001348ada1306d33fcu128, 0x30400000000000001000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_097, bid_dpd_to_bid128, 0, 0x220800000000000026154d02513ceddcu128, 0x30400000000000002000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_098, bid_dpd_to_bid128, 0, 0x22080000000000004c476a062277ae8cu128, 0x30400000000000004000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_099, bid_dpd_to_bid128, 0, 0x2208000000000000948df20da5cfd42eu128, 0x30400000000000008000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_100, bid_dpd_to_bid128, 0, 0x2208000000000001891bc41cf89b4716u128, 0x30400000000000010000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_101, bid_dpd_to_bid128, 0, 0x2208000000000003647e4e31e1920d32u128, 0x30400000000000020000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_102, bid_dpd_to_bid128, 0, 0x22080000000000073fabfc5693e41a64u128, 0x30400000000000040000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_103, bid_dpd_to_bid128, 0, 0x220800000000000c7bcddcb3f76849aeu128, 0x30400000000000080000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_104, bid_dpd_to_bid128, 0, 0x2208000000000015b31e8d3e5d28b75cu128, 0x30400000000000100000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_105, bid_dpd_to_bid128, 0, 0x2208000000000029a56c1c76385d4792u128, 0x30400000000000200000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_106, bid_dpd_to_bid128, 0, 0x2208000000000048aa6f20e5e1160e24u128, 0x30400000000000400000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_107, bid_dpd_to_bid128, 0, 0x220800000000009e12ad418d12cc1a2eu128, 0x30400000000000800000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_108, bid_dpd_to_bid128, 0, 0x220800000000013a279a2acbf4544f7au128, 0x30400000000001000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_109, bid_dpd_to_bid128, 0, 0x22080000000002644ecaedee51a89dbau128, 0x30400000000002000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_110, bid_dpd_to_bid128, 0, 0x2208000000000606f994bd9e28a973cau128, 0x30400000000004000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_111, bid_dpd_to_bid128, 0, 0x2208000000000dff82f56cf74e1e26e8u128, 0x30400000000008000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_112, bid_dpd_to_bid128, 0, 0x2208000000001d6d75b7a5a71a3864b6u128, 0x30400000000010000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_113, bid_dpd_to_bid128, 0, 0x22080000000034495e9e514bb464f972u128, 0x30400000000020000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_114, bid_dpd_to_bid128, 0, 0x220800000000609319518dd5d3bddac4u128, 0x30400000000040000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_115, bid_dpd_to_bid128, 0, 0x220800000000c12622bf0d652eb74c4eu128, 0x30400000000080000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_116, bid_dpd_to_bid128, 0, 0x220800000001422ad0ff14ca4f4e18f6u128, 0x30400000000100000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_117, bid_dpd_to_bid128, 0, 0x22080000000285c5dce529561c9849d2u128, 0x30400000000200000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_118, bid_dpd_to_bid128, 0, 0x2208000000048f7835e258a5b5e8b384u128, 0x30400000000400000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_119, bid_dpd_to_bid128, 0, 0x220800000009dc606b5b9d0cdfbd2608u128, 0x30400000000800000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_120, bid_dpd_to_bid128, 0, 0x2208000000197091d24e3c19bdb57b1cu128, 0x30400000001000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_121, bid_dpd_to_bid128, 0, 0x220800000038d2f2649f682ce9abef32u128, 0x30400000002000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_122, bid_dpd_to_bid128, 0, 0x2208000000777c552955b659c8b36d64u128, 0x30400000004000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_123, bid_dpd_to_bid128, 0, 0x2208000000d4f0a8427372ad1e266aa8u128, 0x30400000008000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_124, bid_dpd_to_bid128, 0, 0x22080000018992c090b5c51a3a4e2c56u128, 0x30400000010000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_125, bid_dpd_to_bid128, 0, 0x2208000003183f019d0b1a2de49b8892u128, 0x30400000020000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_126, bid_dpd_to_bid128, 0, 0x2208000005373303952d8a5d07f29124u128, 0x30400000040000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_127, bid_dpd_to_bid128, 0, 0x220800000a7503878bc3e0b27de52248u128, 0x30400000080000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_128, bid_dpd_to_bid128, 0, 0x2208000010ddf80d7306a117efa9eb1eu128, 0x30400000100000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_129, bid_dpd_to_bid128, 0, 0x22080000258da81944ac4237cba67fbau128, 0x30400000200000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_130, bid_dpd_to_bid128, 0, 0x22080000670d10328b984a779cbe3ecau128, 0x30400000400000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_131, bid_dpd_to_bid128, 0, 0x22080000e71402d572c8e8febf1bf4e8u128, 0x30400000800000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_132, bid_dpd_to_bid128, 0, 0x22080001e52838a94591b7aeec3371b6u128, 0x30400001000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_133, bid_dpd_to_bid128, 0, 0x22080003625669428aa3753ac6b23372u128, 0x30400002000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_134, bid_dpd_to_bid128, 0, 0x22080006599cd405715dd07d0f5035c4u128, 0x30400004000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_135, bid_dpd_to_bid128, 0, 0x2208000cce2d6009425380f21d1c0b4eu128, 0x30400008000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_136, bid_dpd_to_bid128, 0, 0x220800159f50c01284a6019eb83415f6u128, 0x30400010000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_137, bid_dpd_to_bid128, 0, 0x2208002ad581402569610c67e06843d2u128, 0x30400020000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_138, bid_dpd_to_bid128, 0, 0x22080051c3028019ce7705e391c0b684u128, 0x30400040000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_139, bid_dpd_to_bid128, 0, 0x220800b2a73885306c2f5bf3c0531064u128, 0x30400400000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_140, bid_dpd_to_bid128, 0, 0x2208010305040362d8f5117f725d0c08u128, 0x30400080000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_141, bid_dpd_to_bid128, 0, 0x22080204aa09c0f51dc223f35515a816u128, 0x30400100000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_142, bid_dpd_to_bid128, 0, 0x2208040b903f41d837034d87a82bc832u128, 0x30400200000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_143, bid_dpd_to_bid128, 0, 0x22080e2565764bd1378d9bbe010538a8u128, 0x30400800000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_144, bid_dpd_to_bid128, 0, 0x22081a4a62d3d6226e9bab35820bd956u128, 0x30401000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_145, bid_dpd_to_bid128, 0, 0x22083490dc876595d94ee66484134a92u128, 0x30402000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_146, bid_dpd_to_bid128, 0, 0x2263c000000000000000000000000000u128, 0x331e0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_147, bid_dpd_to_bid128, 0, 0x2263c000000000000000000000000001u128, 0x331e0000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_148, bid_dpd_to_bid128, 0, 0x22640000000000000000000000000000u128, 0x33200000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_149, bid_dpd_to_bid128, 0, 0x22640000000000000000000000000001u128, 0x33200000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_150, bid_dpd_to_bid128, 0, 0x2264000000000000000534b9c1e28e56u128, 0x3320000000000000000462d53c8abac0u128, 0x00);
dec_test!(bid_dpd_to_bid128_151, bid_dpd_to_bid128, 0, 0x22640000000000000040000000000000u128, 0x3320000000000000002386f26fc10000u128, 0x00);
dec_test!(bid_dpd_to_bid128_152, bid_dpd_to_bid128, 0, 0x22644000000000000000000000000000u128, 0x33220000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_153, bid_dpd_to_bid128, 0, 0x30000000000000000000000000000001u128, 0x2000c5371912364ce3056c2800000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_154, bid_dpd_to_bid128, 0, 0x40000000000000000000000000000001u128, 0x40000000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_155, bid_dpd_to_bid128, 0, 0x43ff4000000000000000000000000000u128, 0x5ffa0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_156, bid_dpd_to_bid128, 0, 0x43ff4000000000000000000000000001u128, 0x5ffa0000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_157, bid_dpd_to_bid128, 0, 0x43ff8000000000000000000000000000u128, 0x5ffc0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_158, bid_dpd_to_bid128, 0, 0x43ff8000000000000000000000000001u128, 0x5ffc0000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_159, bid_dpd_to_bid128, 0, 0x47ff934b9c1e28e56f3c127177823534u128, 0x5ffc3cde6fff9732de825cd07e96aff2u128, 0x00);
dec_test!(bid_dpd_to_bid128_160, bid_dpd_to_bid128, 0, 0x47ffd34b9c1e28e56f3c127177823534u128, 0x5ffe3cde6fff9732de825cd07e96aff2u128, 0x00);
dec_test!(bid_dpd_to_bid128_161, bid_dpd_to_bid128, 0, 0x50000000000000000000000000000001u128, 0x4000c5371912364ce3056c2800000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_162, bid_dpd_to_bid128, 0, 0x60000000000000000000000000000001u128, 0x00018a6e32246c99c60ad85000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_163, bid_dpd_to_bid128, 0, 0x64000000000000000000000000000011u128, 0x0001bbbbf868fa2cfecc335a0000000bu128, 0x00);
dec_test!(bid_dpd_to_bid128_164, bid_dpd_to_bid128, 0, 0x64000ff3fcff3fcff3fcff3fcff3fcffu128, 0x0001ed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid_dpd_to_bid128_165, bid_dpd_to_bid128, 0, 0x68000000000000000000000000000101u128, 0x20018a6e32246c99c60ad850000000c9u128, 0x00);
dec_test!(bid_dpd_to_bid128_166, bid_dpd_to_bid128, 0, 0x70000000000000000000000000001001u128, 0x40018a6e32246c99c60ad85000000fa1u128, 0x00);
dec_test!(bid_dpd_to_bid128_167, bid_dpd_to_bid128, 0, 0x77ffcff3fcff3fcff3fcff3fcff3fcfeu128, 0x5fffed09bead87c0378d8e63fffffffeu128, 0x00);
dec_test!(bid_dpd_to_bid128_168, bid_dpd_to_bid128, 0, 0x77ffcff3fcff3fcff3fcff3fcff3fcffu128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid_dpd_to_bid128_169, bid_dpd_to_bid128, 0, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_170, bid_dpd_to_bid128, 0, 0x7c000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_171, bid_dpd_to_bid128, 0, 0x7c000000000000000000000000000001u128, 0x7c000000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_172, bid_dpd_to_bid128, 0, 0x7c000001e52838a94591b7aeec3371b6u128, 0x7c000001000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_173, bid_dpd_to_bid128, 0, 0x7e000000000000000000000000000000u128, 0x7e000000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_174, bid_dpd_to_bid128, 0, 0x7e000000000000000000000000000001u128, 0x7e000000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_175, bid_dpd_to_bid128, 0, 0x7e000001e52838a94591b7aeec3371b6u128, 0x7e000001000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_176, bid_dpd_to_bid128, 0, 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_177, bid_dpd_to_bid128, 0, 0x80000000000000000000000000000001u128, 0x80000000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_178, bid_dpd_to_bid128, 0, 0x800000000000000000000000000049c6u128, 0x8000000000000000000000000000303au128, 0x00);
dec_test!(bid_dpd_to_bid128_179, bid_dpd_to_bid128, 0, 0x80000a395bcf049c5de08d4d2e7078a3u128, 0x800006163e665beb7ca6a2e1a64244cbu128, 0x00);
dec_test!(bid_dpd_to_bid128_180, bid_dpd_to_bid128, 0, 0x8001c000000000000000000000000000u128, 0x800e0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_181, bid_dpd_to_bid128, 0, 0xa0edc000000000000000000000000000u128, 0xa76e0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_182, bid_dpd_to_bid128, 0, 0xa1a48000000000000000000000000000u128, 0xad240000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_183, bid_dpd_to_bid128, 0, 0xa1a4800000000000000534b9c1e28e56u128, 0xad24000000000000000462d53c8abac0u128, 0x00);
dec_test!(bid_dpd_to_bid128_184, bid_dpd_to_bid128, 0, 0xa1a4c000000000000000000000000000u128, 0xad260000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_185, bid_dpd_to_bid128, 0, 0xa1a4c000000000000000000000000001u128, 0xad260000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_186, bid_dpd_to_bid128, 0, 0xa1a7c000000000000000000000000000u128, 0xad3e0000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_187, bid_dpd_to_bid128, 0, 0xa2640000000000000000000000000000u128, 0xb3200000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_188, bid_dpd_to_bid128, 0, 0xa2640000000000000040000000000000u128, 0xb320000000000000002386f26fc10000u128, 0x00);
dec_test!(bid_dpd_to_bid128_189, bid_dpd_to_bid128, 0, 0xa2644000000000000000000000000000u128, 0xb3220000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_190, bid_dpd_to_bid128, 0, 0xc7ff934b9c1e28e56f3c127177823534u128, 0xdffc3cde6fff9732de825cd07e96aff2u128, 0x00);
dec_test!(bid_dpd_to_bid128_191, bid_dpd_to_bid128, 0, 0xf797bbcaf7f325d9fddb7ebcff32ffdfu128, 0xdcbde9edc32bb534b77d29c425c75637u128, 0x00);
dec_test!(bid_dpd_to_bid128_192, bid_dpd_to_bid128, 0, 0xf7ffcff3fcff3fcff3fcff3fcff3fcffu128, 0xdfffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid_dpd_to_bid128_193, bid_dpd_to_bid128, 0, 0xf8000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_194, bid_dpd_to_bid128, 0, 0xfb7d291aa1368e3c4e6c2e8aabc4543bu128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_195, bid_dpd_to_bid128, 0, 0xfc000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_196, bid_dpd_to_bid128, 0, 0xfc000000000000000000000000000001u128, 0xfc000000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_197, bid_dpd_to_bid128, 0, 0xfc000001e52838a94591b7aeec3371b6u128, 0xfc000001000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_198, bid_dpd_to_bid128, 0, 0xfe000000000000000000000000000000u128, 0xfe000000000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_199, bid_dpd_to_bid128, 0, 0xfe000000000000000000000000000001u128, 0xfe000000000000000000000000000001u128, 0x00);
dec_test!(bid_dpd_to_bid128_200, bid_dpd_to_bid128, 0, 0xfe000001e52838a94591b7aeec3371b6u128, 0xfe000001000000000000000000000000u128, 0x00);
dec_test!(bid_dpd_to_bid128_201, bid_dpd_to_bid128, 0, 0xff7ffdffbff9fdf7ae633d73ef56fef7u128, 0xfe0027714a64129af3c6894ed77165f9u128, 0x00);