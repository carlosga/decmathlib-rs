/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_snake_case)]

use crate::bid128::{bid_nr_digits, bid_ten2k128, bid_ten2k64};
use crate::bid128_compare::{bid128_quiet_equal, bid128_quiet_greater, bid128_quiet_not_equal};
use crate::bid_conf::{BID_HIGH_128W, BID_LOW_128W};
use crate::bid_internal::{__mul_128x64_to_128, __mul_64x64_to_128MACH};
use crate::constants::{EXP_MIN, EXP_P1, MASK_ANY_INF, MASK_COEFF, MASK_EXP, MASK_INF, MASK_NAN, MASK_SIGN, MASK_SNAN, MASK_SPECIAL, P34};
use crate::core::StatusFlags;
use crate::d128::{_IDEC_flags, BID_UI64DOUBLE, BID_UINT128, BID_UINT64};

/// Returns the least 128-bit decimal floating-point number that
/// compares greater than the operand
pub (crate) fn bid128_nextup(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::default();
    let x_sign: BID_UINT64;
    let mut x_exp: BID_UINT64;
    let exp: i32;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: usize;
    let mut q1: i32;
    let ind: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default(); // C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (BID_UINT64)

    //BID_SWAP128 (x);
    // unpack the argument
    x_sign = x.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        if (x.w[1] & MASK_NAN) == MASK_NAN {
            // x is NAN
            let mut x = *x;
            // if x = NaN, then res = Q (x)
            // check first for non-canonical NaN payload
            if  ((x.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
            || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
              && (x.w[0] > 0x38c15b09ffffffffu64))
            {
                x.w[1] &= 0xffffc00000000000u64;
                x.w[0]  = 0x0u64;
            }
            if (x.w[1] & MASK_SNAN) == MASK_SNAN {
                // x is SNAN
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return quiet (x)
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64; // clear out also G[6]-G[16]
                res.w[0] = x.w[0];
            } else {
                // x is QNaN
                // return x
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64; // clear out G[6]-G[16]
                res.w[0] = x.w[0];
            }
        } else {
            // x is not NaN, so it must be infinity
            if x_sign == 0 {
                // x is +inf
                res.w[1] = 0x7800000000000000u64; // +inf
                res.w[0] = 0x0000000000000000u64;
            } else {
                // x is -inf
                res.w[1] = 0xdfffed09bead87c0u64; // -MAXFP = -999...99 * 10^emax
                res.w[0] = 0x378d8e63ffffffffu64;
            }
        }
        return res;
    }
    // check for non-canonical values (treated as zero)
    if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
        // G0_G1=11
        // non-canonical
        x_exp = (x.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
        C1.w[1] = 0; // significand high
        C1.w[0] = 0; // significand low
    } else {
        // G0_G1 != 11
        x_exp = x.w[1] & MASK_EXP; // biased and shifted left 49 bits
        if  C1.w[1]  > 0x0001ed09bead87c0u64
        || (C1.w[1] == 0x0001ed09bead87c0u64
         && C1.w[0]  > 0x378d8e63ffffffffu64) {
            // x is non-canonical if coefficient is larger than 10^34 -1
            C1.w[1] = 0;
            C1.w[0] = 0;
        } else {
            // canonical
        }
    }

    if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is +/-0
        res.w[1] = 0x0000000000000000u64; // +1 * 10^emin
        res.w[0] = 0x0000000000000001u64;
    } else {
        // x is not special and is not zero
        if x.w[1] == 0x5fffed09bead87c0u64 && x.w[0] == 0x378d8e63ffffffffu64 {
            // x = +MAXFP = 999...99 * 10^emax
            res.w[1] = 0x7800000000000000u64; // +inf
            res.w[0] = 0x0000000000000000u64;
        } else if x.w[1] == 0x8000000000000000u64 && x.w[0] == 0x0000000000000001u64 {
            // x = -MINFP = 1...99 * 10^emin
            res.w[1] = 0x8000000000000000u64; // -0
            res.w[0] = 0x0000000000000000u64;
        } else {
            // -MAXFP <= x <= -MINFP - 1 ulp OR MINFP <= x <= MAXFP - 1 ulp
            // can add/subtract 1 ulp to the significand

            unsafe {
                // Note: we could check here if x >= 10^34 to speed up the case q1 = 34
                // q1 = nr. of decimal digits in x
                // determine first the nr. of bits in x
                if C1.w[1] == 0 {
                    if C1.w[0] >= 0x0020000000000000u64 {
                        // x >= 2^53
                        // split the 64-bit value in two 32-bit halves to avoid rnd errors
                        if C1.w[0] >= 0x0000000100000000u64 {
                            // x >= 2^32
                            tmp1.d    = (C1.w[0] >> 32) as f64; // exact conversion
                            x_nr_bits = (33 + (((tmp1.i >> 52) & 0x7ff) - 0x3ff)) as usize;
                        } else {
                            // x < 2^32
                            tmp1.d    = C1.w[0] as f64; // exact conversion
                            x_nr_bits = (1 + (((tmp1.i >> 52) & 0x7ff) - 0x3ff)) as usize;
                        }
                    } else {
                        // if x < 2^53
                        tmp1.d    = C1.w[0] as f64; // exact conversion
                        x_nr_bits = (1 + (((tmp1.i >> 52) & 0x7ff) - 0x3ff)) as usize;
                    }
                } else {
                    // C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                    tmp1.d    = C1.w[1] as f64; // exact conversion
                    x_nr_bits = (65 + (((tmp1.i >> 52) & 0x7ff) - 0x3ff)) as usize;
                }
            }
            q1 = bid_nr_digits[x_nr_bits - 1].digits as i32;
            if q1 == 0 {
                q1 = bid_nr_digits[x_nr_bits - 1].digits1 as i32;
                if  C1.w[1] > bid_nr_digits[x_nr_bits - 1].threshold_hi
                || (C1.w[1] == bid_nr_digits[x_nr_bits - 1].threshold_hi
                 && C1.w[0] >= bid_nr_digits[x_nr_bits - 1].threshold_lo) {
                    q1 += 1;
                }
            }
            // if q1 < P34 then pad the significand with zeros
            if q1 < P34 {
                exp = ((x_exp >> 49) - 6176) as i32;
                if exp + 6176 > P34 - q1 {
                    ind = P34 - q1; // 1 <= ind <= P34 - 1
                                    // pad with P34 - q1 zeros, until exponent = emin
                                    // C1 = C1 * 10^ind
                    if q1 <= 19 {
                        // 64-bit C1
                        C1 = if ind <= 19 {
                            // 64-bit 10^ind and 64-bit C1
                            __mul_64x64_to_128MACH(C1.w[0], bid_ten2k64[ind as usize])
                        } else {
                            // 128-bit 10^ind and 64-bit C1
                            __mul_128x64_to_128(C1.w[0], &bid_ten2k128[(ind - 20) as usize])
                        };
                    } else {
                        // C1 is (most likely) 128-bit
                        C1 = if ind <= 14 {
                            // 64-bit 10^ind and 128-bit C1 (most likely)
                            __mul_128x64_to_128(bid_ten2k64[ind as usize], &C1)
                        } else if ind <= 19 {
                            // 64-bit 10^ind and 64-bit C1 (q1 <= 19)
                            __mul_64x64_to_128MACH(C1.w[0], bid_ten2k64[ind as usize])
                        } else {
                            // 128-bit 10^ind and 64-bit C1 (C1 must be 64-bit)
                            __mul_128x64_to_128(C1.w[0], &bid_ten2k128[(ind - 20) as usize])
                        };
                    }
                    x_exp -= (ind as BID_UINT64) << 49;
                } else {
                    // pad with zeros until the exponent reaches emin
                    ind = exp + 6176;
                    // C1 = C1 * 10^ind
                    if ind <= 19 {
                        // 1 <= P34 - q1 <= 19 <=> 15 <= q1 <= 33
                        C1 = if q1 <= 19 {
                            // 64-bit C1, 64-bit 10^ind
                            __mul_64x64_to_128MACH(C1.w[0], bid_ten2k64[ind as usize])
                        } else {
                            // 20 <= q1 <= 33 => 128-bit C1, 64-bit 10^ind
                            __mul_128x64_to_128(bid_ten2k64[ind as usize], &C1)
                        };
                    } else {
                        // if 20 <= P34 - q1 <= 33 <=> 1 <= q1 <= 14 =>
                        // 64-bit C1, 128-bit 10^ind
                        C1 = __mul_128x64_to_128(C1.w[0], &bid_ten2k128[(ind - 20) as usize]);
                    }
                    x_exp = EXP_MIN;
                }
            }
            if x_sign == 0 {
                // x > 0
                // add 1 ulp (add 1 to the significand)
                C1.w[0] += 1;
                if C1.w[0] == 0 {
                    C1.w[1] += 1;
                }
                if C1.w[1] == 0x0001ed09bead87c0u64 && C1.w[0] == 0x378d8e6400000000u64 {
                    // if  C1 = 10^34
                    C1.w[1] = 0x0000314dc6448d93u64; // C1 = 10^33
                    C1.w[0] = 0x38c15b0a00000000u64;
                    x_exp  += EXP_P1;
                }
            } else {
                // x < 0
                // subtract 1 ulp (subtract 1 from the significand)
                C1.w[0] -= 1;
                if C1.w[0] == 0xffffffffffffffffu64 {
                    C1.w[1] -= 1;
                }
                if x_exp != 0
                    && C1.w[1] == 0x0000314dc6448d93u64
                    && C1.w[0] == 0x38c15b09ffffffffu64
                {
                    // if  C1 = 10^33 - 1
                    C1.w[1] = 0x0001ed09bead87c0u64; // C1 = 10^34 - 1
                    C1.w[0] = 0x378d8e63ffffffffu64;
                    x_exp  -= EXP_P1;
                }
            }
            // assemble the result
            res.w[1] = x_sign | x_exp | C1.w[1];
            res.w[0] = C1.w[0];
        } // end -MAXFP <= x <= -MINFP - 1 ulp OR MINFP <= x <= MAXFP - 1 ulp
    } // end x is not special and is not zero
    res
}

