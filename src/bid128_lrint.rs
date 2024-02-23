/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid128_to_int64::{bid128_to_int64_xceil, bid128_to_int64_xfloor, bid128_to_int64_xint, bid128_to_int64_xrnint, bid128_to_int64_xrninta};
use crate::bid_internal::BID_UINT128;
use crate::d128::{_IDEC_flags, RoundingMode};

/// DESCRIPTION:
///   The lrint function rounds its argument to the nearest integer value of
///   type long int, rounding according to the current rounding direction.
/// RETURN VALUE:
///   If the rounded value is outside the range of the return type or the
///   argument is infinity or NaN, the result is the largest negative value
///   and the invalid exception is signaled
/// EXCEPTIONS SIGNALED:
///   invalid and inexact
pub (crate) fn bid128_lrint(x: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> i64 {
    match rnd_mode {
        RoundingMode::BID_ROUNDING_TO_NEAREST => bid128_to_int64_xrnint(x, pfpsf),
        RoundingMode::BID_ROUNDING_TIES_AWAY  => bid128_to_int64_xrninta(x, pfpsf),
        RoundingMode::BID_ROUNDING_DOWN       => bid128_to_int64_xfloor(x, pfpsf),
        RoundingMode::BID_ROUNDING_UP         => bid128_to_int64_xceil(x, pfpsf),
        _                                     => bid128_to_int64_xint(x, pfpsf)
    }
}

