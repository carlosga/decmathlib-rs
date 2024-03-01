/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_round_integral_nearest_even_001, bid128_round_integral_nearest_even, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_002, bid128_round_integral_nearest_even, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_003, bid128_round_integral_nearest_even, 0, "0"                                   , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_004, bid128_round_integral_nearest_even, 0, "-0"                                  , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_005, bid128_round_integral_nearest_even, 0, 0x3000000060081000658c42208884a010u128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_006, bid128_round_integral_nearest_even, 0, 0x3001400000002010ffffe8e77ffbfdf7u128, 0x30400000000000000000000000000041u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_007, bid128_round_integral_nearest_even, 0, 0x303c84a9c4090924a881e505a7542dd8u128, 0x304001539e1ecf73910b89f86064ae8au128, 0x00);
dec_test!(bid128_round_integral_nearest_even_008, bid128_round_integral_nearest_even, 0, 0x303e81c2d0398347dcafe585defb1fe5u128, 0x30400cf9e19f59ed961196f3c97f8330u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_009, bid128_round_integral_nearest_even, 0, 0x3085ED09BEAD87C0378D8E63ffffffffu128, 0x3085ed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_nearest_even_010, bid128_round_integral_nearest_even, 0, 0x3085ED09BEAD87C0378D8E6400000000u128, 0x30840000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_011, bid128_round_integral_nearest_even, 0, 0x3CB2314DC6448D9338C15B09ffffffffu128, 0x3cb2314dc6448d9338c15b09ffffffffu128, 0x00);
dec_test!(bid128_round_integral_nearest_even_012, bid128_round_integral_nearest_even, 0, 0x3CB2314DC6448D9338C15B0A00000000u128, 0x3cb2314dc6448d9338c15b0a00000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_013, bid128_round_integral_nearest_even, 0, 0x3f4836d53889b4b0b9e6c9ff6fe5fff7u128, 0x3f4836d53889b4b0b9e6c9ff6fe5fff7u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_014, bid128_round_integral_nearest_even, 0, 0x40000000000000000000000000000000u128, 0x40000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_015, bid128_round_integral_nearest_even, 0, 0x40000000000000000c3522204a902821u128, 0x40000000000000000c3522204a902821u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_016, bid128_round_integral_nearest_even, 0, 0x41400000000000000004400000001800u128, 0x41400000000000000004400000001800u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_017, bid128_round_integral_nearest_even, 0, 0x488080c21440030a45c352ae05f7ca95u128, 0x488080c21440030a45c352ae05f7ca95u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_018, bid128_round_integral_nearest_even, 0, 0x7acdf567db3a69159a859ae63a9f889cu128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_019, bid128_round_integral_nearest_even, 0, 0x7c00314dc6448d9338c15b0a00000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_020, bid128_round_integral_nearest_even, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_021, bid128_round_integral_nearest_even, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_022, bid128_round_integral_nearest_even, 0, 0x7e000c337e6ca979447968a6af3e155bu128, 0x7c000c337e6ca979447968a6af3e155bu128, 0x01);
dec_test!(bid128_round_integral_nearest_even_023, bid128_round_integral_nearest_even, 0, 0x7e002667f89f6fe4105a8b590ab81f56u128, 0x7c002667f89f6fe4105a8b590ab81f56u128, 0x01);
dec_test!(bid128_round_integral_nearest_even_024, bid128_round_integral_nearest_even, 0, 0x7e0029d324b819ac59f16667ff456c9bu128, 0x7c0029d324b819ac59f16667ff456c9bu128, 0x01);
dec_test!(bid128_round_integral_nearest_even_025, bid128_round_integral_nearest_even, 0, 0x8229809401845ed0249944e50c2c4e3eu128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_026, bid128_round_integral_nearest_even, 0, 0xb02ec313a85b8cea3f1a65dfb58b4ebfu128, 0xb0400000000345d932451cf442781680u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_027, bid128_round_integral_nearest_even, 0, 0xbd080ae6518374ebe3b6da828802961cu128, 0xbd080ae6518374ebe3b6da828802961cu128, 0x00);
dec_test!(bid128_round_integral_nearest_even_028, bid128_round_integral_nearest_even, 0, 0xd77fffeeffbedffffffffffffffffbffu128, 0xd77e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_029, bid128_round_integral_nearest_even, 0, 0xd8ffb23fee9786e2fefbeb7fb7f99fb6u128, 0xd8ffb23fee9786e2fefbeb7fb7f99fb6u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_030, bid128_round_integral_nearest_even, 0, 0xDF7FED09BEAD87C0378D8E63ffffffffu128, 0xdf7fed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_nearest_even_031, bid128_round_integral_nearest_even, 0, 0xDF7FED09BEAD87C0378D8E6400000000u128, 0xdf7e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_032, bid128_round_integral_nearest_even, 0, 0xebfdfefd7e36b163e8c7e4baa92e7bdcu128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_033, bid128_round_integral_nearest_even, 0, 0xfa918e40178d23896abfea84476d92c1u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_034, bid128_round_integral_nearest_even, 0, 0xfdff5ff7fffffefb7d7d87364da34f2du128, 0xfc001ff7fffffefb7d7d87364da34f2du128, 0x00);
dec_test!(bid128_round_integral_nearest_even_035, bid128_round_integral_nearest_even, 0, 0xfe0002b7e26df76235ab7f58a136d72au128, 0xfc0002b7e26df76235ab7f58a136d72au128, 0x01);
dec_test!(bid128_round_integral_nearest_even_036, bid128_round_integral_nearest_even, 0, 0xfe00126fba1c1afcd27b58d7b26d9855u128, 0xfc00126fba1c1afcd27b58d7b26d9855u128, 0x01);
dec_test!(bid128_round_integral_nearest_even_037, bid128_round_integral_nearest_even, 0, 0xfe001b8b5af30648e03fb062625ec69du128, 0xfc001b8b5af30648e03fb062625ec69du128, 0x01);
dec_test!(bid128_round_integral_nearest_even_038, bid128_round_integral_nearest_even, 0, 0xfe0022c25476ccf7e52dea461f294667u128, 0xfc0022c25476ccf7e52dea461f294667u128, 0x01);
dec_test!(bid128_round_integral_nearest_even_039, bid128_round_integral_nearest_even, 0, 0xfe002517a20f5706ce416fbe679dcfbcu128, 0xfc002517a20f5706ce416fbe679dcfbcu128, 0x01);
dec_test!(bid128_round_integral_nearest_even_040, bid128_round_integral_nearest_even, 0, 0xfe002fbbf6ad3928d23ec1161e3218b1u128, 0xfc002fbbf6ad3928d23ec1161e3218b1u128, 0x01);
dec_test!(bid128_round_integral_nearest_even_041, bid128_round_integral_nearest_even, 0, 0xffbfeafe7fffbf7f882000a020209850u128, 0xfc002afe7fffbf7f882000a020209850u128, 0x01);
dec_test!(bid128_round_integral_nearest_even_042, bid128_round_integral_nearest_even, 0, 0xfffdbfffffffffbffeff9effa5faacddu128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_nearest_even_043, bid128_round_integral_nearest_even, 0, "-Infinity"                           , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_044, bid128_round_integral_nearest_even, 0, "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_nearest_even_045, bid128_round_integral_nearest_even, 0, "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);

