/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_maxnum_mag_001, bid128_maxnum_mag, 0x0000000000200000ff7edfdffedffb57u128, 0x0020000000000000f3454c8a69445b00u128, 0x0020000000000000f3454c8a69445b00u128, 0x00);
dec_test!(bid128_maxnum_mag_002, bid128_maxnum_mag, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_mag_003, bid128_maxnum_mag, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_mag_004, bid128_maxnum_mag, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_mag_005, bid128_maxnum_mag, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_mag_006, bid128_maxnum_mag, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_mag_007, bid128_maxnum_mag, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_008, bid128_maxnum_mag, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_009, bid128_maxnum_mag, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_010, bid128_maxnum_mag, 0x00641040800040002a6a5fff4d5880f7u128, 0x1492b2a8303429005232b369c08c105eu128, 0x1492b2a8303429005232b369c08c105eu128, 0x00);
dec_test!(bid128_maxnum_mag_011, bid128_maxnum_mag, 0x0100020000800208d1c8ecb1200aa485u128, 0x00d104002c010804df143eeff57f7ccbu128, 0x0100020000800208d1c8ecb1200aa485u128, 0x00);
dec_test!(bid128_maxnum_mag_012, bid128_maxnum_mag, 0x0400000000040000fffffbbeffff65cbu128, 0xfdcfbd08a3b4cb8b41c82e9545c0d73eu128, 0x0400000000040000fffffbbeffff65cbu128, 0x00);
dec_test!(bid128_maxnum_mag_013, bid128_maxnum_mag, 0x0ad60000000000000000000000000000u128, 0xa3f20000000000000000000000000000u128, 0xa3f20000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_014, bid128_maxnum_mag, "0"                                   , "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_mag_015, bid128_maxnum_mag, 0x1bad4f2097138a19d7dffb19891d90c3u128, 0xfe000000000000000000000000000000u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_mag_016, bid128_maxnum_mag, 0x1c39130e40988123efdffeebfffffffdu128, 0xf41fac9a7b40a5f716262844df37aff6u128, 0x1c39130e40988123efdffeebfffffffdu128, 0x00);
dec_test!(bid128_maxnum_mag_017, bid128_maxnum_mag, "-1E+368"                             , "1E+368"                              , 0x33200000000000000000000000000001u128, 0x00);
dec_test!(bid128_maxnum_mag_018, bid128_maxnum_mag, "1E+368"                              , "-1E+368"                             , 0x33200000000000000000000000000001u128, 0x00);
dec_test!(bid128_maxnum_mag_019, bid128_maxnum_mag, 0x263ba2e601bd6329922ebd7e19ef4fabu128, 0x9608e71859649bdf01b36c8d541279f1u128, 0x263ba2e601bd6329922ebd7e19ef4fabu128, 0x00);
dec_test!(bid128_maxnum_mag_020, bid128_maxnum_mag, 0x30082881a0811245420a832140010600u128, 0xafffdcb77ffbea5e1b7a8d1e6b697dccu128, 0x30082881a0811245420a832140010600u128, 0x00);
dec_test!(bid128_maxnum_mag_021, bid128_maxnum_mag, 0x5092b89a95eeab5cf0c77fcc135a96e8u128, 0x4ed48fe33b6a99fa509dfec6e9bdbae8u128, 0x5092b89a95eeab5cf0c77fcc135a96e8u128, 0x00);
dec_test!(bid128_maxnum_mag_022, bid128_maxnum_mag, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_mag_023, bid128_maxnum_mag, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_024, bid128_maxnum_mag, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_025, bid128_maxnum_mag, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_026, bid128_maxnum_mag, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_maxnum_mag_027, bid128_maxnum_mag, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_028, bid128_maxnum_mag, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_029, bid128_maxnum_mag, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_030, bid128_maxnum_mag, 0x7ca3c7ffff7e79dd7fefbffffffed79fu128, 0xff77f777fffddbf9105c11204826e019u128, 0x7c0007ffff7e79dd7fefbffffffed79fu128, 0x01);
dec_test!(bid128_maxnum_mag_031, bid128_maxnum_mag, 0x7e000000000000000000000000000000u128, 0x89f20000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_mag_032, bid128_maxnum_mag, 0x8fffff7ffffefcfd2120800112880208u128, 0xffffbfffffffffff040808fc20488028u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_mag_033, bid128_maxnum_mag, 0x964987cd3157314f10cffc876bc0ff0bu128, 0x29f6ac72eee0487120de712326112596u128, 0x29f6ac72eee0487120de712326112596u128, 0x00);
dec_test!(bid128_maxnum_mag_034, bid128_maxnum_mag, 0x970e0b1b56280b01c973c0208469fa41u128, 0x2100000000000000f874b7b6d8b6cfa2u128, 0x2100000000000000f874b7b6d8b6cfa2u128, 0x00);
dec_test!(bid128_maxnum_mag_035, bid128_maxnum_mag, "-9.9E0"                              , "-0"                                  , 0xb03e0000000000000000000000000063u128, 0x00);
dec_test!(bid128_maxnum_mag_036, bid128_maxnum_mag, 0x9c8a931fd008e65ea9873b88f8bac160u128, 0x1cc492d33fe2a5dfe71df7407adb8472u128, 0x1cc492d33fe2a5dfe71df7407adb8472u128, 0x00);
dec_test!(bid128_maxnum_mag_037, bid128_maxnum_mag, 0x9d53d8013b6b8f8d894ed536acddb85fu128, 0xa8140000000000000000000000000000u128, 0x9d53d8013b6b8f8d894ed536acddb85fu128, 0x00);
dec_test!(bid128_maxnum_mag_038, bid128_maxnum_mag, 0xa5bdb86d21ef6ec6374d060f857c2b7fu128, 0x7e000000000000000000000000000000u128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_mag_039, bid128_maxnum_mag, 0xa7c95f791bc72f21385c495d72fbab94u128, 0xb7fbf7ddffdfbfff4ff9fffff37fa7ffu128, 0xa7c95f791bc72f21385c495d72fbab94u128, 0x00);
dec_test!(bid128_maxnum_mag_040, bid128_maxnum_mag, 0xb5d5a074ef82398fca1ff8b6c84317ceu128, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_041, bid128_maxnum_mag, 0xb6fa9cf8706c4a61172cc79d6309288cu128, 0xc63eea6ca209d7f05851bdb535e1ebf6u128, 0xc63eea6ca209d7f05851bdb535e1ebf6u128, 0x00);
dec_test!(bid128_maxnum_mag_042, bid128_maxnum_mag, 0xb83e7213dd517504265b3c55420186ceu128, 0x281d2910e41774e48cdef786c7039cd4u128, 0xb83e7213dd517504265b3c55420186ceu128, 0x00);
dec_test!(bid128_maxnum_mag_043, bid128_maxnum_mag, 0xd367780f595ae7f58bbbf16219e97237u128, 0x99147f1e68296d7a4001028001000000u128, 0xd367780f595ae7f58bbbf16219e97237u128, 0x00);
dec_test!(bid128_maxnum_mag_044, bid128_maxnum_mag, 0xe9aa72710a5910ff0480304010000800u128, 0x7bff7fffffd37f3e1ca2094318943dd8u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_045, bid128_maxnum_mag, 0xf7fb7f5759fc7c9f47fefffbfffffbd6u128, 0xf68ce1d1ae8eb67be1e26d91cb2c5ae8u128, 0xda320000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_046, bid128_maxnum_mag, 0xfbbd5b9d8573bcbff77b63fbeebef67eu128, 0x3e79bbbaef7ecf7a1000200229481244u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_047, bid128_maxnum_mag, 0xfffffffffffffffed3420083088d9e17u128, 0xf6efdff976f9ffef1000011000800441u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_mag_048, bid128_maxnum_mag, "Infinity"                            , "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_049, bid128_maxnum_mag, "-Infinity"                           , "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_mag_050, bid128_maxnum_mag, "QNaN"                                , "0"                                   , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_051, bid128_maxnum_mag, "QNaN"                                , "-Infinity"                           , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_maxnum_mag_052, bid128_maxnum_mag, "SNaN"                                , "0"                                   , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_maxnum_mag_053, bid128_maxnum_mag, "SNaN"                                , "Infinity"                            , 0x7c000000000000000000000000000000u128, 0x01);
