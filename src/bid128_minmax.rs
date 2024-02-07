/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid128::{bid_ten2k128, bid_ten2k64};
use crate::bid_internal::{__mul_128x128_to_256, __mul_64x128_to_192};
use crate::constants::{MASK_ANY_INF, MASK_COEFF, MASK_EXP, MASK_INF, MASK_NAN, MASK_SIGN, MASK_SNAN, MASK_STEERING_BITS};
use crate::core::StatusFlags;
use crate::d128::{_IDEC_flags, BID_UINT128, BID_UINT192, BID_UINT256};

/// Returns the canonicalized floating-point number x if x < y,
/// y if y < x, the canonicalized floating-point number if one operand is
/// a floating-point number and the other a quiet NaN. Otherwise it is either x or y, canonicalized.
pub (crate) fn bid128_minnum(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let res: BID_UINT128;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let sig_n_prime192: BID_UINT192;
    let sig_n_prime256: BID_UINT256;
    let mut x_is_zero: bool = false;
    let mut y_is_zero: bool = false;
    let mut x: BID_UINT128 = *x;
    let mut y: BID_UINT128 = *y;

    //BID_SWAP128 (x);
    //BID_SWAP128 (y);

    // check for non-canonical x
    if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
        x.w[1] &= 0xfe003fffffffffffu64; // clear out G[6]-G[16]
        // check for non-canonical NaN payload
        if  ((x.w[1] & 0x00003fffffffffffu64)  > 0x0000314dc6448d93u64)
        || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (x.w[0] > 0x38c15b09ffffffffu64)) {
            x.w[1] &= 0xffffc00000000000u64;
            x.w[0]  = 0x0u64;
        }
    } else if (x.w[1] & MASK_ANY_INF) == MASK_INF { // x = inf
        x.w[1] &= MASK_SIGN | MASK_INF;
        x.w[0] = 0x0u64;
    } else { // x is not special
        // check for non-canonical values - treated as zero
        if (x.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS { // G0_G1=11
            // non-canonical
            x.w[1] = (x.w[1] & MASK_SIGN) | ((x.w[1] << 2) & MASK_EXP);
            x.w[0] = 0x0u64;
        } else { // G0_G1 != 11
            if  (x.w[1] & MASK_COEFF)  > 0x0001ed09bead87c0u64
            || ((x.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
              && x.w[0] > 0x378d8e63ffffffffu64) {
                // x is non-canonical if coefficient is larger than 10^34 -1
                x.w[1] = (x.w[1] & MASK_SIGN) | (x.w[1] & MASK_EXP);
                x.w[0] = 0x0u64;
            } else {
                // canonical
            }
        }
    }
    // check for non-canonical y
    if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
        y.w[1] &= 0xfe003fffffffffffu64; // clear out G[6]-G[16]
        // check for non-canonical NaN payload
        if  ((y.w[1] & 0x00003fffffffffffu64)  > 0x0000314dc6448d93u64)
        || (((y.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (y.w[0] > 0x38c15b09ffffffffu64)) {
            y.w[1] &= 0xffffc00000000000u64;
            y.w[0]  = 0x0u64;
        }
    } else if (y.w[1] & MASK_ANY_INF) == MASK_INF { // y = inf
        y.w[1] &= MASK_SIGN | MASK_INF;
        y.w[0] = 0x0u64;
    } else { // y is not special
        // check for non-canonical values - treated as zero
        if (y.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS { // G0_G1=11
            // non-canonical
            y.w[1] = (y.w[1] & MASK_SIGN) | ((y.w[1] << 2) & MASK_EXP);
            y.w[0] = 0x0u64;
        } else { // G0_G1 != 11
            if  (y.w[1] & MASK_COEFF)  > 0x0001ed09bead87c0u64
            || ((y.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
              && y.w[0] > 0x378d8e63ffffffffu64) {
                // y is non-canonical if coefficient is larger than 10^34 -1
                y.w[1] = (y.w[1] & MASK_SIGN) | (y.w[1] & MASK_EXP);
                y.w[0] = 0x0u64;
            } else {
                // canonical
            }
        }
    }

    // NaN (CASE1)
    if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
        res = if (x.w[1] & MASK_SNAN) == MASK_SNAN { // x is SNaN
            // if x is SNAN, then return quiet (x)
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set exception if SNaN
            x.w[1] &= 0xfdffffffffffffffu64; // quietize x
            x
        } else { // x is QNaN
            if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
                if (y.w[1] & MASK_SNAN) == MASK_SNAN { // y is SNAN
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set invalid flag
                }
                x
            } else {
                y
            }
        };
        return res;
    } else if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NaN, but x is not
        res = if (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set exception if SNaN
            y.w[1] &= 0xfdffffffffffffffu64; // quietize y
            y
        } else {
            // will return x (which is not NaN)
            x
        };
        return res;
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal (not Greater).
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = x;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x is neg infinity, there is no way it is greater than y, return 0
        res = if (x.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y };
        return res;
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is positive infinity, then x is less, return 0
        //                 if y is negative infinity, then x is greater, return 1
        res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x };
        return res;
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B => ignore the exponent
    //    field
    //    (Any non-canonical # is considered 0)
    if (sig_x.w[1] == 0) && (sig_x.w[0] == 0) {
        x_is_zero = true;
    }
    if (sig_y.w[1] == 0) && (sig_y.w[0] == 0) {
        y_is_zero = true;
    }

    if x_is_zero && y_is_zero {
        // if both numbers are zero, neither is greater => return either number
        res = x;
        return res;
    } else if x_is_zero {
        // is x is zero, it is greater if Y is negative
        res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x };
        return res;
    } else if y_is_zero {
        // is y is zero, X is greater if it is positive
        res = if (x.w[1] & MASK_SIGN) != MASK_SIGN { y } else { x };
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ, x is greater if y is negative
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x };
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    // if exponents are the same, then we have a simple comparison of
    //    the significands
    if exp_y == exp_x {
        res = if ((sig_x.w[1] > sig_y.w[1])
               || (sig_x.w[1] == sig_y.w[1]
                && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN) { y } else { x };
        return res;
    }
    // if both components are either bigger or smaller, it is clear what
    //    needs to be done
    if sig_x.w[1] >= sig_y.w[1] && sig_x.w[0] >= sig_y.w[0] &&
        exp_x > exp_y {
        res = if (x.w[1] & MASK_SIGN) != MASK_SIGN { y } else { x };
        return res;
    }
    if sig_x.w[1] <= sig_y.w[1] && sig_x.w[0] <= sig_y.w[0] &&
        exp_x < exp_y {
        res = if (x.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x };
        return res;
    }

    diff = exp_x - exp_y;

    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            // difference cannot be greater than 10^33
            res = if (x.w[1] & MASK_SIGN) != MASK_SIGN { y } else { x };
            return res;
        }
        if diff > 19 { //128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &bid_ten2k128[(diff - 20) as usize]);
            // if postitive, return whichever significand is larger
            // (converse if negative)
            res = if (((sig_n_prime256.w[3]  > 0)
                     || sig_n_prime256.w[2]  > 0)
                    || (sig_n_prime256.w[1]  > sig_y.w[1])
                    || (sig_n_prime256.w[1] == sig_y.w[1]
                     && sig_n_prime256.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN) { y } else { x };
            return res;
        }
        sig_n_prime192 = __mul_64x128_to_192(bid_ten2k64[diff as usize], &sig_x);
        // if postitive, return whichever significand is larger
        // (converse if negative)
        res = if ((sig_n_prime192.w[2]  > 0)
               || (sig_n_prime192.w[1]  > sig_y.w[1])
               || (sig_n_prime192.w[1] == sig_y.w[1]
                && sig_n_prime192.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN) { y } else { x };
        return res;
    }
    diff = exp_y - exp_x;
    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = if (x.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x };
        return res;
    }
    if diff > 19 { //128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &bid_ten2k128[(diff - 20) as usize]);
        // if postitive, return whichever significand is larger
        // (converse if negative)
        res = if (sig_n_prime256.w[3] != 0
               || sig_n_prime256.w[2] != 0
              || (sig_n_prime256.w[1]  > sig_x.w[1]
              || (sig_n_prime256.w[1] == sig_x.w[1]
               && sig_n_prime256.w[0]  > sig_x.w[0]))) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN) { x } else { y };
        return res;
    }
    // adjust the y significand upwards
    sig_n_prime192 = __mul_64x128_to_192(bid_ten2k64[diff as usize], &sig_y);
    // if postitive, return whichever significand is larger (converse if negative)
    res = if (sig_n_prime192.w[2] != 0
           || (sig_n_prime192.w[1] > sig_x.w[1]
           || (sig_n_prime192.w[1] == sig_x.w[1]
            && sig_n_prime192.w[0] > sig_x.w[0]))) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN) { x } else { y };
    res
}

