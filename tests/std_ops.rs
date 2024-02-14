/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */

macro_rules! std_op_test {
    ($name: ident, $op: tt, $input1: expr, $input2: expr, $exp: expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = dec1 $op dec2;

            assert_eq!(exp, res1);
        }
    };
}

macro_rules! std_op_ref_test {
    ($name: ident, $op: tt, $input1: expr, $input2: expr, $exp: expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = &dec1 $op &dec2;

            assert_eq!(exp, res1);
        }
    };
}

macro_rules! std_aop_test {
    ($name: ident, $op: tt, $input1: expr, $input2: expr, $exp: expr) => {
        #[test]
        fn $name() {
            let mut dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);

            dec1 $op dec2;

            assert_eq!(exp, dec1);
        }
    };
}

std_op_test!(stc_op_add_001, +, 0x00000000000000005dfecf59bad3acaau128, 0x4014d000d4008a04ffffffddfdfdfeffu128, 0x4014d000d4008a04ffffffddfdfdfeffu128);
std_op_test!(stc_op_add_002, +, 0x0000000000000000cfefffff1f5fb6ebu128, 0x4441ca6997b33fa3a1f5f866809b3dc6u128, 0x4441ca6997b33fa3a1f5f866809b3dc6u128);
std_op_test!(stc_op_add_003, +, 0x0000000000008000004910c400000000u128, 0x5fe5f9ffd9ebcf7f000404e2000600a0u128, 0x0000000000008000004910c400000000u128);
std_op_test!(std_op_sub_001, -, 0x0000000000000000572997bd3b8d4dc5u128, 0x00020000000000005e53d92fd7bfde90u128, 0x8000000000000003581ce42131f163dbu128);
std_op_test!(std_op_sub_002, -, 0x00004000000000000004000840000400u128, 0x00200000000000000603d836132873c6u128, 0x800095b244d383528ff810d5e245fc00u128);
std_op_test!(std_op_sub_003, -, 0x000040800080001000c0004003044a00u128, 0x00004e208a0002002000000200000000u128, 0x80000da0898001f01f3fffc1fcfbb600u128);
std_op_test!(std_op_div_001, /, 0x0000000000000000a4e0e3a5011dfdb3u128, 0x00000000000000000000000000080000u128, 0x301a0b2c2f82b26a5d8dedadcff7d6a7u128);
std_op_test!(std_op_div_002, /, 0x00000000001000000000000010000000u128, 0x1f90612e201990adffdfbfffffffd7ffu128, 0x105de3d7592c3b3a7670b2eea8801aadu128);
std_op_test!(std_op_div_003, /, 0xb0457561041f2a538564461e0cc0c855u128, 0xb030000000000000dd77b6811f136c62u128, 0x302ee9f84615c4b2f831147e0365dac4u128);
std_op_test!(std_op_mul_001, *, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128);
std_op_test!(std_op_mul_002, *, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128);
std_op_test!(std_op_mul_003, *, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128);
std_op_test!(std_op_rem_001, %, 0x00000080024520145e7ff56d67eb67a5u128, 0x000000c100000020d277330430a0ee20u128, 0x80000040fdbae00c73f73d96c8b5867bu128);
std_op_test!(std_op_rem_002, %, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128);
std_op_test!(std_op_rem_003, %, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128);

