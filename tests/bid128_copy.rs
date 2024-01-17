/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

mod common;

dec_test!(bid128copy_001, bid128_copy, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128);
dec_test!(bid128copy_002, bid128_copy, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128);
dec_test!(bid128copy_003, bid128_copy, 0u64                                  , 0x30400000000000000000000000000000u128);
dec_test!(bid128copy_004, bid128_copy, 0x0ab55d45ba5c2626b47ddbc6aa021c95u128, 0x0ab55d45ba5c2626b47ddbc6aa021c95u128);
dec_test!(bid128copy_005, bid128_copy, -0i64                                 , 0xb0400000000000000000000000000000u128);
dec_test!(bid128copy_006, bid128_copy, 0x29443c736a0efa8ba221e17adf1c9fc9u128, 0x29443c736a0efa8ba221e17adf1c9fc9u128);
dec_test!(bid128copy_007, bid128_copy, 0x47940000000000000000000000000000u128, 0x47940000000000000000000000000000u128);
dec_test!(bid128copy_008, bid128_copy, 0x7c0009caa3e842abb29f72dd820c15dcu128, 0x7c0009caa3e842abb29f72dd820c15dcu128);
dec_test!(bid128copy_009, bid128_copy, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128);
dec_test!(bid128copy_010, bid128_copy, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128);
dec_test!(bid128copy_011, bid128_copy, 0x88e252f2cc9f2c3f6e07569844058777u128, 0x88e252f2cc9f2c3f6e07569844058777u128);
dec_test!(bid128copy_012, bid128_copy, 0x93a00000000000000000000000000000u128, 0x93a00000000000000000000000000000u128);
dec_test!(bid128copy_013, bid128_copy, 0x93cfd2202869ccd5379d5f0716254290u128, 0x93cfd2202869ccd5379d5f0716254290u128);
dec_test!(bid128copy_014, bid128_copy, 0xac891470e69092a16c8bb59894765b3du128, 0xac891470e69092a16c8bb59894765b3du128);
dec_test!(bid128copy_015, bid128_copy, 0xcb5a0000000000000000000000000000u128, 0xcb5a0000000000000000000000000000u128);
dec_test!(bid128copy_016, bid128_copy, 0xd3bb2e1467023578a75d9f49ed048a17u128, 0xd3bb2e1467023578a75d9f49ed048a17u128);
dec_test!(bid128copy_017, bid128_copy, 0xfde7bf7f3dfbafd8ffffbefbfff7fffeu128, 0xfde7bf7f3dfbafd8ffffbefbfff7fffeu128);
// dec_test!(bid128copy_018, bid128_copy, Infinity [78000000000000000000000000000000]);
// dec_test!(bid128copy_019, bid128_copy, -Infinity [f8000000000000000000000000000000]);
// dec_test!(bid128copy_020, bid128_copy, SNaN [7e000000000000000000000000000000]);
