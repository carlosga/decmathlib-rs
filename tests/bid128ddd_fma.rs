/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128ddd_fma_001, bid128ddd_fma, 0, 0x0000000000000200u64, 0x1a13bc50c402d5c3u64, 0x4c31067288024880u64, 0x31c2ec459ba7304fe49a4fafd2000000u128, 0x20);
dec_test!(bid128ddd_fma_002, bid128ddd_fma, 0, 0x1739a39084f14140u64, 0x926a2b0a0b343625u64, 0x0000000000000001u64, 0x2ce2314dc6448d9338c15b0a00000000u128, 0x20);
dec_test!(bid128ddd_fma_003, bid128ddd_fma, 0, 0x2000a080a0205904u64, 0xfffffffffffeffffu64, 0xc10282f50825c160u64, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128ddd_fma_004, bid128ddd_fma, 0, 0xd6df8feb53d2d16eu64, 0x0080000000002000u64, 0x4589aa10001a14d1u64, 0x3158861e83b104da4f3a31e778a40000u128, 0x20);
