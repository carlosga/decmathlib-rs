/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_nan_001, bid128_nan, "0", 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nan_002, bid128_nan, "" , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_nan_003, bid128_nan, "1", 0x7c000000000000000000000000000001u128, 0x00);

