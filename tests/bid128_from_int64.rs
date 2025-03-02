/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128_from_int64_001, bid128_from_int64, -1297714641311568497i64 , 0xb0400000000000001202689737331271u128);
dec_test!(bid128_from_int64_002, bid128_from_int64, -1382533918512158604i64 , 0xb040000000000000132fbf461ed8978cu128);
dec_test!(bid128_from_int64_003, bid128_from_int64,  1991200058281483048i64 , 0x30400000000000001ba229e7369adb28u128);
dec_test!(bid128_from_int64_004, bid128_from_int64, -1998782831704824207i64 , 0xb0400000000000001bbd1a653aea598fu128);
dec_test!(bid128_from_int64_005, bid128_from_int64,  2081344241858484150i64 , 0x30400000000000001ce26b8f7f4797b6u128);
dec_test!(bid128_from_int64_006, bid128_from_int64, -3494168784785967941i64 , 0xb040000000000000307dc7ff326d0b45u128);
dec_test!(bid128_from_int64_007, bid128_from_int64, -3623475514305027548i64 , 0xb04000000000000032492bc8427a1ddcu128);
dec_test!(bid128_from_int64_008, bid128_from_int64, -3719337100671242603i64 , 0xb040000000000000339dbd631d70d16bu128);
dec_test!(bid128_from_int64_009, bid128_from_int64,  4134085355817711631i64 , 0x3040000000000000395f38ba50a8b80fu128);
dec_test!(bid128_from_int64_010, bid128_from_int64, -4543384426972292615i64 , 0xb0400000000000003f0d58107fb9a607u128);
dec_test!(bid128_from_int64_011, bid128_from_int64, -456313642285820771i64  , 0xb040000000000000065526d70a9c6b63u128);
dec_test!(bid128_from_int64_012, bid128_from_int64,  572333114425411975i64  , 0x304000000000000007f155ef6a850987u128);
dec_test!(bid128_from_int64_013, bid128_from_int64, -5896812445967530814i64 , 0xb04000000000000051d5afad4cff3b3eu128);
dec_test!(bid128_from_int64_014, bid128_from_int64,  6084829525942336602i64 , 0x30400000000000005471a8370c99fc5au128);
dec_test!(bid128_from_int64_015, bid128_from_int64,  6343298205297819548i64 , 0x30400000000000005807ec1c07263f9cu128);
dec_test!(bid128_from_int64_016, bid128_from_int64, -640590259490231037i64  , 0xb04000000000000008e3d5726f7d92fdu128);
dec_test!(bid128_from_int64_017, bid128_from_int64,  7071067091834809443i64 , 0x304000000000000062217a0e4fc4ac63u128);
dec_test!(bid128_from_int64_018, bid128_from_int64,  7575924962073745274i64 , 0x30400000000000006923179d316b977au128);
dec_test!(bid128_from_int64_019, bid128_from_int64, -8534495841839544214i64 , 0xb04000000000000076709ec01de4f796u128);
dec_test!(bid128_from_int64_020, bid128_from_int64,  945634496444636363i64  , 0x30400000000000000d1f919077f01ccbu128);
