/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_snake_case)]

use crate::bid128_next::bid128_nextafter;
use crate::d128::{_IDEC_flags, BID_UINT128};

/// Returns the next representable value after x in the direction of y.
pub (crate) fn bid128_nexttoward(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
  bid128_nextafter(x, y, pfpsf)
}

