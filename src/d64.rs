/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

#![allow(non_camel_case_types)]

use crate::d128::BID_UINT64;

#[derive(Copy, Clone, Debug, Default)]
pub struct d64(pub BID_UINT64);