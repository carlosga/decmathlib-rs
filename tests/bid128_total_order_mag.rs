/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_total_order_mag_001, bid128_total_order_mag, 0x0000000000000000ffffffffffffffffu128,  0x0000000000000000ffffffffffffffffu128, true);
dec_test!(bid128_total_order_mag_002, bid128_total_order_mag, 0x0001ed09bead87c0378d8e62ffffffffu128,  0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_total_order_mag_003, bid128_total_order_mag, 0x0001ed09bead87c0378d8e62ffffffffu128,  0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_total_order_mag_004, bid128_total_order_mag, 0x0001ed09bead87c0378d8e62ffffffffu128,  0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_total_order_mag_005, bid128_total_order_mag, 0x0001ed09bead87c0378d8e62ffffffffu128,  0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_total_order_mag_006, bid128_total_order_mag, 0x0001ed09bead87c0378d8e64ffffffffu128,  0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_total_order_mag_007, bid128_total_order_mag, 0x0001ed09bead87c0378d8e64ffffffffu128,  0x0001ed09bead87c0378d8e64ffffffffu128, true);
dec_test!(bid128_total_order_mag_008, bid128_total_order_mag, 0x0001ed09bead87c0378d8e64ffffffffu128,  0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_total_order_mag_009, bid128_total_order_mag, 0x0001ed09bead87c0378d8e64ffffffffu128,  0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_total_order_mag_010, bid128_total_order_mag, "0"                                   , "-0"                                   , true);
dec_test!(bid128_total_order_mag_011, bid128_total_order_mag, 0x0028314020110011e6ac6b687fdf8799u128,  0x0000508709422280cffbffd796feef77u128, false);
dec_test!(bid128_total_order_mag_012, bid128_total_order_mag, 0x003fbc5db19556c47f5c8cd6a1b1f29fu128,  0x007c01b21f5380be1a0a082210006c07u128, true);
dec_test!(bid128_total_order_mag_013, bid128_total_order_mag, 0x0210a24480458100581bf32f8507f4cbu128,  0x9dce33bbbd6a92f1302ddfeacd6c884eu128, true);
dec_test!(bid128_total_order_mag_014, bid128_total_order_mag, 0x088dda260a6cc577fffeffffffeffffdu128,  0x0002000003814020fffbdb57fabe5e73u128, false);
dec_test!(bid128_total_order_mag_015, bid128_total_order_mag, "1.0"                                 ,    "1"                                 , true);
dec_test!(bid128_total_order_mag_016, bid128_total_order_mag,   "1"                                 ,  "1.0"                                 , false);
dec_test!(bid128_total_order_mag_017, bid128_total_order_mag, 0x276a0000000000000000000000000000u128,  0xbe5753c0543a6cf9c8d5b1e8301dadecu128, true);
dec_test!(bid128_total_order_mag_018, bid128_total_order_mag, 0x2a129da1c2c8ab57c05bf2f3a3dd6c79u128,  0x24b76ab1cf5b1ccf7bdb02f454ec5262u128, false);
dec_test!(bid128_total_order_mag_019, bid128_total_order_mag, 0x35500000000000000000000000000000u128,  0xd47d8d9d12923a2fdf4548763768b5c4u128, true);
dec_test!(bid128_total_order_mag_020, bid128_total_order_mag, 0x4000280084800001adeaa717a9cb1212u128,  0xbfdba7ce3f6fefbdb81fe6de106e659du128, false);
dec_test!(bid128_total_order_mag_021, bid128_total_order_mag, 0x4dd20000000000000000000000000000u128,  0xce0c0000000000000000000000000000u128, true);
dec_test!(bid128_total_order_mag_022, bid128_total_order_mag, 0x5169449868718a6ac0159130231f8508u128,  0xd1ffd1569cfa3483e96b8cbb20bf3378u128, true);
dec_test!(bid128_total_order_mag_023, bid128_total_order_mag, 0x5b520000000000000000000000000000u128,  0x7e00147c43f054b1ad3a93204a1da3d9u128, true);
dec_test!(bid128_total_order_mag_024, bid128_total_order_mag, 0x7c003fffffffffff38c15b08ffffffffu128,  0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_total_order_mag_025, bid128_total_order_mag, 0x7c003fffffffffff38c15b08ffffffffu128,  0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_total_order_mag_026, bid128_total_order_mag, 0x7c003fffffffffff38c15b08ffffffffu128,  0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_total_order_mag_027, bid128_total_order_mag, 0x7c003fffffffffff38c15b08ffffffffu128,  0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_total_order_mag_028, bid128_total_order_mag, 0x7c003fffffffffff38c15b0affffffffu128,  0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_total_order_mag_029, bid128_total_order_mag, 0x7c003fffffffffff38c15b0affffffffu128,  0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_total_order_mag_030, bid128_total_order_mag, 0x7c003fffffffffff38c15b0affffffffu128,  0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_total_order_mag_031, bid128_total_order_mag, 0x7c003fffffffffff38c15b0affffffffu128,  0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_total_order_mag_032, bid128_total_order_mag, 0x82190be94470063f10401000c0508100u128,  0x82247014e98cc58c01162860c1040200u128, true);
dec_test!(bid128_total_order_mag_033, bid128_total_order_mag, 0x86269a7979ca2d874978b5044e274c0au128,  0xd5b8b907c86dbdcb94841026237637edu128, true);
dec_test!(bid128_total_order_mag_034, bid128_total_order_mag, 0x89052df8fc1fffe8b97605d51a4d31d8u128,  0xca41a994068522dfcc95eb1728935769u128, true);
dec_test!(bid128_total_order_mag_035, bid128_total_order_mag, 0xaffafda2ee0927d440a387e5f875866bu128,  0x78000000000000000000000000000000u128, true);
dec_test!(bid128_total_order_mag_036, bid128_total_order_mag, 0xbc161ffd96c45d0d615042fd53173352u128,  0xeb7de35af4f967bbfdff5dfb6deffdfeu128, false);
dec_test!(bid128_total_order_mag_037, bid128_total_order_mag, 0xbe53197ffac646e2ef8ba7eecb1f798bu128,  0xfadc2fd79bc3de5b7ef77ff5f7f78b7fu128, true);
dec_test!(bid128_total_order_mag_038, bid128_total_order_mag, 0xcd648f28071ceffd84e53b0e64b782fbu128,  0xd5d831cbf07184032fe43caf13be00bbu128, true);
dec_test!(bid128_total_order_mag_039, bid128_total_order_mag, 0xd75ffb7b77efff1db1440e2a480d018au128,  0xf3151396fee635a002bb952491cb9fbbu128, false);
dec_test!(bid128_total_order_mag_040, bid128_total_order_mag, 0xda9eb397b9a8a0e7667c064baeeb9399u128,  0xa168cfa21b0d15db8b8d052f84264e13u128, false);
dec_test!(bid128_total_order_mag_041, bid128_total_order_mag, 0xedf53ff1f7fcf3a51000821c0b10266cu128,  0xff3ded162f7635f70818580950004148u128, true);
dec_test!(bid128_total_order_mag_042, bid128_total_order_mag, 0xeeaf6da8b4958a353e6b3e71cdf8871cu128,  0x42e6362eb8e491c59901200038910e58u128, true);
dec_test!(bid128_total_order_mag_043, bid128_total_order_mag, 0xefffeffbf7fffffff7ef5ffeddf9defau128,  0x3ffffff7fffff7ff84084b013e452495u128, true);
dec_test!(bid128_total_order_mag_044, bid128_total_order_mag, 0xfa56994806d37cc85af4e64953940c4fu128,  0x5e0e5ebce21eb6bbfeefdefd3b6ff3efu128, false);
dec_test!(bid128_total_order_mag_045, bid128_total_order_mag, 0xfc002aa35b8e728575fa419351eef6dcu128,  0xb22837a2d039a8bf438738bbbbc1eb45u128, false);
dec_test!(bid128_total_order_mag_046, bid128_total_order_mag, 0xfef7fefffafffffffffffeffeffedaffu128,  0x03de304a7b96c2923f63a7c7fffb6fdfu128, false);
dec_test!(bid128_total_order_mag_047, bid128_total_order_mag, 0xff3af6697ebbffff3ed6db7d1db085eeu128,  0x7c5f158e134285fbcb700965f8b4ea03u128, true);
dec_test!(bid128_total_order_mag_048, bid128_total_order_mag, 0xff3fdffcfeffdbbecbfe75e3bded5b74u128,  0xfe4feef72fc7fff61ecc263683f984d3u128, true);
dec_test!(bid128_total_order_mag_049, bid128_total_order_mag, 0xfffad7edfe7f33360900100008013144u128,  0xffffffffffffffff4fb1ce6acb373b57u128, false);
dec_test!(bid128_total_order_mag_050, bid128_total_order_mag, 0xffffffffffffffff3008004900200008u128,  0xfffeabffffffffff95a2105266a63869u128, true);
dec_test!(bid128_total_order_mag_051, bid128_total_order_mag, "Infinity"                            ,  "SNaN"                                , true);
dec_test!(bid128_total_order_mag_052, bid128_total_order_mag,     "QNaN"                            , "-0"                                   , false);
dec_test!(bid128_total_order_mag_053, bid128_total_order_mag,     "QNaN"                            , "-Infinity"                            , false);
dec_test!(bid128_total_order_mag_054, bid128_total_order_mag,     "SNaN"                            , "-0"                                   , false);
dec_test!(bid128_total_order_mag_055, bid128_total_order_mag,     "SNaN"                            ,  "0"                                   , false);
// TODO: Review
// dec_test!(bid128_total_order_mag_056, bid128_total_order_mag,     "SNaN"                            , "-2.3565784324E0"                      , false);
dec_test!(bid128_total_order_mag_057, bid128_total_order_mag,     "SNaN"                            , "-Infinity"                            , false);