/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

mod common;

dec_test!(bid128dqd_fma_001, bid128dqd_fma, 0, 0x0086160000000020u64, 0xb1391009800a0042c000908a04042868u128, 0x0ffcc8873e852186u64, 0xae43d2029d2629774758ed4174e92652u128, 0x20);
dec_test!(bid128dqd_fma_002, bid128dqd_fma, 0, 0x0200000000000221u64, 0xd1b1c5c21b11c1790256eb0280101c44u128, 0x2ec6dd1e23efc638u64, 0xcebaf74c58fe2829a146690a529823e2u128, 0x20);
dec_test!(bid128dqd_fma_003, bid128dqd_fma, 0, 0x0498824b421b570bu64, 0xb0a4203090e084034c69fd547b1ee813u128, 0x8eff36f9b0984713u64, 0xadf041869773c92d493dd6ca4ccf76c8u128, 0x20);
dec_test!(bid128dqd_fma_004, bid128dqd_fma, 0, 0x29880451882a8a88u64, 0xad24beb7771757d1b64b78f5e685f47au128, 0x0030040080001001u64, 0x2d02de42f505a1157cfdf129e763ffffu128, 0x20);
dec_test!(bid128dqd_fma_005, bid128dqd_fma, 0, 0x2d1d42e928a9803au64, 0x0000000000010000ffffffffffffffffu128, 0x67ebb6ff2ff7ffffu64, 0x00000000000000000000000000000064u128, 0x30);
dec_test!(bid128dqd_fma_006, bid128dqd_fma, 0, 0x365fff6945393ad2u64, 0x4000000000000000ffdffffffff7ffffu128, 0x1fffb9ffffefffffu64, 0x404c51dfc6002a6d3a05fa77cf150718u128, 0x20);
dec_test!(bid128dqd_fma_007, bid128dqd_fma, 0, 0x4463ac7236f84442u64, 0x4800000000000200d57d1f27af3fc4bau128, 0x010cba61d9eab2a5u64, 0x4931e24aeec020234b865286e1c65ec1u128, 0x20);
dec_test!(bid128dqd_fma_008, bid128dqd_fma, 0, 0x5fc000000000000au64, 0xaffc0000000000000000000000000005u128, 0x5fe000000000000au64, 0x32e2314dc6448d9338c15b0a00000000u128, 0x20);
dec_test!(bid128dqd_fma_009, bid128dqd_fma, 0, 0x70105daa2a61403bu64, 0xafa7525a66752d34572321e3fb7b7a5cu128, 0xbaf219be0c80e664u64, 0xb0b037f1b4a14284f7f1a6cd8e7cb8a1u128, 0x20);
dec_test!(bid128dqd_fma_010, bid128dqd_fma, 0, 0xdfdafbd7757b5c75u64, 0xb0215e4da4a9882a0f71b9fddb2406ddu128, 0xddafdfeff9bff7bfu64, 0x33210a106e84676b01e6c72ef6b8af2au128, 0x20);
dec_test!(bid128dqd_fma_011, bid128dqd_fma, 0, 0xfdeffffd7afb6fdfu64, 0x04611048200040000000000000000024u128, 0x4778fe9c3de62935u64, 0xfc000000000000000000000000000000u128, 0x00);
dec_test!(bid128dqd_fma_012, bid128dqd_fma, 1, 0x5fc000000000000au64, 0xaffc0000000000000000000000000005u128, 0x5fe000000000000au64, 0x32e1ed09bead87c0378d8e63ffffffffu128, 0x20);
dec_test!(bid128dqd_fma_013, bid128dqd_fma, 2, 0x5fc000000000000au64, 0xaffc0000000000000000000000000005u128, 0x5fe000000000000au64, 0x32e2314dc6448d9338c15b0a00000000u128, 0x20);
dec_test!(bid128dqd_fma_014, bid128dqd_fma, 3, 0x5fc000000000000au64, 0xaffc0000000000000000000000000005u128, 0x5fe000000000000au64, 0x32e1ed09bead87c0378d8e63ffffffffu128, 0x20);
dec_test!(bid128dqd_fma_015, bid128dqd_fma, 4, 0x5fc000000000000au64, 0xaffc0000000000000000000000000005u128, 0x5fe000000000000au64, 0x32e2314dc6448d9338c15b0a00000000u128, 0x20);