std_op_ref_test!(stc_op_ref_add_001, +, 0x00000000000000005dfecf59bad3acaau128, 0x4014d000d4008a04ffffffddfdfdfeffu128, 0x4014d000d4008a04ffffffddfdfdfeffu128);
std_op_ref_test!(stc_op_ref_add_002, +, 0x0000000000000000cfefffff1f5fb6ebu128, 0x4441ca6997b33fa3a1f5f866809b3dc6u128, 0x4441ca6997b33fa3a1f5f866809b3dc6u128);
std_op_ref_test!(stc_op_ref_add_003, +, 0x0000000000008000004910c400000000u128, 0x5fe5f9ffd9ebcf7f000404e2000600a0u128, 0x0000000000008000004910c400000000u128);
std_op_ref_test!(std_op_ref_sub_001, -, 0x0000000000000000572997bd3b8d4dc5u128, 0x00020000000000005e53d92fd7bfde90u128, 0x8000000000000003581ce42131f163dbu128);
std_op_ref_test!(std_op_ref_sub_002, -, 0x00004000000000000004000840000400u128, 0x00200000000000000603d836132873c6u128, 0x800095b244d383528ff810d5e245fc00u128);
std_op_ref_test!(std_op_ref_sub_003, -, 0x000040800080001000c0004003044a00u128, 0x00004e208a0002002000000200000000u128, 0x80000da0898001f01f3fffc1fcfbb600u128);
std_op_ref_test!(std_op_ref_div_001, /, 0x0000000000000000a4e0e3a5011dfdb3u128, 0x00000000000000000000000000080000u128, 0x301a0b2c2f82b26a5d8dedadcff7d6a7u128);
std_op_ref_test!(std_op_ref_div_002, /, 0x00000000001000000000000010000000u128, 0x1f90612e201990adffdfbfffffffd7ffu128, 0x105de3d7592c3b3a7670b2eea8801aadu128);
std_op_ref_test!(std_op_ref_div_003, /, 0xb0457561041f2a538564461e0cc0c855u128, 0xb030000000000000dd77b6811f136c62u128, 0x302ee9f84615c4b2f831147e0365dac4u128);
std_op_ref_test!(std_op_ref_mul_001, *, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128);
std_op_ref_test!(std_op_ref_mul_002, *, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128);
std_op_ref_test!(std_op_ref_mul_003, *, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128);
std_op_ref_test!(std_op_ref_rem_001, %, 0x00000080024520145e7ff56d67eb67a5u128, 0x000000c100000020d277330430a0ee20u128, 0x80000040fdbae00c73f73d96c8b5867bu128);
std_op_ref_test!(std_op_ref_rem_002, %, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128);
std_op_ref_test!(std_op_ref_rem_003, %, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128);

std_aop_test!(std_aop_add_001, +=, 0x00000000000000005dfecf59bad3acaau128, 0x4014d000d4008a04ffffffddfdfdfeffu128, 0x4014d000d4008a04ffffffddfdfdfeffu128);
std_aop_test!(std_aop_add_002, +=, 0x0000000000000000cfefffff1f5fb6ebu128, 0x4441ca6997b33fa3a1f5f866809b3dc6u128, 0x4441ca6997b33fa3a1f5f866809b3dc6u128);
std_aop_test!(std_aop_add_003, +=, 0x0000000000008000004910c400000000u128, 0x5fe5f9ffd9ebcf7f000404e2000600a0u128, 0x0000000000008000004910c400000000u128);
std_aop_test!(std_aop_sub_001, -=, 0x0000000000000000572997bd3b8d4dc5u128, 0x00020000000000005e53d92fd7bfde90u128, 0x8000000000000003581ce42131f163dbu128);
std_aop_test!(std_aop_sub_002, -=, 0x00004000000000000004000840000400u128, 0x00200000000000000603d836132873c6u128, 0x800095b244d383528ff810d5e245fc00u128);
std_aop_test!(std_aop_sub_003, -=, 0x000040800080001000c0004003044a00u128, 0x00004e208a0002002000000200000000u128, 0x80000da0898001f01f3fffc1fcfbb600u128);
std_aop_test!(std_aop_div_001, /=, 0x0000000000000000a4e0e3a5011dfdb3u128, 0x00000000000000000000000000080000u128, 0x301a0b2c2f82b26a5d8dedadcff7d6a7u128);
std_aop_test!(std_aop_div_002, /=, 0x00000000001000000000000010000000u128, 0x1f90612e201990adffdfbfffffffd7ffu128, 0x105de3d7592c3b3a7670b2eea8801aadu128);
std_aop_test!(std_aop_div_003, /=, 0xb0457561041f2a538564461e0cc0c855u128, 0xb030000000000000dd77b6811f136c62u128, 0x302ee9f84615c4b2f831147e0365dac4u128);
std_aop_test!(std_aop_mul_001, *=, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128);
std_aop_test!(std_aop_mul_002, *=, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x00000000000000000000000000000000u128);
std_aop_test!(std_aop_mul_003, *=, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x7c003fffffffffff38c15b08ffffffffu128, 0x7c000000000000000000000000000000u128);
std_aop_test!(std_aop_rem_001, %=, 0x00000080024520145e7ff56d67eb67a5u128, 0x000000c100000020d277330430a0ee20u128, 0x80000040fdbae00c73f73d96c8b5867bu128);
std_aop_test!(std_aop_rem_002, %=, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x00000000000000000000000000000000u128);
std_aop_test!(std_aop_rem_003, %=, 0x0001ed09bead87c0378d8e62ffffffffu128, 0x0001ed09bead87c0378d8e64ffffffffu128, 0x7c000000000000000000000000000000u128);