/// Returns the canonicalized floating-point number x if |x| < |y|,
/// y if |y| < |x|, otherwise this function is identical to __bid64_minnum
pub (crate) fn bid128_minnum_mag(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let res: BID_UINT128;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let sig_n_prime192: BID_UINT192;
    let sig_n_prime256: BID_UINT256;
    let mut x: BID_UINT128 = *x;
    let mut y: BID_UINT128 = *y;

    //BID_SWAP128 (x);
    //BID_SWAP128 (y);

    // check for non-canonical x
    if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
        x.w[1] &= 0xfe003fffffffffffu64; // clear out G[6]-G[16]
        // check for non-canonical NaN payload
        if  ((x.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
        || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (x.w[0] > 0x38c15b09ffffffffu64)) {
            x.w[1] &= 0xffffc00000000000u64;
            x.w[0]  = 0x0u64;
        }
    } else if (x.w[1] & MASK_ANY_INF) == MASK_INF { // x = inf
        x.w[1] &= MASK_SIGN | MASK_INF;
        x.w[0]  = 0x0u64;
    } else { // x is not special
        // check for non-canonical values - treated as zero
        if (x.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS { // G0_G1=11
            // non-canonical
            x.w[1] = (x.w[1] & MASK_SIGN) | ((x.w[1] << 2) & MASK_EXP);
            x.w[0] = 0x0u64;
        } else { // G0_G1 != 11
            if  (x.w[1] & MASK_COEFF) > 0x0001ed09bead87c0u64
            || ((x.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
              && x.w[0] > 0x378d8e63ffffffffu64) {
                // x is non-canonical if coefficient is larger than 10^34 -1
                x.w[1] = (x.w[1] & MASK_SIGN) | (x.w[1] & MASK_EXP);
                x.w[0] = 0x0u64;
            } else {
                // canonical
            }
        }
    }
    // check for non-canonical y
    if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
        y.w[1] &= 0xfe003fffffffffffu64; // clear out G[6]-G[16]
        // check for non-canonical NaN payload
        if  ((y.w[1] & 0x00003fffffffffffu64)  > 0x0000314dc6448d93u64)
        || (((y.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (y.w[0] > 0x38c15b09ffffffffu64)) {
            y.w[1] &= 0xffffc00000000000u64;
            y.w[0]  = 0x0u64;
        }
    } else if (y.w[1] & MASK_ANY_INF) == MASK_INF { // y = inf
        y.w[1] &= MASK_SIGN | MASK_INF;
        y.w[0]  = 0x0u64;
    } else { // y is not special
        // check for non-canonical values - treated as zero
        if (y.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS { // G0_G1=11
            // non-canonical
            y.w[1] = (y.w[1] & MASK_SIGN) | ((y.w[1] << 2) & MASK_EXP);
            y.w[0] = 0x0u64;
        } else { // G0_G1 != 11
            if  (y.w[1] & MASK_COEFF)  > 0x0001ed09bead87c0u64
            || ((y.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
             && y.w[0] > 0x378d8e63ffffffffu64) {
                // y is non-canonical if coefficient is larger than 10^34 -1
                y.w[1] = (y.w[1] & MASK_SIGN) | (y.w[1] & MASK_EXP);
                y.w[0] = 0x0u64;
            } else {
                // canonical
            }
        }
    }

    // NaN (CASE1)
    if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
        res = if (x.w[1] & MASK_SNAN) == MASK_SNAN { // x is SNaN
            // if x is SNAN, then return quiet (x)
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set exception if SNaN
            x.w[1] &= 0xfdffffffffffffffu64; // quietize x
            x
        } else { // x is QNaN
            if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
                if (y.w[1] & MASK_SNAN) == MASK_SNAN { // y is SNAN
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set invalid flag
                }
                x
            } else {
                y
            }
        };
        return res;
    } else if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NaN, but x is not
        res = if (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set exception if SNaN
            y.w[1] &= 0xfdffffffffffffffu64; // quietize y
            y
        } else {
            // will return x (which is not NaN)
            x
        };
        return res;
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal (not Greater).
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = y;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x infinity, it has maximum magnitude.
        // Check if magnitudes are equal.  If x is negative, return it.
        res = if (x.w[1] & MASK_SIGN) == MASK_SIGN && (y.w[1] & MASK_INF) == MASK_INF { x } else { y };
        return res;
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is infinity, then x is less in magnitude
        res = x;
        return res;
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B =>
    //        therefore ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if (sig_x.w[1] == 0) && (sig_x.w[0] == 0) {
        res = x;
        return res;
    }
    if (sig_y.w[1] == 0) && (sig_y.w[0] == 0) {
        res = y;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    // check if exponents are the same and significands are the same
    if exp_y == exp_x && sig_x.w[1] == sig_y.w[1] &&
        sig_x.w[0] == sig_y.w[0] {
        res = if (x.w[1] & 0x8000000000000000u64) == 0x8000000000000000u64 { // x is negative
            x
        } else {
            y
        };
        return res;
    } else if ((sig_x.w[1]  > sig_y.w[1]
            || (sig_x.w[1] == sig_y.w[1]
             && sig_x.w[0]  > sig_y.w[0])) && exp_x == exp_y)
           || ((sig_x.w[1]  > sig_y.w[1]
            || (sig_x.w[1] == sig_y.w[1]
             && sig_x.w[0] >= sig_y.w[0])) && exp_x > exp_y) {
        // if both components are either bigger or smaller, it is clear what
        // needs to be done; also if the magnitudes are equal
        res = y;
        return res;
    } else if ((sig_y.w[1]  > sig_x.w[1]
            || (sig_y.w[1] == sig_x.w[1]
             && sig_y.w[0]  > sig_x.w[0])) && exp_y == exp_x)
           || ((sig_y.w[1]  > sig_x.w[1]
            || (sig_y.w[1] == sig_x.w[1]
             && sig_y.w[0] >= sig_x.w[0])) && exp_y > exp_x) {
        res = x;
        return res;
    } else {
        // continue
    }
    diff = exp_x - exp_y;
    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            res = y; // difference cannot be greater than 10^33
            return res;
        }
        if diff > 19 { //128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &bid_ten2k128[(diff - 20) as usize]);
            // if positive, return whichever significand is larger
            // (converse if negative)
            if  sig_n_prime256.w[3] == 0
            && (sig_n_prime256.w[2] == 0)
             && sig_n_prime256.w[1] == sig_y.w[1]
            && (sig_n_prime256.w[0] == sig_y.w[0]) {
                res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x }; // if equal
                return res;
            }
            res = if ((sig_n_prime256.w[3] > 0)
                    || sig_n_prime256.w[2] > 0)
                   || (sig_n_prime256.w[1] > sig_y.w[1])
                   || (sig_n_prime256.w[1] == sig_y.w[1]
                    && sig_n_prime256.w[0] > sig_y.w[0]) { y } else { x };
            return res;
        }
        sig_n_prime192 = __mul_64x128_to_192(bid_ten2k64[diff as usize], &sig_x);
        // if positive, return whichever significand is larger
        // (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            // if = in magnitude, return +, (if possible)
            res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x };
            return res;
        }
        res = if (sig_n_prime192.w[2]  > 0)
              || (sig_n_prime192.w[1]  > sig_y.w[1])
              || (sig_n_prime192.w[1] == sig_y.w[1]
               && sig_n_prime192.w[0]  > sig_y.w[0]) { y } else { x };
        return res;
    }
    diff = exp_y - exp_x;
    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = x;
        return res;
    }
    if diff > 19 { //128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &bid_ten2k128[(diff - 20) as usize]);
        // if positive, return whichever significand is larger
        // (converse if negative)
        if  sig_n_prime256.w[3] == 0
        && (sig_n_prime256.w[2] == 0)
         && sig_n_prime256.w[1] == sig_x.w[1]
        && (sig_n_prime256.w[0] == sig_x.w[0]) {
            // if = in magnitude, return +, (if possible)
            res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x };
            return res;
        }
        res = if sig_n_prime256.w[3] == 0
              && sig_n_prime256.w[2] == 0
             && (sig_n_prime256.w[1] < sig_x.w[1]
             || (sig_n_prime256.w[1] == sig_x.w[1]
              && sig_n_prime256.w[0] < sig_x.w[0])) { y } else { x };
        return res;
    }
    // adjust the y significand upwards
    sig_n_prime192 = __mul_64x128_to_192(bid_ten2k64[diff as usize], &sig_y);
    // if positive, return whichever significand is larger (converse if negative)
    if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0]) {
        // if = in magnitude, return +, if possible)
        res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x };
        return res;
    }
    res = if sig_n_prime192.w[2] == 0
         && (sig_n_prime192.w[1] < sig_x.w[1]
         || (sig_n_prime192.w[1] == sig_x.w[1]
          && sig_n_prime192.w[0] < sig_x.w[0])) { y } else { x };
    res
}

