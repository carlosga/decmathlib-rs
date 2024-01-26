/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(unused_assignments)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::bid128::*;
use crate::bid128_fma::bid64qqq_fma;
use crate::bid_conf::BID_HIGH_128W;
use crate::bid_internal::{__mul_128x128_to_256, __mul_128x64_to_128, __mul_64x64_to_128MACH};
use crate::constants::*;
use crate::convert::bid64_to_bid128;
use crate::core::{RoundingMode, StatusFlags};
use crate::d128::{_IDEC_flags, BID_UI64DOUBLE, BID_UINT128, BID_UINT256, BID_UINT64};

/////////////////////////////////////
/// BID64/BID128 add
/////////////////////////////////////

pub (crate) fn bid64dq_add(x: BID_UINT64, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let x1: BID_UINT128 = bid64_to_bid128(x, pfpsf);
    let res: BID_UINT64 = bid64qq_add(&x1, y, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid64qd_add(x: &BID_UINT128, y: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let y1: BID_UINT128 = bid64_to_bid128(y, pfpsf);
    let res: BID_UINT64 = bid64qq_add(x, &y1, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid64qq_add(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let one: BID_UINT128 = BID_UINT128::new(0x0000000000000001u64, 0x3040000000000000u64);
    // Swapped on ::new
    // BID_SWAP128(one);
    let res: BID_UINT64 = bid64qqq_fma(&one, x, y, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid128dd_add(x: BID_UINT64, y: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let x1: BID_UINT128  = bid64_to_bid128(x, pfpsf);
    let y1: BID_UINT128  = bid64_to_bid128(y, pfpsf);
    let res: BID_UINT128 = bid128_add(&x1, &y1, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid128dq_add(x: BID_UINT64, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let x1: BID_UINT128  = bid64_to_bid128(x, pfpsf);
    let res: BID_UINT128 = bid128_add(&x1, y, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid128qd_add(x: &BID_UINT128, y: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let y1: BID_UINT128  = bid64_to_bid128(y, pfpsf);
    let res: BID_UINT128 = bid128_add(x, &y1, rnd_mode, pfpsf);
    res
}

// bid128_add stands for bid128qq_add

/////////////////////////////////////
/// BID64/BID128 sub
/////////////////////////////////////

pub (crate) fn bid64dq_sub(x: BID_UINT64, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let x1: BID_UINT128 = bid64_to_bid128(x, pfpsf);
    let res: BID_UINT64 = bid64qq_sub(&x1, y, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid64qd_sub(x: &BID_UINT128, y: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let y1: BID_UINT128 = bid64_to_bid128(y, pfpsf);
    let res: BID_UINT64 = bid64qq_sub(x, &y1, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid64qq_sub(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let one: BID_UINT128   = BID_UINT128::new(0x0000000000000001u64, 0x3040000000000000u64);
    let mut y: BID_UINT128 = *y;
    // swapped on ::new
    // BID_SWAP128(one);
    if (y.w[BID_HIGH_128W] & MASK_NAN) != MASK_NAN {   // y is not NAN
        // change its sign
        let y_sign = y.w[BID_HIGH_128W] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative

        y.w[BID_HIGH_128W] = if y_sign != 0 {
            y.w[BID_HIGH_128W] & 0x7fffffffffffffffu64
        } else {
            y.w[BID_HIGH_128W] | 0x8000000000000000u64
        };
    }
    let res = bid64qqq_fma(&one, x, &y, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid128dd_sub(x: BID_UINT64, y: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let x1  = bid64_to_bid128(x, pfpsf);
    let y1  = bid64_to_bid128(y, pfpsf);
    let res = bid128_sub(&x1, &y1, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid128dq_sub(x: BID_UINT64, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let x1  = bid64_to_bid128(x, pfpsf);
    let res = bid128_sub(&x1, y, rnd_mode, pfpsf);
    res
}

pub (crate) fn bid128qd_sub(x: &BID_UINT128, y: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let y1  = bid64_to_bid128(y, pfpsf);
    let res = bid128_sub(x, &y1, rnd_mode, pfpsf);
    res
}

// bid128_sub stands for bid128qq_sub

/////////////////////////////////////
///  BID128 sub
/////////////////////////////////////

// DFP_WRAPFN_DFP_DFP(128, bid128_sub, 128, 128)
pub (crate) fn bid128_sub(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut y: BID_UINT128 = *y;
    if (y.w[BID_HIGH_128W] & MASK_NAN) != MASK_NAN {
        // y is not NAN
        // change its sign
        let y_sign = y.w[BID_HIGH_128W] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
        y.w[BID_HIGH_128W] = if y_sign != 0 {
            y.w[BID_HIGH_128W] & 0x7fffffffffffffffu64
        } else {
            y.w[BID_HIGH_128W] | 0x8000000000000000u64
        };
    }
    let res = bid128_add(x, &y, rnd_mode, pfpsf);
    res
}

/////////////////////////////////////
/// BID128 add
/////////////////////////////////////

// DFP_WRAPFN_DFP_DFP(128, bid128_add, 128, 128)
pub (crate) fn bid128_add(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128 { w: [0xbaddbaddbaddbaddu64, 0xbaddbaddbaddbaddu64] };
    let mut x_sign: BID_UINT64;
    let mut y_sign: BID_UINT64;
    let mut tmp_sign: BID_UINT64;
    let mut x_exp: BID_UINT64;
    let mut y_exp: BID_UINT64;
    let tmp_exp: BID_UINT64; // e1 = x_exp, e2 = y_exp
    let mut C1_hi: BID_UINT64;
    let mut C2_hi: BID_UINT64;
    let tmp_signif_hi: BID_UINT64;
    let mut C1_lo: BID_UINT64;
    let mut C2_lo: BID_UINT64;
    let tmp_signif_lo: BID_UINT64;
    // Note: C1.w[1], C1.w[0] represent C1_hi, C1_lo (all BID_UINT64)
    // Note: C2.w[1], C2.w[0] represent C2_hi, C2_lo (all BID_UINT64)
    let mut tmp64: BID_UINT64;
    let mut tmp64A: BID_UINT64;
    let mut tmp64B: BID_UINT64;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let mut tmp2: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: i32;
    let y_nr_bits: i32;
    let mut q1: i32;
    let mut q2: i32;
    let delta: i32;
    let mut scale: i32;
    let mut x1: i32;
    let mut ind: i32;
    let mut shift: i32;
    let mut tmp_inexact: bool = false;
    let halfulp64: BID_UINT64;
    let mut halfulp128: &BID_UINT128;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C2: BID_UINT128 = BID_UINT128::default();
    let mut ten2m1: BID_UINT128 = BID_UINT128::default();
    let mut highf2star: BID_UINT128 = BID_UINT128::default(); // top 128 bits in f2*; low 128 bits in R256[1], R256[0]
    let mut P256: BID_UINT256;
    let mut Q256: BID_UINT256;
    let mut R256: BID_UINT256 = BID_UINT256::default();
    let mut is_inexact: bool = false;
    let mut is_midpoint_lt_even: bool = false;
    let mut is_midpoint_gt_even: bool = false;
    let mut is_inexact_lt_midpoint: bool = false;
    let mut is_inexact_gt_midpoint: bool = false;
    let mut second_pass: bool = false;
    let mut x = *x;
    let mut y = *y;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut y);

    x_sign = x.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
    y_sign = y.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative

    // check for NaN or Infinity
    if ((x.w[1] & MASK_SPECIAL) == MASK_SPECIAL) || ((y.w[1] & MASK_SPECIAL) == MASK_SPECIAL) {
        // x is special or y is special
        if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
            // check first for non-canonical NaN payload
            if  ((x.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
            || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
              && (x.w[0] > 0x38c15b09ffffffffu64)) {
                x.w[1] = x.w[1] & 0xffffc00000000000u64;
                x.w[0] = 0x0u64;
            }
            if (x.w[1] & MASK_SNAN) == MASK_SNAN { // x is SNAN
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return quiet (x)
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64;
                // clear out also G[6]-G[16]
                res.w[0] = x.w[0];
            } else { // x is QNaN
                // return x
                res.w[1] = x.w[1] & 0xfc003fffffffffffu64;
                // clear out G[6]-G[16]
                res.w[0] = x.w[0];
                // if y = SNaN signal invalid exception
                if (y.w[1] & MASK_SNAN) == MASK_SNAN {
                    // set invalid flag
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                }
            }

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            return res;
        } else if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
            // check first for non-canonical NaN payload
            if  ((y.w[1] & 0x00003fffffffffffu64)  > 0x0000314dc6448d93u64)
            || (((y.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
              && (y.w[0] > 0x38c15b09ffffffffu64)) {
                y.w[1] = y.w[1] & 0xffffc00000000000u64;
                y.w[0] = 0x0u64;
            }
            if (y.w[1] & MASK_SNAN) == MASK_SNAN { // y is SNAN
                // set invalid flag
                *pfpsf  |= StatusFlags::BID_INVALID_EXCEPTION;
                // return quiet (y)
                res.w[1] = y.w[1] & 0xfc003fffffffffffu64;
                // clear out also G[6]-G[16]
                res.w[0] = y.w[0];
            } else { // y is QNaN
                // return y
                res.w[1] = y.w[1] & 0xfc003fffffffffffu64;
                // clear out G[6]-G[16]
                res.w[0] = y.w[0];
            }

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            return res;
        } else {                                         // neither x not y is NaN; at least one is infinity
            if (x.w[1] & MASK_ANY_INF) == MASK_INF {     // x is infinity
                if (y.w[1] & MASK_ANY_INF) == MASK_INF { // y is infinity
                    // if same sign, return either of them
                    if (x.w[1] & MASK_SIGN) == (y.w[1] & MASK_SIGN) {
                        res.w[1] = x_sign | MASK_INF;
                        res.w[0] = 0x0u64;
                    } else { // x and y are infinities of opposite signs
                        // set invalid flag
                        *pfpsf  |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return QNaN Indefinite
                        res.w[1] = 0x7c00000000000000u64;
                        res.w[0] = 0x0000000000000000u64;
                    }
                } else { // y is 0 or finite
                    // return x
                    res.w[1] = x_sign | MASK_INF;
                    res.w[0] = 0x0u64;
                }
            } else { // x is not NaN or infinity, so y must be infinity
                res.w[1] = y_sign | MASK_INF;
                res.w[0] = 0x0u64;
            }

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            return res;
        }
    }
    // unpack the arguments

    // unpack x
    C1_hi = x.w[1] & MASK_COEFF;
    C1_lo = x.w[0];
    // test for non-canonical values:
    // - values whose encoding begins with x00, x01, or x10 and whose
    //   coefficient is larger than 10^34 -1, or
    // - values whose encoding begins with x1100, x1101, x1110 (if NaNs
    //   and infinitis were eliminated already this test is reduced to
    //   checking for x10x)

    // x is not infinity; check for non-canonical values - treated as zero
    if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
        // G0_G1=11; non-canonical
        x_exp = (x.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
        C1_hi = 0;                        // significand high
        C1_lo = 0;                        // significand low
    } else {                              // G0_G1 != 11
        x_exp = x.w[1] & MASK_EXP;        // biased and shifted left 49 bits
        if C1_hi > 0x0001ed09bead87c0u64 || (C1_hi == 0x0001ed09bead87c0u64 && C1_lo > 0x378d8e63ffffffffu64) {
            // x is non-canonical if coefficient is larger than 10^34 -1
            C1_hi = 0;
            C1_lo = 0;
        } else {
            // canonical
        }
    }

    // unpack y
    C2_hi = y.w[1] & MASK_COEFF;
    C2_lo = y.w[0];
    // y is not infinity; check for non-canonical values - treated as zero
    if (y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
        // G0_G1=11; non-canonical
        y_exp = (y.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
        C2_hi = 0;                        // significand high
        C2_lo = 0;                        // significand low
    } else {                              // G0_G1 != 11
        y_exp = y.w[1] & MASK_EXP;        // biased and shifted left 49 bits
        if C2_hi > 0x0001ed09bead87c0u64 || (C2_hi == 0x0001ed09bead87c0u64 && C2_lo > 0x378d8e63ffffffffu64) {
            // y is non-canonical if coefficient is larger than 10^34 -1
            C2_hi = 0;
            C2_lo = 0;
        } else {
            // canonical
        }
    }

    if (C1_hi == 0x0u64) && (C1_lo == 0x0u64) {
        // x is 0 and y is not special
        // if y is 0 return 0 with the smaller exponent
        if (C2_hi == 0x0u64) && (C2_lo == 0x0u64) {
            res.w[1] = if x_exp < y_exp { x_exp } else { y_exp };
            if x_sign != 0 && y_sign != 0 {
                res.w[1] = res.w[1] | x_sign; // both negative
            } else if rnd_mode == RoundingMode::BID_ROUNDING_DOWN && x_sign != y_sign {
                res.w[1] = res.w[1] | 0x8000000000000000u64; // -0
            }
            // else; // res = +0
            res.w[0] = 0;
        } else {
            // for 0 + y return y, with the preferred exponent
            if y_exp <= x_exp {
                res.w[1] = y.w[1];
                res.w[0] = y.w[0];
            } else {
                // if y_exp > x_exp
                // return (C2 * 10^scale) * 10^(y_exp - scale)
                // where scale = min (P34-q2, y_exp-x_exp)
                // determine q2 = nr. of decimal digits in y
                //  determine first the nr. of bits in y (y_nr_bits)

                unsafe {
                    if C2_hi == 0 {                           // y_bits is the nr. of bits in C2_lo
                        if C2_lo >= 0x0020000000000000u64 {   // y >= 2^53
                                                                // split the 64-bit value in two 32-bit halves to avoid
                                                                // rounding errors
                            tmp2.d    = (C2_lo >> 32) as f64;   // exact conversion
                            y_nr_bits = (32 + ((((tmp2.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                        } else {                                // if y < 2^53
                            tmp2.d    = C2_lo as f64;           // exact conversion
                            y_nr_bits = ((((tmp2.i >> 52) as u32) & 0x7ff) - 0x3ff) as i32;
                        }
                    } else {                                    // C2_hi != 0 => nr. bits = 64 + nr_bits (C2_hi)
                        tmp2.d    = C2_hi as f64;               // exact conversion
                        y_nr_bits = (64 + ((((tmp2.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                    }
                }
                q2 = bid_nr_digits[y_nr_bits as usize].digits as i32;
                if q2 == 0 {
                    q2 = bid_nr_digits[y_nr_bits as usize].digits1 as i32;
                    if  C2_hi  > bid_nr_digits[y_nr_bits as usize].threshold_hi
                    || (C2_hi == bid_nr_digits[y_nr_bits as usize].threshold_hi
                     && C2_lo >= bid_nr_digits[y_nr_bits as usize].threshold_lo) {
                        q2 += 1;
                    }
                }
                // return (C2 * 10^scale) * 10^(y_exp - scale)
                // where scale = min (P34-q2, y_exp-x_exp)
                scale = P34 - q2;
                ind   = ((y_exp - x_exp) >> 49) as i32;
                if ind < scale {
                    scale = ind;
                }
                if scale == 0 {
                    res.w[1] = y.w[1];
                    res.w[0] = y.w[0];
                } else if q2 <= 19 { // y fits in 64 bits
                    res = if scale <= 19 {
                        // 10^scale fits in 64 bits
                        // 64 x 64 C2_lo * bid_ten2k64[scale as usize]
                        __mul_64x64_to_128MACH(C2_lo, bid_ten2k64[scale as usize])
                    } else {
                        // 10^scale fits in 128 bits
                        // 64 x 128 C2_lo * bid_ten2k128[(scale - 20) as usize]
                        __mul_128x64_to_128(C2_lo, &bid_ten2k128[(scale - 20) as usize])
                    };
                } else { // y fits in 128 bits, but 10^scale must fit in 64 bits
                    // 64 x 128 bid_ten2k64[scale as usize] * C2
                    C2.w[1] = C2_hi;
                    C2.w[0] = C2_lo;
                    res     = __mul_128x64_to_128(bid_ten2k64[scale as usize], &C2);
                }
                // subtract scale from the exponent
                y_exp    = y_exp - ((scale as BID_UINT64) << 49);
                res.w[1] = res.w[1] | y_sign | y_exp;
            }
        }

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        res
    } else if (C2_hi == 0x0u64) && (C2_lo == 0x0u64) {
        // y is 0 and x is not special, and not zero
        // for x + 0 return x, with the preferred exponent
        if x_exp <= y_exp {
            res.w[1] = x.w[1];
            res.w[0] = x.w[0];
        } else {
            unsafe {
                // if x_exp > y_exp
                // return (C1 * 10^scale) * 10^(x_exp - scale)
                // where scale = min (P34-q1, x_exp-y_exp)
                // determine q1 = nr. of decimal digits in x
                //  determine first the nr. of bits in x
                if C1_hi == 0 {                           // x_bits is the nr. of bits in C1_lo
                    if C1_lo >= 0x0020000000000000u64 {   // x >= 2^53
                                                            // split the 64-bit value in two 32-bit halves to avoid
                                                            // rounding errors
                        tmp1.d    = (C1_lo >> 32) as f64;   // exact conversion
                        x_nr_bits = (32 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                    } else {                                // if x < 2^53
                        tmp1.d    = C1_lo as f64;           // exact conversion
                        x_nr_bits = ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff) as i32;
                    }
                } else {                                    // C1_hi != 0 => nr. bits = 64 + nr_bits (C1_hi)
                    tmp1.d    = C1_hi as f64;               // exact conversion
                    x_nr_bits = (64 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                }
            }
            q1 = bid_nr_digits[x_nr_bits as usize].digits as i32;
            if q1 == 0 {
                q1 = bid_nr_digits[x_nr_bits as usize].digits1 as i32;
                if  C1_hi  > bid_nr_digits[x_nr_bits as usize].threshold_hi
                || (C1_hi == bid_nr_digits[x_nr_bits as usize].threshold_hi
                 && C1_lo >= bid_nr_digits[x_nr_bits as usize].threshold_lo) {
                    q1 += 1;
                }
            }
            // return (C1 * 10^scale) * 10^(x_exp - scale)
            // where scale = min (P34-q1, x_exp-y_exp)
            scale = P34 - q1;
            ind   = ((x_exp - y_exp) >> 49) as i32;
            if ind < scale {
                scale = ind;
            }
            if scale == 0 {
                res.w[1] = x.w[1];
                res.w[0] = x.w[0];
            } else if q1 <= 19 { // x fits in 64 bits
                res = if scale <= 19 {
                    // 10^scale fits in 64 bits
                    // 64 x 64 C1_lo * bid_ten2k64[scale as usize]
                    __mul_64x64_to_128MACH(C1_lo, bid_ten2k64[scale as usize])
                } else {
                    // 10^scale fits in 128 bits
                    // 64 x 128 C1_lo * bid_ten2k128[(scale - 20) as usize]
                    __mul_128x64_to_128(C1_lo, &bid_ten2k128[(scale - 20) as usize])
                };
            } else { // x fits in 128 bits, but 10^scale must fit in 64 bits
                // 64 x 128 bid_ten2k64[scale as usize] * C1
                C1.w[1] = C1_hi;
                C1.w[0] = C1_lo;
                res = __mul_128x64_to_128(bid_ten2k64[scale as usize], &C1);
            }
            // subtract scale from the exponent
            x_exp    = x_exp - ((scale as BID_UINT64) << 49);
            res.w[1] = res.w[1] | x_sign | x_exp;
        }

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    } else {
        // x and y are not canonical, not special, and are not zero
        // note that the result may still be zero, and then it has to have the
        // preferred exponent
        if x_exp < y_exp { // if exp_x < exp_y then swap x and y
            tmp_sign = x_sign;
            tmp_exp = x_exp;
            tmp_signif_hi = C1_hi;
            tmp_signif_lo = C1_lo;
            x_sign = y_sign;
            x_exp = y_exp;
            C1_hi = C2_hi;
            C1_lo = C2_lo;
            y_sign = tmp_sign;
            y_exp = tmp_exp;
            C2_hi = tmp_signif_hi;
            C2_lo = tmp_signif_lo;
        }
        unsafe {
            // q1 = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1_hi == 0 {                           // x_bits is the nr. of bits in C1_lo
                if C1_lo >= 0x0020000000000000u64 {   // x >= 2^53
                                                        // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1_lo >> 32) as f64;   // exact conversion
                    x_nr_bits = (32 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                } else {                                // if x < 2^53
                    tmp1.d    = C1_lo as f64;           // exact conversion
                    x_nr_bits = ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff) as i32;
                }
            } else {                                    // C1_hi != 0 => nr. bits = 64 + nr_bits (C1_hi)
                tmp1.d    = C1_hi as f64;               // exact conversion
                x_nr_bits = (64 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            }
        }

        q1 = bid_nr_digits[x_nr_bits as usize].digits as i32;
        if q1 == 0 {
            q1 = bid_nr_digits[x_nr_bits as usize].digits1 as i32;
            if C1_hi  > bid_nr_digits[x_nr_bits as usize].threshold_hi
            || (C1_hi == bid_nr_digits[x_nr_bits as usize].threshold_hi
             && C1_lo >= bid_nr_digits[x_nr_bits as usize].threshold_lo) {
                 q1 += 1;
            }
        }
        unsafe {
            // q2 = nr. of decimal digits in y
            //  determine first the nr. of bits in y (y_nr_bits)
            if C2_hi == 0 {                           // y_bits is the nr. of bits in C2_lo
                if C2_lo >= 0x0020000000000000u64 {   // y >= 2^53
                                                        // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp2.d    = (C2_lo >> 32) as f64;   // exact conversion
                    y_nr_bits = (32 + ((((tmp2.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                } else {                                // if y < 2^53
                    tmp2.d    = C2_lo as f64;           // exact conversion
                    y_nr_bits = ((((tmp2.i >> 52) as u32) & 0x7ff) - 0x3ff) as i32;
                }
            } else {                                    // C2_hi != 0 => nr. bits = 64 + nr_bits (C2_hi)
                tmp2.d    = C2_hi as f64;               // exact conversion
                y_nr_bits = (64 + ((((tmp2.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            }
        }

        q2 = bid_nr_digits[y_nr_bits as usize].digits as i32;
        if q2 == 0 {
            q2 = bid_nr_digits[y_nr_bits as usize].digits1 as i32;
            if C2_hi  > bid_nr_digits[y_nr_bits as usize].threshold_hi
            || (C2_hi == bid_nr_digits[y_nr_bits as usize].threshold_hi
             && C2_lo >= bid_nr_digits[y_nr_bits as usize].threshold_lo) {
                q2 += 1;
            }
        }

        delta = q1 + ((x_exp >> 49) as i32) - q2 - ((y_exp >> 49) as i32);

        if delta >= P34 {
            // round the result directly because 0 < C2 < ulp (C1 * 10^(x_exp-e2))
            // n = C1 * 10^e1 or n = C1 +/- 10^(q1-P34)) * 10^e1
            // the result is inexact; the preferred exponent is the least possible

            if delta >= P34 + 1 {
                // for RN the result is the operand with the larger magnitude,
                // possibly scaled up by 10^(P34-q1)
                // an overflow cannot occur in this case (rounding to nearest)
                if q1 < P34 { // scale C1 up by 10^(P34-q1)
                    // Note: because delta >= P34+1 it is certain that
                    //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
                    scale = P34 - q1;
                    if q1 <= 19 { // C1 fits in 64 bits
                        // 1 <= q1 <= 19 => 15 <= scale <= 33
                        C1 = if scale <= 19 { // 10^scale fits in 64 bits
                            __mul_64x64_to_128MACH(bid_ten2k64[scale as usize], C1_lo)
                        } else { // if 20 <= scale <= 33
                            // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                            // (C1 * 10^(scale-19)) fits in 64 bits
                            C1_lo = C1_lo * bid_ten2k64[(scale - 19) as usize];
                            __mul_64x64_to_128MACH(bid_ten2k64[19], C1_lo)
                        };
                    } else { // if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                        // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                        C1.w[1] = C1_hi;
                        C1.w[0] = C1_lo;
                        // C1 = bid_ten2k64[(P34 - q1) as usize] * C1
                        C1      = __mul_128x64_to_128(bid_ten2k64[(P34 - q1) as usize], &C1);
                    }
                    x_exp = x_exp - ((scale as BID_UINT64) << 49);
                    C1_hi = C1.w[1];
                    C1_lo = C1.w[0];
                }
                // some special cases arise: if delta = P34 + 1 and C1 = 10^(P34-1)
                // (after scaling) and x_sign != y_sign and C2 > 5*10^(q2-1) =>
                // subtract 1 ulp
                // Note: do this only for rounding to nearest; for other rounding
                // modes the correction will be applied next
                if (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST
                  || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY) && delta == (P34 + 1)
                  && C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b0a00000000u64 && x_sign != y_sign
                  && ((q2 <= 19 && C2_lo > bid_midpoint64[(q2 - 1) as usize])
                   || (q2 >= 20 && (C2_hi > bid_midpoint128[(q2 - 20) as usize].w[1]
                   || (C2_hi == bid_midpoint128[(q2 - 20) as usize].w[1]
                    && C2_lo  > bid_midpoint128[(q2 - 20) as usize].w[0])))) {
                    // C1 = 10^34 - 1 and decrement x_exp by 1 (no underflow possible)
                    C1_hi = 0x0001ed09bead87c0u64;
                    C1_lo = 0x378d8e63ffffffffu64;
                    x_exp = x_exp - EXP_P1;
                }
                if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                    if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN && x_sign != 0 && y_sign != 0)
                     || (rnd_mode == RoundingMode::BID_ROUNDING_UP   && x_sign == 0 && y_sign == 0) {
                        // add 1 ulp and then check for overflow
                        C1_lo = C1_lo.wrapping_add(1);
                        if C1_lo == 0 { // rounding overflow in the low 64 bits
                            C1_hi = C1_hi.wrapping_add(1);
                        }
                        if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                            // C1 = 10^34 => rounding overflow
                            C1_hi = 0x0000314dc6448d93u64;
                            C1_lo = 0x38c15b0a00000000u64; // 10^33
                            x_exp = x_exp + EXP_P1;
                            if x_exp == EXP_MAX_P1 {         // overflow
                                C1_hi = 0x7800000000000000u64; // +inf
                                C1_lo = 0x0u64;
                                x_exp = 0; // x_sign is preserved
                                // set overflow flag (the inexact flag was set too)
                                *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                            }
                        }
                    } else if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN    && x_sign == 0 && y_sign != 0)
                            || (rnd_mode == RoundingMode::BID_ROUNDING_UP      && x_sign != 0 && y_sign == 0)
                            || (rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO && x_sign != y_sign) {
                        // subtract 1 ulp from C1
                        // Note: because delta >= P34 + 1 the result cannot be zero
                        C1_lo = C1_lo.wrapping_sub(1);
                        if C1_lo == 0xffffffffffffffffu64 {
                            C1_hi = C1_hi.wrapping_sub(1);
                        }
                        // if the coefficient is 10^33 - 1 then make it 10^34 - 1 and
                        // decrease the exponent by 1 (because delta >= P34 + 1 the
                        // exponent will not become less than e_min)
                        // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
                        // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
                        if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 {
                            // make C1 = 10^34  - 1
                            C1_hi = 0x0001ed09bead87c0u64;
                            C1_lo = 0x378d8e63ffffffffu64;
                            x_exp = x_exp - EXP_P1;
                        }
                    } else {
                        // the result is already correct
                    }
                }
                // set the inexact flag
                *pfpsf  |= StatusFlags::BID_INEXACT_EXCEPTION;
                // assemble the result
                res.w[1] = x_sign | x_exp | C1_hi;
                res.w[0] = C1_lo;
            } else { // delta = P34
                // in most cases, the smaller operand may be < or = or > 1/2 ulp of the
                // larger operand
                // however, the case C1 = 10^(q1-1) and x_sign != y_sign is special due
                // to accuracy loss after subtraction, and will be treated separately
                if x_sign == y_sign
                || (q1 <= 20 && (C1_hi != 0 || C1_lo != bid_ten2k64[(q1 - 1) as usize]))
                || (q1 >= 21 && (C1_hi != bid_ten2k128[(q1 - 21) as usize].w[1]
                 || C1_lo != bid_ten2k128[(q1 - 21) as usize].w[0])) {
                    // if x_sign == y_sign or C1 != 10^(q1-1)
                    // compare C2 with 1/2 ulp = 5 * 10^(q2-1), the latter read from table
                    // Note: cases q1<=19 and q1>=20 can be coalesced at some latency cost
                    if q2 <= 19 {                         // C2 and 5*10^(q2-1) both fit in 64 bits
                        halfulp64 = bid_midpoint64[(q2 - 1) as usize]; // 5 * 10^(q2-1)
                        if C2_lo < halfulp64 {            // n2 < 1/2 ulp (n1)
                            // for RN the result is the operand with the larger magnitude,
                            // possibly scaled up by 10^(P34-q1)
                            // an overflow cannot occur in this case (rounding to nearest)
                            if q1 < P34 { // scale C1 up by 10^(P34-q1)
                                // Note: because delta = P34 it is certain that
                                //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
                                scale = P34 - q1;
                                if q1 <= 19 { // C1 fits in 64 bits
                                    // 1 <= q1 <= 19 => 15 <= scale <= 33
                                    C1 = if scale <= 19 { // 10^scale fits in 64 bits
                                        __mul_64x64_to_128MACH(bid_ten2k64[scale as usize], C1_lo)
                                    } else { // if 20 <= scale <= 33
                                        // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                                        // (C1 * 10^(scale-19)) fits in 64 bits
                                        C1_lo = C1_lo * bid_ten2k64[(scale - 19) as usize];
                                        __mul_64x64_to_128MACH(bid_ten2k64[19], C1_lo)
                                    };
                                } else { // if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                                    // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                                    C1.w[1] = C1_hi;
                                    C1.w[0] = C1_lo;
                                    // C1 = bid_ten2k64[(P34 - q1) as usize] * C1
                                    C1 = __mul_128x64_to_128(bid_ten2k64[(P34 - q1) as usize], &C1);
                                }
                                x_exp = x_exp - ((scale as BID_UINT64) << 49);
                                C1_hi = C1.w[1];
                                C1_lo = C1.w[0];
                            }
                            if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                                if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN && x_sign != 0 && y_sign != 0) ||
                                    (rnd_mode == RoundingMode::BID_ROUNDING_UP   && x_sign == 0 && y_sign == 0) {
                                    // add 1 ulp and then check for overflow
                                    C1_lo = C1_lo + 1;
                                    if C1_lo == 0 { // rounding overflow in the low 64 bits
                                        C1_hi = C1_hi + 1;
                                    }
                                    if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                        // C1 = 10^34 => rounding overflow
                                        C1_hi = 0x0000314dc6448d93u64;
                                        C1_lo = 0x38c15b0a00000000u64; // 10^33
                                        x_exp = x_exp + EXP_P1;
                                        if x_exp == EXP_MAX_P1 {         // overflow
                                            C1_hi = 0x7800000000000000u64; // +inf
                                            C1_lo = 0x0u64;
                                            x_exp = 0; // x_sign is preserved
                                            // set overflow flag (the inexact flag was set too)
                                            *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                                        }
                                    }
                                } else if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN    && x_sign == 0 && y_sign != 0)
                                        || (rnd_mode == RoundingMode::BID_ROUNDING_UP      && x_sign != 0 && y_sign == 0)
                                        || (rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO && x_sign != y_sign) {
                                    // subtract 1 ulp from C1
                                    // Note: because delta >= P34 + 1 the result cannot be zero
                                    C1_lo = C1_lo - 1;
                                    if C1_lo == 0xffffffffffffffffu64 {
                                        C1_hi = C1_hi - 1;
                                    }
                                    // if the coefficient is 10^33-1 then make it 10^34-1 and
                                    // decrease the exponent by 1 (because delta >= P34 + 1 the
                                    // exponent will not become less than e_min)
                                    // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
                                    // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
                                    if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 {
                                        // make C1 = 10^34  - 1
                                        C1_hi = 0x0001ed09bead87c0u64;
                                        C1_lo = 0x378d8e63ffffffffu64;
                                        x_exp = x_exp - EXP_P1;
                                    }
                                } else {
                                    // the result is already correct
                                }
                            }
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            // assemble the result
                            res.w[1] = x_sign | x_exp | C1_hi;
                            res.w[0] = C1_lo;
                        } else if (C2_lo == halfulp64) && (q1 < P34 || ((C1_lo & 0x1) == 0)) {
                            // n2 = 1/2 ulp (n1) and q1 < P34 or C1 is even
                            // the result is the operand with the larger magnitude,
                            // possibly scaled up by 10^(P34-q1)
                            // an overflow cannot occur in this case (rounding to nearest)
                            if q1 < P34 { // scale C1 up by 10^(P34-q1)
                                // Note: because delta = P34 it is certain that
                                //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
                                scale = P34 - q1;
                                if q1 <= 19 { // C1 fits in 64 bits
                                    // 1 <= q1 <= 19 => 15 <= scale <= 33
                                    C1 = if scale <= 19 { // 10^scale fits in 64 bits
                                        __mul_64x64_to_128MACH(bid_ten2k64[scale as usize], C1_lo)
                                    } else { // if 20 <= scale <= 33
                                        // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                                        // (C1 * 10^(scale-19)) fits in 64 bits
                                        C1_lo = C1_lo * bid_ten2k64[(scale - 19) as usize];
                                        __mul_64x64_to_128MACH(bid_ten2k64[19], C1_lo)
                                    };
                                } else { // if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                                    // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                                    C1.w[1] = C1_hi;
                                    C1.w[0] = C1_lo;
                                    // C1 = bid_ten2k64[(P34 - q1) as usize] * C1
                                    C1 = __mul_128x64_to_128(bid_ten2k64[(P34 - q1) as usize], &C1);
                                }
                                x_exp = x_exp - ((scale as BID_UINT64) << 49);
                                C1_hi = C1.w[1];
                                C1_lo = C1.w[0];
                            }
                            if (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && x_sign == y_sign && (C1_lo & 0x01) == 0x01)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY  && x_sign == y_sign)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_UP         && x_sign == 0 && y_sign == 0)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_DOWN       && x_sign != 0 && y_sign != 0) {
                                // add 1 ulp and then check for overflow
                                C1_lo = C1_lo + 1;
                                if C1_lo == 0 { // rounding overflow in the low 64 bits
                                    C1_hi = C1_hi + 1;
                                }
                                if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                    // C1 = 10^34 => rounding overflow
                                    C1_hi = 0x0000314dc6448d93u64;
                                    C1_lo = 0x38c15b0a00000000u64; // 10^33
                                    x_exp = x_exp + EXP_P1;
                                    if x_exp == EXP_MAX_P1 {         // overflow
                                        C1_hi = 0x7800000000000000u64; // +inf
                                        C1_lo = 0x0u64;
                                        x_exp = 0; // x_sign is preserved
                                        // set overflow flag (the inexact flag was set too)
                                        *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                                    }
                                }
                            } else if (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && x_sign != y_sign && (C1_lo & 0x01) == 0x01)
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_DOWN       && x_sign == 0 && y_sign != 0)
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_UP         && x_sign != 0 && y_sign == 0)
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO    && x_sign != y_sign) {
                                // subtract 1 ulp from C1
                                // Note: because delta >= P34 + 1 the result cannot be zero
                                C1_lo = C1_lo - 1;
                                if C1_lo == 0xffffffffffffffffu64 {
                                    C1_hi = C1_hi - 1;
                                }
                                // if the coefficient is 10^33 - 1 then make it 10^34 - 1
                                // and decrease the exponent by 1 (because delta >= P34 + 1
                                // the exponent will not become less than e_min)
                                // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
                                // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
                                if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 {
                                    // make C1 = 10^34  - 1
                                    C1_hi = 0x0001ed09bead87c0u64;
                                    C1_lo = 0x378d8e63ffffffffu64;
                                    x_exp = x_exp - EXP_P1;
                                }
                            } else {
                                // the result is already correct
                            }
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            // assemble the result
                            res.w[1] = x_sign | x_exp | C1_hi;
                            res.w[0] = C1_lo;
                        } else { // if C2_lo > halfulp64 ||
                            // (C2_lo == halfulp64 && q1 == P34 && ((C1_lo & 0x1) == 1)), i.e.
                            // 1/2 ulp(n1) < n2 < 1 ulp(n1) or n2 = 1/2 ulp(n1) and C1 odd
                            // res = x+1 ulp if n1*n2 > 0 and res = x-1 ulp if n1*n2 < 0
                            if q1 < P34 { // then 1 ulp = 10^(e1+q1-P34) < 10^e1
                                // Note: if (q1 == P34) then 1 ulp = 10^(e1+q1-P34) = 10^e1
                                // because q1 < P34 we must first replace C1 by
                                // C1 * 10^(P34-q1), and must decrease the exponent by
                                // (P34-q1) (it will still be at least e_min)
                                scale = P34 - q1;
                                if q1 <= 19 { // C1 fits in 64 bits
                                    // 1 <= q1 <= 19 => 15 <= scale <= 33
                                    C1 = if scale <= 19 { // 10^scale fits in 64 bits
                                        __mul_64x64_to_128MACH(bid_ten2k64[scale as usize], C1_lo)
                                    } else { // if 20 <= scale <= 33
                                        // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                                        // (C1 * 10^(scale-19)) fits in 64 bits
                                        C1_lo = C1_lo * bid_ten2k64[(scale - 19) as usize];
                                        __mul_64x64_to_128MACH(bid_ten2k64[19], C1_lo)
                                    };
                                } else { // if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                                    // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                                    C1.w[1] = C1_hi;
                                    C1.w[0] = C1_lo;
                                    // C1 = bid_ten2k64[(P34 - q1) as usize] * C1
                                    C1 = __mul_128x64_to_128(bid_ten2k64[(P34 - q1) as usize], &C1);
                                }
                                x_exp = x_exp - ((scale as BID_UINT64) << 49);
                                C1_hi = C1.w[1];
                                C1_lo = C1.w[0];
                                // check for rounding overflow
                                if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                    // C1 = 10^34 => rounding overflow
                                    C1_hi = 0x0000314dc6448d93u64;
                                    C1_lo = 0x38c15b0a00000000u64; // 10^33
                                    x_exp = x_exp + EXP_P1;
                                }
                            }
                            if (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && x_sign != y_sign)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY  && x_sign != y_sign && C2_lo != halfulp64)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_DOWN       && x_sign == 0 && y_sign != 0)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_UP         && x_sign != 0 && y_sign == 0)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO    && x_sign != y_sign) {
                                // the result is x - 1
                                // for RN n1 * n2 < 0; underflow not possible
                                C1_lo = C1_lo - 1;
                                if C1_lo == 0xffffffffffffffffu64 {
                                    C1_hi -= 1;
                                }
                                // check if we crossed into the lower decade
                                if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 { // 10^33 - 1
                                    C1_hi = 0x0001ed09bead87c0u64;                                      // 10^34 - 1
                                    C1_lo = 0x378d8e63ffffffffu64;
                                    x_exp = x_exp - EXP_P1; // no underflow, because n1 >> n2
                                }
                            } else if (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && x_sign == y_sign)
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY  && x_sign == y_sign)
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_DOWN       && x_sign != 0 && y_sign != 0)
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_UP         && x_sign == 0 && y_sign == 0) {
                                // the result is x + 1
                                // for RN x_sign = y_sign, i.e. n1*n2 > 0
                                C1_lo = C1_lo + 1;
                                if C1_lo == 0 { // rounding overflow in the low 64 bits
                                    C1_hi = C1_hi + 1;
                                }
                                if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                    // C1 = 10^34 => rounding overflow
                                    C1_hi = 0x0000314dc6448d93u64;
                                    C1_lo = 0x38c15b0a00000000u64;     // 10^33
                                    x_exp = x_exp + EXP_P1;
                                    if x_exp == EXP_MAX_P1 {         // overflow
                                        C1_hi = 0x7800000000000000u64; // +inf
                                        C1_lo = 0x0u64;
                                        x_exp = 0; // x_sign is preserved
                                        // set the overflow flag
                                        *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                                    }
                                }
                            } else {
                                // the result is x
                            }
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            // assemble the result
                            res.w[1] = x_sign | x_exp | C1_hi;
                            res.w[0] = C1_lo;
                        }
                    } else { // if q2 >= 20 then 5*10^(q2-1) and C2 (the latter in
                        // most cases) fit only in more than 64 bits
                        halfulp128 = &bid_midpoint128[(q2 - 20) as usize]; // 5 * 10^(q2-1)
                        if (C2_hi < halfulp128.w[1]) || (C2_hi == halfulp128.w[1] && C2_lo < halfulp128.w[0]) {
                            // n2 < 1/2 ulp (n1)
                            // the result is the operand with the larger magnitude,
                            // possibly scaled up by 10^(P34-q1)
                            // an overflow cannot occur in this case (rounding to nearest)
                            if q1 < P34 { // scale C1 up by 10^(P34-q1)
                                // Note: because delta = P34 it is certain that
                                //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
                                scale = P34 - q1;
                                if q1 <= 19 { // C1 fits in 64 bits
                                    // 1 <= q1 <= 19 => 15 <= scale <= 33
                                    C1 = if scale <= 19 { // 10^scale fits in 64 bits
                                        __mul_64x64_to_128MACH(bid_ten2k64[scale as usize], C1_lo)
                                    } else { // if 20 <= scale <= 33
                                        // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                                        // (C1 * 10^(scale-19)) fits in 64 bits
                                        C1_lo = C1_lo * bid_ten2k64[(scale - 19) as usize];
                                        __mul_64x64_to_128MACH(bid_ten2k64[19], C1_lo)
                                    };
                                } else { // if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                                    // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                                    C1.w[1] = C1_hi;
                                    C1.w[0] = C1_lo;
                                    // C1 = bid_ten2k64[(P34 - q1) as usize] * C1
                                    C1 = __mul_128x64_to_128(bid_ten2k64[(P34 - q1) as usize], &C1);
                                }
                                C1_hi = C1.w[1];
                                C1_lo = C1.w[0];
                                x_exp = x_exp - ((scale as BID_UINT64) << 49);
                            }
                            if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                                if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN && x_sign != 0 && y_sign != 0)
                                 || (rnd_mode == RoundingMode::BID_ROUNDING_UP   && x_sign == 0 && y_sign == 0) {
                                    // add 1 ulp and then check for overflow
                                    C1_lo = C1_lo.wrapping_add(1);
                                    if C1_lo == 0 { // rounding overflow in the low 64 bits
                                        C1_hi = C1_hi.wrapping_add(1);
                                    }
                                    if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                        // C1 = 10^34 => rounding overflow
                                        C1_hi = 0x0000314dc6448d93u64;
                                        C1_lo = 0x38c15b0a00000000u64; // 10^33
                                        x_exp = x_exp + EXP_P1;
                                        if x_exp == EXP_MAX_P1 {         // overflow
                                            C1_hi = 0x7800000000000000u64; // +inf
                                            C1_lo = 0x0u64;
                                            x_exp = 0; // x_sign is preserved
                                            // set overflow flag (the inexact flag was set too)
                                            *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                                        }
                                    }
                                } else if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN    && x_sign == 0 && y_sign != 0)
                                        || (rnd_mode == RoundingMode::BID_ROUNDING_UP      && x_sign != 0 && y_sign == 0)
                                        || (rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO && x_sign != y_sign) {
                                    // subtract 1 ulp from C1
                                    // Note: because delta >= P34 + 1 the result cannot be zero
                                    C1_lo = C1_lo - 1;
                                    if C1_lo == 0xffffffffffffffffu64 {
                                        C1_hi = C1_hi - 1;
                                    }
                                    // if the coefficient is 10^33-1 then make it 10^34-1 and
                                    // decrease the exponent by 1 (because delta >= P34 + 1 the
                                    // exponent will not become less than e_min)
                                    // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
                                    // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
                                    if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 {
                                        // make C1 = 10^34  - 1
                                        C1_hi = 0x0001ed09bead87c0u64;
                                        C1_lo = 0x378d8e63ffffffffu64;
                                        x_exp = x_exp - EXP_P1;
                                    }
                                } else {
                                    // the result is already correct
                                }
                            }
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            // assemble the result
                            res.w[1] = x_sign | x_exp | C1_hi;
                            res.w[0] = C1_lo;
                        } else if (C2_hi == halfulp128.w[1] && C2_lo == halfulp128.w[0]) && (q1 < P34 || ((C1_lo & 0x1) == 0)) {
                            // set the inexact flag
                            // midpoint & lsb in C1 is 0
                            // n2 = 1/2 ulp (n1) and C1 is even
                            // the result is the operand with the larger magnitude,
                            // possibly scaled up by 10^(P34-q1)
                            // an overflow cannot occur in this case (rounding to nearest)
                            if q1 < P34 { // scale C1 up by 10^(P34-q1)
                                // Note: because delta = P34 it is certain that
                                //     x_exp - ((BID_UINT64)scale << 49) will stay above e_min
                                scale = P34 - q1;
                                if q1 <= 19 { // C1 fits in 64 bits
                                    // 1 <= q1 <= 19 => 15 <= scale <= 33
                                    C1 = if scale <= 19 { // 10^scale fits in 64 bits
                                        __mul_64x64_to_128MACH(bid_ten2k64[scale as usize], C1_lo)
                                    } else { // if 20 <= scale <= 33
                                        // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                                        // (C1 * 10^(scale-19)) fits in 64 bits
                                        C1_lo = C1_lo * bid_ten2k64[(scale - 19) as usize];
                                        __mul_64x64_to_128MACH(bid_ten2k64[19], C1_lo)
                                    };
                                } else { // if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                                    // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                                    C1.w[1] = C1_hi;
                                    C1.w[0] = C1_lo;
                                    // C1 = bid_ten2k64[(P34 - q1) as usize] * C1
                                    C1 = __mul_128x64_to_128(bid_ten2k64[(P34 - q1) as usize], &C1);
                                }
                                x_exp = x_exp - ((scale as BID_UINT64) << 49);
                                C1_hi = C1.w[1];
                                C1_lo = C1.w[0];
                            }
                            if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                                if (rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY && x_sign == y_sign)
                                 || (rnd_mode == RoundingMode::BID_ROUNDING_UP        && x_sign == 0 && y_sign == 0)
                                 || (rnd_mode == RoundingMode::BID_ROUNDING_DOWN      && x_sign != 0 && y_sign != 0) {
                                    // add 1 ulp and then check for overflow
                                    C1_lo = C1_lo + 1;
                                    if C1_lo == 0 { // rounding overflow in the low 64 bits
                                        C1_hi = C1_hi + 1;
                                    }
                                    if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                        // C1 = 10^34 => rounding overflow
                                        C1_hi = 0x0000314dc6448d93u64;
                                        C1_lo = 0x38c15b0a00000000u64; // 10^33
                                        x_exp = x_exp + EXP_P1;
                                        if x_exp == EXP_MAX_P1 {         // overflow
                                            C1_hi = 0x7800000000000000u64; // +inf
                                            C1_lo = 0x0u64;
                                            x_exp = 0; // x_sign is preserved
                                            // set overflow flag (the inexact flag was set too)
                                            *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                                        }
                                    }
                                } else if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN    && x_sign == 0 && y_sign != 0)
                                        || (rnd_mode == RoundingMode::BID_ROUNDING_UP      && x_sign != 0 && y_sign == 0)
                                        || (rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO && x_sign != y_sign) {
                                    // subtract 1 ulp from C1
                                    // Note: because delta >= P34 + 1 the result cannot be zero
                                    C1_lo = C1_lo - 1;
                                    if C1_lo == 0xffffffffffffffffu64 {
                                        C1_hi = C1_hi - 1;
                                    }
                                    // if the coefficient is 10^33 - 1 then make it 10^34 - 1
                                    // and decrease the exponent by 1 (because delta >= P34 + 1
                                    // the exponent will not become less than e_min)
                                    // 10^33 - 1 = 0x0000314dc6448d9338c15b09ffffffff
                                    // 10^34 - 1 = 0x0001ed09bead87c0378d8e63ffffffff
                                    if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 {
                                        // make C1 = 10^34  - 1
                                        C1_hi = 0x0001ed09bead87c0u64;
                                        C1_lo = 0x378d8e63ffffffffu64;
                                        x_exp = x_exp - EXP_P1;
                                    }
                                } else {
                                    // the result is already correct
                                }
                            }
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            // assemble the result
                            res.w[1] = x_sign | x_exp | C1_hi;
                            res.w[0] = C1_lo;
                        } else { // if C2 > halfulp128 ||
                            // (C2 == halfulp128 && q1 == P34 && ((C1 & 0x1) == 1)), i.e.
                            // 1/2 ulp(n1) < n2 < 1 ulp(n1) or n2 = 1/2 ulp(n1) and C1 odd
                            // res = x+1 ulp if n1*n2 > 0 and res = x-1 ulp if n1*n2 < 0
                            if q1 < P34 { // then 1 ulp = 10^(e1+q1-P34) < 10^e1
                                // Note: if (q1 == P34) then 1 ulp = 10^(e1+q1-P34) = 10^e1
                                // because q1 < P34 we must first replace C1 by C1*10^(P34-q1),
                                // and must decrease the exponent by (P34-q1) (it will still be
                                // at least e_min)
                                scale = P34 - q1;
                                if q1 <= 19 { // C1 fits in 64 bits
                                    // 1 <= q1 <= 19 => 15 <= scale <= 33
                                    C1 = if scale <= 19 { // 10^scale fits in 64 bits
                                        __mul_64x64_to_128MACH(bid_ten2k64[scale as usize], C1_lo)
                                    } else { // if 20 <= scale <= 33
                                        // C1 * 10^scale = (C1 * 10^(scale-19)) * 10^19 where
                                        // (C1 * 10^(scale-19)) fits in 64 bits
                                        C1_lo = C1_lo * bid_ten2k64[(scale - 19) as usize];
                                        __mul_64x64_to_128MACH(bid_ten2k64[19], C1_lo)
                                    };
                                } else { // if 20 <= q1 <= 33=P34-1 then C1 fits only in 128 bits
                                    // => 1 <= P34 - q1 <= 14 so 10^(P34-q1) fits in 64 bits
                                    C1.w[1] = C1_hi;
                                    C1.w[0] = C1_lo;
                                    // C1 = bid_ten2k64[(P34 - q1) as usize] * C1
                                    C1      = __mul_128x64_to_128(bid_ten2k64[(P34 - q1) as usize], &C1);
                                }
                                C1_hi = C1.w[1];
                                C1_lo = C1.w[0];
                                x_exp = x_exp - ((scale as BID_UINT64) << 49);
                            }
                            if (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && x_sign != y_sign)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY  && x_sign != y_sign && (C2_hi != halfulp128.w[1] || C2_lo != halfulp128.w[0]))
                             || (rnd_mode == RoundingMode::BID_ROUNDING_DOWN       && x_sign == 0 && y_sign != 0)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_UP         && x_sign != 0 && y_sign == 0)
                             || (rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO    && x_sign != y_sign) {
                                // the result is x - 1
                                // for RN n1 * n2 < 0; underflow not possible
                                C1_lo = C1_lo - 1;
                                if C1_lo == 0xffffffffffffffffu64 {
                                    C1_hi -= 1;
                                }
                                // check if we crossed into the lower decade
                                if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 { // 10^33 - 1
                                    C1_hi = 0x0001ed09bead87c0u64;                                      // 10^34 - 1
                                    C1_lo = 0x378d8e63ffffffffu64;
                                    x_exp = x_exp - EXP_P1; // no underflow, because n1 >> n2
                                }
                            } else if (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && x_sign == y_sign)
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY  && x_sign == y_sign)
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_DOWN       && x_sign != 0 && y_sign != 0)
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_UP         && x_sign == 0 && y_sign == 0) {
                                // the result is x + 1
                                // for RN x_sign = y_sign, i.e. n1*n2 > 0
                                C1_lo = C1_lo.wrapping_add(1);
                                if C1_lo == 0 { // rounding overflow in the low 64 bits
                                    C1_hi = C1_hi.wrapping_add(1);
                                }
                                if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                    // C1 = 10^34 => rounding overflow
                                    C1_hi = 0x0000314dc6448d93u64;
                                    C1_lo = 0x38c15b0a00000000u64; // 10^33
                                    x_exp = x_exp + EXP_P1;
                                    if x_exp == EXP_MAX_P1 {         // overflow
                                        C1_hi = 0x7800000000000000u64; // +inf
                                        C1_lo = 0x0u64;
                                        x_exp = 0; // x_sign is preserved
                                        // set the overflow flag
                                        *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                                    }
                                }
                            } else {
                                // the result is x
                            }
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            // assemble the result
                            res.w[1] = x_sign | x_exp | C1_hi;
                            res.w[0] = C1_lo;
                        }
                    }    // end q1 >= 20
                         // end case where C1 != 10^(q1-1)
                } else { // C1 = 10^(q1-1) and x_sign != y_sign
                    // instead of C' = (C1 * 10^(e1-e2) + C2)rnd,P34
                    // calculate C' = C1 * 10^(e1-e2-x1) + (C2 * 10^(-x1))rnd,P34
                    // where x1 = q2 - 1, 0 <= x1 <= P34 - 1
                    // Because C1 = 10^(q1-1) and x_sign != y_sign, C' will have P34
                    // digits and n = C' * 10^(e2+x1)
                    // If the result has P34+1 digits, redo the steps above with x1+1
                    // If the result has P34-1 digits or less, redo the steps above with
                    // x1-1 but only if initially x1 >= 1
                    // NOTE: these two steps can be improved, e.g we could guess if
                    // P34+1 or P34-1 digits will be obtained by adding/subtracting
                    // just the top 64 bits of the two operands
                    // The result cannot be zero, and it cannot overflow
                    x1 = q2 - 1; // 0 <= x1 <= P34-1
                    // Calculate C1 * 10^(e1-e2-x1) where 1 <= e1-e2-x1 <= P34
                    // scale = (int)(e1 >> 49) - (int)(e2 >> 49) - x1; 0 <= scale <= P34-1
                    scale = P34 - q1 + 1; // scale=e1-e2-x1 = P34+1-q1; 1<=scale<=P34
                    // either C1 or 10^(e1-e2-x1) may not fit is 64 bits,
                    // but their product fits with certainty in 128 bits
                    C1 = if scale >= 20 { // 10^(e1-e2-x1) doesn't fit in 64 bits, but C1 does
                        __mul_128x64_to_128(C1_lo, &bid_ten2k128[(scale - 20) as usize])
                    } else { // if (scale >= 1
                        // if 1 <= scale <= 19 then 10^(e1-e2-x1) fits in 64 bits
                        if q1 <= 19 { // C1 fits in 64 bits
                            __mul_64x64_to_128MACH(C1_lo, bid_ten2k64[scale as usize])
                        } else { // q1 >= 20
                            C1.w[1] = C1_hi;
                            C1.w[0] = C1_lo;
                            __mul_128x64_to_128(bid_ten2k64[scale as usize], &C1)
                        }
                    };
                    tmp64 = C1.w[0]; // C1.w[1], C1.w[0] contains C1 * 10^(e1-e2-x1)

                    // now round C2 to q2-x1 = 1 decimal digit
                    // C2' = C2 + 1/2 * 10^x1 = C2 + 5 * 10^(x1-1)
                    ind = x1 - 1;   // -1 <= ind <= P34 - 2
                    if ind >= 0 { // if (x1 >= 1)
                        C2.w[0] = C2_lo;
                        C2.w[1] = C2_hi;
                        if ind <= 18 {
                            C2.w[0] = C2.w[0] + bid_midpoint64[ind as usize];
                            if C2.w[0] < C2_lo {
                                C2.w[1] += 1;
                            }
                        } else { // 19 <= ind <= 32
                            C2.w[0] = C2.w[0] + bid_midpoint128[(ind - 19) as usize].w[0];
                            C2.w[1] = C2.w[1] + bid_midpoint128[(ind - 19) as usize].w[1];
                            if C2.w[0] < C2_lo {
                                C2.w[1] += 1;
                            }
                        }
                        // the approximation of 10^(-x1) was rounded up to 118 bits
                        R256 = __mul_128x128_to_256(&C2, &bid_ten2mk128[ind as usize]); // R256 = C2*, f2*
                        // calculate C2* and f2*
                        // C2* is actually floor(C2*) in this case
                        // C2* and f2* need shifting and masking, as shown by
                        // bid_shiftright128[] and bid_maskhigh128[]
                        // the top Ex bits of 10^(-x1) are T* = bid_ten2mk128trunc[(ind) as usize], e.g.
                        // if x1=1, T*=bid_ten2mk128trunc[0]=0x19999999999999999999999999999999
                        // if (0 < f2* < 10^(-x1)) then
                        //   if floor(C1+C2*) is even then C2* = floor(C2*) - logical right
                        //       shift; C2* has p decimal digits, correct by Prop. 1)
                        //   else if floor(C1+C2*) is odd C2* = floor(C2*)-1 (logical right
                        //       shift; C2* has p decimal digits, correct by Pr. 1)
                        // else
                        //   C2* = floor(C2*) (logical right shift; C has p decimal digits,
                        //       correct by Property 1)
                        // n = C2* * 10^(e2+x1)

                        if ind <= 2 {
                            highf2star.w[1] = 0x0;
                            highf2star.w[0] = 0x0; // low f2* ok
                        } else if ind <= 21 {
                            highf2star.w[1] = 0x0;
                            highf2star.w[0] = R256.w[2] & bid_maskhigh128[ind as usize]; // low f2* ok
                        } else {
                            highf2star.w[1] = R256.w[3] & bid_maskhigh128[ind as usize];
                            highf2star.w[0] = R256.w[2]; // low f2* is ok
                        }
                        // shift right C2* by Ex-128 = bid_shiftright128[(ind) as usize]
                        if ind >= 3 {
                            shift = bid_shiftright128[ind as usize];
                            if shift < 64 { // 3 <= shift <= 63
                                R256.w[2] = (R256.w[2] >> shift) | (R256.w[3] << (64 - shift));
                                R256.w[3] = R256.w[3] >> shift;
                            } else { // 66 <= shift <= 102
                                R256.w[2] = R256.w[3] >> (shift - 64);
                                R256.w[3] = 0x0u64;
                            }
                        }
                        // redundant
                        is_inexact_lt_midpoint = false;
                        is_inexact_gt_midpoint = false;
                        is_midpoint_lt_even    = false;
                        is_midpoint_gt_even    = false;
                        // determine inexactness of the rounding of C2*
                        // (cannot be followed by a second rounding)
                        // if (0 < f2* - 1/2 < 10^(-x1)) then
                        //   the result is exact
                        // else (if f2* - 1/2 > T* then)
                        //   the result of is inexact
                        if ind <= 2 {
                            if R256.w[1] > 0x8000000000000000u64 || (R256.w[1] == 0x8000000000000000u64 && R256.w[0] > 0x0u64) {
                                // f2* > 1/2 and the result may be exact
                                tmp64A = R256.w[1] - 0x8000000000000000u64; // f* - 1/2
                                if tmp64A      > bid_ten2mk128trunc[ind as usize].w[1]
                                || (tmp64A    == bid_ten2mk128trunc[ind as usize].w[1]
                                 && R256.w[0] >= bid_ten2mk128trunc[ind as usize].w[0]) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                    // this rounding is applied to C2 only!
                                    // x_sign != y_sign
                                    is_inexact_gt_midpoint = true;
                                }    // else the result is exact
                                     // rounding down, unless a midpoint in [ODD, EVEN]
                            } else { // the result is inexact; f2* <= 1/2
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                // this rounding is applied to C2 only!
                                // x_sign != y_sign
                                is_inexact_lt_midpoint = true;
                            }
                        } else if ind <= 21 { // if 3 <= ind <= 21
                            if highf2star.w[1]  > 0x0 || (highf2star.w[1] == 0x0 && highf2star.w[0] > bid_onehalf128[ind as usize])
                            || (highf2star.w[1] == 0x0 &&  highf2star.w[0] == bid_onehalf128[ind as usize] && (R256.w[1] != 0 || R256.w[0] != 0)) {
                                // f2* > 1/2 and the result may be exact
                                // Calculate f2* - 1/2
                                tmp64A = highf2star.w[0] - bid_onehalf128[ind as usize];
                                tmp64B = highf2star.w[1];
                                if tmp64A > highf2star.w[0] {
                                    tmp64B -= 1;
                                }
                                if  tmp64B != 0 || tmp64A != 0
                                 || R256.w[1]  > bid_ten2mk128trunc[ind as usize].w[1]
                                || (R256.w[1] == bid_ten2mk128trunc[ind as usize].w[1]
                                 && R256.w[0]  > bid_ten2mk128trunc[ind as usize].w[0]) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                    // this rounding is applied to C2 only!
                                    // x_sign != y_sign
                                    is_inexact_gt_midpoint = true;
                                }    // else the result is exact
                            } else { // the result is inexact; f2* <= 1/2
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                // this rounding is applied to C2 only!
                                // x_sign != y_sign
                                is_inexact_lt_midpoint = true;
                            }
                        } else { // if 22 <= ind <= 33
                            if  highf2star.w[1]  > bid_onehalf128[ind as usize]
                            || (highf2star.w[1] == bid_onehalf128[ind as usize]
                            && (highf2star.w[0] != 0 || R256.w[1] != 0 || R256.w[0] != 0)) {
                                // f2* > 1/2 and the result may be exact
                                // Calculate f2* - 1/2
                                // tmp64A = highf2star.w[0];
                                tmp64B = highf2star.w[1] - bid_onehalf128[ind as usize];
                                if  tmp64B != 0 || highf2star.w[0] != 0
                                 || R256.w[1]  > bid_ten2mk128trunc[ind as usize].w[1]
                                || (R256.w[1] == bid_ten2mk128trunc[ind as usize].w[1]
                                 && R256.w[0]  > bid_ten2mk128trunc[ind as usize].w[0]) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                    // this rounding is applied to C2 only!
                                    // x_sign != y_sign
                                    is_inexact_gt_midpoint = true;
                                }    // else the result is exact
                            } else { // the result is inexact; f2* <= 1/2
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                // this rounding is applied to C2 only!
                                // x_sign != y_sign
                                is_inexact_lt_midpoint = true;
                            }
                        }
                        // check for midpoints after determining inexactness
                        if (R256.w[1] != 0 || R256.w[0] != 0) && (highf2star.w[1] == 0) && (highf2star.w[0] == 0)
                        && (R256.w[1]  < bid_ten2mk128trunc[ind as usize].w[1]
                        || (R256.w[1] == bid_ten2mk128trunc[ind as usize].w[1]
                         && R256.w[0] <= bid_ten2mk128trunc[ind as usize].w[0])) {
                            // the result is a midpoint
                            if ((tmp64 + R256.w[2]) & 0x01) == 0x01 { // MP in [EVEN, ODD]
                                // if floor(C2*) is odd C = floor(C2*) - 1; the result may be 0
                                R256.w[2] -= 1;
                                if R256.w[2] == 0xffffffffffffffffu64 {
                                    R256.w[3] -= 1;
                                }
                                // this rounding is applied to C2 only!
                                // x_sign != y_sign
                                is_midpoint_lt_even    = true;
                                is_inexact_lt_midpoint = false;
                                is_inexact_gt_midpoint = false;
                            } else {
                                // else MP in [ODD, EVEN]
                                // this rounding is applied to C2 only!
                                // x_sign != y_sign
                                is_midpoint_gt_even    = true;
                                is_inexact_lt_midpoint = false;
                                is_inexact_gt_midpoint = false;
                            }
                        }
                    } else { // if (ind == -1) only when x1 = 0
                        R256.w[2] = C2_lo;
                        R256.w[3] = C2_hi;
                        is_midpoint_lt_even    = false;
                        is_midpoint_gt_even    = false;
                        is_inexact_lt_midpoint = false;
                        is_inexact_gt_midpoint = false;
                    }
                    // and now subtract C1 * 10^(e1-e2-x1) - (C2 * 10^(-x1))rnd,P34
                    // because x_sign != y_sign this last operation is exact
                    C1.w[0] = C1.w[0] - R256.w[2];
                    C1.w[1] = C1.w[1] - R256.w[3];
                    if C1.w[0] > tmp64 {
                        C1.w[1] -= 1;                       // borrow
                    }
                    if C1.w[1] >= 0x8000000000000000u64 { // negative coefficient!
                        C1.w[0]  = !C1.w[0];
                        C1.w[0] += 1;
                        C1.w[1]  = !C1.w[1];
                        if C1.w[0] == 0x0 {
                            C1.w[1] += 1;
                        }
                        tmp_sign = y_sign; // the result will have the sign of y
                    } else {
                        tmp_sign = x_sign;
                    }
                    // the difference has exactly P34 digits
                    x_sign = tmp_sign;
                    if x1 >= 1 {
                        y_exp = y_exp + ((x1 as BID_UINT64) << 49);
                    }
                    C1_hi = C1.w[1];
                    C1_lo = C1.w[0];
                    // general correction from RN to RA, RM, RP, RZ; result uses y_exp
                    if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                        if (x_sign == 0 && ((rnd_mode == RoundingMode::BID_ROUNDING_UP && is_inexact_lt_midpoint)
                                        || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
                                          || rnd_mode == RoundingMode::BID_ROUNDING_UP) && is_midpoint_gt_even)))
                        || (x_sign != 0 && ((rnd_mode == RoundingMode::BID_ROUNDING_DOWN && is_inexact_lt_midpoint)
                                        || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
                                          || rnd_mode == RoundingMode::BID_ROUNDING_DOWN) && is_midpoint_gt_even))) {
                            // C1 = C1 + 1
                            C1_lo = C1_lo + 1;
                            if C1_lo == 0 { // rounding overflow in the low 64 bits
                                C1_hi = C1_hi + 1;
                            }
                            if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                // C1 = 10^34 => rounding overflow
                                C1_hi = 0x0000314dc6448d93u64;
                                C1_lo = 0x38c15b0a00000000u64; // 10^33
                                y_exp = y_exp + EXP_P1;
                            }
                        } else if (is_midpoint_lt_even || is_inexact_gt_midpoint)
                               && ((x_sign != 0 && (rnd_mode == RoundingMode::BID_ROUNDING_UP
                                                 || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))
                                || (x_sign != 0 && (rnd_mode == RoundingMode::BID_ROUNDING_DOWN
                                                 || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))) {
                            // C1 = C1 - 1
                            C1_lo = C1_lo - 1;
                            if C1_lo == 0xffffffffffffffffu64 {
                                C1_hi -= 1;
                            }
                            // check if we crossed into the lower decade
                            if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 { // 10^33 - 1
                                C1_hi = 0x0001ed09bead87c0u64;                                      // 10^34 - 1
                                C1_lo = 0x378d8e63ffffffffu64;
                                y_exp = y_exp - EXP_P1;
                                // no underflow, because delta + q2 >= P34 + 1
                            }
                        } else {
                            // exact, the result is already correct
                        }
                    }
                    // assemble the result
                    res.w[1] = x_sign | y_exp | C1_hi;
                    res.w[0] = C1_lo;
                }
            }                 // end delta = P34
        } else {              // if (|delta| <= P34 - 1)
            if delta >= 0 { // if (0 <= delta <= P34 - 1)
                if delta <= P34 - 1 - q2 {
                    // calculate C' directly; the result is exact
                    // in this case 1<=q1<=P34-1, 1<=q2<=P34-1 and 0 <= e1-e2 <= P34-2
                    // The coefficient of the result is C1 * 10^(e1-e2) + C2 and the
                    // exponent is e2; either C1 or 10^(e1-e2) may not fit is 64 bits,
                    // but their product fits with certainty in 128 bits (actually in 113)
                    scale = delta - q1 + q2; // scale = (int)(e1 >> 49) - (int)(e2 >> 49)

                    if scale >= 20 { // 10^(e1-e2) does not fit in 64 bits, but C1 does
                        C1 = __mul_128x64_to_128(C1_lo, &bid_ten2k128[(scale - 20) as usize]);
                        C1_hi = C1.w[1];
                        C1_lo = C1.w[0];
                    } else if scale >= 1 {
                        // if 1 <= scale <= 19 then 10^(e1-e2) fits in 64 bits
                        C1 = if q1 <= 19 { // C1 fits in 64 bits
                            __mul_64x64_to_128MACH(C1_lo, bid_ten2k64[scale as usize])
                        } else { // q1 >= 20
                            C1.w[1] = C1_hi;
                            C1.w[0] = C1_lo;
                            __mul_128x64_to_128(bid_ten2k64[scale as usize], &C1)
                        };
                        C1_hi = C1.w[1];
                        C1_lo = C1.w[0];
                    } else {             // if (scale == 0) C1 is unchanged
                        C1.w[0] = C1_lo; // C1.w[1] = C1_hi;
                    }
                    // now add C2
                    if x_sign == y_sign {
                        // the result cannot overflow
                        C1_lo = (C1_lo as u128 + C2_lo as u128) as BID_UINT64;
                        C1_hi = (C1_hi as u128 + C2_hi as u128) as BID_UINT64;
                        if C1_lo < C1.w[0] {
                            C1_hi += 1;
                        }
                    } else { // if x_sign != y_sign
                        C1_lo = C1_lo.wrapping_sub(C2_lo);
                        C1_hi = C1_hi.wrapping_sub(C2_hi);
                        if C1_lo > C1.w[0] {
                            C1_hi = C1_hi.wrapping_sub(1);
                        }
                        // the result can be zero, but it cannot overflow
                        if C1_lo == 0 && C1_hi == 0 {
                            // assemble the result
                            res.w[1] = if x_exp < y_exp { x_exp }  else {  y_exp };
                            res.w[0] = 0;
                            if rnd_mode == RoundingMode::BID_ROUNDING_DOWN {
                                res.w[1] |= 0x8000000000000000u64;
                            }

                            #[cfg(target_endian = "big")]
                            BID_SWAP128(&mut res);

                            return res;
                        }
                        if C1_hi >= 0x8000000000000000u64 { // negative coefficient!
                            C1_lo  = !C1_lo;
                            C1_lo += 1;
                            C1_hi  = !C1_hi;
                            if C1_lo == 0x0 {
                                C1_hi += 1;
                            }
                            x_sign = y_sign; // the result will have the sign of y
                        }
                    }
                    // assemble the result
                    res.w[1] = x_sign | y_exp | C1_hi;
                    res.w[0] = C1_lo;
                } else if delta == P34 - q2 {
                    // calculate C' directly; the result may be inexact if it requires
                    // P34+1 decimal digits; in this case the 'cutoff' point for addition
                    // is at the position of the lsb of C2, so 0 <= e1-e2 <= P34-1
                    // The coefficient of the result is C1 * 10^(e1-e2) + C2 and the
                    // exponent is e2; either C1 or 10^(e1-e2) may not fit is 64 bits,
                    // but their product fits with certainty in 128 bits (actually in 113)
                    scale = delta - q1 + q2; // scale = (int)(e1 >> 49) - (int)(e2 >> 49)
                    if scale >= 20 {       // 10^(e1-e2) does not fit in 64 bits, but C1 does
                        C1 = __mul_128x64_to_128(C1_lo, &bid_ten2k128[(scale - 20) as usize]);
                    } else if scale >= 1 {
                        // if 1 <= scale <= 19 then 10^(e1-e2) fits in 64 bits
                        C1 = if q1 <= 19 { // C1 fits in 64 bits
                            __mul_64x64_to_128MACH(C1_lo, bid_ten2k64[scale as usize])
                        } else { // q1 >= 20
                            C1.w[1] = C1_hi;
                            C1.w[0] = C1_lo;
                            __mul_128x64_to_128(bid_ten2k64[scale as usize], &C1)
                        };
                    } else { // if (scale == 0) C1 is unchanged
                        C1.w[1] = C1_hi;
                        C1.w[0] = C1_lo; // only the low part is necessary
                    }
                    C1_hi = C1.w[1];
                    C1_lo = C1.w[0];
                    // now add C2
                    if x_sign == y_sign {
                        // the result can overflow!
                        C1_lo = C1_lo.wrapping_add(C2_lo);
                        C1_hi = C1_hi.wrapping_add(C2_hi);
                        if C1_lo < C1.w[0] {
                            C1_hi += 1;
                        }
                        // test for overflow, possible only when C1 >= 10^34
                        if C1_hi  > 0x0001ed09bead87c0u64
                        || (C1_hi == 0x0001ed09bead87c0u64
                         && C1_lo >= 0x378d8e6400000000u64) { // C1 >= 10^34
                            // in this case q = P34 + 1 and x = q - P34 = 1, so multiply
                            // C'' = C'+ 5 = C1 + 5 by k1 ~ 10^(-1) calculated for P34 + 1
                            // decimal digits
                            // Calculate C'' = C' + 1/2 * 10^x
                            if C1_lo >= 0xfffffffffffffffbu64 { // low half add has carry
                                C1_lo = C1_lo + 5;
                                C1_hi = C1_hi + 1;
                            } else {
                                C1_lo = C1_lo + 5;
                            }
                            // the approximation of 10^(-1) was rounded up to 118 bits
                            // 10^(-1) =~ 33333333333333333333333333333400 * 2^-129
                            // 10^(-1) =~ 19999999999999999999999999999a00 * 2^-128
                            C1.w[1]     = C1_hi;
                            C1.w[0]     = C1_lo; // C''
                            ten2m1.w[1] = 0x1999999999999999u64;
                            ten2m1.w[0] = 0x9999999999999a00u64;
                            P256        = __mul_128x128_to_256(&C1, &ten2m1); // P256 = C*, f*
                            // C* is actually floor(C*) in this case
                            // the top Ex = 128 bits of 10^(-1) are
                            // T* = 0x00199999999999999999999999999999
                            // if (0 < f* < 10^(-x)) then
                            //   if floor(C*) is even then C = floor(C*) - logical right
                            //       shift; C has p decimal digits, correct by Prop. 1)
                            //   else if floor(C*) is odd C = floor(C*) - 1 (logical right
                            //       shift; C has p decimal digits, correct by Pr. 1)
                            // else
                            //   C = floor(C*) (logical right shift; C has p decimal digits,
                            //       correct by Property 1)
                            // n = C * 10^(e2+x)
                            if (P256.w[1] != 0 || P256.w[0] != 0)
                            && (P256.w[1]  < 0x1999999999999999u64
                            || (P256.w[1] == 0x1999999999999999u64
                             && P256.w[0] <= 0x9999999999999999u64)) {
                                // the result is a midpoint
                                if (P256.w[2] & 0x01) == 0x01 {
                                    is_midpoint_gt_even = true;
                                    // if floor(C*) is odd C = floor(C*) - 1; the result is not 0
                                    P256.w[2] -= 1;
                                    if P256.w[2] == 0xffffffffffffffffu64 {
                                        P256.w[3] -= 1;
                                    }
                                } else {
                                    is_midpoint_lt_even = true;
                                }
                            }
                            // n = Cstar * 10^(e2+1)
                            y_exp = y_exp + EXP_P1;
                            // C* != 10^P because C* has P34 digits
                            // check for overflow
                            if y_exp == EXP_MAX_P1 && (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST
                                                     || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY) {
                                // overflow for RN
                                res.w[1] = x_sign | 0x7800000000000000u64; // +/-inf
                                res.w[0] = 0x0u64;
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                // set the overflow flag
                                *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;

                                #[cfg(target_endian = "big")]
                                BID_SWAP128(&mut res);

                                return res;
                            }
                            // if (0 < f* - 1/2 < 10^(-x)) then
                            //   the result of the addition is exact
                            // else
                            //   the result of the addition is inexact
                            if  P256.w[1] > 0x8000000000000000u64
                            || (P256.w[1] == 0x8000000000000000u64 && P256.w[0] > 0x0u64) { // the result may be exact
                                tmp64 = P256.w[1] - 0x8000000000000000u64;                   // f* - 1/2
                                if  tmp64 > 0x1999999999999999u64
                                || (tmp64 == 0x1999999999999999u64 && P256.w[0] >= 0x9999999999999999u64) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                    is_inexact = true;
                                }    // else the result is exact
                            } else { // the result is inexact
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                is_inexact = true;
                            }
                            C1_hi = P256.w[3];
                            C1_lo = P256.w[2];
                            if !is_midpoint_gt_even && !is_midpoint_lt_even {
                                is_inexact_lt_midpoint = is_inexact && (P256.w[1] & 0x8000000000000000u64) == 0x8000000000000000u64;
                                is_inexact_gt_midpoint = is_inexact && (P256.w[1] & 0x8000000000000000u64) != 0x8000000000000000u64;
                            }
                            // general correction from RN to RA, RM, RP, RZ;
                            // result uses y_exp
                            if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                                if (x_sign == 0 && ((rnd_mode == RoundingMode::BID_ROUNDING_UP && is_inexact_lt_midpoint)
                                                || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
                                                  || rnd_mode == RoundingMode::BID_ROUNDING_UP) && is_midpoint_gt_even)))
                                || (x_sign != 0 && ((rnd_mode == RoundingMode::BID_ROUNDING_DOWN && is_inexact_lt_midpoint)
                                                || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
                                                  || rnd_mode == RoundingMode::BID_ROUNDING_DOWN) && is_midpoint_gt_even))) {
                                    // C1 = C1 + 1
                                    C1_lo = C1_lo + 1;
                                    if C1_lo == 0 { // rounding overflow in the low 64 bits
                                        C1_hi = C1_hi + 1;
                                    }
                                    if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                        // C1 = 10^34 => rounding overflow
                                        C1_hi = 0x0000314dc6448d93u64;
                                        C1_lo = 0x38c15b0a00000000u64; // 10^33
                                        y_exp = y_exp + EXP_P1;
                                    }
                                } else if (is_midpoint_lt_even || is_inexact_gt_midpoint)
                                       && ((x_sign != 0 && (rnd_mode == RoundingMode::BID_ROUNDING_UP
                                                         || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))
                                        || (x_sign == 0 && (rnd_mode == RoundingMode::BID_ROUNDING_DOWN
                                                         || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))) {
                                    // C1 = C1 - 1
                                    C1_lo = C1_lo - 1;
                                    if C1_lo == 0xffffffffffffffffu64 {
                                        C1_hi -= 1;
                                    }
                                    // check if we crossed into the lower decade
                                    if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 { // 10^33 - 1
                                        C1_hi = 0x0001ed09bead87c0u64;                                      // 10^34 - 1
                                        C1_lo = 0x378d8e63ffffffffu64;
                                        y_exp = y_exp - EXP_P1;
                                        // no underflow, because delta + q2 >= P34 + 1
                                    }
                                } else {
                                    // exact, the result is already correct
                                }
                                // in all cases check for overflow (RN and RA solved already)
                                if y_exp == EXP_MAX_P1 {                           // overflow
                                    if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN && x_sign != 0)   // RM and res < 0
                                    || (rnd_mode == RoundingMode::BID_ROUNDING_UP   && x_sign == 0) { // RP and res > 0
                                        C1_hi = 0x7800000000000000u64;               // +inf
                                        C1_lo = 0x0u64;
                                    } else { // RM and res > 0, RP and res < 0, or RZ
                                        C1_hi = 0x5fffed09bead87c0u64;
                                        C1_lo = 0x378d8e63ffffffffu64;
                                    }
                                    y_exp = 0; // x_sign is preserved
                                    // set the inexact flag (in case the exact addition was exact)
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                    // set the overflow flag
                                    *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                                }
                            }
                        }    // else if (C1 < 10^34) then C1 is the coeff.; the result is exact
                    } else { // if x_sign != y_sign the result is exact
                        C1_lo = C1_lo.wrapping_sub(C2_lo);
                        C1_hi = C1_hi.wrapping_sub(C2_hi);
                        if C1_lo > C1.w[0] {
                            C1_hi -= 1;
                        }
                        // the result can be zero, but it cannot overflow
                        if C1_lo == 0 && C1_hi == 0 {
                            // assemble the result
                            res.w[1] = if x_exp < y_exp { x_exp } else { y_exp };
                            res.w[0] = 0;
                            if rnd_mode == RoundingMode::BID_ROUNDING_DOWN {
                                res.w[1] |= 0x8000000000000000u64;
                            }

                            #[cfg(target_endian = "big")]
                            BID_SWAP128(&mut res);

                            return res;
                        }
                        if C1_hi >= 0x8000000000000000u64 { // negative coefficient!
                            C1_lo  = !C1_lo;
                            C1_lo += 1;
                            C1_hi  = !C1_hi;
                            if C1_lo == 0x0 {
                                C1_hi += 1;
                            }
                            x_sign = y_sign; // the result will have the sign of y
                        }
                    }
                    // assemble the result
                    res.w[1] = x_sign | y_exp | C1_hi;
                    res.w[0] = C1_lo;
                } else { // if (delta >= P34 + 1 - q2)
                    // instead of C' = (C1 * 10^(e1-e2) + C2)rnd,P34
                    // calculate C' = C1 * 10^(e1-e2-x1) + (C2 * 10^(-x1))rnd,P34
                    // where x1 = q1 + e1 - e2 - P34, 1 <= x1 <= P34 - 1
                    // In most cases C' will have P34 digits, and n = C' * 10^(e2+x1)
                    // If the result has P34+1 digits, redo the steps above with x1+1
                    // If the result has P34-1 digits or less, redo the steps above with
                    // x1-1 but only if initially x1 >= 1
                    // NOTE: these two steps can be improved, e.g we could guess if
                    // P34+1 or P34-1 digits will be obtained by adding/subtracting just
                    // the top 64 bits of the two operands
                    // The result cannot be zero, but it can overflow
                    x1 = delta + q2 - P34; // 1 <= x1 <= P34-1
                // TODO: goto roundC2
                // roundC2:
                    'roundC2: loop {
                        // Calculate C1 * 10^(e1-e2-x1) where 0 <= e1-e2-x1 <= P34 - 1
                        // scale = (int)(e1 >> 49) - (int)(e2 >> 49) - x1; 0 <= scale <= P34-1
                        scale = delta - q1 + q2 - x1; // scale = e1 - e2 - x1 = P34 - q1
                        // either C1 or 10^(e1-e2-x1) may not fit is 64 bits,
                        // but their product fits with certainty in 128 bits (actually in 113)
                        if scale >= 20 { // 10^(e1-e2-x1) doesn't fit in 64 bits, but C1 does
                            C1 = __mul_128x64_to_128(C1_lo, &bid_ten2k128[(scale - 20) as usize]);
                        } else if scale >= 1 {
                            // if 1 <= scale <= 19 then 10^(e1-e2-x1) fits in 64 bits
                            C1 = if q1 <= 19 { // C1 fits in 64 bits
                                __mul_64x64_to_128MACH(C1_lo, bid_ten2k64[scale as usize])
                            } else { // q1 >= 20
                                C1.w[1] = C1_hi;
                                C1.w[0] = C1_lo;
                                __mul_128x64_to_128(bid_ten2k64[scale as usize], &C1)
                            };
                        } else { // if (scale == 0) C1 is unchanged
                            C1.w[1] = C1_hi;
                            C1.w[0] = C1_lo;
                        }
                        tmp64 = C1.w[0]; // C1.w[1], C1.w[0] contains C1 * 10^(e1-e2-x1)

                        // now round C2 to q2-x1 decimal digits, where 1<=x1<=q2-1<=P34-1
                        // (but if we got here a second time after x1 = x1 - 1, then
                        // x1 >= 0; note that for x1 = 0 C2 is unchanged)
                        // C2' = C2 + 1/2 * 10^x1 = C2 + 5 * 10^(x1-1)
                        ind = x1 - 1; // 0 <= ind <= q2-2<=P34-2=32; but note that if x1 = 0
                        // during a second pass, then ind = -1
                        if ind >= 0 { // if (x1 >= 1)
                            C2.w[0] = C2_lo;
                            C2.w[1] = C2_hi;
                            if ind <= 18 {
                                C2.w[0] = C2.w[0].wrapping_add(bid_midpoint64[ind as usize]);
                                if C2.w[0] < C2_lo {
                                    C2.w[1] += 1;
                                }
                            } else { // 19 <= ind <= 32
                                C2.w[0] = C2.w[0].wrapping_add(bid_midpoint128[(ind - 19) as usize].w[0]);
                                C2.w[1] = C2.w[1].wrapping_add(bid_midpoint128[(ind - 19) as usize].w[1]);
                                if C2.w[0] < C2_lo {
                                    C2.w[1] += 1;
                                }
                            }
                            // the approximation of 10^(-x1) was rounded up to 118 bits
                            R256 = __mul_128x128_to_256(&C2, &bid_ten2mk128[ind as usize]); // R256 = C2*, f2*
                            // calculate C2* and f2*
                            // C2* is actually floor(C2*) in this case
                            // C2* and f2* need shifting and masking, as shown by
                            // bid_shiftright128[] and bid_maskhigh128[]
                            // the top Ex bits of 10^(-x1) are T* = bid_ten2mk128trunc[(ind) as usize], e.g.
                            // if x1=1, T*=bid_ten2mk128trunc[0]=0x19999999999999999999999999999999
                            // if (0 < f2* < 10^(-x1)) then
                            //   if floor(C1+C2*) is even then C2* = floor(C2*) - logical right
                            //       shift; C2* has p decimal digits, correct by Prop. 1)
                            //   else if floor(C1+C2*) is odd C2* = floor(C2*)-1 (logical right
                            //       shift; C2* has p decimal digits, correct by Pr. 1)
                            // else
                            //   C2* = floor(C2*) (logical right shift; C has p decimal digits,
                            //       correct by Property 1)
                            // n = C2* * 10^(e2+x1)

                            if ind <= 2 {
                                highf2star.w[1] = 0x0;
                                highf2star.w[0] = 0x0; // low f2* ok
                            } else if ind <= 21 {
                                highf2star.w[1] = 0x0;
                                highf2star.w[0] = R256.w[2] & bid_maskhigh128[ind as usize]; // low f2* ok
                            } else {
                                highf2star.w[1] = R256.w[3] & bid_maskhigh128[ind as usize];
                                highf2star.w[0] = R256.w[2]; // low f2* is ok
                            }
                            // shift right C2* by Ex-128 = bid_shiftright128[(ind) as usize]
                            if ind >= 3 {
                                shift = bid_shiftright128[ind as usize];
                                if shift < 64 { // 3 <= shift <= 63
                                    R256.w[2] = (R256.w[2] >> shift) | (R256.w[3] << (64 - shift));
                                    R256.w[3] = R256.w[3] >> shift;
                                } else { // 66 <= shift <= 102
                                    R256.w[2] = R256.w[3] >> (shift - 64);
                                    R256.w[3] = 0x0u64;
                                }
                            }
                            if second_pass {
                                is_inexact_lt_midpoint = false;
                                is_inexact_gt_midpoint = false;
                                is_midpoint_lt_even    = false;
                                is_midpoint_gt_even    = false;
                            }
                            // determine inexactness of the rounding of C2* (this may be
                            // followed by a second rounding only if we get P34+1
                            // decimal digits)
                            // if (0 < f2* - 1/2 < 10^(-x1)) then
                            //   the result is exact
                            // else (if f2* - 1/2 > T* then)
                            //   the result of is inexact
                            if ind <= 2 {
                                if  R256.w[1]  > 0x8000000000000000u64
                                || (R256.w[1] == 0x8000000000000000u64
                                 && R256.w[0]  > 0x0u64) {
                                    // f2* > 1/2 and the result may be exact
                                    tmp64A = R256.w[1] - 0x8000000000000000u64; // f* - 1/2
                                    if  tmp64A      > bid_ten2mk128trunc[ind as usize].w[1]
                                     || (tmp64A    == bid_ten2mk128trunc[ind as usize].w[1]
                                      && R256.w[0] >= bid_ten2mk128trunc[ind as usize].w[0]) {
                                        // set the inexact flag
                                        // *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                        tmp_inexact = true; // may be set again during a second pass
                                        // this rounding is applied to C2 only!
                                        if x_sign == y_sign {
                                            is_inexact_lt_midpoint = true;
                                        } else { // if (x_sign != y_sign)
                                            is_inexact_gt_midpoint = true;
                                        }
                                    }    // else the result is exact
                                         // rounding down, unless a midpoint in [ODD, EVEN]
                                } else { // the result is inexact; f2* <= 1/2
                                    // set the inexact flag
                                    // *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                    tmp_inexact = true; // just in case we will round a second time
                                    // rounding up, unless a midpoint in [EVEN, ODD]
                                    // this rounding is applied to C2 only!
                                    if x_sign == y_sign {
                                        is_inexact_gt_midpoint = true;
                                    } else { // if (x_sign != y_sign)
                                        is_inexact_lt_midpoint = true;
                                    }
                                }
                            } else if ind <= 21 { // if 3 <= ind <= 21
                                if  highf2star.w[1]  > 0x0 || (highf2star.w[1] == 0x0 && highf2star.w[0] > bid_onehalf128[ind as usize])
                                || (highf2star.w[1] == 0x0 &&  highf2star.w[0] == bid_onehalf128[ind as usize] && (R256.w[1] != 0 || R256.w[0] != 0)) {
                                    // f2* > 1/2 and the result may be exact
                                    // Calculate f2* - 1/2
                                    tmp64A = highf2star.w[0] - bid_onehalf128[ind as usize];
                                    tmp64B = highf2star.w[1];
                                    if tmp64A > highf2star.w[0] {
                                        tmp64B -= 1;
                                    }
                                    if  tmp64B != 0 || tmp64A!= 0 || R256.w[1] > bid_ten2mk128trunc[ind as usize].w[1]
                                    || (R256.w[1] == bid_ten2mk128trunc[ind as usize].w[1]
                                     && R256.w[0]  > bid_ten2mk128trunc[ind as usize].w[0]) {
                                        // set the inexact flag
                                        // *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                        tmp_inexact = true; // may be set again during a second pass
                                        // this rounding is applied to C2 only!
                                        if x_sign == y_sign {
                                            is_inexact_lt_midpoint = true;
                                        } else { // if (x_sign != y_sign)
                                            is_inexact_gt_midpoint = true;
                                        }
                                    }    // else the result is exact
                                } else { // the result is inexact; f2* <= 1/2
                                    // set the inexact flag
                                    // *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                    tmp_inexact = true; // may be set again during a second pass
                                    // rounding up, unless a midpoint in [EVEN, ODD]
                                    // this rounding is applied to C2 only!
                                    if x_sign == y_sign {
                                        is_inexact_gt_midpoint = true;
                                    } else { // if (x_sign != y_sign)
                                        is_inexact_lt_midpoint = true;
                                    }
                                }
                            } else { // if 22 <= ind <= 33
                                if  highf2star.w[1]  > bid_onehalf128[ind as usize]
                                || (highf2star.w[1] == bid_onehalf128[ind as usize]
                                && (highf2star.w[0] != 0 || R256.w[1] != 0 || R256.w[0] != 0)) {
                                    // f2* > 1/2 and the result may be exact
                                    // Calculate f2* - 1/2
                                    // tmp64A = highf2star.w[0];
                                    tmp64B = highf2star.w[1] - bid_onehalf128[ind as usize];
                                    if tmp64B != 0 || highf2star.w[0] != 0 || R256.w[1] > bid_ten2mk128trunc[ind as usize].w[1]
                                    || (R256.w[1] == bid_ten2mk128trunc[ind as usize].w[1]
                                     && R256.w[0]  > bid_ten2mk128trunc[ind as usize].w[0]) {
                                        // set the inexact flag
                                        // *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                        tmp_inexact = true; // may be set again during a second pass
                                        // this rounding is applied to C2 only!
                                        if x_sign == y_sign {
                                            is_inexact_lt_midpoint = true;
                                        } else { // if (x_sign != y_sign)
                                            is_inexact_gt_midpoint = true;
                                        }
                                    }    // else the result is exact
                                } else { // the result is inexact; f2* <= 1/2
                                    // set the inexact flag
                                    // *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                    tmp_inexact = true; // may be set again during a second pass
                                    // rounding up, unless a midpoint in [EVEN, ODD]
                                    // this rounding is applied to C2 only!
                                    if x_sign == y_sign {
                                        is_inexact_gt_midpoint = true;
                                    } else { // if (x_sign != y_sign)
                                        is_inexact_lt_midpoint = true;
                                    }
                                }
                            }
                            // check for midpoints
                            if (R256.w[1] != 0 || R256.w[0] != 0)
                            && (highf2star.w[1] == 0)
                            && (highf2star.w[0] == 0)
                            && (R256.w[1]  < bid_ten2mk128trunc[ind as usize].w[1]
                            || (R256.w[1] == bid_ten2mk128trunc[ind as usize].w[1]
                             && R256.w[0] <= bid_ten2mk128trunc[ind as usize].w[0])) {
                                // the result is a midpoint
                                if ((tmp64.wrapping_add(R256.w[2])) & 0x01) == 0x01 { // MP in [EVEN, ODD]
                                    // if floor(C2*) is odd C = floor(C2*) - 1; the result may be 0
                                    R256.w[2] -= 1;
                                    if R256.w[2] == 0xffffffffffffffffu64 {
                                        R256.w[3] -= 1;
                                    }
                                    // this rounding is applied to C2 only!
                                    if x_sign == y_sign {
                                        is_midpoint_gt_even = true;
                                    } else { // if (x_sign != y_sign)
                                        is_midpoint_lt_even = true;
                                    }
                                    is_inexact_lt_midpoint = false;
                                    is_inexact_gt_midpoint = false;
                                } else {
                                    // else MP in [ODD, EVEN]
                                    // this rounding is applied to C2 only!
                                    if x_sign == y_sign {
                                        is_midpoint_lt_even = true;
                                    } else { // if (x_sign != y_sign)
                                        is_midpoint_gt_even = true;
                                    }
                                    is_inexact_lt_midpoint = false;
                                    is_inexact_gt_midpoint = false;
                                }
                            }
                            // end if (ind >= 0)
                        } else { // if (ind == -1); only during a 2nd pass, and when x1 = 0
                            R256.w[2] = C2_lo;
                            R256.w[3] = C2_hi;
                            tmp_inexact = false;
                            // to correct a possible setting to 1 from 1st pass
                            if second_pass {
                                is_midpoint_lt_even    = false;
                                is_midpoint_gt_even    = false;
                                is_inexact_lt_midpoint = false;
                                is_inexact_gt_midpoint = false;
                            }
                        }
                        // and now add/subtract C1 * 10^(e1-e2-x1) +/- (C2 * 10^(-x1))rnd,P34
                        if x_sign == y_sign { // addition; could overflow
                            // no second pass is possible this way (only for x_sign != y_sign)
                            C1.w[0] = C1.w[0].wrapping_add(R256.w[2]);
                            C1.w[1] = C1.w[1].wrapping_add(R256.w[3]);
                            if C1.w[0] < tmp64 {
                                C1.w[1] += 1; // carry
                            }
                            // if the sum has P34+1 digits, i.e. C1>=10^34 redo the calculation
                            // with x1=x1+1
                            if C1.w[1]   > 0x0001ed09bead87c0u64
                            || (C1.w[1] == 0x0001ed09bead87c0u64
                             && C1.w[0] >= 0x378d8e6400000000u64) { // C1 >= 10^34
                                // chop off one more digit from the sum, but make sure there is
                                // no double-rounding error (see table - double rounding logic)
                                // now round C1 from P34+1 to P34 decimal digits
                                // C1' = C1 + 1/2 * 10 = C1 + 5
                                if C1.w[0] >= 0xfffffffffffffffbu64 { // low half add has carry
                                    C1.w[0] = C1.w[0] + 5;
                                    C1.w[1] = C1.w[1] + 1;
                                } else {
                                    C1.w[0] = C1.w[0] + 5;
                                }
                                // the approximation of 10^(-1) was rounded up to 118 bits
                                Q256 = __mul_128x128_to_256(&C1, &bid_ten2mk128[0]); // Q256 = C1*, f1*
                                // C1* is actually floor(C1*) in this case
                                // the top 128 bits of 10^(-1) are
                                // T* = bid_ten2mk128trunc[0]=0x19999999999999999999999999999999
                                // if (0 < f1* < 10^(-1)) then
                                //   if floor(C1*) is even then C1* = floor(C1*) - logical right
                                //       shift; C1* has p decimal digits, correct by Prop. 1)
                                //   else if floor(C1*) is odd C1* = floor(C1*) - 1 (logical right
                                //       shift; C1* has p decimal digits, correct by Pr. 1)
                                // else
                                //   C1* = floor(C1*) (logical right shift; C has p decimal digits
                                //       correct by Property 1)
                                // n = C1* * 10^(e2+x1+1)
                                if (Q256.w[1] != 0 || Q256.w[0] != 0)
                                && (Q256.w[1]  < bid_ten2mk128trunc[0].w[1]
                                || (Q256.w[1] == bid_ten2mk128trunc[0].w[1]
                                 && Q256.w[0] <= bid_ten2mk128trunc[0].w[0])) {
                                    // the result is a midpoint
                                    if is_inexact_lt_midpoint { // for the 1st rounding
                                        is_inexact_gt_midpoint = true;
                                        is_inexact_lt_midpoint = false;
                                        is_midpoint_gt_even    = false;
                                        is_midpoint_lt_even    = false;
                                    } else if is_inexact_gt_midpoint { // for the 1st rounding
                                        Q256.w[2] -= 1;
                                        if Q256.w[2] == 0xffffffffffffffffu64 {
                                            Q256.w[3] -= 1;
                                        }
                                        is_inexact_gt_midpoint = false;
                                        is_inexact_lt_midpoint = true;
                                        is_midpoint_gt_even    = false;
                                        is_midpoint_lt_even    = false;
                                    } else if is_midpoint_gt_even { // for the 1st rounding
                                        // Note: cannot have is_midpoint_lt_even
                                        is_inexact_gt_midpoint = false;
                                        is_inexact_lt_midpoint = true;
                                        is_midpoint_gt_even    = false;
                                        is_midpoint_lt_even    = false;
                                    } else {                    // the first rounding must have been exact
                                        if (Q256.w[2] & 0x01) == 0x01 { // MP in [EVEN, ODD]
                                            // the truncated result is correct
                                            Q256.w[2] -= 1;
                                            if Q256.w[2] == 0xffffffffffffffffu64 {
                                                Q256.w[3] -= 1;
                                            }
                                            is_inexact_gt_midpoint = false;
                                            is_inexact_lt_midpoint = false;
                                            is_midpoint_gt_even    = true;
                                            is_midpoint_lt_even    = false;
                                        } else { // MP in [ODD, EVEN]
                                            is_inexact_gt_midpoint = false;
                                            is_inexact_lt_midpoint = false;
                                            is_midpoint_gt_even    = false;
                                            is_midpoint_lt_even    = true;
                                        }
                                    }
                                    tmp_inexact = true; // in all cases
                                } else {             // the result is not a midpoint
                                    // determine inexactness of the rounding of C1 (the sum C1+C2*)
                                    // if (0 < f1* - 1/2 < 10^(-1)) then
                                    //   the result is exact
                                    // else (if f1* - 1/2 > T* then)
                                    //   the result of is inexact
                                    // ind = 0
                                    if Q256.w[1] > 0x8000000000000000u64 || (Q256.w[1] == 0x8000000000000000u64 && Q256.w[0] > 0x0u64) {
                                        // f1* > 1/2 and the result may be exact
                                        Q256.w[1] = Q256.w[1] - 0x8000000000000000u64; // f1* - 1/2
                                        if  Q256.w[1]  > bid_ten2mk128trunc[0].w[1]
                                        || (Q256.w[1] == bid_ten2mk128trunc[0].w[1]
                                         && Q256.w[0]  > bid_ten2mk128trunc[0].w[0]) {
                                            is_inexact_gt_midpoint = false;
                                            is_inexact_lt_midpoint = true;
                                            is_midpoint_gt_even    = false;
                                            is_midpoint_lt_even    = false;
                                            // set the inexact flag
                                            tmp_inexact = true;
                                            // *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                        } else {               // else the result is exact for the 2nd rounding
                                            if tmp_inexact { // if the previous rounding was inexact
                                                if is_midpoint_lt_even {
                                                    is_inexact_gt_midpoint = true;
                                                    is_midpoint_lt_even    = false;
                                                } else if is_midpoint_gt_even {
                                                    is_inexact_lt_midpoint = true;
                                                    is_midpoint_gt_even    = false;
                                                } else {
                                                    // no change
                                                }
                                            }
                                        }
                                        // rounding down, unless a midpoint in [ODD, EVEN]
                                    } else { // the result is inexact; f1* <= 1/2
                                        is_inexact_gt_midpoint = true;
                                        is_inexact_lt_midpoint = false;
                                        is_midpoint_gt_even    = false;
                                        is_midpoint_lt_even    = false;
                                        // set the inexact flag
                                        tmp_inexact            = true;
                                        // *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                    }
                                } // end 'the result is not a midpoint'
                                // n = C1 * 10^(e2+x1)
                                C1.w[1] = Q256.w[3];
                                C1.w[0] = Q256.w[2];
                                y_exp   = y_exp + (((x1 + 1) as BID_UINT64) << 49);
                            } else { // C1 < 10^34
                                // C1.w[1] and C1.w[0] already set
                                // n = C1 * 10^(e2+x1)
                                y_exp = y_exp + ((x1 as BID_UINT64) << 49);
                            }
                            // check for overflow
                            if y_exp == EXP_MAX_P1 && (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST
                                                     || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY) {
                                res.w[1] = 0x7800000000000000u64 | x_sign; // +/-inf
                                res.w[0] = 0x0u64;
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                // set the overflow flag
                                *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;

                                #[cfg(target_endian = "big")]
                                BID_SWAP128(&mut res);

                                return res;
                            }    // else no overflow
                        } else { // if x_sign != y_sign the result of this subtract. is exact
                            C1.w[0] = C1.w[0].wrapping_sub(R256.w[2]);
                            C1.w[1] = C1.w[1].wrapping_sub(R256.w[3]);
                            if C1.w[0] > tmp64 {
                                C1.w[1] -= 1;                       // borrow
                            }
                            if C1.w[1] >= 0x8000000000000000u64 { // negative coefficient!
                                C1.w[0]  = !C1.w[0];
                                C1.w[0] += 1;
                                C1.w[1]  = !C1.w[1];
                                if C1.w[0] == 0x0 {
                                    C1.w[1] += 1;
                                }
                                tmp_sign = y_sign;
                                // the result will have the sign of y if last rnd
                            } else {
                                tmp_sign = x_sign;
                            }
                            // if the difference has P34-1 digits or less, i.e. C1 < 10^33 then
                            //   redo the calculation with x1=x1-1;
                            // redo the calculation also if C1 = 10^33 and
                            //   (is_inexact_gt_midpoint or is_midpoint_lt_even);
                            //   (the last part should have really been
                            //   (is_inexact_lt_midpoint or is_midpoint_gt_even) from
                            //    the rounding of C2, but the position flags have been reversed)
                            // 10^33 = 0x0000314dc6448d93 0x38c15b0a00000000
                            if (C1.w[1]  < 0x0000314dc6448d93u64
                            || (C1.w[1] == 0x0000314dc6448d93u64 && C1.w[0]  < 0x38c15b0a00000000u64))
                            || (C1.w[1] == 0x0000314dc6448d93u64 && C1.w[0] == 0x38c15b0a00000000u64
                            && (is_inexact_gt_midpoint || is_midpoint_lt_even)) { // C1=10^33
                                x1 = x1 - 1;                                        // x1 >= 0
                                if x1 >= 0 {
                                    // clear position flags and tmp_inexact
                                    is_midpoint_lt_even    = false;
                                    is_midpoint_gt_even    = false;
                                    is_inexact_lt_midpoint = false;
                                    is_inexact_gt_midpoint = false;
                                    tmp_inexact            = false;
                                    second_pass            = true;
                                    // TODO: goto roundC2
                                    // goto roundC2; // else result has less than P34 digits
                                    continue 'roundC2;
                                }
                            }
                            // if the coefficient of the result is 10^34 it means that this
                            // must be the second pass, and we are done
                            if C1.w[1] == 0x0001ed09bead87c0u64 && C1.w[0] == 0x378d8e6400000000u64 { // if  C1 = 10^34
                                C1.w[1] = 0x0000314dc6448d93u64;                                      // C1 = 10^33
                                C1.w[0] = 0x38c15b0a00000000u64;
                                y_exp = y_exp + ((1 as BID_UINT64) << 49);
                            }
                            x_sign = tmp_sign;
                            if x1 >= 1 {
                                y_exp = y_exp + ((x1 as BID_UINT64) << 49);
                            }
                            // x1 = -1 is possible at the end of a second pass when the
                            // first pass started with x1 = 1
                        }
                        break 'roundC2;
                    } // loop
                    C1_hi = C1.w[1];
                    C1_lo = C1.w[0];
                    // general correction from RN to RA, RM, RP, RZ; result uses y_exp
                    if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                        if (x_sign == 0 && ((rnd_mode == RoundingMode::BID_ROUNDING_UP && is_inexact_lt_midpoint)
                                        || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
                                          || rnd_mode == RoundingMode::BID_ROUNDING_UP) && is_midpoint_gt_even)))
                        || (x_sign != 0 && ((rnd_mode == RoundingMode::BID_ROUNDING_DOWN && is_inexact_lt_midpoint)
                                        || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
                                          || rnd_mode == RoundingMode::BID_ROUNDING_DOWN) && is_midpoint_gt_even))) {
                            // C1 = C1 + 1
                            C1_lo = C1_lo + 1;
                            if C1_lo == 0 { // rounding overflow in the low 64 bits
                                C1_hi = C1_hi + 1;
                            }
                            if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                // C1 = 10^34 => rounding overflow
                                C1_hi = 0x0000314dc6448d93u64;
                                C1_lo = 0x38c15b0a00000000u64; // 10^33
                                y_exp = y_exp + EXP_P1;
                            }
                        } else if (is_midpoint_lt_even || is_inexact_gt_midpoint)
                               && ((x_sign != 0 && (rnd_mode == RoundingMode::BID_ROUNDING_UP
                                                 || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))
                                || (x_sign == 0 && (rnd_mode == RoundingMode::BID_ROUNDING_DOWN
                                                 || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))) {
                            // C1 = C1 - 1
                            C1_lo = C1_lo - 1;
                            if C1_lo == 0xffffffffffffffffu64 {
                                C1_hi -= 1;
                            }
                            // check if we crossed into the lower decade
                            if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 { // 10^33 - 1
                                C1_hi = 0x0001ed09bead87c0u64;                                      // 10^34 - 1
                                C1_lo = 0x378d8e63ffffffffu64;
                                y_exp = y_exp - EXP_P1;
                                // no underflow, because delta + q2 >= P34 + 1
                            }
                        } else {
                            // exact, the result is already correct
                        }
                        // in all cases check for overflow (RN and RA solved already)
                        if y_exp == EXP_MAX_P1 {                            // overflow
                            if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN && x_sign != 0)     // RM and res < 0
                            || (rnd_mode == RoundingMode::BID_ROUNDING_UP   && x_sign == 0) {   // RP and res > 0
                                C1_hi = 0x7800000000000000u64;              // +inf
                                C1_lo = 0x0u64;
                            } else { // RM and res > 0, RP and res < 0, or RZ
                                C1_hi = 0x5fffed09bead87c0u64;
                                C1_lo = 0x378d8e63ffffffffu64;
                            }
                            y_exp = 0; // x_sign is preserved
                            // set the inexact flag (in case the exact addition was exact)
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            // set the overflow flag
                            *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                        }
                    }
                    // assemble the result
                    res.w[1] = x_sign | y_exp | C1_hi;
                    res.w[0] = C1_lo;
                    if tmp_inexact {
                        *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                    }
                }
            } else { // if (-P34 + 1 <= delta <= -1) <=> 1 <= -delta <= P34 - 1
                // NOTE: the following, up to "} else { // if x_sign != y_sign
                // the result is exact" is identical to "else if (delta == P34 - q2) {"
                // from above; also, the code is not symmetric: a+b and b+a may take
                // different paths (need to unify eventually!)
                // calculate C' = C2 + C1 * 10^(e1-e2) directly; the result may be
                // inexact if it requires P34 + 1 decimal digits; in either case the
                // 'cutoff' point for addition is at the position of the lsb of C2
                // The coefficient of the result is C1 * 10^(e1-e2) + C2 and the
                // exponent is e2; either C1 or 10^(e1-e2) may not fit is 64 bits,
                // but their product fits with certainty in 128 bits (actually in 113)
                // Note that 0 <= e1 - e2 <= P34 - 2
                //   -P34 + 1 <= delta <= -1 <=> -P34 + 1 <= delta <= -1 <=>
                //   -P34 + 1 <= q1 + e1 - q2 - e2 <= -1 <=>
                //   q2 - q1 - P34 + 1 <= e1 - e2 <= q2 - q1 - 1 <=>
                //   1 - P34 - P34 + 1 <= e1-e2 <= P34 - 1 - 1 => 0 <= e1-e2 <= P34 - 2
                scale = delta - q1 + q2; // scale = (int)(e1 >> 49) - (int)(e2 >> 49)
                if scale >= 20 {         // 10^(e1-e2) does not fit in 64 bits, but C1 does
                    C1 = __mul_128x64_to_128(C1_lo, &bid_ten2k128[(scale - 20) as usize]);
                } else if scale >= 1 {
                    // if 1 <= scale <= 19 then 10^(e1-e2) fits in 64 bits
                    C1 = if q1 <= 19 { // C1 fits in 64 bits
                        __mul_64x64_to_128MACH(C1_lo, bid_ten2k64[scale as usize])
                    } else { // q1 >= 20
                        C1.w[1] = C1_hi;
                        C1.w[0] = C1_lo;
                        __mul_128x64_to_128(bid_ten2k64[scale as usize], &C1)
                    };
                } else { // if (scale == 0) C1 is unchanged
                    C1.w[1] = C1_hi;
                    C1.w[0] = C1_lo; // only the low part is necessary
                }
                C1_hi = C1.w[1];
                C1_lo = C1.w[0];
                // now add C2
                if x_sign == y_sign {
                    // the result can overflow!
                    C1_lo = C1_lo.wrapping_add(C2_lo);
                    C1_hi = C1_hi.wrapping_add(C2_hi);
                    if C1_lo < C1.w[0] {
                        C1_hi += 1;
                    }
                    // test for overflow, possible only when C1 >= 10^34
                    if  C1_hi  > 0x0001ed09bead87c0u64
                    || (C1_hi == 0x0001ed09bead87c0u64
                     && C1_lo >= 0x378d8e6400000000u64) { // C1 >= 10^34
                        // in this case q = P34 + 1 and x = q - P34 = 1, so multiply
                        // C'' = C'+ 5 = C1 + 5 by k1 ~ 10^(-1) calculated for P34 + 1
                        // decimal digits
                        // Calculate C'' = C' + 1/2 * 10^x
                        if C1_lo >= 0xfffffffffffffffbu64 { // low half add has carry
                            C1_lo = C1_lo + 5;
                            C1_hi = C1_hi + 1;
                        } else {
                            C1_lo = C1_lo + 5;
                        }
                        // the approximation of 10^(-1) was rounded up to 118 bits
                        // 10^(-1) =~ 33333333333333333333333333333400 * 2^-129
                        // 10^(-1) =~ 19999999999999999999999999999a00 * 2^-128
                        C1.w[1]     = C1_hi;
                        C1.w[0]     = C1_lo; // C''
                        ten2m1.w[1] = 0x1999999999999999u64;
                        ten2m1.w[0] = 0x9999999999999a00u64;
                        P256        = __mul_128x128_to_256(&C1, &ten2m1); // P256 = C*, f*
                        // C* is actually floor(C*) in this case
                        // the top Ex = 128 bits of 10^(-1) are
                        // T* = 0x00199999999999999999999999999999
                        // if (0 < f* < 10^(-x)) then
                        //   if floor(C*) is even then C = floor(C*) - logical right
                        //       shift; C has p decimal digits, correct by Prop. 1)
                        //   else if floor(C*) is odd C = floor(C*) - 1 (logical right
                        //       shift; C has p decimal digits, correct by Pr. 1)
                        // else
                        //   C = floor(C*) (logical right shift; C has p decimal digits,
                        //       correct by Property 1)
                        // n = C * 10^(e2+x)
                        if (P256.w[1] != 0|| P256.w[0] != 0)
                        && (P256.w[1]  < 0x1999999999999999u64
                        || (P256.w[1] == 0x1999999999999999u64
                         && P256.w[0] <= 0x9999999999999999u64)) {
                            // the result is a midpoint
                            if (P256.w[2] & 0x01) == 0x01 {
                                is_midpoint_gt_even = true;
                                // if floor(C*) is odd C = floor(C*) - 1; the result is not 0
                                P256.w[2] -= 1;
                                if P256.w[2] == 0xffffffffffffffffu64 {
                                    P256.w[3] -= 1;
                                }
                            } else {
                                is_midpoint_lt_even = true;
                            }
                        }
                        // n = Cstar * 10^(e2+1)
                        y_exp = y_exp + EXP_P1;
                        // C* != 10^P34 because C* has P34 digits
                        // check for overflow
                        if y_exp == EXP_MAX_P1 && (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST
                                                || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY) {
                            // overflow for RN
                            res.w[1] = x_sign | 0x7800000000000000u64; // +/-inf
                            res.w[0] = 0x0u64;
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            // set the overflow flag
                            *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;

                            #[cfg(target_endian = "big")]
                            BID_SWAP128(&mut res);

                            return res;
                        }
                        // if (0 < f* - 1/2 < 10^(-x)) then
                        //   the result of the addition is exact
                        // else
                        //   the result of the addition is inexact
                        if  P256.w[1]  > 0x8000000000000000u64
                        || (P256.w[1] == 0x8000000000000000u64
                         && P256.w[0]  > 0x0u64) {                     // the result may be exact
                            tmp64 = P256.w[1] - 0x8000000000000000u64;  // f* - 1/2
                            if  tmp64  > 0x1999999999999999u64
                            || (tmp64 == 0x1999999999999999u64 && P256.w[0] >= 0x9999999999999999u64) {
                                // set the inexact flag
                                *pfpsf    |= StatusFlags::BID_INEXACT_EXCEPTION;
                                is_inexact = true;
                            }    // else the result is exact
                        } else { // the result is inexact
                            // set the inexact flag
                            *pfpsf    |= StatusFlags::BID_INEXACT_EXCEPTION;
                            is_inexact = true;
                        }
                        C1_hi = P256.w[3];
                        C1_lo = P256.w[2];
                        if !is_midpoint_gt_even && !is_midpoint_lt_even {
                            is_inexact_lt_midpoint = is_inexact && (P256.w[1] & 0x8000000000000000u64) == 0x8000000000000000u64;
                            is_inexact_gt_midpoint = is_inexact && (P256.w[1] & 0x8000000000000000u64) != 0x8000000000000000u64;
                        }
                        // general correction from RN to RA, RM, RP, RZ; result uses y_exp
                        if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                            if (x_sign == 0 && ((rnd_mode == RoundingMode::BID_ROUNDING_UP && is_inexact_lt_midpoint)
                                            || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
                                              || rnd_mode == RoundingMode::BID_ROUNDING_UP) && is_midpoint_gt_even)))
                            || (x_sign != 0 && ((rnd_mode == RoundingMode::BID_ROUNDING_DOWN && is_inexact_lt_midpoint)
                                            || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
                                              || rnd_mode == RoundingMode::BID_ROUNDING_DOWN) && is_midpoint_gt_even))) {
                                // C1 = C1 + 1
                                C1_lo = C1_lo + 1;
                                if C1_lo == 0 { // rounding overflow in the low 64 bits
                                    C1_hi = C1_hi + 1;
                                }
                                if C1_hi == 0x0001ed09bead87c0u64 && C1_lo == 0x378d8e6400000000u64 {
                                    // C1 = 10^34 => rounding overflow
                                    C1_hi = 0x0000314dc6448d93u64;
                                    C1_lo = 0x38c15b0a00000000u64; // 10^33
                                    y_exp = y_exp + EXP_P1;
                                }
                            } else if (is_midpoint_lt_even || is_inexact_gt_midpoint)
                                   && ((x_sign != 0 && (rnd_mode == RoundingMode::BID_ROUNDING_UP
                                                     || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))
                                   ||  (x_sign == 0 && (rnd_mode == RoundingMode::BID_ROUNDING_DOWN
                                                     || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))) {
                                // C1 = C1 - 1
                                C1_lo = C1_lo - 1;
                                if C1_lo == 0xffffffffffffffffu64 {
                                    C1_hi -= 1;
                                }
                                // check if we crossed into the lower decade
                                if C1_hi == 0x0000314dc6448d93u64 && C1_lo == 0x38c15b09ffffffffu64 { // 10^33 - 1
                                    C1_hi = 0x0001ed09bead87c0u64;                                      // 10^34 - 1
                                    C1_lo = 0x378d8e63ffffffffu64;
                                    y_exp = y_exp - EXP_P1;
                                    // no underflow, because delta + q2 >= P34 + 1
                                }
                            } else {
                                // exact, the result is already correct
                            }
                            // in all cases check for overflow (RN and RA solved already)
                            if y_exp == EXP_MAX_P1 {                           // overflow
                                if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN && x_sign != 0)    // RM and res < 0
                                || (rnd_mode == RoundingMode::BID_ROUNDING_UP   && x_sign == 0) { // RP and res > 0
                                    C1_hi = 0x7800000000000000u64;               // +inf
                                    C1_lo = 0x0u64;
                                } else { // RM and res > 0, RP and res < 0, or RZ
                                    C1_hi = 0x5fffed09bead87c0u64;
                                    C1_lo = 0x378d8e63ffffffffu64;
                                }
                                y_exp = 0; // x_sign is preserved
                                // set the inexact flag (in case the exact addition was exact)
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                // set the overflow flag
                                *pfpsf |= StatusFlags::BID_OVERFLOW_EXCEPTION;
                            }
                        }
                    } // else if (C1 < 10^34) then C1 is the coeff.; the result is exact
                    // assemble the result
                    res.w[1] = x_sign | y_exp | C1_hi;
                    res.w[0] = C1_lo;
                } else { // if x_sign != y_sign the result is exact
                    C1_lo = C2_lo.wrapping_sub(C1_lo);
                    C1_hi = C2_hi.wrapping_sub(C1_hi);
                    if C1_lo > C2_lo {
                        C1_hi -= 1;
                    }
                    if C1_hi >= 0x8000000000000000u64 { // negative coefficient!
                        C1_lo  = !C1_lo;
                        C1_lo += 1;
                        C1_hi  = !C1_hi;
                        if C1_lo == 0x0 {
                            C1_hi += 1;
                        }
                        x_sign = y_sign; // the result will have the sign of y
                    }
                    // the result can be zero, but it cannot overflow
                    if C1_lo == 0 && C1_hi == 0 {
                        // assemble the result
                        res.w[1] = if x_exp < y_exp { x_exp } else { y_exp };
                        res.w[0] = 0;
                        if rnd_mode == RoundingMode::BID_ROUNDING_DOWN {
                            res.w[1] |= 0x8000000000000000u64;
                        }

                        #[cfg(target_endian = "big")]
                        BID_SWAP128(&mut res);

                        return res;
                    }
                    // assemble the result
                    res.w[1] = y_sign | y_exp | C1_hi;
                    res.w[0] = C1_lo;
                }
            }
        }

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    }
}
