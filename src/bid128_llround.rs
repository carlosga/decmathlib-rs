/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid128_to_int64::bid128_to_int64_rninta;
use crate::bid_internal::BID_UINT128;
use crate::d128::_IDEC_flags;

///  DESCRIPTION:
///    The llround function rounds its argument to the nearest integer value of
///    type long int, using rounding to nearest-away
///  RETURN VALUE:
///    If the rounded value is outside the range of the return type or the
///    argument is infinity or NaN, the result is the largest negative value
///    and the invalid exception is signaled
///  EXCEPTIONS SIGNALED:
///    invalid
pub (crate) fn bid128_llround(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> i64 {
    // the sizeof (long long) = 8 (BID_SIZE_LONG==8)
    bid128_to_int64_rninta(x, pfpsf)
}
