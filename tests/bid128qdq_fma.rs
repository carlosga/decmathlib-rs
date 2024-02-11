/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128qdq_fma_001, bid128qdq_fma, 0, 0x00143000000200009d130a680aaed8d2u128, 0xae9bad38658eacbdu64, 0x0000000000020000caafcf1cf4ef79dfu128, 0x80002564b2da4f62be0ff83ac3167443u128, 0x30);
dec_test!(bid128qdq_fma_002, bid128qdq_fma, 0, 0x0080000008c00000aa10100409acc091u128, 0xa86c71419de03055u64, 0x000035fad313489fe2044242ef1affcau128, 0x0000314dc6448d9338c15b0a00000000u128, 0x30); // underflow_before_only
dec_test!(bid128qdq_fma_003, bid128qdq_fma, 0, 0x020482000800000070910c644250dcabu128, 0x8ebec72b99e22addu64, 0x00001108000000002e662ba6970a70c7u128, 0x00001107ff430d2063fadd44b8d70028u128, 0x30);
dec_test!(bid128qdq_fma_004, bid128qdq_fma, 0, 0x33295dc068a0fd1d0000200000110080u128, 0x10000100004c0010u64, 0xffffffffffffffdf5183b66893080870u128, 0xfc000000000000000000000000000000u128, 0x01);
dec_test!(bid128qdq_fma_005, bid128qdq_fma, 0, 0x80848000408a00009291814b84454224u128, 0xa77ebcf7fde53559u64, 0xe8080227f96f750528414ca031808001u128, 0x00000b131dd820accecc178d827a3f05u128, 0x30);
dec_test!(bid128qdq_fma_006, bid128qdq_fma, 0, 0xbafefffbff7fffffffffffffffffffffu128, 0x0000200000000000u64, 0x5505eeb8127902140005280000040901u128, 0xb7fe5a1105a3c30d4c0ba26d9782e4d5u128, 0x20);
dec_test!(bid128qdq_fma_007, bid128qdq_fma, 1, 0x0080000008c00000aa10100409acc091u128, 0xa86c71419de03055u64, 0x000035fad313489fe2044242ef1affcau128, 0x0000314dc6448d9338c15b09ffffffffu128, 0x30);
dec_test!(bid128qdq_fma_008, bid128qdq_fma, 2, 0x0080000008c00000aa10100409acc091u128, 0xa86c71419de03055u64, 0x000035fad313489fe2044242ef1affcau128, 0x0000314dc6448d9338c15b0a00000000u128, 0x30); // underflow_before_only
dec_test!(bid128qdq_fma_009, bid128qdq_fma, 3, 0x0080000008c00000aa10100409acc091u128, 0xa86c71419de03055u64, 0x000035fad313489fe2044242ef1affcau128, 0x0000314dc6448d9338c15b09ffffffffu128, 0x30);
dec_test!(bid128qdq_fma_010, bid128qdq_fma, 4, 0x0080000008c00000aa10100409acc091u128, 0xa86c71419de03055u64, 0x000035fad313489fe2044242ef1affcau128, 0x0000314dc6448d9338c15b0a00000000u128, 0x30); // underflow_before_only
// TODO: goto delta_ge_zero;
// dec_test!(bid128qdq_fma_011, bid128qdq_fma, 4, 0xdfdb8bb7ba1dc5d58fedf5ebf9deedeeu128, 0x4000000000000001u64, 0x0000000248400184280203008a005208u128, 0xf8000000000000000000000000000000u128, 0x28);
// dec_test!(bid128qdq_fma_012, bid128qdq_fma, 0, "-1e-6078", "5e-100" "1e-6143", "1000000000000000000000000000000000E-6176", 0x30); underflow_before_only
// dec_test!(bid128qdq_fma_013, bid128qdq_fma, 1, "-1e-6078", "5e-100" "1e-6143", " 999999999999999999999999999999999E-6176", 0x30);
// dec_test!(bid128qdq_fma_014, bid128qdq_fma, 2, "-1e-6078", "5e-100" "1e-6143", "1000000000000000000000000000000000E-6176", 0x30); underflow_before_only
// dec_test!(bid128qdq_fma_015, bid128qdq_fma, 3, "-1e-6078", "5e-100" "1e-6143", " 999999999999999999999999999999999E-6176", 0x30);
// dec_test!(bid128qdq_fma_016, bid128qdq_fma, 4, "-1e-6078", "5e-100" "1e-6143", "1000000000000000000000000000000000E-6176", 0x30); underflow_before_only
