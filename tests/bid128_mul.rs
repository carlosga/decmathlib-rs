/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_mul_001, bid128_mul, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_002, bid128_mul, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_003, bid128_mul, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_004, bid128_mul, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_005, bid128_mul, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_006, bid128_mul, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_007, bid128_mul, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_008, bid128_mul, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_009, bid128_mul, 0, 0x006a563a06a04c0aef3a3c23e2d36044u128, 0x07dce5d3ce823c76e4dfa77086711c61u128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_010, bid128_mul, 0, 0x03ac0000000000000000000000000000u128, 0x51ab2723d21d4c916b5f75f600dcbbe2u128, 0x25160000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_011, bid128_mul, 0, 0x0aabe0d152074dfb112a9edce0e067c9u128, 0x8d13a79968c6b31ca569c623cd4fc5e6u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_012, bid128_mul, 0, 0x0d12f8e176524a70cc5defb99c48f4beu128, 0x10ed50c9b1ac645edef7136ee58dd67bu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_013, bid128_mul, 0, 0x0f15be3dcaa7dfc995b386b7eb810dc4u128, 0x039d2ae0bcc67eee830d4ecc5e2b577eu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_014, bid128_mul, 0, "-0"                                  , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_015, bid128_mul, 0,  "0"                                  , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
// dec_test!(bid128_mul_016, bid128_mul, 0, -1000101110110110011.1E0 -972.2639553368233236E0 [3027df695f2b352e7e7a36cc2c3b9415] , 0x20);
// dec_test!(bid128_mul_017, bid128_mul, 0, -110011000.010110111E0 -47356549229256.67778E0 [302900dc2e26fe4ef5b7b8b43afa0f7a] , 0x20);
dec_test!(bid128_mul_018, bid128_mul, 0, 0x133152f5bc6f288c89983ce0c603cf36u128, 0x904132dac1cc5c56519f7db3adbbbfe8u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_019, bid128_mul, 0, 0x150a2e0d6728de4e95595bd43d654036u128, 0xc47aef17e9919a5569aaaf503275e8f4u128, 0xa986df536afc692da7504914a519abd5u128, 0x20);
dec_test!(bid128_mul_020, bid128_mul, 0, 0x26493373d8eb14f4ef1a0b294d6f0687u128, 0x5ffdffbfe34e8bfdf3c1551d20d58048u128, 0x56040000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_021, bid128_mul, 0, 0x2f427234502eb9d5ef1e471739109d17u128, 0x88371ed9f0bc3751ff9136d91da9f3d9u128, 0x877c4271caf1beb185e97c4d7d084dbau128, 0x20);
dec_test!(bid128_mul_022, bid128_mul, 0, 0x31155f3baa6a49f68431049cfbdab6c9u128, 0x5054e9c9e2631401d383b4964f87e05eu128, 0x516ca68c2d79d4b93e54601d1a8780b6u128, 0x20);
dec_test!(bid128_mul_023, bid128_mul, 0, 0x40e20000000000000000000000000000u128, 0x4db862ab3b7a3a4a872d344e3263ff63u128, 0x5e5a0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_024, bid128_mul, 0, 0x46f8c66750b4255ee5f50c08bb2dec23u128, 0xb0739679b9d6a42fd261a9d744b1adb8u128, 0xc76ea391e575aec63c18ab5068cb6cbdu128, 0x20);
dec_test!(bid128_mul_025, bid128_mul, 0, 0x4a660000000000000000000000000000u128, 0xdd740000000000000000000000000000u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_026, bid128_mul, 0, 0x4db6022c6ac90c122f31f9e027c0cb3cu128, 0x5e5a3663fad59572b27d2f9994f1ed2eu128, 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_mul_027, bid128_mul, 0, 0x530919f1e09dd59f9b951f8183574016u128, 0x169eef000f7a5d7761d331242cf44747u128, 0x39aa88ac3b2436ed02af1c1b7c6e9d9fu128, 0x20);
dec_test!(bid128_mul_028, bid128_mul, 0, 0x565a0000000000000000000000000000u128, 0xd91854823ba2af952bb97cf8f2d4c083u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_029, bid128_mul, 0, 0x57125da9a5b8c9a339046f20cf22c152u128, 0x3f1f0ae34f231d874f1ff6ae8d5b03c7u128, 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_mul_030, bid128_mul, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_031, bid128_mul, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_032, bid128_mul, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_033, bid128_mul, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_034, bid128_mul, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_035, bid128_mul, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_036, bid128_mul, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_037, bid128_mul, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_038, bid128_mul, 0, 0x807368e46255ca5d588cfe473c254563u128, 0x1f59d74176b6df4a76db4cfb2788b807u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_039, bid128_mul, 0, 0x82a4531e6dfac7247b2af5f3554e7e83u128, 0xb99d5e676c84d32e12a6af7cb2f61a3cu128, 0x0c443b12a8dbc75006fa90a2aa727872u128, 0x20);
dec_test!(bid128_mul_040, bid128_mul, 0, 0x8ec7a09091dc32b2bce36c840a1e3b09u128, 0x89b040fc053bf28c09fc7bd2902fe28du128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_041, bid128_mul, 0, 0x93b156741e3961009422ad67ad806698u128, 0x99f7d2c00d96a0ec38b3e16736b0c3fau128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_042, bid128_mul, 0, 0x94c7d1f889198a96842e29d55cb821c0u128, 0x588efd2cd70a1fde9da7dc0ddb569cf9u128, 0xbd58ef46b08263af9b9e88771ea06911u128, 0x20);
dec_test!(bid128_mul_043, bid128_mul, 0, 0x95a56f957b5afdf78e550ab858bed620u128, 0x0f8f316b321a1c2d9c72537f9bed8d45u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_044, bid128_mul, 0, 0x970c0000000000000000000000000000u128, 0xd7d9186750ca22882d23d2cb53e46703u128, 0x3ea40000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_045, bid128_mul, 0, 0x9c727f7fae575afc5334029e9553a602u128, 0xa5600000000000000000000000000000u128, 0x11920000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_046, bid128_mul, 0, 0x9f8ca44956311f864a91e13c61ad8be4u128, 0x129fd783cd14dbf5f6d01c8e3d4956e6u128, 0x822e9d1d5ab22c4d8a4fbfa6c7d1f6c6u128, 0x20);
dec_test!(bid128_mul_047, bid128_mul, 0, 0xaaf5c59f39cce565b3f956ea0bb0026eu128, 0xcd5fd6c6a1d24cc0f4a7617f943b3b70u128, 0x4857b123bb020f02a66c16f22ea8ade6u128, 0x20);
dec_test!(bid128_mul_048, bid128_mul, 0, 0xae7f324e2487cef3d04046b7603dbc92u128, 0x78000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_049, bid128_mul, 0, 0xbabfff55ea0a5aa6ad4249db9b4b1dbcu128, 0x9008a18408344020fff7d5d8f197db7au128, 0x1a860000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_050, bid128_mul, 0, 0xbdea1d8a96ef0987ed743f875d54d67fu128, 0x82a53be053aa524e95c2f33fc529cb60u128, 0x1090bd437c2250979afeb07cd0f09dd3u128, 0x20);
dec_test!(bid128_mul_051, bid128_mul, 0, 0xd261e79d08c12070d5c241f3d7f02504u128, 0xd0ec877840c4d4963b42037ee877ec03u128, 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_mul_052, bid128_mul, 0, 0xd4c641b3a7a0d36f7c58e0915a454073u128, 0xaace0000000000000000000000000000u128, 0x4f540000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_053, bid128_mul, 0, 0xd6809c5dae6efdc492ee1f5fcc48dc7du128, 0x7e003009107549d6ebea61d422a3f73cu128, 0x7c003009107549d6ebea61d422a3f73cu128, 0x01);
dec_test!(bid128_mul_054, bid128_mul, 0, 0xd6fb2c0a9edd8b13a888a98ad171d726u128, 0x7e0015e5379f538dac0b835438210c04u128, 0x7c0015e5379f538dac0b835438210c04u128, 0x01);
dec_test!(bid128_mul_055, bid128_mul, 0, 0xddfefff37ff3fffffdf7f9d21892719eu128, 0xf0896903e59e331553444a8769ee3c3au128, 0x5ffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_056, bid128_mul, 0, 0xf400db8a143020970000010010810001u128, 0xefc9687ca77e0eafffffffff73fefd7bu128, 0x5ee60000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_057, bid128_mul, 0, "Infinity"                            , "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_058, bid128_mul, 0, "Infinity"                            , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_059, bid128_mul, 0,     "QNaN"                            , "Infinity"                            , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_060, bid128_mul, 0,     "SNaN"                            , "0"                                   , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_mul_061, bid128_mul, 1, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_062, bid128_mul, 1, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_063, bid128_mul, 1, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_064, bid128_mul, 1, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_065, bid128_mul, 1, 0x006a563a06a04c0aef3a3c23e2d36044u128, 0x07dce5d3ce823c76e4dfa77086711c61u128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_066, bid128_mul, 1, 0x03ac0000000000000000000000000000u128, 0x51ab2723d21d4c916b5f75f600dcbbe2u128, 0x25160000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_067, bid128_mul, 1, 0x0aabe0d152074dfb112a9edce0e067c9u128, 0x8d13a79968c6b31ca569c623cd4fc5e6u128, 0x80000000000000000000000000000001u128, 0x30);
dec_test!(bid128_mul_068, bid128_mul, 1, 0x0d12f8e176524a70cc5defb99c48f4beu128, 0x10ed50c9b1ac645edef7136ee58dd67bu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_069, bid128_mul, 1, 0x0f15be3dcaa7dfc995b386b7eb810dc4u128, 0x039d2ae0bcc67eee830d4ecc5e2b577eu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_070, bid128_mul, 1, "-0"                                  , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_071, bid128_mul, 1,  "0"                                  , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
// dec_test!(bid128_mul_072, bid128_mul, 1, -1000101110110110011.1E0 -972.2639553368233236E0 [3027df695f2b352e7e7a36cc2c3b9414] , 0x20);
// dec_test!(bid128_mul_073, bid128_mul, 1, -110011000.010110111E0 -47356549229256.67778E0 [302900dc2e26fe4ef5b7b8b43afa0f79] , 0x20);
dec_test!(bid128_mul_074, bid128_mul, 1, 0x133152f5bc6f288c89983ce0c603cf36u128, 0x904132dac1cc5c56519f7db3adbbbfe8u128, 0x80000000000000000000000000000001u128, 0x30);
dec_test!(bid128_mul_075, bid128_mul, 1, 0x150a2e0d6728de4e95595bd43d654036u128, 0xc47aef17e9919a5569aaaf503275e8f4u128, 0xa986df536afc692da7504914a519abd5u128, 0x20);
dec_test!(bid128_mul_076, bid128_mul, 1, 0x31155f3baa6a49f68431049cfbdab6c9u128, 0x5054e9c9e2631401d383b4964f87e05eu128, 0x516ca68c2d79d4b93e54601d1a8780b5u128, 0x20);
dec_test!(bid128_mul_077, bid128_mul, 1, 0x40e20000000000000000000000000000u128, 0x4db862ab3b7a3a4a872d344e3263ff63u128, 0x5e5a0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_078, bid128_mul, 1, 0x46f8c66750b4255ee5f50c08bb2dec23u128, 0xb0739679b9d6a42fd261a9d744b1adb8u128, 0xc76ea391e575aec63c18ab5068cb6cbeu128, 0x20);
dec_test!(bid128_mul_079, bid128_mul, 1, 0x4a660000000000000000000000000000u128, 0xdd740000000000000000000000000000u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_080, bid128_mul, 1, 0x4db6022c6ac90c122f31f9e027c0cb3cu128, 0x5e5a3663fad59572b27d2f9994f1ed2eu128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x28);
dec_test!(bid128_mul_081, bid128_mul, 1, 0x530919f1e09dd59f9b951f8183574016u128, 0x169eef000f7a5d7761d331242cf44747u128, 0x39aa88ac3b2436ed02af1c1b7c6e9d9fu128, 0x20);
dec_test!(bid128_mul_082, bid128_mul, 1, 0x565a0000000000000000000000000000u128, 0xd91854823ba2af952bb97cf8f2d4c083u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_083, bid128_mul, 1, 0x57125da9a5b8c9a339046f20cf22c152u128, 0x3f1f0ae34f231d874f1ff6ae8d5b03c7u128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x28);
dec_test!(bid128_mul_084, bid128_mul, 1, 0x82a4531e6dfac7247b2af5f3554e7e83u128, 0xb99d5e676c84d32e12a6af7cb2f61a3cu128, 0x0c443b12a8dbc75006fa90a2aa727872u128, 0x20);
dec_test!(bid128_mul_085, bid128_mul, 1, 0x8ec7a09091dc32b2bce36c840a1e3b09u128, 0x89b040fc053bf28c09fc7bd2902fe28du128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_086, bid128_mul, 1, 0x93b156741e3961009422ad67ad806698u128, 0x99f7d2c00d96a0ec38b3e16736b0c3fau128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_087, bid128_mul, 1, 0x94c7d1f889198a96842e29d55cb821c0u128, 0x588efd2cd70a1fde9da7dc0ddb569cf9u128, 0xbd58ef46b08263af9b9e88771ea06911u128, 0x20);
dec_test!(bid128_mul_088, bid128_mul, 1, 0x95a56f957b5afdf78e550ab858bed620u128, 0x0f8f316b321a1c2d9c72537f9bed8d45u128, 0x80000000000000000000000000000001u128, 0x30);
dec_test!(bid128_mul_089, bid128_mul, 1, 0x970c0000000000000000000000000000u128, 0xd7d9186750ca22882d23d2cb53e46703u128, 0x3ea40000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_090, bid128_mul, 1, 0x9c727f7fae575afc5334029e9553a602u128, 0xa5600000000000000000000000000000u128, 0x11920000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_091, bid128_mul, 1, 0x9f8ca44956311f864a91e13c61ad8be4u128, 0x129fd783cd14dbf5f6d01c8e3d4956e6u128, 0x822e9d1d5ab22c4d8a4fbfa6c7d1f6c6u128, 0x20);
dec_test!(bid128_mul_092, bid128_mul, 1, 0xaaf5c59f39cce565b3f956ea0bb0026eu128, 0xcd5fd6c6a1d24cc0f4a7617f943b3b70u128, 0x4857b123bb020f02a66c16f22ea8ade6u128, 0x20);
dec_test!(bid128_mul_093, bid128_mul, 1, 0xae7f324e2487cef3d04046b7603dbc92u128, 0x78000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_094, bid128_mul, 1, 0xbdea1d8a96ef0987ed743f875d54d67fu128, 0x82a53be053aa524e95c2f33fc529cb60u128, 0x1090bd437c2250979afeb07cd0f09dd3u128, 0x20);
dec_test!(bid128_mul_095, bid128_mul, 1, 0xd261e79d08c12070d5c241f3d7f02504u128, 0xd0ec877840c4d4963b42037ee877ec03u128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x28);
dec_test!(bid128_mul_096, bid128_mul, 1, 0xd4c641b3a7a0d36f7c58e0915a454073u128, 0xaace0000000000000000000000000000u128, 0x4f540000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_097, bid128_mul, 1, 0xd6809c5dae6efdc492ee1f5fcc48dc7du128, 0x7e003009107549d6ebea61d422a3f73cu128, 0x7c003009107549d6ebea61d422a3f73cu128, 0x01);
dec_test!(bid128_mul_098, bid128_mul, 1, 0xd6fb2c0a9edd8b13a888a98ad171d726u128, 0x7e0015e5379f538dac0b835438210c04u128, 0x7c0015e5379f538dac0b835438210c04u128, 0x01);
dec_test!(bid128_mul_099, bid128_mul, 1, "Infinity"                            , "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_100, bid128_mul, 1, "Infinity"                            ,     "QNaN"                            , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_101, bid128_mul, 1,     "QNaN"                            , "Infinity"                            , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_102, bid128_mul, 1,     "SNaN"                            ,        "0"                            , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_mul_103, bid128_mul, 2, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_mul_104, bid128_mul, 2, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_105, bid128_mul, 2, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_106, bid128_mul, 2, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_107, bid128_mul, 2, 0x006a563a06a04c0aef3a3c23e2d36044u128, 0x07dce5d3ce823c76e4dfa77086711c61u128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_mul_108, bid128_mul, 2, 0x03ac0000000000000000000000000000u128, 0x51ab2723d21d4c916b5f75f600dcbbe2u128, 0x25160000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_109, bid128_mul, 2, 0x0aabe0d152074dfb112a9edce0e067c9u128, 0x8d13a79968c6b31ca569c623cd4fc5e6u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_110, bid128_mul, 2, 0x0d12f8e176524a70cc5defb99c48f4beu128, 0x10ed50c9b1ac645edef7136ee58dd67bu128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_mul_111, bid128_mul, 2, 0x0f15be3dcaa7dfc995b386b7eb810dc4u128, 0x039d2ae0bcc67eee830d4ecc5e2b577eu128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_mul_112, bid128_mul, 2, "-0"                                   , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_113, bid128_mul, 2,  "0"                                  , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
// dec_test!(bid128_mul_114, bid128_mul, 2, -1000101110110110011.1E0 -972.2639553368233236E0 [3027df695f2b352e7e7a36cc2c3b9415] , 0x20);
// dec_test!(bid128_mul_115, bid128_mul, 2, -110011000.010110111E0 -47356549229256.67778E0 [302900dc2e26fe4ef5b7b8b43afa0f7a] , 0x20);
dec_test!(bid128_mul_116, bid128_mul, 2, 0x133152f5bc6f288c89983ce0c603cf36u128, 0x904132dac1cc5c56519f7db3adbbbfe8u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_117, bid128_mul, 2, 0x150a2e0d6728de4e95595bd43d654036u128, 0xc47aef17e9919a5569aaaf503275e8f4u128, 0xa986df536afc692da7504914a519abd4u128, 0x20);
dec_test!(bid128_mul_118, bid128_mul, 2, 0x31155f3baa6a49f68431049cfbdab6c9u128, 0x5054e9c9e2631401d383b4964f87e05eu128, 0x516ca68c2d79d4b93e54601d1a8780b6u128, 0x20);
dec_test!(bid128_mul_119, bid128_mul, 2, 0x40e20000000000000000000000000000u128, 0x4db862ab3b7a3a4a872d344e3263ff63u128, 0x5e5a0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_120, bid128_mul, 2, 0x46f8c66750b4255ee5f50c08bb2dec23u128, 0xb0739679b9d6a42fd261a9d744b1adb8u128, 0xc76ea391e575aec63c18ab5068cb6cbdu128, 0x20);
dec_test!(bid128_mul_121, bid128_mul, 2, 0x4a660000000000000000000000000000u128, 0xdd740000000000000000000000000000u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_122, bid128_mul, 2, 0x4db6022c6ac90c122f31f9e027c0cb3cu128, 0x5e5a3663fad59572b27d2f9994f1ed2eu128, 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_mul_123, bid128_mul, 2, 0x530919f1e09dd59f9b951f8183574016u128, 0x169eef000f7a5d7761d331242cf44747u128, 0x39aa88ac3b2436ed02af1c1b7c6e9da0u128, 0x20);
dec_test!(bid128_mul_124, bid128_mul, 2, 0x565a0000000000000000000000000000u128, 0xd91854823ba2af952bb97cf8f2d4c083u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_125, bid128_mul, 2, 0x57125da9a5b8c9a339046f20cf22c152u128, 0x3f1f0ae34f231d874f1ff6ae8d5b03c7u128, 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_mul_126, bid128_mul, 2, 0x82a4531e6dfac7247b2af5f3554e7e83u128, 0xb99d5e676c84d32e12a6af7cb2f61a3cu128, 0x0c443b12a8dbc75006fa90a2aa727873u128, 0x20);
dec_test!(bid128_mul_127, bid128_mul, 2, 0x8ec7a09091dc32b2bce36c840a1e3b09u128, 0x89b040fc053bf28c09fc7bd2902fe28du128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_mul_128, bid128_mul, 2, 0x93b156741e3961009422ad67ad806698u128, 0x99f7d2c00d96a0ec38b3e16736b0c3fau128, 0x00000000000000000000000000000001u128, 0x30);
dec_test!(bid128_mul_129, bid128_mul, 2, 0x94c7d1f889198a96842e29d55cb821c0u128, 0x588efd2cd70a1fde9da7dc0ddb569cf9u128, 0xbd58ef46b08263af9b9e88771ea06910u128, 0x20);
dec_test!(bid128_mul_130, bid128_mul, 2, 0x95a56f957b5afdf78e550ab858bed620u128, 0x0f8f316b321a1c2d9c72537f9bed8d45u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_131, bid128_mul, 2, 0x970c0000000000000000000000000000u128, 0xd7d9186750ca22882d23d2cb53e46703u128, 0x3ea40000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_132, bid128_mul, 2, 0x9c727f7fae575afc5334029e9553a602u128, 0xa5600000000000000000000000000000u128, 0x11920000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_133, bid128_mul, 2, 0x9f8ca44956311f864a91e13c61ad8be4u128, 0x129fd783cd14dbf5f6d01c8e3d4956e6u128, 0x822e9d1d5ab22c4d8a4fbfa6c7d1f6c5u128, 0x20);
dec_test!(bid128_mul_134, bid128_mul, 2, 0xaaf5c59f39cce565b3f956ea0bb0026eu128, 0xcd5fd6c6a1d24cc0f4a7617f943b3b70u128, 0x4857b123bb020f02a66c16f22ea8ade7u128, 0x20);
dec_test!(bid128_mul_135, bid128_mul, 2, 0xae7f324e2487cef3d04046b7603dbc92u128, 0x78000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_136, bid128_mul, 2, 0xbdea1d8a96ef0987ed743f875d54d67fu128, 0x82a53be053aa524e95c2f33fc529cb60u128, 0x1090bd437c2250979afeb07cd0f09dd4u128, 0x20);
dec_test!(bid128_mul_137, bid128_mul, 2, 0xd261e79d08c12070d5c241f3d7f02504u128, 0xd0ec877840c4d4963b42037ee877ec03u128, 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_mul_138, bid128_mul, 2, 0xd4c641b3a7a0d36f7c58e0915a454073u128, 0xaace0000000000000000000000000000u128, 0x4f540000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_139, bid128_mul, 2, 0xd6809c5dae6efdc492ee1f5fcc48dc7du128, 0x7e003009107549d6ebea61d422a3f73cu128, 0x7c003009107549d6ebea61d422a3f73cu128, 0x01);
dec_test!(bid128_mul_140, bid128_mul, 2, 0xd6fb2c0a9edd8b13a888a98ad171d726u128, 0x7e0015e5379f538dac0b835438210c04u128, 0x7c0015e5379f538dac0b835438210c04u128, 0x01);
dec_test!(bid128_mul_141, bid128_mul, 2, "Infinity"                            , "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_142, bid128_mul, 2, "Infinity"                            ,     "QNaN"                            , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_143, bid128_mul, 2,     "QNaN"                            , "Infinity"                            , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_144, bid128_mul, 2,     "SNaN"                            ,        "0"                            , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_mul_145, bid128_mul, 3, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_146, bid128_mul, 3, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_147, bid128_mul, 3, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_148, bid128_mul, 3, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_149, bid128_mul, 3, 0x006a563a06a04c0aef3a3c23e2d36044u128, 0x07dce5d3ce823c76e4dfa77086711c61u128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_150, bid128_mul, 3, 0x03ac0000000000000000000000000000u128, 0x51ab2723d21d4c916b5f75f600dcbbe2u128, 0x25160000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_151, bid128_mul, 3, 0x0aabe0d152074dfb112a9edce0e067c9u128, 0x8d13a79968c6b31ca569c623cd4fc5e6u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_152, bid128_mul, 3, 0x0d12f8e176524a70cc5defb99c48f4beu128, 0x10ed50c9b1ac645edef7136ee58dd67bu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_153, bid128_mul, 3, 0x0f15be3dcaa7dfc995b386b7eb810dc4u128, 0x039d2ae0bcc67eee830d4ecc5e2b577eu128, 0x00000000000000000000000000000000u128, 0x30);
// dec_test!(bid128_mul_154, bid128_mul, 3, -0 QNaN [7c000000000000000000000000000000] , 0x00);
// dec_test!(bid128_mul_155, bid128_mul, 3, 0 QNaN [7c000000000000000000000000000000] , 0x00);
// dec_test!(bid128_mul_156, bid128_mul, 3, -1000101110110110011.1E0 -972.2639553368233236E0 [3027df695f2b352e7e7a36cc2c3b9414] , 0x20);
// dec_test!(bid128_mul_157, bid128_mul, 3, -110011000.010110111E0 -47356549229256.67778E0 [302900dc2e26fe4ef5b7b8b43afa0f79] , 0x20);
dec_test!(bid128_mul_158, bid128_mul, 3, 0x133152f5bc6f288c89983ce0c603cf36u128, 0x904132dac1cc5c56519f7db3adbbbfe8u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_159, bid128_mul, 3, 0x150a2e0d6728de4e95595bd43d654036u128, 0xc47aef17e9919a5569aaaf503275e8f4u128, 0xa986df536afc692da7504914a519abd4u128, 0x20);
dec_test!(bid128_mul_160, bid128_mul, 3, 0x31155f3baa6a49f68431049cfbdab6c9u128, 0x5054e9c9e2631401d383b4964f87e05eu128, 0x516ca68c2d79d4b93e54601d1a8780b5u128, 0x20);
dec_test!(bid128_mul_161, bid128_mul, 3, 0x40e20000000000000000000000000000u128, 0x4db862ab3b7a3a4a872d344e3263ff63u128, 0x5e5a0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_162, bid128_mul, 3, 0x46f8c66750b4255ee5f50c08bb2dec23u128, 0xb0739679b9d6a42fd261a9d744b1adb8u128, 0xc76ea391e575aec63c18ab5068cb6cbdu128, 0x20);
dec_test!(bid128_mul_163, bid128_mul, 3, 0x4a660000000000000000000000000000u128, 0xdd740000000000000000000000000000u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_164, bid128_mul, 3, 0x4db6022c6ac90c122f31f9e027c0cb3cu128, 0x5e5a3663fad59572b27d2f9994f1ed2eu128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x28);
dec_test!(bid128_mul_165, bid128_mul, 3, 0x530919f1e09dd59f9b951f8183574016u128, 0x169eef000f7a5d7761d331242cf44747u128, 0x39aa88ac3b2436ed02af1c1b7c6e9d9fu128, 0x20);
dec_test!(bid128_mul_166, bid128_mul, 3, 0x565a0000000000000000000000000000u128, 0xd91854823ba2af952bb97cf8f2d4c083u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_167, bid128_mul, 3, 0x57125da9a5b8c9a339046f20cf22c152u128, 0x3f1f0ae34f231d874f1ff6ae8d5b03c7u128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x28);
dec_test!(bid128_mul_168, bid128_mul, 3, 0x82a4531e6dfac7247b2af5f3554e7e83u128, 0xb99d5e676c84d32e12a6af7cb2f61a3cu128, 0x0c443b12a8dbc75006fa90a2aa727872u128, 0x20);
dec_test!(bid128_mul_169, bid128_mul, 3, 0x8ec7a09091dc32b2bce36c840a1e3b09u128, 0x89b040fc053bf28c09fc7bd2902fe28du128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_170, bid128_mul, 3, 0x93b156741e3961009422ad67ad806698u128, 0x99f7d2c00d96a0ec38b3e16736b0c3fau128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_171, bid128_mul, 3, 0x94c7d1f889198a96842e29d55cb821c0u128, 0x588efd2cd70a1fde9da7dc0ddb569cf9u128, 0xbd58ef46b08263af9b9e88771ea06910u128, 0x20);
dec_test!(bid128_mul_172, bid128_mul, 3, 0x95a56f957b5afdf78e550ab858bed620u128, 0x0f8f316b321a1c2d9c72537f9bed8d45u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_173, bid128_mul, 3, 0x970c0000000000000000000000000000u128, 0xd7d9186750ca22882d23d2cb53e46703u128, 0x3ea40000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_174, bid128_mul, 3, 0x9c727f7fae575afc5334029e9553a602u128, 0xa5600000000000000000000000000000u128, 0x11920000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_175, bid128_mul, 3, 0x9f8ca44956311f864a91e13c61ad8be4u128, 0x129fd783cd14dbf5f6d01c8e3d4956e6u128, 0x822e9d1d5ab22c4d8a4fbfa6c7d1f6c5u128, 0x20);
dec_test!(bid128_mul_176, bid128_mul, 3, 0xaaf5c59f39cce565b3f956ea0bb0026eu128, 0xcd5fd6c6a1d24cc0f4a7617f943b3b70u128, 0x4857b123bb020f02a66c16f22ea8ade6u128, 0x20);
dec_test!(bid128_mul_177, bid128_mul, 3, 0xae7f324e2487cef3d04046b7603dbc92u128, 0x78000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_178, bid128_mul, 3, 0xbdea1d8a96ef0987ed743f875d54d67fu128, 0x82a53be053aa524e95c2f33fc529cb60u128, 0x1090bd437c2250979afeb07cd0f09dd3u128, 0x20);
dec_test!(bid128_mul_179, bid128_mul, 3, 0xd261e79d08c12070d5c241f3d7f02504u128, 0xd0ec877840c4d4963b42037ee877ec03u128, 0x5fffed09bead87c0378d8e63ffffffffu128, 0x28);
dec_test!(bid128_mul_180, bid128_mul, 3, 0xd4c641b3a7a0d36f7c58e0915a454073u128, 0xaace0000000000000000000000000000u128, 0x4f540000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_181, bid128_mul, 3, 0xd6809c5dae6efdc492ee1f5fcc48dc7du128, 0x7e003009107549d6ebea61d422a3f73cu128, 0x7c003009107549d6ebea61d422a3f73cu128, 0x01);
dec_test!(bid128_mul_182, bid128_mul, 3, 0xd6fb2c0a9edd8b13a888a98ad171d726u128, 0x7e0015e5379f538dac0b835438210c04u128, 0x7c0015e5379f538dac0b835438210c04u128, 0x01);
dec_test!(bid128_mul_183, bid128_mul, 3, "Infinity"                            , "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_184, bid128_mul, 3, "Infinity"                            ,     "QNaN"                            , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_185, bid128_mul, 3,     "QNaN"                            , "Infinity"                            , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_186, bid128_mul, 3,     "SNaN"                            ,        "0"                            , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_mul_187, bid128_mul, 4, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_188, bid128_mul, 4, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_189, bid128_mul, 4, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_190, bid128_mul, 4, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_191, bid128_mul, 4, 0x006a563a06a04c0aef3a3c23e2d36044u128, 0x07dce5d3ce823c76e4dfa77086711c61u128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_192, bid128_mul, 4, 0x03ac0000000000000000000000000000u128, 0x51ab2723d21d4c916b5f75f600dcbbe2u128, 0x25160000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_193, bid128_mul, 4, 0x0aabe0d152074dfb112a9edce0e067c9u128, 0x8d13a79968c6b31ca569c623cd4fc5e6u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_194, bid128_mul, 4, 0x0d12f8e176524a70cc5defb99c48f4beu128, 0x10ed50c9b1ac645edef7136ee58dd67bu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_195, bid128_mul, 4, 0x0f15be3dcaa7dfc995b386b7eb810dc4u128, 0x039d2ae0bcc67eee830d4ecc5e2b577eu128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_196, bid128_mul, 4, "-0"                                  , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_197, bid128_mul, 4,  "0"                                  , "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
// dec_test!(bid128_mul_198, bid128_mul, 4, -1000101110110110011.1E0 -972.2639553368233236E0 [3027df695f2b352e7e7a36cc2c3b9415] , 0x20);
// dec_test!(bid128_mul_199, bid128_mul, 4, "-110011000.010110111E0"              , "-47356549229256.67778E0"             , 0x302900dc2e26fe4ef5b7b8b43afa0f7au128, 0x20);
dec_test!(bid128_mul_200, bid128_mul, 4, 0x133152f5bc6f288c89983ce0c603cf36u128, 0x904132dac1cc5c56519f7db3adbbbfe8u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_201, bid128_mul, 4, 0x150a2e0d6728de4e95595bd43d654036u128, 0xc47aef17e9919a5569aaaf503275e8f4u128, 0xa986df536afc692da7504914a519abd5u128, 0x20);
dec_test!(bid128_mul_202, bid128_mul, 4, 0x31155f3baa6a49f68431049cfbdab6c9u128, 0x5054e9c9e2631401d383b4964f87e05eu128, 0x516ca68c2d79d4b93e54601d1a8780b6u128, 0x20);
dec_test!(bid128_mul_203, bid128_mul, 4, 0x40e20000000000000000000000000000u128, 0x4db862ab3b7a3a4a872d344e3263ff63u128, 0x5e5a0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_204, bid128_mul, 4, 0x46f8c66750b4255ee5f50c08bb2dec23u128, 0xb0739679b9d6a42fd261a9d744b1adb8u128, 0xc76ea391e575aec63c18ab5068cb6cbdu128, 0x20);
dec_test!(bid128_mul_205, bid128_mul, 4, 0x4a660000000000000000000000000000u128, 0xdd740000000000000000000000000000u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_206, bid128_mul, 4, 0x4db6022c6ac90c122f31f9e027c0cb3cu128, 0x5e5a3663fad59572b27d2f9994f1ed2eu128, 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_mul_207, bid128_mul, 4, 0x530919f1e09dd59f9b951f8183574016u128, 0x169eef000f7a5d7761d331242cf44747u128, 0x39aa88ac3b2436ed02af1c1b7c6e9d9fu128, 0x20);
dec_test!(bid128_mul_208, bid128_mul, 4, 0x565a0000000000000000000000000000u128, 0xd91854823ba2af952bb97cf8f2d4c083u128, 0xdffe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_209, bid128_mul, 4, 0x57125da9a5b8c9a339046f20cf22c152u128, 0x3f1f0ae34f231d874f1ff6ae8d5b03c7u128, 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_mul_210, bid128_mul, 4, 0x82a4531e6dfac7247b2af5f3554e7e83u128, 0xb99d5e676c84d32e12a6af7cb2f61a3cu128, 0x0c443b12a8dbc75006fa90a2aa727872u128, 0x20);
dec_test!(bid128_mul_211, bid128_mul, 4, 0x8ec7a09091dc32b2bce36c840a1e3b09u128, 0x89b040fc053bf28c09fc7bd2902fe28du128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_212, bid128_mul, 4, 0x93b156741e3961009422ad67ad806698u128, 0x99f7d2c00d96a0ec38b3e16736b0c3fau128, 0x00000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_213, bid128_mul, 4, 0x94c7d1f889198a96842e29d55cb821c0u128, 0x588efd2cd70a1fde9da7dc0ddb569cf9u128, 0xbd58ef46b08263af9b9e88771ea06911u128, 0x20);
dec_test!(bid128_mul_214, bid128_mul, 4, 0x95a56f957b5afdf78e550ab858bed620u128, 0x0f8f316b321a1c2d9c72537f9bed8d45u128, 0x80000000000000000000000000000000u128, 0x30);
dec_test!(bid128_mul_215, bid128_mul, 4, 0x970c0000000000000000000000000000u128, 0xd7d9186750ca22882d23d2cb53e46703u128, 0x3ea40000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_216, bid128_mul, 4, 0x9c727f7fae575afc5334029e9553a602u128, 0xa5600000000000000000000000000000u128, 0x11920000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_217, bid128_mul, 4, 0x9f8ca44956311f864a91e13c61ad8be4u128, 0x129fd783cd14dbf5f6d01c8e3d4956e6u128, 0x822e9d1d5ab22c4d8a4fbfa6c7d1f6c6u128, 0x20);
dec_test!(bid128_mul_218, bid128_mul, 4, 0xaaf5c59f39cce565b3f956ea0bb0026eu128, 0xcd5fd6c6a1d24cc0f4a7617f943b3b70u128, 0x4857b123bb020f02a66c16f22ea8ade6u128, 0x20);
dec_test!(bid128_mul_219, bid128_mul, 4, 0xae7f324e2487cef3d04046b7603dbc92u128, 0x78000000000000000000000000000000u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_220, bid128_mul, 4, 0xbdea1d8a96ef0987ed743f875d54d67fu128, 0x82a53be053aa524e95c2f33fc529cb60u128, 0x1090bd437c2250979afeb07cd0f09dd3u128, 0x20);
dec_test!(bid128_mul_221, bid128_mul, 4, 0xd261e79d08c12070d5c241f3d7f02504u128, 0xd0ec877840c4d4963b42037ee877ec03u128, 0x78000000000000000000000000000000u128, 0x28);
dec_test!(bid128_mul_222, bid128_mul, 4, 0xd4c641b3a7a0d36f7c58e0915a454073u128, 0xaace0000000000000000000000000000u128, 0x4f540000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_223, bid128_mul, 4, 0xd6809c5dae6efdc492ee1f5fcc48dc7du128, 0x7e003009107549d6ebea61d422a3f73cu128, 0x7c003009107549d6ebea61d422a3f73cu128, 0x01);
dec_test!(bid128_mul_224, bid128_mul, 4, 0xd6fb2c0a9edd8b13a888a98ad171d726u128, 0x7e0015e5379f538dac0b835438210c04u128, 0x7c0015e5379f538dac0b835438210c04u128, 0x01);
dec_test!(bid128_mul_225, bid128_mul, 4, "Infinity"                            , "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_226, bid128_mul, 4, "Infinity"                            ,     "QNaN"                            , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_mul_227, bid128_mul, 4,     "QNaN"                            , "Infinity"                            , 0x7c000000000000000000000000000000u128 , 0x00);
dec_test!(bid128_mul_228, bid128_mul, 4,     "SNaN"                            ,        "0"                            , 0x7c000000000000000000000000000000u128 , 0x01);
