/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

mod common;

dec_test!(bid128_to_bid64_001, bid128_to_bid64, 0, 0x00000000000000000080100000800080u128, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_002, bid128_to_bid64, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_003, bid128_to_bid64, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0000000000000000u64, 00);
// bid128_to_bid64_004, bid128_to_bid64, 0, 1.234567890123456E-397 [000000000000000c] 30
// bid128_to_bid64_005, bid128_to_bid64, 0, 1.234567890123456E-398 [0000000000000001] 30
dec_test!(bid128_to_bid64_006, bid128_to_bid64, 0, 0x2ce1230b00900000002a008000c20080u128, 0x0000000000000001u64, 30);
dec_test!(bid128_to_bid64_007, bid128_to_bid64, 0, 0x2cffed09bead87c0378d8e63ffffffffu128, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_008, bid128_to_bid64, 0, 0x2d1e0000000000000de0b6b3a763ffa1u128, 0x00038d7ea4c68000u64, 30);
dec_test!(bid128_to_bid64_009, bid128_to_bid64, 0, 0x2d20000000000000016345785d89fffbu128, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_010, bid128_to_bid64, 0, 0x2f720000000000000000000005f5e0fbu128, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_011, bid128_to_bid64, 0, 0x4000000000000000009010e002120858u128, 0x7800000000000000u64, 28);
dec_test!(bid128_to_bid64_012, bid128_to_bid64, 0, 0x4e37fa4093c6d5fa7eff7fef7f7afff6u128, 0x5fe0000000000000u64, 00);
dec_test!(bid128_to_bid64_013, bid128_to_bid64, 0, 0x7c0000033b2e3c9fd0803ce800000000u128, 0x7c00003b9aca0000u64, 00);
dec_test!(bid128_to_bid64_014, bid128_to_bid64, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_015, bid128_to_bid64, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_016, bid128_to_bid64, 0, 0xacffed09bead87c0378d8e63ffffffffu128, 0x80038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_017, bid128_to_bid64, 0, 0xaf720000000000000000000005f5e0fbu128, 0xa4e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_018, bid128_to_bid64, 0, 0xe3396e4f8d790fc9b5c7464701b29b6bu128, 0x8000000000000000u64, 00);
dec_test!(bid128_to_bid64_019, bid128_to_bid64, 0, 0xffffff7fffffffffffc77fffff3fffdfu128, 0xfc00000000000000u64, 01);
dec_test!(bid128_to_bid64_020, bid128_to_bid64, 1, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_021, bid128_to_bid64, 1, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0000000000000000u64, 00);
dec_test!(bid128_to_bid64_022, bid128_to_bid64, 1, 0x2cffed09bead87c0378d8e63ffffffffu128, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_023, bid128_to_bid64, 1, 0x2d1e0000000000000de0b6b3a763ffa1u128, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_024, bid128_to_bid64, 1, 0x2d20000000000000016345785d89fffbu128, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_025, bid128_to_bid64, 1, 0x2f720000000000000000000005f5e0fbu128, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_026, bid128_to_bid64, 1, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_027, bid128_to_bid64, 1, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_028, bid128_to_bid64, 1, 0xacffed09bead87c0378d8e63ffffffffu128, 0x80038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_029, bid128_to_bid64, 1, 0xaf720000000000000000000005f5e0fbu128, 0xa4e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_030, bid128_to_bid64, 2, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0000000000000001u64, 30);
dec_test!(bid128_to_bid64_031, bid128_to_bid64, 2, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0000000000000000u64, 00);
dec_test!(bid128_to_bid64_032, bid128_to_bid64, 2, 0x2cffed09bead87c0378d8e63ffffffffu128, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_033, bid128_to_bid64, 2, 0x2d1e0000000000000de0b6b3a763ffa1u128, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_034, bid128_to_bid64, 2, 0x2d20000000000000016345785d89fffbu128, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_035, bid128_to_bid64, 2, 0x2f720000000000000000000005f5e0fbu128, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_036, bid128_to_bid64, 2, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_037, bid128_to_bid64, 2, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_038, bid128_to_bid64, 2, 0x8ee8068322a31587b7a0d168b1fee707u128, 0x8000000000000000u64, 30);
dec_test!(bid128_to_bid64_039, bid128_to_bid64, 2, 0xacffed09bead87c0378d8e63ffffffffu128, 0x80038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_040, bid128_to_bid64, 2, 0xaf720000000000000000000005f5e0fbu128, 0xa4e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_041, bid128_to_bid64, 3, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_042, bid128_to_bid64, 3, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0000000000000000u64, 00);
dec_test!(bid128_to_bid64_043, bid128_to_bid64, 3, 0x2cffed09bead87c0378d8e63ffffffffu128, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_044, bid128_to_bid64, 3, 0x2d1e0000000000000de0b6b3a763ffa1u128, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_045, bid128_to_bid64, 3, 0x2d20000000000000016345785d89fffbu128, 0x00038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_046, bid128_to_bid64, 3, 0x2f720000000000000000000005f5e0fbu128, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_047, bid128_to_bid64, 3, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_048, bid128_to_bid64, 3, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_049, bid128_to_bid64, 3, 0xacffed09bead87c0378d8e63ffffffffu128, 0x80038d7ea4c67fffu64, 30);
dec_test!(bid128_to_bid64_050, bid128_to_bid64, 3, 0xaf720000000000000000000005f5e0fbu128, 0xa4e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_051, bid128_to_bid64, 4, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0000000000000000u64, 30);
dec_test!(bid128_to_bid64_052, bid128_to_bid64, 4, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0000000000000000u64, 00);
dec_test!(bid128_to_bid64_053, bid128_to_bid64, 4, 0x2cffed09bead87c0378d8e63ffffffffu128, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_054, bid128_to_bid64, 4, 0x2d1e0000000000000de0b6b3a763ffa1u128, 0x00038d7ea4c68000u64, 30);
dec_test!(bid128_to_bid64_055, bid128_to_bid64, 4, 0x2d20000000000000016345785d89fffbu128, 0x00038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_056, bid128_to_bid64, 4, 0x2f720000000000000000000005f5e0fbu128, 0x24e0000005f5e0fbu64, 00);
dec_test!(bid128_to_bid64_057, bid128_to_bid64, 4, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_058, bid128_to_bid64, 4, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c00000000000000u64, 00);
dec_test!(bid128_to_bid64_059, bid128_to_bid64, 4, 0xacffed09bead87c0378d8e63ffffffffu128, 0x80038d7ea4c68000u64, 30); // underflow_before_only
dec_test!(bid128_to_bid64_060, bid128_to_bid64, 4, 0xaf720000000000000000000005f5e0fbu128, 0xa4e0000005f5e0fbu64, 00);