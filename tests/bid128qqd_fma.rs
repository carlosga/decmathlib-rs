/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128qqd_fma_001, bid128qqd_fma, 0, 0x33200000000000000000000000000005u128, 0xaffa000000000000000000000000000au128, 0x5fc000000000000au64, 0x32e0314dc6448d9338c15b0a00000000u128, 0x20);
dec_test!(bid128qqd_fma_002, bid128qqd_fma, 0, 0x3e3457aafb3df933c3fee7c69c7e2cedu128, 0x9ef1afc349a9e0f7ea6eddcb5fba5f77u128, 0x0400000000000400u64, 0xad2706da8013883c784a53ea962fb9a1u128, 0x20);
dec_test!(bid128qqd_fma_003, bid128qqd_fma, 0, 0x7ebfbeccd7abf5fd59cfb7fcddf796b9u128, 0xf7ffdfcfbfffdfa76090116410408204u128, 0x5e00c510800080a5u64, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128qqd_fma_004, bid128qqd_fma, 0, 0x8fb728087e5a3d6fb6ec3208b2eeef49u128, 0x5083808465200044fbfffefd7fff6eebu128, 0x2d3d531f65fdc592u64, 0xb03ce6dfd4e881d4e9a5446bfbe2bdcau128, 0x20);
dec_test!(bid128qqd_fma_005, bid128qqd_fma, 0, 0xd9fe7fffffbff7fa0000000000000000u128, 0x87a296fea05f07f10000000000000000u128, 0xc22b5a8f762025a1u64, 0x31a38801250e5648ef50d2e20648113cu128, 0x20);
dec_test!(bid128qqd_fma_006, bid128qqd_fma, 1, 0x33200000000000000000000000000005u128, 0xaffa000000000000000000000000000au128, 0x5fc000000000000au64, 0x32dfed09bead87c0378d8e63ffffffffu128, 0x20);
dec_test!(bid128qqd_fma_007, bid128qqd_fma, 2, 0x33200000000000000000000000000005u128, 0xaffa000000000000000000000000000au128, 0x5fc000000000000au64, 0x32e0314dc6448d9338c15b0a00000000u128, 0x20);
dec_test!(bid128qqd_fma_008, bid128qqd_fma, 3, 0x33200000000000000000000000000005u128, 0xaffa000000000000000000000000000au128, 0x5fc000000000000au64, 0x32dfed09bead87c0378d8e63ffffffffu128, 0x20);
dec_test!(bid128qqd_fma_009, bid128qqd_fma, 4, 0x33200000000000000000000000000005u128, 0xaffa000000000000000000000000000au128, 0x5fc000000000000au64, 0x32e0314dc6448d9338c15b0a00000000u128, 0x20);
