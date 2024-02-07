/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_snake_case)]

use crate::bid_decimal_data::{bid_estimate_decimal_digits, bid_power10_index_binexp_128};
use crate::bid_internal::{__set_status_flags, unpack_BID128_value};
use crate::constants::DECIMAL_EXPONENT_BIAS_128;
use crate::core::StatusFlags;
use crate::d128::{_IDEC_flags, BID_SINT64, BID_UI32FLOAT, BID_UINT128, BID_UINT64};

pub (crate) fn bid128_ilogb(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> i32 {
    let mut CX: BID_UINT128 = BID_UINT128::default();
    let mut sign_x: BID_UINT64 = 0;
    let mut f64: BID_UI32FLOAT = BID_UI32FLOAT::default();
    let mut fx: BID_UI32FLOAT = BID_UI32FLOAT::default();
    let mut exponent_x: i32 = 0;
    let bin_expon_cx: usize;
    let mut digits: i32;

    if unpack_BID128_value(&mut sign_x, &mut exponent_x, &mut CX, x) == 0 {
        __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
        return if (x.w[1] & 0x7c00000000000000u64) == 0x7800000000000000u64 {
            0x7fffffff
        } else {
            0x80000000u32 as i32
        };
    }
    // find number of digits in coefficient
    // 2^64
    f64.i = 0x5f800000;
    // fx ~ CX
    unsafe {
        fx.d         = (CX.w[1] as f32) * f64.d + (CX.w[0] as f32);
        bin_expon_cx = (((fx.i >> 23) & 0xff) - 0x7f) as usize;
    }
    digits = bid_estimate_decimal_digits[bin_expon_cx];
    // scale = 38-estimate_decimal_digits[bin_expon_cx];

    let D: BID_SINT64 = (CX.w[1] - bid_power10_index_binexp_128[bin_expon_cx].w[1]) as BID_SINT64;
    if D > 0 || (D == 0 && CX.w[0] >= bid_power10_index_binexp_128[bin_expon_cx].w[0]) {
        digits += 1;
    }
    exponent_x - DECIMAL_EXPONENT_BIAS_128 - 1 + digits
}