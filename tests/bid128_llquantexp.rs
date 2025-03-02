/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_llquantexp_001, bid128_llquantexp, 0x3046000000000000000000000000007Bu128, 3    , 0x00);
dec_test!(bid128_llquantexp_002, bid128_llquantexp, 0xB046000000000000000000000000007Bu128, 3    , 0x00);
dec_test!(bid128_llquantexp_003, bid128_llquantexp, 0x30480000000000000000000000000000u128, 4    , 0x00);
dec_test!(bid128_llquantexp_004, bid128_llquantexp, 0xB0480000000000000000000000000000u128, 4    , 0x00);
dec_test!(bid128_llquantexp_005, bid128_llquantexp, 0x5FFE314DC6448D9338C15B0A00000000u128, 6111 , 0x00);
dec_test!(bid128_llquantexp_006, bid128_llquantexp, 0xDFFE314DC6448D9338C15B0A00000000u128, 6111 , 0x00);
dec_test!(bid128_llquantexp_007, bid128_llquantexp, 0x00000000000000000000000000000001u128, -6176, 0x00);
dec_test!(bid128_llquantexp_008, bid128_llquantexp, 0x80000000000000000000000000000001u128, -6176, 0x00);
