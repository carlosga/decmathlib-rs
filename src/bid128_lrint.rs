/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(unused_assignments)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::bid128_to_int64::{bid128_to_int64_xceil, bid128_to_int64_xfloor, bid128_to_int64_xint, bid128_to_int64_xrnint, bid128_to_int64_xrninta};
use crate::core::RoundingMode;
use crate::d128::{_IDEC_flags, BID_UINT128};

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

