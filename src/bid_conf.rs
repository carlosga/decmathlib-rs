/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_snake_case)]

#[cfg(target_endian = "big")]
use crate::d128::{BID_UINT128, BID_UINT64};

#[cfg(target_endian = "big")]
pub (crate) const BID_HIGH_128W: usize = 0;

#[cfg(target_endian = "big")]
pub (crate) const BID_LOW_128W: usize = 1;

#[cfg(target_endian = "little")]
pub (crate) const BID_HIGH_128W: usize = 1;

#[cfg(target_endian = "little")]
pub (crate) const BID_LOW_128W: usize = 0;

#[cfg(target_endian = "big")]
pub (crate) fn BID_SWAP128(x: &mut BID_UINT128) {
    let sw: BID_UINT64 = x.w[1];
    x.w[1] = x.w[0];
    x.w[0] = sw;
}