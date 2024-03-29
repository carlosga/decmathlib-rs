/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#[cfg(target_endian = "big")]
use crate::bid_internal::BID_SWAP128;

use crate::bid128_fma::{bid128_fma};
use crate::bid_internal::*;
use crate::d128::{_IDEC_flags, RoundingMode};

/// Decimal floating-point multiplication
pub (crate) fn bid128_mul(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: RoundingMode, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    #[cfg(target_endian = "big")]
    let mut z: BID_UINT128 = BID_UINT128 { w: [0x0000000000000000u64, 0x5ffe000000000000u64] };

    #[cfg(target_endian = "little")]
    let z: BID_UINT128 = BID_UINT128 { w: [0x0000000000000000u64, 0x5ffe000000000000u64] };

    let x_sign: BID_UINT64;
    let y_sign: BID_UINT64;
    let p_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let y_exp: BID_UINT64;
    let p_exp: BID_UINT64;
    let true_p_exp: i32;
    let mut C1: BID_UINT128 = Default::default();
    let mut C2: BID_UINT128 = Default::default();

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
            #[cfg(target_endian = "big")]
            let mut res = BID_UINT128 { w: [0x0, p_sign | p_exp] }; // preferred exponent in [EXP_MIN, EXP_MAX]

            #[cfg(target_endian = "little")]
            let res = BID_UINT128 { w: [0x0, p_sign | p_exp] }; // preferred exponent in [EXP_MIN, EXP_MAX]

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

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
    #[cfg(target_endian = "big")]
    return bid128_fma(&y, &x, &z, rnd_mode, pfpsf);

    #[cfg(target_endian = "little")]
    return bid128_fma(y, x, &z, rnd_mode, pfpsf);
}