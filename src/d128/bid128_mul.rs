/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(target_endian = "big")]
use crate::d128::bid_conf::BID_SWAP128;

use crate::d128::bid128_fma::{bid128_ext_fma, bid64qqq_fma};
use crate::d128::constants::*;
use crate::d128::convert::bid64_to_bid128;
use crate::d128::dec128::{_IDEC_flags, BID_SINT64, BID_UINT128, BID_UINT64};

pub (crate) fn bid64dq_mul(x: &BID_UINT64, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let x1: BID_UINT128 = bid64_to_bid128(*x, pfpsf);

    bid64qq_mul(&x1, y, rnd_mode, pfpsf)
}

pub (crate) fn bid64qd_mul(x: &BID_UINT128, y: &BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let y1: BID_UINT128 = bid64_to_bid128(*y, pfpsf);

    bid64qq_mul(x, &y1, rnd_mode, pfpsf)
}

pub (crate) fn bid64qq_mul(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let z: BID_UINT128 = BID_UINT128 { w: [0x0000000000000000u64, 0x5ffe000000000000u64] };
    let x_sign: BID_UINT64;
    let y_sign: BID_UINT64;
    let p_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let y_exp: BID_UINT64;
    let p_exp: BID_UINT64;
    let true_p_exp: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C2: BID_UINT128 = BID_UINT128::default();

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut z);

    // skip cases where at least one operand is NaN or infinity
    if !(((x.w[BID_HIGH_128W]  & MASK_NAN)    == MASK_NAN)
      || ((y.w[BID_HIGH_128W] & MASK_NAN)     == MASK_NAN)
      || ((x.w[BID_HIGH_128W] & MASK_ANY_INF) == MASK_INF)
      || ((y.w[BID_HIGH_128W] & MASK_ANY_INF) == MASK_INF)) {
        // x, y are 0 or f but not inf or NaN => unpack the arguments and check
        // for non-canonical values

        x_sign  = x.w[BID_HIGH_128W] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
        C1.w[1] = x.w[BID_HIGH_128W] & MASK_COEFF;
        C1.w[0] = x.w[BID_LOW_128W];
        // check for non-canonical values - treated as zero
        if (x.w[BID_HIGH_128W] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            // G0_G1=11 => non-canonical
            x_exp = (x.w[BID_HIGH_128W] << 2) & MASK_EXP;	// biased and shifted left 49 bits
            C1.w[1] = 0;	// significand high
            C1.w[0] = 0;	// significand low
        } else {	// G0_G1 != 11
            x_exp = x.w[BID_HIGH_128W] & MASK_EXP;	// biased and shifted left 49 bits
            if C1.w[1] > 0x0001ed09bead87c0u64 || (C1.w[1] == 0x0001ed09bead87c0u64 && C1.w[0] > 0x378d8e63ffffffffu64) {
                // x is non-canonical if coefficient is larger than 10^34 -1
                C1.w[1] = 0;
                C1.w[0] = 0;
            } else {
                // canonical
            }
        }
        y_sign  = y.w[BID_HIGH_128W] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
        C2.w[1] = y.w[BID_HIGH_128W] & MASK_COEFF;
        C2.w[0] = y.w[BID_LOW_128W];
        // check for non-canonical values - treated as zero
        if (y.w[BID_HIGH_128W] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            // G0_G1=11 => non-canonical
            y_exp = (y.w[BID_HIGH_128W] << 2) & MASK_EXP;	// biased and shifted left 49 bits
            C2.w[1] = 0;	// significand high
            C2.w[0] = 0;	// significand low
        } else {	// G0_G1 != 11
            y_exp = y.w[BID_HIGH_128W] & MASK_EXP;	// biased and shifted left 49 bits
            if C2.w[1] > 0x0001ed09bead87c0u64 || (C2.w[1] == 0x0001ed09bead87c0u64 && C2.w[0] > 0x378d8e63ffffffffu64) {
                // y is non-canonical if coefficient is larger than 10^34 -1
                C2.w[1] = 0;
                C2.w[0] = 0;
            } else {
                // canonical
            }
        }
        p_sign = x_sign ^ y_sign;	// sign of the product

        true_p_exp = ((x_exp >> 49) - 6176 + (y_exp >> 49) - 6176) as i32;
        // true_p_exp, p_exp are used only for 0 * 0, 0 * f, or f * 0
        p_exp = if true_p_exp < -398 {
            0                                       // cannot be less than EXP_MIN
        } else if true_p_exp > 369 {
            ((369 + 398) as BID_UINT64) << 53       // cannot be more than EXP_MAX
        } else {
            ((true_p_exp + 398) as BID_UINT64) << 53
        };

        if (C1.w[1] == 0x0 && C1.w[0] == 0x0) || (C2.w[1] == 0x0 && C2.w[0] == 0x0) {
            // x = 0 or y = 0
            // the result is 0
            return p_sign | p_exp;	// preferred exponent in [EXP_MIN, EXP_MAX]
        }// else continue
    }
    // swap x and y - ensure that a NaN in x has 'higher precedence' than one in y
    bid64qqq_fma(y, x, &z, rnd_mode, pfpsf)
}

