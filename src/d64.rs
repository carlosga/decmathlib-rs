/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_camel_case_types)]

use crate::bid_internal::BID_UINT64;

#[derive(Copy, Clone, Debug, Default)]
#[repr(transparent)]
pub struct d64(pub BID_UINT64);