/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid_to_dpd128_001, bid_to_dpd128, 0, 0x00018a6e32246c99c60ad85000000001u128, 0x60000000000000000000000000000001u128, 0x00);
dec_test!(bid_to_dpd128_002, bid_to_dpd128, 0, 0x0001bbbbf868fa2cfecc335a0000000bu128, 0x64000000000000000000000000000011u128, 0x00);
dec_test!(bid_to_dpd128_003, bid_to_dpd128, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x64000ff3fcff3fcff3fcffa7f850cb83u128, 0x00);
dec_test!(bid_to_dpd128_004, bid_to_dpd128, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid_to_dpd128_005, bid_to_dpd128, 0, 0x2000c5371912364ce3056c2800000001u128, 0x30000000000000000000000000000001u128, 0x00);
dec_test!(bid_to_dpd128_006, bid_to_dpd128, 0, 0x20018a6e32246c99c60ad850000000c9u128, 0x68000000000000000000000000000101u128, 0x00);
dec_test!(bid_to_dpd128_007, bid_to_dpd128, 0, 0x40000000000000000000000000000001u128, 0x40000000000000000000000000000001u128, 0x00);
dec_test!(bid_to_dpd128_008, bid_to_dpd128, 0, 0x4000c5371912364ce3056c2800000001u128, 0x50000000000000000000000000000001u128, 0x00);
dec_test!(bid_to_dpd128_009, bid_to_dpd128, 0, 0x40018a6e32246c99c60ad85000000fa1u128, 0x70000000000000000000000000001001u128, 0x00);
dec_test!(bid_to_dpd128_010, bid_to_dpd128, 0, 0x5ffc0000000000000000000000000001u128, 0x43ff8000000000000000000000000001u128, 0x00);
dec_test!(bid_to_dpd128_011, bid_to_dpd128, 0, 0x5fffed09bead87c0378d8e63fffffffeu128, 0x77ffcff3fcff3fcff3fcff3fcff3fcfeu128, 0x00);
dec_test!(bid_to_dpd128_012, bid_to_dpd128, 0, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x77ffcff3fcff3fcff3fcff3fcff3fcffu128, 0x00);
dec_test!(bid_to_dpd128_013, bid_to_dpd128, 0, 0x7c000000000000000000000000000001u128, 0x7c000000000000000000000000000001u128, 0x00);
dec_test!(bid_to_dpd128_014, bid_to_dpd128, 0, 0x7c00314dc6448d9338c15b0b00000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid_to_dpd128_015, bid_to_dpd128, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid_to_dpd128_016, bid_to_dpd128, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid_to_dpd128_017, bid_to_dpd128, 0, 0xd1ebb9a6abc8f2350000000000001010u128, 0xf23d7ddf1f7514dce5d92444fb9ce00cu128, 0x00);
dec_test!(bid_to_dpd128_018, bid_to_dpd128, 0, 0xd75ffffbffffffff000400c000000100u128, 0xc2ebc000000000000000000000000000u128, 0x00);
dec_test!(bid_to_dpd128_019, bid_to_dpd128, 0, 0xe9d2f58ff58a18ae17828fe467e1f528u128, 0xa0e94000000000000000000000000000u128, 0x00);
dec_test!(bid_to_dpd128_020, bid_to_dpd128, 0, 0xfa8c419c31b4b492e48b05e2288b0459u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid_to_dpd128_021, bid_to_dpd128, 0, 0xfefffdfddffdff6feefffd68fdffb87fu128, 0xfe000000000000000000000000000000u128, 0x00);
dec_test!(bid_to_dpd128_022, bid_to_dpd128, 0, 0x7c000000000000000000000000000001u128, 0x7c000000000000000000000000000001u128, 0x00);