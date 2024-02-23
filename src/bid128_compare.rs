/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(unused_assignments)]
#![allow(dead_code)]

use crate::bid128::{BID_TEN2K128, BID_TEN2K64};
use crate::bid_internal::{__mul_128x128_to_256, __mul_64x128_to192, __mul_64x128_to_192, BID_UINT128, BID_UINT192, BID_UINT256, swap};
use crate::constants::{MASK_INF, MASK_NAN, MASK_SIGN, MASK_SNAN};
use crate::d128::StatusFlags;
use crate::d128::_IDEC_flags;

/// Compare 128-bit decimal floating-point numbers for specified relation;
/// do not signal invalid exception for quiet NaNs
pub (crate) fn bid128_quiet_equal(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> bool {
    let res: bool;
    let mut exp_x: i32;
    let mut exp_y: i32;
    let mut exp_t: i32 = 0;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let mut sig_t: BID_UINT128 = BID_UINT128::default();
    let mut sig_n_prime192: BID_UINT192 = BID_UINT192::default();
    let mut sig_n_prime256: BID_UINT256 = BID_UINT256::default();
    let mut x_is_zero: bool = false;
    let mut y_is_zero: bool = false;
    let mut non_canon_x: bool = false;
    let mut non_canon_y: bool = false;

    // NaN (CASE1)
    // if either number is NAN, the comparison is unordered,
    // rather than equal : return 0
    if ((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN) {
        if (x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        }
        res = false;
        return res;
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equivalent.
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = true;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        res = if (y.w[1] & MASK_INF) == MASK_INF {
            ((x.w[1] ^ y.w[1]) & MASK_SIGN) != MASK_SIGN
        } else {
            false
        };
        return res;
    }
    if (y.w[1] & MASK_INF) == MASK_INF {
        res = false;
        return res;
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF X IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //   If the value exceeds that, it is interpreted as 0.
    non_canon_x =   (sig_x.w[1]  > 0x0001ed09bead87c0u64)
                || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_x.w[0]  > 0x378d8e63ffffffffu64))
                || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF Y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    // If the value exceeds that, it is interpreted as 0.
    non_canon_y =  (sig_y.w[1]  > 0x0001ed09bead87c0u64)
                || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_y.w[0]  > 0x378d8e63ffffffffu64))
                ||  ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
    //    ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
    }
    if non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
    }

    if x_is_zero && y_is_zero {
        res = true;
        return res;
    } else if (x_is_zero && !y_is_zero) || (!x_is_zero && y_is_zero) {
        res = false;
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ => not equal : return 0
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = false;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    if exp_x > exp_y {                                                      // to simplify the loop below,
        swap(&mut exp_x, &mut exp_y, &mut exp_t);                           // put the larger exp in y,
        swap(&mut sig_x.w[1], &mut sig_y.w[1], &mut sig_t.w[1]);    // and the smaller exp in x
        swap(&mut sig_x.w[0], &mut sig_y.w[0], &mut sig_t.w[0]);    // and the smaller exp in x
    }

    if exp_y - exp_x > 33 {
        res = false;
        return res;
    } // difference cannot be greater than 10^33

    if exp_y - exp_x > 19 {
        // recalculate y's significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(exp_y - exp_x - 20) as usize]);
        {
            res =  (sig_n_prime256.w[3] == 0)
                && (sig_n_prime256.w[2] == 0)
                && (sig_n_prime256.w[1] == sig_x.w[1])
                && (sig_n_prime256.w[0] == sig_x.w[0]);
            return res;
        }
    }
    // else{
    //  recalculate y's significand upwards
    sig_n_prime192 = __mul_64x128_to_192(BID_TEN2K64[(exp_y - exp_x) as usize], &sig_y);

    res =  (sig_n_prime192.w[2] == 0)
        && (sig_n_prime192.w[1] == sig_x.w[1])
        && (sig_n_prime192.w[0] == sig_x.w[0]);
    res
}

