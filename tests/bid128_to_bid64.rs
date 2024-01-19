/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

mod common;

dec_test!(bid128_same_quantum_001, bid128_same_quantum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x00000000000000000080100000800080u128, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0000000000000000u64, 00);
// bid128_to_bid64_, bid128_to_bid64, 0, 1.234567890123456E-397 [000000000000000c] 30
// bid128_to_bid64_, bid128_to_bid64, 0, 1.234567890123456E-398 [0000000000000001] 30
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x2ce1230b00900000002a008000c20080, 0x0000000000000001u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x2cffed09bead87c0378d8e63ffffffff, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x2d1e0000000000000de0b6b3a763ffa1, 0x00038d7ea4c68000u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x2d20000000000000016345785d89fffb, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x2f720000000000000000000005f5e0fb, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x4000000000000000009010e002120858, 0x7800000000000000u64, 28);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x4e37fa4093c6d5fa7eff7fef7f7afff6, 0x5fe0000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x7c0000033b2e3c9fd0803ce800000000, 0x7c00003b9aca0000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x7c003fffffffffff38c15b08ffffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0x7c003fffffffffff38c15b0affffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0xacffed09bead87c0378d8e63ffffffff, 0x80038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0xaf720000000000000000000005f5e0fb, 0xa4e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0xe3396e4f8d790fc9b5c7464701b29b6b, 0x8000000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 0, 0xffffff7fffffffffffc77fffff3fffdf, 0xfc00000000000000u64, 01);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0x0001ed09bead87c0378d8e62ffffffff, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0x0001ed09bead87c0378d8e64ffffffff, 0x0000000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0x2cffed09bead87c0378d8e63ffffffff, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0x2d1e0000000000000de0b6b3a763ffa1, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0x2d20000000000000016345785d89fffb, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0x2f720000000000000000000005f5e0fb, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0x7c003fffffffffff38c15b08ffffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0x7c003fffffffffff38c15b0affffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0xacffed09bead87c0378d8e63ffffffff, 0x80038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 1, 0xaf720000000000000000000005f5e0fb, 0xa4e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0x0001ed09bead87c0378d8e62ffffffff, 0x0000000000000001u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0x0001ed09bead87c0378d8e64ffffffff, 0x0000000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0x2cffed09bead87c0378d8e63ffffffff, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0x2d1e0000000000000de0b6b3a763ffa1, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0x2d20000000000000016345785d89fffb, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0x2f720000000000000000000005f5e0fb, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0x7c003fffffffffff38c15b08ffffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0x7c003fffffffffff38c15b0affffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0x8ee8068322a31587b7a0d168b1fee707, 0x8000000000000000u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0xacffed09bead87c0378d8e63ffffffff, 0x80038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 2, 0xaf720000000000000000000005f5e0fb, 0xa4e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0x0001ed09bead87c0378d8e62ffffffff, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0x0001ed09bead87c0378d8e64ffffffff, 0x0000000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0x2cffed09bead87c0378d8e63ffffffff, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0x2d1e0000000000000de0b6b3a763ffa1, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0x2d20000000000000016345785d89fffb, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0x2f720000000000000000000005f5e0fb, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0x7c003fffffffffff38c15b08ffffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0x7c003fffffffffff38c15b0affffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0xacffed09bead87c0378d8e63ffffffff, 0x80038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 3, 0xaf720000000000000000000005f5e0fb, 0xa4e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0x0001ed09bead87c0378d8e62ffffffff, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0x0001ed09bead87c0378d8e64ffffffff, 0x0000000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0x2cffed09bead87c0378d8e63ffffffff, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0x2d1e0000000000000de0b6b3a763ffa1, 0x00038d7ea4c68000u64, 30);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0x2d20000000000000016345785d89fffb, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0x2f720000000000000000000005f5e0fb, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0x7c003fffffffffff38c15b08ffffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0x7c003fffffffffff38c15b0affffffff, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0xacffed09bead87c0378d8e63ffffffff, 0x80038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_, bid128_to_bid64, 4, 0xaf720000000000000000000005f5e0fb, 0xa4e0000005f5e0fbu64, 00);
