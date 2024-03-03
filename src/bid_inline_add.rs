/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid_decimal_data::{BID_ESTIMATE_DECIMAL_DIGITS, BID_POWER10_TABLE_128};
use crate::bid_internal::{__unsigned_compare_ge_128, BID_UI64DOUBLE, BID_UINT128, MASK_BINARY_EXPONENT};

/// return number of decimal digits in 128-bit value X
pub (crate) fn __get_dec_digits64(X: &BID_UINT128) -> i32 {
    let mut tempx: BID_UI64DOUBLE = BID_UI64DOUBLE:: default();
    let mut digits_x: i32;
    let bin_expon_cx: usize;

    if X.w[1] == 0 {
        if X.w[0] == 0 {
            return 0;
        }
        unsafe {
            //--- get number of bits in the coefficients of x and y ---
            tempx.d      = X.w[0] as f64;
            bin_expon_cx = (((tempx.ui64 & MASK_BINARY_EXPONENT) >> 52) - 0x3ff) as usize;
        }
        // get number of decimal digits in the coeff_x
        digits_x = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon_cx];
        if X.w[0] >= BID_POWER10_TABLE_128[digits_x as usize].w[0] {
            digits_x += 1;
        }
        return digits_x;
    }
    unsafe {
        tempx.d      = X.w[1] as f64;
        bin_expon_cx = (((tempx.ui64 & MASK_BINARY_EXPONENT) >> 52) - 0x3ff) as usize;
    }
    // get number of decimal digits in the coeff_x
    digits_x = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon_cx + 64];
    if __unsigned_compare_ge_128(X, &BID_POWER10_TABLE_128[digits_x as usize]) {
        digits_x += 1;
    }
    digits_x
}
