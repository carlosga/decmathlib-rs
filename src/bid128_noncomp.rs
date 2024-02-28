/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */
/* BID128 non-computational functions:                                                                */
/*  - bid128_abs                                                                                      */
/*  - bid128_class                                                                                    */
/*  - bid128_copy                                                                                     */
/*  - bid128_copy_sign                                                                                */
/*  - bid128_inf                                                                                      */
/*  - bid128_is_canonical                                                                             */
/*  - bid128_is_finite                                                                                */
/*  - bid128_is_inf                                                                                   */
/*  - bid128_is_nan                                                                                   */
/*  - bid128_is_normal                                                                                */
/*  - bid128_is_signaling                                                                             */
/*  - bid128_is_signed                                                                                */
/*  - bid128_is_subnormal                                                                             */
/*  - bid128_is_zero                                                                                  */
/*  - bid128_nan                                                                                      */
/*  - bid128_negate                                                                                   */
/*  - bid128_radix                                                                                    */
/*  - bid128_same_quantum                                                                             */
/*  - bid128_total_order                                                                              */
/*  - bid128_total_order_mag                                                                          */
/* -------------------------------------------------------------------------------------------------- */

#[cfg(target_endian = "big")]
use crate::bid_conf::BID_SWAP128;

use crate::bid128::{BID_NR_DIGITS, BID_TEN2K128, BID_TEN2K64};
use crate::bid128_string::bid128_from_string;
use crate::bid_internal::*;
use crate::d128::{DEFAULT_ROUNDING_MODE, _IDEC_flags, ClassTypes};

/// Return true if and only if x has negative sign
pub (crate) fn bid128_is_signed(x: &BID_UINT128) -> bool {
    (x.w[BID_HIGH_128W] & MASK_SIGN) == MASK_SIGN
}

