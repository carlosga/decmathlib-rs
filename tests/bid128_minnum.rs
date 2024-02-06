/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_minnum_001, bid128_minnum, 0x00009005c2420001088d8a13d1000082u128, 0x80060010091020041399074b7522c59fu128, 0x80060010091020041399074b7522c59fu128, 0x00);
dec_test!(bid128_minnum_002, bid128_minnum, 0x0001000000600004f7ffffffffd6bfffu128, 0x00000000800000000004000a30030402u128, 0x00000000800000000004000a30030402u128, 0x00);
dec_test!(bid128_minnum_003, bid128_minnum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_004, bid128_minnum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_005, bid128_minnum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_006, bid128_minnum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_007, bid128_minnum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_008, bid128_minnum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_009, bid128_minnum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_010, bid128_minnum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_011, bid128_minnum, 0x00201840280060900000000040002001u128, 0x000000000080000040c0810000001002u128, 0x000000000080000040c0810000001002u128, 0x00);
dec_test!(bid128_minnum_012, bid128_minnum, 0x0040000000000040bfdffeffb7f7ee59u128, 0x00004000000000000200000000002000u128, 0x00004000000000000200000000002000u128, 0x00);
dec_test!(bid128_minnum_013, bid128_minnum, "-0"                                  , "0"                                   , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_014, bid128_minnum, 0x01000504000000002100804858904c40u128, 0x0112000da2b95024371d1c8656d3d2c0u128, 0x01000504000000002100804858904c40u128, 0x00);
dec_test!(bid128_minnum_015, bid128_minnum, 0x02000180081200000010480002000004u128, 0xff7dffffffffefdfb442019805eb1075u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_minnum_016, bid128_minnum, 0x0347dcdae0aaa652bbd2de6cba73a4f2u128, 0x52239323eede5d299760d59a8000ff63u128, 0x0347dcdae0aaa652bbd2de6cba73a4f2u128, 0x00);
dec_test!(bid128_minnum_017, bid128_minnum, "0E+368"                              , 0x7e000000000000000000000000000001u128, 0x7c000000000000000000000000000001u128, 0x01);
dec_test!(bid128_minnum_018, bid128_minnum, "0"                                   , "Infinity"                            , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_019, bid128_minnum, "0"                                   , "-Infinity"                           , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_020, bid128_minnum, 0x151002b72b02c7949e4bf55b377dcfdfu128, 0xfbb7487169ace7fdf95f7bcedc2cda5bu128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_021, bid128_minnum, 0x160ec932727f1a8ce5dbc0c211dc7256u128, 0x4d48a0f72a685fff3024391054b0b2e8u128, 0x160ec932727f1a8ce5dbc0c211dc7256u128, 0x00);
dec_test!(bid128_minnum_022, bid128_minnum, 0x17100000000000000000000000000000u128, 0x28e70922e9e4260465ef3156f4c20245u128, 0x17100000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_023, bid128_minnum, 0x193be6702b02cd8124da4a01ff19b553u128, 0x78000000000000000000000000000000u128, 0x193be6702b02cd8124da4a01ff19b553u128, 0x00);
dec_test!(bid128_minnum_024, bid128_minnum, 0x207feb4a0291e95068960d1470150f01u128, 0x32da0000000000000000000000000000u128, 0x32da0000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_025, bid128_minnum, 0x28ddaa9ae994f89527f6092238ba55d7u128, 0x245d7ab76258431165701913c7d62199u128, 0x245d7ab76258431165701913c7d62199u128, 0x00);
dec_test!(bid128_minnum_026, bid128_minnum, 0x485438cb0caf54fb5c69ce43c092baf1u128, 0xd2c8e350f918d0f97ed9db13f3aec63eu128, 0xd2c8e350f918d0f97ed9db13f3aec63eu128, 0x00);
dec_test!(bid128_minnum_027, bid128_minnum, 0x5178ff58cf9c6b1df53d798156303c9du128, 0x2b4acbd19c0356034528f560b2f4fea6u128, 0x2b4acbd19c0356034528f560b2f4fea6u128, 0x00);
dec_test!(bid128_minnum_028, bid128_minnum, 0x54ac946b7cce1b31d781d5d450b98464u128, 0x27bea255e5d59b55fe13d14eed812927u128, 0x27bea255e5d59b55fe13d14eed812927u128, 0x00);
dec_test!(bid128_minnum_029, bid128_minnum, 0x58a19f4b9621dbaa8556a1e254a2f5c9u128, 0x78000000000000000000000000000000u128, 0x58a19f4b9621dbaa8556a1e254a2f5c9u128, 0x00);
dec_test!(bid128_minnum_030, bid128_minnum, 0x7bf8bdfffffff6ef0048000000000008u128, 0x1080010000000000ffff77fffff7ffffu128, 0x1080010000000000ffff77fffff7ffffu128, 0x00);
dec_test!(bid128_minnum_031, bid128_minnum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_032, bid128_minnum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_033, bid128_minnum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_034, bid128_minnum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_035, bid128_minnum, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_036, bid128_minnum, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_037, bid128_minnum, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_038, bid128_minnum, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_039, bid128_minnum, 0x7e001dbfd43a3ca7e1d31c445bea1a55u128, 0x26fcf2597c7ac88d8b7e9433fb8c38f5u128, 0x7c001dbfd43a3ca7e1d31c445bea1a55u128, 0x01);
dec_test!(bid128_minnum_040, bid128_minnum, 0x7fff36f7ffff4fd9fffffff9ffffbfbfu128, 0xe178bad63da08f9c7f1f6ff77fffcfffu128, 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_minnum_041, bid128_minnum, 0x85f5cb23db101aef8125da3f63ae0bf6u128, 0x55380000000000000000000000000000u128, 0x85f5cb23db101aef8125da3f63ae0bf6u128, 0x00);
dec_test!(bid128_minnum_042, bid128_minnum, 0x97bffdffdffff5bf3fade4e373e93a1fu128, 0x0008051800201004dd9ffedeb7efdd1fu128, 0x97be0000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_043, bid128_minnum, 0xa000808835003541cc84a0402298f360u128, 0xa040010000001820e40d252e62046500u128, 0xa040010000001820e40d252e62046500u128, 0x00);
dec_test!(bid128_minnum_044, bid128_minnum, 0xa26d84695a88fd768d0d14ba3d2218b3u128, 0xeffae7ffffffffdd9800000000000000u128, 0xa26d84695a88fd768d0d14ba3d2218b3u128, 0x00);
dec_test!(bid128_minnum_045, bid128_minnum, 0xa3f162b44e10e5210402042201084014u128, 0xbf676edce3c6ece80410400000000000u128, 0xbf676edce3c6ece80410400000000000u128, 0x00);
dec_test!(bid128_minnum_046, bid128_minnum, 0xbd068006408102098169fde394089040u128, 0x801415810d030f522845a5229800c0c2u128, 0xbd068006408102098169fde394089040u128, 0x00);
dec_test!(bid128_minnum_047, bid128_minnum, 0xe7eff4f3b9f5f53f001000045c085002u128, 0x577dfca9df2e3ffd015810910100081au128, 0x9fbe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_048, bid128_minnum, 0xe9ffffff7fdefd3d0002000200000000u128, 0x7dfdeffd6cfafffe0000000000000000u128, 0xa7fe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_049, bid128_minnum, 0xfde79787ce96db86b57efff7ffe5fb7fu128, 0xfc30a1e8452c7bf94b3ea385d72b4677u128, 0xfc001787ce96db86b57efff7ffe5fb7fu128, 0x00);
dec_test!(bid128_minnum_050, bid128_minnum, 0xfdef778e7defeeff0004129008282120u128, 0xfffffffffcefffbf01000c1100022800u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_minnum_051, bid128_minnum, "-Infinity"                           , "-0"                                  , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_052, bid128_minnum, "-Infinity"                           , "-Infinity"                           , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_053, bid128_minnum, "Infinity"                            , "QNaN"                                , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_054, bid128_minnum, "-Infinity"                           , "QNaN"                                , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_055, bid128_minnum, "QNaN"                                , "0"                                   , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_056, bid128_minnum, "SNaN"                                , "0"                                   , 0x7c000000000000000000000000000000u128, 0x01);
