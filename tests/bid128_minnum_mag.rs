/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.  */
/* ----------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_minnum_mag_001, bid128_minnum_mag, 0x0000000028a000040a000119004400c0u128, 0x983c84fa67f5f4db4e9f51bc0bcfab38u128, 0x0000000028a000040a000119004400c0u128, 0x00);
dec_test!(bid128_minnum_mag_002, bid128_minnum_mag, 0x0000090000a20000810002401188e008u128, 0xfbeedeacf6faf7daca40ce2042238080u128, 0x0000090000a20000810002401188e008u128, 0x00);
dec_test!(bid128_minnum_mag_003, bid128_minnum_mag, 0x0000800000000010dfb7ffdfffffff7fu128, 0x0002240000010584edff77bfefdbfdffu128, 0x0000800000000010dfb7ffdfffffff7fu128, 0x00);
dec_test!(bid128_minnum_mag_004, bid128_minnum_mag, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_mag_005, bid128_minnum_mag, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_006, bid128_minnum_mag, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_mag_007, bid128_minnum_mag, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_mag_008, bid128_minnum_mag, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_009, bid128_minnum_mag, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_010, bid128_minnum_mag, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_011, bid128_minnum_mag, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_012, bid128_minnum_mag, 0x001c000061000005ffffffffffffffffu128, 0x00120009b100014b14940ba296c0b000u128, 0x00120009b100014b14940ba296c0b000u128, 0x00);
dec_test!(bid128_minnum_mag_013, bid128_minnum_mag, 0x08380000000000000000000000000000u128, 0x0e6495e998f0e7632ad98fe54ccbd0c9u128, 0x08380000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_014, bid128_minnum_mag, 0x09101000582780e8ff6ffc8f7ffdb3beu128, 0x004a49800c50ec400000000000000000u128, 0x004a49800c50ec400000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_015, bid128_minnum_mag, 0x14019400e2110802760388a9ddeca130u128, 0xbbffffffffffffff904c01892209498eu128, 0xbbfe0000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_016, bid128_minnum_mag, "-1E+368"                             , "1E+368"                              , 0xb3200000000000000000000000000001u128, 0x00);
dec_test!(bid128_minnum_mag_017, bid128_minnum_mag, "1E+368"                              , "-1E+368"                             , 0xb3200000000000000000000000000001u128, 0x00);
dec_test!(bid128_minnum_mag_018, bid128_minnum_mag, 0x2dd00000000000000000000000000000u128, 0x560e023e84c82e2877e8d6dbc599a595u128, 0x2dd00000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_019, bid128_minnum_mag, 0x3ffb7ff7fbffbdf79e97fffeff7e5ffdu128, 0x00000800000020097fee5fd77ee97dd6u128, 0x00000800000020097fee5fd77ee97dd6u128, 0x00);
dec_test!(bid128_minnum_mag_020, bid128_minnum_mag, 0x4a29fdd3add4507ab6884d65b20ee626u128, 0xed47f4a2ed981eb5fe78fff8fbfd7fedu128, 0x4a280000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_021, bid128_minnum_mag, 0x54bf6c1bec4051ed5270a069ab2f5d6eu128, 0xcd13a7b99da396da28ded6d9e9b064d7u128, 0xcd13a7b99da396da28ded6d9e9b064d7u128, 0x00);
dec_test!(bid128_minnum_mag_022, bid128_minnum_mag, 0x54bfe027e57ffeae0020008000000000u128, 0x54ef062f8e6ec8742c00561a6d69014cu128, 0x54bfe027e57ffeae0020008000000000u128, 0x00);
dec_test!(bid128_minnum_mag_023, bid128_minnum_mag, 0x73e403cd92524802ffdffff7eef7f578u128, 0x04d0042382040a113b28bdf7e0acf044u128, 0x4f900000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_024, bid128_minnum_mag, 0x7a7d77d79bcb7ac54040008000040002u128, 0x24a21000054005413fbb6efbf9efbdffu128, 0x24a21000054005413fbb6efbf9efbdffu128, 0x00);
dec_test!(bid128_minnum_mag_025, bid128_minnum_mag, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_mag_026, bid128_minnum_mag, 0x7c003fffffffffff38c15b08ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_027, bid128_minnum_mag, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_028, bid128_minnum_mag, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_029, bid128_minnum_mag, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00);
dec_test!(bid128_minnum_mag_030, bid128_minnum_mag, 0x7c003fffffffffff38c15b0affffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_031, bid128_minnum_mag, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_032, bid128_minnum_mag, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c003fffffffffff38c15b0affffffffu128, 0x7c000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_033, bid128_minnum_mag, 0x80c8339dda223cb05a365edd5da934fau128, 0x8158505728f286b69ebd240393503610u128, 0x80c8339dda223cb05a365edd5da934fau128, 0x00);
dec_test!(bid128_minnum_mag_034, bid128_minnum_mag, 0x8cacc47873ddc3447ba430d5ad2201a0u128, 0xd720feb38561656742f63de744ed22b0u128, 0x8cacc47873ddc3447ba430d5ad2201a0u128, 0x00);
dec_test!(bid128_minnum_mag_035, bid128_minnum_mag, 0x8eaa0c92f3eb92b8bb59bd398219fae6u128, 0xfffffffbfdfd7fbffffffffffefbdbffu128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_minnum_mag_036, bid128_minnum_mag, 0x93b6fb3240336f431418bcd005435397u128, 0x11d0e3034b009131ff0d21a63dfee067u128, 0x11d0e3034b009131ff0d21a63dfee067u128, 0x00);
dec_test!(bid128_minnum_mag_037, bid128_minnum_mag, 0xab8dc8c6ab487b28696d44b545efab1bu128, 0x4d180000000000000000000000000000u128, 0x4d180000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_038, bid128_minnum_mag, 0xaeded18284d2f520f8fe7c087c6fc392u128, 0xc1bf74edcb6a9910aaeae4cdf8f8521bu128, 0xaeded18284d2f520f8fe7c087c6fc392u128, 0x00);
dec_test!(bid128_minnum_mag_039, bid128_minnum_mag, 0xcabd42170215e9bd4c1d830f132661c3u128, 0x570ccd8da603bfd042e6b693917ffb46u128, 0xcabd42170215e9bd4c1d830f132661c3u128, 0x00);
dec_test!(bid128_minnum_mag_040, bid128_minnum_mag, 0xcb6404c5613c11d45a303248435ee06eu128, 0x21f9cabbd92950a238fab390fae2c7f7u128, 0x21f9cabbd92950a238fab390fae2c7f7u128, 0x00);
dec_test!(bid128_minnum_mag_041, bid128_minnum_mag, 0xd3420083088d9e171910200689060010u128, 0x634cf4dedb29090085c80bacac675afdu128, 0x0d320000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_042, bid128_minnum_mag, 0xdcce5b6605bb31aa51b2488103b952eeu128, 0xdca2701d5c075220020280000004a002u128, 0xdca2701d5c075220020280000004a002u128, 0x00);
dec_test!(bid128_minnum_mag_043, bid128_minnum_mag, 0xde160000000000000000000000000000u128, 0x25fe36562e4193c6056e01ccaec5e99du128, 0xde160000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_044, bid128_minnum_mag, 0xf9ddddddafd6b7eb4191828030714083u128, 0xf84fc15c6f29464811fc3c799a25713au128, 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_045, bid128_minnum_mag, 0xffd7dbf6fdf797effe97ffefffffffdfu128, 0x1acfe5e77ec8d6c3d48944eb4f933f97u128, 0xfc001bf6fdf797effe97ffefffffffdfu128, 0x01);
dec_test!(bid128_minnum_mag_046, bid128_minnum_mag, 0xffffffffffffffff886eb9414b0ee367u128, 0xe4fd782140bcac5ade8408ad4d60e62du128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128_minnum_mag_047, bid128_minnum_mag, "-Infinity"                           , "0"                                   , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_048, bid128_minnum_mag, "Infinity"                            , "-0"                                  , 0xb0400000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_049, bid128_minnum_mag, "Infinity"                            , "QNaN"                                , 0x78000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_050, bid128_minnum_mag, "-Infinity"                           , "QNaN"                                , 0xf8000000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_051, bid128_minnum_mag, "Infinity"                            , "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_minnum_mag_052, bid128_minnum_mag, "QNaN"                                , "0"                                   , 0x30400000000000000000000000000000u128, 0x00);
dec_test!(bid128_minnum_mag_053, bid128_minnum_mag, "QNaN"                                , "SNaN"                                , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_minnum_mag_054, bid128_minnum_mag, "SNaN"                                , "-0"                                  , 0x7c000000000000000000000000000000u128, 0x01);
dec_test!(bid128_minnum_mag_055, bid128_minnum_mag, "SNaN"                                , "-Infinity"                           , 0x7c000000000000000000000000000000u128, 0x01);