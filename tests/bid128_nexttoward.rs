/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_nexttoward_001, bid128_nexttoward, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x303e000000000000000000000000000au128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nexttoward_002, bid128_nexttoward, 0x2FFE314DC6448D9338C15B0A00000000u128, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x2FFE314DC6448D9338C15B0A00000001u128, 0x00);
dec_test!(bid128_nexttoward_003, bid128_nexttoward, 0x6003b75d7734cd9e1234567890123456u128, 0x303e000000000000000000000000000au128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nexttoward_004, bid128_nexttoward, 0x69dbb75d7734cd9e1234567890123456u128, 0x303e000000000000000000000000000au128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nexttoward_005, bid128_nexttoward, 0x78000000000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nexttoward_006, bid128_nexttoward, 0x78000001000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nexttoward_007, bid128_nexttoward, 0x7c000000000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nexttoward_008, bid128_nexttoward, 0x7c000001000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0x7c000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nexttoward_009, bid128_nexttoward, 0x7cff3fffffffefffffffffffffffffffu128, 0x303e000000000000000000000000000au128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nexttoward_010, bid128_nexttoward, 0x7e000000000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nexttoward_011, bid128_nexttoward, 0x7e003fffffffefffffffffffffffffffu128, 0x303e000000000000000000000000000au128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nexttoward_012, bid128_nexttoward, 0x7ef00000000000000000000000000001u128, 0x303e000000000000000000000000000au128, 0x7c000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nexttoward_013, bid128_nexttoward, 0xe003b75d7734cd9e1234567890123456u128, 0x303e000000000000000000000000000au128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nexttoward_014, bid128_nexttoward, 0xe9dbb75d7734cd9e1234567890123456u128, 0x303e000000000000000000000000000au128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_nexttoward_015, bid128_nexttoward, 0xf8000000000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0xdfffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nexttoward_016, bid128_nexttoward, 0xf8000001000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0xdfffed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nexttoward_017, bid128_nexttoward, 0xfc000000000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nexttoward_018, bid128_nexttoward, 0xfc000001000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0xfc000001000000000000000000000000u128, 0x00);
dec_test!(bid128_nexttoward_019, bid128_nexttoward, 0xfe000000000000000000000000000000u128, 0x303e000000000000000000000000000au128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_nexttoward_020, bid128_nexttoward, 0xfef00000000000000000000000000001u128, 0x303e000000000000000000000000000au128, 0xfc000000000000000000000000000001u128, 0x01);
dec_test!(bid128_nexttoward_021, bid128_nexttoward, 0x2FFE314DC6448D9338C15B0A00000000u128, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x2FFE314DC6448D9338C15B0A00000001u128, 0x00);
dec_test!(bid128_nexttoward_022, bid128_nexttoward, 0x2FFE314DC6448D9338C15B0A00000000u128, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x2FFE314DC6448D9338C15B0A00000001u128, 0x00);
dec_test!(bid128_nexttoward_023, bid128_nexttoward, 0x2FFE314DC6448D9338C15B0A00000000u128, 0x5FFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x2FFE314DC6448D9338C15B0A00000001u128, 0x00);
dec_test!(bid128_nexttoward_024, bid128_nexttoward, 0x2FFE314DC6448D9338C15B0A00000000u128, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x2ffded09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nexttoward_025, bid128_nexttoward, 0x2FFE314DC6448D9338C15B0A00000000u128, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x2ffded09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nexttoward_026, bid128_nexttoward, 0x2FFE314DC6448D9338C15B0A00000000u128, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x2ffded09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nexttoward_027, bid128_nexttoward, 0x2FFE314DC6448D9338C15B0A00000000u128, 0xDFFFED09BEAD87C0378D8E63FFFFFFFFu128, 0x2ffded09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_nexttoward_028, bid128_nexttoward, 0x80000000000000000000000000000000u128, 0x00000000000000000000000000000000u128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nexttoward_029, bid128_nexttoward, 0x00000000000000000000000000000000u128, 0x80000000000000000000000000000000u128, 0x80000000000000000000000000000000u128, 0x00);