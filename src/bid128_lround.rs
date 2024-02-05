/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

use crate::bid128_to_int64::{bid128_to_int64_rninta};
use crate::d128::{_IDEC_flags, BID_UINT128};

///  DESCRIPTION:
///    The lround function rounds its argument to the nearest integer value of
///    type long int, using rounding to nearest-away
///  RETURN VALUE:
///    If the rounded value is outside the range of the return type or the
///    argument is infinity or NaN, the result is the largest negative value
///    and the invalid exception is signaled
///  EXCEPTIONS SIGNALED:
///    invalid
///
pub (crate) fn bid128_lround(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> i64 {
// #if BID_SIZE_LONG==4
//   BID_SINT32 res;
//   BIDECIMAL_CALL1_NORND (bid128_to_int32_rninta, res, x);
// #else
    bid128_to_int64_rninta(x, pfpsf)
// #endif
}
