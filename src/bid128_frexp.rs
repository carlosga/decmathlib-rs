/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid128::BID_NR_DIGITS;
use crate::bid_internal::{BID_UI64DOUBLE, BID_UINT128, BID_UINT64, MASK_COEFF, MASK_EXP, MASK_EXP2, MASK_SNAN, MASK_SPECIAL};

/// Decomposes given decimal floating point value num into a normalized fraction and an integral power of two.
pub (crate) fn bid128_frexp(x: &BID_UINT128) -> (BID_UINT128, i32) {
    /*
      If x is not a floating-point number, the results are unspecified (this
      implementation returns x and *exp = 0). Otherwise, the frexp function
      returns the value res, such that res has a magnitude in the interval
      [1/10, 1) or zero, and x = res*2^*exp. If x is zero, both parts of the
      result are zero frexp does not raise any exceptions
    */

    let mut res: BID_UINT128 = Default::default();
    let mut sig_x: BID_UINT128 = Default::default();
    let exp_x: u32;
    let mut tmp: BID_UI64DOUBLE = Default::default();
    let x_nr_bits: usize;
    let mut q: i32;
    let exp: i32;

    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // if NaN or infinity
        exp = 0;
        res = *x;
        // the binary frexp quitetizes SNaNs, so do the same
        if (x.w[1] & MASK_SNAN) == MASK_SNAN { // x is SNAN
            //   // set invalid flag
            //   *pfpsf |= BID_INVALID_EXCEPTION;
            // return quiet (x)
            res.w[1] = x.w[1] & 0xfdffffffffffffffu64;
        }
        (res, exp)
    } else {
        // x is 0, non-canonical, normal, or subnormal
        // check for non-canonical values with 114 bit-significands; can be zero too
        if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            exp      = 0;
            exp_x    = ((x.w[1] & MASK_EXP2) >> 47) as u32; // biased
            res.w[1] = (x.w[1] & 0x8000000000000000u64) | ((exp_x as BID_UINT64) << 49);
            // zero of same sign
            res.w[0] = 0x0000000000000000u64;
            return (res, exp);
        }
        // unpack x
        exp_x      = ((x.w[1] & MASK_EXP) >> 49) as u32; // biased
        sig_x.w[1] = x.w[1] & MASK_COEFF;
        sig_x.w[0] = x.w[0];
        // check for non-canonical values or zero
        if  (sig_x.w[1]  > 0x0001ed09bead87c0u64)
         || (sig_x.w[1] == 0x0001ed09bead87c0u64
         && (sig_x.w[0]  > 0x378d8e63ffffffffu64))
        || ((sig_x.w[1] == 0x0u64) && (sig_x.w[0] == 0x0u64)) {
            exp      = 0;
            res.w[1] = (x.w[1] & 0x8000000000000000u64) | ((exp_x as BID_UINT64) << 49);
            // zero of same sign
            res.w[0] = 0x0000000000000000u64;
            return (res, exp);
        } else {
            // continue, x is neither zero nor non-canonical
        }
        // x is normal or subnormal, with exp_x=biased exponent & sig_x=coefficient
        // determine the number of decimal digits in sig_x, which fits in 113 bits
        // q = nr. of decimal digits in sig_x (1 <= q <= 34)
        //  determine first the nr. of bits in sig_x
        unsafe {
            if sig_x.w[1] == 0 {
                if sig_x.w[0] >= 0x0020000000000000u64 { // z >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    if sig_x.w[0] >= 0x0000000100000000u64 { // z >= 2^32
                        tmp.d     = (sig_x.w[0] >> 32) as f64; // exact conversion
                        x_nr_bits = (32 + ((((tmp.ui64 >> 52) as u32) & 0x7ff) - 0x3ff)) as usize;
                    } else { // z < 2^32
                        tmp.d     = sig_x.w[0] as f64; // exact conversion
                        x_nr_bits = ((((tmp.ui64 >> 52) as u32) & 0x7ff) - 0x3ff) as usize;

                    }
                } else { // if z < 2^53
                    tmp.d     = sig_x.w[0] as f64; // exact conversion
                    x_nr_bits = ((((tmp.ui64 >> 52) as u32) & 0x7ff) - 0x3ff) as usize;
                }
            } else { // sig_x.w[1] != 0 => nr. bits = 65 + nr_bits (sig_x.w[1])
                tmp.d     = sig_x.w[1] as f64; // exact conversion
                x_nr_bits = (64 + ((((tmp.ui64 >> 52) as u32) & 0x7ff) - 0x3ff)) as usize;
            }
        }
        q = BID_NR_DIGITS[x_nr_bits].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[x_nr_bits].digits1 as i32;
            if  sig_x.w[1]  > BID_NR_DIGITS[x_nr_bits].threshold_hi
            || (sig_x.w[1] == BID_NR_DIGITS[x_nr_bits].threshold_hi
             && sig_x.w[0] >= BID_NR_DIGITS[x_nr_bits].threshold_lo) {
                q += 1;
            }
        }
        // Do not add trailing zeros if q < 34; leave sig_x with q digits
        exp      = (exp_x - 6176 + (q as u32)) as i32;
        // assemble the result; sig_x < 2^113 so it fits in 113 bits
        res.w[1] = (x.w[1] & 0x8001ffffffffffffu64) | ((((-q as i64) + 6176i64) << 49) as BID_UINT64);
        res.w[0] = x.w[0];
        // replace exponent
        (res, exp)
    }
}