/// Returns the canonicalized floating-point number y if x < y,
/// x if y < x, the canonicalized floating-point number if one operand is a
/// floating-point number and the other a quiet NaN.
pub (crate) fn bid128_maxnum(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let res: BID_UINT128;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let sig_n_prime192: BID_UINT192;
    let sig_n_prime256: BID_UINT256;
    let mut x_is_zero: bool = false;
    let mut y_is_zero: bool = false;
    let mut x: BID_UINT128 = *x;
    let mut y: BID_UINT128 = *y;

    //BID_SWAP128 (x);
    //BID_SWAP128 (y);

    // check for non-canonical x
    if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
        x.w[1] &= 0xfe003fffffffffffu64; // clear out G[6]-G[16]
        // check for non-canonical NaN payload
        if  ((x.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
        || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (x.w[0] > 0x38c15b09ffffffffu64)) {
            x.w[1] &= 0xffffc00000000000u64;
            x.w[0]  = 0x0u64;
        }
    } else if (x.w[1] & MASK_ANY_INF) == MASK_INF { // x = inf
        x.w[1] &= MASK_SIGN | MASK_INF;
        x.w[0]  = 0x0u64;
    } else { // x is not special
        // check for non-canonical values - treated as zero
        if (x.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS { // G0_G1=11
            // non-canonical
            x.w[1] = (x.w[1] & MASK_SIGN) | ((x.w[1] << 2) & MASK_EXP);
            x.w[0] = 0x0u64;
        } else { // G0_G1 != 11
            if  (x.w[1] & MASK_COEFF) > 0x0001ed09bead87c0u64
            || ((x.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
              && x.w[0] > 0x378d8e63ffffffffu64) {
                // x is non-canonical if coefficient is larger than 10^34 -1
                x.w[1] = (x.w[1] & MASK_SIGN) | (x.w[1] & MASK_EXP);
                x.w[0] = 0x0u64;
            } else {
                // canonicals
            }
        }
    }
    // check for non-canonical y
    if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
        y.w[1] &= 0xfe003fffffffffffu64; // clear out G[6]-G[16]
        // check for non-canonical NaN payload
        if  ((y.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
        || (((y.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (y.w[0] > 0x38c15b09ffffffffu64)) {
            y.w[1] &= 0xffffc00000000000u64;
            y.w[0]  = 0x0u64;
        }
    } else if (y.w[1] & MASK_ANY_INF) == MASK_INF { // y = inf
        y.w[1] &= MASK_SIGN | MASK_INF;
        y.w[0]  = 0x0u64;
    } else { // y is not special
        // check for non-canonical values - treated as zero
        if (y.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS { // G0_G1=11
            // non-canonical
            y.w[1] = (y.w[1] & MASK_SIGN) | ((y.w[1] << 2) & MASK_EXP);
            y.w[0] = 0x0u64;
        } else { // G0_G1 != 11
            if  (y.w[1] & MASK_COEFF) > 0x0001ed09bead87c0u64
            || ((y.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
              && y.w[0] > 0x378d8e63ffffffffu64) {
                // y is non-canonical if coefficient is larger than 10^34 -1
                y.w[1] = (y.w[1] & MASK_SIGN) | (y.w[1] & MASK_EXP);
                y.w[0] = 0x0u64;
            } else {
                // canonical
            }
        }
    }

    // NaN (CASE1)
    if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
        res = if (x.w[1] & MASK_SNAN) == MASK_SNAN { // x is SNaN
            // if x is SNAN, then return quiet (x)
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set exception if SNaN
            x.w[1] &= 0xfdffffffffffffffu64; // quietize x
            x
        } else { // x is QNaN
            if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
                if (y.w[1] & MASK_SNAN) == MASK_SNAN { // y is SNAN
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set invalid flag
                }
                x
            } else {
                y
            }
        };
        return res;
    } else if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NaN, but x is not
        res = if (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set exception if SNaN
            y.w[1] &= 0xfdffffffffffffffu64; // quietize y
            y
        } else {
            // will return x (which is not NaN)
            x
        };
        return res;
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal (not Greater).
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = x;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        res = if (x.w[1] & MASK_SIGN) == MASK_SIGN { y } else { x };
        return res;
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is positive infinity, then x is less, return 0
        //                 if y is negative infinity, then x is greater, return 1
        res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y };
        return res;
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B =>
    //        therefore ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if (sig_x.w[1] == 0) && (sig_x.w[0] == 0) {
        x_is_zero = true;
    }
    if (sig_y.w[1] == 0) && (sig_y.w[0] == 0) {
        y_is_zero = true;
    }

    if x_is_zero && y_is_zero {
        // if both numbers are zero, neither is greater => return either number
        res = x;
        return res;
    } else if x_is_zero {
        // is x is zero, it is greater if Y is negative
        res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y };
        return res;
    } else if y_is_zero {
        // is y is zero, X is greater if it is positive
        res = if (x.w[1] & MASK_SIGN) != MASK_SIGN { x } else { y };
        return res;
    }
    // OPPOSITE SIGN (CASE5)
    // now, if the sign bits differ, x is greater if y is negative
    if ((x.w[1] ^ y.w[1]) & MASK_SIGN) == MASK_SIGN {
        res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y };
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    // if exponents are the same, then we have a simple comparison of
    // the significands
    if exp_y == exp_x {
        res = if ((sig_x.w[1]  > sig_y.w[1])
               || (sig_x.w[1] == sig_y.w[1]
                && sig_x.w[0] >= sig_y.w[0])) ^ ((x.w[1] & MASK_SIGN) == MASK_SIGN) { x } else { y };
        return res;
    }
    // if both components are either bigger or smaller, it is clear what
    // needs to be done
    if (sig_x.w[1] > sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] > sig_y.w[0])) && exp_x >= exp_y {
        res = if (x.w[1] & MASK_SIGN) != MASK_SIGN { x } else { y };
        return res;
    }
    if (sig_x.w[1] < sig_y.w[1] || (sig_x.w[1] == sig_y.w[1] && sig_x.w[0] < sig_y.w[0])) && exp_x <= exp_y {
        res = if (x.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y };
        return res;
    }
    diff = exp_x - exp_y;
    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            // difference cannot be greater than 10^33
            res = if (x.w[1] & MASK_SIGN) != MASK_SIGN { x } else { y };
            return res;
        }
        if diff > 19 { //128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &bid_ten2k128[(diff - 20) as usize]);
            // if postitive, return whichever significand is larger
            // (converse if negative)
            res = if (((sig_n_prime256.w[3]  > 0)
                      || sig_n_prime256.w[2]  > 0)
                     || (sig_n_prime256.w[1]  > sig_y.w[1])
                     || (sig_n_prime256.w[1] == sig_y.w[1]
                      && sig_n_prime256.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN) { x } else { y };
            return res;
        }
        sig_n_prime192 = __mul_64x128_to_192(bid_ten2k64[diff as usize], &sig_x);
        // if postitive, return whichever significand is larger
        // (converse if negative)
        res = if ((sig_n_prime192.w[2]  > 0)
                || (sig_n_prime192.w[1]  > sig_y.w[1])
                || (sig_n_prime192.w[1] == sig_y.w[1]
                 && sig_n_prime192.w[0]  > sig_y.w[0])) ^ ((y.w[1] & MASK_SIGN) == MASK_SIGN) { x } else { y };
        return res;
    }
    diff = exp_y - exp_x;
    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = if (x.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y };
        return res;
    }
    if diff > 19 { //128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &bid_ten2k128[(diff - 20) as usize]);
        // if postitive, return whichever significand is larger
        // (converse if negative)
        res = if (sig_n_prime256.w[3] != 0
               || sig_n_prime256.w[2] != 0
              || (sig_n_prime256.w[1]  > sig_x.w[1]
              || (sig_n_prime256.w[1] == sig_x.w[1]
               && sig_n_prime256.w[0]  > sig_x.w[0]))) ^ ((x.w[1] & MASK_SIGN) != MASK_SIGN) { x } else { y };
        return res;
    }
    // adjust the y significand upwards
    sig_n_prime192 = __mul_64x128_to_192(bid_ten2k64[diff as usize], &sig_y);
    // if postitive, return whichever significand is larger (converse if negative)
    res = if (sig_n_prime192.w[2] != 0
          || (sig_n_prime192.w[1]  > sig_x.w[1]
          || (sig_n_prime192.w[1] == sig_x.w[1]
           && sig_n_prime192.w[0]  > sig_x.w[0]))) ^ ((y.w[1] & MASK_SIGN) != MASK_SIGN) { x } else { y };
    res
}

