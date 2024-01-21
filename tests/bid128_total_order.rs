/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_total_order_001, bid128_total_order, 0x0000000000000000ffffffffffffffffu128, 0x0000000000000000ffffffffffffffffu128, true);
dec_test!(bid128_total_order_002, bid128_total_order, 0x0000800001000200028008416282930cu128, 0x00200081010010002020400c02040100u128, true);
dec_test!(bid128_total_order_003, bid128_total_order, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_total_order_004, bid128_total_order, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_total_order_005, bid128_total_order, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_total_order_006, bid128_total_order, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_total_order_007, bid128_total_order, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_total_order_008, bid128_total_order, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true);
dec_test!(bid128_total_order_009, bid128_total_order, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_total_order_010, bid128_total_order, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_total_order_011, bid128_total_order, 0x000400002013004825fa092c1e1ee6c8u128, 0x00004000000000000cd15632d0300103u128, true);
dec_test!(bid128_total_order_012, bid128_total_order, 0x004a580447021010018f6f3bbd0d2b88u128, 0x0084008281a14083fb7dfdffff77ffffu128, true);
dec_test!(bid128_total_order_013, bid128_total_order, 0x0d000c1446001003fff5ffffffffffffu128, 0x08002e000209441478f0040000120004u128, false);
// dec_test!(bid128_total_order_014, bid128_total_order, 0 -Infinity 0);
// dec_test!(bid128_total_order_015, bid128_total_order, 0 QNaN 1);
// dec_test!(bid128_total_order_016, bid128_total_order, 1.0 1 1);
// dec_test!(bid128_total_order_017, bid128_total_order, 1 1.0 0);
dec_test!(bid128_total_order_018, bid128_total_order, 0x133a0000000000000000000000000000u128, 0x2ade0000000000000000000000000000u128, true);
dec_test!(bid128_total_order_019, bid128_total_order, 0x14802824191014a1779d8daf351dfbeeu128, 0x144d08441c682a028020066102104124u128, false);
dec_test!(bid128_total_order_020, bid128_total_order, 0x185f5fa24d678c66b29a0597043e64fcu128, 0x64002002000000002fffddfefff3e34eu128, false);
dec_test!(bid128_total_order_021, bid128_total_order, 0x246b0a5e3e2f03248efd6c2a05a2aceau128, 0x278b7fd44a9b967aa629ff533ce0677au128, true);
dec_test!(bid128_total_order_022, bid128_total_order, 0x32696fa0fd49eb8df7318b060050df56u128, 0xbac66c3159b9fa682e68dc8db7f63bd8u128, false);
dec_test!(bid128_total_order_023, bid128_total_order, 0x3a28583606597e4cb960d8f0ddd43c6bu128, 0x34fe0000000000000000000000000000u128, false);
dec_test!(bid128_total_order_024, bid128_total_order, 0x53ae5da8aee8ba2e8dfcb7418d496b79u128, 0xbbc600ff3db059c21ee248bdb3d15c2cu128, false);
dec_test!(bid128_total_order_025, bid128_total_order, 0x58ba441288e818bebc6faf765177bea8u128, 0xc391142a9af6d5867fd21fc8b8784561u128, false);
dec_test!(bid128_total_order_026, bid128_total_order, 0x6afe92605d9dbde41406dc372a61e19du128, 0x6e67cd1242f59aad8bc82ab07f0ec0b7u128, true);
dec_test!(bid128_total_order_027, bid128_total_order, 0x7bef7f677f3afb5f1348491fb70fa9cau128, 0x4244302914040e41f9485447f5f41a0bu128, false);
dec_test!(bid128_total_order_028, bid128_total_order, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_total_order_029, bid128_total_order, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_total_order_030, bid128_total_order, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_total_order_031, bid128_total_order, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_total_order_032, bid128_total_order, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_total_order_033, bid128_total_order, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_total_order_034, bid128_total_order, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_total_order_035, bid128_total_order, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_total_order_036, bid128_total_order, 0x7f38682481572d590b67b82e477cd7deu128, 0x7fd6165ccd7e394155c06abeb13d164au128, false);
dec_test!(bid128_total_order_037, bid128_total_order, 0x7f92e6cefdbfdbf5acfef76ffcf9fdfeu128, 0x7c634f77456b79abc09a0440490302b8u128, true);
dec_test!(bid128_total_order_038, bid128_total_order, 0x7fbddf7fe0f7fcef0000000000000044u128, 0x5538785dc81d8f2169efe5ec26c4cc1cu128, false);
dec_test!(bid128_total_order_039, bid128_total_order, 0x7fbfffffffffefffdfbffeffdbff3fd3u128, 0x7e77797e867ffee6fe47ae03f239dee9u128, true);
dec_test!(bid128_total_order_040, bid128_total_order, 0x7fdcd2f4deacbfad83fef3574ff7cf17u128, 0x7edfbefe5d3fff9fdf96ca7b7356e9eau128, false);
dec_test!(bid128_total_order_041, bid128_total_order, 0x84805120d24f080397907a7f05a0db1au128, 0xdf7fbff77f7bbdbf16baefbfbfffbffdu128, false);
dec_test!(bid128_total_order_042, bid128_total_order, 0x9321daf777a7c9f745012202bd800100u128, 0xc0a4e6830729b80d0000008000000900u128, false);
dec_test!(bid128_total_order_043, bid128_total_order, 0x95c6a7ea4b25d7cf6c8e2de8dcd31341u128, 0x00000000001000006086088f8a13d900u128, true);
dec_test!(bid128_total_order_044, bid128_total_order, 0xb4ed28f19377656650ede266cec5a024u128, 0x85000811208080409fa00800188f365cu128, true);
dec_test!(bid128_total_order_045, bid128_total_order, 0xb78f9a2ad65fd8afc2fdac509f2bdc67u128, 0xb5e738ae96c4c8f6974cd11cdcd27cc8u128, true);
dec_test!(bid128_total_order_046, bid128_total_order, 0xbfbffeff4fffff9fffffffffffffcfffu128, 0xefef96ffddfffd9e6c964cc486963418u128, true);
dec_test!(bid128_total_order_047, bid128_total_order, 0xcb79c5a0de13eb98e8aecf850026cca6u128, 0x3cfba2a583390d064d0547c14266f9e3u128, true);
dec_test!(bid128_total_order_048, bid128_total_order, 0xddf2cc200b3b8941afcf7ffef1ffff79u128, 0xfbdfb7ffa99fff7b93bbb3f53fec6fc6u128, false);
dec_test!(bid128_total_order_049, bid128_total_order, 0xdee13a4422840ac7080a21756dda1bfeu128, 0xa5ae0000000000000000000000000000u128, true);
dec_test!(bid128_total_order_050, bid128_total_order, 0xe0a24502dd0784c2364c30b8f85ed837u128, 0xa7b1e02861f6197968317a09807c26cfu128, false);
dec_test!(bid128_total_order_051, bid128_total_order, 0xf77ffffff7ffb9f7ffffffffffffffffu128, 0xfeef85516b0b4d6c2de2a3a4a70855cau128, false);
dec_test!(bid128_total_order_052, bid128_total_order, 0xf8000000000000000000000000000000u128, 0x4c094a8ea1ef7251af241ec9d388942au128, true);
dec_test!(bid128_total_order_053, bid128_total_order, 0xfbeeb7ff032f7ff70000200012010000u128, 0xe7efd96f747f796fffffffffffffffffu128, true);
dec_test!(bid128_total_order_054, bid128_total_order, 0xfdf2ff2dff6bbf7a2818000001000250u128, 0xff2c8f8b4b10c8185c26005e04e5b158u128, true);
dec_test!(bid128_total_order_055, bid128_total_order, 0xfeffeffffedefdffbffb587655e7a90eu128, 0xff6edfefffefffdda0002655100208c8u128, true);
dec_test!(bid128_total_order_056, bid128_total_order, 0xffeffbffbfbacdff529db3ce9947f86bu128, 0x1bfe5c2f9969b3f7bfe3fbfdffdffe7fu128, true);
dec_test!(bid128_total_order_057, bid128_total_order, 0xffffffff7fdffdff4cb54af98e997b8au128, 0xfffdfefdfdfff7770100000000008084u128, true);
dec_test!(bid128_total_order_058, bid128_total_order, 0xffffffffffffffffffefdfffffffff9fu128, 0xffbc1f7fbbddcc7f0150880281258400u128, false);
// dec_test!(bid128_total_order_059, bid128_total_order, Infinity 0 0);
// dec_test!(bid128_total_order_060, bid128_total_order, Infinity QNaN 1);
// dec_test!(bid128_total_order_061, bid128_total_order, -Infinity SNaN 1);
// dec_test!(bid128_total_order_062, bid128_total_order, Infinity SNaN 1);
// dec_test!(bid128_total_order_063, bid128_total_order, QNaN 0 0);
// dec_test!(bid128_total_order_064, bid128_total_order, QNaN -Infinity 0);
// dec_test!(bid128_total_order_065, bid128_total_order, SNaN QNaN 1);