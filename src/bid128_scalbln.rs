/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(overflowing_literals)]

use crate::bid128_scalbn::bid128_scalbn;
use crate::bid_internal::BID_UINT128;
use crate::d128::{_IDEC_flags, RoundingMode};

/// Returns x * 10^N
pub (crate) fn bid128_scalbln(x: &BID_UINT128, n: i64, rnd_mode: RoundingMode, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut n1: i32 = n as i32;
    n1 = match n1 as i64 {
        val if val < n => 0x7fffffffi32,
        val if val > n => 0x80000000i32,
        _              => n1
    };

    bid128_scalbn(x, n1, rnd_mode, pfpsf)
}
