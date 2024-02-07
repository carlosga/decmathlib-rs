/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_from_uint64_001, bid128_from_uint64, 12838043691698676900u64, 0x3040000000000000b229e4b7fea4e4a4u128);
dec_test!(bid128_from_uint64_002, bid128_from_uint64, 13039381436817401260u64, 0x3040000000000000b4f53054c852e5acu128);
dec_test!(bid128_from_uint64_003, bid128_from_uint64, 14329515802341157305u64, 0x3040000000000000c6dcaab381e429b9u128);
dec_test!(bid128_from_uint64_004, bid128_from_uint64, 14891321118899701533u64, 0x3040000000000000cea899a8f84a071du128);
dec_test!(bid128_from_uint64_005, bid128_from_uint64, 15299397707630953249u64, 0x3040000000000000d4526127c8c4c321u128);
dec_test!(bid128_from_uint64_006, bid128_from_uint64, 155043678304746152u64  , 0x30400000000000000226d3676a063aa8u128);
dec_test!(bid128_from_uint64_007, bid128_from_uint64, 15813025721173857557u64, 0x3040000000000000db73271cc6aed515u128);
dec_test!(bid128_from_uint64_008, bid128_from_uint64, 15951128221084643628u64, 0x3040000000000000dd5dca9ab875712cu128);
dec_test!(bid128_from_uint64_009, bid128_from_uint64, 16512022100427752653u64, 0x3040000000000000e5267c9dab6544cdu128);
dec_test!(bid128_from_uint64_010, bid128_from_uint64, 17242795393441160067u64, 0x3040000000000000ef4ab70dd8cd5b83u128);
dec_test!(bid128_from_uint64_011, bid128_from_uint64, 17470592975898584298u64, 0x3040000000000000f27403bf97dea4eau128);
dec_test!(bid128_from_uint64_012, bid128_from_uint64, 18144984012892654914u64, 0x3040000000000000fbcfeed2cf9bcd42u128);
dec_test!(bid128_from_uint64_013, bid128_from_uint64, 2714021962226553302u64 , 0x304000000000000025aa24985b1f51d6u128);
dec_test!(bid128_from_uint64_014, bid128_from_uint64, 2858886672715936018u64 , 0x304000000000000027acce482454f112u128);
dec_test!(bid128_from_uint64_015, bid128_from_uint64, 2973717257055240449u64 , 0x30400000000000002944c417013a0101u128);
dec_test!(bid128_from_uint64_016, bid128_from_uint64, 3783918067584438023u64 , 0x304000000000000034832d6f358d8f07u128);
dec_test!(bid128_from_uint64_017, bid128_from_uint64, 6135131516986689308u64 , 0x304000000000000055245d9c004b871cu128);
dec_test!(bid128_from_uint64_018, bid128_from_uint64, 6826231342173745096u64 , 0x30400000000000005ebba53d20e76fc8u128);
dec_test!(bid128_from_uint64_019, bid128_from_uint64, 8530537025811641231u64 , 0x304000000000000076628e3a6ed1f38fu128);
dec_test!(bid128_from_uint64_020, bid128_from_uint64, 9069128140613606892u64 , 0x30400000000000007ddc03fe5bc5ddecu128);
