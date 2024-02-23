/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid_internal::BID_UINT64;
use crate::constants::{MASK_ANY_INF, MASK_INF, MASK_STEERING_BITS, NAN_MASK64, QUIET_MASK64};
use crate::bid_internal::BID_UINT128;

/// The quantumdN functions compute the quantum of a finite argument.
/// If x is infinite, the result is +Inf. If x is NaN, the result is NaN.
pub (crate) fn bid128_quantum(x: &BID_UINT128) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::default();

    // If x is infinite, the result is +Inf. If x is NaN, the result is NaN
    if (x.w[1] & MASK_ANY_INF) == MASK_INF {
        res.w[1] = 0x7800000000000000u64;
        res.w[0] = 0x0000000000000000u64;
        return res;
    } else if (x.w[1] & NAN_MASK64) == NAN_MASK64 {
        res.w[1] = x.w[1] & QUIET_MASK64;
        return res;
    }

    // Extract exponent
    let int_exp = if (x.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS {
        (((x.w[1] >> 47) & 0x3fff) as i32) - 6176
    } else {
        (((x.w[1] >> 49) & 0x3fff) as i32) - 6176
    };

    // Form 10^new_exponent*1
    res.w[1] = (((int_exp as i64) << 49 ) + 0x3040000000000000i64) as BID_UINT64;
    res.w[0] = 0x0000000000000001u64;

    res
}
