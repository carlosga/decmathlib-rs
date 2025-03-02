/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

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

dec_test!(bid128_minnum_057, bid128_minnum, 0x59dbb75d7734cd9e1234567890123456u128   , 0x59dbb75d7734cd9e1234567890123457u128   , 0x59dbb75d7734cd9e1234567890123456u128, 0x00);
dec_test!(bid128_minnum_058, bid128_minnum, 0x59dbb75d7734cd9e1234567890123457u128   , 0x59dbb75d7734cd9e1234567890123456u128   , 0x59dbb75d7734cd9e1234567890123456u128, 0x00);
dec_test!(bid128_minnum_059, bid128_minnum, 0x59dbb75d7734cd9e1234567890123456u128   , 0x59dbb75d7734cd9e1234567890123456u128   , 0x59dbb75d7734cd9e1234567890123456u128, 0x00);
dec_test!(bid128_minnum_060, bid128_minnum, "1.11e100"                               , "111e98"                                 , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_061, bid128_minnum, "1.11e100"                               , "111000e95"                              , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_062, bid128_minnum, "1.11e100"                               , "111000000e92"                           , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_063, bid128_minnum, "1.11e100"                               , "111000000000e89"                        , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_064, bid128_minnum, "1.11e100"                               , "111000000000000e86"                     , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_065, bid128_minnum, "1.11e100"                               , "111000000000000000e83"                  , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_066, bid128_minnum, "1.11e100"                               , "111000000000000000000e80"               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_067, bid128_minnum, "1.11e100"                               , "111000000000000000000000000000e71"      , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_068, bid128_minnum, "1.11e100"                               , "11100000000000000000000000000000000e66" , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_069, bid128_minnum, "1.11e100"                               , "111e100"                                , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_070, bid128_minnum, "1.11e100"                               , "111000e100"                             , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_071, bid128_minnum, "1.11e100"                               , "111000000e100"                          , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_072, bid128_minnum, "1.11e100"                               , "111000000000e100"                       , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_073, bid128_minnum, "1.11e100"                               , "111000000000000e100"                    , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_074, bid128_minnum, "1.11e100"                               , "111000000000000000e100"                 , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_075, bid128_minnum, "1.11e100"                               , "111000000000000000000e100"              , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_076, bid128_minnum, "1.11e100"                               , "111000000000000000000000000000e100"     , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_077, bid128_minnum, "1.11e100"                               , "11100000000000000000000000000000000e100", 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_078, bid128_minnum, "111e100"                                , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_079, bid128_minnum, "111000e100"                             , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_080, bid128_minnum, "111000000e100"                          , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_081, bid128_minnum, "111000000000e100"                       , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_082, bid128_minnum, "111000000000000e100"                    , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_083, bid128_minnum, "111000000000000000e100"                 , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_084, bid128_minnum, "111000000000000000000e100"              , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_085, bid128_minnum, "111000000000000000000000000000e100"     , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_086, bid128_minnum, "11100000000000000000000000000000000e100", "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_087, bid128_minnum, "1.11e100"                               , "111e98"                                 , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_088, bid128_minnum, "1.11e100"                               , "111001e95"                              , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_089, bid128_minnum, "1.11e100"                               , "111000001e92"                           , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_090, bid128_minnum, "1.11e100"                               , "111000000001e89"                        , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_091, bid128_minnum, "1.11e100"                               , "111000000000001e86"                     , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_092, bid128_minnum, "1.11e100"                               , "111000000000000001e83"                  , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_093, bid128_minnum, "1.11e100"                               , "111000000000000000001e80"               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_094, bid128_minnum, "1.11e100"                               , "111000000000000000000000000001e71"      , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_095, bid128_minnum, "1.11e100"                               , "11100000000000000000000000000000001e66" , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_096, bid128_minnum, "1.11e101"                               , "111e98"                                 , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_097, bid128_minnum, "1.11e101"                               , "111001e95"                              , 0x30fe000000000000000000000001b199u128, 0x00);
dec_test!(bid128_minnum_098, bid128_minnum, "1.11e101"                               , "111000001e92"                           , 0x30f800000000000000000000069db9c1u128, 0x00);
dec_test!(bid128_minnum_099, bid128_minnum, "1.11e101"                               , "111000000001e89"                        , 0x30f200000000000000000019d81d9601u128, 0x00);
dec_test!(bid128_minnum_100, bid128_minnum, "1.11e101"                               , "111000000000001e86"                     , 0x30ec000000000000000064f43391f001u128, 0x00);
dec_test!(bid128_minnum_101, bid128_minnum, "1.11e101"                               , "111000000000000001e83"                  , 0x30e6000000000000018a59e972118001u128, 0x00);
dec_test!(bid128_minnum_102, bid128_minnum, "1.11e101"                               , "111000000000000000001e80"               , 0x30e0000000000006046f37e5945c0001u128, 0x00);
dec_test!(bid128_minnum_103, bid128_minnum, "1.11e101"                               , "111000000000000000000000000001e71"      , 0x30ce000166a90c494b679a6898000001u128, 0x00);
dec_test!(bid128_minnum_104, bid128_minnum, "1.11e101"                               , "11100000000000000000000000000000001e66" , 0x30c636ba2b6fef117eff95b180000000u128, 0x00);
dec_test!(bid128_minnum_105, bid128_minnum, "111e98"                                 , "1.11e101"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_106, bid128_minnum, "111001e95"                              , "1.11e101"                               , 0x30fe000000000000000000000001b199u128, 0x00);
dec_test!(bid128_minnum_107, bid128_minnum, "111000001e92"                           , "1.11e101"                               , 0x30f800000000000000000000069db9c1u128, 0x00);
dec_test!(bid128_minnum_108, bid128_minnum, "111000000001e89"                        , "1.11e101"                               , 0x30f200000000000000000019d81d9601u128, 0x00);
dec_test!(bid128_minnum_109, bid128_minnum, "111000000000001e86"                     , "1.11e101"                               , 0x30ec000000000000000064f43391f001u128, 0x00);
dec_test!(bid128_minnum_110, bid128_minnum, "111000000000000001e83"                  , "1.11e101"                               , 0x30e6000000000000018a59e972118001u128, 0x00);
dec_test!(bid128_minnum_111, bid128_minnum, "111000000000000000001e80"               , "1.11e101"                               , 0x30e0000000000006046f37e5945c0001u128, 0x00);
dec_test!(bid128_minnum_112, bid128_minnum, "111000000000000000000000000001e71"      , "1.11e101"                               , 0x30ce000166a90c494b679a6898000001u128, 0x00);
dec_test!(bid128_minnum_113, bid128_minnum, "11100000000000000000000000000000001e66" , "1.11e101"                               , 0x30c636ba2b6fef117eff95b180000000u128, 0x00);
dec_test!(bid128_minnum_114, bid128_minnum, "111e98"                                 , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_115, bid128_minnum, "111001e95"                              , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_116, bid128_minnum, "111000001e92"                           , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_117, bid128_minnum, "111000000001e89"                        , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_118, bid128_minnum, "111000000000001e86"                     , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_119, bid128_minnum, "111000000000000001e83"                  , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_120, bid128_minnum, "111000000000000000001e80"               , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_121, bid128_minnum, "111000000000000000000000000001e71"      , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_122, bid128_minnum, "11100000000000000000000000000000001e66" , "1.11e100"                               , 0x3104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_123, bid128_minnum, "-1.11e100"                              , "111e98"                                 , 0xb104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_124, bid128_minnum, "1.11e100"                               , "-111e98"                                , 0xb104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_125, bid128_minnum, "-1.11e100"                              , "-111e98"                                , 0xb104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_126, bid128_minnum, "-1.11e100"                              , "111e98"                                 , 0xb104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_127, bid128_minnum, "1.11e100"                               , "-111e98"                                , 0xb104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_128, bid128_minnum, "-1.11e100"                              , "-111e98"                                , 0xb104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_129, bid128_minnum, "-1.11e100"                              , "-111000000000000000000e80"              , 0xb0e0000000000006046f37e5945c0000u128, 0x00);
dec_test!(bid128_minnum_130, bid128_minnum, "-1.11e100"                              , "-11100000000000000000000000000000000e66", 0xb0c636ba2b6fef117eff95b180000000u128, 0x00);
dec_test!(bid128_minnum_131, bid128_minnum, "-1.11e100"                              , "111000000000000000000e80"               , 0xb104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_132, bid128_minnum, "-1.11e100"                              , "11100000000000000000000000000000000e66" , 0xb104000000000000000000000000006fu128, 0x00);
dec_test!(bid128_minnum_133, bid128_minnum, "1.11e100"                               , "-111000000000000000000e80"              , 0xb0e0000000000006046f37e5945c0000u128, 0x00);
dec_test!(bid128_minnum_134, bid128_minnum, "1.11e100"                               , "-11100000000000000000000000000000000e66", 0xb0c636ba2b6fef117eff95b180000000u128, 0x00);
dec_test!(bid128_minnum_135, bid128_minnum, 0x30540000000000000000000000000000u128   , "1.0e11"                                 , 0x30540000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_136, bid128_minnum, 0x30540000000000000000000000000000u128   , "-1.0e11"                                , 0xb054000000000000000000000000000au128, 0x00);
dec_test!(bid128_minnum_137, bid128_minnum, 0xb0540000000000000000000000000000u128   , "-1.0e11"                                , 0xb054000000000000000000000000000au128, 0x00);
dec_test!(bid128_minnum_138, bid128_minnum, 0xb0540000000000000000000000000000u128   , "1.0e11"                                 , 0xb0540000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_139, bid128_minnum, "1.0e11"                                 , 0x30540000000000000000000000000000u128   , 0x30540000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_140, bid128_minnum, "-1.0e11"                                , 0x30540000000000000000000000000000u128   , 0xb054000000000000000000000000000au128, 0x00);
dec_test!(bid128_minnum_141, bid128_minnum, "-1.0e11"                                , 0xb0540000000000000000000000000000u128   , 0xb054000000000000000000000000000au128, 0x00);
dec_test!(bid128_minnum_142, bid128_minnum, "1.0e11"                                 , 0xb0540000000000000000000000000000u128   , 0xb0540000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_143, bid128_minnum, 0x7c00314dc6448d9338c15b0a00000000u128   , 0xfc00314dc6448d9338c15b0a00000000u128   , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_144, bid128_minnum, 0x7c00314dc6448d9338c15b0a00000000u128   , 0x7c00314dc6448d9338c15b09ffffffffu128   , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_145, bid128_minnum, 0x7c00314dc6448d9338c15b09ffffffffu128   , 0x7c00314dc6448d9338c15b0a00000000u128   , 0x7c00314dc6448d9338c15b09ffffffffu128, 0x00);
dec_test!(bid128_minnum_146, bid128_minnum, 0x7c00314dc6448d9338c15b09ffffffffu128   , 0x7c00314dc6448d9338c15b09fffffffeu128   , 0x7c00314dc6448d9338c15b09ffffffffu128, 0x00);
dec_test!(bid128_minnum_147, bid128_minnum, 0x69dbb75d7734cd9e1234567890123456u128   , 0x7c00314dc6448d9338c15b0a00000000u128   , 0x276e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_148, bid128_minnum, 0x69dbb75d7734cd9e1234567890123456u128   , 0x7c00314dc6448d9338c15b09fffffffeu128   , 0x276e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_149, bid128_minnum, 0x59dbb75d7734cd9e1234567890123456u128   , 0x7c00314dc6448d9338c15b09fffffffeu128   , 0x59dbb75d7734cd9e1234567890123456u128, 0x00);
dec_test!(bid128_minnum_150, bid128_minnum, 0x59dbb75d7734cd9e1234567890123456u128   , 0x7c00314dc6448d9338c15b0a00000000u128   , 0x59dbb75d7734cd9e1234567890123456u128, 0x00);
dec_test!(bid128_minnum_151, bid128_minnum, 0x7c00314dc6448d9338c15b0a00000000u128   , 0x69dbb75d7734cd9e1234567890123456u128   , 0x276e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_152, bid128_minnum, 0x7c00314dc6448d9338c15b09fffffffeu128   , 0x69dbb75d7734cd9e1234567890123456u128   , 0x276e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_153, bid128_minnum, 0x7c00314dc6448d9338c15b09fffffffeu128   , 0x59dbb75d7734cd9e1234567890123456u128   , 0x59dbb75d7734cd9e1234567890123456u128, 0x00);
dec_test!(bid128_minnum_154, bid128_minnum, 0x7c00314dc6448d9338c15b0a00000000u128   , 0x59dbb75d7734cd9e1234567890123456u128   , 0x59dbb75d7734cd9e1234567890123456u128, 0x00);