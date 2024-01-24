/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_same_quantum_001, bid128_same_quantum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_same_quantum_002, bid128_same_quantum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true);
dec_test!(bid128_same_quantum_003, bid128_same_quantum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false);
dec_test!(bid128_same_quantum_004, bid128_same_quantum, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false);
dec_test!(bid128_same_quantum_005, bid128_same_quantum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, true);
dec_test!(bid128_same_quantum_006, bid128_same_quantum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, true);
dec_test!(bid128_same_quantum_007, bid128_same_quantum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, false);
dec_test!(bid128_same_quantum_008, bid128_same_quantum, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, false);
dec_test!(bid128_same_quantum_009, bid128_same_quantum, "0"                                   , "-0"                                  , true);
dec_test!(bid128_same_quantum_010, bid128_same_quantum, "0"                                   ,  "0"                                  , true);
dec_test!(bid128_same_quantum_011, bid128_same_quantum, 0x051c74281ad2cdb5f119577a273dfe79u128, 0xdf176c57b133950366495a5a3b54eda9u128, false);
dec_test!(bid128_same_quantum_012, bid128_same_quantum, 0x08d06098f22341df7f5d008240ff8dd0u128, 0x3cecd8ccc6f2442cd093bdbd23f43799u128, false);
dec_test!(bid128_same_quantum_013, bid128_same_quantum, "-0"                                  , "SNaN"                                , false);
dec_test!(bid128_same_quantum_014, bid128_same_quantum,  "0"                                  , "SNaN"                                , false);
dec_test!(bid128_same_quantum_015, bid128_same_quantum, 0x17881f4ee218bad982f3e7c1878d47efu128, 0x1f720000000000000000000000000000u128, false);
dec_test!(bid128_same_quantum_016, bid128_same_quantum, 0x3ccfe12d62401d521b5f04ac64412a34u128, 0x441e0000000000000000000000000000u128, false);
dec_test!(bid128_same_quantum_017, bid128_same_quantum, 0x536290ca09fb8b8652123b3cdeaf6c90u128, 0xbb8f2d13f90ccbb478d152167e7c5279u128, false);
dec_test!(bid128_same_quantum_018, bid128_same_quantum, 0x6ff7b7fff73fd3aafff7cdfff6ff5bcfu128, 0xefb9bc0e0d423190ee325a636805251fu128, false);
dec_test!(bid128_same_quantum_019, bid128_same_quantum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_same_quantum_020, bid128_same_quantum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_same_quantum_021, bid128_same_quantum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_same_quantum_022, bid128_same_quantum, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_same_quantum_023, bid128_same_quantum, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, false);
dec_test!(bid128_same_quantum_024, bid128_same_quantum, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, false);
dec_test!(bid128_same_quantum_025, bid128_same_quantum, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, true);
dec_test!(bid128_same_quantum_026, bid128_same_quantum, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, true);
dec_test!(bid128_same_quantum_027, bid128_same_quantum, 0x7f5925b54c3d9dd6c4414022fb251949u128, 0xa1d68282f8c992545101110280002040u128, false);
dec_test!(bid128_same_quantum_028, bid128_same_quantum, 0x8cd7b9b8bc4f202ad2d914b2ed4bd0c4u128, 0x09654b47455a90b253a2421194503a32u128, false);
dec_test!(bid128_same_quantum_029, bid128_same_quantum, 0x8d8c53ded1fe4b28b83e18a73f3dcc9fu128, 0x98624a824286a1b55dd6844c9d9f588bu128, false);
dec_test!(bid128_same_quantum_030, bid128_same_quantum, 0x9165bf5333bf3231ce13539fd9473752u128, 0xf9aeaeedfaed76737cf58ad73712ed66u128, false);
dec_test!(bid128_same_quantum_031, bid128_same_quantum, 0xc5eac98dc305e7e23d40f5b48e2e469cu128, 0x35f8e672f8527b48b69adb8222fcb612u128, false);
dec_test!(bid128_same_quantum_032, bid128_same_quantum, 0xc9d8803897920d2805f51fbb0724ac1cu128, 0xa26c23a0e28474fa27057eab64dc55d7u128, false);
dec_test!(bid128_same_quantum_033, bid128_same_quantum, 0xce7ab182e60ef9f6e7878a50950422c2u128, 0x196109cfaa417d79727b4abe4cb75a32u128, false);
dec_test!(bid128_same_quantum_034, bid128_same_quantum, 0xd4ddaae62ab05fdaea8d6cf791bf223du128, 0xd9d55e041a032419a0c2a34f3ee1bed7u128, false);
dec_test!(bid128_same_quantum_035, bid128_same_quantum, 0xe256138caf1cd6cffbffa5ffdfffdfffu128, 0x2000000000001a203feffa463c7b507fu128, false);
dec_test!(bid128_same_quantum_036, bid128_same_quantum, "-Infinity"                           ,       "-0"                            , false);
dec_test!(bid128_same_quantum_037, bid128_same_quantum, "-Infinity"                           ,        "0"                            , false);
dec_test!(bid128_same_quantum_038, bid128_same_quantum,  "Infinity"                           , "Infinity"                            , true);
dec_test!(bid128_same_quantum_039, bid128_same_quantum, "-Infinity"                           ,     "QNaN"                            , false);
dec_test!(bid128_same_quantum_040, bid128_same_quantum,      "SNaN"                           , "Infinity"                            , false);
