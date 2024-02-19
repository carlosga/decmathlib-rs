/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_snake_case)]

use crate::bid128::{bid_maskhigh128, bid_midpoint128, bid_midpoint64, bid_nr_digits, bid_shiftright128, bid_ten2mk128};
use crate::bid_internal::__mul_128x128_to_256;
use crate::constants::{MASK_COEFF, MASK_EXP, MASK_NAN, MASK_SIGN, MASK_SNAN, MASK_SPECIAL};
use crate::core::{RoundingMode, StatusFlags};
use crate::d128::{_IDEC_flags, BID_UI64DOUBLE, BID_UINT128, BID_UINT256, BID_UINT64};

/// Rounds the decimal floating-point value num to an integer value in decicmal floating-point format, using the given rounding mode.
pub (crate) fn bid128_nearbyint(x: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::new(0xbaddbaddbaddbaddu64, 0xbaddbaddbaddbaddu64);
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;                      // unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let tmp64: BID_UINT64;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: usize;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut fstar: BID_UINT256 = BID_UINT256::default();
    let P256: BID_UINT256;

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        return if (x.w[1] & MASK_NAN) == MASK_NAN {                    // x is NAN
            let mut x: BID_UINT128 = *x;
            // if x = NaN, then res = Q (x)
            // check first for non-canonical NaN payload
            if  ((x.w[1] & 0x00003fffffffffffu64)  > 0x0000314dc6448d93u64)
            || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
             && (x.w[0] > 0x38c15b09ffffffffu64)) {
                x.w[1] &= 0xffffc00000000000u64;
                x.w[0] = 0x0u64;
            }
            if (x.w[1] & MASK_SNAN) == MASK_SNAN {              // x is SNAN
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return quiet (x)
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64;      // clear out also G[6]-G[16]
                res.w[0] = x.w[0];
            } else {                                            // x is QNaN
                // return x
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64;      // clear out G[6]-G[16]
                res.w[0] = x.w[0];
            }
            res
        } else {                                                // x is not a NaN, so it must be infinity
            if (x.w[1] & MASK_SIGN) == 0x0u64 {                 // x is +inf
                // return +inf
                res.w[1] = 0x7800000000000000u64;
                res.w[0] = 0x0000000000000000u64;
            } else {                                            // x is -inf
                // return -inf
                res.w[1] = 0xf800000000000000u64;
                res.w[0] = 0x0000000000000000u64;
            }
            res
        }
    }
    // unpack x
    x_sign  = x.w[1] & MASK_SIGN;  // 0 for positive, MASK_SIGN for negative
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for non-canonical values (treated as zero)
    if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {      // G0_G1=11
        // non-canonical
        x_exp   = (x.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
        C1.w[1] = 0;                        // significand high
        C1.w[0] = 0;                        // significand low
    } else {                                // G0_G1 != 11
        x_exp = x.w[1] & MASK_EXP;          // biased and shifted left 49 bits
        if C1.w[1] > 0x0001ed09bead87c0u64 || (C1.w[1] == 0x0001ed09bead87c0u64 && C1.w[0] > 0x378d8e63ffffffffu64) {
            // x is non-canonical if coefficient is larger than 10^34 -1
            C1.w[1] = 0;
            C1.w[0] = 0;
        } else {
            // canonical
        }
    }

    // test for input equal to zero
    if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        // return 0 preserving the sign bit and the preferred exponent
        // of MAX(Q(x), 0)
        res.w[1] = if x_exp <= (0x1820u64 << 49) {
            (x.w[1] & 0x8000000000000000u64) | 0x3040000000000000u64
        } else {
            x_sign | x_exp
        };
        res.w[0] = 0x0000000000000000u64;
        return res;
    }
    // x is not special and is not zero

    match rnd_mode {
        RoundingMode::BID_ROUNDING_TO_NEAREST | RoundingMode::BID_ROUNDING_TIES_AWAY => {
            // if (exp <= -(p+1)) return 0.0
            if x_exp <= 0x2ffa000000000000u64 {       // 0x2ffa000000000000u64 == -35
                res.w[1] = x_sign | 0x3040000000000000u64;
                res.w[0] = 0x0000000000000000u64;
                return res;
            }
        },
        RoundingMode::BID_ROUNDING_DOWN => {
            // if (exp <= -p) return -1.0 or +0.0
            if x_exp <= 0x2ffc000000000000u64 {       // 0x2ffa000000000000u64 == -34
                if x_sign != 0 {
                    // if negative, return negative 1, because we know coefficient
                    // is non-zero (would have been caught above)
                    res.w[1] = 0xb040000000000000u64;
                    res.w[0] = 0x0000000000000001u64;
                } else {
                    // if positive, return positive 0, because we know coefficient is
                    // non-zero (would have been caught above)
                    res.w[1] = 0x3040000000000000u64;
                    res.w[0] = 0x0000000000000000u64;
                }
                return res;
            }
        },
        RoundingMode::BID_ROUNDING_UP => {
            // if (exp <= -p) return -0.0 or +1.0
            if x_exp <= 0x2ffc000000000000u64 {       // 0x2ffc000000000000u64 == -34
                if x_sign != 0 {
                    // if negative, return negative 0, because we know the coefficient
                    // is non-zero (would have been caught above)
                    res.w[1] = 0xb040000000000000u64;
                    res.w[0] = 0x0000000000000000u64;
                } else {
                    // if positive, return positive 1, because we know coefficient is
                    // non-zero (would have been caught above)
                    res.w[1] = 0x3040000000000000u64;
                    res.w[0] = 0x0000000000000001u64;
                }
                return res;
            }
        },
        RoundingMode::BID_ROUNDING_TO_ZERO => {
            // if (exp <= -p) return -0.0 or +0.0
            if x_exp <= 0x2ffc000000000000u64 {       // 0x2ffc000000000000u64 == -34
                res.w[1] = x_sign | 0x3040000000000000u64;
                res.w[0] = 0x0000000000000000u64;
                return res;
            }
        },
        _ => {}
    }

    unsafe {
        // q = nr. of decimal digits in x
        //  determine first the nr. of bits in x
        if C1.w[1] == 0 {
            if C1.w[0] >= 0x0020000000000000u64 {   // x >= 2^53
                // split the 64-bit value in two 32-bit halves to avoid rounding errors
                tmp1.d    = (C1.w[0] >> 32) as f64; // exact conversion
                x_nr_bits = (33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff)) as usize;
            } else {                                // if x < 2^53
                tmp1.d    = C1.w[0] as f64;         // exact conversion
                x_nr_bits = (1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff)) as usize;
            }
        } else {                                    // C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
            tmp1.d    = C1.w[1] as f64;             // exact conversion
            x_nr_bits = (65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff)) as usize;
        }
    }

    q = bid_nr_digits[x_nr_bits - 1].digits as i32;
    if q == 0 {
        q = bid_nr_digits[x_nr_bits - 1].digits1 as i32;
        if  C1.w[1]  > bid_nr_digits[x_nr_bits - 1].threshold_hi
        || (C1.w[1] == bid_nr_digits[x_nr_bits - 1].threshold_hi
         && C1.w[0] >= bid_nr_digits[x_nr_bits - 1].threshold_lo) {
            q += 1;
        }
    }
    exp = ((x_exp >> 49) - 6176) as i32;
    if exp >= 0 {   // -exp <= 0
        // the argument is an integer already
        res.w[1] = x.w[1];
        res.w[0] = x.w[0];
        return res;
    }
    // exp < 0
    match rnd_mode {
        RoundingMode::BID_ROUNDING_TO_NEAREST => {
            return if (q + exp) >= 0 {  // exp < 0 and 1 <= -exp <= q
                // need to shift right -exp digits from the coefficient; exp will be 0
                ind = -exp;             // 1 <= ind <= 34; ind is a synonym for 'x'
                // chop off ind digits from the lower part of C1
                // C1 = C1 + 1/2 * 10^x where the result C1 fits in 127 bits
                tmp64 = C1.w[0];
                if ind <= 19 {
                    C1.w[0] += bid_midpoint64[(ind - 1) as usize];
                } else {
                    C1.w[0] += bid_midpoint128[(ind - 20) as usize].w[0];
                    C1.w[1] += bid_midpoint128[(ind - 20) as usize].w[1];
                }
                if C1.w[0] < tmp64 {
                    C1.w[1] += 1;
                }
                // calculate C* and f*
                // C* is actually floor(C*) in this case
                // C* and f* need shifting and masking, as shown by
                // bid_shiftright128[] and bid_maskhigh128[]
                // 1 <= x <= 34
                // kx = 10^(-x) = bid_ten2mk128[(ind - 1) as usize]
                // C* = (C1 + 1/2 * 10^x) * 10^(-x)
                // the approximation of 10^(-x) was rounded up to 118 bits
                P256 = __mul_128x128_to_256(&C1, &bid_ten2mk128[(ind - 1) as usize]);
                // determine the value of res and fstar

                if ind - 1 <= 2 {       // 0 <= ind - 1 <= 2 => shift = 0
                    // redundant shift = bid_shiftright128[(ind - 1) as usize]; // shift = 0
                    res.w[1] = P256.w[3];
                    res.w[0] = P256.w[2];
                    // redundant fstar.w[3] = 0;
                    // redundant fstar.w[2] = 0;
                    fstar.w[1] = P256.w[1];
                    fstar.w[0] = P256.w[0];
                    // fraction f* < 10^(-x) <=> midpoint
                    // f* is in the right position to be compared with
                    // 10^(-x) from bid_ten2mk128[]
                    // if 0 < fstar < 10^(-x), subtract 1 if odd (for rounding to even)
                    if (res.w[0] & 0x0000000000000001u64) == 0x0000000000000001u64  // is result odd and from a midpoint?
                    && ((fstar.w[1]  < (bid_ten2mk128[(ind - 1) as usize].w[1]))
                    || ((fstar.w[1] == bid_ten2mk128[(ind - 1) as usize].w[1])
                     && (fstar.w[0]  < bid_ten2mk128[(ind - 1) as usize].w[0]))) {
                        // subtract 1 to make even
                        res.w[0] -= 1;
                    }
                } else if ind - 1 <= 21 {       // 3 <= ind - 1 <= 21 => 3 <= shift <= 63
                    shift    = bid_shiftright128[(ind - 1) as usize]; // 3 <= shift <= 63
                    res.w[1] = P256.w[3] >> shift;
                    res.w[0] = (P256.w[3] << (64 - shift)) | (P256.w[2] >> shift);
                    // redundant fstar.w[3] = 0;
                    fstar.w[2] = P256.w[2] & bid_maskhigh128[(ind - 1) as usize];
                    fstar.w[1] = P256.w[1];
                    fstar.w[0] = P256.w[0];
                    // fraction f* < 10^(-x) <=> midpoint
                    // f* is in the right position to be compared with
                    // 10^(-x) from bid_ten2mk128[]
                    if (res.w[0] & 0x0000000000000001u64) == 0x0000000000000001u64  // is result odd and from a midpoint?
                     && fstar.w[2] == 0 && (fstar.w[1] < bid_ten2mk128[(ind - 1) as usize].w[1]
                    || (fstar.w[1] == bid_ten2mk128[(ind - 1) as usize].w[1]
                     && fstar.w[0]  < bid_ten2mk128[(ind - 1) as usize].w[0])) {
                        // subtract 1 to make even
                        res.w[0] -= 1;
                    }
                } else {  // 22 <= ind - 1 <= 33
                    shift      = bid_shiftright128[(ind - 1) as usize] - 64;        // 2 <= shift <= 38
                    res.w[1]   = 0;
                    res.w[0]   = P256.w[3] >> shift;
                    fstar.w[3] = P256.w[3] & bid_maskhigh128[(ind - 1) as usize];
                    fstar.w[2] = P256.w[2];
                    fstar.w[1] = P256.w[1];
                    fstar.w[0] = P256.w[0];
                    // fraction f* < 10^(-x) <=> midpoint
                    // f* is in the right position to be compared with
                    // 10^(-x) from bid_ten2mk128[]
                    if (res.w[0] & 0x0000000000000001u64) == 0x0000000000000001u64  // is result odd and from a midpoint?
                     && fstar.w[3] == 0 && fstar.w[2] == 0
                    && (fstar.w[1]  < bid_ten2mk128[(ind - 1) as usize].w[1]
                    || (fstar.w[1] == bid_ten2mk128[(ind - 1) as usize].w[1]
                     && fstar.w[0]  < bid_ten2mk128[(ind - 1) as usize].w[0])) {
                        // subtract 1 to make even
                        res.w[0] -= 1;
                    }
                }
                res.w[1] |= x_sign | 0x3040000000000000u64;
                res
            } else {                    // if ((q + exp) < 0) <=> q < -exp
                // the result is +0 or -0
                res.w[1] = x_sign | 0x3040000000000000u64;
                res.w[0] = 0x0000000000000000u64;
                res
            }
        },
        RoundingMode::BID_ROUNDING_TIES_AWAY => {
            return if (q + exp) >= 0 {  // exp < 0 and 1 <= -exp <= q
                // need to shift right -exp digits from the coefficient; exp will be 0
                ind = -exp;             // 1 <= ind <= 34; ind is a synonym for 'x'
                // chop off ind digits from the lower part of C1
                // C1 = C1 + 1/2 * 10^x where the result C1 fits in 127 bits
                tmp64 = C1.w[0];
                if ind <= 19 {
                    C1.w[0] += bid_midpoint64[(ind - 1) as usize];
                } else {
                    C1.w[0] += bid_midpoint128[(ind - 20) as usize].w[0];
                    C1.w[1] += bid_midpoint128[(ind - 20) as usize].w[1];
                }
                if C1.w[0] < tmp64 {
                    C1.w[1] += 1;
                }
                // calculate C* and f*
                // C* is actually floor(C*) in this case
                // C* and f* need shifting and masking, as shown by
                // bid_shiftright128[] and bid_maskhigh128[]
                // 1 <= x <= 34
                // kx = 10^(-x) = bid_ten2mk128[(ind - 1) as usize]
                // C* = (C1 + 1/2 * 10^x) * 10^(-x)
                // the approximation of 10^(-x) was rounded up to 118 bits
                P256 = __mul_128x128_to_256(&C1, &bid_ten2mk128[(ind - 1) as usize]);
                // the top Ex bits of 10^(-x) are T* = bid_ten2mk128trunc[ind], e.g.
                // if x=1, T*=bid_ten2mk128trunc[0]=0x19999999999999999999999999999999
                // if (0 < f* < 10^(-x)) then the result is a midpoint
                //   if floor(C*) is even then C* = floor(C*) - logical right
                //       shift; C* has p decimal digits, correct by Prop. 1)
                //   else if floor(C*) is odd C* = floor(C*)-1 (logical right
                //       shift; C* has p decimal digits, correct by Pr. 1)
                // else
                //   C* = floor(C*) (logical right shift; C has p decimal digits,
                //       correct by Property 1)
                // n = C* * 10^(e+x)

                if ind - 1 <= 2 {       // 0 <= ind - 1 <= 2 => shift = 0
                    // redundant shift = bid_shiftright128[(ind - 1) as usize]; // shift = 0
                    res.w[1] = P256.w[3];
                    res.w[0] = P256.w[2];
                    // redundant fstar.w[3] = 0;
                    // redundant fstar.w[2] = 0;
                } else if ind - 1 <= 21 {       // 3 <= ind - 1 <= 21 => 3 <= shift <= 63
                    shift    = bid_shiftright128[(ind - 1) as usize]; // 3 <= shift <= 63
                    res.w[1] = P256.w[3] >> shift;
                    res.w[0] = (P256.w[3] << (64 - shift)) | (P256.w[2] >> shift);
                    // redundant fstar.w[3] = 0;
                } else {                  // 22 <= ind - 1 <= 33
                    shift    = bid_shiftright128[(ind - 1) as usize] - 64;    // 2 <= shift <= 38
                    res.w[1] = 0;
                    res.w[0] = P256.w[3] >> shift;
                }
                // if the result was a midpoint, it was already rounded away from zero
                res.w[1] |= x_sign | 0x3040000000000000u64;
                res
            } else {                    // if ((q + exp) < 0) <=> q < -exp
                // the result is +0 or -0
                res.w[1] = x_sign | 0x3040000000000000u64;
                res.w[0] = 0x0000000000000000u64;
                res
            }
        },
        RoundingMode::BID_ROUNDING_DOWN => {
            return if (q + exp) > 0 {   // exp < 0 and 1 <= -exp < q
                // need to shift right -exp digits from the coefficient; exp will be 0
                ind = -exp;             // 1 <= ind <= 34; ind is a synonym for 'x'
                // (number of digits to be chopped off)
                // chop off ind digits from the lower part of C1
                // FOR ROUND_TO_NEAREST, WE ADD 1/2 ULP(y) then truncate
                // FOR ROUND_TO_ZERO, WE DON'T NEED TO ADD 1/2 ULP
                // FOR ROUND_TO_POSITIVE_INFINITY, WE TRUNCATE, THEN ADD 1 IF POSITIVE
                // FOR ROUND_TO_NEGATIVE_INFINITY, WE TRUNCATE, THEN ADD 1 IF NEGATIVE
                // tmp64 = C1.w[0];
                // if (ind <= 19) {
                //   C1.w[0] = C1.w[0] + bid_midpoint64[(ind - 1) as usize];
                // } else {
                //   C1.w[0] = C1.w[0] + bid_midpoint128[(ind - 20) as usize].w[0];
                //   C1.w[1] = C1.w[1] + bid_midpoint128[(ind - 20) as usize].w[1];
                // }
                // if (C1.w[0] < tmp64) C1.w[1]++;
                // if carry-out from C1.w[0], increment C1.w[1]
                // calculate C* and f*
                // C* is actually floor(C*) in this case
                // C* and f* need shifting and masking, as shown by
                // bid_shiftright128[] and bid_maskhigh128[]
                // 1 <= x <= 34
                // kx = 10^(-x) = bid_ten2mk128[(ind - 1) as usize]
                // C* = (C1 + 1/2 * 10^x) * 10^(-x)
                // the approximation of 10^(-x) was rounded up to 118 bits
                P256 = __mul_128x128_to_256(&C1, &bid_ten2mk128[(ind - 1) as usize]);
                if ind - 1 <= 2 {       // 0 <= ind - 1 <= 2 => shift = 0
                    res.w[1] = P256.w[3];
                    res.w[0] = P256.w[2];
                    // redundant fstar.w[3] = 0;
                    // redundant fstar.w[2] = 0;
                    // redundant fstar.w[1] = P256.w[1];
                    // redundant fstar.w[0] = P256.w[0];
                    // f* is in the right position to be compared with
                    // 10^(-x) from bid_ten2mk128[]
                    if (P256.w[1]  > bid_ten2mk128[(ind - 1) as usize].w[1])
                    || (P256.w[1] == bid_ten2mk128[(ind - 1) as usize].w[1]
                    && (P256.w[0] >= bid_ten2mk128[(ind - 1) as usize].w[0])) {
                        // if positive, the truncated value is already the correct result
                        if x_sign != 0 {         // if negative
                            res.w[0] += 1;
                            if res.w[0] == 0 {
                                res.w[1] += 1;
                            }
                            // if ++res.w[0] == 0 {
                            //     res.w[1] += 1;
                            // }
                        }
                    }
                } else if ind - 1 <= 21 {       // 3 <= ind - 1 <= 21 => 3 <= shift <= 63
                    shift    = bid_shiftright128[(ind - 1) as usize]; // 0 <= shift <= 102
                    res.w[1] = P256.w[3] >> shift;
                    res.w[0] = (P256.w[3] << (64 - shift)) | (P256.w[2] >> shift);
                    // redundant fstar.w[3] = 0;
                    fstar.w[2] = P256.w[2] & bid_maskhigh128[(ind - 1) as usize];
                    fstar.w[1] = P256.w[1];
                    fstar.w[0] = P256.w[0];
                    // f* is in the right position to be compared with
                    // 10^(-x) from bid_ten2mk128[]
                    if  fstar.w[2] != 0
                    ||  fstar.w[1]  > bid_ten2mk128[(ind - 1) as usize].w[1]
                    || (fstar.w[1] == bid_ten2mk128[(ind - 1) as usize].w[1]
                     && fstar.w[0] >= bid_ten2mk128[(ind - 1) as usize].w[0]) {
                        // if positive, the truncated value is already the correct result
                        if x_sign != 0 {         // if negative
                            res.w[0] += 1;
                            if res.w[0] == 0 {
                                res.w[1] += 1;
                            }
                            //     if ++res.w[0] == 0 {
                            //         res.w[1] += 1;
                            //     }
                        }
                    }
                } else {    // 22 <= ind - 1 <= 33
                    shift      = bid_shiftright128[(ind - 1) as usize] - 64;    // 2 <= shift <= 38
                    res.w[1]   = 0;
                    res.w[0]   = P256.w[3] >> shift;
                    fstar.w[3] = P256.w[3] & bid_maskhigh128[(ind - 1) as usize];
                    fstar.w[2] = P256.w[2];
                    fstar.w[1] = P256.w[1];
                    fstar.w[0] = P256.w[0];
                    // f* is in the right position to be compared with
                    // 10^(-x) from bid_ten2mk128[]
                    if  fstar.w[3] != 0 || fstar.w[2] != 0
                    ||  fstar.w[1]  > bid_ten2mk128[(ind - 1) as usize].w[1]
                    || (fstar.w[1] == bid_ten2mk128[(ind - 1) as usize].w[1]
                     && fstar.w[0] >= bid_ten2mk128[(ind - 1) as usize].w[0]) {
                        // if positive, the truncated value is already the correct result
                        if x_sign != 0 {         // if negative
                            res.w[0] += 1;
                            if res.w[0] == 0 {
                                res.w[1] += 1;
                            }
                            //     if ++res.w[0] == 0 {
                            //         res.w[1] += 1;
                            //     }
                        }
                    }
                }
                res.w[1] |= x_sign | 0x3040000000000000u64;
                res
            } else {                    // if exp < 0 and q + exp <= 0
                if x_sign != 0 {        // negative rounds down to -1.0
                    res.w[1] = 0xb040000000000000u64;
                    res.w[0] = 0x0000000000000001u64;
                } else {                // positive rpunds down to +0.0
                    res.w[1] = 0x3040000000000000u64;
                    res.w[0] = 0x0000000000000000u64;
                }
                res
            }
        },
        RoundingMode::BID_ROUNDING_UP => {
            return if (q + exp) > 0 {   // exp < 0 and 1 <= -exp < q
                // need to shift right -exp digits from the coefficient; exp will be 0
                ind = -exp;             // 1 <= ind <= 34; ind is a synonym for 'x'
                // (number of digits to be chopped off)
                // chop off ind digits from the lower part of C1
                // FOR ROUND_TO_NEAREST, WE ADD 1/2 ULP(y) then truncate
                // FOR ROUND_TO_ZERO, WE DON'T NEED TO ADD 1/2 ULP
                // FOR ROUND_TO_POSITIVE_INFINITY, WE TRUNCATE, THEN ADD 1 IF POSITIVE
                // FOR ROUND_TO_NEGATIVE_INFINITY, WE TRUNCATE, THEN ADD 1 IF NEGATIVE
                // tmp64 = C1.w[0];
                // if (ind <= 19) {
                //   C1.w[0] = C1.w[0] + bid_midpoint64[(ind - 1) as usize];
                // } else {
                //   C1.w[0] = C1.w[0] + bid_midpoint128[(ind - 20) as usize].w[0];
                //   C1.w[1] = C1.w[1] + bid_midpoint128[(ind - 20) as usize].w[1];
                // }
                // if (C1.w[0] < tmp64) C1.w[1]++;
                // if carry-out from C1.w[0], increment C1.w[1]
                // calculate C* and f*
                // C* is actually floor(C*) in this case
                // C* and f* need shifting and masking, as shown by
                // bid_shiftright128[] and bid_maskhigh128[]
                // 1 <= x <= 34
                // kx = 10^(-x) = bid_ten2mk128[(ind - 1) as usize]
                // C* = C1 * 10^(-x)
                // the approximation of 10^(-x) was rounded up to 118 bits
                P256 = __mul_128x128_to_256(&C1, &bid_ten2mk128[(ind - 1) as usize]);
                if ind - 1 <= 2 {       // 0 <= ind - 1 <= 2 => shift = 0
                    res.w[1] = P256.w[3];
                    res.w[0] = P256.w[2];
                    // redundant fstar.w[3] = 0;
                    // redundant fstar.w[2] = 0;
                    // redundant fstar.w[1] = P256.w[1];
                    // redundant fstar.w[0] = P256.w[0];
                    // f* is in the right position to be compared with
                    // 10^(-x) from bid_ten2mk128[]
                    if (P256.w[1]  > bid_ten2mk128[(ind - 1) as usize].w[1])
                    || (P256.w[1] == bid_ten2mk128[(ind - 1) as usize].w[1]
                    && (P256.w[0] >= bid_ten2mk128[(ind - 1) as usize].w[0])) {
                        // if negative, the truncated value is already the correct result
                        if x_sign == 0 {        // if positive
                            res.w[0] += 1;
                            if res.w[0] == 0 {
                                res.w[1] += 1;
                            }
                            // if ++res.w[0] == 0 {
                            //     res.w[1] += 1;
                            // }
                        }
                    }
                } else if ind - 1 <= 21 {                               // 3 <= ind - 1 <= 21 => 3 <= shift <= 63
                    shift    = bid_shiftright128[(ind - 1) as usize];   // 3 <= shift <= 63
                    res.w[1] = P256.w[3] >> shift;
                    res.w[0] = (P256.w[3] << (64 - shift)) | (P256.w[2] >> shift);
                    // redundant fstar.w[3] = 0;
                    fstar.w[2] = P256.w[2] & bid_maskhigh128[(ind - 1) as usize];
                    fstar.w[1] = P256.w[1];
                    fstar.w[0] = P256.w[0];
                    // f* is in the right position to be compared with
                    // 10^(-x) from bid_ten2mk128[]
                    if  fstar.w[2] != 0
                     || fstar.w[1]  > bid_ten2mk128[(ind - 1) as usize].w[1]
                    || (fstar.w[1] == bid_ten2mk128[(ind - 1) as usize].w[1]
                     && fstar.w[0] >= bid_ten2mk128[(ind - 1) as usize].w[0]) {
                        // if negative, the truncated value is already the correct result
                        if x_sign == 0 {        // if positive
                            res.w[0] += 1;
                            if res.w[0] == 0 {
                                res.w[1] += 1;
                            }
                            // if ++res.w[0] == 0 {
                            //     res.w[1] += 1;
                            // }
                        }
                    }
                } else {                                                        // 22 <= ind - 1 <= 33
                    shift      = bid_shiftright128[(ind - 1) as usize] - 64;    // 2 <= shift <= 38
                    res.w[1]   = 0;
                    res.w[0]   = P256.w[3] >> shift;
                    fstar.w[3] = P256.w[3] & bid_maskhigh128[(ind - 1) as usize];
                    fstar.w[2] = P256.w[2];
                    fstar.w[1] = P256.w[1];
                    fstar.w[0] = P256.w[0];
                    // f* is in the right position to be compared with
                    // 10^(-x) from bid_ten2mk128[]
                    if  fstar.w[3] != 0 || fstar.w[2] != 0
                     || fstar.w[1]  > bid_ten2mk128[(ind - 1) as usize].w[1]
                    || (fstar.w[1] == bid_ten2mk128[(ind - 1) as usize].w[1]
                     && fstar.w[0] >= bid_ten2mk128[(ind - 1) as usize].w[0]) {
                        // if negative, the truncated value is already the correct result
                        if x_sign == 0 {        // if positive
                            res.w[0] += 1;
                            if res.w[0] == 0 {
                                res.w[1] += 1;
                            }
                            // if ++res.w[0] == 0 {
                            //     res.w[1] += 1;
                            // }
                        }
                    }
                }
                res.w[1] |= x_sign | 0x3040000000000000u64;
                res
            } else {                // if exp < 0 and q + exp <= 0
                if x_sign != 0 {    // negative rounds up to -0.0
                    res.w[1] = 0xb040000000000000u64;
                    res.w[0] = 0x0000000000000000u64;
                } else {            // positive rpunds up to +1.0
                    res.w[1] = 0x3040000000000000u64;
                    res.w[0] = 0x0000000000000001u64;
                }
                res
            }
        },
        RoundingMode::BID_ROUNDING_TO_ZERO => {
            return if (q + exp) > 0 {   // exp < 0 and 1 <= -exp < q
                // need to shift right -exp digits from the coefficient; exp will be 0
                ind = -exp;             // 1 <= ind <= 34; ind is a synonym for 'x'
                // (number of digits to be chopped off)
                // chop off ind digits from the lower part of C1
                // FOR ROUND_TO_NEAREST, WE ADD 1/2 ULP(y) then truncate
                // FOR ROUND_TO_ZERO, WE DON'T NEED TO ADD 1/2 ULP
                // FOR ROUND_TO_POSITIVE_INFINITY, WE TRUNCATE, THEN ADD 1 IF POSITIVE
                // FOR ROUND_TO_NEGATIVE_INFINITY, WE TRUNCATE, THEN ADD 1 IF NEGATIVE
                //tmp64 = C1.w[0];
                // if (ind <= 19) {
                //   C1.w[0] = C1.w[0] + bid_midpoint64[(ind - 1) as usize];
                // } else {
                //   C1.w[0] = C1.w[0] + bid_midpoint128[(ind - 20) as usize].w[0];
                //   C1.w[1] = C1.w[1] + bid_midpoint128[(ind - 20) as usize].w[1];
                // }
                // if (C1.w[0] < tmp64) C1.w[1]++;
                // if carry-out from C1.w[0], increment C1.w[1]
                // calculate C* and f*
                // C* is actually floor(C*) in this case
                // C* and f* need shifting and masking, as shown by
                // bid_shiftright128[] and bid_maskhigh128[]
                // 1 <= x <= 34
                // kx = 10^(-x) = bid_ten2mk128[(ind - 1) as usize]
                // C* = (C1 + 1/2 * 10^x) * 10^(-x)
                // the approximation of 10^(-x) was rounded up to 118 bits
                P256 = __mul_128x128_to_256(&C1, &bid_ten2mk128[(ind - 1) as usize]);
                if ind - 1 <= 2 {       // 0 <= ind - 1 <= 2 => shift = 0
                    res.w[1] = P256.w[3];
                    res.w[0] = P256.w[2];
                    // redundant fstar.w[3] = 0;
                    // redundant fstar.w[2] = 0;
                    // redundant fstar.w[1] = P256.w[1];
                    // redundant fstar.w[0] = P256.w[0];
                } else if ind - 1 <= 21 {       // 3 <= ind - 1 <= 21 => 3 <= shift <= 63
                    shift    = bid_shiftright128[(ind - 1) as usize]; // 3 <= shift <= 63
                    res.w[1] = P256.w[3] >> shift;
                    res.w[0] = (P256.w[3] << (64 - shift)) | (P256.w[2] >> shift);
                    // redundant fstar.w[3] = 0;
                } else {                  // 22 <= ind - 1 <= 33
                    shift = bid_shiftright128[(ind - 1) as usize] - 64;    // 2 <= shift <= 38
                    res.w[1] = 0;
                    res.w[0] = P256.w[3] >> shift;
                }
                res.w[1] |= x_sign | 0x3040000000000000u64;
                res
            } else {    // if exp < 0 and q + exp <= 0 the result is +0 or -0
                res.w[1] = x_sign | 0x3040000000000000u64;
                res.w[0] = 0x0000000000000000u64;
                res
            }
        }
        _ => {}
    }

    res
}
