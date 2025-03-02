/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_round_integral_positive_001, bid128_round_integral_positive, 0, 0x00000132280008005abfeffcfdab9cf9u128, 0x30400000000000000000000000000001u128, 0x00);
dec_test!(bid128_round_integral_positive_002, bid128_round_integral_positive, 0, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x30400000000000000000000000000001u128, 0x00);
dec_test!(bid128_round_integral_positive_003, bid128_round_integral_positive, 0, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_004, bid128_round_integral_positive, 0, "0"                                   , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_005, bid128_round_integral_positive, 0, "-0"                                  , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_006, bid128_round_integral_positive, 0, 0x144d870e001694043c9f5f5597637768u128, 0x30400000000000000000000000000001u128, 0x00);
dec_test!(bid128_round_integral_positive_007, bid128_round_integral_positive, 0, 0x1f6aefe24c11d374ba7425c25f9c9cfbu128, 0x30400000000000000000000000000001u128, 0x00);
dec_test!(bid128_round_integral_positive_008, bid128_round_integral_positive, 0, 0x230ae00c50234d5984b43c93d47ddb1du128, 0x30400000000000000000000000000001u128, 0x00);
dec_test!(bid128_round_integral_positive_009, bid128_round_integral_positive, 0, 0x30000092000a06014688a5a1a0869764u128, 0x30400000000000000000000000000001u128, 0x00);
dec_test!(bid128_round_integral_positive_010, bid128_round_integral_positive, 0, 0x30000c1e1274dac88201763020002400u128, 0x30400000000000000000000000000003u128, 0x00);
dec_test!(bid128_round_integral_positive_011, bid128_round_integral_positive, 0, 0x30218c18080100008203000000000060u128, 0x30400000000000000b26283a94e63700u128, 0x00);
dec_test!(bid128_round_integral_positive_012, bid128_round_integral_positive, 0, 0x303b65bebb06e334efffbfdffbffedffu128, 0x3040005b952dd4b50b81061472af1a9cu128, 0x00);
dec_test!(bid128_round_integral_positive_013, bid128_round_integral_positive, 0, 0x303f7eae427fdfe3ffffffffffffffffu128, 0x30402644a03ffcca0000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_014, bid128_round_integral_positive, 0, 0x3085ED09BEAD87C0378D8E63ffffffffu128, 0x3085ed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_positive_015, bid128_round_integral_positive, 0, 0x3085ED09BEAD87C0378D8E6400000000u128, 0x30840000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_016, bid128_round_integral_positive, 0, 0x3CB2314DC6448D9338C15B09ffffffffu128, 0x3cb2314dc6448d9338c15b09ffffffffu128, 0x00);
dec_test!(bid128_round_integral_positive_017, bid128_round_integral_positive, 0, 0x3CB2314DC6448D9338C15B0A00000000u128, 0x3cb2314dc6448d9338c15b0a00000000u128, 0x00);
dec_test!(bid128_round_integral_positive_018, bid128_round_integral_positive, 0, 0x40000000000000000000000000000000u128, 0x40000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_019, bid128_round_integral_positive, 0, 0x4000000000000000b3c02053e084c426u128, 0x4000000000000000b3c02053e084c426u128, 0x00);
dec_test!(bid128_round_integral_positive_020, bid128_round_integral_positive, 0, 0x40080000000000000003400080000002u128, 0x40080000000000000003400080000002u128, 0x00);
dec_test!(bid128_round_integral_positive_021, bid128_round_integral_positive, 0, 0x400a042801020ec02105a1c861202064u128, 0x400a042801020ec02105a1c861202064u128, 0x00);
dec_test!(bid128_round_integral_positive_022, bid128_round_integral_positive, 0, 0x524096290ce6c59f4d1bf826a836324fu128, 0x524096290ce6c59f4d1bf826a836324fu128, 0x00);
dec_test!(bid128_round_integral_positive_023, bid128_round_integral_positive, 0, 0x78000000000000000000000000000000u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_024, bid128_round_integral_positive, 0, 0x7a4044a1606f738be482a60ca002b521u128, 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_025, bid128_round_integral_positive, 0, 0x7c00314dc6448d9338c15b0a00000000u128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_026, bid128_round_integral_positive, 0, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_027, bid128_round_integral_positive, 0, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_028, bid128_round_integral_positive, 0, 0x7dd1c6d9c9f5ebf3de9dce62137a6f68u128, 0x7c0006d9c9f5ebf3de9dce62137a6f68u128, 0x00);
dec_test!(bid128_round_integral_positive_029, bid128_round_integral_positive, 0, 0x7e000466e2af7726ac13727a266a84f3u128, 0x7c000466e2af7726ac13727a266a84f3u128, 0x01);
dec_test!(bid128_round_integral_positive_030, bid128_round_integral_positive, 0, 0x7e002a20d924994aad08dc73e845f988u128, 0x7c002a20d924994aad08dc73e845f988u128, 0x01);
dec_test!(bid128_round_integral_positive_031, bid128_round_integral_positive, 0, 0x840476e3e0e003d0ed7a9b12fffe7ab9u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_032, bid128_round_integral_positive, 0, 0x91785bbb00ce5c4381e70483653d7dc7u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_033, bid128_round_integral_positive, 0, 0xa95a2e2078a547b7a5a1d77ceb55286bu128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_034, bid128_round_integral_positive, 0, 0xb000000a8104a108ef7b67bfbafb9bbeu128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_035, bid128_round_integral_positive, 0, 0xb4683219a7b1d2fb2c042824b39685d8u128, 0xb4683219a7b1d2fb2c042824b39685d8u128, 0x00);
dec_test!(bid128_round_integral_positive_036, bid128_round_integral_positive, 0, 0xc1666d0c2c12d6efd9bae4a380161961u128, 0xc1666d0c2c12d6efd9bae4a380161961u128, 0x00);
dec_test!(bid128_round_integral_positive_037, bid128_round_integral_positive, 0, 0xcdf9f5d1eab026a3ff33aed2fba1c1e9u128, 0xcdf80000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_038, bid128_round_integral_positive, 0, 0xd41c0000000000000000000000000000u128, 0xd41c0000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_039, bid128_round_integral_positive, 0, 0xDF7FED09BEAD87C0378D8E63ffffffffu128, 0xdf7fed09bead87c0378d8e63ffffffffu128, 0x00);
dec_test!(bid128_round_integral_positive_040, bid128_round_integral_positive, 0, 0xDF7FED09BEAD87C0378D8E6400000000u128, 0xdf7e0000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_041, bid128_round_integral_positive, 0, 0xe6dfaef705bb178ca8410b0600200002u128, 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_042, bid128_round_integral_positive, 0, 0xf6ffb9f7f57fffef0008000000001800u128, 0xdbfe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_043, bid128_round_integral_positive, 0, 0xfbd2a56cbeeebbb73bb45f0fee418e90u128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_044, bid128_round_integral_positive, 0, 0xfffb3cddd2bf4eff0000000009200801u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_round_integral_positive_045, bid128_round_integral_positive, 0, "Infinity"                            , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_046, bid128_round_integral_positive, 0, "QNaN"                                , 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_round_integral_positive_047, bid128_round_integral_positive, 0, "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);

dec_test!(bid128_round_integral_positive_048, bid128_round_integral_positive, 0, "18446744073709551616500E-3"          , "18446744073709551617E+0"             , 0x00);
dec_test!(bid128_round_integral_positive_049, bid128_round_integral_positive, 0, "18446744073709551616000E-3"          , "18446744073709551616E+0"             , 0x00);
dec_test!(bid128_round_integral_positive_050, bid128_round_integral_positive, 0, "91E-1"                               , 10                                    , 0x00);
dec_test!(bid128_round_integral_positive_051, bid128_round_integral_positive, 0, "90001E-4"                            , 10                                    , 0x00);
dec_test!(bid128_round_integral_positive_052, bid128_round_integral_positive, 0, "900000000000000000000001E-23"        , 10                                    , 0x00);
dec_test!(bid128_round_integral_positive_053, bid128_round_integral_positive, 0, "9600000000000000000000000E-23"       , 96                                    , 0x00);