/// Return true if and only if x is normal (not zero, subnormal, infinite, or NaN)
pub (crate) fn bid128_is_normal(x: &BID_UINT128) -> bool {
    let x_exp: BID_UINT64;
    let C1_hi: BID_UINT64;
    let C1_lo: BID_UINT64;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let exp: i32;
    let mut q: i32;
    let x_nr_bits: i32;

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    // test for special values - infinity or NaN
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        return false;
    }
    // unpack x
    x_exp = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1_hi = x.w[1] & MASK_COEFF;
    C1_lo = x.w[0];
    // test for zero
    if C1_hi == 0 && C1_lo == 0 {
        return false;
    }
    // test for non-canonical values of the argument x
    if (((C1_hi  > 0x0001ed09bead87c0u64)
     || ((C1_hi == 0x0001ed09bead87c0u64)
      && (C1_lo  > 0x378d8e63ffffffffu64)))
     && ((x.w[1] & 0x6000000000000000u64) != 0x6000000000000000u64))
     || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64) {
        return false;
    }
    // x is subnormal or normal
    // determine the number of digits q in the significand
    // q = nr. of decimal digits in x
    // determine first the nr. of bits in x
    unsafe {
        if C1_hi == 0 {
            if C1_lo >= 0x0020000000000000u64 {	// x >= 2^53
                // split the 64-bit value in two 32-bit halves to avoid rounding errors
                tmp1.d    = (C1_lo >> 32) as f64;	// exact conversion
                x_nr_bits = (33 + ((((tmp1.ui64 >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            } else {	// if x < 2^53
                tmp1.d    = C1_lo as f64;	// exact conversion
                x_nr_bits = (1 + ((((tmp1.ui64 >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            }
        } else {	// C1_hi != 0 => nr. bits = 64 + nr_bits (C1_hi)
            tmp1.d    = C1_hi as f64;	// exact conversion
            x_nr_bits = (65 + ((((tmp1.ui64 >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
        }
    }
    q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
    if q == 0 {
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
        if  C1_hi  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
        || (C1_hi == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
         && C1_lo >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
            q += 1;
        }
    }
    exp = (x_exp >> 49) as i32 - 6176;
    // test for subnormal values of x
    if exp + q <= -6143 { false } else { true }
}

/// Return true if and only if x is subnormal
pub (crate) fn bid128_is_subnormal(x: &BID_UINT128) -> bool {
    let x_exp: BID_UINT64;
    let C1_hi: BID_UINT64;
    let C1_lo: BID_UINT64;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let exp: i32;
    let mut q: i32;
    let x_nr_bits: i32;

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    // test for special values - infinity or NaN
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        return false;
    }
    // unpack x
    x_exp = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1_hi = x.w[1] & MASK_COEFF;
    C1_lo = x.w[0];
    // test for zero
    if C1_hi == 0 && C1_lo == 0 {
        return false;
    }
    // test for non-canonical values of the argument x
    if (((C1_hi  > 0x0001ed09bead87c0u64)
     || ((C1_hi == 0x0001ed09bead87c0u64)
      && (C1_lo  > 0x378d8e63ffffffffu64)))
     && ((x.w[1] & 0x6000000000000000u64) != 0x6000000000000000u64))
     || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64) {
        return false;
    }
    // x is subnormal or normal
    // determine the number of digits q in the significand
    // q = nr. of decimal digits in x
    // determine first the nr. of bits in x
    unsafe {
        if C1_hi == 0 {
            if C1_lo >= 0x0020000000000000u64 {	// x >= 2^53
                // split the 64-bit value in two 32-bit halves to avoid rounding errors
                tmp1.d    = (C1_lo >> 32) as f64;	// exact conversion
                x_nr_bits = (33 + ((((tmp1.ui64 >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            } else {	// if x < 2^53
                tmp1.d    = C1_lo as f64;	// exact conversion
                x_nr_bits = (1 + ((((tmp1.ui64 >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            }
        } else {	// C1_hi != 0 => nr. bits = 64 + nr_bits (C1_hi)
            tmp1.d    = C1_hi as f64;	// exact conversion
            x_nr_bits = (65 + ((((tmp1.ui64 >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
        }
    }
    q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
    if q == 0 {
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
        if  C1_hi  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
        || (C1_hi == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
         && C1_lo >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
            q += 1;
        }
    }
    exp = (x_exp >> 49) as i32 - 6176;
    // test for subnormal values of x
    exp + q <= -6143
    // if (exp + q <= -6143) {  1 } else { 0 }
}

/// Return true if and only if x is zero, subnormal or normal (not infinite or NaN)
pub (crate) fn bid128_is_finite(x: &BID_UINT128) -> bool {
    (x.w[BID_HIGH_128W] & MASK_INF) != MASK_INF
}

/// Return true if and only if x is +0 or -0
pub (crate) fn bid128_is_zero(x: &BID_UINT128) -> bool {
    let mut sig_x: BID_UINT128 = BID_UINT128::default();

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(x);

    if (x.w[1] & MASK_INF) == MASK_INF {
        return false;
    }
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    if (sig_x.w[1] > 0x0001ed09bead87c0u64) ||	                                            // significand is non-canonical
        ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||	// significand is non-canonical
        ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64) ||                      // significand is non-canonical
        (sig_x.w[1] == 0 && sig_x.w[0] == 0) {	                                            // significand is 0
        return true;
    }
    false
}

/// Return true if and only if x is infinite
pub (crate) fn bid128_is_inf(x: &BID_UINT128) -> bool {
    ((x.w[BID_HIGH_128W] & MASK_INF) == MASK_INF) && ((x.w[BID_HIGH_128W] & MASK_NAN) != MASK_NAN)
}

/// Return true if and only if x is a signaling NaN
pub (crate) fn bid128_is_signaling(x: &BID_UINT128) -> bool {
    (x.w[BID_HIGH_128W] & MASK_SNAN) == MASK_SNAN
}

/// Return true if and only if x is a finite number, infinity, or NaN that is canonical
pub (crate) fn bid128_is_canonical(x: &BID_UINT128) -> bool {
    let mut sig_x: BID_UINT128 = BID_UINT128::default();

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    if (x.w[1] & MASK_NAN) == MASK_NAN {	// NaN
        if x.w[1] & 0x01ffc00000000000u64 != 0 {
            return false;
        }
        sig_x.w[1] = x.w[1] & 0x00003fffffffffffu64;	// 46 bits
        sig_x.w[0] = x.w[0];	// 64 bits
        // payload must be < 10^33 = 0x0000314dc6448d93_38c15b0a00000000
        return sig_x.w[1]  < 0x0000314dc6448d93u64
           || (sig_x.w[1] == 0x0000314dc6448d93u64
            && sig_x.w[0]  < 0x38c15b0a00000000u64)
    } else if (x.w[1] & MASK_INF) == MASK_INF {	// infinity
        return if (x.w[1] & 0x03ffffffffffffffu64) != 0 || x.w[0] != 0 { false } else { true };
    }
    // not NaN or infinity; extract significand to ensure it is canonical
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    // a canonical number has a coefficient < 10^34
    //    (0x0001ed09_bead87c0_378d8e64_00000000)
    if (sig_x.w[1]  > 0x0001ed09bead87c0u64) ||	// significand is non-canonical
      ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||	// significand is non-canonical
      ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64) {
        false
    } else {
        true
    }
}

/// Return true if and only if x is a NaN
pub (crate) fn bid128_is_nan(x: &BID_UINT128) -> bool {
    (x.w[BID_HIGH_128W] & MASK_NAN) == MASK_NAN
}

/// Copies a decimal floating-point operand x to a destination in the same format, with no change
pub (crate) fn bid128_copy(x: &BID_UINT128) -> BID_UINT128 {
    *x
}

/// Copies a 128-bit decimal floating-point operand x to a destination in the same format, reversing the sign
pub (crate) fn bid128_negate(x: &BID_UINT128) -> BID_UINT128 {
    let mut res = *x;
    res.w[BID_HIGH_128W] ^= MASK_SIGN;
    res
}

/// Copies a 128-bit decimal floating-point operand x to a destination in the same format, changing the sign to positive
pub (crate) fn bid128_abs(x: &BID_UINT128) -> BID_UINT128 {
    let mut res = *x;
    res.w[BID_HIGH_128W] &= !MASK_SIGN;
    res
}

/// Copies a 128-bit decimal floating-point operand x to a destination in the same format as x, but with the sign of y
pub (crate) fn bid128_copy_sign(x: &BID_UINT128, y: &BID_UINT128) -> BID_UINT128 {
    let mut res = *x;
    res.w[BID_HIGH_128W] = (x.w[BID_HIGH_128W] & !MASK_SIGN) | y.w[BID_HIGH_128W] & MASK_SIGN;
    res
}

/// Tells which of the following ten classes x falls into (details in the IEEE Standard 754-2008):
/// signalingNaN, quietNaN, negativeInfinity, negativeNormal, negativeSubnormal, negativeZero, positiveZero,
/// positiveSubnormal, positiveNormal, positiveInfinity
pub (crate) fn bid128_class(x: &BID_UINT128) -> ClassTypes {
    let sig_x_prime256: BID_UINT256;
    let sig_x_prime192: BID_UINT192;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let exp_x: i32;

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    if (x.w[1] & MASK_NAN) == MASK_NAN {
        return if (x.w[1] & MASK_SNAN) == MASK_SNAN { ClassTypes::SignalingNaN } else { ClassTypes::QuietNaN };
    }
    if (x.w[1] & MASK_INF) == MASK_INF {
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN { ClassTypes::NegativeInfinity } else { ClassTypes::PositiveInfinity };
    }
    // decode number into exponent and significand
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    // check for zero or non-canonical
    if (sig_x.w[1] > 0x0001ed09bead87c0u64)
   || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64))
   || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64)
   || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN { ClassTypes::NegativeZero } else { ClassTypes::PositiveZero };
    }
    exp_x = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;
    // if exponent is less than -6176, the number may be subnormal
    // (less than the smallest normal value)
    //  the smallest normal value is 1 x 10^-6143 = 10^33 x 10^-6176
    //  if (exp_x - 6176 < -6143)
    if exp_x < 33 {	// sig_x * 10^exp_x
        if exp_x > 19 {
            sig_x_prime256 = __mul_128x128_to_256(&sig_x, &BID_TEN2K128[(exp_x - 20) as usize]);
            // 10^33 = 0x0000314dc6448d93_38c15b0a00000000
            if (sig_x_prime256.w[3] == 0) && (sig_x_prime256.w[2] == 0)
                && ((sig_x_prime256.w[1] < 0x0000314dc6448d93u64) || ((sig_x_prime256.w[1] == 0x0000314dc6448d93u64)
                && (sig_x_prime256.w[0] < 0x38c15b0a00000000u64))) {
                return if (x.w[1] & MASK_SIGN) == MASK_SIGN {
                    ClassTypes::NegativeSubnormal
                } else {
                    ClassTypes::PositiveSubnormal
                };
            }
        } else {
            sig_x_prime192 = __mul_64x128_to_192(BID_TEN2K64[exp_x as usize], &sig_x);
            // 10^33 = 0x0000314dc6448d93_38c15b0a00000000
            if (sig_x_prime192.w[2] == 0)
                && ((sig_x_prime192.w[1] < 0x0000314dc6448d93u64) || ((sig_x_prime192.w[1] == 0x0000314dc6448d93u64)
                && (sig_x_prime192.w[0] < 0x38c15b0a00000000u64))) {
                return if (x.w[1] & MASK_SIGN) == MASK_SIGN {
                    ClassTypes::NegativeSubnormal
                } else {
                    ClassTypes::PositiveSubnormal
                }
            }
        }
    }
    // otherwise, normal number, determine the sign
    if (x.w[1] & MASK_SIGN) == MASK_SIGN { ClassTypes::NegativeNormal } else { ClassTypes::PositiveNormal }
}

/// sameQuantum(x, y) is true if the exponents of x and y are the same, and false otherwise;
/// sameQuantum(NaN, NaN) and sameQuantum(inf, inf) are true;
/// if exactly one operand is infinite or exactly one operand is NaN, sameQuantum is false
pub (crate) fn bid128_same_quantum(x: &BID_UINT128, y: &BID_UINT128) -> bool {
    let x_exp: BID_UINT64;
    let y_exp: BID_UINT64;

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    #[cfg(target_endian = "big")]
    let mut y = *y;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut y);

    // if both operands are NaN, return true
    if (x.w[1] & MASK_NAN) == MASK_NAN || ((y.w[1] & MASK_NAN) == MASK_NAN) {
        return (x.w[1] & MASK_NAN) == MASK_NAN && (y.w[1] & MASK_NAN) == MASK_NAN;
    }
    // if both operands are INF, return true
    if (x.w[1] & MASK_INF) == MASK_INF || (y.w[1] & MASK_INF) == MASK_INF {
        return ((x.w[1] & MASK_INF) == MASK_INF) && ((y.w[1] & MASK_INF) == MASK_INF);
    }
    // decode exponents for both numbers, and return true if they match
    if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {	// G0_G1=11
        x_exp = (x.w[1] << 2) & MASK_EXP;	// biased and shifted left 49 bits
    } else {	// G0_G1 != 11
        x_exp = x.w[1] & MASK_EXP;	// biased and shifted left 49 bits
    }
    if (y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {	// G0_G1=11
        y_exp = (y.w[1] << 2) & MASK_EXP;	// biased and shifted left 49 bits
    } else {	// G0_G1 != 11
        y_exp = y.w[1] & MASK_EXP;	// biased and shifted left 49 bits
    }
    x_exp == y_exp
}

/// Return true if x and y _oare ordered (see the IEEE Standard 754-2008)
pub (crate) fn bid128_total_order(x: &BID_UINT128, y: &BID_UINT128) -> bool {
    let mut exp_x: i32;
    let mut exp_y: i32;
    let mut sig_x: BID_UINT128  = BID_UINT128::default();
    let mut sig_y: BID_UINT128  = BID_UINT128::default();
    let mut pyld_y: BID_UINT128 = BID_UINT128::default();
    let mut pyld_x: BID_UINT128 = BID_UINT128::default();
    let sig_n_prime192: BID_UINT192;
    let sig_n_prime256: BID_UINT256;
    let mut x_is_zero = false;
    let mut y_is_zero = false;

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(x);

    #[cfg(target_endian = "big")]
    let mut y = *y;

    #[cfg(target_endian = "big")]
    BID_SWAP128(y);

    // NaN (CASE 1)
    // if x and y are unordered numerically because either operand is NaN
    //    (1) totalOrder(-NaN, number) is true
    //    (2) totalOrder(number, +NaN) is true
    //    (3) if x and y are both NaN:
    //       i) negative sign bit < positive sign bit
    //       ii) signaling < quiet for +NaN, reverse for -NaN
    //       iii) lesser payload < greater payload for +NaN (reverse for -NaN)
    //       iv) else if bitwise identical (in canonical form), return 1
    if (x.w[1] & MASK_NAN) == MASK_NAN {
        // if x is -NaN
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN {
            // return true, unless y is -NaN also
            if (y.w[1] & MASK_NAN) != MASK_NAN || (y.w[1] & MASK_SIGN) != MASK_SIGN {
                true   // y is a number, return 1
            } else {        // if y and x are both -NaN
                pyld_x.w[1] = x.w[1] & 0x00003fffffffffffu64;
                pyld_x.w[0] = x.w[0];
                pyld_y.w[1] = y.w[1] & 0x00003fffffffffffu64;
                pyld_y.w[0] = y.w[0];
                if (pyld_x.w[1] > 0x0000314dc6448d93u64)
                    || ((pyld_x.w[1] == 0x0000314dc6448d93u64) && (pyld_x.w[0] > 0x38c15b09ffffffffu64)) {
                    pyld_x.w[1] = 0;
                    pyld_x.w[0] = 0;
                }
                if (pyld_y.w[1] > 0x0000314dc6448d93u64)
                    || ((pyld_y.w[1] == 0x0000314dc6448d93u64) && (pyld_y.w[0] > 0x38c15b09ffffffffu64)) {
                    pyld_y.w[1] = 0;
                    pyld_y.w[0] = 0;
                }
                // if x and y are both -SNaN or both -QNaN, we have to compare payloads
                // this statement evaluates to true if both are SNaN or QNaN
                if !(((y.w[1] & MASK_SNAN) == MASK_SNAN) ^ ((x.w[1] & MASK_SNAN) == MASK_SNAN)) {
                    // it comes down to the payload.  we want to return true if x has a
                    // larger payload, or if the payloads are equal (canonical forms
                    // are bitwise identical)
                    (pyld_x.w[1] > pyld_y.w[1]) || ((pyld_x.w[1] == pyld_y.w[1]) && (pyld_x.w[0] >= pyld_y.w[0]))
                } else {
                    // either x = -SNaN and y = -QNaN or x = -QNaN and y = -SNaN
                    (y.w[1] & MASK_SNAN) == MASK_SNAN
                    // totalOrder (-QNaN, -SNaN) == 1
                }
            }
        } else {    // x is +NaN
            // return false, unless y is +NaN also
            if (y.w[1] & MASK_NAN) != MASK_NAN || (y.w[1] & MASK_SIGN) == MASK_SIGN {
                false    // y is a number, return 1
            } else {
                // x and y are both +NaN;
                pyld_x.w[1] = x.w[1] & 0x00003fffffffffffu64;
                pyld_x.w[0] = x.w[0];
                pyld_y.w[1] = y.w[1] & 0x00003fffffffffffu64;
                pyld_y.w[0] = y.w[0];
                if (pyld_x.w[1] > 0x0000314dc6448d93u64)
                    || ((pyld_x.w[1] == 0x0000314dc6448d93u64) && (pyld_x.w[0] > 0x38c15b09ffffffffu64)) {
                    pyld_x.w[1] = 0;
                    pyld_x.w[0] = 0;
                }
                if (pyld_y.w[1] > 0x0000314dc6448d93u64)
                    || ((pyld_y.w[1] == 0x0000314dc6448d93u64) && (pyld_y.w[0] > 0x38c15b09ffffffffu64)) {
                    pyld_y.w[1] = 0;
                    pyld_y.w[0] = 0;
                }
                // if x and y are both +SNaN or both +QNaN, we have to compare payloads
                // this statement evaluates to true if both are SNaN or QNaN
                if !(((y.w[1] & MASK_SNAN) == MASK_SNAN) ^ ((x.w[1] & MASK_SNAN) == MASK_SNAN)) {
                    // it comes down to the payload.  we want to return true if x has a
                    // smaller payload, or if the payloads are equal (canonical forms
                    // are bitwise identical)
                    (pyld_x.w[1] < pyld_y.w[1]) || ((pyld_x.w[1] == pyld_y.w[1]) && (pyld_x.w[0] <= pyld_y.w[0]))
                } else {
                    // either x = SNaN and y = QNaN or x = QNaN and y = SNaN
                    (x.w[1] & MASK_SNAN) == MASK_SNAN
                    // totalOrder (-QNaN, -SNaN) == 1
                }
            }
        }
    } else if (y.w[1] & MASK_NAN) == MASK_NAN {
        // x is certainly not NAN in this case.
        // return true if y is positive
        return (y.w[1] & MASK_SIGN) != MASK_SIGN;
    }
    // SIMPLE (CASE 2)
    // if all the bits are the same, the numbers are equal.
    if (x.w[1] == y.w[1]) && (x.w[0] == y.w[0]) {
        return true;
    }
    // OPPOSITE SIGNS (CASE 3)
    // if signs are opposite, return 1 if x is negative
    // (if x < y, totalOrder is true)
    if ((x.w[1] & MASK_SIGN) == MASK_SIGN) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN) {
        return (x.w[1] & MASK_SIGN) == MASK_SIGN;
    }
    // INFINITY (CASE 4)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x == neg_inf, return (y == neg_inf);
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN {
            true
        } else {
            // x is positive infinity, only return1 if y is positive infinity as well
            (y.w[1] & MASK_INF) == MASK_INF
            // && (y & MASK_SIGN) != MASK_SIGN); (we know y has same sign as x)
        }
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so:
        //    if y is +inf, x<y
        //    if y is -inf, x>y
        return (y.w[1] & MASK_SIGN) != MASK_SIGN;
    }
    // CONVERT x
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF x IS CANONICAL
    // 9999999999999999999999999999999999 (decimal) =
    //     1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    // If the value exceeds that, it is interpreted as 0.
    if (((sig_x.w[1]  > 0x0001ed09bead87c0u64)
     || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
      && (sig_x.w[0]  > 0x378d8e63ffffffffu64)))
     && ((x.w[1] & 0x6000000000000000u64) != 0x6000000000000000u64))
     || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64)
     || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
        // check for the case where the exponent is shifted right by 2 bits!
        if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            exp_x = ((x.w[1] >> 47) & 0x000000000003fffu64) as i32;
        }
    }
    // CONVERT y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //     1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    // If the value exceeds that, it is interpreted as 0.
    if (((sig_y.w[1]  > 0x0001ed09bead87c0u64)
     || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
      && (sig_y.w[0]  > 0x378d8e63ffffffffu64)))
     && ((y.w[1] & 0x6000000000000000u64) != 0x6000000000000000u64))
     || ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64)
     || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
        // check for the case where the exponent is shifted right by 2 bits!
        if (y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            exp_y = ((y.w[1] >> 47) & 0x000000000003fffu64) as i32;
        }
    }
    // ZERO (CASE 5)
    // if x and y represent the same entities, and both are negative
    // return true iff exp_x <= exp_y
    if x_is_zero && y_is_zero {
        // we know that signs must be the same because we would have caught it
        // in case3 if signs were different
        // totalOrder(x,y) iff exp_x >= exp_y for negative numbers
        // totalOrder(x,y) iff exp_x <= exp_y for positive numbers
        if exp_x == exp_y {
            return true;
        }
        return (exp_x <= exp_y) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    }
    // if x is zero and y isn't, clearly x has the smaller payload
    if x_is_zero {
        return (y.w[1] & MASK_SIGN) != MASK_SIGN;
    }
    // if y is zero, and x isn't, clearly y has the smaller payload
    if y_is_zero {
        return (x.w[1] & MASK_SIGN) == MASK_SIGN;
    }
    // REDUNDANT REPRESENTATIONS (CASE 6)
    // if both components are either bigger or smaller
    if ((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y {
        return (x.w[1] & MASK_SIGN) == MASK_SIGN;
    }
    if ((sig_x.w[1] < sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y {
        return (x.w[1] & MASK_SIGN) != MASK_SIGN;
    }
    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if exp_x > exp_y {
        // if exp_x is 33 greater than exp_y, it is definitely larger,
        // so no need for compensation
        if exp_x - exp_y > 33 {
            return (x.w[1] & MASK_SIGN) == MASK_SIGN;
            // difference cannot be greater than 10^33
        }
        // otherwise adjust the x significand upwards
        if exp_x - exp_y > 19 {
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &BID_TEN2K128[(exp_x - exp_y - 20) as usize]);
            // the compensated significands are equal (ie "x and y represent the same
            // entities") return 1 if (negative && expx > expy) ||
            // (positive && expx < expy)
            if (sig_n_prime256.w[3] == 0) && (sig_n_prime256.w[2] == 0)
                && (sig_n_prime256.w[1] == sig_y.w[1])
                && (sig_n_prime256.w[0] == sig_y.w[0]) {
                // the case exp_x == exp_y  cannot occur, because all bits must be
                // the same - would have been caught if (x == y)
                return (exp_x <= exp_y) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
            }
            // if positive, return 1 if adjusted x is smaller than y
            return ((sig_n_prime256.w[3] == 0) && (sig_n_prime256.w[2] == 0)
                && ((sig_n_prime256.w[1] < sig_y.w[1])
                || (sig_n_prime256.w[1] == sig_y.w[1]
                && sig_n_prime256.w[0] < sig_y.w[0]))) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        }
        sig_n_prime192 = __mul_64x128_to_192(BID_TEN2K64[(exp_x - exp_y) as usize], &sig_x);
        // if positive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            return (exp_x <= exp_y) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        }
        return ((sig_n_prime192.w[2] == 0)
            && ((sig_n_prime192.w[1] < sig_y.w[1])
            || (sig_n_prime192.w[1] == sig_y.w[1]
            && sig_n_prime192.w[0]   < sig_y.w[0]))) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    }
    // if exp_x is 33 less than exp_y, it is definitely smaller,
    // no need for compensation
    if exp_y - exp_x > 33 {
        return (x.w[1] & MASK_SIGN) != MASK_SIGN;
    }
    if exp_y - exp_x > 19 {
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(exp_y - exp_x - 20) as usize]);
        // if x and y represent the same entities and both are negative
        // return true iff exp_x <= exp_y
        if (sig_n_prime256.w[3] == 0)
            && (sig_n_prime256.w[2] == 0)
            && (sig_n_prime256.w[1] == sig_x.w[1])
            && (sig_n_prime256.w[0] == sig_x.w[0]) {
            return (exp_x <= exp_y) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        }
        // values are not equal, for positive numbers return 1 if x is less than y
        // and 0 otherwise
        return ((sig_n_prime256.w[3] != 0) ||
            // if upper128 bits of compensated y are non-zero, y is bigger
            (sig_n_prime256.w[2] != 0) ||
            // if upper128 bits of compensated y are non-zero, y is bigger
            (sig_n_prime256.w[1] > sig_x.w[1]) ||
            // if compensated y is bigger, y is bigger
            (sig_n_prime256.w[1] == sig_x.w[1]
                && sig_n_prime256.w[0] > sig_x.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    }
    sig_n_prime192 = __mul_64x128_to_192(BID_TEN2K64[(exp_y - exp_x) as usize], &sig_y);
    if (sig_n_prime192.w[2] == 0) && (sig_n_prime192.w[1] == sig_x.w[1])
        && (sig_n_prime192.w[0] == sig_x.w[0]) {
        return (exp_x <= exp_y) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    }
    ((sig_n_prime192.w[2] != 0)
     // if upper128 bits of compensated y are non-zero, y is bigger
  || (sig_n_prime192.w[1] > sig_x.w[1])
     // if compensated y is bigger, y is bigger
  || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] > sig_x.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN)
}

/// Return true if the absolute values of x and y are ordered (see the IEEE Standard 754-2008)
pub (crate) fn bid128_total_order_mag(x: &BID_UINT128, y: &BID_UINT128) -> bool {
    let mut exp_x: i32;
    let mut exp_y: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let mut pyld_y: BID_UINT128 = BID_UINT128::default();
    let mut pyld_x: BID_UINT128 = BID_UINT128::default();
    let sig_n_prime192: BID_UINT192;
    let sig_n_prime256: BID_UINT256;
    let mut x_is_zero = false;
    let mut y_is_zero = false;
    let mut x = *x;
    let mut y = *y;

    #[cfg(target_endian = "big")]
    BID_SWAP128 (x);

    #[cfg(target_endian = "big")]
    BID_SWAP128 (y);

    x.w[1] &= 0x7fffffffffffffffu64;
    y.w[1] &= 0x7fffffffffffffffu64;

    // NaN (CASE 1)
    // if x and y are unordered numerically because either operand is NaN
    //    (1) totalOrder(number, +NaN) is true
    //    (2) if x and y are both NaN:
    //       i) signaling < quiet for +NaN
    //       ii) lesser payload < greater payload for +NaN
    //       iii) else if bitwise identical (in canonical form), return 1
    if (x.w[1] & MASK_NAN) == MASK_NAN {
        // x is +NaN
        // return false, unless y is +NaN also
        return if (y.w[1] & MASK_NAN) != MASK_NAN {
            false    // y is a number, return 0
        } else {
            // x and y are both +NaN;
            pyld_x.w[1] = x.w[1] & 0x00003fffffffffffu64;
            pyld_x.w[0] = x.w[0];
            pyld_y.w[1] = y.w[1] & 0x00003fffffffffffu64;
            pyld_y.w[0] = y.w[0];
            if (pyld_x.w[1] > 0x0000314dc6448d93u64)
                || ((pyld_x.w[1] == 0x0000314dc6448d93u64) && (pyld_x.w[0] > 0x38c15b09ffffffffu64)) {
                pyld_x.w[1] = 0;
                pyld_x.w[0] = 0;
            }
            if (pyld_y.w[1] > 0x0000314dc6448d93u64)
                || ((pyld_y.w[1] == 0x0000314dc6448d93u64) && (pyld_y.w[0] > 0x38c15b09ffffffffu64)) {
                pyld_y.w[1] = 0;
                pyld_y.w[0] = 0;
            }
            // if x and y are both +SNaN or both +QNaN, we have to compare payloads
            // this statement evaluates to true if both are SNaN or QNaN
            if !(((y.w[1] & MASK_SNAN) == MASK_SNAN) ^ ((x.w[1] & MASK_SNAN) == MASK_SNAN)) {
                // it comes down to the payload.  we want to return true if x has a
                // smaller payload, or if the payloads are equal (canonical forms
                // are bitwise identical)
                (pyld_x.w[1] < pyld_y.w[1]) || ((pyld_x.w[1] == pyld_y.w[1]) && (pyld_x.w[0] <= pyld_y.w[0]))
            } else {
                // either x = SNaN and y = QNaN or x = QNaN and y = SNaN
                (x.w[1] & MASK_SNAN) == MASK_SNAN
                // totalOrder (-QNaN, -SNaN) == 1
            }
        }
    } else if (y.w[1] & MASK_NAN) == MASK_NAN {
        // x is certainly not NAN in this case.
        // return true because y is positive
        return true;
    }
    // SIMPLE (CASE 2)
    // if all the bits are the same, the numbers are equal.
    if (x.w[1] == y.w[1]) && (x.w[0] == y.w[0]) {
        return true;
    }
    // INFINITY (CASE 3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // x is positive infinity, only return 1 if y is positive infinity as well
        return (y.w[1] & MASK_INF) == MASK_INF;
        // (we know y has same sign as x)
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so:
        //    since y is +inf, x<y
        return true;
    } else {
        // continue
    }

    // CONVERT x
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF x IS CANONICAL
    // 9999999999999999999999999999999999 (decimal) =
    //     1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    // If the value exceeds that, it is interpreted as 0.
    if (((sig_x.w[1]  > 0x0001ed09bead87c0u64)
     || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
      && (sig_x.w[0]  > 0x378d8e63ffffffffu64)))
     && ((x.w[1] & 0x6000000000000000u64) != 0x6000000000000000u64))
     || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64)
     || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
        // check for the case where the exponent is shifted right by 2 bits!
        if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            exp_x = ((x.w[1] >> 47) & 0x000000000003fffu64) as i32;
        }
    }
    // CONVERT y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //     1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    // If the value exceeds that, it is interpreted as 0.
    if (((sig_y.w[1]  > 0x0001ed09bead87c0u64)
     || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
      && (sig_y.w[0]  > 0x378d8e63ffffffffu64)))
     && ((y.w[1] & 0x6000000000000000u64) != 0x6000000000000000u64))
     || ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64)
     || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
        // check for the case where the exponent is shifted right by 2 bits!
        if (y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            exp_y = ((y.w[1] >> 47) & 0x000000000003fffu64) as i32;
        }
    }
    // ZERO (CASE 4)
    if x_is_zero && y_is_zero {
        // we know that signs must be the same because we would have caught it
        // in case3 if signs were different
        // totalOrder(x,y) iff exp_x <= exp_y for positive numbers
        if exp_x == exp_y {
            return true;
        }
        return exp_x <= exp_y;
    }
    // if x is zero and y isn't, clearly x has the smaller payload
    if x_is_zero {
        return true;
    }
    // if y is zero, and x isn't, clearly y has the smaller payload
    if y_is_zero {
        return false;
    }
    // REDUNDANT REPRESENTATIONS (CASE 5)
    // if both components are either bigger or smaller
    if ((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y {
        return false;
    }
    if ((sig_x.w[1] < sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y {
        return true;
    }
    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if exp_x > exp_y {
        // if exp_x is 33 greater than exp_y, it is definitely larger,
        // so no need for compensation
        if exp_x - exp_y > 33 {
            return false;	// difference cannot be greater than 10^33
        }
        // otherwise adjust the x significand upwards
        if exp_x - exp_y > 19 {
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &BID_TEN2K128[(exp_x - exp_y - 20) as usize]);
            // the compensated significands are equal (ie "x and y represent the same
            // entities") return 1 if (negative && expx > expy) ||
            // (positive && expx < expy)
            if (sig_n_prime256.w[3] == 0) && (sig_n_prime256.w[2] == 0)
                && (sig_n_prime256.w[1] == sig_y.w[1])
                && (sig_n_prime256.w[0] == sig_y.w[0]) {
                // the case (exp_x == exp_y) cannot occur, because all bits must be
                // the same - would have been caught if (x == y)
                return false; // res = (exp_x <= exp_y); but exp_x > exp_y in this case
            }
            // since positive, return 1 if adjusted x is smaller than y
            return (sig_n_prime256.w[3] == 0) && (sig_n_prime256.w[2] == 0)
                && ((sig_n_prime256.w[1] < sig_y.w[1])
                || (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] < sig_y.w[0]));
        }
        sig_n_prime192 = __mul_64x128_to_192(BID_TEN2K64[(exp_x - exp_y) as usize], &sig_x);
        // if positive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            return false; // res = (exp_x <= exp_y); but exp_x > exp_y in this case
        }
        return (sig_n_prime192.w[2] == 0)
            && ((sig_n_prime192.w[1] < sig_y.w[1])
            || (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] < sig_y.w[0]));
    }
    // if exp_x is 33 less than exp_y, it is definitely smaller,
    // no need for compensation
    if exp_y - exp_x > 33 {
        return true;
    } // from this point on 0 <= exp_y - exp_x <= 32
    if exp_y - exp_x > 19 {
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(exp_y - exp_x - 20) as usize]);
        if (sig_n_prime256.w[3] == 0) && (sig_n_prime256.w[2] == 0)
            && (sig_n_prime256.w[1] == sig_x.w[1])
            && (sig_n_prime256.w[0] == sig_x.w[0]) {
            return true; // res = (exp_x <= exp_y); but 0 <= exp_y - exp_x <= 32 in this case
        }
        // values are not equal, for positive numbers return 1 if x is less than y
        // and 0 otherwise
        return (sig_n_prime256.w[3] != 0) ||
            // if upper128 bits of compensated y are non-zero, y is bigger
            (sig_n_prime256.w[2] != 0) ||
            // if upper128 bits of compensated y are non-zero, y is bigger
            (sig_n_prime256.w[1] > sig_x.w[1]) ||
            // if compensated y is bigger, y is bigger
            (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] > sig_x.w[0]);
    } // from this point on 0 <= exp_y - exp_x <= 19
    sig_n_prime192 = __mul_64x128_to_192(BID_TEN2K64[(exp_y - exp_x) as usize], &sig_y);
    if (sig_n_prime192.w[2] == 0) && (sig_n_prime192.w[1] == sig_x.w[1]) && (sig_n_prime192.w[0] == sig_x.w[0]) {
        return true; // res = (exp_x <= exp_y); but 0 <= exp_y - exp_x <= 19 in this case
    }
    (sig_n_prime192.w[2] != 0)
    // if upper128 bits of compensated y are non-zero, y is bigger
 || (sig_n_prime192.w[1] > sig_x.w[1])
    // if compensated y is bigger, y is bigger
 || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] > sig_x.w[0])
}

/// Return the radix b of the format of x, 2 or 10
// pub (crate) fn bid128_radix(_: &BID_UINT128) -> i32 {
//     // (x.w[BID_LOW_128W]) { 10 } else { 10 }
//     10
// }

pub (crate) fn bid128_inf() -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::default();
    res.w[BID_HIGH_128W] = 0x7800000000000000u64; // +inf
    res.w[BID_LOW_128W]  = 0x0000000000000000u64;
    res
}

pub (crate) fn bid128_nan(tagp: &str, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::default();

    res.w[BID_HIGH_128W] = 0x7c00000000000000u64; // +QNaN
    res.w[BID_LOW_128W]  = 0x0000000000000000u64;

    if tagp.is_empty() {
        return res;
    }

    let mut x: BID_UINT128 = bid128_from_string(tagp, DEFAULT_ROUNDING_MODE, pfpsf);

    x.w[BID_HIGH_128W]   &= 0x0000cfffffffffffu64; // valid values fit in 110 bits=46+64
    res.w[BID_HIGH_128W] |= x.w[BID_HIGH_128W];
    res.w[BID_LOW_128W]   = x.w[BID_LOW_128W];

    res
}