/// Returns the canonicalized floating-point number x if |x| > |y|,
/// y if |y| > |x|, otherwise this function is identical to __bid128_maxnum
pub (crate) fn bid128_maxnum_mag(x: &BID_UINT128, y: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let res: BID_UINT128;
    let exp_x: i32;
    let exp_y: i32;
    let mut diff: i32;
    let mut sig_x: BID_UINT128 = BID_UINT128::default();
    let mut sig_y: BID_UINT128 = BID_UINT128::default();
    let sig_n_prime192: BID_UINT192;
    let sig_n_prime256: BID_UINT256;
    let mut x: BID_UINT128 = *x;
    let mut y: BID_UINT128 = *y;

    //BID_SWAP128 (x);
    //BID_SWAP128 (y);

    // check for non-canonical x
    if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
        x.w[1] &= 0xfe003fffffffffffu64; // clear out G[6]-G[16]
        // check for non-canonical NaN payload
        if  ((x.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
        || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (x.w[0] > 0x38c15b09ffffffffu64)) {
            x.w[1] &= 0xffffc00000000000u64;
            x.w[0]  = 0x0u64;
        }
    } else if (x.w[1] & MASK_ANY_INF) == MASK_INF { // x = inf
        x.w[1] &= MASK_SIGN | MASK_INF;
        x.w[0]  = 0x0u64;
    } else { // x is not special
        // check for non-canonical values - treated as zero
        if (x.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS { // G0_G1=11
            // non-canonical
            x.w[1] = (x.w[1] & MASK_SIGN) | ((x.w[1] << 2) & MASK_EXP);
            x.w[0] = 0x0u64;
        } else { // G0_G1 != 11
            if  (x.w[1] & MASK_COEFF) > 0x0001ed09bead87c0u64
            || ((x.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
              && x.w[0] > 0x378d8e63ffffffffu64) {
                // x is non-canonical if coefficient is larger than 10^34 -1
                x.w[1] = (x.w[1] & MASK_SIGN) | (x.w[1] & MASK_EXP);
                x.w[0] = 0x0u64;
            } else {
                // canonical
            }
        }
    }
    // check for non-canonical y
    if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
        y.w[1] &= 0xfe003fffffffffffu64; // clear out G[6]-G[16]
        // check for non-canonical NaN payload
        if  ((y.w[1] & 0x00003fffffffffffu64) > 0x0000314dc6448d93u64)
        || (((y.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (y.w[0] > 0x38c15b09ffffffffu64)) {
            y.w[1] &= 0xffffc00000000000u64;
            y.w[0]  = 0x0u64;
        }
    } else if (y.w[1] & MASK_ANY_INF) == MASK_INF { // y = inf
        y.w[1] &= MASK_SIGN | MASK_INF;
        y.w[0] = 0x0u64;
    } else { // y is not special
        // check for non-canonical values - treated as zero
        if (y.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS { // G0_G1=11
            // non-canonical
            y.w[1] = (y.w[1] & MASK_SIGN) | ((y.w[1] << 2) & MASK_EXP);
            y.w[0] = 0x0u64;
        } else { // G0_G1 != 11
            if  (y.w[1] & MASK_COEFF) > 0x0001ed09bead87c0u64
            || ((y.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
              && y.w[0] > 0x378d8e63ffffffffu64) {
                // y is non-canonical if coefficient is larger than 10^34 -1
                y.w[1] = (y.w[1] & MASK_SIGN) | (y.w[1] & MASK_EXP);
                y.w[0] = 0x0u64;
            } else {
                // canonical
            }
        }
    }

    // NaN (CASE1)
    if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
        res = if (x.w[1] & MASK_SNAN) == MASK_SNAN { // x is SNaN
            // if x is SNAN, then return quiet (x)
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set exception if SNaN
            x.w[1] &= 0xfdffffffffffffffu64; // quietize x
            x
        } else { // x is QNaN
            if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
                if (y.w[1] & MASK_SNAN) == MASK_SNAN { // y is SNAN
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set invalid flag
                }
                x
            } else {
                y
            }
        };
        return res;
    } else if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NaN, but x is not
        res = if (y.w[1] & MASK_SNAN) == MASK_SNAN {
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION; // set exception if SNaN
            y.w[1] &= 0xfdffffffffffffffu64; // quietize y
            y
        } else {
            // will return x (which is not NaN)
            x
        };
        return res;
    }
    // SIMPLE (CASE2)
    // if all the bits are the same, these numbers are equal (not Greater).
    if x.w[0] == y.w[0] && x.w[1] == y.w[1] {
        res = y;
        return res;
    }
    // INFINITY (CASE3)
    if (x.w[1] & MASK_INF) == MASK_INF {
        // if x infinity, it has maximum magnitude
        res = if (x.w[1] & MASK_SIGN) == MASK_SIGN && (y.w[1] & MASK_INF) == MASK_INF { y } else { x };
        return res;
    } else if (y.w[1] & MASK_INF) == MASK_INF {
        // x is finite, so if y is positive infinity, then x is less, return 0
        //                 if y is negative infinity, then x is greater, return 1
        res = y;
        return res;
    }
    // CONVERT X
    sig_x.w[1] = x.w[1] & 0x0001ffffffffffffu64;
    sig_x.w[0] = x.w[0];
    exp_x      = ((x.w[1] >> 49) & 0x000000000003fffu64) as i32;

    // CONVERT Y
    exp_y      = ((y.w[1] >> 49) & 0x0000000000003fffu64) as i32;
    sig_y.w[1] = y.w[1] & 0x0001ffffffffffffu64;
    sig_y.w[0] = y.w[0];

    // ZERO (CASE4)
    // some properties:
    //    (+ZERO == -ZERO) => therefore ignore the sign
    //    (ZERO x 10^A == ZERO x 10^B) for any valid A, B =>
    //         therefore ignore the exponent field
    //    (Any non-canonical # is considered 0)
    if (sig_x.w[1] == 0) && (sig_x.w[0] == 0) {
        res = y;
        return res;
    }
    if (sig_y.w[1] == 0) && (sig_y.w[0] == 0) {
        res = x;
        return res;
    }
    // REDUNDANT REPRESENTATIONS (CASE6)
    if exp_y == exp_x && sig_x.w[1] == sig_y.w[1] &&
        sig_x.w[0] == sig_y.w[0] {
        // check if exponents are the same and significands are the same
        res = if (x.w[1] & 0x8000000000000000u64) == 0x8000000000000000u64 { // x is negative
            y
        } else {
            x
        };
        return res;
    } else if ((sig_x.w[1]  > sig_y.w[1]
            || (sig_x.w[1] == sig_y.w[1]
             && sig_x.w[0]  > sig_y.w[0])) && exp_x == exp_y)
           || ((sig_x.w[1]  > sig_y.w[1]
            || (sig_x.w[1] == sig_y.w[1]
             && sig_x.w[0] >= sig_y.w[0])) && exp_x > exp_y) {
        // if both components are either bigger or smaller, it is clear what
        // needs to be done; also if the magnitudes are equal
        res = x;
        return res;
    } else if ((sig_y.w[1]  > sig_x.w[1]
            || (sig_y.w[1] == sig_x.w[1]
             && sig_y.w[0]  > sig_x.w[0])) && exp_y == exp_x)
           || ((sig_y.w[1]  > sig_x.w[1]
            || (sig_y.w[1] == sig_x.w[1]
             && sig_y.w[0] >= sig_x.w[0])) && exp_y > exp_x) {
        res = y;
        return res;
    } else {
        // continue
    }
    diff = exp_x - exp_y;
    // if |exp_x - exp_y| < 33, it comes down to the compensated significand
    if diff > 0 { // to simplify the loop below,
        // if exp_x is 33 greater than exp_y, no need for compensation
        if diff > 33 {
            res = x; // difference cannot be greater than 10^33
            return res;
        }
        if diff > 19 { //128 by 128 bit multiply -> 256 bits
            sig_n_prime256 = __mul_128x128_to_256(&sig_x, &bid_ten2k128[(diff - 20) as usize]);
            // if postitive, return whichever significand is larger
            // (converse if negative)
            if  sig_n_prime256.w[3] == 0
            && (sig_n_prime256.w[2] == 0)
             && sig_n_prime256.w[1] == sig_y.w[1]
            && (sig_n_prime256.w[0] == sig_y.w[0]) {
                res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y }; // if equal
                return res;
            }
            res = if ((sig_n_prime256.w[3] > 0)
                    || sig_n_prime256.w[2] > 0)
                   || (sig_n_prime256.w[1] > sig_y.w[1])
                   || (sig_n_prime256.w[1] == sig_y.w[1]
                    && sig_n_prime256.w[0] > sig_y.w[0]) { x } else { y };
            return res;
        }
        sig_n_prime192 = __mul_64x128_to_192(bid_ten2k64[diff as usize], &sig_x);
        // if postitive, return whichever significand is larger (converse if negative)
        if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_y.w[1] && (sig_n_prime192.w[0] == sig_y.w[0]) {
            // if equal, return positive magnitude
            res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y };
            return res;
        }
        res = if (sig_n_prime192.w[2]  > 0)
              || (sig_n_prime192.w[1]  > sig_y.w[1])
              || (sig_n_prime192.w[1] == sig_y.w[1]
               && sig_n_prime192.w[0]  > sig_y.w[0]) { x } else { y };
        return res;
    }
    diff = exp_y - exp_x;
    // if exp_x is 33 less than exp_y, no need for compensation
    if diff > 33 {
        res = y;
        return res;
    }
    if diff > 19 { //128 by 128 bit multiply -> 256 bits
        // adjust the y significand upwards
        sig_n_prime256 = __mul_128x128_to_256(&sig_y, &bid_ten2k128[(diff - 20) as usize]);
        // if postitive, return whichever significand is larger
        // (converse if negative)
        if  sig_n_prime256.w[3] == 0
        && (sig_n_prime256.w[2] == 0)
         && sig_n_prime256.w[1] == sig_x.w[1]
        && (sig_n_prime256.w[0] == sig_x.w[0]) {
            // if equal, return positive (if possible)
            res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y };
            return res;
        }
        res = if  sig_n_prime256.w[3] == 0 && sig_n_prime256.w[2] == 0
              && (sig_n_prime256.w[1] < sig_x.w[1]
              || (sig_n_prime256.w[1] == sig_x.w[1]
               && sig_n_prime256.w[0] < sig_x.w[0])) { x } else { y };
        return res;
    }
    // adjust the y significand upwards
    sig_n_prime192 = __mul_64x128_to_192(bid_ten2k64[diff as usize], &sig_y);
    // if postitive, return whichever significand is larger (converse if negative)
    if (sig_n_prime192.w[2] == 0) && sig_n_prime192.w[1] == sig_x.w[1] && (sig_n_prime192.w[0] == sig_x.w[0]) {
        // if equal, return positive (if possible)
        res = if (y.w[1] & MASK_SIGN) == MASK_SIGN { x } else { y };
        return res;
    }
    res = if  sig_n_prime192.w[2] == 0
          && (sig_n_prime192.w[1] < sig_x.w[1]
          || (sig_n_prime192.w[1] == sig_x.w[1]
           && sig_n_prime192.w[0] < sig_x.w[0])) { x } else { y };
    res
}
