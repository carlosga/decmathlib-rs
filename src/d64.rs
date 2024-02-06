/* --------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

#![allow(non_camel_case_types)]

use crate::d128::BID_UINT64;

#[derive(Copy, Clone, Debug, Default)]
pub struct d64(pub BID_UINT64);