/// Compare 128-bit decimal floating-point numbers for specified relation;
/// do not signal invalid exception for quiet NaNs
pub (crate) fn bid128_quiet_greater(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> bool {
    let res: bool;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let mut sig_n_prime192: BID_UINT192 = BID_UINT192::default();
    let mut sig_n_prime256: BID_UINT256 = BID_UINT256::default();
    let mut x_is_zero = false;
    let mut y_is_zero = false;
    let non_canon_x: bool;
    let non_canon_y: bool;

    // NaN (CASE1)
    // if either number is NAN, the comparison is unordered, rather than
    // equal : return 0
    if ((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN) {
        if (x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        }
        {
            res = false;
            return res;
        }
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal (not Greater).
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = false;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x is neg infinity, there is no way it is greater than y, return 0
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN {
            res = false;
            res
        }
        // x is pos infinity, it is greater, unless y is positive infinity =>
        // return y!=pos_infinity
        else {
            res = ((y.w[1] & MASK_INF) != MASK_INF) || ((y.w[1] & MASK_SIGN) == MASK_SIGN);
            res
        }
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is positive infinity, then x is less, return 0
        //                 if y is negative infinity, then x is greater, return 1
        {
            res = (y.w[1] & MASK_SIGN) == MASK_SIGN;
            return res;
        }
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF X IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_x =  (sig_x.w[1]  > 0x0001ed09bead87c0u64)
                || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_x.w[0] > 0x378d8e63ffffffffu64))
                || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF Y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_y =   (sig_y.w[1]  > 0x0001ed09bead87c0u64)
                || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_y.w[0]  > 0x378d8e63ffffffffu64))
                || ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
    //    ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
    }
    if non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
    }
    // if both numbers are zero, neither is greater => return NOTGREATERTHAN
    if x_is_zero && y_is_zero {
        res = false;
        return res;
    }
    // is x is zero, it is greater if Y is negative
    else if x_is_zero {
        res = (y.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    // is y is zero, X is greater if it is positive
    else if y_is_zero {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ, x is greater if y is negative
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = (y.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    // if exponents are the same, then we have a simple comparison
    // of the significands
    if exp_y == exp_x {
        res = ((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    }
    // if both components are either bigger or smaller,
    // it is clear what needs to be done
    if (sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y {
        {
            res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
            return res;
        }
    }
    if (sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y {
        {
            res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
            return res;
        }
    }

    diff = exp_x - exp_y;

    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
            return res;
        } // difference cannot be greater than 10^33

        if diff > 19 { // 128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &BID_TEN2K128[(diff - 20) as usize]);

            // if postitive, return whichever significand is larger
            // (converse if negative)
            if  sig_n_prime256.w[3] == 0
            && (sig_n_prime256.w[2] == 0)
             && sig_n_prime256.w[1] == sig_y.w[1]
            && (sig_n_prime256.w[0] == sig_y.w[0]) {
                res = false;
                return res;
            } // if equal, return 0
            {
                res = (((sig_n_prime256.w[3]  > 0)
                      || sig_n_prime256.w[2]  > 0)
                     || (sig_n_prime256.w[1]  > sig_y.w[1])
                     || (sig_n_prime256.w[1] == sig_y.w[1]
                      && sig_n_prime256.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
                return res;
            }
        }
        // else { //128 by 64 bit multiply -> 192 bits
        sig_n_prime192 = __mul_64x128_to_192(BID_TEN2K64[diff as usize], &sig_x);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            res = false;
            return res;
        } // if equal, return 0
        {
            res = ((sig_n_prime192.w[2]  > 0)
                || (sig_n_prime192.w[1]  > sig_y.w[1])
                || (sig_n_prime192.w[1] == sig_y.w[1]
                 && sig_n_prime192.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
            return res;
        }
    }

    diff = exp_y - exp_x;

    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }

    if diff > 19 { // 128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if  sig_n_prime256.w[3] == 0
        && (sig_n_prime256.w[2] == 0)
         && sig_n_prime256.w[1] == sig_x.w[1]
        && (sig_n_prime256.w[0] == sig_x.w[0]) {
            res = false;
            return res;
        } // if equal, return 0
        {
            res = (sig_n_prime256.w[3] != 0
                || sig_n_prime256.w[2] != 0
               || (sig_n_prime256.w[1] > sig_x.w[1]
               || (sig_n_prime256.w[1] == sig_x.w[1]
                && sig_n_prime256.w[0] > sig_x.w[0]))) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN);
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    //  adjust the y significand upwards
    sig_n_prime192 = __mul_64x128_to_192(BID_TEN2K64[diff as usize], &sig_y);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0]) {
        res = false;
        return res;
    } // if equal, return 0

    res = (sig_n_prime192.w[2] != 0
       || (sig_n_prime192.w[1]  > sig_x.w[1]
       || (sig_n_prime192.w[1] == sig_x.w[1]
        && sig_n_prime192.w[0] > sig_x.w[0]))) ^ ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    res
}

/// Compare 128-bit decimal floating-point numbers for specified relation;
/// do not signal invalid exception for quiet NaNs
pub (crate) fn bid128_quiet_greater_equal(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> bool {
    let res: bool;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let mut sig_n_prime192: BID_UINT192 = BID_UINT192::default();
    let mut sig_n_prime256: BID_UINT256 = BID_UINT256::default();
    let mut x_is_zero: bool = false;
    let mut y_is_zero: bool = false;
    let mut non_canon_x: bool = false;
    let mut non_canon_y: bool = false;

    // NaN (CASE1)
    // if either number is NAN, the comparison is unordered,
    // rather than equal : return 1
    if ((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN) {
        if (x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        }
        {
            res = false;
            return res;
        }
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal (not Greater).
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = true;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x==neg_inf, { res = (y == neg_inf)?1:0; BID_RETURN_VAL (res) }
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN { // x is -inf, so it is less than y unless y is -inf
            res = ((y.w[1] & MASK_INF) == MASK_INF) && (y.w[1] & MASK_SIGN) == MASK_SIGN;
            res
        } else { // x is pos_inf, no way for it to be less than y
            res = true;
            res
        }
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is positive infinity, then x is less, return 0
        //                 if y is negative infinity, then x is greater, return 1
        {
            res = (y.w[1] & MASK_SIGN) == MASK_SIGN;
            return res;
        }
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF X IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_x =   (sig_x.w[1]  > 0x0001ed09bead87c0u64)
                || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_x.w[0]  > 0x378d8e63ffffffffu64))
                || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF Y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_y =   (sig_y.w[1]  > 0x0001ed09bead87c0u64)
                || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_y.w[0]  > 0x378d8e63ffffffffu64))
                || ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
    //    ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
    }
    if non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
    }
    // if both numbers are zero, neither is greater => return NOTGREATERTHAN
    if x_is_zero && y_is_zero {
        res = true;
        return res;
    }
    // is x is zero, it is greater if Y is negative
    else if x_is_zero {
        res = (y.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    // is y is zero, X is greater if it is positive
    else if y_is_zero {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ, x is greater if y is negative
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = (y.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    // if exponents are the same, then we have a simple comparison of the
    // significands
    if exp_y == exp_x {
        res = ((sig_x.w[1]  > sig_y.w[1])
             || (sig_x.w[1] == sig_y.w[1]
              && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    }
    // if both components are either bigger or smaller,
    // it is clear what needs to be done
    if sig_x.w[1] >= sig_y.w[1] && sig_x.w[0] >= sig_y.w[0] && exp_x > exp_y {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    if sig_x.w[1] <= sig_y.w[1] && sig_x.w[0] <= sig_y.w[0] && exp_x < exp_y {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }

    diff = exp_x - exp_y;

    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
            return res;
        } // difference cannot be greater than 10^33

        if diff > 19 { // 128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &BID_TEN2K128[(diff - 20) as usize]);

            // if postitive, return whichever significand is larger
            // (converse if negative)
            if  sig_n_prime256.w[3] == 0
            && (sig_n_prime256.w[2] == 0)
             && sig_n_prime256.w[1] == sig_y.w[1]
            && (sig_n_prime256.w[0] == sig_y.w[0]) {
                res = true;
                return res;
            } // if equal, return 1
            {
                res = (((sig_n_prime256.w[3]  > 0)
                      || sig_n_prime256.w[2]  > 0)
                     || (sig_n_prime256.w[1]  > sig_y.w[1])
                     || (sig_n_prime256.w[1] == sig_y.w[1]
                      && sig_n_prime256.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
                return res;
            }
        }
        // else { //128 by 64 bit multiply -> 192 bits
        sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_x);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            res = true;
            return res;
        } // if equal, return 1
        {
            res = ((sig_n_prime192.w[2]  > 0)
                 || (sig_n_prime192.w[1]  > sig_y.w[1])
                 || (sig_n_prime192.w[1] == sig_y.w[1]
                  && sig_n_prime192.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
            return res;
        }
    }

    diff = exp_y - exp_x;

    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }

    if diff > 19 { // 128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if sig_n_prime256.w[3] == 0
        && (sig_n_prime256.w[2] == 0)
         && sig_n_prime256.w[1] == sig_x.w[1]
        && (sig_n_prime256.w[0] == sig_x.w[0]) {
            res = true;
            return res;
        } // if equal, return 1
        {
            res = (sig_n_prime256.w[3] == 0
                 && sig_n_prime256.w[2] == 0
                && (sig_n_prime256.w[1]  < sig_x.w[1]
                || (sig_n_prime256.w[1] == sig_x.w[1]
                 && sig_n_prime256.w[0] < sig_x.w[0]))) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    //  adjust the y significand upwards
    sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_y);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0]) {
        res = true;
        return res;
    } // if equal, return 1
    {
        res = (sig_n_prime192.w[2] == 0
           && (sig_n_prime192.w[1]  < sig_x.w[1]
           || (sig_n_prime192.w[1] == sig_x.w[1]
            && sig_n_prime192.w[0] < sig_x.w[0]))) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        res
    }
}

/// Compare 128-bit decimal floating-point numbers for specified relation;
/// do not signal invalid exception for quiet NaNs
pub (crate) fn bid128_quiet_greater_unordered(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> bool {
    let res: bool;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let mut sig_n_prime192: BID_UINT192 = BID_UINT192::default();
    let mut sig_n_prime256: BID_UINT256 = BID_UINT256::default();
    let mut x_is_zero: bool = false;
    let mut y_is_zero: bool = false;
    let mut non_canon_x: bool = false;
    let mut non_canon_y: bool = false;

    // NaN (CASE1)
    // if either number is NAN, the comparison is unordered,
    // rather than
    // equal : return 1
    if ((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN) {
        if (x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        }
        {
            res = true;
            return res;
        }
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal (not Greater).
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = false;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x is neg infinity, there is no way it is greater than y, return 0
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN {
            res = false;
            res
        }
        // x is pos infinity, it is greater, unless y is positive infinity =>
        // return y!=pos_infinity
        else {
            res = ((y.w[1] & MASK_INF) != MASK_INF) || ((y.w[1] & MASK_SIGN) == MASK_SIGN);
            res
        }
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is positive infinity, then x is less, return 0
        //                 if y is negative infinity, then x is greater, return 1
        {
            res = (y.w[1] & MASK_SIGN) == MASK_SIGN;
            return res;
        }
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF X IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_x =   (sig_x.w[1] > 0x0001ed09bead87c0u64)
                || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_x.w[0] > 0x378d8e63ffffffffu64))
                || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF Y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_y = (sig_y.w[1] > 0x0001ed09bead87c0u64)
                || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_y.w[0] > 0x378d8e63ffffffffu64))
                || ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
    //    ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
    }
    if non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
    }
    // if both numbers are zero, neither is greater => return NOTGREATERTHAN
    if x_is_zero && y_is_zero {
        res = false;
        return res;
    }
    // is x is zero, it is greater if Y is negative
    else if x_is_zero {
        res = (y.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    // is y is zero, X is greater if it is positive
    else if y_is_zero {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ, x is greater if y is negative
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = (y.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    // if exponents are the same, then we have a simple comparison of the
    // significands
    if exp_y == exp_x {
        res = ((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    }
    // if both components are either bigger or smaller,
    // it is clear what needs to be done
    if sig_x.w[1] >= sig_y.w[1] && sig_x.w[0] >= sig_y.w[0] && exp_x > exp_y {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    if sig_x.w[1] <= sig_y.w[1] && sig_x.w[0] <= sig_y.w[0] && exp_x < exp_y {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }

    diff = exp_x - exp_y;

    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
            return res;
        } // difference cannot be greater than 10^33

        if diff > 19 { // 128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &BID_TEN2K128[(diff - 20) as usize]);

            // if postitive, return whichever significand is larger
            // (converse if negative)
            if sig_n_prime256.w[3] == 0
            && (sig_n_prime256.w[2] == 0)
             && sig_n_prime256.w[1] == sig_y.w[1]
            && (sig_n_prime256.w[0] == sig_y.w[0]) {
                res = false;
                return res;
            } // if equal, return 0
            {
                res = (((sig_n_prime256.w[3]  > 0)
                      || sig_n_prime256.w[2]  > 0)
                     || (sig_n_prime256.w[1]  > sig_y.w[1])
                     || (sig_n_prime256.w[1] == sig_y.w[1]
                      && sig_n_prime256.w[0] > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
                return res;
            }
        }
        // else { //128 by 64 bit multiply -> 192 bits
        sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_x);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            res = false;
            return res;
        } // if equal, return 0
        {
            res =  ((sig_n_prime192.w[2] > 0)
                 || (sig_n_prime192.w[1] > sig_y.w[1])
                 || (sig_n_prime192.w[1] == sig_y.w[1]
                  && sig_n_prime192.w[0] > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
            return res;
        }
    }

    diff = exp_y - exp_x;

    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }

    if diff > 19 { // 128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if sig_n_prime256.w[3] == 0
        && (sig_n_prime256.w[2] == 0)
         && sig_n_prime256.w[1] == sig_x.w[1]
        && (sig_n_prime256.w[0] == sig_x.w[0]) {
            res = false;
            return res;
        } // if equal, return 0
        {
            res = (sig_n_prime256.w[3] == 0
                 && sig_n_prime256.w[2] == 0
                && (sig_n_prime256.w[1]  < sig_x.w[1]
                || (sig_n_prime256.w[1] == sig_x.w[1]
                 && sig_n_prime256.w[0] < sig_x.w[0]))) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    //  adjust the y significand upwards
    sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_y);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0]) {
        res = false;
        return res;
    } // if equal, return 0
    {
        res = (sig_n_prime192.w[2] == 0
           && (sig_n_prime192.w[1]  < sig_x.w[1]
           || (sig_n_prime192.w[1] == sig_x.w[1]
            && sig_n_prime192.w[0]  < sig_x.w[0]))) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        res
    }
}

/// Compare 128-bit decimal floating-point numbers for specified relation;
/// do not signal invalid exception for quiet NaNs
pub (crate) fn bid128_quiet_less(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> bool {
    let res: bool;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let mut sig_n_prime192: BID_UINT192 = BID_UINT192::default();
    let mut sig_n_prime256: BID_UINT256 = BID_UINT256::default();
    let mut x_is_zero: bool = false;
    let mut y_is_zero: bool = false;
    let mut non_canon_x: bool = false;
    let mut non_canon_y: bool = false;

    // NaN (CASE1)
    // if either number is NAN, the comparison is unordered,
    // rather than equal : return 0
    if ((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN) {
        if (x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        }
        {
            res = false;
            return res;
        }
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal.
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = false;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x==neg_inf, { res = (y == neg_inf)?1:0; BID_RETURN_VAL (res) }
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN
        // x is -inf, so it is less than y unless y is -inf
        {
            res = ((y.w[1] & MASK_INF) != MASK_INF) || (y.w[1] & MASK_SIGN) != MASK_SIGN;
            res
        } else {    // x is pos_inf, no way for it to be less than y
            res = false;
            res
        }
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is positive infinity, then x is less, return 0
        //                 if y is negative infinity, then x is greater, return 1
        {
            res = (y.w[1] & MASK_SIGN) != MASK_SIGN;
            return res;
        }
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF X IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_x =   (sig_x.w[1]   > 0x0001ed09bead87c0u64)
                || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_x.w[0]  > 0x378d8e63ffffffffu64))
                || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF Y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_y = (sig_y.w[1]   > 0x0001ed09bead87c0u64)
                || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_y.w[0]  > 0x378d8e63ffffffffu64))
                || ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
    //    ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
    }
    if non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
    }
    // if both numbers are zero, neither is greater => return NOTGREATERTHAN
    if x_is_zero && y_is_zero {
        res = false;
        return res;
    }
    // is x is zero, it is greater if Y is negative
    else if x_is_zero {
        res = (y.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    // is y is zero, X is greater if it is positive
    else if y_is_zero {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ, x is greater if y is negative
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = (y.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    // if exponents are the same, then we have a simple comparison of the
    // significands
    if exp_y == exp_x {
        res = ((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    }
    // if both components are either bigger or smaller,
    // it is clear what needs to be done
    if (sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    if (sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }

    diff = exp_x - exp_y;

    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
            return res;
        } // difference cannot be greater than 10^33

        if diff > 19 { // 128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &BID_TEN2K128[(diff - 20) as usize]);

            // if postitive, return whichever significand is larger
            // (converse if negative)
            if sig_n_prime256.w[3] == 0
            && (sig_n_prime256.w[2] == 0)
             && sig_n_prime256.w[1] == sig_y.w[1]
            && (sig_n_prime256.w[0] == sig_y.w[0]) {
                res = false;
                return res;
            } // if equal, return 0
            {
                res = (((sig_n_prime256.w[3] > 0)
                       || sig_n_prime256.w[2] > 0)
                      || (sig_n_prime256.w[1] > sig_y.w[1])
                      || (sig_n_prime256.w[1] == sig_y.w[1]
                       && sig_n_prime256.w[0] > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) != MASK_SIGN);
                return res;
            }
        }
        // else { //128 by 64 bit multiply -> 192 bits
        sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_x);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            res = false;
            return res;
        } // if equal, return 0
        {
            res = ((sig_n_prime192.w[2] > 0)
                 || (sig_n_prime192.w[1] > sig_y.w[1])
                 || (sig_n_prime192.w[1] == sig_y.w[1]
                  && sig_n_prime192.w[0] > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) != MASK_SIGN);
            return res;
        }
    }

    diff = exp_y - exp_x;

    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }

    if diff > 19 { // 128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if sig_n_prime256.w[3] == 0
        && (sig_n_prime256.w[2] == 0)
         && sig_n_prime256.w[1] == sig_x.w[1]
        && (sig_n_prime256.w[0] == sig_x.w[0]) {
            res = false;
            return res;
        } // if equal, return 1
        {
            res = (sig_n_prime256.w[3] != 0
                 || sig_n_prime256.w[2] != 0
                || (sig_n_prime256.w[1]  > sig_x.w[1]
                || (sig_n_prime256.w[1] == sig_x.w[1]
                 && sig_n_prime256.w[0] > sig_x.w[0]))) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    //  adjust the y significand upwards
    sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_y);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0]) {
        res = false;
        return res;
    } // if equal, return 0
    {
        res = (sig_n_prime192.w[2] != 0
           || (sig_n_prime192.w[1]  > sig_x.w[1]
           || (sig_n_prime192.w[1] == sig_x.w[1]
            && sig_n_prime192.w[0]  > sig_x.w[0]))) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        res
    }
}

/// Compare 128-bit decimal floating-point numbers for specified relation;
/// do not signal invalid exception for quiet NaNs
pub (crate) fn bid128_quiet_less_equal(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> bool {
    let res: bool;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let mut sig_n_prime192: BID_UINT192 = BID_UINT192::default();
    let mut sig_n_prime256: BID_UINT256 = BID_UINT256::default();
    let mut x_is_zero: bool = false;
    let mut y_is_zero: bool = false;
    let mut non_canon_x: bool = false;
    let mut non_canon_y: bool = false;

    // NaN (CASE1)
    // if either number is NAN, the comparison is unordered,
    // rather than equal : return 0
    if ((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN) {
        if (x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        }
        {
            res = false;
            return res;
        }
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal (not Greater).
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = true;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x is neg infinity, there is no way it is greater than y, return 1
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN {
            res = true;
            res
        }
        // x is pos infinity, it is greater, unless y is positive infinity =>
        // return y!=pos_infinity
        else {
            res = ((y.w[1] & MASK_INF) == MASK_INF) && ((y.w[1] & MASK_SIGN) != MASK_SIGN);
            res
        }
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is positive infinity, then x is less, return 0
        //                 if y is negative infinity, then x is greater, return 1
        {
            res = (y.w[1] & MASK_SIGN) != MASK_SIGN;
            return res;
        }
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF X IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_x =   (sig_x.w[1]  > 0x0001ed09bead87c0u64)
                || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_x.w[0]  > 0x378d8e63ffffffffu64))
                || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF Y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_y =   (sig_y.w[1]  > 0x0001ed09bead87c0u64)
                || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_y.w[0] > 0x378d8e63ffffffffu64))
                || ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
    //    ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
    }
    if non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
    }
    // if both numbers are zero, neither is greater => return NOTGREATERTHAN
    if x_is_zero && y_is_zero {
        res = true;
        return res;
    }
    // is x is zero, it is greater if Y is negative
    else if x_is_zero {
        res = (y.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    // is y is zero, X is greater if it is positive
    else if y_is_zero {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ, x is greater if y is negative
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = (y.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    // if exponents are the same, then we have a simple comparison of the
    // significands
    if exp_y == exp_x {
        res = ((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    }
    // if both components are either bigger or smaller,
    // it is clear what needs to be done
    if (sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    if (sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }

    diff = exp_x - exp_y;

    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
            return res;
        } // difference cannot be greater than 10^33

        if diff > 19 { // 128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &BID_TEN2K128[(diff - 20) as usize]);

            // if postitive, return whichever significand is larger
            // (converse if negative)
            if sig_n_prime256.w[3] == 0
            && (sig_n_prime256.w[2] == 0)
             && sig_n_prime256.w[1] == sig_y.w[1]
            && (sig_n_prime256.w[0] == sig_y.w[0]) {
                res = true;
                return res;
            } // if equal, return 0
            {
                res = (((sig_n_prime256.w[3] > 0)
                       || sig_n_prime256.w[2] > 0)
                      || (sig_n_prime256.w[1] > sig_y.w[1])
                      || (sig_n_prime256.w[1] == sig_y.w[1]
                       && sig_n_prime256.w[0] > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) != MASK_SIGN);
                return res;
            }
        }
        // else { //128 by 64 bit multiply -> 192 bits
        sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_x);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            res = true;
            return res;
        } // if equal, return 0
        {
            res = ((sig_n_prime192.w[2]  > 0)
                 || (sig_n_prime192.w[1]  > sig_y.w[1])
                 || (sig_n_prime192.w[1] == sig_y.w[1]
                  && sig_n_prime192.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) != MASK_SIGN);
            return res;
        }
    }

    diff = exp_y - exp_x;

    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }

    if diff > 19 { // 128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if sig_n_prime256.w[3] == 0
        && (sig_n_prime256.w[2] == 0)
         && sig_n_prime256.w[1] == sig_x.w[1]
        && (sig_n_prime256.w[0] == sig_x.w[0]) {
            res = true;
            return res;
        } // if equal, return 0
        {
            res = (sig_n_prime256.w[3] != 0
                 || sig_n_prime256.w[2] != 0
                || (sig_n_prime256.w[1]  > sig_x.w[1]
                || (sig_n_prime256.w[1] == sig_x.w[1]
                 && sig_n_prime256.w[0] > sig_x.w[0]))) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    //  adjust the y significand upwards
    sig_n_prime192= __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_y);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0]) {
        res = true;
        return res;
    } // if equal, return 0
    {
        res = (sig_n_prime192.w[2] != 0
           || (sig_n_prime192.w[1]  > sig_x.w[1]
           || (sig_n_prime192.w[1] == sig_x.w[1]
            && sig_n_prime192.w[0] > sig_x.w[0]))) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        res
    }
}

/// Compare 128-bit decimal floating-point numbers for specified relation;
/// do not signal invalid exception for quiet NaNs
pub (crate) fn bid128_quiet_less_unordered(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> bool {
    let res: bool;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let mut sig_n_prime192: BID_UINT192 = BID_UINT192::default();
    let mut sig_n_prime256: BID_UINT256 = BID_UINT256::default();
    let mut x_is_zero: bool = false;
    let mut y_is_zero: bool = false;
    let mut non_canon_x: bool = false;
    let mut non_canon_y: bool = false;

    // NaN (CASE1)
    // if either number is NAN, the comparison is unordered
    if ((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN) {
        if (x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        }
        {
            res = true;
            return res;
        }
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal.
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = false;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x==neg_inf, { res = (y == neg_inf)?1:0; BID_RETURN_VAL (res) }
        return if (x.w[1] & MASK_SIGN) == MASK_SIGN
        // x is -inf, so it is less than y unless y is -inf
        {
            res = ((y.w[1] & MASK_INF) != MASK_INF) || (y.w[1] & MASK_SIGN) != MASK_SIGN;
            res
        } else {    // x is pos_inf, no way for it to be less than y
            res = false;
            res
        }
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is positive infinity, then x is less, return 0
        //                 if y is negative infinity, then x is greater, return 1
        {
            res = (y.w[1] & MASK_SIGN) != MASK_SIGN;
            return res;
        }
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF X IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_x =   (sig_x.w[1] > 0x0001ed09bead87c0u64)
                || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
                 && (sig_x.w[0] > 0x378d8e63ffffffffu64))
                || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF Y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_y =  (sig_y.w[1] > 0x0001ed09bead87c0u64)
               || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
                && (sig_y.w[0] > 0x378d8e63ffffffffu64))
               ||  ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
    //    ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
    }
    if non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
    }
    // if both numbers are zero, neither is greater => return NOTGREATERTHAN
    if x_is_zero && y_is_zero {
        res = false;
        return res;
    }
    // is x is zero, it is greater if Y is negative
    else if x_is_zero {
        res = (y.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    // is y is zero, X is greater if it is positive
    else if y_is_zero {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ, x is greater if y is negative
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = (y.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    // if exponents are the same, then we have a simple comparison
    // of the significands
    if exp_y == exp_x {
        res = ((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    }
    // if both components are either bigger or smaller,
    // it is clear what needs to be done
    if (sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y {
        res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
        return res;
    }
    if (sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }

    diff = exp_x - exp_y;

    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            res = (x.w[1] & MASK_SIGN) == MASK_SIGN;
            return res;
        } // difference cannot be greater than 10^33

        if diff > 19 { // 128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &BID_TEN2K128[(diff - 20) as usize]);

            // if postitive, return whichever significand is larger
            // (converse if negative)
            if  sig_n_prime256.w[3] == 0
            && (sig_n_prime256.w[2] == 0)
             && sig_n_prime256.w[1] == sig_y.w[1]
            && (sig_n_prime256.w[0] == sig_y.w[0]) {
                res = false;
                return res;
            } // if equal, return 0
            {
                res =  (((sig_n_prime256.w[3]  > 0)
                       || sig_n_prime256.w[2]  > 0)
                      || (sig_n_prime256.w[1]  > sig_y.w[1])
                      || (sig_n_prime256.w[1] == sig_y.w[1]
                       && sig_n_prime256.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) != MASK_SIGN);
                return res;
            }
        }
        // else { //128 by 64 bit multiply -> 192 bits
        sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_x);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            res = false;
            return res;
        } // if equal, return 0
        {
            res = ((sig_n_prime192.w[2]  > 0)
                 || (sig_n_prime192.w[1]  > sig_y.w[1])
                 || (sig_n_prime192.w[1] == sig_y.w[1]
                  && sig_n_prime192.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) != MASK_SIGN);
            return res;
        }
    }

    diff = exp_y - exp_x;

    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = (x.w[1] & MASK_SIGN) != MASK_SIGN;
        return res;
    }

    if diff > 19 { // 128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if sig_n_prime256.w[3] == 0
        && (sig_n_prime256.w[2] == 0)
         && sig_n_prime256.w[1] == sig_x.w[1]
        && (sig_n_prime256.w[0] == sig_x.w[0]) {
            res = false;
            return res;
        } // if equal, return 1
        {
            res = (sig_n_prime256.w[3] != 0
                 || sig_n_prime256.w[2] != 0
                || (sig_n_prime256.w[1]  > sig_x.w[1]
                || (sig_n_prime256.w[1] == sig_x.w[1]
                 && sig_n_prime256.w[0] > sig_x.w[0]))) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN);
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    //  adjust the y significand upwards
    sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[diff as usize], &sig_y);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0]) {
        res = false;
        return res;
    } // if equal, return 0
    {
        res = (sig_n_prime192.w[2] != 0
           || (sig_n_prime192.w[1]  > sig_x.w[1]
           || (sig_n_prime192.w[1] == sig_x.w[1]
            && sig_n_prime192.w[0] > sig_x.w[0]))) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        res
    }
}

/// Compare 128-bit decimal floating-point numbers for specified relation;
/// do not signal invalid exception for quiet NaNs
pub (crate) fn bid128_quiet_not_equal(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> bool {
    let res: bool;
    let mut exp_x: i32;
    let mut exp_y: i32;
    let mut exp_t: i32 = 0;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let mut sig_t: BID_UINT128 = BID_UINT128::default();
    let mut sig_n_prime192: BID_UINT192 = BID_UINT192::default();
    let mut sig_n_prime256: BID_UINT256 = BID_UINT256::default();
    let mut x_is_zero: bool = false;
    let mut y_is_zero: bool = false;
    let mut non_canon_x: bool = false;
    let mut non_canon_y: bool = false;

    // NaN (CASE1)
    // if either number is NAN, the comparison is unordered,
    // rather than equal : return 0
    if ((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN) {
        if (x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        }
        {
            res = true;
            return res;
        }
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equivalent.
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = false;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        return if (y.w[1] & MASK_INF) == MASK_INF {
            res = ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN;
            res
        } else {
            res = true;
            res
        }
    }
    if (y.w[1] & MASK_INF) == MASK_INF {
        res = true;
        return res;
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CHECK IF X IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_x = (sig_x.w[1]  > 0x0001ed09bead87c0u64)
               || ((sig_x.w[1] == 0x0001ed09bead87c0u64)
                && (sig_x.w[0] > 0x378d8e63ffffffffu64))
               || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // CHECK IF Y IS CANONICAL
    // 9999999999999999999999999999999999(decimal) =
    //   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
    // [0, 10^34) is the 754 supported canonical range.
    //    If the value exceeds that, it is interpreted as 0.
    non_canon_y = (sig_y.w[1]  > 0x0001ed09bead87c0u64)
               || ((sig_y.w[1] == 0x0001ed09bead87c0u64)
                && (sig_y.w[0] > 0x378d8e63ffffffffu64))
               || ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64);

    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
    //    ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0)) {
        x_is_zero = true;
    }
    if non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0)) {
        y_is_zero = true;
    }

    if x_is_zero && y_is_zero {
        res = false;
        return res;
    } else if (x_is_zero && !y_is_zero) || (!x_is_zero && y_is_zero) {
        res = true;
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ => not equal : return 0
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = true;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    if exp_x > exp_y {                                                      // to simplify the loop below,
        swap(&mut exp_x, &mut exp_y, &mut exp_t);                           // put the larger exp in y,
        swap(&mut sig_x.w[1], &mut sig_y.w[1], &mut sig_t.w[1]);    // and the smaller exp in x
        swap(&mut sig_x.w[0], &mut sig_y.w[0], &mut sig_t.w[0]);    // and the smaller exp in x
    }

    if exp_y - exp_x > 33 {
        res = true;
        return res;
    } // difference cannot be greater than 10^33

    if exp_y - exp_x > 19 {
        // recalculate y's significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &BID_TEN2K128[(exp_y - exp_x - 20) as usize]);
        {
            res = (sig_n_prime256.w[3] != 0)
                || (sig_n_prime256.w[2] != 0)
                || (sig_n_prime256.w[1] != sig_x.w[1])
                || (sig_n_prime256.w[0] != sig_x.w[0]);
            return res;
        }
    }
    // else{
    //  recalculate y's significand upwards
    sig_n_prime192 = __mul_64x128_to192(BID_TEN2K64[(exp_y - exp_x) as usize], &sig_y);
    {
        res = (sig_n_prime192.w[2] != 0) || (sig_n_prime192.w[1] != sig_x.w[1]) || (sig_n_prime192.w[0] != sig_x.w[0]);
        res
    }
}

/*
BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_quiet_not_greater, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered,
// rather than equal : return 0
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    if ((x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN) {
        *pfpsf |= BID_INVALID_EXCEPTION;
    }
    {
        res = 1;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal (not Greater).
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 1;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x is neg infinity, there is no way it is greater than y, return 1
    if (((x.w[1] & MASK_SIGN) == MASK_SIGN)) {
        res = 1;
        return res;
    }
    // x is pos infinity, it is greater, unless y is positive infinity => return y!=pos_infinity
    else {
        res = (((y.w[1] & MASK_INF) == MASK_INF) && ((y.w[1] & MASK_SIGN) != MASK_SIGN));
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//    If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//    If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 1;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)
// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if ((sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
if ((sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 1;
            return res;
        } // if equal, return 0
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) != MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 1;
        return res;
    } // if equal, return 0
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) != MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, no need for compensation
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 1;
        return res;
    } // if equal, return 0
    {
        res = ((sig_n_prime256.w[3] != 0 || sig_n_prime256.w[2] != 0 ||
                (sig_n_prime256.w[1] > sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] > sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger
// (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 1;
    return res;
} // if equal, return 0
{
    res = (sig_n_prime192.w[2] != 0 ||
           (sig_n_prime192.w[1] > sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] > sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_quiet_not_less, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered,
// rather than equal : return 1
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    if ((x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN) {
        *pfpsf |= BID_INVALID_EXCEPTION;
    }
    {
        res = 1;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal (not Greater).
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 1;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x==neg_inf, { res = (y == neg_inf)?1:0; BID_RETURN_VAL (res) }
    if ((x.w[1] & MASK_SIGN) == MASK_SIGN)
    // x is -inf, so it is less than y unless y is -inf
    {
        res = (((y.w[1] & MASK_INF) == MASK_INF) && (y.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    } else
    // x is pos_inf, no way for it to be less than y
    {
        res = 1;
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 1;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)

// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if (sig_x.w[1] >= sig_y.w[1] && sig_x.w[0] >= sig_y.w[0] && exp_x > exp_y) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
if (sig_x.w[1] <= sig_y.w[1] && sig_x.w[0] <= sig_y.w[0] && exp_x < exp_y) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 1;
            return res;
        } // if equal, return 1
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) == MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 1;
        return res;
    } // if equal, return 1
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, no need for compensation
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 1;
        return res;
    } // if equal, return 1
    {
        res = ((sig_n_prime256.w[3] == 0 && sig_n_prime256.w[2] == 0 &&
                (sig_n_prime256.w[1] < sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] < sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger
// (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 1;
    return res;
} // if equal, return 1
{
    res = (sig_n_prime192.w[2] == 0 &&
           (sig_n_prime192.w[1] < sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] < sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_quiet_ordered, x, y)

int res;

// NaN (CASE1)
// if either number is NAN, the comparison is ordered : return 1
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    if ((x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN) {
        *pfpsf |= BID_INVALID_EXCEPTION;
    }
    {
        res = 0;
        return res;
    }
}
{
    res = 1;
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_quiet_unordered, x, y)

int res;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered : return 1
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    if ((x.w[1] & MASK_SNAN) == MASK_SNAN || (y.w[1] & MASK_SNAN) == MASK_SNAN) {
        *pfpsf |= BID_INVALID_EXCEPTION;
    }
    {
        res = 1;
        return res;
    }
}
{
    res = 0;
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_signaling_greater, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered,
// rather than equal : return 0
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    *pfpsf |= BID_INVALID_EXCEPTION;
    {
        res = 0;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal (not Greater).
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 0;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x is neg infinity, there is no way it is greater than y, return 0
    if (((x.w[1] & MASK_SIGN) == MASK_SIGN)) {
        res = 0;
        return res;
    }
    // x is pos infinity, it is greater, unless y is positive infinity => return y!=pos_infinity
    else {
        res = (((y.w[1] & MASK_INF) != MASK_INF) || ((y.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 0;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)
// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if ((sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y) {
    {
        res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    }
}
if ((sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y) {
    {
        res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    }
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 0;
            return res;
        } // if equal, return 0
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) == MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to_192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 0;
        return res;
    } // if equal, return 0
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, no need for compensation
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 0;
        return res;
    } // if equal, return 0
    {
        res = ((sig_n_prime256.w[3] != 0 || sig_n_prime256.w[2] != 0 ||
                (sig_n_prime256.w[1] > sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] > sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) != MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to_192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger
// (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 0;
    return res;
} // if equal, return 0
{
    res = (sig_n_prime192.w[2] != 0 ||
           (sig_n_prime192.w[1] > sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] > sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_signaling_greater_equal, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered,
// rather than equal : return 1
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    *pfpsf |= BID_INVALID_EXCEPTION;
    {
        res = 0;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal (not Greater).
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 1;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x==neg_inf, { res = (y == neg_inf)?1:0; BID_RETURN_VAL (res) }
    if ((x.w[1] & MASK_SIGN) == MASK_SIGN)
    // x is -inf, so it is less than y unless y is -inf
    {
        res = (((y.w[1] & MASK_INF) == MASK_INF) && (y.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    } else
    // x is pos_inf, no way for it to be less than y
    {
        res = 1;
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 1;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)
// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if (sig_x.w[1] >= sig_y.w[1] && sig_x.w[0] >= sig_y.w[0] && exp_x > exp_y) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
if (sig_x.w[1] <= sig_y.w[1] && sig_x.w[0] <= sig_y.w[0] && exp_x < exp_y) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 1;
            return res;
        } // if equal, return 1
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) == MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 1;
        return res;
    } // if equal, return 1
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, no need for compensation
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 1;
        return res;
    } // if equal, return 1
    {
        res = ((sig_n_prime256.w[3] == 0 && sig_n_prime256.w[2] == 0 &&
                (sig_n_prime256.w[1] < sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] < sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger
// (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 1;
    return res;
} // if equal, return 1
{
    res = (sig_n_prime192.w[2] == 0 &&
           (sig_n_prime192.w[1] < sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] < sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_signaling_greater_unordered, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered,
// rather than equal : return 1
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    *pfpsf |= BID_INVALID_EXCEPTION;
    {
        res = 1;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal (not Greater).
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 0;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x is neg infinity, there is no way it is greater than y, return 0
    if (((x.w[1] & MASK_SIGN) == MASK_SIGN)) {
        res = 0;
        return res;
    }
    // x is pos infinity, it is greater, unless y is positive infinity => return y!=pos_infinity
    else {
        res = (((y.w[1] & MASK_INF) != MASK_INF) || ((y.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 0;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)
// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if (sig_x.w[1] >= sig_y.w[1] && sig_x.w[0] >= sig_y.w[0] && exp_x > exp_y) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
if (sig_x.w[1] <= sig_y.w[1] && sig_x.w[0] <= sig_y.w[0] && exp_x < exp_y) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 0;
            return res;
        } // if equal, return 0
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) == MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 0;
        return res;
    } // if equal, return 0
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, no need for compensation
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 0;
        return res;
    } // if equal, return 0
    {
        res = ((sig_n_prime256.w[3] == 0 && sig_n_prime256.w[2] == 0 &&
                (sig_n_prime256.w[1] < sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] < sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger
// (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 0;
    return res;
} // if equal, return 0
{
    res = (sig_n_prime192.w[2] == 0 &&
           (sig_n_prime192.w[1] < sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] < sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_signaling_less, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered,
// rather than equal : return 0
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    *pfpsf |= BID_INVALID_EXCEPTION;
    {
        res = 0;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal.
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 0;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x==neg_inf, { res = (y == neg_inf)?1:0; BID_RETURN_VAL (res) }
    if ((x.w[1] & MASK_SIGN) == MASK_SIGN)
    // x is -inf, so it is less than y unless y is -inf
    {
        res = (((y.w[1] & MASK_INF) != MASK_INF) || (y.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    } else
    // x is pos_inf, no way for it to be less than y
    {
        res = 0;
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 0;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)
// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if ((sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
if ((sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 0;
            return res;
        } // if equal, return 0
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) != MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 0;
        return res;
    } // if equal, return 0
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) != MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, |x| < |y|, return 1 if positive
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 0;
        return res;
    } // if equal, return 1
    {
        res = ((sig_n_prime256.w[3] != 0 || sig_n_prime256.w[2] != 0 ||
                (sig_n_prime256.w[1] > sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] > sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger
// (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 0;
    return res;
} // if equal, return 0
{
    res = (sig_n_prime192.w[2] != 0 ||
           (sig_n_prime192.w[1] > sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] > sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_signaling_less_equal, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered,
// rather than equal : return 0
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    *pfpsf |= BID_INVALID_EXCEPTION;
    {
        res = 0;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal (not Greater).
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 1;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x is neg infinity, there is no way it is greater than y, return 1
    if (((x.w[1] & MASK_SIGN) == MASK_SIGN)) {
        res = 1;
        return res;
    }
    // x is pos infinity, it is greater, unless y is positive infinity => return y!=pos_infinity
    else {
        res = (((y.w[1] & MASK_INF) == MASK_INF) && ((y.w[1] & MASK_SIGN) != MASK_SIGN));
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 1;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)
// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if ((sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
if ((sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 1;
            return res;
        } // if equal, return 0
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) != MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 1;
        return res;
    } // if equal, return 0
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) != MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, no need for compensation
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 1;
        return res;
    } // if equal, return 0
    {
        res = ((sig_n_prime256.w[3] != 0 || sig_n_prime256.w[2] != 0 ||
                (sig_n_prime256.w[1] > sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] > sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger
// (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 1;
    return res;
} // if equal, return 0
{
    res = (sig_n_prime192.w[2] != 0 ||
           (sig_n_prime192.w[1] > sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] > sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_signaling_less_unordered, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    *pfpsf |= BID_INVALID_EXCEPTION;
    {
        res = 1;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal.
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 0;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x==neg_inf, { res = (y == neg_inf)?1:0; BID_RETURN_VAL (res) }
    if ((x.w[1] & MASK_SIGN) == MASK_SIGN)
    // x is -inf, so it is less than y unless y is -inf
    {
        res = (((y.w[1] & MASK_INF) != MASK_INF) || (y.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    } else
    // x is pos_inf, no way for it to be less than y
    {
        res = 0;
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 0;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)
// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if ((sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
if ((sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 0;
            return res;
        } // if equal, return 0
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) != MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 0;
        return res;
    } // if equal, return 0
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) != MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, no need for compensation
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 0;
        return res;
    } // if equal, return 1
    {
        res = ((sig_n_prime256.w[3] != 0 || sig_n_prime256.w[2] != 0 ||
                (sig_n_prime256.w[1] > sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] > sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 0;
    return res;
} // if equal, return 0
{
    res = (sig_n_prime192.w[2] != 0 ||
           (sig_n_prime192.w[1] > sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] > sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_signaling_not_greater, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered,
// rather than equal : return 0
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    *pfpsf |= BID_INVALID_EXCEPTION;
    {
        res = 1;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal (not Greater).
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 1;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x is neg infinity, there is no way it is greater than y, return 1
    if (((x.w[1] & MASK_SIGN) == MASK_SIGN)) {
        res = 1;
        return res;
    }
    // x is pos infinity, it is greater, unless y is positive infinity => return y!=pos_infinity
    else {
        res = (((y.w[1] & MASK_INF) == MASK_INF) && ((y.w[1] & MASK_SIGN) != MASK_SIGN));
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 1;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)
// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if ((sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
if ((sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 1;
            return res;
        } // if equal, return 0
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) != MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 1;
        return res;
    } // if equal, return 0
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) != MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, no need for compensation
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 1;
        return res;
    } // if equal, return 0
    {
        res = ((sig_n_prime256.w[3] != 0 || sig_n_prime256.w[2] != 0 ||
                (sig_n_prime256.w[1] > sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] > sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger
// (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 1;
    return res;
} // if equal, return 0
{
    res = (sig_n_prime192.w[2] != 0 ||
           (sig_n_prime192.w[1] > sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] > sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
}

BID128_FUNCTION_ARG2_NORND_CUSTOMRESTYPE(int, bid128_signaling_not_less, x, y)

int res;
int exp_x, exp_y;
int diff;
BID_UINT128 sig_x, sig_y;
BID_UINT192 sig_n_prime192;
BID_UINT256 sig_n_prime256;
char x_is_zero = 0, y_is_zero = 0, non_canon_x, non_canon_y;

// NaN (CASE1)
// if either number is NAN, the comparison is unordered,
// rather than equal : return 1
if (((x.w[1] & MASK_NAN) == MASK_NAN) || ((y.w[1] & MASK_NAN) == MASK_NAN)) {
    *pfpsf |= BID_INVALID_EXCEPTION;
    {
        res = 1;
        return res;
    }
}
// SIMPLE (CASE2)
// if all the bits are the same, these numbers are equal (not Greater).
if (x.w[0] == y.w[0] && x.w[1] == y.w[1]) {
    res = 1;
    return res;
}
// INFINITY (CASE3)
if ((x.w[1] & MASK_INF) == MASK_INF) {
    // if x==neg_inf, { res = (y == neg_inf)?1:0; BID_RETURN_VAL (res) }
    if ((x.w[1] & MASK_SIGN) == MASK_SIGN)
    // x is -inf, so it is less than y unless y is -inf
    {
        res = (((y.w[1] & MASK_INF) == MASK_INF) && (y.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    } else
    // x is pos_inf, no way for it to be less than y
    {
        res = 1;
        return res;
    }
} else if ((y.w[1] & MASK_INF) == MASK_INF) {
    // x is finite, so if y is positive infinity, then x is less, return 0
    //                 if y is negative infinity, then x is greater, return 1
    {
        res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
        return res;
    }
}
// CONVERT X
sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
sig_x.w[0] = x.w[0];
exp_x = (x.w[1] >> 49) & 0x000000000003fffu64;

// CHECK IF X IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_x.w[1] > 0x0001ed09bead87c0u64) || ((sig_x.w[1] == 0x0001ed09bead87c0u64) && (sig_x.w[0] > 0x378d8e63ffffffffu64)) ||
    ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_x = 1;
else
    non_canon_x = 0;

// CONVERT Y
exp_y = (y.w[1] >> 49) & 0x0000000000003fffu64;
sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
sig_y.w[0] = y.w[0];

// CHECK IF Y IS CANONICAL
// 9999999999999999999999999999999999(decimal) =
//   1ed09_bead87c0_378d8e63_ffffffff(hexadecimal)
// [0, 10^34) is the 754 supported canonical range.
//     If the value exceeds that, it is interpreted as 0.
if ((sig_y.w[1] > 0x0001ed09bead87c0u64) || ((sig_y.w[1] == 0x0001ed09bead87c0u64) && (sig_y.w[0] > 0x378d8e63ffffffffu64)) ||
    ((y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64))
    non_canon_y = 1;
else
    non_canon_y = 0;

// ZERO (CASE4)
// some properties:
//    (+ZERO == -ZERO) => therefore ignore the sign
//    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => therefore
//    ignore the exponent field
//    (Any non-canonical # is considered 0)
if (non_canon_x || ((sig_x.w[1] == 0) && (sig_x.w[0] == 0))) {
    x_is_zero = 1;
}
if (non_canon_y || ((sig_y.w[1] == 0) && (sig_y.w[0] == 0))) {
    y_is_zero = 1;
}
// if both numbers are zero, neither is greater => return NOTGREATERTHAN
if (x_is_zero && y_is_zero) {
    res = 1;
    return res;
}
// is x is zero, it is greater if Y is negative
else if (x_is_zero) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// is y is zero, X is greater if it is positive
else if (y_is_zero) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
// OPPOSITE SIGN (CASE5)
// now, if the sign bits differ, x is greater if y is negative
if (((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN) {
    res = ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
// REDUNDANT REPRESENTATIONS (CASE6)

// if exponents are the same, then we have a simple comparison
// of the significands
if (exp_y == exp_x) {
    res = (((sig_x.w[1] > sig_y.w[1]) || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN));
    return res;
}
// if both components are either bigger or smaller,
// it is clear what needs to be done
if (sig_x.w[1] >= sig_y.w[1] && sig_x.w[0] >= sig_y.w[0] && exp_x > exp_y) {
    res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
    return res;
}
if (sig_x.w[1] <= sig_y.w[1] && sig_x.w[0] <= sig_y.w[0] && exp_x < exp_y) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}

diff = exp_x - exp_y;

// if |exp_x - exp_y| < 33, it comes down to the compensated significand
if (diff > 0) { // to simplify the loop below,

    // if exp_x is 33 greater than exp_y, no need for compensation
    if (diff > 33) {
        res = ((x.w[1] & MASK_SIGN) != MASK_SIGN);
        return res;
    } // difference cannot be greater than 10^33

    if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
        __mul_128x128_to_256(sig_n_prime256, sig_x, BID_TEN2K128[(diff - 20) as usize]);

        // if postitive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_y.w[1] &&
            (sig_n_prime256.w[0] == sig_y.w[0])) {
            res = 1;
            return res;
        } // if equal, return 1
        {
            res = ((((sig_n_prime256.w[3] > 0) || sig_n_prime256.w[2] > 0) || (sig_n_prime256.w[1] > sig_y.w[1]) ||
                    (sig_n_prime256.w[1] == sig_y.w[1] && sig_n_prime256.w[0] > sig_y.w[0])) ^
                   ((y.w[1] & MASK_SIGN) == MASK_SIGN));
            return res;
        }
    }
    // else { //128 by 64 bit multiply -> 192 bits
    __mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_x);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0])) {
        res = 1;
        return res;
    } // if equal, return 1
    {
        res = (((sig_n_prime192.w[2] > 0) || (sig_n_prime192.w[1] > sig_y.w[1]) ||
                (sig_n_prime192.w[1] == sig_y.w[1] && sig_n_prime192.w[0] > sig_y.w[0])) ^
               ((y.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}

diff = exp_y - exp_x;

// if exp_x is 33 less than exp_y, no need for compensation
if (diff > 33) {
    res = ((x.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}

if (diff > 19) { // 128 by 128 bit multiply -> 256 bits
    // adjust the y significand upwards
    __mul_128x128_to_256(sig_n_prime256, sig_y, BID_TEN2K128[(diff - 20) as usize]);

    // if postitive, return whichever significand is larger
    // (converse if negative)
    if (sig_n_prime256.w[3] == 0 && (sig_n_prime256.w[2] == 0) && sig_n_prime256.w[1] == sig_x.w[1] &&
        (sig_n_prime256.w[0] == sig_x.w[0])) {
        res = 1;
        return res;
    } // if equal, return 1
    {
        res = ((sig_n_prime256.w[3] == 0 && sig_n_prime256.w[2] == 0 &&
                (sig_n_prime256.w[1] < sig_x.w[1] || (sig_n_prime256.w[1] == sig_x.w[1] && sig_n_prime256.w[0] < sig_x.w[0]))) ^
               ((x.w[1] & MASK_SIGN) == MASK_SIGN));
        return res;
    }
}
// else { //128 by 64 bit multiply -> 192 bits
//  adjust the y significand upwards
__mul_64x128_to192(sig_n_prime192, BID_TEN2K64[diff as usize], sig_y);

// if postitive, return whichever significand is larger (converse if negative)
if ((sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0])) {
    res = 1;
    return res;
} // if equal, return 1
{
    res = (sig_n_prime192.w[2] == 0 &&
           (sig_n_prime192.w[1] < sig_x.w[1] || (sig_n_prime192.w[1] == sig_x.w[1] && sig_n_prime192.w[0] < sig_x.w[0]))) ^
          ((y.w[1] & MASK_SIGN) == MASK_SIGN);
    return res;
}
}
*/