pub (crate) fn bid128_fma(x: &BID_UINT128, y: &BID_UINT128, z: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut is_midpoint_lt_even: bool = false;
    let mut is_midpoint_gt_even: bool = false;
    let mut is_inexact_lt_midpoint: bool = false;
    let mut is_inexact_gt_midpoint: bool = false;

    let res = bid128_ext_fma(
        &mut is_midpoint_lt_even,
        &mut is_midpoint_gt_even,
        &mut is_inexact_lt_midpoint,
        &mut is_inexact_gt_midpoint,
        x, y,  z,
        rnd_mode, pfpsf);

    res
}

pub (crate) fn bid128dd_mul(x: &BID_UINT64, y: &BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let x1  = bid64_to_bid128(*x, pfpsf);
    let y1  = bid64_to_bid128(*y, pfpsf);
    let res = bid128_mul(&x1, &y1, rnd_mode, pfpsf);

    res
}

pub (crate) fn bid128dq_mul(x: &BID_UINT64, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let x1  = bid64_to_bid128(*x, pfpsf);
    let res = bid128_mul(&x1, y, rnd_mode, pfpsf);

    res
}

pub (crate) fn bid128qd_mul(x: &BID_UINT128, y: &BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
  let y1  = bid64_to_bid128(*y, pfpsf);
  let res = bid128_mul(x, &y1, rnd_mode, pfpsf);

  res
}

/// bid128_mul stands for bid128qq_mul
pub (crate) fn bid128_mul(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let z: BID_UINT128 = BID_UINT128 { w: [0x0000000000000000u64, 0x5ffe000000000000u64] };
    let x_sign: BID_UINT64;
    let y_sign: BID_UINT64;
    let p_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let y_exp: BID_UINT64;
    let p_exp: BID_UINT64;
    let true_p_exp: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C2: BID_UINT128 = BID_UINT128::default();

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    #[cfg(target_endian = "big")]
    let mut y = *y;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut y);

    // skip cases where at least one operand is NaN or infinity
    if !(((x.w[1] & MASK_NAN)     == MASK_NAN)
      || ((y.w[1] & MASK_NAN)     == MASK_NAN)
      || ((x.w[1] & MASK_ANY_INF) == MASK_INF)
      || ((y.w[1] & MASK_ANY_INF) == MASK_INF)) {
        // x, y are 0 or f but not inf or NaN => unpack the arguments and check
        // for non-canonical values

        x_sign  = x.w[1] & MASK_SIGN;           // 0 for positive, MASK_SIGN for negative
        C1.w[1] = x.w[1] & MASK_COEFF;
        C1.w[0] = x.w[0];
        // check for non-canonical values - treated as zero
        if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            // G0_G1=11 => non-canonical
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
        y_sign  = y.w[1] & MASK_SIGN;           // 0 for positive, MASK_SIGN for negative
        C2.w[1] = y.w[1] & MASK_COEFF;
        C2.w[0] = y.w[0];
        // check for non-canonical values - treated as zero
        if (y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            // G0_G1=11 => non-canonical
            y_exp   = (y.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
            C2.w[1] = 0;                        // significand high
            C2.w[0] = 0;                        // significand low
        } else {                                // G0_G1 != 11
            y_exp = y.w[1] & MASK_EXP;	// biased and shifted left 49 bits
            if C2.w[1] > 0x0001ed09bead87c0u64 || (C2.w[1] == 0x0001ed09bead87c0u64 && C2.w[0] > 0x378d8e63ffffffffu64) {
                // y is non-canonical if coefficient is larger than 10^34 -1
                C2.w[1] = 0;
                C2.w[0] = 0;
            } else {
                // canonical
            }
        }
        p_sign = x_sign ^ y_sign;	// sign of the product

        true_p_exp = (((x_exp >> 49) as BID_SINT64) - 6176 + ((y_exp >> 49) as BID_SINT64) - 6176) as i32;
        // true_p_exp, p_exp are used only for 0 * 0, 0 * f, or f * 0
        p_exp = if true_p_exp < -6176 {
            0u64                                    // cannot be less than EXP_MIN
        } else if true_p_exp > 6111 {
            ((6111 + 6176) as BID_UINT64) << 49	    // cannot be more than EXP_MAX
        } else {
            ((true_p_exp + 6176) as BID_UINT64) << 49
        };

        if (C1.w[1] == 0x0 && C1.w[0] == 0x0) || (C2.w[1] == 0x0 && C2.w[0] == 0x0) {
            // x = 0 or y = 0
            // the result is 0
            let mut res = BID_UINT128 { w: [0x0, p_sign | p_exp] }; // preferred exponent in [EXP_MIN, EXP_MAX]
            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut resres);
            return res;
        }	// else continue
    }

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut y);

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut z);

    // swap x and y - ensure that a NaN in x has 'higher precedence' than one in y
    bid128_fma(y, x, &z, rnd_mode, pfpsf)
}