dec_test!(bid128_round_integral_nearest_even_046, bid128_round_integral_nearest_even, 0, "18446744073709551616500E-3"          , "18446744073709551616E+0"             , 0x00);
dec_test!(bid128_round_integral_nearest_even_047, bid128_round_integral_nearest_even, 0, "91E-1"                               , 9                                     , 0x00);
dec_test!(bid128_round_integral_nearest_even_048, bid128_round_integral_nearest_even, 1, "91E-1"                               , 9                                     , 0x00);
dec_test!(bid128_round_integral_nearest_even_049, bid128_round_integral_nearest_even, 2, "91E-1"                               , 9                                     , 0x00);
dec_test!(bid128_round_integral_nearest_even_050, bid128_round_integral_nearest_even, 3, "91E-1"                               , 9                                     , 0x00);
dec_test!(bid128_round_integral_nearest_even_051, bid128_round_integral_nearest_even, 0, "90001E-4"                            , 9                                     , 0x00);
dec_test!(bid128_round_integral_nearest_even_052, bid128_round_integral_nearest_even, 1, "90001E-4"                            , 9                                     , 0x00);
dec_test!(bid128_round_integral_nearest_even_053, bid128_round_integral_nearest_even, 2, "90001E-4"                            , 9                                     , 0x00);
dec_test!(bid128_round_integral_nearest_even_054, bid128_round_integral_nearest_even, 3, "90001E-4"                            , 9                                     , 0x00);
dec_test!(bid128_round_integral_nearest_even_055, bid128_round_integral_nearest_even, 0, "96000000000000000000008778781E-23"   , 960000                                , 0x00);
dec_test!(bid128_round_integral_nearest_even_056, bid128_round_integral_nearest_even, 0, "900000000000000000000001E-23"        , 9                                     , 0x00);
dec_test!(bid128_round_integral_nearest_even_057, bid128_round_integral_nearest_even, 0, "205000E-4"                           , 20                                    , 0x00);
dec_test!(bid128_round_integral_nearest_even_058, bid128_round_integral_nearest_even, 0, "205001E-4"                           , 21                                    , 0x00);
dec_test!(bid128_round_integral_nearest_even_059, bid128_round_integral_nearest_even, 0, "2050000000000000000000000E-23"       , 20                                    , 0x00);
dec_test!(bid128_round_integral_nearest_even_060, bid128_round_integral_nearest_even, 0, "96E-1"                               , 10                                    , 0x00);
