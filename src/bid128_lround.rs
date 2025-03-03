/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid128_to_int64::bid128_to_int64_rninta;
use crate::bid_internal::BID_UINT128;
use crate::d128::_IDEC_flags;

///  DESCRIPTION:
///    The lround function rounds its argument to the nearest integer value of
///    type long int, using rounding to nearest-away
///  RETURN VALUE:
///    If the rounded value is outside the range of the return type or the
///    argument is infinity or NaN, the result is the largest negative value
///    and the invalid exception is signaled
///  EXCEPTIONS SIGNALED:
///    invalid
pub (crate) fn bid128_lround(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> i64 {
// #if BID_SIZE_LONG==4
//   BID_SINT32 res;
//   BIDECIMAL_CALL1_NORND (bid128_to_int32_rninta, res, x);
// #else
    bid128_to_int64_rninta(x, pfpsf)
// #endif
}
