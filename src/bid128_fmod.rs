/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid_decimal_data::{BID_ESTIMATE_DECIMAL_DIGITS, BID_POWER10_INDEX_BINEXP_128, BID_POWER10_TABLE_128};
use crate::bid_div_macros::bid___div_128_by_128;
use crate::bid_internal::*;
use crate::d128::{_IDEC_flags, StatusFlags};

/// Computes the decimal floating point remainder of the division operation x / y.
pub (crate) fn bid128_fmod(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let P256: BID_UINT256;
    let mut CX: BID_UINT128 = BID_UINT128::default();
    let mut CY: BID_UINT128 = BID_UINT128::default();
    let CR: BID_UINT128;
    let mut T: &BID_UINT128;
    let mut CXS: BID_UINT128;
    let mut P128: BID_UINT128 = BID_UINT128::default();
    let mut res: BID_UINT128 = BID_UINT128::default();
    let mut sign_x: BID_UINT64 = 0;
    let mut sign_y: BID_UINT64 = 0;
    let mut D: BID_SINT64;
    let mut f64: BID_UI32FLOAT = BID_UI32FLOAT::default();
    let mut fx: BID_UI32FLOAT = BID_UI32FLOAT::default();
    let mut exponent_x: i32 = 0;
    let mut exponent_y: i32 = 0;
    let mut diff_expon: i32;
    let mut bin_expon_cx: i32;
    let mut scale: i32;
    let mut scale0: i32;

    // unpack arguments, check for NaN or Infinity

    let valid_y: BID_UINT64 = unpack_BID128_value(&mut sign_y, &mut exponent_y, &mut CY, y);

    if unpack_BID128_value(&mut sign_x, &mut exponent_x, &mut CX, x) == 0 {
        if (y.w[1] & SNAN_MASK64) == SNAN_MASK64 { // y is sNaN
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
        }
        // test if x is NaN
        if (x.w[1] & 0x7c00000000000000u64) == 0x7c00000000000000u64 {
            if (x.w[1] & SNAN_MASK64) == SNAN_MASK64 { // y is sNaN
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            res.w[1] = CX.w[1] & QUIET_MASK64;
            res.w[0] = CX.w[0];
            return res;
        }
        // x is Infinity?
        if (x.w[1] & 0x7800000000000000u64) == 0x7800000000000000u64 {
            // check if y is Inf.
            if (y.w[1] & 0x7c00000000000000u64) != 0x7c00000000000000u64 { // return NaN
                // set status flags
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
                res.w[1] = 0x7c00000000000000u64;
                res.w[0] = 0;
                return res;
            }
        }
        // x is 0
        if (CY.w[1] == 0) && (CY.w[0] == 0) {
            // set status flags
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            // x=y=0, return NaN
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0;
            return res;
        }
        if valid_y != 0 || ((y.w[1] & NAN_MASK64) == INFINITY_MASK64) {
            // return 0
            if (exponent_x > exponent_y) && ((y.w[1] & NAN_MASK64) != INFINITY_MASK64) {
                exponent_x = exponent_y;
            }

            res.w[1] = sign_x | ((exponent_x as BID_UINT64) << 49);
            res.w[0] = 0;
            return res;
        }
    }
    if valid_y == 0 {
        // y is Inf. or NaN

        // test if y is NaN
        if (y.w[1] & 0x7c00000000000000u64) == 0x7c00000000000000u64 {
            if (y.w[1] & SNAN_MASK64) == SNAN_MASK64 {    // y is sNaN
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            res.w[1] = CY.w[1] & QUIET_MASK64;
            res.w[0] = CY.w[0];
            return res;
        }
        // y is Infinity?
        if (y.w[1] & 0x7800000000000000u64) == 0x7800000000000000u64 {
            // return x
            res.w[1] = x.w[1];
            res.w[0] = x.w[0];
            return res;
        }
        // y is 0
        // set status flags
        __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
        res.w[1] = 0x7c00000000000000u64;
        res.w[0] = 0;
        return res;
    }

    diff_expon = exponent_x - exponent_y;

    if diff_expon <= 0 {
        diff_expon = -diff_expon;

        if diff_expon > 34 {
            // |x|<|y| in this case
            res = *x;
            return res;
        }
        // set exponent of y to exponent_x, scale coefficient_y
        T    = &BID_POWER10_TABLE_128[diff_expon as usize];
        P256 = __mul_128x128_to_256(&CY, T);

        if P256.w[2] != 0 || P256.w[3] != 0 {
            // |x|<|y| in this case
            res = *x;
            return res;
        }

        if __unsigned_compare_gt_256_128(&P256, &CX) {
            // |x|<|y| in this case
            res = *x;
            return res;
        }

        P128.w[0] = P256.w[0];
        P128.w[1] = P256.w[1];
        (_, CR)   = bid___div_128_by_128(&CX, &P128);

        res = bid_get_BID128_very_fast(sign_x, exponent_x, &CR);
        return res;
    }
    // 2^64
    f64.ui32  = 0x5f800000;
    scale0 = 38;

    if CY.w[1] == 0 {
        scale0 = 34;
    }

    unsafe {
        while diff_expon > 0 {
            // get number of digits in CX and scale=38-digits
            // fx ~ CX
            fx.d         = (CX.w[1] as f32) * f64.d + (CX.w[0] as f32);
            bin_expon_cx = (((fx.ui32 >> 23) & 0xff) - 0x7f) as i32;
            scale        = scale0 - BID_ESTIMATE_DECIMAL_DIGITS[bin_expon_cx as usize];
            // scale = 38-estimate_decimal_digits[bin_expon_cx];
            D            = (CX.w[1] - BID_POWER10_INDEX_BINEXP_128[bin_expon_cx as usize].w[1]) as BID_SINT64;
            if D > 0 || (D == 0 && CX.w[0] >= BID_POWER10_INDEX_BINEXP_128[bin_expon_cx as usize].w[0]) {
                scale -= 1;
            }

            if diff_expon >= scale {
                diff_expon -= scale;
            } else {
                scale = diff_expon;
                diff_expon = 0;
            }

            T       = &BID_POWER10_TABLE_128[scale as usize];
            CXS     = __mul_128x128_low(&CX, T);
            (_, CX) = bid___div_128_by_128(&CXS, &CY);

            // check for remainder == 0
            if CX.w[1] == 0 && CX.w[0] == 0 {
                res = bid_get_BID128_very_fast(sign_x, exponent_y, &CX);
                return res;
            }
        }
    }

    res = bid_get_BID128_very_fast(sign_x, exponent_y, &CX);
    res
}
