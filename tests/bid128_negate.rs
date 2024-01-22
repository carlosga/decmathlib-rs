/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_negate_001, bid128_negate,  0x0001ed09bead87c0378d8e62ffffffffu128, 0x8001ed09bead87c0378d8e62ffffffffu128);
dec_test!(bid128_negate_002, bid128_negate,  0x0001ed09bead87c0378d8e64ffffffffu128, 0x8001ed09bead87c0378d8e64ffffffffu128);
// dec_test!(bid128_negate_003, bid128_negate,  -0i64                                 , 0x30400000000000000000000000000000u128);
// dec_test!(bid128_negate_004, bid128_negate,  0u64                                  , 0xb0400000000000000000000000000000u128);
dec_test!(bid128_negate_005, bid128_negate,  0x126fdbaf02bceaeed9697b435c361fe1u128, 0x926fdbaf02bceaeed9697b435c361fe1u128);
dec_test!(bid128_negate_006, bid128_negate,  0x2b237eb316d839ca2d37d68b0227c11eu128, 0xab237eb316d839ca2d37d68b0227c11eu128);
dec_test!(bid128_negate_007, bid128_negate,  0x4e1c2958e3bd4c516f86468008d2aa28u128, 0xce1c2958e3bd4c516f86468008d2aa28u128);
dec_test!(bid128_negate_008, bid128_negate,  0x50d3b7f263af5b88b87a6768ec8b46d0u128, 0xd0d3b7f263af5b88b87a6768ec8b46d0u128);
dec_test!(bid128_negate_009, bid128_negate,  0x50d9e61cdd280bd8e25a31953ffc0a48u128, 0xd0d9e61cdd280bd8e25a31953ffc0a48u128);
dec_test!(bid128_negate_010, bid128_negate,  0x7c003fffffffffff38c15b08ffffffffu128, 0xfc003fffffffffff38c15b08ffffffffu128);
dec_test!(bid128_negate_011, bid128_negate,  0x7c003fffffffffff38c15b0affffffffu128, 0xfc003fffffffffff38c15b0affffffffu128);
dec_test!(bid128_negate_012, bid128_negate,  0x8daa55a8f5cff6db55b5bea4a37b8a72u128, 0x0daa55a8f5cff6db55b5bea4a37b8a72u128);
dec_test!(bid128_negate_013, bid128_negate,  0x9b147225df5ff554350ecd34592da44au128, 0x1b147225df5ff554350ecd34592da44au128);
dec_test!(bid128_negate_014, bid128_negate,  0x9e477b02c3805ef4b738f98b244817ddu128, 0x1e477b02c3805ef4b738f98b244817ddu128);
dec_test!(bid128_negate_015, bid128_negate,  0xa21e1edbb763cb704d680be7caaddd1au128, 0x221e1edbb763cb704d680be7caaddd1au128);
dec_test!(bid128_negate_016, bid128_negate,  0xaa1a04dcb7d4ea0f8363e10e1240d936u128, 0x2a1a04dcb7d4ea0f8363e10e1240d936u128);
dec_test!(bid128_negate_017, bid128_negate,  0xfffffffffeffffbab0086298e0a00010u128, 0x7ffffffffeffffbab0086298e0a00010u128);
// dec_test!(bid128_negate_018, bid128_negate, -Infinity [78000000000000000000000000000000]);
// dec_test!(bid128_negate_019, bid128_negate, Infinity [f8000000000000000000000000000000]);
// dec_test!(bid128_negate_020, bid128_negate, QNaN [fc000000000000000000000000000000]);
// dec_test!(bid128_negate_021, bid128_negate, SNaN [fe000000000000000000000000000000]);