/// Returns the greatest 128-bit decimal floating-point number that
/// compares less than the operand
pub (crate) fn bid128_nextdown(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::default();
    let x_sign: BID_UINT64;
    let mut x_exp: BID_UINT64;
    let exp: i32;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: usize;
    let mut q1: i32;
    let ind: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default(); // C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (BID_UINT64)

    //BID_SWAP128 (x);
    // unpack the argument
    x_sign  = x.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        if (x.w[1] & MASK_NAN) == MASK_NAN {
            // x is NAN
            let mut x = *x;
            // if x = NaN, then res = Q (x)
            // check first for non-canonical NaN payload
            if  ((x.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
            || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
              && (x.w[0] > 0x38c15b09ffffffffu64)) {
                x.w[1] &= 0xffffc00000000000u64;
                x.w[0]  = 0x0u64;
            }
            if (x.w[1] & MASK_SNAN) == MASK_SNAN {
                // x is SNAN
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return quiet (x)
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64; // clear out also G[6]-G[16]
                res.w[0] = x.w[0];
            } else {
                // x is QNaN
                // return x
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64; // clear out G[6]-G[16]
                res.w[0] = x.w[0];
            }
        } else {
            // x is not NaN, so it must be infinity
            if x_sign == 0 {
                // x is +inf
                res.w[1] = 0x5fffed09bead87c0u64; // +MAXFP = +999...99 * 10^emax
                res.w[0] = 0x378d8e63ffffffffu64;
            } else {
                // x is -inf
                res.w[1] = 0xf800000000000000u64; // -inf
                res.w[0] = 0x0000000000000000u64;
            }
        }
        return res;
    }
    // check for non-canonical values (treated as zero)
    if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
        // G0_G1=11
        // non-canonical
        x_exp   = (x.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
        C1.w[1] = 0; // significand high
        C1.w[0] = 0; // significand low
    } else {
        // G0_G1 != 11
        x_exp = x.w[1] & MASK_EXP; // biased and shifted left 49 bits
        if  C1.w[1]  > 0x0001ed09bead87c0u64
        || (C1.w[1] == 0x0001ed09bead87c0u64
         && C1.w[0]  > 0x378d8e63ffffffffu64) {
            // x is non-canonical if coefficient is larger than 10^34 -1
            C1.w[1] = 0;
            C1.w[0] = 0;
        } else {
            // canonical
        }
    }

    if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is +/-0
        res.w[1] = 0x8000000000000000u64; // -1 * 10^emin
        res.w[0] = 0x0000000000000001u64;
    } else {
        // x is not special and is not zero
        if x.w[1] == 0xdfffed09bead87c0u64 && x.w[0] == 0x378d8e63ffffffffu64 {
            // x = -MAXFP = -999...99 * 10^emax
            res.w[1] = 0xf800000000000000u64; // -inf
            res.w[0] = 0x0000000000000000u64;
        } else if x.w[1] == 0x0u64 && x.w[0] == 0x0000000000000001u64 {
            // +MINFP
            res.w[1] = 0x0000000000000000u64; // +0
            res.w[0] = 0x0000000000000000u64;
        } else {
            // -MAXFP <= x <= -MINFP - 1 ulp OR MINFP <= x <= MAXFP - 1 ulp
            // can add/subtract 1 ulp to the significand

            unsafe {
                // Note: we could check here if x >= 10^34 to speed up the case q1 = 34
                // q1 = nr. of decimal digits in x
                // determine first the nr. of bits in x
                if C1.w[1] == 0 {
                    if C1.w[0] >= 0x0020000000000000u64 {
                        // x >= 2^53
                        // split the 64-bit value in two 32-bit halves to avoid rnd errors
                        if C1.w[0] >= 0x0000000100000000u64 {
                            // x >= 2^32
                            tmp1.d    = (C1.w[0] >> 32) as f64; // exact conversion
                            x_nr_bits = (33 + (((tmp1.i >> 52) & 0x7ff) - 0x3ff)) as usize;
                        } else {
                            // x < 2^32
                            tmp1.d    = C1.w[0] as f64; // exact conversion
                            x_nr_bits = (1 + (((tmp1.i >> 52) & 0x7ff) - 0x3ff)) as usize;
                        }
                    } else {
                        // if x < 2^53
                        tmp1.d    = C1.w[0] as f64; // exact conversion
                        x_nr_bits = (1 + (((tmp1.i >> 52) & 0x7ff) - 0x3ff)) as usize;
                    }
                } else {
                    // C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                    tmp1.d    = C1.w[1] as f64; // exact conversion
                    x_nr_bits = (65 + (((tmp1.i >> 52) & 0x7ff) - 0x3ff)) as usize;
                }
            }
            q1 = bid_nr_digits[x_nr_bits - 1].digits as i32;
            if q1 == 0 {
                q1 = bid_nr_digits[x_nr_bits - 1].digits1 as i32;
                if  C1.w[1]  > bid_nr_digits[x_nr_bits - 1].threshold_hi
                || (C1.w[1] == bid_nr_digits[x_nr_bits - 1].threshold_hi
                 && C1.w[0] >= bid_nr_digits[x_nr_bits - 1].threshold_lo)
                {
                    q1 += 1;
                }
            }
            // if q1 < P then pad the significand with zeros
            if q1 < P34 {
                exp = ((x_exp >> 49) - 6176) as i32;
                if exp + 6176 > P34 - q1 {
                    ind = P34 - q1; // 1 <= ind <= P34 - 1
                                    // pad with P34 - q1 zeros, until exponent = emin
                                    // C1 = C1 * 10^ind
                    if q1 <= 19 {
                        // 64-bit C1
                        C1 = if ind <= 19 {
                            // 64-bit 10^ind and 64-bit C1
                            __mul_64x64_to_128MACH(C1.w[0], bid_ten2k64[ind as usize])
                        } else {
                            // 128-bit 10^ind and 64-bit C1
                            __mul_128x64_to_128(C1.w[0], &bid_ten2k128[(ind - 20) as usize])
                        };
                    } else {
                        // C1 is (most likely) 128-bit
                        C1 = if ind <= 14 {
                            // 64-bit 10^ind and 128-bit C1 (most likely)
                            __mul_128x64_to_128(bid_ten2k64[ind as usize], &C1)
                        } else if ind <= 19 {
                            // 64-bit 10^ind and 64-bit C1 (q1 <= 19)
                            __mul_64x64_to_128MACH(C1.w[0], bid_ten2k64[ind as usize])
                        } else {
                            // 128-bit 10^ind and 64-bit C1 (C1 must be 64-bit)
                            __mul_128x64_to_128(C1.w[0], &bid_ten2k128[(ind - 20) as usize])
                        };
                    }
                    x_exp -= (ind as BID_UINT64) << 49;
                } else {
                    // pad with zeros until the exponent reaches emin
                    ind = exp + 6176;
                    // C1 = C1 * 10^ind
                    if ind <= 19 {
                        // 1 <= P34 - q1 <= 19 <=> 15 <= q1 <= 33
                        C1 = if q1 <= 19 {
                            // 64-bit C1, 64-bit 10^ind
                            __mul_64x64_to_128MACH(C1.w[0], bid_ten2k64[ind as usize])
                        } else {
                            // 20 <= q1 <= 33 => 128-bit C1, 64-bit 10^ind
                            __mul_128x64_to_128(bid_ten2k64[ind as usize], &C1)
                        };
                    } else {
                        // if 20 <= P34 - q1 <= 33 <=> 1 <= q1 <= 14 =>
                        // 64-bit C1, 128-bit 10^ind
                        C1 = __mul_128x64_to_128(C1.w[0], &bid_ten2k128[(ind - 20) as usize]);
                    };
                    x_exp = EXP_MIN;
                }
            }
            if x_sign != 0 {
                // x < 0
                // add 1 ulp (add 1 to the significand)
                C1.w[0] += 1;
                if C1.w[0] == 0 {
                    C1.w[1] += 1;
                }
                if C1.w[1] == 0x0001ed09bead87c0u64 && C1.w[0] == 0x378d8e6400000000u64 {
                    // if  C1 = 10^34
                    C1.w[1] = 0x0000314dc6448d93u64; // C1 = 10^33
                    C1.w[0] = 0x38c15b0a00000000u64;
                    x_exp  += EXP_P1;
                }
            } else {
                // x > 0
                // subtract 1 ulp (subtract 1 from the significand)
                C1.w[0] -= 1;
                if C1.w[0] == 0xffffffffffffffffu64 {
                    C1.w[1] -= 1;
                }
                if x_exp != 0 && C1.w[1] == 0x0000314dc6448d93u64 && C1.w[0] == 0x38c15b09ffffffffu64 {
                    // if  C1 = 10^33 - 1
                    C1.w[1] = 0x0001ed09bead87c0u64; // C1 = 10^34 - 1
                    C1.w[0] = 0x378d8e63ffffffffu64;
                    x_exp  -= EXP_P1;
                }
            }
            // assemble the result
            res.w[1] = x_sign | x_exp | C1.w[1];
            res.w[0] = C1.w[0];
        } // end -MAXFP <= x <= -MINFP - 1 ulp OR MINFP <= x <= MAXFP - 1 ulp
    } // end x is not special and is not zero
    res
}

/// Returns the next 128-bit decimal floating-point number that neighbors
/// the first operand in the direction toward the second operand
pub (crate) fn bid128_nextafter(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::default();
    let mut tmp1: BID_UINT128 = BID_UINT128::default();
    let mut tmp2: BID_UINT128 = BID_UINT128::default();
    let mut tmp3: BID_UINT128 = BID_UINT128::default();
    let mut tmp_fpsf: _IDEC_flags;		// dummy fpsf for calls to comparison functions
    let mut res1: bool;
    let mut res2: bool;
    let x_exp: BID_UINT64;
    let xnswp: BID_UINT128 = *x;
    let ynswp: BID_UINT128 = *y;
    let mut x: BID_UINT128 = *x;
    let mut y: BID_UINT128 = *y;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut xnswp);

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut ynswp);

    // check for NaNs
    if ((x.w[1] & MASK_SPECIAL) == MASK_SPECIAL) || ((y.w[1] & MASK_SPECIAL) == MASK_SPECIAL) {
        // x is special or y is special
        if (x.w[1] & MASK_NAN) == MASK_NAN {	// x is NAN
            // if x = NaN, then res = Q (x)
            // check first for non-canonical NaN payload
            if  ((x.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
            || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
              && (x.w[0] > 0x38c15b09ffffffffu64)) {
                x.w[1] = x.w[1] & 0xffffc00000000000u64;
                x.w[0] = 0x0u64;
            }
            if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return quiet (x)
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64;	// clear out also G[6]-G[16]
                res.w[0] = x.w[0];
            } else {	// x is QNaN
                // return x
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64;	// clear out G[6]-G[16]
                res.w[0] = x.w[0];
                if (y.w[1] & MASK_SNAN) == MASK_SNAN {	// y is SNAN
                    // set invalid flag
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                }
            }
            return res;
        } else if (y.w[1] & MASK_NAN) == MASK_NAN {	// y is NAN
            // if x = NaN, then res = Q (x)
            // check first for non-canonical NaN payload
            if  ((y.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
            || (((y.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
              && (y.w[0] > 0x38c15b09ffffffffu64)) {
                y.w[1] = y.w[1] & 0xffffc00000000000u64;
                y.w[0] = 0x0u64;
            }
            if (y.w[1] & MASK_SNAN) == MASK_SNAN {	// y is SNAN
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return quiet (x)
                res.w[1] = y.w[1] & 0xfc003fffffffffffu64;	// clear out also G[6]-G[16]
                res.w[0] = y.w[0];
            } else {	// x is QNaN
                // return x
                res.w[1] = y.w[1] & 0xfc003fffffffffffu64;	// clear out G[6]-G[16]
                res.w[0] = y.w[0];
            }
            return res;
        } else {	// at least one is infinity
            if (x.w[1] & MASK_ANY_INF) == MASK_INF {	// x = inf
                x.w[1] = x.w[1] & (MASK_SIGN | MASK_INF);
                x.w[0] = 0x0u64;
            }
            if (y.w[1] & MASK_ANY_INF) == MASK_INF {	// y = inf
                y.w[1] = y.w[1] & (MASK_SIGN | MASK_INF);
                y.w[0] = 0x0u64;
            }
        }
    }
    // neither x nor y is NaN

    // if not infinity, check for non-canonical values x (treated as zero)
    if (x.w[1] & MASK_ANY_INF) != MASK_INF {	// x != inf
        if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {	// G0_G1=11
            // non-canonical
            x_exp = (x.w[1] << 2) & MASK_EXP;	// biased and shifted left 49 bits
            x.w[1] = (x.w[1] & MASK_SIGN) | x_exp;
            x.w[0] = 0x0u64;
        } else {	// G0_G1 != 11
            x_exp = x.w[1] & MASK_EXP;	// biased and shifted left 49 bits
            if (x.w[1] & MASK_COEFF) > 0x0001ed09bead87c0u64 ||
                    ((x.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
                     && x.w[0] > 0x378d8e63ffffffffu64) {
                // x is non-canonical if coefficient is larger than 10^34 -1
                x.w[1] = (x.w[1] & MASK_SIGN) | x_exp;
                x.w[0] = 0x0u64;
            } else {
                // canonical
            }
        }
    }
    // no need to check for non-canonical y

    // neither x nor y is NaN
    tmp_fpsf = *pfpsf;	// save fpsf
    res1     = bid128_quiet_equal(&xnswp, &ynswp, pfpsf);
    res2     = bid128_quiet_greater(&xnswp, &ynswp, pfpsf);
    *pfpsf   = tmp_fpsf;	// restore fpsf

    if res1 {	// x = y
        // return x with the sign of y
        res.w[1] = (x.w[1] & 0x7fffffffffffffffu64) | (y.w[1] & 0x8000000000000000u64);
        res.w[0] = x.w[0];
    } else if res2 {	// x > y
        res = bid128_nextdown(&xnswp, pfpsf);
        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);
    } else {	// x < y
        res = bid128_nextup(&xnswp, pfpsf);
        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);
    }
    // if the operand x is finite but the result is infinite, signal
    // overflow and inexact
    if ((x.w[1] & MASK_SPECIAL) != MASK_SPECIAL) && ((res.w[1] & MASK_SPECIAL) == MASK_SPECIAL) {
        // set the inexact flag
        *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
        // set the overflow flag
        *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
    }
    // if the result is in (-10^emin, 10^emin), and is different from the
    // operand x, signal underflow and inexact
    tmp1.w[BID_HIGH_128W] = 0x0000314dc6448d93u64;
    tmp1.w[BID_LOW_128W]  = 0x38c15b0a00000000u64;	// +100...0[34] * 10^emin
    tmp2.w[BID_HIGH_128W] = res.w[1] & 0x7fffffffffffffffu64;
    tmp2.w[BID_LOW_128W]  = res.w[0];
    tmp3.w[BID_HIGH_128W] = res.w[1];
    tmp3.w[BID_LOW_128W]  = res.w[0];
    tmp_fpsf              = *pfpsf;	// save fpsf

    res1 = bid128_quiet_greater(&tmp1, &tmp2, pfpsf);
    res2 = bid128_quiet_not_equal(&xnswp, &tmp3, pfpsf);

    *pfpsf = tmp_fpsf;	// restore fpsf

    if res1 && res2 {
        // set the inexact flag
        *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
        // set the underflow flag
        *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
    }

    return res;
}