/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(target_endian = "big")]
use crate::bid_conf::BID_SWAP128;

use crate::bid128::*;
use crate::bid_conf::{BID_HIGH_128W, BID_LOW_128W};
use crate::bid_internal::{__mul_128x128_to_256, __mul_128x64_to_128, __mul_64x128_full, __mul_64x128_to_128, __mul_64x64_to_128MACH};
use crate::bid_round::*;
use crate::constants::*;
use crate::convert::{bid128_to_bid64, bid64_to_bid128};
use crate::core::{RoundingMode, StatusFlags};
use crate::d128::{_IDEC_flags, BID_SINT64, BID_UI64DOUBLE, BID_UINT128, BID_UINT192, BID_UINT256, BID_UINT64};

//////////////////////////////////////////////
// BID128 fma   x * y + z
//////////////////////////////////////////////

pub (crate) fn bid_rounding_correction(
    rnd_mode: u32,
    is_inexact_lt_midpoint: bool,
    is_inexact_gt_midpoint: bool,
    is_midpoint_lt_even: bool,
    is_midpoint_gt_even: bool,
    unbexp: i32,
    ptrres: &mut BID_UINT128,
    ptrfpsf: &mut _IDEC_flags) {

    // unbiased true exponent unbexp may be larger than emax

    let mut res: BID_UINT128 = *ptrres; // expected to have the correct sign and coefficient
    // (the exponent field is ignored, as unbexp is used instead)
    let sign: BID_UINT64;
    let mut exp: BID_UINT64;
    let mut C_hi: BID_UINT64;
    let mut C_lo: BID_UINT64;
    let mut unbexp = unbexp;

    // general correction from RN to RA, RM, RP, RZ
    // Note: if the result is negative, then is_inexact_lt_midpoint,
    // is_inexact_gt_midpoint, is_midpoint_lt_even, and is_midpoint_gt_even
    // have to be considered as if determined for the absolute value of the
    // result (so they seem to be reversed)

    if is_inexact_lt_midpoint || is_inexact_gt_midpoint || is_midpoint_lt_even || is_midpoint_gt_even {
        *ptrfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
    }
    // apply correction to result calculated with unbounded exponent
    sign = res.w[1] & MASK_SIGN;
    exp  = ((unbexp + 6176) as BID_UINT64) << 49; // valid only if expmin<=unbexp<=expmax
    C_hi = res.w[1] & MASK_COEFF;
    C_lo = res.w[0];
    if (sign == 0
    && ((rnd_mode == RoundingMode::BID_ROUNDING_UP && is_inexact_lt_midpoint)
    || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
     || rnd_mode == RoundingMode::BID_ROUNDING_UP) && is_midpoint_gt_even)))
    || (sign != 0 && ((rnd_mode == RoundingMode::BID_ROUNDING_DOWN && is_inexact_lt_midpoint)
     || ((rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY
       || rnd_mode == RoundingMode::BID_ROUNDING_DOWN) && is_midpoint_gt_even))) {
        // C = C + 1
        C_lo += 1;
        if C_lo == 0 {
            C_hi += 1;
        }
        if C_hi == 0x0001ed09bead87c0u64 && C_lo == 0x378d8e6400000000u64 {
            // C = 10^34 => rounding overflow
            C_hi    = 0x0000314dc6448d93u64;
            C_lo    = 0x38c15b0a00000000u64; // 10^33
            // exp = exp + EXP_P1;
            unbexp += 1;
            exp     = ((unbexp + 6176)as BID_UINT64) << 49;
        }
    } else if (is_midpoint_lt_even || is_inexact_gt_midpoint)
          && ((sign != 0 && (rnd_mode == RoundingMode::BID_ROUNDING_UP   || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))
           || (sign == 0 && (rnd_mode == RoundingMode::BID_ROUNDING_DOWN || rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO))) {
        // C = C - 1
        C_lo -= 1;
        if C_lo == 0xffffffffffffffffu64 {
            C_hi -= 1;
        }
        // check if we crossed into the lower decade
        if C_hi == 0x0000314dc6448d93u64 && C_lo == 0x38c15b09ffffffffu64 {
            // C = 10^33 - 1
            if exp > 0 {
                C_hi = 0x0001ed09bead87c0u64; // 10^34 - 1
                C_lo = 0x378d8e63ffffffffu64;
                // exp = exp - EXP_P1;
                unbexp -= 1;
                exp     = ((unbexp + 6176) as BID_UINT64) << 49;
            } else { // if exp = 0 the result is tiny & inexact
                *ptrfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
            }
        }
    } else {
        // the result is already correct
    }
    if unbexp > EXP_MAX_UNBIASED { // 6111
        *ptrfpsf |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
        exp       = 0;
        if sign == 0 { // result is positive
            (C_hi, C_lo) = if rnd_mode == RoundingMode::BID_ROUNDING_UP || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY { // +inf
                (0x7800000000000000u64, 0x0000000000000000u64)
            } else { // res = +MAXFP = (10^34-1) * 10^emax
                (0x5fffed09bead87c0u64, 0x378d8e63ffffffffu64)
            };
        } else { // result is negative
            (C_hi, C_lo) = if rnd_mode == RoundingMode::BID_ROUNDING_DOWN || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY { // -inf
                (0xf800000000000000u64, 0x0000000000000000u64)
            } else { // res = -MAXFP = -(10^34-1) * 10^emax
                (0xdfffed09bead87c0u64, 0x378d8e63ffffffffu64)
            };
        }
    }
    // assemble the result
    res.w[1] = sign | exp | C_hi;
    res.w[0] = C_lo;
    *ptrres = res;
}

pub(crate) fn bid_add256(x: &BID_UINT256, y: &BID_UINT256) -> BID_UINT256 {
    // *z = x + yl assume the sum fits in 256 bits
    let mut x: BID_UINT256 = *x;
    let y: BID_UINT256 = *y;
    let mut z: BID_UINT256 = BID_UINT256::default();
    z.w[0] = x.w[0] + y.w[0];
    if z.w[0] < x.w[0] {
        x.w[1] += 1;
        if x.w[1] == 0x0000000000000000u64 {
            x.w[2] += 1;
            if x.w[2] == 0x0000000000000000u64 {
                x.w[3] += 1;
            }
        }
    }
    z.w[1] = x.w[1] + y.w[1];
    if z.w[1] < x.w[1] {
        x.w[2] += 1;
        if x.w[2] == 0x0000000000000000u64 {
            x.w[3] += 1;
        }
    }
    z.w[2] = x.w[2] + y.w[2];
    if z.w[2] < x.w[2] {
        x.w[3] += 1;
    }
    z.w[3] = x.w[3] + y.w[3]; // it was assumed that no carry is possible
    z
}

pub(crate) fn bid_sub256(x: &BID_UINT256, y: &BID_UINT256) -> BID_UINT256 {
    // *z = x - y; assume x >= y
    let mut x: BID_UINT256 = *x;
    let y: BID_UINT256 = *y;
    let mut z: BID_UINT256 = BID_UINT256::default();
    z.w[0] = x.w[0] - y.w[0];
    if z.w[0] > x.w[0] {
        x.w[1] -= 1;
        if x.w[1] == 0xffffffffffffffffu64 {
            x.w[2] -= 1;
            if x.w[2] == 0xffffffffffffffffu64 {
                x.w[3] -= 1;
            }
        }
    }
    z.w[1] = x.w[1] - y.w[1];
    if z.w[1] > x.w[1] {
        x.w[2] -= 1;
        if x.w[2] == 0xffffffffffffffffu64 {
            x.w[3] -= 1;
        }
    }
    z.w[2] = x.w[2] - y.w[2];
    if z.w[2] > x.w[2] {
        x.w[3] -= 1;
    }
    z.w[3] = x.w[3] - y.w[3]; // no borrow possible, because x >= y
    z
}

pub (crate) fn bid_bid_nr_digits256(r256: &BID_UINT256) -> i32 {
    let mut ind: i32;

    // determine the number of decimal digits in r256
    if r256.w[3] == 0x0 && r256.w[2] == 0x0 && r256.w[1] == 0x0 {
        // between 1 and 19 digits
        ind = bid_ten2k64[1..=19]
            .iter().enumerate()
            .take_while(|&(_, x)| r256.w[0] >= *x)
            .count() as i32;
        // ind digits
    } else if r256.w[3]  == 0x0 && r256.w[2] == 0x0
           && (r256.w[1]  < bid_ten2k128[0].w[1]
           || (r256.w[1] == bid_ten2k128[0].w[1]
            && r256.w[0]  < bid_ten2k128[0].w[0])) {
        // 20 digits
        ind = 20;
    } else if r256.w[3] == 0x0 && r256.w[2] == 0x0 {
        // between 21 and 38 digits
        ind = bid_ten2k128[1..=18]
            .iter().enumerate()
            .take_while(|&(_, d)|
                !(r256.w[1] < d.w[1] || (r256.w[1] == d.w[1] && r256.w[0] < d.w[0]))
            ).count() as i32;

        // ind + 20 digits
        ind += 20;
    } else if r256.w[3]  == 0x0 &&
             (r256.w[2]   < bid_ten2k256[0].w[2] ||
              (r256.w[2] == bid_ten2k256[0].w[2] &&
               r256.w[1]  < bid_ten2k256[0].w[1]) ||
              (r256.w[2] == bid_ten2k256[0].w[2] &&
               r256.w[1] == bid_ten2k256[0].w[1] &&
               r256.w[0]  < bid_ten2k256[0].w[0])) {
        // 39 digits
        ind = 39;
    } else {
        // between 40 and 68 digits
        ind = bid_ten2k256[1..=29usize]
            .iter().enumerate()
            .take_while(|&(_, d)|
                !(r256.w[3]  < d.w[3]
              || (r256.w[3] == d.w[3] && r256.w[2]  < d.w[2])
              || (r256.w[3] == d.w[3] && r256.w[2] == d.w[2] && r256.w[1]  < d.w[1])
              || (r256.w[3] == d.w[3] && r256.w[2] == d.w[2] && r256.w[1] == d.w[1] && r256.w[0] < d.w[0])))
            .count() as i32;
        // ind + 39 digits
        ind += 39;
    }

    ind
}

/// add/subtract C4 and C3 * 10^scale; this may follow a previous rounding, so
/// use the rounding information from ptr_is_* to avoid a double rounding error
pub (crate) fn bid_add_and_round(
    q3: i32,
    q4: i32,
    e4: i32,
    delta: i32,
    p34: i32,
    z_sign: BID_UINT64,
    p_sign: BID_UINT64,
    C3: &BID_UINT128,
    C4: &BID_UINT256,
    rnd_mode: u32,
    ptr_is_midpoint_lt_even: &mut bool,
    ptr_is_midpoint_gt_even: &mut bool,
    ptr_is_inexact_lt_midpoint: &mut bool,
    ptr_is_inexact_gt_midpoint: &mut bool,
    ptrfpsf: &mut _IDEC_flags) -> BID_UINT128 {

    let mut ptrres: BID_UINT128 = BID_UINT128::default();
    let mut scale: i32;
    let mut x0: i32;
    let mut ind: i32;
    let R64: BID_UINT64;
    let mut P128: BID_UINT128 = BID_UINT128::default();
    let mut R128: BID_UINT128 = BID_UINT128::default();
    let mut P192: BID_UINT192 = BID_UINT192::default();
    let R192: BID_UINT192;
    let mut R256: BID_UINT256 = BID_UINT256::default();
    let mut is_midpoint_lt_even: bool = false;
    let mut is_midpoint_gt_even: bool = false;
    let mut is_inexact_lt_midpoint: bool = false;
    let mut is_inexact_gt_midpoint: bool = false;
    let is_midpoint_lt_even0: bool;
    let is_midpoint_gt_even0: bool;
    let is_inexact_lt_midpoint0: bool;
    let is_inexact_gt_midpoint0: bool;
    let mut incr_exp: bool = false;
    let mut is_tiny: bool = false;
    let mut lt_half_ulp: bool = false;
    let mut eq_half_ulp: bool = false;
    // let gt_half_ulp: bool = false;
    let mut res: BID_UINT128 = ptrres;
    let mut p_sign: BID_UINT64 = p_sign;
    let mut e4: i32 = e4;

    // scale C3 up by 10^(q4-delta-q3), 0 <= q4-delta-q3 <= 2*P34-2 = 66
    scale = q4 - delta - q3; // 0 <= scale <= 66 (or 0 <= scale <= 68 if this
    // comes from Cases (2), (3), (4), (5), (6), with 0 <= |delta| <= 1

    // calculate C3 * 10^scale in R256 (it has at most 67 decimal digits for
    // Cases (15),(16),(17) and at most 69 for Cases (2),(3),(4),(5),(6))
    match scale {
        val if val == 0 => {
            R256.w[3] = 0x0u64;
            R256.w[2] = 0x0u64;
            R256.w[1] = C3.w[1];
            R256.w[0] = C3.w[0];
        },
        val if val <= 19 => { // 10^scale fits in 64 bits
            P128.w[1] = 0;
            P128.w[0] = bid_ten2k64[scale as usize];
            R256 = __mul_128x128_to_256(&P128, C3);
        },
        val if val <= 38 => { // 10^scale fits in 128 bits
            R256 = __mul_128x128_to_256(&bid_ten2k128[(scale - 20) as usize], C3);
        },
        val if val <= 57 => { // 39 <= scale <= 57
            // 10^scale fits in 192 bits but C3 * 10^scale fits in 223 or 230 bits
            // (10^67 has 223 bits; 10^69 has 230 bits);
            // must split the computation:
            // 10^scale * C3 = 10*38 * 10^(scale-38) * C3 where 10^38 takes 127
            // bits and so 10^(scale-38) * C3 fits in 128 bits with certainty
            // Note that 1 <= scale - 38 <= 19 => 10^(scale-38) fits in 64 bits
            R128 = __mul_64x128_to_128(bid_ten2k64[(scale - 38) as usize], C3);
            // now multiply R128 by 10^38
            R256 = __mul_128x128_to_256(&R128, &bid_ten2k128[18]);
        },
        _ => { // 58 <= scale <= 66
            // 10^scale takes between 193 and 220 bits,
            // and C3 * 10^scale fits in 223 bits (10^67/10^69 has 223/230 bits)
            // must split the computation:
            // 10^scale * C3 = 10*38 * 10^(scale-38) * C3 where 10^38 takes 127
            // bits and so 10^(scale-38) * C3 fits in 128 bits with certainty
            // Note that 20 <= scale - 38 <= 30 => 10^(scale-38) fits in 128 bits
            // Calculate first 10^(scale-38) * C3, which fits in 128 bits; because
            // 10^(scale-38) takes more than 64 bits, C3 will take less than 64
            R128 = __mul_64x128_to_128(C3.w[0], &bid_ten2k128[(scale - 58) as usize]);
            // now calculate 10*38 * 10^(scale-38) * C3
            R256 = __mul_128x128_to_256(&R128, &bid_ten2k128[18]);
        }
    }
    // C3 * 10^scale is now in R256

    // for Cases (15), (16), (17) C4 > C3 * 10^scale because C4 has at least
    // one extra digit; for Cases (2), (3), (4), (5), or (6) any order is
    // possible
    // add/subtract C4 and C3 * 10^scale; the exponent is e4
    if p_sign == z_sign { // R256 = C4 + R256
        // calculate R256 = C4 + C3 * 10^scale = C4 + R256 which is exact,
        // but may require rounding
        R256 = bid_add256(C4, &R256);
    } else { // if (p_sign != z_sign) { // R256 = C4 - R256
        // calculate R256 = C4 - C3 * 10^scale = C4 - R256 or
        // R256 = C3 * 10^scale - C4 = R256 - C4 which is exact,
        // but may require rounding

        // compare first R256 = C3 * 10^scale and C4
        if R256.w[3]  > C4.w[3] || (R256.w[3] == C4.w[3] && R256.w[2]  > C4.w[2])
       || (R256.w[3] == C4.w[3] &&  R256.w[2] == C4.w[2] && R256.w[1]  > C4.w[1])
       || (R256.w[3] == C4.w[3] &&  R256.w[2] == C4.w[2] && R256.w[1] == C4.w[1]
        && R256.w[0] >= C4.w[0]) { // C3 * 10^scale >= C4
            // calculate R256 = C3 * 10^scale - C4 = R256 - C4, which is exact,
            // but may require rounding
            R256 = bid_sub256(C4, &R256);
            // flip p_sign too, because the result has the sign of z
            p_sign = z_sign;
        } else { // if C4 > C3 * 10^scale
            // calculate R256 = C4 - C3 * 10^scale = C4 - R256, which is exact,
            // but may require rounding
            R256 = bid_sub256(C4, &R256);
        }
        // if the result is pure zero, the sign depends on the rounding mode
        // (x*y and z had opposite signs)
        if R256.w[3] == 0x0u64 && R256.w[2] == 0x0u64
        && R256.w[1] == 0x0u64 && R256.w[0] == 0x0u64 {
            p_sign = if rnd_mode != RoundingMode::BID_ROUNDING_DOWN {
                0x0000000000000000u64
            } else {
                0x8000000000000000u64
            };
            // the exponent is max (e4, expmin)
            if e4 < -6176 {
                e4 = EXP_MIN_UNBIASED;
            }
            // assemble result
            res.w[1] = p_sign | (((e4 + 6176) as BID_UINT64) << 49);
            res.w[0] = 0x0;
            ptrres = res;
            return ptrres;
        }
    }

    // determine the number of decimal digits in R256
    ind = bid_bid_nr_digits256(&R256);

    // the exact result is (-1)^p_sign * R256 * 10^e4 where q (R256) = ind;
    // round to the destination precision, with unbounded exponent

    if ind <= p34 {
        // result rounded to the destination precision with unbounded exponent
        // is exact
        if ind + e4 < p34 + EXP_MIN_UNBIASED {
            is_tiny = true; // applies to all rounding modes
            // (regardless of the tininess detection method)
        }
        res.w[1] = p_sign | (((e4 + 6176) as BID_UINT64) << 49) | R256.w[1];
        res.w[0] = R256.w[0];
        // Note: res is correct only if expmin <= e4 <= expmax
    } else { // if (ind > p34)
        // if more than P digits, round to nearest to P digits
        // round R256 to p34 digits
        x0 = ind - p34; // 1 <= x0 <= 34 as 35 <= ind <= 68
        match ind {
            val if val <= 38 => {
                P128.w[1] = R256.w[1];
                P128.w[0] = R256.w[0];
                R128 = bid_round128_19_38(
                    ind, x0, &P128,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
            },
            val if val <= 57 => {
                P192.w[2] = R256.w[2];
                P192.w[1] = R256.w[1];
                P192.w[0] = R256.w[0];
                R192 = bid_round192_39_57(
                    ind, x0, &P192,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                R128.w[1] = R192.w[1];
                R128.w[0] = R192.w[0];
            },
            _ => { // if (ind <= 68)
                R256 = bid_round256_58_76(
                    ind, x0, &R256,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                R128.w[1] = R256.w[1];
                R128.w[0] = R256.w[0];
            }
        }

        #[cfg(not(feature = "DECIMAL_TINY_DETECTION_AFTER_ROUNDING"))]
        if e4 + x0 < EXP_MIN_UNBIASED { // for all rounding modes
            is_tiny = true;
        }

        // the rounded result has p34 = 34 digits
        e4 = e4 + x0 + if incr_exp { 1 } else { 0 };
        if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST {
            #[cfg(feature = "DECIMAL_TINY_DETECTION_AFTER_ROUNDING")]
            if e4 < EXP_MIN_UNBIASED {
                is_tiny = true; // for other rounding modes apply correction
            }
        } else {
            // for RM, RP, RZ, RA apply correction in order to determine tininess
            // but do not save the result; apply the correction to
            // (-1)^p_sign * significand * 10^0
            P128.w[1] = p_sign | 0x3040000000000000u64 | R128.w[1];
            P128.w[0] = R128.w[0];
            bid_rounding_correction(
                rnd_mode,
                is_inexact_lt_midpoint,
                is_inexact_gt_midpoint,
                is_midpoint_lt_even,
                is_midpoint_gt_even,
                0, &mut P128, ptrfpsf);

            scale = (((P128.w[1] & MASK_EXP) >> 49) - 6176) as i32; // -1, 0, or +1
            // the number of digits in the significand is p34 = 34
            #[cfg(feature = "DECIMAL_TINY_DETECTION_AFTER_ROUNDING")]
            if (e4 + scale) < EXP_MIN_UNBIASED {
                is_tiny = true;
            }
        }
        ind      = p34; // the number of decimal digits in the signifcand of res
        res.w[1] = p_sign | (((e4 + 6176) as BID_UINT64) << 49) | R128.w[1]; // RN
        res.w[0] = R128.w[0];
        // Note: res is correct only if expmin <= e4 <= expmax
        // set the inexact flag after rounding with bounded exponent, if any
    }
    // at this point we have the result rounded with unbounded exponent in
    // res and we know its tininess:
    // res = (-1)^p_sign * significand * 10^e4,
    // where q (significand) = ind <= p34
    // Note: res is correct only if expmin <= e4 <= expmax

    // check for overflow if RN
    if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && (ind + e4) > (p34 + EXP_MAX_UNBIASED) {
        res.w[1]  = p_sign | 0x7800000000000000u64;
        res.w[0]  = 0x0000000000000000u64;
        ptrres    = res;
        *ptrfpsf |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
        return ptrres; // BID_RETURN (res)
    } // else not overflow or not RN, so continue

    // if (e4 >= expmin) we have the result rounded with bounded exponent
    if e4 < EXP_MIN_UNBIASED {
        x0 = EXP_MIN_UNBIASED - e4; // x0 >= 1; the number of digits to chop off of res
        // where the result rounded [at most] once is
        //   (-1)^p_sign * significand_res * 10^e4

        // avoid double rounding error
        is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
        is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
        is_midpoint_lt_even0    = is_midpoint_lt_even;
        is_midpoint_gt_even0    = is_midpoint_gt_even;
        is_inexact_lt_midpoint  = false;
        is_inexact_gt_midpoint  = false;
        is_midpoint_lt_even     = false;
        is_midpoint_gt_even     = false;

        match x0 {
            val if val > ind => {
                // nothing is left of res when moving the decimal point left x0 digits
                is_inexact_lt_midpoint = true;
                res.w[1]               = p_sign | 0x0000000000000000u64;
                res.w[0]               = 0x0000000000000000u64;
                e4                     = EXP_MIN_UNBIASED;
            },
            val if val == ind => { // 1 <= x0 = ind <= p34 = 34
                // this is <, =, or > 1/2 ulp
                // compare the ind-digit value in the significand of res with
                // 1/2 ulp = 5*10^(ind-1), i.e. determine whether it is
                // less than, equal to, or greater than 1/2 ulp (significand of res)
                R128.w[1] = res.w[1] & MASK_COEFF;
                R128.w[0] = res.w[0];
                if ind <= 19 {
                    match bid_midpoint64[(ind - 1) as usize] {
                        val if R128.w[0] < val => { // < 1/2 ulp
                            lt_half_ulp            = true;
                            is_inexact_lt_midpoint = true;
                        },
                        val if R128.w[0] == val => { // = 1/2 ulp
                            eq_half_ulp         = true;
                            is_midpoint_gt_even = true;
                        },
                        _ => {                                  // > 1/2 ulp
                            // gt_half_ulp = 1;
                            is_inexact_gt_midpoint = true;
                        }
                    }
                } else { // if (ind <= 38) {
                    if  R128.w[1]  < bid_midpoint128[(ind - 20) as usize].w[1]
                    || (R128.w[1] == bid_midpoint128[(ind - 20) as usize].w[1]
                     && R128.w[0]  < bid_midpoint128[(ind - 20) as usize].w[0]) { // < 1/2 ulp
                        lt_half_ulp            = true;
                        is_inexact_lt_midpoint = true;
                    } else if R128.w[1] == bid_midpoint128[(ind - 20) as usize].w[1]
                           && R128.w[0] == bid_midpoint128[(ind - 20) as usize].w[0] { // = 1/2 ulp
                        eq_half_ulp         = true;
                        is_midpoint_gt_even = true;
                    } else { // > 1/2 ulp
                        // gt_half_ulp = 1;
                        is_inexact_gt_midpoint = true;
                    }
                }
                if lt_half_ulp || eq_half_ulp {
                    // res = +0.0 * 10^expmin
                    res.w[1] = 0x0000000000000000u64;
                    res.w[0] = 0x0000000000000000u64;
                } else { // if (gt_half_ulp)
                    // res = +1 * 10^expmin
                    res.w[1] = 0x0000000000000000u64;
                    res.w[0] = 0x0000000000000001u64;
                }
                res.w[1] |= p_sign;
                e4        = EXP_MIN_UNBIASED;
            },
            _ => { // if (1 <= x0 <= ind - 1 <= 33)
                // round the ind-digit result to ind - x0 digits

                if ind <= 18 { // 2 <= ind <= 18
                    R64 = bid_round64_2_18(
                        ind, x0, res.w[0], &mut incr_exp,
                        &mut is_midpoint_lt_even,
                        &mut is_midpoint_gt_even,
                        &mut is_inexact_lt_midpoint,
                        &mut is_inexact_gt_midpoint);
                    res.w[1] = 0x0;
                    res.w[0] = R64;
                } else if ind <= 38 {
                    P128.w[1] = res.w[1] & MASK_COEFF;
                    P128.w[0] = res.w[0];
                    res = bid_round128_19_38(
                        ind, x0, &P128, &mut incr_exp,
                        &mut is_midpoint_lt_even,
                        &mut is_midpoint_gt_even,
                        &mut is_inexact_lt_midpoint,
                        &mut is_inexact_gt_midpoint);
                }
                e4 += x0; // expmin
                // we want the exponent to be expmin, so if incr_exp = 1 then
                // multiply the rounded result by 10 - it will still fit in 113 bits
                if incr_exp {
                    // 64 x 128 -> 128
                    P128.w[1] = res.w[1] & MASK_COEFF;
                    P128.w[0] = res.w[0];
                    res       = __mul_64x128_to_128(bid_ten2k64[1], &P128);
                }
                res.w[1] = p_sign | (((e4 + 6176) as BID_UINT64) << 49) | (res.w[1] & MASK_COEFF);
                // avoid a double rounding error
                if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even { // double rounding error upward
                    // res = res - 1
                    res.w[0] -= 1;
                    if res.w[0] == 0xffffffffffffffffu64 {
                        res.w[1] -= 1;
                    }
                    // Note: a double rounding error upward is not possible; for this
                    // the result after the first rounding would have to be 99...95
                    // (35 digits in all), possibly followed by a number of zeros; this
                    // is not possible in Cases (2)-(6) or (15)-(17) which may get here
                    is_midpoint_lt_even    = false;
                    is_inexact_lt_midpoint = true;
                } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even { // double rounding error downward
                    // res = res + 1
                    res.w[0] += 1;
                    if res.w[0] == 0 {
                        res.w[1] += 1;
                    }
                    is_midpoint_gt_even    = false;
                    is_inexact_gt_midpoint = true;
                } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                    // if this second rounding was exact the result may still be
                    // inexact because of the first rounding
                    if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
                        is_inexact_gt_midpoint = true;
                    }
                    if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
                        is_inexact_lt_midpoint = true;
                    }
                } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
                    // pu64ed up to a midpoint
                    is_inexact_lt_midpoint = true;
                    is_inexact_gt_midpoint = false;
                    is_midpoint_lt_even    = false;
                    is_midpoint_gt_even    = false;
                } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
                    // pu64ed down to a midpoint
                    is_inexact_lt_midpoint = true;
                    is_inexact_gt_midpoint = true;
                    is_midpoint_lt_even    = false;
                    is_midpoint_gt_even    = false;
                } else {
                    // ;
                }
            }
        }
    }
    // res contains the correct result
    // apply correction if not rounding to nearest
    if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
        bid_rounding_correction(
            rnd_mode,
            is_inexact_lt_midpoint,
            is_inexact_gt_midpoint,
            is_midpoint_lt_even,
            is_midpoint_gt_even,
            e4, &mut res, ptrfpsf);
    }
    if is_midpoint_lt_even || is_midpoint_gt_even || is_inexact_lt_midpoint || is_inexact_gt_midpoint {
        // set the inexact flag
        *ptrfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
        if is_tiny {
            *ptrfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
        }
    }

    *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;
    ptrres = res;

    ptrres
}

pub (crate) fn bid128_ext_fma(
    ptr_is_midpoint_lt_even: &mut bool,
    ptr_is_midpoint_gt_even: &mut bool,
    ptr_is_inexact_lt_midpoint: &mut bool,
    ptr_is_inexact_gt_midpoint: &mut bool,
    x: &BID_UINT128,
    y: &BID_UINT128,
    z: &BID_UINT128,
    rnd_mode: u32,
    pfpsf: &mut _IDEC_flags) -> BID_UINT128 {

    let mut res: BID_UINT128 = BID_UINT128 { w: [0xbaddbaddbaddbaddu64, 0xbaddbaddbaddbaddu64] };
    let x_sign: BID_UINT64;
    let y_sign: BID_UINT64;
    let mut z_sign: BID_UINT64;
    let mut p_sign: BID_UINT64;
    let tmp_sign: BID_UINT64;
    let mut x_exp: BID_UINT64 = 0;
    let mut y_exp: BID_UINT64 = 0;
    let mut z_exp: BID_UINT64 = 0;
    let mut p_exp: BID_UINT64;
    let true_p_exp: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C2: BID_UINT128 = BID_UINT128::default();
    let mut C3: BID_UINT128 = BID_UINT128::default();
    let mut C4: BID_UINT256 = BID_UINT256::default();
    let mut q1: i32 = 0;
    let mut q2: i32 = 0;
    let mut q3: i32 = 0;
    let mut q4: i32;
    let e1: i32;
    let e2: i32;
    let mut e3: i32;
    let mut e4: i32;
    let mut scale: i32;
    let mut ind: i32;
    let mut delta: i32;
    let mut x0: i32;
    let p34: i32 = P34; // used to modify the limit on the number of digits
    let mut tmp: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: i32;
    let y_nr_bits: i32;
    let z_nr_bits: i32;
    let save_fpsf: u32;
    let mut is_midpoint_lt_even: bool = false;
    let mut is_midpoint_gt_even: bool = false;
    let mut is_inexact_lt_midpoint: bool = false;
    let mut is_inexact_gt_midpoint: bool = false;
    let mut is_midpoint_lt_even0: bool;
    let mut is_midpoint_gt_even0: bool;
    let mut is_inexact_lt_midpoint0: bool;
    let mut is_inexact_gt_midpoint0: bool;
    let mut incr_exp: bool = false;
    let lsb: bool;
    let mut lt_half_ulp: bool = false;
    let mut eq_half_ulp: bool = false;
    let mut gt_half_ulp: bool = false;
    let mut is_tiny: bool = false;
    let mut R64: BID_UINT64;
    let tmp64: BID_UINT64;
    let mut P128: BID_UINT128 = BID_UINT128::default();
    let mut R128: BID_UINT128 = BID_UINT128::default();
    let mut P192: BID_UINT192 = BID_UINT192::default();
    let mut R192: BID_UINT192;
    let mut R256: BID_UINT256 = BID_UINT256::default();
    let C4gt5toq4m1: bool;

    #[cfg(target_endian = "big")]
    let mut x: BID_UINT128 = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    #[cfg(target_endian = "big")]
    let mut y: BID_UINT128 = *y;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut y);

    #[cfg(target_endian = "big")]
    let mut z: BID_UINT128 = *z;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut z);

    // the following are based on the table of special cases for fma; the NaN
    // behavior is similar to that of the IA-64 Architecture fma

    // identify cases where at least one operand is NaN

    if (y.w[1] & MASK_NAN) == MASK_NAN { // y is NAN
        let mut y = *y;
        // if x = {0, f, inf, NaN}, y = NaN, z = {0, f, inf, NaN} then res = Q (y)
        // check first for non-canonical NaN payload
        if  ((y.w[1] & 0x00003fffffffffffu64)  > 0x0000314dc6448d93u64)
        || (((y.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (y.w[0] > 0x38c15b09ffffffffu64)) {
            y.w[1] &= 0xffffc00000000000u64;
            y.w[0]  = 0x0u64;
        }
        if (y.w[1] & MASK_SNAN) == MASK_SNAN { // y is SNAN
            // set invalid flag
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
            // return quiet (y)
            res.w[1] = y.w[1] & 0xfc003fffffffffffu64; // clear out also G[6]-G[16]
            res.w[0] = y.w[0];
        } else { // y is QNaN
            // return y
            res.w[1] = y.w[1] & 0xfc003fffffffffffu64; // clear out G[6]-G[16]
            res.w[0] = y.w[0];
            // if z = SNaN or x = SNaN signal invalid exception
            if (z.w[1] & MASK_SNAN) == MASK_SNAN || (x.w[1] & MASK_SNAN) == MASK_SNAN {
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
            }
        }

        *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    } else if (z.w[1] & MASK_NAN) == MASK_NAN { // z is NAN
        let mut z = *z;
        // if x = {0, f, inf, NaN}, y = {0, f, inf}, z = NaN then res = Q (z)
        // check first for non-canonical NaN payload
        if  ((z.w[1] & 0x00003fffffffffffu64)  > 0x0000314dc6448d93u64)
        || (((z.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (z.w[0] > 0x38c15b09ffffffffu64)) {
            z.w[1] &= 0xffffc00000000000u64;
            z.w[0]  = 0x0u64;
        }
        if (z.w[1] & MASK_SNAN) == MASK_SNAN { // z is SNAN
            // set invalid flag
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
            // return quiet (z)
            res.w[1] = z.w[1] & 0xfc003fffffffffffu64; // clear out also G[6]-G[16]
            res.w[0] = z.w[0];
        } else { // z is QNaN
            // return z
            res.w[1] = z.w[1] & 0xfc003fffffffffffu64; // clear out G[6]-G[16]
            res.w[0] = z.w[0];
            // if x = SNaN signal invalid exception
            if (x.w[1] & MASK_SNAN) == MASK_SNAN {
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
            }
        }
        *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    } else if (x.w[1] & MASK_NAN) == MASK_NAN { // x is NAN
        let mut x = *x;
        // if x = NaN, y = {0, f, inf}, z = {0, f, inf} then res = Q (x)
        // check first for non-canonical NaN payload
        if  ((x.w[1] & 0x00003fffffffffffu64)  > 0x0000314dc6448d93u64)
        || (((x.w[1] & 0x00003fffffffffffu64) == 0x0000314dc6448d93u64)
          && (x.w[0] > 0x38c15b09ffffffffu64)) {
            x.w[1] &= 0xffffc00000000000u64;
            x.w[0]  = 0x0u64;
        }
        if (x.w[1] & MASK_SNAN) == MASK_SNAN { // x is SNAN
            // set invalid flag
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
            // return quiet (x)
            res.w[1] = x.w[1] & 0xfc003fffffffffffu64; // clear out also G[6]-G[16]
            res.w[0] = x.w[0];
        } else { // x is QNaN
            // return x
            res.w[1] = x.w[1] & 0xfc003fffffffffffu64; // clear out G[6]-G[16]
            res.w[0] = x.w[0];
        }
        *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    }
    // x, y, z are 0, f, or inf but not NaN => unpack the arguments and check
    // for non-canonical values

    x_sign  = x.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];
    if (x.w[1] & MASK_ANY_INF) != MASK_INF { // x != inf
        // if x is not infinity check for non-canonical values - treated as zero
        if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 { // G0_G1=11
            // non-canonical
            x_exp   = (x.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
            C1.w[1] = 0; // significand high
            C1.w[0] = 0; // significand low
        } else { // G0_G1 != 11
            x_exp = x.w[1] & MASK_EXP; // biased and shifted left 49 bits
            if C1.w[1] > 0x0001ed09bead87c0u64 || (C1.w[1] == 0x0001ed09bead87c0u64 && C1.w[0] > 0x378d8e63ffffffffu64) {
                // x is non-canonical if coefficient is larger than 10^34 -1
                C1.w[1] = 0;
                C1.w[0] = 0;
            } else {
                // canonical
            }
        }
    }
    y_sign  = y.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
    C2.w[1] = y.w[1] & MASK_COEFF;
    C2.w[0] = y.w[0];
    if (y.w[1] & MASK_ANY_INF) != MASK_INF { // y != inf
        // if y is not infinity check for non-canonical values - treated as zero
        if (y.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 { // G0_G1=11
            // non-canonical
            y_exp   = (y.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
            C2.w[1] = 0; // significand high
            C2.w[0] = 0; // significand low
        } else { // G0_G1 != 11
            y_exp = y.w[1] & MASK_EXP; // biased and shifted left 49 bits
            if  C2.w[1]  > 0x0001ed09bead87c0u64
            || (C2.w[1] == 0x0001ed09bead87c0u64
             && C2.w[0]  > 0x378d8e63ffffffffu64) {
                // y is non-canonical if coefficient is larger than 10^34 -1
                C2.w[1] = 0;
                C2.w[0] = 0;
            } else {
                // canonical
            }
        }
    }
    z_sign  = z.w[1] & MASK_SIGN; // 0 for positive, MASK_SIGN for negative
    C3.w[1] = z.w[1] & MASK_COEFF;
    C3.w[0] = z.w[0];
    if (z.w[1] & MASK_ANY_INF) != MASK_INF { // z != inf
        // if z is not infinity check for non-canonical values - treated as zero
        if (z.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 { // G0_G1=11
            // non-canonical
            z_exp   = (z.w[1] << 2) & MASK_EXP; // biased and shifted left 49 bits
            C3.w[1] = 0; // significand high
            C3.w[0] = 0; // significand low
        } else { // G0_G1 != 11
            z_exp = z.w[1] & MASK_EXP; // biased and shifted left 49 bits
            if C3.w[1] > 0x0001ed09bead87c0u64 || (C3.w[1] == 0x0001ed09bead87c0u64 && C3.w[0] > 0x378d8e63ffffffffu64) {
                // z is non-canonical if coefficient is larger than 10^34 -1
                C3.w[1] = 0;
                C3.w[0] = 0;
            } else {
                // canonical
            }
        }
    }

    p_sign = x_sign ^ y_sign; // sign of the product

    // identify cases where at least one operand is infinity

    if (x.w[1] & MASK_ANY_INF) == MASK_INF { // x = inf
        if (y.w[1] & MASK_ANY_INF) == MASK_INF { // y = inf
            if (z.w[1] & MASK_ANY_INF) == MASK_INF { // z = inf
                if p_sign == z_sign {
                    res.w[1] = z_sign | MASK_INF;
                    res.w[0] = 0x0;
                } else {
                    // return QNaN Indefinite
                    res.w[1] = 0x7c00000000000000u64;
                    res.w[0] = 0x0000000000000000u64;
                    // set invalid flag
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                }
            } else { // z = 0 or z = f
                res.w[1] = p_sign | MASK_INF;
                res.w[0] = 0x0;
            }
        } else if C2.w[1] != 0 || C2.w[0] != 0 { // y = f
            if (z.w[1] & MASK_ANY_INF) == MASK_INF { // z = inf
                if p_sign == z_sign {
                    res.w[1] = z_sign | MASK_INF;
                    res.w[0] = 0x0;
                } else {
                    // return QNaN Indefinite
                    res.w[1] = 0x7c00000000000000u64;
                    res.w[0] = 0x0000000000000000u64;
                    // set invalid flag
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                }
            } else { // z = 0 or z = f
                res.w[1] = p_sign | MASK_INF;
                res.w[0] = 0x0;
            }
        } else { // y = 0
            // return QNaN Indefinite
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0x0000000000000000u64;
            // set invalid flag
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        }
        *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    } else if (y.w[1] & MASK_ANY_INF) == MASK_INF { // y = inf
        if (z.w[1] & MASK_ANY_INF) == MASK_INF { // z = inf
            // x = f, necessarily
            if (p_sign != z_sign) || (C1.w[1] == 0x0u64 && C1.w[0] == 0x0u64) {
                // return QNaN Indefinite
                res.w[1] = 0x7c00000000000000u64;
                res.w[0] = 0x0000000000000000u64;
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
            } else {
                res.w[1] = z_sign | MASK_INF;
                res.w[0] = 0x0;
            }
        } else if C1.w[1] == 0x0 && C1.w[0] == 0x0 { // x = 0
            // z = 0, f, inf
            // return QNaN Indefinite
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0x0000000000000000u64;
            // set invalid flag
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        } else {
            // x = f and z = 0, f, necessarily
            res.w[1] = p_sign | MASK_INF;
            res.w[0] = 0x0;
        }
        *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    } else if (z.w[1] & MASK_ANY_INF) == MASK_INF { // z = inf
        // x = 0, f and y = 0, f, necessarily
        res.w[1] = z_sign | MASK_INF;
        res.w[0] = 0x0;
        *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    }

    true_p_exp = (((x_exp >> 49) as BID_SINT64) - 6176 + ((y_exp >> 49) as BID_SINT64) - 6176) as i32;
    p_exp = if true_p_exp < -6176 {
        0 // cannot be less than EXP_MIN
    } else {
        ((true_p_exp + 6176) as BID_UINT64) << 49
    };

    if ((C1.w[1] == 0x0 && C1.w[0] == 0x0)
     || (C2.w[1] == 0x0 && C2.w[0] == 0x0))
      && C3.w[1] == 0x0 && C3.w[0] == 0x0 { // (x = 0 or y = 0) and z = 0
        // the result is 0
        res.w[1] = if p_exp < z_exp {
            p_exp // preferred exponent
        } else {
            z_exp // preferred exponent
        };
        if p_sign == z_sign {
            res.w[1] |= z_sign;
            res.w[0] = 0x0;
        } else { // x * y and z have opposite signs
            if rnd_mode == RoundingMode::BID_ROUNDING_DOWN {
                // res = -0.0
                res.w[1] |= MASK_SIGN;
                res.w[0] = 0x0;
            } else {
                // res = +0.0
                // res.w[1] |= 0x0;
                res.w[0] = 0x0;
            }
        }
        *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    }
    // from this point on, we may need to know the number of decimal digits
    // in the significands of x, y, z when x, y, z != 0

    unsafe {
        if C1.w[1] != 0 || C1.w[0] != 0 { // x = f (non-zero finite)
            // q1 = nr. of decimal digits in x
            // determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 { // x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp.d     = (C1.w[0] >> 32) as f64; // exact conversion
                    x_nr_bits = (33 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                } else { // if x < 2^53
                    tmp.d     = C1.w[0] as f64;         // exact conversion
                    x_nr_bits = (1 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                }
            } else { // C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                tmp.d     = C1.w[1] as f64;           // exact conversion
                x_nr_bits = (65 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            }
            q1 = bid_nr_digits[(x_nr_bits - 1) as usize].digits as i32;
            if q1 == 0 {
                q1 = bid_nr_digits[(x_nr_bits - 1) as usize].digits1 as i32;
                if C1.w[1]  > bid_nr_digits[(x_nr_bits - 1) as usize].threshold_hi
               || (C1.w[1] == bid_nr_digits[(x_nr_bits - 1) as usize].threshold_hi
                && C1.w[0] >= bid_nr_digits[(x_nr_bits - 1) as usize].threshold_lo) {
                    q1 += 1;
                }
            }
        }

        if C2.w[1] != 0 || C2.w[0] != 0 { // y = f (non-zero finite)
            // q2 = nr. of decimal digits in y
            // determine first the nr. of bits in y
            if C2.w[1] == 0 {
                if C2.w[0] >= 0x0020000000000000u64 { // y >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp.d     = (C2.w[0] >> 32) as f64; // exact conversion
                    y_nr_bits = (33 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                } else { // if y < 2^53
                    tmp.d     = C2.w[0] as f64;         // exact conversion
                    y_nr_bits = (1 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                }
            } else { // C2.w[1] != 0 => nr. bits = 64 + nr_bits (C2.w[1])
                tmp.d     = C2.w[1] as f64;           // exact conversion
                y_nr_bits = (65 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            }
            q2 = bid_nr_digits[(y_nr_bits - 1) as usize].digits as i32;
            if q2 == 0 {
                q2 = bid_nr_digits[(y_nr_bits - 1) as usize].digits1 as i32;
                if  C2.w[1]  > bid_nr_digits[(y_nr_bits - 1) as usize].threshold_hi
                || (C2.w[1] == bid_nr_digits[(y_nr_bits - 1) as usize].threshold_hi
                 && C2.w[0] >= bid_nr_digits[(y_nr_bits - 1) as usize].threshold_lo) {
                    q2 += 1;
                }
            }
        }

        if C3.w[1] != 0 || C3.w[0] != 0 { // z = f (non-zero finite)
            // q3 = nr. of decimal digits in z
            // determine first the nr. of bits in z
            if C3.w[1] == 0 {
                if C3.w[0] >= 0x0020000000000000u64 { // z >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp.d     = (C3.w[0] >> 32) as f64; // exact conversion
                    z_nr_bits = (33 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                } else { // if z < 2^53
                    tmp.d     = C3.w[0] as f64;         // exact conversion
                    z_nr_bits = (1 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
                }
            } else { // C3.w[1] != 0 => nr. bits = 64 + nr_bits (C3.w[1])
                tmp.d     = C3.w[1] as f64;          // exact conversion
                z_nr_bits = (65 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            }
            q3 = bid_nr_digits[(z_nr_bits - 1) as usize].digits as i32;
            if q3 == 0 {
                q3 = bid_nr_digits[(z_nr_bits - 1) as usize].digits1 as i32;
                if C3.w[1]  > bid_nr_digits[(z_nr_bits - 1) as usize].threshold_hi
               || (C3.w[1] == bid_nr_digits[(z_nr_bits - 1) as usize].threshold_hi
                && C3.w[0] >= bid_nr_digits[(z_nr_bits - 1) as usize].threshold_lo) {
                    q3 += 1;
                }
            }
        }
    }

    if (C1.w[1] == 0x0 && C1.w[0] == 0x0)
    || (C2.w[1] == 0x0 && C2.w[0] == 0x0) {
        // x = 0 or y = 0
        // z = f, necessarily; for 0 + z return z, with the preferred exponent
        // the result is z, but need to get the preferred exponent
        if z_exp <= p_exp { // the preferred exponent is z_exp
            res.w[1] = z_sign | (z_exp & MASK_EXP) | C3.w[1];
            res.w[0] = C3.w[0];
        } else { // if (p_exp < z_exp) the preferred exponent is p_exp
            // return (C3 * 10^scale) * 10^(z_exp - scale)
            // where scale = min (p34-q3, (z_exp-p_exp) >> 49)
            scale = p34 - q3;
            ind   = ((z_exp - p_exp) >> 49) as i32;
            if ind < scale {
                scale = ind;
            }
            if scale == 0 {
                res.w[1] = z.w[1];  // & MASK_COEFF, which is redundant
                res.w[0] = z.w[0];
            } else if q3 <= 19 {  // z fits in 64 bits
                if scale <= 19 {    // 10^scale fits in 64 bits
                    // 64 x 64 C3.w[0] * bid_ten2k64[scale as usize]
                    res = __mul_64x64_to_128MACH(C3.w[0], bid_ten2k64[scale as usize]);
                } else { // 10^scale fits in 128 bits
                    // 64 x 128 C3.w[0] * bid_ten2k128[(scale - 20) as usize]
                    res = __mul_128x64_to_128(C3.w[0], &bid_ten2k128[(scale - 20) as usize]);
                }
            } else { // z fits in 128 bits, but 10^scale must fit in 64 bits
                // 64 x 128 bid_ten2k64[scale as usize] * C3
                res = __mul_128x64_to_128(bid_ten2k64[scale as usize], &C3);
            }
            // subtract scale from the exponent
            z_exp    -= (scale as BID_UINT64) << 49;
            res.w[1] |= z_sign | (z_exp & MASK_EXP);
        }
        *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

        #[cfg(target_endian = "big")]
        BID_SWAP128(& mut res);

        return res;
    } else {
        // continue with x = f, y = f, z = 0 or x = f, y = f, z = f
    }

    e1 = (((x_exp >> 49) as BID_SINT64) - 6176) as i32; // unbiased exponent of x
    e2 = (((y_exp >> 49) as BID_SINT64) - 6176) as i32; // unbiased exponent of y
    e3 = (((z_exp >> 49) as BID_SINT64) - 6176) as i32; // unbiased exponent of z
    e4 = e1 + e2;              // unbiased exponent of the exact x * y

    // calculate C1 * C2 and its number of decimal digits, q4

    // the exact product has either q1 + q2 - 1 or q1 + q2 decimal digits
    // where 2 <= q1 + q2 <= 68
    // calculate C4 = C1 * C2 and determine q
    C4.w[3] = 0;
    C4.w[2] = 0;
    C4.w[1] = 0;
    C4.w[0] = 0;
    if q1 + q2 <= 19 { // if 2 <= q1 + q2 <= 19, C4 = C1 * C2 fits in 64 bits
        C4.w[0] = C1.w[0] * C2.w[0];
        // if C4 < 10^(q1+q2-1) then q4 = q1 + q2 - 1 else q4 = q1 + q2
        q4 = if C4.w[0] < bid_ten2k64[(q1 + q2 - 1) as usize] {
            q1 + q2 - 1 // q4 in [1, 18]
        } else {
            q1 + q2     // q4 in [2, 19]
        };
        // length of C1 * C2 rounded up to a multiple of 64 bits is len = 64;
    } else if q1 + q2 == 20 { // C4 = C1 * C2 fits in 64 or 128 bits
        // q1 <= 19 and q2 <= 19 so both C1 and C2 fit in 64 bits
        let tmp: BID_UINT128 = __mul_64x64_to_128MACH(C1.w[0], C2.w[0]);
        C4.w[0] = tmp.w[0];
        C4.w[1] = tmp.w[1];
        // if C4 < 10^(q1+q2-1) = 10^19 then q4 = q1+q2-1 = 19 else q4 = q1+q2 = 20
        q4 = if C4.w[1] == 0 && C4.w[0] < bid_ten2k64[19] { // 19 = q1+q2-1
            // length of C1 * C2 rounded up to a multiple of 64 bits is len = 64;
            19 // 19 = q1 + q2 - 1
        } else {
            // if (C4.w[1] == 0)
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 64;
            // else
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 128;
            20 // 20 = q1 + q2
        };
    } else if q1 + q2 <= 38 { // 21 <= q1 + q2 <= 38
        // C4 = C1 * C2 fits in 64 or 128 bits
        // (64 bits possibly, but only when q1 + q2 = 21 and C4 has 20 digits)
        // at least one of C1, C2 has at most 19 decimal digits & fits in 64 bits
        if q1 <= 19 {
            let tmp: BID_UINT128 = __mul_128x64_to_128(C1.w[0], &C2);
            C4.w[0] = tmp.w[0];
            C4.w[1] = tmp.w[1];
        } else { // q2 <= 19
            let tmp: BID_UINT128 = __mul_128x64_to_128(C2.w[0], &C1);
            C4.w[0] = tmp.w[0];
            C4.w[1] = tmp.w[1];
        }
        // if C4 < 10^(q1+q2-1) then q4 = q1 + q2 - 1 else q4 = q1 + q2
        q4 = if C4.w[1]  < bid_ten2k128[(q1 + q2 - 21) as usize].w[1]
            || (C4.w[1] == bid_ten2k128[(q1 + q2 - 21) as usize].w[1]
             && C4.w[0]  < bid_ten2k128[(q1 + q2 - 21) as usize].w[0]) {
            // if (C4.w[1] == 0) // q4 = 20, necessarily
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 64;
            // else
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 128;
            q1 + q2 - 1 // q4 in [20, 37]
        } else {
            // length of C1 * C2 rounded up to a multiple of 64 bits is len = 128;
            q1 + q2 // q4 in [21, 38]
        };
    } else if q1 + q2 == 39 { // C4 = C1 * C2 fits in 128 or 192 bits
        // both C1 and C2 fit in 128 bits (actually in 113 bits)
        // may replace this by 128x128_to192
        C4 = __mul_128x128_to_256(&C1, &C2); // C4.w[3] is 0
        // if C4 < 10^(q1+q2-1) = 10^38 then q4 = q1+q2-1 = 38 else q4 = q1+q2 = 39
        q4 = if C4.w[2] == 0
            && (C4.w[1]  < bid_ten2k128[18].w[1]
            || (C4.w[1] == bid_ten2k128[18].w[1]
             && C4.w[0]  < bid_ten2k128[18].w[0])) {
            // 18 = 38 - 20 = q1+q2-1 - 20
            // length of C1 * C2 rounded up to a multiple of 64 bits is len = 128;
            38 // 38 = q1 + q2 - 1
        } else {
            // if (C4.w[2] == 0)
            // length of C1 * C2 rounded up to a multiple of 64 bits is len = 128;
            // else
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 192;
            39 // 39 = q1 + q2
        };
    } else if q1 + q2 <= 57 { // 40 <= q1 + q2 <= 57
        // C4 = C1 * C2 fits in 128 or 192 bits
        // (128 bits possibly, but only when q1 + q2 = 40 and C4 has 39 digits)
        // both C1 and C2 fit in 128 bits (actually in 113 bits); at most one
        // may fit in 64 bits
        if C1.w[1] == 0 { // C1 fits in 64 bits
            // __mul_64x128_full (REShi64, RESlo128, A64, B128)
            let (a, b) = __mul_64x128_full(C1.w[0], &C2);
            C4.w[0] = b.w[0];
            C4.w[1] = b.w[1];
            C4.w[2] = a;
        } else if C2.w[1] == 0 { // C2 fits in 64 bits
            // __mul_64x128_full (REShi64, RESlo128, A64, B128)
            let (a, b) = __mul_64x128_full(C2.w[0], &C1);
            C4.w[0] = b.w[0];
            C4.w[1] = b.w[1];
            C4.w[2] = a;
        } else { // both C1 and C2 require 128 bits
            // may use __mul_128x128_to_192 (C4.w[2], C4.w[0], C2.w[0], C1);
            C4 = __mul_128x128_to_256(&C1, &C2); // C4.w[3] = 0
        }
        // if C4 < 10^(q1+q2-1) then q4 = q1 + q2 - 1 else q4 = q1 + q2
        q4 = if C4.w[2]  < bid_ten2k256[(q1 + q2 - 40) as usize].w[2]
            || (C4.w[2] == bid_ten2k256[(q1 + q2 - 40) as usize].w[2]
            && (C4.w[1]  < bid_ten2k256[(q1 + q2 - 40) as usize].w[1]
            || (C4.w[1] == bid_ten2k256[(q1 + q2 - 40) as usize].w[1]
             && C4.w[0]  < bid_ten2k256[(q1 + q2 - 40) as usize].w[0]))) {
            // if (C4.w[2] == 0) // q4 = 39, necessarily
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 128;
            // else
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 192;
            q1 + q2 - 1 // q4 in [39, 56]
        } else {
            // length of C1 * C2 rounded up to a multiple of 64 bits is len = 192;
            q1 + q2 // q4 in [40, 57]
        };
    } else if q1 + q2 == 58 { // C4 = C1 * C2 fits in 192 or 256 bits;
        // both C1 and C2 fit in 128 bits (actually in 113 bits); none can
        // fit in 64 bits, because each number must have at least 24 decimal
        // digits for the sum to have 58 (as the max. nr. of digits is 34) =>
        // C1.w[1] != 0 and C2.w[1] != 0
        C4 = __mul_128x128_to_256(&C1, &C2);
        // if C4 < 10^(q1+q2-1) = 10^57 then q4 = q1+q2-1 = 57 else q4 = q1+q2 = 58
        q4 = if C4.w[3] == 0
            && (C4.w[2]  < bid_ten2k256[18].w[2]
            || (C4.w[2] == bid_ten2k256[18].w[2]
            && (C4.w[1]  < bid_ten2k256[18].w[1]
            || (C4.w[1] == bid_ten2k256[18].w[1]
            && C4.w[0]   < bid_ten2k256[18].w[0])))) {
            // 18 = 57 - 39 = q1+q2-1 - 39
            // length of C1 * C2 rounded up to a multiple of 64 bits is len = 192;
            57 // 57 = q1 + q2 - 1
        } else {
            // if (C4.w[3] == 0)
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 192;
            // else
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 256;
            58 // 58 = q1 + q2
        };
    } else { // if 59 <= q1 + q2 <= 68
        // C4 = C1 * C2 fits in 192 or 256 bits
        // (192 bits possibly, but only when q1 + q2 = 59 and C4 has 58 digits)
        // both C1 and C2 fit in 128 bits (actually in 113 bits); none fits in
        // 64 bits
        // may use __mul_128x128_to_192 (C4.w[2], C4.w[0], C2.w[0], C1);
        C4 = __mul_128x128_to_256(&C1, &C2); // C4.w[3] = 0
        // if C4 < 10^(q1+q2-1) then q4 = q1 + q2 - 1 else q4 = q1 + q2
        q4 = if C4.w[3]  < bid_ten2k256[(q1 + q2 - 40) as usize].w[3]
            || (C4.w[3] == bid_ten2k256[(q1 + q2 - 40) as usize].w[3]
            && (C4.w[2]  < bid_ten2k256[(q1 + q2 - 40) as usize].w[2]
            || (C4.w[2] == bid_ten2k256[(q1 + q2 - 40) as usize].w[2]
            && (C4.w[1]  < bid_ten2k256[(q1 + q2 - 40) as usize].w[1]
            || (C4.w[1] == bid_ten2k256[(q1 + q2 - 40) as usize].w[1]
            &&  C4.w[0]  < bid_ten2k256[(q1 + q2 - 40) as usize].w[0]))))) {
            // if (C4.w[3] == 0) // q4 = 58, necessarily
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 192;
            // else
            //   length of C1 * C2 rounded up to a multiple of 64 bits is len = 256;
            q1 + q2 - 1 // q4 in [58, 67]
        } else {
            // length of C1 * C2 rounded up to a multiple of 64 bits is len = 256;
            q1 + q2     // q4 in [59, 68]
        };
    }

    if C3.w[1] == 0x0 && C3.w[0] == 0x0 { // x = f, y = f, z = 0
        save_fpsf = *pfpsf; // sticky bits - caller value must be preserved
        *pfpsf    = 0;

        if q4 > p34 {
            // truncate C4 to p34 digits into res
            // x = q4-p34, 1 <= x <= 34 because 35 <= q4 <= 68
            x0 = q4 - p34;
            if q4 <= 38 {
                P128.w[1] = C4.w[1];
                P128.w[0] = C4.w[0];
                res = bid_round128_19_38(
                    q4, x0, &P128,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
            } else if q4 <= 57 { // 35 <= q4 <= 57
                P192.w[2] = C4.w[2];
                P192.w[1] = C4.w[1];
                P192.w[0] = C4.w[0];
                R192 = bid_round192_39_57(
                    q4, x0, &P192,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                res.w[0] = R192.w[0];
                res.w[1] = R192.w[1];
            } else { // if (q4 <= 68)
                R256 = bid_round256_58_76(
                    q4, x0, &C4,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                res.w[0] = R256.w[0];
                res.w[1] = R256.w[1];
            }
            e4 += x0;
            q4  = p34;
            if incr_exp {
                e4 += 1;

                #[cfg(not(feature = "DECIMAL_TINY_DETECTION_AFTER_ROUNDING"))]
                if q4 + e4 == EXP_MIN_UNBIASED + p34 {
                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_UNDERFLOW_EXCEPTION;
                }
            }
            // res is now the coefficient of the result rounded to the destination
            // precision, with unbounded exponent; the exponent is e4; q4=digits(res)
        } else { // if (q4 <= p34)
            // C4 * 10^e4 is the result rounded to the destination precision, with
            // unbounded exponent (which is exact)

            if (q4 + e4 <= p34 + EXP_MAX_UNBIASED) && (e4 > EXP_MAX_UNBIASED) {
                // e4 is too large, but can be brought within range by scaling up C4
                scale = e4 - EXP_MAX_UNBIASED; // 1 <= scale < P-q4 <= P-1 => 1 <= scale <= P-2
                // res = (C4 * 10^scale) * 10^expmax
                if q4 <= 19 { // C4 fits in 64 bits
                    if scale <= 19 { // 10^scale fits in 64 bits
                        // 64 x 64 C4.w[0] * bid_ten2k64[scale as usize]
                        res = __mul_64x64_to_128MACH(C4.w[0], bid_ten2k64[scale as usize]);
                    } else { // 10^scale fits in 128 bits
                        // 64 x 128 C4.w[0] * bid_ten2k128[(scale - 20) as usize]
                        res = __mul_128x64_to_128(C4.w[0], &bid_ten2k128[(scale - 20) as usize]);
                    }
                } else { // C4 fits in 128 bits, but 10^scale must fit in 64 bits
                    // 64 x 128 bid_ten2k64[scale as usize] * CC43
                    let tmp = BID_UINT128 { w: [C4.w[0], C4.w[1]] };
                    res = __mul_128x64_to_128(bid_ten2k64[scale as usize], &tmp);
                }
                e4 -= scale; // expmax
                q4 += scale;
            } else {
                res.w[1] = C4.w[1];
                res.w[0] = C4.w[0];
            }
            // res is the coefficient of the result rounded to the destination
            // precision, with unbounded exponent (it has q4 digits); the exponent
            // is e4 (exact result)
        }

        // check for overflow
        if q4 + e4 > p34 + EXP_MAX_UNBIASED {
            if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST {
                res.w[1] = p_sign | 0x7800000000000000u64; // +/-inf
                res.w[0] = 0x0000000000000000u64;
                *pfpsf  |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
            } else {
                res.w[1] |= p_sign;
                bid_rounding_correction(
                    rnd_mode,
                    is_inexact_lt_midpoint,
                    is_inexact_gt_midpoint,
                    is_midpoint_lt_even,
                    is_midpoint_gt_even,
                    e4, &mut res, pfpsf);
            }
            *pfpsf                     |= save_fpsf;
            *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            return res;
        }
        // check for underflow
        if q4 + e4 < EXP_MIN_UNBIASED + p34 {
            is_tiny = true; // the result is tiny
            // (good also for most cases if 'before rounding')
            if e4 < EXP_MIN_UNBIASED {
                // if e4 < expmin, we must truncate more of res
                x0                      = EXP_MIN_UNBIASED - e4; // x0 >= 1
                is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
                is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
                is_midpoint_lt_even0    = is_midpoint_lt_even;
                is_midpoint_gt_even0    = is_midpoint_gt_even;
                is_inexact_lt_midpoint  = false;
                is_inexact_gt_midpoint  = false;
                is_midpoint_lt_even     = false;
                is_midpoint_gt_even     = false;
                // the number of decimal digits in res is q4
                if x0 < q4 { // 1 <= x0 <= q4-1 => round res to q4 - x0 digits
                    if q4 <= 18 { // 2 <= q4 <= 18, 1 <= x0 <= 17
                        R64 = bid_round64_2_18(
                            q4, x0, res.w[0],
                            &mut incr_exp,
                            &mut is_midpoint_lt_even,
                            &mut is_midpoint_gt_even,
                            &mut is_inexact_lt_midpoint,
                            &mut is_inexact_gt_midpoint);

                        if incr_exp {
                            // R64 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 1, 1 <= q4 - x0 <= 17
                            R64 = bid_ten2k64[(q4 - x0) as usize];
                        }
                        // res.w[1] = 0; (from above)
                        res.w[0] = R64;
                    } else { // if (q4 <= 34)
                        // 19 <= q4 <= 38
                        P128.w[1] = res.w[1];
                        P128.w[0] = res.w[0];
                        res = bid_round128_19_38(
                            q4, x0, &P128,
                            &mut incr_exp,
                            &mut is_midpoint_lt_even,
                            &mut is_midpoint_gt_even,
                            &mut is_inexact_lt_midpoint,
                            &mut is_inexact_gt_midpoint);
                        if incr_exp {
                            // increase coefficient by a factor of 10; this will be <= 10^33
                            // R128 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 1, 1 <= q4 - x0 <= 37
                            if q4 - x0 <= 19 { // 1 <= q4 - x0 <= 19
                                // res.w[1] = 0;
                                res.w[0] = bid_ten2k64[(q4 - x0) as usize];
                            } else { // 20 <= q4 - x0 <= 37
                                res.w[0] = bid_ten2k128[(q4 - x0 - 20) as usize].w[0];
                                res.w[1] = bid_ten2k128[(q4 - x0 - 20) as usize].w[1];
                            }
                        }
                    }
                    e4 += x0; // expmin
                } else if x0 == q4 {
                    // the second rounding is for 0.d(0)d(1)...d(q4-1) * 10^emin
                    // determine relationship with 1/2 ulp
                    if q4 <= 19 {
                        match bid_midpoint64[(q4 - 1) as usize] {
                            val if res.w[0] < val => { // < 1/2 ulp
                                lt_half_ulp            = true;
                                is_inexact_lt_midpoint = true;
                            },
                            val if res.w[0] == val => { // = 1/2 ulp
                                eq_half_ulp         = true;
                                is_midpoint_gt_even = true;
                            },
                            _ =>  {                                 // > 1/2 ulp
                                // gt_half_ulp = 1;
                                is_inexact_gt_midpoint = true;
                            }
                        }
                    } else { // if (q4 <= 34)
                        if res.w[1]  < bid_midpoint128[(q4 - 20) as usize].w[1]
                       || (res.w[1] == bid_midpoint128[(q4 - 20) as usize].w[1]
                        && res.w[0]  < bid_midpoint128[(q4 - 20) as usize].w[0]) { // < 1/2 ulp
                            lt_half_ulp            = true;
                            is_inexact_lt_midpoint = true;
                        } else if res.w[1] == bid_midpoint128[(q4 - 20) as usize].w[1]
                               && res.w[0] == bid_midpoint128[(q4 - 20) as usize].w[0] { // = 1/2 ulp
                            eq_half_ulp         = true;
                            is_midpoint_gt_even = true;
                        } else { // > 1/2 ulp
                            // gt_half_ulp = 1;
                            is_inexact_gt_midpoint = true;
                        }
                    }
                    if lt_half_ulp || eq_half_ulp {
                        // res = +0.0 * 10^expmin
                        res.w[1] = 0x0000000000000000u64;
                        res.w[0] = 0x0000000000000000u64;
                    } else { // if (gt_half_ulp)
                        // res = +1 * 10^expmin
                        res.w[1] = 0x0000000000000000u64;
                        res.w[0] = 0x0000000000000001u64;
                    }
                    e4 = EXP_MIN_UNBIASED;
                } else { // if (x0 > q4)
                    // the second rounding is for 0.0...d(0)d(1)...d(q4-1) * 10^emin
                    res.w[1] = 0;
                    res.w[0] = 0;
                    e4       = EXP_MIN_UNBIASED;
                    is_inexact_lt_midpoint = true;
                }
                // avoid a double rounding error
                if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even { // double rounding error upward
                    // res = res - 1
                    res.w[0] -= 1;
                    if res.w[0] == 0xffffffffffffffffu64 {
                        res.w[1] -= 1;
                    }
                    // Note: a double rounding error upward is not possible; for this
                    // the result after the first rounding would have to be 99...95
                    // (35 digits in all), possibly followed by a number of zeros; this
                    // not possible for f * f + 0
                    is_midpoint_lt_even    = false;
                    is_inexact_lt_midpoint = true;
                } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even { // double rounding error downward
                    // res = res + 1
                    res.w[0] += 1;
                    if res.w[0] == 0 {
                        res.w[1] += 1;
                    }
                    is_midpoint_gt_even    = false;
                    is_inexact_gt_midpoint = true;
                } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                    // if this second rounding was exact the result may still be
                    // inexact because of the first rounding
                    if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
                        is_inexact_gt_midpoint = true;
                    }
                    if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
                        is_inexact_lt_midpoint = true;
                    }
                } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
                    // pu64ed up to a midpoint
                    is_inexact_lt_midpoint = true;
                    is_inexact_gt_midpoint = false;
                    is_midpoint_lt_even    = false;
                    is_midpoint_gt_even    = false;
                } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
                    // pu64ed down to a midpoint
                    is_inexact_lt_midpoint = false;
                    is_inexact_gt_midpoint = true;
                    is_midpoint_lt_even    = false;
                    is_midpoint_gt_even    = false;
                } else {
                    // ;
                }
            } else { // if e4 >= emin then q4 < P and the result is tiny and exact
                if e3 < e4 {
                    // if (e3 < e4) the preferred exponent is e3
                    // return (C4 * 10^scale) * 10^(e4 - scale)
                    // where scale = min (p34-q4, (e4 - e3))
                    scale = p34 - q4;
                    ind   = e4 - e3;
                    if ind < scale {
                        scale = ind;
                    }
                    if scale == 0 {
                        // res and e4 are unchanged
                    } else if q4 <= 19 { // C4 fits in 64 bits
                        if scale <= 19 { // 10^scale fits in 64 bits
                            // 64 x 64 res.w[0] * bid_ten2k64[scale as usize]
                            res = __mul_64x64_to_128MACH(res.w[0], bid_ten2k64[scale as usize]);
                        } else { // 10^scale fits in 128 bits
                            // 64 x 128 res.w[0] * bid_ten2k128[(scale - 20) as usize]
                            res = __mul_128x64_to_128(res.w[0], &bid_ten2k128[(scale - 20) as usize]);
                        }
                    } else { // res fits in 128 bits, but 10^scale must fit in 64 bits
                        // 64 x 128 bid_ten2k64[scale as usize] * C3
                        res = __mul_128x64_to_128(bid_ten2k64[scale as usize], &res);
                    }
                    // subtract scale from the exponent
                    e4 -= scale;
                }
            }

            // check for inexact result
            if is_inexact_lt_midpoint || is_inexact_gt_midpoint || is_midpoint_lt_even || is_midpoint_gt_even {
                // set the inexact flag and the underflow flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
            }
            res.w[1] |= p_sign | (((e4 + 6176) as BID_UINT64) << 49);
            if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                bid_rounding_correction(
                    rnd_mode,
                    is_inexact_lt_midpoint,
                    is_inexact_gt_midpoint,
                    is_midpoint_lt_even,
                    is_midpoint_gt_even,
                    e4, &mut res, pfpsf);
            }
            *pfpsf                     |= save_fpsf;
            *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            return res;
        }
        // no overflow, and no underflow for rounding to nearest
        // (although if tininess is detected 'before rounding', we may
        // get here if incr_exp = 1 and then q4 + e4 == expmin + p34)
        res.w[1] |= p_sign | (((e4 + 6176) as BID_UINT64) << 49);

        if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
            bid_rounding_correction(
                rnd_mode,
                is_inexact_lt_midpoint,
                is_inexact_gt_midpoint,
                is_midpoint_lt_even,
                is_midpoint_gt_even,
                e4, &mut res, pfpsf);
            // if e4 = expmin && significand < 10^33 => result is tiny (for RD, RZ)
            if e4 == EXP_MIN_UNBIASED {
                if  (res.w[1] & MASK_COEFF)  < 0x0000314dc6448d93u64
                || ((res.w[1] & MASK_COEFF) == 0x0000314dc6448d93u64
                  && res.w[0]                < 0x38c15b0a00000000u64) {
                    is_tiny = true;
                }
            }
        }

        if is_inexact_lt_midpoint || is_inexact_gt_midpoint || is_midpoint_lt_even || is_midpoint_gt_even {
            // set the inexact flag
            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            if is_tiny {
                *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
            }
        }

        if (*pfpsf & StatusFlags::BID_INEXACT_EXCEPTION) == 0 { // x * y is exact
            // need to ensure that the result has the preferred exponent
            p_exp = res.w[1] & MASK_EXP;
            if z_exp < p_exp { // the preferred exponent is z_exp
                // signficand of res in C3
                C3.w[1] = res.w[1] & MASK_COEFF;
                C3.w[0] = res.w[0];
                // the number of decimal digits of x * y is q4 <= 34
                // Note: the coefficient fits in 128 bits

                // return (C3 * 10^scale) * 10^(p_exp - scale)
                // where scale = min (p34-q4, (p_exp-z_exp) >> 49)
                scale = p34 - q4;
                ind   = ((p_exp - z_exp) >> 49) as i32;
                if ind < scale {
                    scale = ind;
                }
                // subtract scale from the exponent
                p_exp -= (scale as BID_UINT64) << 49;
                if scale == 0 {
                    // leave res unchanged
                } else if q4 <= 19 { // x * y fits in 64 bits
                    res = if scale <= 19 { // 10^scale fits in 64 bits
                        // 64 x 64 C3.w[0] * bid_ten2k64[scale as usize]
                        __mul_64x64_to_128MACH(C3.w[0], bid_ten2k64[scale as usize])
                    } else { // 10^scale fits in 128 bits
                        // 64 x 128 C3.w[0] * bid_ten2k128[(scale - 20) as usize]
                        __mul_128x64_to_128(C3.w[0], &bid_ten2k128[(scale - 20) as usize])
                    };
                    res.w[1] |= p_sign | (p_exp & MASK_EXP);
                } else { // x * y fits in 128 bits, but 10^scale must fit in 64 bits
                    // 64 x 128 bid_ten2k64[scale as usize] * C3
                    res       = __mul_128x64_to_128(bid_ten2k64[scale as usize], &C3);
                    res.w[1] |= p_sign | (p_exp & MASK_EXP);
                }
            } // else leave the result as it is, because p_exp <= z_exp
        }
        *pfpsf                      |= save_fpsf;
        *ptr_is_midpoint_lt_even     = is_midpoint_lt_even;
        *ptr_is_midpoint_gt_even     = is_midpoint_gt_even;
        *ptr_is_inexact_lt_midpoint  = is_inexact_lt_midpoint;
        *ptr_is_inexact_gt_midpoint  = is_inexact_gt_midpoint;

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut res);

        return res;
    } // else we have f * f + f

    // continue with x = f, y = f, z = f

    delta = q3 + e3 - q4 - e4;

// TODO: Try to ge around C goto
// delta_ge_zero:
    if delta >= 0 {
        return if  p34 <= delta - 1 	                   // Case (1')
               || (p34 == delta && e3 + 6176 < p34 - q3) { // Case (1''A)
            // check for overflow, which can occur only in Case (1')
            if (q3 + e3) > (p34 + EXP_MAX_UNBIASED) && p34 <= delta - 1 {
                // e3 > expmax implies p34 <= delta-1 and e3 > expmax is a necessary
                // condition for (q3 + e3) > (p34 + expmax)
                if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST {
                    res.w[1] = z_sign | 0x7800000000000000u64; // +/-inf
                    res.w[0] = 0x0000000000000000u64;
                    *pfpsf  |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
                } else {
                    if p_sign == z_sign {
                        is_inexact_lt_midpoint = true;
                    } else {
                        is_inexact_gt_midpoint = true;
                    }
                    // q3 <= p34; if (q3 < p34) scale C3 up by 10^(p34-q3)
                    scale = p34 - q3;
                    if scale == 0 {
                        res.w[1] = z_sign | C3.w[1];
                        res.w[0] = C3.w[0];
                    } else {
                        res = if q3 <= 19 { // C3 fits in 64 bits
                            if scale <= 19 { // 10^scale fits in 64 bits
                                // 64 x 64 C3.w[0] * bid_ten2k64[scale as usize]
                                __mul_64x64_to_128MACH(C3.w[0], bid_ten2k64[scale as usize])
                            } else { // 10^scale fits in 128 bits
                                // 64 x 128 C3.w[0] * bid_ten2k128[(scale - 20) as usize]
                                __mul_128x64_to_128(C3.w[0], &bid_ten2k128[(scale - 20) as usize])
                            }
                        } else { // C3 fits in 128 bits, but 10^scale must fit in 64 bits
                            // 64 x 128 bid_ten2k64[scale as usize] * C3
                            __mul_128x64_to_128(bid_ten2k64[scale as usize], &C3)
                        }
                        // the coefficient in res has q3 + scale = p34 digits
                    }
                    e3       -= scale;
                    res.w[1] |= z_sign;
                    bid_rounding_correction(
                        rnd_mode,
                        is_inexact_lt_midpoint,
                        is_inexact_gt_midpoint,
                        is_midpoint_lt_even,
                        is_midpoint_gt_even,
                        e3, &mut res, pfpsf);
                }
                *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
                *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
                *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

                #[cfg(target_endian = "big")]
                BID_SWAP128(&mut res);
                return res;
            }
            // res = z
            if q3 < p34 { // the preferred exponent is z_exp - (p34 - q3)
                // return (C3 * 10^scale) * 10^(z_exp - scale)
                // where scale = min (p34-q3, z_exp-EMIN)
                scale = p34 - q3;
                ind   = e3 + 6176;
                if ind < scale {
                    scale = ind;
                }
                if scale == 0 {
                    res.w[1] = C3.w[1];
                    res.w[0] = C3.w[0];
                } else if q3 <= 19 { // z fits in 64 bits
                    res = if scale <= 19 { // 10^scale fits in 64 bits
                        // 64 x 64 C3.w[0] * bid_ten2k64[scale as usize]
                        __mul_64x64_to_128MACH(C3.w[0], bid_ten2k64[scale as usize])
                    } else { // 10^scale fits in 128 bits
                        // 64 x 128 C3.w[0] * bid_ten2k128[(scale - 20) as usize]
                        __mul_128x64_to_128(C3.w[0], &bid_ten2k128[(scale - 20) as usize])
                    };
                } else { // z fits in 128 bits, but 10^scale must fit in 64 bits
                    // 64 x 128 bid_ten2k64[scale as usize] * C3
                    res = __mul_128x64_to_128(bid_ten2k64[scale as usize], &C3);
                }
                // the coefficient in res has q3 + scale digits
                // subtract scale from the exponent
                z_exp    -= (scale as BID_UINT64) << 49;
                e3       -= scale;
                res.w[1] |= z_sign | (z_exp & MASK_EXP);
                if scale + q3 < p34 {
                    *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;  // OK for tininess detection
                    // before or after rounding, because the exponent of the
                    // rounded result with unbounded exponent does not change
                    // due to rounding overflow
                }
            } else { // if q3 = p34
                scale    = 0;
                res.w[1] = z_sign | (((e3 + 6176) as BID_UINT64) << 49) | C3.w[1];
                res.w[0] = C3.w[0];
            }

            // use the following to avoid double rounding errors when operating on
            // mixed formats in rounding to nearest, and for correcting the result
            // if not rounding to nearest
            if (p_sign != z_sign) && (delta == (q3 + scale + 1)) {
                // there is a gap of exactly one digit between the scaled C3 and C4
                // C3 * 10^ scale = 10^(q3+scale-1) <=> C3 = 10^(q3-1) is a special case
                if (q3 <= 19 && C3.w[0] != bid_ten2k64[(q3 - 1) as usize])
                || (q3 == 20 && (C3.w[1] != 0 || C3.w[0] != bid_ten2k64[19]))
                || (q3 >= 21 && (C3.w[1] != bid_ten2k128[(q3 - 21) as usize].w[1]
                 || C3.w[0] != bid_ten2k128[(q3 - 21) as usize].w[0])) {
                    // C3 * 10^ scale != 10^(q3-1)
                    // if ((res.w[1] & MASK_COEFF) != 0x0000314dc6448d93u64 ||
                    // res.w[0] != 0x38c15b0a00000000u64) { // C3 * 10^scale != 10^33
                    is_inexact_gt_midpoint = true; // if (z_sign), set as if for abs. value
                } else { // if C3 * 10^scale = 10^(q3+scale-1)
                    // ok from above e3 = (z_exp >> 49) - 6176;
                    // the result is always inexact
                    if q4 == 1 {
                        R64 = C4.w[0];
                    } else {
                        // if q4 > 1 then truncate C4 from q4 digits to 1 digit;
                        // x = q4-1, 1 <= x <= 67 and check if this operation is exact
                        if q4 <= 18 { // 2 <= q4 <= 18
                            R64 = bid_round64_2_18(
                                q4, q4 - 1, C4.w[0],
                                &mut incr_exp,
                                &mut is_midpoint_lt_even,
                                &mut is_midpoint_gt_even,
                                &mut is_inexact_lt_midpoint,
                                &mut is_inexact_gt_midpoint);
                        } else if q4 <= 38 {
                            P128.w[1] = C4.w[1];
                            P128.w[0] = C4.w[0];
                            R128 = bid_round128_19_38(
                                q4, q4 - 1, &P128,
                                &mut incr_exp,
                                &mut is_midpoint_lt_even,
                                &mut is_midpoint_gt_even,
                                &mut is_inexact_lt_midpoint,
                                &mut is_inexact_gt_midpoint);
                            R64 = R128.w[0]; // one decimal digit
                        } else if q4 <= 57 {
                            P192.w[2] = C4.w[2];
                            P192.w[1] = C4.w[1];
                            P192.w[0] = C4.w[0];
                            R192 = bid_round192_39_57(
                                q4, q4 - 1, &P192,
                                &mut incr_exp,
                                &mut is_midpoint_lt_even,
                                &mut is_midpoint_gt_even,
                                &mut is_inexact_lt_midpoint,
                                &mut is_inexact_gt_midpoint);
                            R64 = R192.w[0]; // one decimal digit
                        } else { // if (q4 <= 68)
                            R256 = bid_round256_58_76(
                                q4, q4 - 1, &C4,
                                &mut incr_exp,
                                &mut is_midpoint_lt_even,
                                &mut is_midpoint_gt_even,
                                &mut is_inexact_lt_midpoint,
                                &mut is_inexact_gt_midpoint);
                            R64 = R256.w[0]; // one decimal digit
                        }
                        if incr_exp {
                            R64 = 10;
                        }
                    }
                    if R64 == 5
                    && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint
                    && !is_midpoint_lt_even    && !is_midpoint_gt_even {
                        is_inexact_lt_midpoint = false;
                        is_inexact_gt_midpoint = false;
                        is_midpoint_lt_even    = true;
                        is_midpoint_gt_even    = false;
                    } else if (e3 == EXP_MIN_UNBIASED) || R64 < 5 || (R64 == 5 && is_inexact_gt_midpoint) {
                        // result does not change
                        is_inexact_lt_midpoint = false;
                        is_inexact_gt_midpoint = true;
                        is_midpoint_lt_even    = false;
                        is_midpoint_gt_even    = false;
                    } else {
                        is_inexact_lt_midpoint = true;
                        is_inexact_gt_midpoint = false;
                        is_midpoint_lt_even    = false;
                        is_midpoint_gt_even    = false;
                        // result decremented is 10^(q3+scale) - 1
                        if q3 + scale <= 19 {
                            res.w[1] = 0;
                            res.w[0] = bid_ten2k64[(q3 + scale) as usize];
                        } else { // if ((q3 + scale + 1) <= 35)
                            res.w[1] = bid_ten2k128[(q3 + scale - 20) as usize].w[1];
                            res.w[0] = bid_ten2k128[(q3 + scale - 20) as usize].w[0];
                        }
                        res.w[0] -= 1; // borrow never occurs
                        z_exp    -= EXP_P1;
                        e3       -= 1;
                        res.w[1] |= z_sign | (((e3 + 6176) as BID_UINT64) << 49);
                    }
                    if e3 == EXP_MIN_UNBIASED {
                        if cfg!(DECIMAL_TINY_DETECTION_AFTER_ROUNDING = "1") {
                            if R64 < 5 || (R64 == 5 && !is_inexact_lt_midpoint) {
                                // result not tiny (in round-to-nearest mode)
                                // rounds to 10^33 * 10^emin
                            } else {
                                *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
                            }
                        } else {
                            *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION; // tiny if detected before rounding
                        }
                    }
                } // end 10^(q3+scale-1)
                // set the inexact flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            } else {
                if p_sign == z_sign {
                    // if (z_sign), set as if for absolute value
                    is_inexact_lt_midpoint = true;
                } else { // if (p_sign != z_sign)
                    // if (z_sign), set as if for absolute value
                    is_inexact_gt_midpoint = true;
                }
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            }
            // the result is always inexact => set the inexact flag
            // Determine tininess:
            //    if (exp > expmin)
            //      the result is not tiny
            //    else // if exp = emin
            //      if (q3 + scale < p34)
            //        the result is tiny
            //      else // if (q3 + scale = p34)
            //        if (C3 * 10^scale > 10^33)
            //          the result is not tiny
            //        else // if C3 * 10^scale = 10^33
            //          if (xy * z > 0)
            //            the result is not tiny
            //          else // if (xy * z < 0)
            //            if (rnd_mode = RN || rnd_mode = RA) and (delta = P+1) and
            //                C4 > 5 * 10^(q4-1)
            //              the result is tiny
            //            else
            //              the result is not tiny
            //          endif
            //        endif
            //      endif
            //    endif

            if cfg!(DECIMAL_TINY_DETECTION_AFTER_ROUNDING = "1") {
                // determine if C4 > 5 * 10^(q4-1)
                if q4 <= 19 {
                    C4gt5toq4m1 = C4.w[0] > bid_midpoint64[(q4 - 1) as usize];
                } else if q4 <= 38 {
                    C4gt5toq4m1 = C4.w[1]  > bid_midpoint128[(q4 - 1) as usize].w[1]
                              || (C4.w[1] == bid_midpoint128[(q4 - 1) as usize].w[1]
                               && C4.w[0]  > bid_midpoint128[(q4 - 1) as usize].w[0]);
                } else if q4 <= 58 {
                    C4gt5toq4m1 = C4.w[2]  > bid_midpoint192[(q4 - 1) as usize].w[2]
                              || (C4.w[2] == bid_midpoint192[(q4 - 1) as usize].w[2]
                              && C4.w[1]   > bid_midpoint192[(q4 - 1) as usize].w[1])
                              || (C4.w[2] == bid_midpoint192[(q4 - 1) as usize].w[2]
                               && C4.w[1] == bid_midpoint192[(q4 - 1) as usize].w[1]
                               && C4.w[0]  > bid_midpoint192[(q4 - 1) as usize].w[0]);
                } else { // if (q4 <= 68)
                    C4gt5toq4m1 =
                         C4.w[3]  > bid_midpoint256[(q4 - 1) as usize].w[3] ||
                        (C4.w[3] == bid_midpoint256[(q4 - 1) as usize].w[3] &&
                         C4.w[2]  > bid_midpoint256[(q4 - 1) as usize].w[2]) ||
                        (C4.w[3] == bid_midpoint256[(q4 - 1) as usize].w[3] &&
                         C4.w[2] == bid_midpoint256[(q4 - 1) as usize].w[2] &&
                         C4.w[1]  > bid_midpoint256[(q4 - 1) as usize].w[1]) ||
                        (C4.w[3] == bid_midpoint256[(q4 - 1) as usize].w[3] &&
                         C4.w[2] == bid_midpoint256[(q4 - 1) as usize].w[2] &&
                         C4.w[1] == bid_midpoint256[(q4 - 1) as usize].w[1] &&
                         C4.w[0]  > bid_midpoint256[(q4 - 1) as usize].w[0]);
                }

                if (e3 == EXP_MIN_UNBIASED && (q3 + scale) < p34)
                || (e3 == EXP_MIN_UNBIASED && (q3 + scale) == p34
                && (res.w[1] & MASK_COEFF) == 0x0000314dc6448d93u64 // 10^33_high
                 && res.w[0] == 0x38c15b0a00000000u64 // 10^33_low
                && z_sign != p_sign
                && (rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY)
                && (delta == (p34 + 1)) && C4gt5toq4m1) {
                    *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
                }
            } else if (e3 == EXP_MIN_UNBIASED && (q3 + scale) < p34)
                   || (e3 == EXP_MIN_UNBIASED && (q3 + scale) == p34
                   && (res.w[1] & MASK_COEFF) == 0x0000314dc6448d93u64 	// 10^33_high
                    && res.w[0] == 0x38c15b0a00000000u64 	                // 10^33_low
                    && z_sign != p_sign) {
                *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION; // for all rounding modes
            }

            if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                bid_rounding_correction(
                    rnd_mode,
                    is_inexact_lt_midpoint,
                    is_inexact_gt_midpoint,
                    is_midpoint_lt_even,
                    is_midpoint_gt_even,
                    e3, &mut res, pfpsf);
            }

            *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            res
        } else if p34 == delta { // Case (1''B)
            // because Case (1''A) was treated above, e3 + 6176 >= p34 - q3
            // and C3 can be scaled up to p34 digits if needed

            // scale C3 to p34 digits if needed
            scale = p34 - q3; // 0 <= scale <= p34 - 1
            if scale == 0 {
                res.w[1] = C3.w[1];
                res.w[0] = C3.w[0];
            } else if q3 <= 19 { // z fits in 64 bits
                res = if scale <= 19 { // 10^scale fits in 64 bits
                    // 64 x 64 C3.w[0] * bid_ten2k64[scale as usize]
                    __mul_64x64_to_128MACH(C3.w[0], bid_ten2k64[scale as usize])
                } else { // 10^scale fits in 128 bits
                    // 64 x 128 C3.w[0] * bid_ten2k128[(scale - 20) as usize]
                    __mul_128x64_to_128(C3.w[0], &bid_ten2k128[(scale - 20) as usize])
                };
            } else { // z fits in 128 bits, but 10^scale must fit in 64 bits
                // 64 x 128 bid_ten2k64[scale as usize] * C3
                res = __mul_128x64_to_128(bid_ten2k64[scale as usize], &C3);
            }
            // subtract scale from the exponent
            z_exp -= (scale as BID_UINT64) << 49;
            e3    -= scale;
            // now z_sign, z_exp, and res correspond to a z scaled to p34 = 34 digits

            // determine whether x * y is less than, equal to, or greater than
            // 1/2 ulp (z)
            if q4 <= 19 {
                match bid_midpoint64[(q4 - 1) as usize] {
                    val if C4.w[0] < val => {   // < 1/2 ulp
                        lt_half_ulp = true;
                    },
                    val if C4.w[0] == val => {  // = 1/2 ulp
                        eq_half_ulp = true;
                    },
                    _ => {                                 // > 1/2 ulp
                        gt_half_ulp = true;
                    }
                }
            } else if q4 <= 38 {
                if C4.w[2] == 0 && (C4.w[1] < bid_midpoint128[(q4 - 20) as usize].w[1]
               || (C4.w[1] == bid_midpoint128[(q4 - 20) as usize].w[1]
                && C4.w[0]  < bid_midpoint128[(q4 - 20) as usize].w[0])) { // < 1/2 ulp
                    lt_half_ulp = true;
                } else if C4.w[2] == 0
                       && C4.w[1] == bid_midpoint128[(q4 - 20) as usize].w[1]
                       && C4.w[0] == bid_midpoint128[(q4 - 20) as usize].w[0] { // = 1/2 ulp
                    eq_half_ulp = true;
                } else { // > 1/2 ulp
                    gt_half_ulp = true;
                }
            } else if q4 <= 58 {
                if C4.w[3] == 0 && (C4.w[2] < bid_midpoint192[(q4 - 39) as usize].w[2]
               || (C4.w[2] == bid_midpoint192[(q4 - 39) as usize].w[2]
                && C4.w[1]  < bid_midpoint192[(q4 - 39) as usize].w[1])
               || (C4.w[2] == bid_midpoint192[(q4 - 39) as usize].w[2]
                && C4.w[1] == bid_midpoint192[(q4 - 39) as usize].w[1]
                && C4.w[0]  < bid_midpoint192[(q4 - 39) as usize].w[0])) { // < 1/2 ulp
                    lt_half_ulp = true;
                } else if C4.w[3] == 0
                       && C4.w[2] == bid_midpoint192[(q4 - 39) as usize].w[2]
                       && C4.w[1] == bid_midpoint192[(q4 - 39) as usize].w[1]
                       && C4.w[0] == bid_midpoint192[(q4 - 39) as usize].w[0] { // = 1/2 ulp
                    eq_half_ulp = true;
                } else { // > 1/2 ulp
                    gt_half_ulp = true;
                }
            } else {
                if C4.w[3]  < bid_midpoint256[(q4 - 59) as usize].w[3]
               || (C4.w[3] == bid_midpoint256[(q4 - 59) as usize].w[3]
                && C4.w[2]  < bid_midpoint256[(q4 - 59) as usize].w[2])
               || (C4.w[3] == bid_midpoint256[(q4 - 59) as usize].w[3]
                && C4.w[2] == bid_midpoint256[(q4 - 59) as usize].w[2]
                && C4.w[1]  < bid_midpoint256[(q4 - 59) as usize].w[1])
               || (C4.w[3] == bid_midpoint256[(q4 - 59) as usize].w[3]
                && C4.w[2] == bid_midpoint256[(q4 - 59) as usize].w[2]
                && C4.w[1] == bid_midpoint256[(q4 - 59) as usize].w[1]
                && C4.w[0]  < bid_midpoint256[(q4 - 59) as usize].w[0]) { // < 1/2 ulp
                    lt_half_ulp = true;
                } else if C4.w[3] == bid_midpoint256[(q4 - 59) as usize].w[3]
                       && C4.w[2] == bid_midpoint256[(q4 - 59) as usize].w[2]
                       && C4.w[1] == bid_midpoint256[(q4 - 59) as usize].w[1]
                       && C4.w[0] == bid_midpoint256[(q4 - 59) as usize].w[0] { // = 1/2 ulp
                    eq_half_ulp = true;
                } else { // > 1/2 ulp
                    gt_half_ulp = true;
                }
            }

            if p_sign == z_sign {
                if lt_half_ulp {
                    res.w[1] |= z_sign | (z_exp & MASK_EXP);
                    // use the following to avoid double rounding errors when operating on
                    // mixed formats in rounding to nearest
                    is_inexact_lt_midpoint = true; // if (z_sign), as if for absolute value
                } else if (eq_half_ulp && (res.w[0] & 0x01) == 0x01) || gt_half_ulp {
                    // add 1 ulp to the significand
                    res.w[0] += 1;
                    if res.w[0] == 0x0u64 {
                        res.w[1] += 1;
                    }
                    // check for rounding overflow, when coeff == 10^34
                    if (res.w[1] & MASK_COEFF) == 0x0001ed09bead87c0u64
                     && res.w[0] == 0x378d8e6400000000u64 { // coefficient = 10^34
                        e3      += 1;
                        // coeff = 10^33
                        z_exp    = (((e3 + 6176) as BID_UINT64) << 49) & MASK_EXP;
                        res.w[1] = 0x0000314dc6448d93u64;
                        res.w[0] = 0x38c15b0a00000000u64;
                    }
                    // end add 1 ulp
                    res.w[1] |= z_sign | (z_exp & MASK_EXP);
                    if eq_half_ulp {
                        is_midpoint_lt_even    = true; // if (z_sign), as if for absolute value
                    } else {
                        is_inexact_gt_midpoint = true; // if (z_sign), as if for absolute value
                    }
                } else { // if (eq_half_ulp && !(res.w[0] & 0x01))
                    // leave unchanged
                    res.w[1]           |= z_sign | (z_exp & MASK_EXP);
                    is_midpoint_gt_even = true; // if (z_sign), as if for absolute value
                }
                // the result is always inexact, and never tiny
                // set the inexact flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                // check for overflow
                if e3 > EXP_MAX_UNBIASED && rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST {
                    res.w[1]                    = z_sign | 0x7800000000000000u64; // +/-inf
                    res.w[0]                    = 0x0000000000000000u64;
                    *pfpsf                     |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
                    *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
                    *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
                    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

                    #[cfg(target_endian = "big")]
                    BID_SWAP128(&mut res);

                    return res;
                }
                if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                    bid_rounding_correction(
                        rnd_mode,
                        is_inexact_lt_midpoint,
                        is_inexact_gt_midpoint,
                        is_midpoint_lt_even, is_midpoint_gt_even,
                        e3, &mut res, pfpsf);
                    z_exp = res.w[1] & MASK_EXP;
                }
            } else { // if (p_sign != z_sign)
                // consider two cases, because C3 * 10^scale = 10^33 is a special case
                if res.w[1] != 0x0000314dc6448d93u64 ||
                    res.w[0] != 0x38c15b0a00000000u64 { // C3 * 10^scale != 10^33
                    if lt_half_ulp {
                        res.w[1] |= z_sign | (z_exp & MASK_EXP);
                        // use the following to avoid double rounding errors when operating
                        // on mixed formats in rounding to nearest
                        is_inexact_gt_midpoint = true; // if (z_sign), as if for absolute value
                    } else if (eq_half_ulp && (res.w[0] & 0x01) == 0x01) || gt_half_ulp {
                        // subtract 1 ulp from the significand
                        res.w[0] -= 1;
                        if res.w[0] == 0xffffffffffffffffu64 {
                            res.w[1] -= 1;
                        }
                        res.w[1] |= z_sign | (z_exp & MASK_EXP);
                        if eq_half_ulp {
                            is_midpoint_gt_even    = true; // if (z_sign), as if for absolute value
                        } else {
                            is_inexact_lt_midpoint = true; //if(z_sign), as if for absolute value
                        }
                    } else { // if (eq_half_ulp && !(res.w[0] & 0x01))
                        // leave unchanged
                        res.w[1]           |= z_sign | (z_exp & MASK_EXP);
                        is_midpoint_lt_even = true; // if (z_sign), as if for absolute value
                    }
                    // the result is always inexact, and never tiny
                    // check for overflow for RN
                    if e3 > EXP_MAX_UNBIASED {
                        if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST {
                            res.w[1] = z_sign | 0x7800000000000000u64; // +/-inf
                            res.w[0] = 0x0000000000000000u64;
                            *pfpsf  |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
                        } else {
                            bid_rounding_correction(
                                rnd_mode,
                                is_inexact_lt_midpoint,
                                is_inexact_gt_midpoint,
                                is_midpoint_lt_even,
                                is_midpoint_gt_even,
                                e3, &mut res, pfpsf);
                        }
                        *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
                        *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
                        *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                        *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

                        #[cfg(target_endian = "big")]
                        BID_SWAP128(&mut res);

                        return res;
                    }
                    // set the inexact flag
                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                    if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                        bid_rounding_correction(
                            rnd_mode,
                            is_inexact_lt_midpoint,
                            is_inexact_gt_midpoint,
                            is_midpoint_lt_even,
                            is_midpoint_gt_even,
                            e3, &mut res, pfpsf);
                    }
                    z_exp = res.w[1] & MASK_EXP;
                } else { // if C3 * 10^scale = 10^33
                    e3 = ((z_exp >> 49) - 6176) as i32;
                    if e3 > EXP_MIN_UNBIASED {
                        // the result is exact if exp > expmin and C4 = d*10^(q4-1),
                        // where d = 1, 2, 3, ..., 9; it could be tiny too, but exact
                        if q4 == 1 {
                            // if q4 = 1 the result is exact
                            // result coefficient = 10^34 - C4
                            res.w[1]  = 0x0001ed09bead87c0u64;
                            res.w[0]  = 0x378d8e6400000000u64 - C4.w[0];
                            z_exp    -= EXP_P1;
                            e3       -= 1;
                            res.w[1] |= z_sign | (z_exp & MASK_EXP);
                        } else {
                            // if q4 > 1 then truncate C4 from q4 digits to 1 digit;
                            // x = q4-1, 1 <= x <= 67 and check if this operation is exact
                            if q4 <= 18 { // 2 <= q4 <= 18
                                R64 = bid_round64_2_18(
                                    q4, q4 - 1, C4.w[0],
                                    &mut incr_exp,
                                    &mut is_midpoint_lt_even,
                                    &mut is_midpoint_gt_even,
                                    &mut is_inexact_lt_midpoint,
                                    &mut is_inexact_gt_midpoint);
                            } else if q4 <= 38 {
                                P128.w[1] = C4.w[1];
                                P128.w[0] = C4.w[0];
                                R128 = bid_round128_19_38(
                                    q4, q4 - 1, &P128,
                                    &mut incr_exp,
                                    &mut is_midpoint_lt_even,
                                    &mut is_midpoint_gt_even,
                                    &mut is_inexact_lt_midpoint,
                                    &mut is_inexact_gt_midpoint);
                                R64 = R128.w[0]; // one decimal digit
                            } else if q4 <= 57 {
                                P192.w[2] = C4.w[2];
                                P192.w[1] = C4.w[1];
                                P192.w[0] = C4.w[0];
                                R192 = bid_round192_39_57(
                                    q4, q4 - 1, &P192,
                                    &mut incr_exp,
                                    &mut is_midpoint_lt_even,
                                    &mut is_midpoint_gt_even,
                                    &mut is_inexact_lt_midpoint,
                                    &mut is_inexact_gt_midpoint);
                                R64 = R192.w[0]; // one decimal digit
                            } else { // if (q4 <= 68)
                                R256 = bid_round256_58_76(
                                    q4, q4 - 1, &C4,
                                    &mut incr_exp,
                                    &mut is_midpoint_lt_even,
                                    &mut is_midpoint_gt_even,
                                    &mut is_inexact_lt_midpoint,
                                    &mut is_inexact_gt_midpoint);
                                R64 = R256.w[0]; // one decimal digit
                            }
                            if !is_midpoint_lt_even    && !is_midpoint_gt_even
                            && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                                // the result is exact: 10^34 - R64
                                // incr_exp = 0 with certainty
                                z_exp   -= EXP_P1;
                                e3      -= 1;
                                res.w[1] = z_sign | (z_exp & MASK_EXP) | 0x0001ed09bead87c0u64;
                                res.w[0] = 0x378d8e6400000000u64 - R64;
                            } else {
                                // We want R64 to be the top digit of C4, but we actually
                                // obtained (C4 * 10^(-q4+1))RN; a correction may be needed,
                                // because the top digit is (C4 * 10^(-q4+1))RZ
                                // however, if incr_exp = 1 then R64 = 10 with certainty
                                if incr_exp {
                                    R64 = 10;
                                }
                                // the result is inexact as C4 has more than 1 significant digit
                                // and C3 * 10^scale = 10^33
                                // example of case that is treated here:
                                // 100...0 * 10^e3 - 0.41 * 10^e3 =
                                // 0999...9.59 * 10^e3 -> rounds to 99...96*10^(e3-1)
                                // note that (e3 > expmin}
                                // in order to round, subtract R64 from 10^34 and then compare
                                // C4 - R64 * 10^(q4-1) with 1/2 ulp
                                // calculate 10^34 - R64
                                res.w[1] = 0x0001ed09bead87c0u64;
                                res.w[0] = 0x378d8e6400000000u64 - R64;
                                z_exp   -= EXP_P1; // will be OR-ed with sign & significand
                                // calculate C4 - R64 * 10^(q4-1); this is a rare case and
                                // R64 is small, 1 <= R64 <= 9
                                e3 -= 1;
                                if is_inexact_lt_midpoint {
                                    is_inexact_lt_midpoint = false;
                                    is_inexact_gt_midpoint = true;
                                } else if is_inexact_gt_midpoint {
                                    is_inexact_gt_midpoint = false;
                                    is_inexact_lt_midpoint = true;
                                } else if is_midpoint_lt_even {
                                    is_midpoint_lt_even = false;
                                    is_midpoint_gt_even = true;
                                } else if is_midpoint_gt_even {
                                    is_midpoint_gt_even = false;
                                    is_midpoint_lt_even = true;
                                } else {
                                    // ;
                                }
                                // the result is always inexact, and never tiny
                                // check for overflow for RN
                                if e3 > EXP_MAX_UNBIASED {
                                    if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST {
                                        res.w[1] = z_sign | 0x7800000000000000u64; // +/-inf
                                        res.w[0] = 0x0000000000000000u64;
                                        *pfpsf  |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
                                    } else {
                                        bid_rounding_correction(
                                            rnd_mode,
                                            is_inexact_lt_midpoint,
                                            is_inexact_gt_midpoint,
                                            is_midpoint_lt_even,
                                            is_midpoint_gt_even,
                                            e3, &mut res, pfpsf);
                                    }
                                    *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
                                    *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
                                    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                                    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

                                    #[cfg(target_endian = "big")]
                                    BID_SWAP128(&mut res);

                                    return res;
                                }
                                // set the inexact flag
                                *pfpsf   |= StatusFlags::BID_INEXACT_EXCEPTION;
                                res.w[1] |= z_sign | (((e3 + 6176) as BID_UINT64) << 49);
                                if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                                    bid_rounding_correction(
                                        rnd_mode,
                                        is_inexact_lt_midpoint,
                                        is_inexact_gt_midpoint,
                                        is_midpoint_lt_even,
                                        is_midpoint_gt_even,
                                        e3, &mut res, pfpsf);
                                }
                                z_exp = res.w[1] & MASK_EXP;
                            } // end result is inexact
                        } // end q4 > 1
                    } else { // if (e3 = emin)
                        // if e3 = expmin the result is also tiny (the condition for
                        // tininess is C4 > 050...0 [q4 digits] which is met because
                        // the msd of C4 is not zero)
                        // the result is tiny and inexact in all rounding modes;
                        // it is either 100...0 or 0999...9 (use lt_half_ulp, eq_half_ulp,
                        // gt_half_ulp to calculate)
                        // if (lt_half_ulp || eq_half_ulp) res = 10^33 stays unchanged

                        // p_sign != z_sign so swap gt_half_ulp and lt_half_ulp
                        if gt_half_ulp { // res = 10^33 - 1
                            res.w[1] = 0x0000314dc6448d93u64;
                            res.w[0] = 0x38c15b09ffffffffu64;
                        } else {
                            res.w[1] = 0x0000314dc6448d93u64;
                            res.w[0] = 0x38c15b0a00000000u64;
                        }
                        res.w[1] |= z_sign | (z_exp & MASK_EXP);
                        *pfpsf   |= StatusFlags::BID_UNDERFLOW_EXCEPTION; // inexact is set later

                        if eq_half_ulp {
                            is_midpoint_lt_even = true; // if (z_sign), as if for absolute value
                        } else if lt_half_ulp {
                            is_inexact_gt_midpoint = true; //if(z_sign), as if for absolute value
                        } else { // if (gt_half_ulp)
                            is_inexact_lt_midpoint = true; //if(z_sign), as if for absolute value
                        }

                        if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                            bid_rounding_correction(
                                rnd_mode,
                                is_inexact_lt_midpoint,
                                is_inexact_gt_midpoint,
                                is_midpoint_lt_even,
                                is_midpoint_gt_even,
                                e3, &mut res, pfpsf);
                            z_exp = res.w[1] & MASK_EXP;
                        }
                    } // end e3 = emin
                    // set the inexact flag (if the result was not exact)
                    if is_inexact_lt_midpoint || is_inexact_gt_midpoint
                    || is_midpoint_lt_even    || is_midpoint_gt_even {
                        *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                    }
                } // end 10^33
            } // end if (p_sign != z_sign)

            res.w[1]                   |= z_sign | (z_exp & MASK_EXP);
            *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            res
        } else if ((q3 <= delta && delta < p34 && p34 < delta + q4)     // Case (2)
            || (q3 <= delta && delta + q4 <= p34)                       // Case (3)
            || (delta < q3 && p34 < delta + q4)                         // Case (4)
            || (delta < q3 && q3 <= delta + q4 && delta + q4 <= p34)    // Case (5)
            || (delta + q4 < q3))                                       // Case (6)
            && !(delta <= 1 && p_sign != z_sign) {                      // Case (2), (3), (4), (5) or (6)

            // the result has the sign of z

            if (q3 <= delta && delta < p34 && p34 < delta + q4)  // Case (2)
            || (delta < q3 && p34 < delta + q4) {               // Case (4)
                // round first the sum x * y + z with unbounded exponent
                // scale C3 up by scale = p34 - q3, 1 <= scale <= p34-1,
                // 1 <= scale <= 33
                // calculate res = C3 * 10^scale
                scale = p34 - q3;
                x0    = delta + q4 - p34;
            } else if delta + q4 < q3 { // Case (6)
                // make Case (6) look like Case (3) or Case (5) with scale = 0
                // by scaling up C4 by 10^(q3 - delta - q4)
                scale = q3 - delta - q4; // 1 <= scale <= 33
                P128 = if q4 <= 19 { // 1 <= scale <= 19; C4 fits in 64 bits
                    if scale <= 19 { // 10^scale fits in 64 bits
                        // 64 x 64 C4.w[0] * bid_ten2k64[scale as usize]
                        __mul_64x64_to_128MACH(C4.w[0], bid_ten2k64[scale as usize])
                    } else { // 10^scale fits in 128 bits
                        // 64 x 128 C4.w[0] * bid_ten2k128[(scale - 20) as usize]
                        __mul_128x64_to_128(C4.w[0], &bid_ten2k128[(scale - 20) as usize])
                    }
                } else { // C4 fits in 128 bits, but 10^scale must fit in 64 bits
                    // 64 x 128 bid_ten2k64[scale as usize] * C4
                    let tmp: BID_UINT128 = BID_UINT128 { w: [C4.w[0], C4.w[1]] };
                    __mul_128x64_to_128(bid_ten2k64[scale as usize], &tmp)
                };
                C4.w[0] = P128.w[0];
                C4.w[1] = P128.w[1];
                // e4 does not need adjustment, as it is not used from this point on
                scale = 0;
                x0    = 0;
                // now Case (6) looks like Case (3) or Case (5) with scale = 0
            } else { // if Case (3) or Case (5)
                // Note: Case (3) is similar to Case (2), but scale differs and the
                // result is exact, unless it is tiny (so x0 = 0 when calculating the
                // result with unbounded exponent)

                // calculate first the sum x * y + z with unbounded exponent (exact)
                // scale C3 up by scale = delta + q4 - q3, 1 <= scale <= p34-1,
                // 1 <= scale <= 33
                // calculate res = C3 * 10^scale
                scale = delta + q4 - q3;
                x0    = 0;
                // Note: the comments which follow refer [mainly] to Case (2)]
            }

// TODO: Try to ge around C goto
// case2_repeat:
            if scale == 0 { // this could happen e.g. if we return to case2_repeat
                // or in Case (4)
                res.w[1] = C3.w[1];
                res.w[0] = C3.w[0];
            } else if q3 <= 19 { // 1 <= scale <= 19; z fits in 64 bits
                res = if scale <= 19 { // 10^scale fits in 64 bits
                    // 64 x 64 C3.w[0] * bid_ten2k64[scale as usize]
                    __mul_64x64_to_128MACH(C3.w[0], bid_ten2k64[scale as usize])
                } else { // 10^scale fits in 128 bits
                    // 64 x 128 C3.w[0] * bid_ten2k128[(scale - 20) as usize]
                    __mul_128x64_to_128(C3.w[0], &bid_ten2k128[(scale - 20) as usize])
                }
            } else { // z fits in 128 bits, but 10^scale must fit in 64 bits
                // 64 x 128 bid_ten2k64[scale as usize] * C3
                res = __mul_128x64_to_128(bid_ten2k64[scale as usize], &C3);
            }
            // e3 is already calculated
            e3 -= scale;
            // now res = C3 * 10^scale and e3 = e3 - scale
            // Note: C3 * 10^scale could be 10^34 if we returned to case2_repeat
            // because the result was too small

            // round C4 to nearest to q4 - x0 digits, where x0 = delta + q4 - p34,
            // 1 <= x0 <= min (q4 - 1, 2 * p34 - 1) <=> 1 <= x0 <= min (q4 - 1, 67)
            // Also: 1 <= q4 - x0 <= p34 -1 => 1 <= q4 - x0 <= 33 (so the result of
            // the rounding fits in 128 bits!)
            // x0 = delta + q4 - p34 (calculated before reaching case2_repeat)
            // because q3 + q4 - x0 <= P => x0 >= q3 + q4 - p34
            if x0 == 0 { // this could happen only if we return to case2_repeat, or
                // for Case (3) or Case (6)
                R128.w[1] = C4.w[1];
                R128.w[0] = C4.w[0];
            } else if q4 <= 18 {
                // 2 <= q4 <= 18, max(1, q3+q4-p34) <= x0 <= q4 - 1, 1 <= x0 <= 17
                R64 = bid_round64_2_18(
                    q4, x0, C4.w[0],
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                if incr_exp {
                    // R64 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 1, 1 <= q4 - x0 <= 17
                    R64 = bid_ten2k64[(q4 - x0) as usize];
                }
                R128.w[1] = 0;
                R128.w[0] = R64;
            } else if q4 <= 38 {
                // 19 <= q4 <= 38, max(1, q3+q4-p34) <= x0 <= q4 - 1, 1 <= x0 <= 37
                P128.w[1] = C4.w[1];
                P128.w[0] = C4.w[0];
                R128 = bid_round128_19_38(
                    q4, x0, &P128,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                if incr_exp {
                    // R128 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 1, 1 <= q4 - x0 <= 37
                    if q4 - x0 <= 19 { // 1 <= q4 - x0 <= 19
                        R128.w[0] = bid_ten2k64[(q4 - x0) as usize];
                        // R128.w[1] stays 0
                    } else { // 20 <= q4 - x0 <= 37
                        R128.w[0] = bid_ten2k128[(q4 - x0 - 20) as usize].w[0];
                        R128.w[1] = bid_ten2k128[(q4 - x0 - 20) as usize].w[1];
                    }
                }
            } else if q4 <= 57 {
                // 38 <= q4 <= 57, max(1, q3+q4-p34) <= x0 <= q4 - 1, 5 <= x0 <= 56
                P192.w[2] = C4.w[2];
                P192.w[1] = C4.w[1];
                P192.w[0] = C4.w[0];
                R192 = bid_round192_39_57(
                    q4, x0, &P192,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                // R192.w[2] is always 0
                if incr_exp {
                    // R192 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 5, 1 <= q4 - x0 <= 52
                    if q4 - x0 <= 19 { // 1 <= q4 - x0 <= 19
                        R192.w[0] = bid_ten2k64[(q4 - x0) as usize];
                        // R192.w[1] stays 0
                        // R192.w[2] stays 0
                    } else { // 20 <= q4 - x0 <= 33
                        R192.w[0] = bid_ten2k128[(q4 - x0 - 20) as usize].w[0];
                        R192.w[1] = bid_ten2k128[(q4 - x0 - 20) as usize].w[1];
                        // R192.w[2] stays 0
                    }
                }
                R128.w[1] = R192.w[1];
                R128.w[0] = R192.w[0];
            } else {
                // 58 <= q4 <= 68, max(1, q3+q4-p34) <= x0 <= q4 - 1, 25 <= x0 <= 67
                R256 = bid_round256_58_76(
                    q4, x0, &C4,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                // R256.w[3] and R256.w[2] are always 0
                if incr_exp {
                    // R256 = 10^(q4-x0), 1 <= q4 - x0 <= q4 - 25, 1 <= q4 - x0 <= 43
                    if q4 - x0 <= 19 { // 1 <= q4 - x0 <= 19
                        R256.w[0] = bid_ten2k64[(q4 - x0) as usize];
                        // R256.w[1] stays 0
                        // R256.w[2] stays 0
                        // R256.w[3] stays 0
                    } else { // 20 <= q4 - x0 <= 33
                        R256.w[0] = bid_ten2k128[(q4 - x0 - 20) as usize].w[0];
                        R256.w[1] = bid_ten2k128[(q4 - x0 - 20) as usize].w[1];
                        // R256.w[2] stays 0
                        // R256.w[3] stays 0
                    }
                }
                R128.w[1] = R256.w[1];
                R128.w[0] = R256.w[0];
            }
            // now add C3 * 10^scale in res and the signed top (q4-x0) digits of C4,
            // rounded to nearest, which were copied into R128
            if z_sign == p_sign {
                lsb = (res.w[0] & 0x01) == 0x01; // lsb of C3 * 10^scale
                // the sum can result in [up to] p34 or p34 + 1 digits
                res.w[0] += R128.w[0];
                res.w[1] += R128.w[1];
                if res.w[0] < R128.w[0] {
                    res.w[1] += 1; // carry
                }
                // if res > 10^34 - 1 need to increase x0 and decrease scale by 1
                if  res.w[1]  > 0x0001ed09bead87c0u64
                || (res.w[1] == 0x0001ed09bead87c0u64
                 && res.w[0]  > 0x378d8e63ffffffffu64) {
                    // avoid double rounding error
                    is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
                    is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
                    is_midpoint_lt_even0    = is_midpoint_lt_even;
                    is_midpoint_gt_even0    = is_midpoint_gt_even;
                    is_inexact_lt_midpoint  = false;
                    is_inexact_gt_midpoint  = false;
                    is_midpoint_lt_even     = false;
                    is_midpoint_gt_even     = false;

                    P128.w[1] = res.w[1];
                    P128.w[0] = res.w[0];

                    res = bid_round128_19_38(
                        35, 1, &P128,
                        &mut incr_exp,
                        &mut is_midpoint_lt_even,
                        &mut is_midpoint_gt_even,
                        &mut is_inexact_lt_midpoint,
                        &mut is_inexact_gt_midpoint);
                    // incr_exp is 0 with certainty in this case
                    // avoid a double rounding error
                    if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even { // double rounding error upward
                        // res = res - 1
                        res.w[0] -= 1;
                        if res.w[0] == 0xffffffffffffffffu64 {
                            res.w[1] -= 1;
                        }
                        // Note: a double rounding error upward is not possible; for this
                        // the result after the first rounding would have to be 99...95
                        // (35 digits in all), possibly followed by a number of zeros; this
                        // not possible in Cases (2)-(6) or (15)-(17) which may get here
                        is_midpoint_lt_even    = false;
                        is_inexact_lt_midpoint = true;
                    } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even { // double rounding error downward
                        // res = res + 1
                        res.w[0] += 1;
                        if res.w[0] == 0 {
                            res.w[1] += 1;
                        }
                        is_midpoint_gt_even    = false;
                        is_inexact_gt_midpoint = true;
                    } else if !is_midpoint_lt_even    && !is_midpoint_gt_even
                           && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                        // if this second rounding was exact the result may still be
                        // inexact because of the first rounding
                        if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
                            is_inexact_gt_midpoint = true;
                        }
                        if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
                            is_inexact_lt_midpoint = true;
                        }
                    } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
                        // pu64ed up to a midpoint
                        is_inexact_lt_midpoint = true;
                        is_inexact_gt_midpoint = false;
                        is_midpoint_lt_even    = false;
                        is_midpoint_gt_even    = false;
                    } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
                        // pu64ed down to a midpoint
                        is_inexact_lt_midpoint = false;
                        is_inexact_gt_midpoint = true;
                        is_midpoint_lt_even    = false;
                        is_midpoint_gt_even    = false;
                    } else {
                        // ;
                    }
                    // adjust exponent
                    e3 += 1;
                    if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                        if is_midpoint_lt_even0 || is_midpoint_gt_even0 || is_inexact_lt_midpoint0 || is_inexact_gt_midpoint0 {
                            is_inexact_lt_midpoint = true;
                        }
                    }
                } else {
                    // this is the result rounded with unbounded exponent, unless a
                    // correction is needed
                    res.w[1] &= MASK_COEFF;
                    if lsb {
                        if is_midpoint_gt_even {
                            // res = res + 1
                            is_midpoint_gt_even = false;
                            is_midpoint_lt_even = true;
                            res.w[0] += 1;
                            if res.w[0] == 0x0 {
                                res.w[1] += 1;
                            }
                            // check for rounding overflow
                            if res.w[1] == 0x0001ed09bead87c0u64 && res.w[0] == 0x378d8e6400000000u64 {
                                // res = 10^34 => rounding overflow
                                res.w[1] = 0x0000314dc6448d93u64;
                                res.w[0] = 0x38c15b0a00000000u64; // 10^33
                                e3      += 1;
                            }
                        } else if is_midpoint_lt_even {
                            // res = res - 1
                            is_midpoint_lt_even = false;
                            is_midpoint_gt_even = true;
                            res.w[0]           -= 1;
                            if res.w[0] == 0xffffffffffffffffu64 {
                                res.w[1] -= 1;
                            }
                            // if the result is pure zero, the sign depends on the rounding
                            // mode (x*y and z had opposite signs)
                            if res.w[1] == 0x0u64 && res.w[0] == 0x0u64 {
                                z_sign = if rnd_mode !=  RoundingMode::BID_ROUNDING_DOWN {
                                    0x0000000000000000u64
                                } else {
                                    0x8000000000000000u64
                                };
                                // the exponent is max (e3, expmin)
                                res.w[1]                    = 0x0;
                                res.w[0]                    = 0x0;
                                *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
                                *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
                                *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                                *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

                                #[cfg(target_endian = "big")]
                                BID_SWAP128(&mut res);

                                return res;
                            }
                        } else {
                            // ;
                        }
                    }
                }
            } else { // if (z_sign != p_sign)
                lsb = (res.w[0] & 0x01) == 0x01; // lsb of C3 * 10^scale; R128 contains rounded C4
                // used to swap rounding indicators if p_sign != z_sign
                // the sum can result in [up to] p34 or p34 - 1 digits
                tmp64     = res.w[0];
                res.w[0] -= R128.w[0];
                res.w[1] -= R128.w[1];
                if res.w[0] > tmp64 {
                    res.w[1] -= 1; // borrow
                }
                // if res < 10^33 and exp > expmin need to decrease x0 and
                // increase scale by 1
                if e3 > EXP_MIN_UNBIASED
                && ((res.w[1] < 0x0000314dc6448d93u64
                || (res.w[1] == 0x0000314dc6448d93u64
                && res.w[0] < 0x38c15b0a00000000u64))
                || ((is_inexact_lt_midpoint | is_midpoint_gt_even)
                && res.w[1] == 0x0000314dc6448d93u64
                && res.w[0] == 0x38c15b0a00000000u64))
                && x0 >= 1 {
                    x0                    -= 1;
                    // first restore e3, otherwise it will be too small
                    e3                    += scale;
                    scale                 += 1;
                    is_inexact_lt_midpoint = false;
                    is_inexact_gt_midpoint = false;
                    is_midpoint_lt_even    = false;
                    is_midpoint_gt_even    = false;
                    incr_exp               = false;

// TODO: Try to ge around C goto
                    panic!("goto case2_repeat");
                }
                // else this is the result rounded with unbounded exponent;
                // because the result has opposite sign to that of C4 which was
                // rounded, need to change the rounding indicators
                if is_inexact_lt_midpoint {
                    is_inexact_lt_midpoint = false;
                    is_inexact_gt_midpoint = true;
                } else if is_inexact_gt_midpoint {
                    is_inexact_gt_midpoint = false;
                    is_inexact_lt_midpoint = true;
                } else if !lsb {
                    if is_midpoint_lt_even {
                        is_midpoint_lt_even = false;
                        is_midpoint_gt_even = true;
                    } else if is_midpoint_gt_even {
                        is_midpoint_gt_even = false;
                        is_midpoint_lt_even = true;
                    } else {
                        // ;
                    }
                } else if lsb  {
                    if is_midpoint_lt_even {
                        // res = res + 1
                        res.w[0] += 1;
                        if res.w[0] == 0x0 {
                            res.w[1] += 1;
                        }
                        // check for rounding overflow
                        if res.w[1] == 0x0001ed09bead87c0u64 && res.w[0] == 0x378d8e6400000000u64 {
                            // res = 10^34 => rounding overflow
                            res.w[1] = 0x0000314dc6448d93u64;
                            res.w[0] = 0x38c15b0a00000000u64; // 10^33
                            e3      += 1;
                        }
                    } else if is_midpoint_gt_even {
                        // res = res - 1
                        res.w[0] -= 1;
                        if res.w[0] == 0xffffffffffffffffu64 {
                            res.w[1] -= 1;
                        }
                        // if the result is pure zero, the sign depends on the rounding
                        // mode (x*y and z had opposite signs)
                        if res.w[1] == 0x0u64 && res.w[0] == 0x0u64 {
                            z_sign = if rnd_mode != RoundingMode::BID_ROUNDING_DOWN {
                                0x0000000000000000u64
                            } else {
                                0x8000000000000000u64
                            };
                            // the exponent is max (e3, expmin)
                            res.w[1]                    = 0x0;
                            res.w[0]                    = 0x0;
                            *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
                            *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
                            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

                            #[cfg(target_endian = "big")]
                            BID_SWAP128(&mut res);

                            return res;
                        }
                    } else {
                        // ;
                    }
                } else {
                    // ;
                }
            }

            // check for underflow
            if e3 == EXP_MIN_UNBIASED { // and if significand < 10^33 => result is tiny
                if (res.w[1] & MASK_COEFF) < 0x0000314dc6448d93u64
                || ((res.w[1] & MASK_COEFF) == 0x0000314dc6448d93u64
                  && res.w[0] < 0x38c15b0a00000000u64) {
                    is_tiny = true;
                }
                #[cfg(not(feature = "DECIMAL_TINY_DETECTION_AFTER_ROUNDING"))]
                if ((res.w[1] & 0x7fffffffffffffffu64) == 0x0000314dc6448d93u64) &&
                    (res.w[0] == 0x38c15b0a00000000u64) &&  // 10^33*10^-6176
                    (z_sign != p_sign) {
                    is_tiny = true;
                }
            } else if e3 < EXP_MIN_UNBIASED {
                // the result is tiny, so we must truncate more of res
                is_tiny                 = true;
                x0                      = EXP_MIN_UNBIASED - e3;
                is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
                is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
                is_midpoint_lt_even0    = is_midpoint_lt_even;
                is_midpoint_gt_even0    = is_midpoint_gt_even;
                is_inexact_lt_midpoint  = false;
                is_inexact_gt_midpoint  = false;
                is_midpoint_lt_even     = false;
                is_midpoint_gt_even     = false;
                // determine the number of decimal digits in res
                if res.w[1] == 0x0 {
                    // between 1 and 19 digits
                    // ind = 1;
                    // while ind <= 19 {
                    //     if R256.w[0] < bid_ten2k64[ind as usize] {
                    //         break;
                    //     }
                    //     ind += 1;
                    // }
                    ind = bid_ten2k64[1..=19]
                        .iter().enumerate()
                        .take_while(|&(_, x)| R256.w[0] >= *x)
                        .count() as i32;
                    // ind digits
                } else if res.w[1] < bid_ten2k128[0].w[1]
                      || (res.w[1] == bid_ten2k128[0].w[1]
                       && res.w[0] < bid_ten2k128[0].w[0]) {
                    // 20 digits
                    ind = 20;
                } else {
                    // between 21 and 38 digits
                    // ind = 1;
                    // while ind <= 18 {
                    //     if  res.w[1]  < bid_ten2k128[ind as usize].w[1]
                    //     || (res.w[1] == bid_ten2k128[ind as usize].w[1]
                    //      && res.w[0]  < bid_ten2k128[ind as usize].w[0]) {
                    //         break;
                    //     }
                    //     ind += 1;
                    // }
                    ind = bid_ten2k128[1..=18]
                        .iter().enumerate()
                        .take_while(|&(_, d)|
                            !(res.w[1] < d.w[1] || (res.w[1] == d.w[1] && res.w[0] < d.w[0]))
                        ).count() as i32;
                    // ind + 20 digits
                    ind += 20;
                }

                // at this point ind >= x0; because delta >= 2 on this path, the case
                // ind = x0 can occur only in Case (2) or case (3), when C3 has one
                // digit (q3 = 1) equal to 1 (C3 = 1), e3 is expmin (e3 = expmin),
                // the signs of x * y and z are opposite, and through cancellation
                // the most significant decimal digit in res has the weight
                // 10^(emin-1); however, it is clear that in this case the most
                // significant digit is 9, so the result before rounding is
                // 0.9... * 10^emin
                // Otherwise, ind > x0 because there are non-zero decimal digits in the
                // result with weight of at least 10^emin, and correction for underflow
                //  can be carried out using the round*_*_2_* () routines
                if x0 == ind { // the result before rounding is 0.9... * 10^emin
                    res.w[1]               = 0x0;
                    res.w[0]               = 0x1;
                    is_inexact_gt_midpoint = true;
                } else if ind <= 18 { // check that 2 <= ind
                    // 2 <= ind <= 18, 1 <= x0 <= 17
                    R64 = bid_round64_2_18(
                        ind, x0, res.w[0],
                        &mut incr_exp,
                        &mut is_midpoint_lt_even,
                        &mut is_midpoint_gt_even,
                        &mut is_inexact_lt_midpoint,
                        &mut is_inexact_gt_midpoint);

                    if incr_exp {
                        // R64 = 10^(ind-x0), 1 <= ind - x0 <= ind - 1, 1 <= ind - x0 <= 17
                        R64 = bid_ten2k64[(ind - x0) as usize];
                    }
                    res.w[1] = 0;
                    res.w[0] = R64;
                } else if ind <= 38 {
                    // 19 <= ind <= 38
                    P128.w[1] = res.w[1];
                    P128.w[0] = res.w[0];
                    res = bid_round128_19_38(
                        ind, x0, &P128,
                        &mut incr_exp,
                        &mut is_midpoint_lt_even,
                        &mut is_midpoint_gt_even,
                        &mut is_inexact_lt_midpoint,
                        &mut is_inexact_gt_midpoint);
                    if incr_exp {
                        // R128 = 10^(ind-x0), 1 <= ind - x0 <= ind - 1, 1 <= ind - x0 <= 37
                        if ind - x0 <= 19 { // 1 <= ind - x0 <= 19
                            res.w[0] = bid_ten2k64[(ind - x0) as usize];
                            // res.w[1] stays 0
                        } else { // 20 <= ind - x0 <= 37
                            res.w[0] = bid_ten2k128[(ind - x0 - 20) as usize].w[0];
                            res.w[1] = bid_ten2k128[(ind - x0 - 20) as usize].w[1];
                        }
                    }
                }
                // avoid a double rounding error
                if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even { // double rounding error upward
                    // res = res - 1
                    res.w[0] -= 1;
                    if res.w[0] == 0xffffffffffffffffu64 {
                        res.w[1] -= 1;
                    }
                    // Note: a double rounding error upward is not possible; for this
                    // the result after the first rounding would have to be 99...95
                    // (35 digits in all), possibly followed by a number of zeros; this
                    // not possible in Cases (2)-(6) which may get here
                    is_midpoint_lt_even    = false;
                    is_inexact_lt_midpoint = true;
                } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even { // double rounding error downward
                    // res = res + 1
                    res.w[0] += 1;
                    if res.w[0] == 0 {
                        res.w[1] += 1;
                    }
                    is_midpoint_gt_even    = false;
                    is_inexact_gt_midpoint = true;
                } else if !is_midpoint_lt_even    && !is_midpoint_gt_even
                       && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                    // if this second rounding was exact the result may still be
                    // inexact because of the first rounding
                    if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
                        is_inexact_gt_midpoint = true;
                    }
                    if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
                        is_inexact_lt_midpoint = true;
                    }
                } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
                    // pu64ed up to a midpoint
                    is_inexact_lt_midpoint = true;
                    is_inexact_gt_midpoint = false;
                    is_midpoint_lt_even    = false;
                    is_midpoint_gt_even    = false;
                } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
                    // pu64ed down to a midpoint
                    is_inexact_lt_midpoint = false;
                    is_inexact_gt_midpoint = true;
                    is_midpoint_lt_even    = false;
                    is_midpoint_gt_even    = false;
                } else {
                    // ;
                }
                // adjust exponent
                e3 += x0;
                if !is_midpoint_lt_even     && !is_midpoint_gt_even
                && !is_inexact_lt_midpoint  && !is_inexact_gt_midpoint
                && (is_midpoint_lt_even0    || is_midpoint_gt_even0
                 || is_inexact_lt_midpoint0 || is_inexact_gt_midpoint0) {
                    is_inexact_lt_midpoint = true;
                }
            } else {
                // not underflow
            }
            // check for inexact result
            if is_inexact_lt_midpoint || is_inexact_gt_midpoint
            || is_midpoint_lt_even    || is_midpoint_gt_even {
                // set the inexact flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                if is_tiny {
                    *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
                }
            }
            // now check for significand = 10^34 (may have resulted from going
            // back to case2_repeat)
            if res.w[1] == 0x0001ed09bead87c0u64 && res.w[0] == 0x378d8e6400000000u64 { // if  res = 10^34
                res.w[1] = 0x0000314dc6448d93u64; // res = 10^33
                res.w[0] = 0x38c15b0a00000000u64;
                e3      += 1;
            }
            res.w[1] |= z_sign | (((e3 + 6176) as BID_UINT64) << 49);
            // check for overflow
            if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && e3 > EXP_MAX_UNBIASED {
                res.w[1] = z_sign | 0x7800000000000000u64; // +/-inf
                res.w[0] = 0x0000000000000000u64;
                *pfpsf  |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
            }
            if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                bid_rounding_correction(
                    rnd_mode,
                    is_inexact_lt_midpoint,
                    is_inexact_gt_midpoint,
                    is_midpoint_lt_even,
                    is_midpoint_gt_even,
                    e3, &mut res, pfpsf);
            }
            *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            res
        } else {
            // we get here only if delta <= 1 in Cases (2), (3), (4), (5), or (6) and
            // the signs of x*y and z are opposite; in these cases massive
            // cancellation can occur, so it is better to scale either C3 or C4 and
            // to perform the subtraction before rounding; rounding is performed
            // next, depending on the number of decimal digits in the result and on
            // the exponent value
            // Note: overlow is not possible in this case
            // this is similar to Cases (15), (16), and (17)

            if delta + q4 < q3 { // from Case (6)
                // Case (6) with 0<= delta <= 1 is similar to Cases (15), (16), and
                // (17) if we swap (C3, C4), (q3, q4), (e3, e4), (z_sign, p_sign)
                // and call bid_add_and_round; delta stays positive
                // C4.w[3] = 0 and C4.w[2] = 0, so swap just the low part of C4 with C3
                P128.w[1] = C3.w[1];
                P128.w[0] = C3.w[0];
                C3.w[1]   = C4.w[1];
                C3.w[0]   = C4.w[0];
                C4.w[1]   = P128.w[1];
                C4.w[0]   = P128.w[0];
                ind       = q3;
                q3        = q4;
                q4        = ind;
                ind       = e3;
                e3        = e4;
                e4        = ind;
                tmp_sign  = z_sign;
                z_sign    = p_sign;
                p_sign    = tmp_sign;
            } else { // from Cases (2), (3), (4), (5)
                // In Cases (2), (3), (4), (5) with 0 <= delta <= 1 C3 has to be
                // scaled up by q4 + delta - q3; this is the same as in Cases (15),
                // (16), and (17) if we just change the sign of delta
                delta = -delta;
            }
            res = bid_add_and_round(
               q3, q4, e4, delta, p34, z_sign, p_sign, &C3, &C4,
               rnd_mode,
               &mut is_midpoint_lt_even,
               &mut is_midpoint_gt_even,
               &mut is_inexact_lt_midpoint,
               &mut is_inexact_gt_midpoint,
               pfpsf);
            *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            res
        }
    } else { // if delta < 0
        delta = -delta;

        if p34 < q4 && q4 <= delta { // Case (7)
            // truncate C4 to p34 digits into res
            // x = q4-p34, 1 <= x <= 34 because 35 <= q4 <= 68
            x0 = q4 - p34;
            if q4 <= 38 {
                P128.w[1] = C4.w[1];
                P128.w[0] = C4.w[0];
                res = bid_round128_19_38(
                    q4, x0, &P128,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
            } else if q4 <= 57 { // 35 <= q4 <= 57
                P192.w[2] = C4.w[2];
                P192.w[1] = C4.w[1];
                P192.w[0] = C4.w[0];
                R192 = bid_round192_39_57(
                    q4, x0, &P192,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                res.w[0] = R192.w[0];
                res.w[1] = R192.w[1];
            } else { // if (q4 <= 68)
                R256 = bid_round256_58_76(
                    q4, x0, &C4,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                res.w[0] = R256.w[0];
                res.w[1] = R256.w[1];
            }
            e4 += x0;
            if incr_exp {
                e4 += 1;
            }
            if !is_midpoint_lt_even    && !is_midpoint_gt_even
            && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                // if C4 rounded to p34 digits is exact then the result is inexact,
                // in a way that depends on the signs of x * y and z
                if p_sign == z_sign {
                    is_inexact_lt_midpoint = true;
                } else { // if (p_sign != z_sign)
                    if res.w[1] != 0x0000314dc6448d93u64 || res.w[0] != 0x38c15b0a00000000u64 { // res != 10^33
                        is_inexact_gt_midpoint = true;
                    } else { // res = 10^33 and exact is a special case
                        // if C3 < 1/2 ulp then res = 10^33 and is_inexact_gt_midpoint = 1
                        // if C3 = 1/2 ulp then res = 10^33 and is_midpoint_lt_even = 1
                        // if C3 > 1/2 ulp then res = 10^34-1 and is_inexact_lt_midpoint = 1
                        // Note: ulp is really ulp/10 (after borrow which propagates to msd)
                        if delta > p34 + 1 { // C3 < 1/2
                            // res = 10^33, unchanged
                            is_inexact_gt_midpoint = true;
                        } else { // if (delta == p34 + 1)
                            if q3 <= 19 {
                                match bid_midpoint64[(q3 - 1) as usize] {
                                    val if C3.w[0] < val => { // C3 < 1/2 ulp
                                        // res = 10^33, unchanged
                                        is_inexact_gt_midpoint = true;
                                    },
                                    val if C3.w[0] == val => { // C3 = 1/2 ulp
                                        // res = 10^33, unchanged
                                        is_midpoint_lt_even = true;
                                    },
                                    _ => { // if (C3.w[0] > bid_midpoint64[q3-1]), C3 > 1/2 ulp
                                        res.w[1] = 0x0001ed09bead87c0u64; // 10^34 - 1
                                        res.w[0] = 0x378d8e63ffffffffu64;
                                        e4      -= 1;
                                        is_inexact_lt_midpoint = true;
                                    }
                                }
                            } else { // if (20 <= q3 <=34)
                                if  C3.w[1]  < bid_midpoint128[(q3 - 20) as usize].w[1]
                                || (C3.w[1] == bid_midpoint128[(q3 - 20) as usize].w[1]
                                 && C3.w[0]  < bid_midpoint128[(q3 - 20) as usize].w[0]) { // C3 < 1/2 ulp
                                    // res = 10^33, unchanged
                                    is_inexact_gt_midpoint = true;
                                } else if C3.w[1] == bid_midpoint128[(q3 - 20) as usize].w[1]
                                       && C3.w[0] == bid_midpoint128[(q3 - 20) as usize].w[0] { // C3 = 1/2 ulp
                                    // res = 10^33, unchanged
                                    is_midpoint_lt_even = true;
                                } else { // if (C3 > bid_midpoint128[q3-20]), C3 > 1/2 ulp
                                    res.w[1] = 0x0001ed09bead87c0u64; // 10^34 - 1
                                    res.w[0] = 0x378d8e63ffffffffu64;
                                    e4      -= 1;
                                    is_inexact_lt_midpoint = true;
                                }
                            }
                        }
                    }
                }
            } else if is_midpoint_lt_even {
                if z_sign != p_sign {
                    // needs correction: res = res - 1
                    res.w[0] -= 1;
                    if res.w[0] == 0xffffffffffffffffu64 {
                        res.w[1] -= 1;
                    }
                    // if it is (10^33-1)*10^e4 then the corect result is
                    // (10^34-1)*10(e4-1)
                    if res.w[1] == 0x0000314dc6448d93u64 && res.w[0] == 0x38c15b09ffffffffu64 {
                        res.w[1] = 0x0001ed09bead87c0u64; // 10^34 - 1
                        res.w[0] = 0x378d8e63ffffffffu64;
                        e4      -= 1;
                    }
                    is_midpoint_lt_even    = false;
                    is_inexact_lt_midpoint = true;
                } else { // if (z_sign == p_sign)
                    is_midpoint_lt_even    = false;
                    is_inexact_gt_midpoint = true;
                }
            } else if is_midpoint_gt_even {
                if z_sign == p_sign {
                    // needs correction: res = res + 1 (cannot cross in the next binade)
                    res.w[0] += 1;
                    if res.w[0] == 0x0000000000000000u64 {
                        res.w[1] += 1;
                    }
                    is_midpoint_gt_even    = false;
                    is_inexact_gt_midpoint = true;
                } else { // if (z_sign != p_sign)
                    is_midpoint_gt_even    = false;
                    is_inexact_lt_midpoint = true;
                }
            } else {
                // the rounded result is already correct
            }
            // check for overflow
            if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && e4 > EXP_MAX_UNBIASED {
                res.w[1] = p_sign | 0x7800000000000000u64;
                res.w[0] = 0x0000000000000000u64;
                *pfpsf  |= StatusFlags::BID_OVERFLOW_EXCEPTION | StatusFlags::BID_INEXACT_EXCEPTION;
            } else { // no overflow or not RN
                p_exp     = ((e4 + 6176) as BID_UINT64) << 49;
                res.w[1] |= p_sign | (p_exp & MASK_EXP);
            }
            if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                bid_rounding_correction(
                    rnd_mode,
                    is_inexact_lt_midpoint,
                    is_inexact_gt_midpoint,
                    is_midpoint_lt_even, is_midpoint_gt_even,
                    e4, &mut res, pfpsf);
            }
            if is_inexact_lt_midpoint || is_inexact_gt_midpoint ||
               is_midpoint_lt_even    || is_midpoint_gt_even {
                // set the inexact flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            }
            *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            return res
        } else if (q4 <= p34 && p34 <= delta)                           // Case (8)
               || (q4 <= delta && delta < p34 && p34 < delta + q3)      // Case (9)
               || (q4 <= delta && delta + q3 <= p34)                    // Case (10)
               || (delta < q4 && q4 <= p34 && p34 < delta + q3)         // Case (13)
               || (delta < q4 && q4 <= delta + q3 && delta + q3 <= p34) // Case (14)
               || (delta + q3 < q4 && q4 <= p34) {                      // Case (18)

            // Case (8) is similar to Case (1), with C3 and C4 swapped
            // Case (9) is similar to Case (2), with C3 and C4 swapped
            // Case (10) is similar to Case (3), with C3 and C4 swapped
            // Case (13) is similar to Case (4), with C3 and C4 swapped
            // Case (14) is similar to Case (5), with C3 and C4 swapped
            // Case (18) is similar to Case (6), with C3 and C4 swapped

            // swap (C3, C4), (q3, q4), (e3, 34), (z_sign, p_sign), (z_exp, p_exp)
            // and go back to delta_ge_zero
            // C4.w[3] = 0 and C4.w[2] = 0, so swap just the low part of C4 with C3
            P128.w[1] = C3.w[1];
            P128.w[0] = C3.w[0];
            C3.w[1]   = C4.w[1];
            C3.w[0]   = C4.w[0];
            C4.w[1]   = P128.w[1];
            C4.w[0]   = P128.w[0];
            ind       = q3;
            q3        = q4;
            q4        = ind;
            ind       = e3;
            e3        = e4;
            e4        = ind;
            tmp_sign  = z_sign;
            z_sign    = p_sign;
            p_sign    = tmp_sign;
            // TODO: Avoid
            unsafe  {
                tmp.i = z_exp;
                z_exp = p_exp;
                p_exp = tmp.i;
            }

// TODO: Try to ge around C goto
            panic!("goto delta_ge_zero;")
        } else if (p34 <= delta && delta < q4 && q4 < delta + q3)   // Case (11)
               || (delta < p34 && p34 < q4 && q4 < delta + q3) {    // Case (12)

            // round C3 to nearest to q3 - x0 digits, where x0 = e4 - e3,
            // 1 <= x0 <= q3 - 1 <= p34 - 1
            x0 = e4 - e3; // or x0 = delta + q3 - q4
            if q3 <= 18 { // 2 <= q3 <= 18
                R64 = bid_round64_2_18(
                    q3, x0, C3.w[0],
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                // C3.w[1] = 0;
                C3.w[0] = R64;
            } else if q3 <= 38 {
                R128 = bid_round128_19_38(
                    q3, x0, &C3,
                    &mut incr_exp,
                    &mut is_midpoint_lt_even,
                    &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint,
                    &mut is_inexact_gt_midpoint);
                C3.w[1] = R128.w[1];
                C3.w[0] = R128.w[0];
            }
            // the rounded result has q3 - x0 digits
            // we want the exponent to be e4, so if incr_exp = 1 then
            // multiply the rounded result by 10 - it will still fit in 113 bits
            if incr_exp {
                // 64 x 128 -> 128
                P128.w[1] = C3.w[1];
                P128.w[0] = C3.w[0];
                C3        = __mul_64x128_to_128(bid_ten2k64[1], &P128);
            }
            e3 += x0; // this is e4
            // now add/subtract the 256-bit C4 and the new (and shorter) 128-bit C3;
            // the result will have the sign of x * y; the exponent is e4
            R256.w[3] = 0;
            R256.w[2] = 0;
            R256.w[1] = C3.w[1];
            R256.w[0] = C3.w[0];
            if p_sign == z_sign { // R256 = C4 + R256
                R256 = bid_add256(&C4, &R256);
            } else { // if (p_sign != z_sign) { // R256 = C4 - R256
                R256 = bid_sub256(&C4, &R256); // the result cannot be pure zero
                // because the result has opposite sign to that of R256 which was
                // rounded, need to change the rounding indicators
                lsb = (C4.w[0] & 0x01) == 0x01;
                if is_inexact_lt_midpoint {
                    is_inexact_lt_midpoint = false;
                    is_inexact_gt_midpoint = true;
                } else if is_inexact_gt_midpoint {
                    is_inexact_gt_midpoint = false;
                    is_inexact_lt_midpoint = true;
                } else if !lsb {
                    if is_midpoint_lt_even {
                        is_midpoint_lt_even = false;
                        is_midpoint_gt_even = true;
                    } else if is_midpoint_gt_even {
                        is_midpoint_gt_even = false;
                        is_midpoint_lt_even = true;
                    } else {
                        // ;
                    }
                } else if lsb {
                    if is_midpoint_lt_even {
                        // res = res + 1
                        R256.w[0] += 1;
                        if R256.w[0] == 0x0 {
                            R256.w[1] += 1;
                            if R256.w[1] == 0x0 {
                                R256.w[2] += 1;
                                if R256.w[2] == 0x0 {
                                    R256.w[3] += 1;
                                }
                            }
                        }
                        // no check for rounding overflow - R256 was a difference
                    } else if is_midpoint_gt_even {
                        // res = res - 1
                        R256.w[0] -= 1;
                        if R256.w[0] == 0xffffffffffffffffu64 {
                            R256.w[1] -= 1;
                            if R256.w[1] == 0xffffffffffffffffu64 {
                                R256.w[2] -= 1;
                                if R256.w[2] == 0xffffffffffffffffu64 {
                                    R256.w[3] -= 1;
                                }
                            }
                        }
                    } else {
                        // ;
                    }
                } else {
                    // ;
                }
            }
            // determine the number of decimal digits in R256
            ind = bid_bid_nr_digits256(&R256); // ind >= p34
            // if R256 is sum, then ind > p34; if R256 is a difference, then
            // ind >= p34; this means that we can calculate the result rounded to
            // the destination precision, with unbounded exponent, starting from R256
            // and using the indicators from the rounding of C3 to avoid a double
            // rounding error

            if ind < p34 {
                // ;
            } else if ind == p34 {
                // the result rounded to the destination precision with
                // unbounded exponent
                // is (-1)^p_sign * R256 * 10^e4
                res.w[1] = R256.w[1];
                res.w[0] = R256.w[0];
            } else { // if (ind > p34)
                // if more than P digits, round to nearest to P digits
                // round R256 to p34 digits
                x0 = ind - p34; // 1 <= x0 <= 34 as 35 <= ind <= 68
                // save C3 rounding indicators to help avoid double rounding error
                is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
                is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
                is_midpoint_lt_even0    = is_midpoint_lt_even;
                is_midpoint_gt_even0    = is_midpoint_gt_even;
                // initialize rounding indicators
                is_inexact_lt_midpoint = false;
                is_inexact_gt_midpoint = false;
                is_midpoint_lt_even    = false;
                is_midpoint_gt_even    = false;
                // round to p34 digits; the result fits in 113 bits
                if ind <= 38 {
                    P128.w[1] = R256.w[1];
                    P128.w[0] = R256.w[0];
                    R128 = bid_round128_19_38(
                        ind, x0, &P128,
                        &mut incr_exp,
                        &mut is_midpoint_lt_even,
                        &mut is_midpoint_gt_even,
                        &mut is_inexact_lt_midpoint,
                        &mut is_inexact_gt_midpoint);
                } else if ind <= 57 {
                    P192.w[2] = R256.w[2];
                    P192.w[1] = R256.w[1];
                    P192.w[0] = R256.w[0];
                    R192 = bid_round192_39_57(
                        ind, x0, &P192,
                        &mut incr_exp,
                        &mut is_midpoint_lt_even,
                        &mut is_midpoint_gt_even,
                        &mut is_inexact_lt_midpoint,
                        &mut is_inexact_gt_midpoint);
                    R128.w[1] = R192.w[1];
                    R128.w[0] = R192.w[0];
                } else { // if (ind <= 68)
                    R256 = bid_round256_58_76(
                        ind, x0, &R256,
                        &mut incr_exp,
                        &mut is_midpoint_lt_even,
                        &mut is_midpoint_gt_even,
                        &mut is_inexact_lt_midpoint,
                        &mut is_inexact_gt_midpoint);
                    R128.w[1] = R256.w[1];
                    R128.w[0] = R256.w[0];
                }
                // the rounded result has p34 = 34 digits
                e4 = e4 + x0 + if incr_exp { 1 } else { 0 };

                res.w[1] = R128.w[1];
                res.w[0] = R128.w[0];

                // avoid a double rounding error
                if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even { // double rounding error upward
                    // res = res - 1
                    res.w[0] -= 1;
                    if res.w[0] == 0xffffffffffffffffu64 {
                        res.w[1] -= 1;
                    }
                    is_midpoint_lt_even    = false;
                    is_inexact_lt_midpoint = true;
                    // Note: a double rounding error upward is not possible; for this
                    // the result after the first rounding would have to be 99...95
                    // (35 digits in all), possibly followed by a number of zeros; this
                    // not possible in Cases (2)-(6) or (15)-(17) which may get here
                    // if this is 10^33 - 1 make it 10^34 - 1 and decrement exponent
                    if res.w[1] == 0x0000314dc6448d93u64 && res.w[0] == 0x38c15b09ffffffffu64 { // 10^33 - 1
                        res.w[1] = 0x0001ed09bead87c0u64; // 10^34 - 1
                        res.w[0] = 0x378d8e63ffffffffu64;
                        e4      -= 1;
                    }
                } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even { // double rounding error downward
                    // res = res + 1
                    res.w[0] += 1;
                    if res.w[0] == 0 {
                        res.w[1] += 1;
                    }
                    is_midpoint_gt_even    = false;
                    is_inexact_gt_midpoint = true;
                } else if !is_midpoint_lt_even && !is_midpoint_gt_even
                       && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                    // if this second rounding was exact the result may still be
                    // inexact because of the first rounding
                    if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
                        is_inexact_gt_midpoint = true;
                    }
                    if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
                        is_inexact_lt_midpoint = true;
                    }
                } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
                    // pu64ed up to a midpoint
                    is_inexact_lt_midpoint = true;
                    is_inexact_gt_midpoint = false;
                    is_midpoint_lt_even    = false;
                    is_midpoint_gt_even    = false;
                } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
                    // pu64ed down to a midpoint
                    is_inexact_lt_midpoint = false;
                    is_inexact_gt_midpoint = true;
                    is_midpoint_lt_even    = false;
                    is_midpoint_gt_even    = false;
                } else {
                    // ;
                }
            }

            // determine tininess
            if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST {
                if e4 < EXP_MIN_UNBIASED {
                    is_tiny = true; // for other rounding modes apply correction
                }
            } else {
                // for RM, RP, RZ, RA apply correction in order to determine tininess
                // but do not save the result; apply the correction to
                // (-1)^p_sign * res * 10^0
                P128.w[1] = p_sign | 0x3040000000000000u64 | res.w[1];
                P128.w[0] = res.w[0];
                bid_rounding_correction(
                    rnd_mode,
                    is_inexact_lt_midpoint,
                    is_inexact_gt_midpoint,
                    is_midpoint_lt_even,
                    is_midpoint_gt_even,
                    0, &mut P128, pfpsf);
                scale = (((P128.w[1] & MASK_EXP) >> 49) - 6176) as i32; // -1, 0, or +1
                // the number of digits in the significand is p34 = 34
                if e4 + scale < EXP_MIN_UNBIASED {
                    is_tiny = true;
                }
            }

            // the result rounded to the destination precision with unbounded exponent
            // is (-1)^p_sign * res * 10^e4
            res.w[1] |= p_sign | (((e4 + 6176) as BID_UINT64) << 49); // RN
            // res.w[0] unchanged;
            // Note: res is correct only if expmin <= e4 <= expmax
            ind = p34; // the number of decimal digits in the signifcand of res

            // at this point we have the result rounded with unbounded exponent in
            // res and we know its tininess:
            // res = (-1)^p_sign * significand * 10^e4,
            // where q (significand) = ind = p34
            // Note: res is correct only if expmin <= e4 <= expmax

            // check for overflow if RN
            if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && (ind + e4) > (p34 + EXP_MAX_UNBIASED) {
                res.w[1] = p_sign | 0x7800000000000000u64;
                res.w[0] = 0x0000000000000000u64;
                *pfpsf  |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
                *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
                *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
                *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
                *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

                #[cfg(target_endian = "big")]
                BID_SWAP128(&mut res);

                return res;
            } // else not overflow or not RN, so continue

            // from this point on this is similar to the last part of the computation
            // for Cases (15), (16), (17)

            // if (e4 >= expmin) we have the result rounded with bounded exponent
            if e4 < EXP_MIN_UNBIASED {
                x0 = EXP_MIN_UNBIASED - e4; // x0 >= 1; the number of digits to chop off of res
                // where the result rounded [at most] once is
                //   (-1)^p_sign * significand_res * 10^e4

                // avoid double rounding error
                is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
                is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
                is_midpoint_lt_even0    = is_midpoint_lt_even;
                is_midpoint_gt_even0    = is_midpoint_gt_even;
                is_inexact_lt_midpoint  = false;
                is_inexact_gt_midpoint  = false;
                is_midpoint_lt_even     = false;
                is_midpoint_gt_even     = false;

                if x0 > ind {
                    // nothing is left of res when moving the decimal point left x0 digits
                    is_inexact_lt_midpoint = true;
                    res.w[1]               = p_sign | 0x0000000000000000u64;
                    res.w[0]               = 0x0000000000000000u64;
                    e4                     = EXP_MIN_UNBIASED;
                } else if x0 == ind { // 1 <= x0 = ind <= p34 = 34
                    // this is <, =, or > 1/2 ulp
                    // compare the ind-digit value in the significand of res with
                    // 1/2 ulp = 5*10^(ind-1), i.e. determine whether it is
                    // less than, equal to, or greater than 1/2 ulp (significand of res)
                    R128.w[1] = res.w[1] & MASK_COEFF;
                    R128.w[0] = res.w[0];
                    if ind <= 19 {
                        match bid_midpoint64[(ind - 1) as usize] {
                            val if R128.w[0] < val => {  // < 1/2 ulp
                                lt_half_ulp            = true;
                                is_inexact_lt_midpoint = true;
                            },
                            val if R128.w[0] == val => { // = 1/2 ulp
                                eq_half_ulp         = true;
                                is_midpoint_gt_even = true;
                            },
                            _=> {                                   // > 1/2 ulp
                                gt_half_ulp            = true;
                                is_inexact_gt_midpoint = true;
                            }
                        }
                    } else { // if (ind <= 38)
                        if  R128.w[1]  < bid_midpoint128[(ind - 20) as usize].w[1]
                        || (R128.w[1] == bid_midpoint128[(ind - 20) as usize].w[1]
                        &&  R128.w[0]  < bid_midpoint128[(ind - 20) as usize].w[0]) {      // < 1/2 ulp
                            lt_half_ulp            = true;
                            is_inexact_lt_midpoint = true;
                        } else if R128.w[1] == bid_midpoint128[(ind - 20) as usize].w[1]
                               && R128.w[0] == bid_midpoint128[(ind - 20) as usize].w[0] { // = 1/2 ulp
                            eq_half_ulp         = true;
                            is_midpoint_gt_even = true;
                        } else { // > 1/2 ulp
                            gt_half_ulp            = true;
                            is_inexact_gt_midpoint = true;
                        }
                    }
                    if lt_half_ulp || eq_half_ulp {
                        // res = +0.0 * 10^expmin
                        res.w[1] = 0x0000000000000000u64;
                        res.w[0] = 0x0000000000000000u64;
                    } else { // if (gt_half_ulp)
                        // res = +1 * 10^expmin
                        res.w[1] = 0x0000000000000000u64;
                        res.w[0] = 0x0000000000000001u64;
                    }
                    res.w[1] |= p_sign;
                    e4        = EXP_MIN_UNBIASED;
                } else { // if (1 <= x0 <= ind - 1 <= 33)
                    // round the ind-digit result to ind - x0 digits

                    if ind <= 18 { // 2 <= ind <= 18
                        R64 = bid_round64_2_18(
                            ind, x0, res.w[0],
                            &mut incr_exp,
                            &mut is_midpoint_lt_even,
                            &mut is_midpoint_gt_even,
                            &mut is_inexact_lt_midpoint,
                            &mut is_inexact_gt_midpoint);
                        res.w[1] = 0x0;
                        res.w[0] = R64;
                    } else if ind <= 38 {
                        P128.w[1] = res.w[1] & MASK_COEFF;
                        P128.w[0] = res.w[0];
                        res = bid_round128_19_38(
                            ind, x0, &P128,
                            &mut incr_exp,
                            &mut is_midpoint_lt_even,
                            &mut is_midpoint_gt_even,
                            &mut is_inexact_lt_midpoint,
                            &mut is_inexact_gt_midpoint);
                    }
                    e4 += x0; // expmin
                    // we want the exponent to be expmin, so if incr_exp = 1 then
                    // multiply the rounded result by 10 - it will still fit in 113 bits
                    if incr_exp {
                        // 64 x 128 -> 128
                        P128.w[1] = res.w[1] & MASK_COEFF;
                        P128.w[0] = res.w[0];
                        res = __mul_64x128_to_128(bid_ten2k64[1], &P128);
                    }
                    res.w[1] = p_sign | (((e4 + 6176) as BID_UINT64) << 49) | (res.w[1] & MASK_COEFF);
                    // avoid a double rounding error
                    if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0)
                     && is_midpoint_lt_even { // double rounding error upward
                        // res = res - 1
                        res.w[0] -= 1;
                        if res.w[0] == 0xffffffffffffffffu64 {
                            res.w[1] -= 1;
                        }
                        // Note: a double rounding error upward is not possible; for this
                        // the result after the first rounding would have to be 99...95
                        // (35 digits in all), possibly followed by a number of zeros; this
                        // not possible in this underflow case
                        is_midpoint_lt_even    = false;
                        is_inexact_lt_midpoint = true;
                    } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even { // double rounding error downward
                        // res = res + 1
                        res.w[0] += 1;
                        if res.w[0] == 0 {
                            res.w[1] += 1;
                        }
                        is_midpoint_gt_even    = false;
                        is_inexact_gt_midpoint = true;
                    } else if !is_midpoint_lt_even    && !is_midpoint_gt_even
                           && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                        // if this second rounding was exact the result may still be
                        // inexact because of the first rounding
                        if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
                            is_inexact_gt_midpoint = true;
                        }
                        if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
                            is_inexact_lt_midpoint = true;
                        }
                    } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
                        // pu64ed up to a midpoint
                        is_inexact_lt_midpoint = true;
                        is_inexact_gt_midpoint = false;
                        is_midpoint_lt_even    = false;
                        is_midpoint_gt_even    = false;
                    } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
                        // pu64ed down to a midpoint
                        is_inexact_lt_midpoint = false;
                        is_inexact_gt_midpoint = true;
                        is_midpoint_lt_even    = false;
                        is_midpoint_gt_even    = false;
                    } else {
                        // ;
                    }
                }
            }
            // res contains the correct result
            // apply correction if not rounding to nearest
            if rnd_mode != RoundingMode::BID_ROUNDING_TO_NEAREST {
                bid_rounding_correction(
                    rnd_mode,
                    is_inexact_lt_midpoint,
                    is_inexact_gt_midpoint,
                    is_midpoint_lt_even, is_midpoint_gt_even,
                    e4, &mut res, pfpsf);
            }
            // correction needed for tininess detection before rounding
            #[cfg(not(feature = "DECIMAL_TINY_DETECTION_AFTER_ROUNDING"))]
            if (((res.w[1] & 0x7fffffffffffffffu64) == 0x0000314dc6448d93u64)  // 10^33*10^-6176_high
              && (res.w[0] == 0x38c15b0a00000000u64))                          // 10^33*10^-6176_low
            && (((rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST
               || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY)
              && (is_midpoint_lt_even || is_inexact_gt_midpoint))
            || ((((rnd_mode == RoundingMode::BID_ROUNDING_UP) && (res.w[1] & MASK_SIGN) == 0)
              || ((rnd_mode == RoundingMode::BID_ROUNDING_DOWN) && (res.w[1] & MASK_SIGN) != 0))
               && (is_midpoint_lt_even    || is_midpoint_gt_even
                || is_inexact_lt_midpoint || is_inexact_gt_midpoint))) {
                is_tiny = true;
            }
            if is_midpoint_lt_even    || is_midpoint_gt_even
            || is_inexact_lt_midpoint || is_inexact_gt_midpoint {
                // set the inexact flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                if is_tiny {
                    *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
                }
            }
            *ptr_is_midpoint_lt_even = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            #[cfg(target_endia = "big")]
            BID_SWAP128 (res);

            return res;
        } else if (p34 <= delta && delta + q3 <= q4)                    // Case (15)
               || (delta < p34 && p34 < delta + q3 && delta + q3 <= q4) // Case (16)
               || (delta + q3 <= p34 && p34 < q4) {                     // Case (17)

            // calculate first the result rounded to the destination precision, with
            // unbounded exponent

            res = bid_add_and_round(
                q3, q4, e4, delta, p34, z_sign, p_sign, &C3, &C4,
               rnd_mode,
               &mut is_midpoint_lt_even,
               &mut is_midpoint_gt_even,
               &mut is_inexact_lt_midpoint,
               &mut is_inexact_gt_midpoint,
               pfpsf);
            *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
            *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
            *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
            *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

            #[cfg(target_endian = "big")]
            BID_SWAP128(&mut res);

            return res;
        } else {
            // ;
        }

    } // end if delta < 0

    *ptr_is_midpoint_lt_even    = is_midpoint_lt_even;
    *ptr_is_midpoint_gt_even    = is_midpoint_gt_even;
    *ptr_is_inexact_lt_midpoint = is_inexact_lt_midpoint;
    *ptr_is_inexact_gt_midpoint = is_inexact_gt_midpoint;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut res);

    res
}

pub (crate) fn bid128_fma(x: &BID_UINT128, y: &BID_UINT128, z: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut is_midpoint_lt_even: bool = false;
    let mut is_midpoint_gt_even: bool = false;
    let mut is_inexact_lt_midpoint: bool = false;
    let mut is_inexact_gt_midpoint: bool = false;

    bid128_ext_fma(
        &mut is_midpoint_lt_even,
        &mut is_midpoint_gt_even,
        &mut is_inexact_lt_midpoint,
        &mut is_inexact_gt_midpoint,
        x, y, z,
        rnd_mode, pfpsf)
}

pub (crate) fn bid128ddd_fma(x: BID_UINT64, y: BID_UINT64, z: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut is_midpoint_lt_even: bool    = false;
    let mut is_midpoint_gt_even: bool    = false;
    let mut is_inexact_lt_midpoint: bool = false;
    let mut is_inexact_gt_midpoint: bool = false;

    let x1: BID_UINT128 = bid64_to_bid128(x, pfpsf);
    let y1: BID_UINT128 = bid64_to_bid128(y, pfpsf);
    let z1: BID_UINT128 = bid64_to_bid128(z, pfpsf);

    bid128_ext_fma(
        &mut is_midpoint_lt_even,
        &mut is_midpoint_gt_even,
        &mut is_inexact_lt_midpoint,
        &mut is_inexact_gt_midpoint,
        &x1, &y1, &z1,
        rnd_mode, pfpsf)
}

pub (crate) fn bid128ddq_fma(x: BID_UINT64, y: BID_UINT64, z: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut is_midpoint_lt_even    = false;
    let mut is_midpoint_gt_even    = false;
    let mut is_inexact_lt_midpoint = false;
    let mut is_inexact_gt_midpoint = false;

    let x1: BID_UINT128  = bid64_to_bid128(x, pfpsf);
    let y1: BID_UINT128  = bid64_to_bid128(y, pfpsf);

    bid128_ext_fma(
        &mut is_midpoint_lt_even,
        &mut is_midpoint_gt_even,
        &mut is_inexact_lt_midpoint,
        &mut is_inexact_gt_midpoint,
        &x1, &y1, z,
        rnd_mode, pfpsf)
}

pub (crate) fn bid128dqd_fma(x: BID_UINT64, y: &BID_UINT128, z: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut is_midpoint_lt_even    = false;
    let mut is_midpoint_gt_even    = false;
    let mut is_inexact_lt_midpoint = false;
    let mut is_inexact_gt_midpoint = false;

    let x1: BID_UINT128  = bid64_to_bid128(x, pfpsf);
    let z1: BID_UINT128  = bid64_to_bid128(z, pfpsf);

    bid128_ext_fma(
        &mut is_midpoint_lt_even,
        &mut is_midpoint_gt_even,
        &mut is_inexact_lt_midpoint,
        &mut is_inexact_gt_midpoint,
        &x1, y, &z1,
        rnd_mode, pfpsf)
}

pub (crate) fn bid128dqq_fma(x: BID_UINT64, y: &BID_UINT128, z: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut is_midpoint_lt_even    = false;
    let mut is_midpoint_gt_even    = false;
    let mut is_inexact_lt_midpoint = false;
    let mut is_inexact_gt_midpoint = false;

    let x1: BID_UINT128 = bid64_to_bid128(x, pfpsf);

    bid128_ext_fma(
        &mut is_midpoint_lt_even,
        &mut is_midpoint_gt_even,
        &mut is_inexact_lt_midpoint,
        &mut is_inexact_gt_midpoint,
        &x1, y, z,
        rnd_mode, pfpsf)
}

pub (crate) fn bid128qdd_fma(x: &BID_UINT128, y: BID_UINT64, z: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut is_midpoint_lt_even    = false;
    let mut is_midpoint_gt_even    = false;
    let mut is_inexact_lt_midpoint = false;
    let mut is_inexact_gt_midpoint = false;

    let y1: BID_UINT128 = bid64_to_bid128(y, pfpsf);
    let z1: BID_UINT128 = bid64_to_bid128(z, pfpsf);

    bid128_ext_fma(
        &mut is_midpoint_lt_even,
        &mut is_midpoint_gt_even,
        &mut is_inexact_lt_midpoint,
        &mut is_inexact_gt_midpoint,
        x, &y1, &z1,
        rnd_mode, pfpsf)
}

pub (crate) fn bid128qdq_fma(x: &BID_UINT128, y: BID_UINT64, z: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut is_midpoint_lt_even    = false;
    let mut is_midpoint_gt_even    = false;
    let mut is_inexact_lt_midpoint = false;
    let mut is_inexact_gt_midpoint = false;

    let y1: BID_UINT128 = bid64_to_bid128(y, pfpsf);

    bid128_ext_fma(
        &mut is_midpoint_lt_even,
        &mut is_midpoint_gt_even,
        &mut is_inexact_lt_midpoint,
        &mut is_inexact_gt_midpoint,
        x, &y1, z,
        rnd_mode, pfpsf)
}

pub (crate) fn bid128qqd_fma(x: &BID_UINT128, y: &BID_UINT128, z: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut is_midpoint_lt_even    = false;
    let mut is_midpoint_gt_even    = false;
    let mut is_inexact_lt_midpoint = false;
    let mut is_inexact_gt_midpoint = false;

    let z1: BID_UINT128 = bid64_to_bid128(z, pfpsf);

    bid128_ext_fma(
        &mut is_midpoint_lt_even,
        &mut is_midpoint_gt_even,
        &mut is_inexact_lt_midpoint,
        &mut is_inexact_gt_midpoint,
        x, y, &z1,
        rnd_mode, pfpsf)
}

// Note: bid128qqq_fma is represented by bid128_fma

// Note: bid64ddd_fma is represented by bid64_fma

pub (crate) fn bid64ddq_fma(x: BID_UINT64, y: BID_UINT64, z: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let x1: BID_UINT128 = bid64_to_bid128(x, pfpsf);
    let y1: BID_UINT128 = bid64_to_bid128(y, pfpsf);

    bid64qqq_fma(&x1, &y1, z, rnd_mode, pfpsf)
}

pub (crate) fn bid64dqd_fma(x: BID_UINT64, y: &BID_UINT128, z: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let x1: BID_UINT128 = bid64_to_bid128(x, pfpsf);
    let z1: BID_UINT128 = bid64_to_bid128(z, pfpsf);

    bid64qqq_fma(&x1, y, &z1, rnd_mode, pfpsf)
}

pub (crate) fn bid64dqq_fma(x: BID_UINT64, y: &BID_UINT128, z: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let x1: BID_UINT128 = bid64_to_bid128(x, pfpsf);

    bid64qqq_fma(&x1, y, z, rnd_mode, pfpsf)
}

pub (crate) fn bid64qdd_fma(x: &BID_UINT128, y: BID_UINT64, z: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let y1: BID_UINT128 = bid64_to_bid128(y, pfpsf);
    let z1: BID_UINT128 = bid64_to_bid128(z, pfpsf);

    bid64qqq_fma(x, &y1, &z1, rnd_mode, pfpsf)
}

pub (crate) fn bid64qdq_fma(x: &BID_UINT128, y: BID_UINT64, z: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let y1: BID_UINT128 = bid64_to_bid128(y, pfpsf);

    bid64qqq_fma(x, &y1, z, rnd_mode, pfpsf)
}

pub (crate) fn bid64qqd_fma(x: &BID_UINT128, y: &BID_UINT128, z: BID_UINT64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let z1: BID_UINT128 = bid64_to_bid128(z, pfpsf);

    bid64qqq_fma(x, y, &z1, rnd_mode, pfpsf)
}

pub (crate) fn bid64qqq_fma(x: &BID_UINT128, y: &BID_UINT128, z: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let mut is_midpoint_lt_even0: bool = false;
    let mut is_midpoint_gt_even0: bool = false;
    let mut is_inexact_lt_midpoint0: bool = false;
    let mut is_inexact_gt_midpoint0: bool = false;
    let mut is_midpoint_lt_even: bool = false;
    let mut is_midpoint_gt_even: bool = false;
    let mut is_inexact_lt_midpoint: bool = false;
    let mut is_inexact_gt_midpoint: bool = false;
    let mut incr_exp: bool = false;
    let res: BID_UINT128;
    let res128: BID_UINT128;
    let mut res1: BID_UINT64 = 0xbaddbaddbaddbaddu64;
    let save_fpsf: u32; // needed because of the call to bid128_ext_fma
    let sign: BID_UINT64;
    let exp: BID_UINT64;
    let mut unbexp: i32;
    let mut C: BID_UINT128 = Default::default();
    let mut tmp: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let nr_bits: i32;
    let mut q: i32;
    let mut x0: i32;
    let scale: i32;
    let mut lt_half_ulp:bool = false;
    let mut eq_half_ulp:bool = false;

    // Note: for rounding modes other than RN or RA, the result can be obtained
    // by rounding first to BID128 and then to BID64

    save_fpsf = *pfpsf; // sticky bits - caller value must be preserved
    *pfpsf    = 0;

    res = bid128_ext_fma(
        &mut is_midpoint_lt_even0,
        &mut is_midpoint_gt_even0,
        &mut is_inexact_lt_midpoint0,
        &mut is_inexact_gt_midpoint0,
        x, y, z,
        rnd_mode, pfpsf);

    if (rnd_mode == RoundingMode::BID_ROUNDING_DOWN) || (rnd_mode == RoundingMode::BID_ROUNDING_UP) ||
       (rnd_mode == RoundingMode::BID_ROUNDING_TO_ZERO) ||      // no double rounding error is possible
       ((res.w[BID_HIGH_128W] & MASK_NAN) == MASK_NAN) ||       // res=QNaN (cannot be SNaN)
       ((res.w[BID_HIGH_128W] & MASK_ANY_INF) == MASK_INF) {    // result is infinity
        res1 = bid128_to_bid64(&res, rnd_mode, pfpsf);
        // determine the unbiased exponent of the result
        unbexp = (((res1 >> 53) & 0x3ff) - 398) as i32;

        if (res1 & MASK_NAN) != MASK_NAN { // res1 not NaN
            // if subnormal, res1  must have exp = -398
            // if tiny and inexact set underflow and inexact status flags
            if (unbexp == -398)
             & ((res1 & MASK_BINARY_SIG1) < 1000000000000000u64)
             && (is_inexact_lt_midpoint0 || is_inexact_gt_midpoint0
              || is_midpoint_lt_even0    || is_midpoint_gt_even0) {
                // set the inexact flag and the underflow flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_UNDERFLOW_EXCEPTION;
            } else if is_inexact_lt_midpoint0  || is_inexact_gt_midpoint0
                   || is_midpoint_lt_even0     || is_midpoint_gt_even0 {
                // set the inexact flag and the underflow flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            }
            #[cfg(not(feature = "DECIMAL_TINY_DETECTION_AFTER_ROUNDING"))]
            // correction needed for tininess detection before rounding
            if ((res1 & 0x7fffffffffffffffu64) == 1000000000000000u64) &&
                // 10^15*10^-398
                (((rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST
                || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY)
               && (is_midpoint_lt_even || is_inexact_gt_midpoint))
              || ((((rnd_mode == RoundingMode::BID_ROUNDING_UP) && (res1 & MASK_SIGN) == 0)
                || ((rnd_mode == RoundingMode::BID_ROUNDING_DOWN) && (res1 & MASK_SIGN) != 0))
                 && (is_midpoint_lt_even    || is_midpoint_gt_even
                  || is_inexact_lt_midpoint || is_inexact_gt_midpoint))) {
                *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
            }
        } // else the result is NaN

        *pfpsf |= save_fpsf;
        return res1;
    } // else continue, and use rounding to nearest to round to 16 digits

    // at this point the result is rounded to nearest (even or away) to 34 digits
    // (or less if exact), and it is zero or finite non-zero canonical [sub]normal
    sign   = res.w[BID_HIGH_128W] & MASK_SIGN;  // 0 for positive, MASK_SIGN for negative
    exp    = res.w[BID_HIGH_128W] & MASK_EXP;   // biased and shifted left 49 bits
    unbexp = ((exp >> 49) - 6176) as i32;
    C.w[1] = res.w[BID_HIGH_128W] & MASK_COEFF;
    C.w[0] = res.w[BID_LOW_128W];

    if (C.w[1] == 0x0 && C.w[0] == 0x0) ||     // result is zero
        (unbexp <= (-398 - 35)) || (unbexp >= (369 + 16)) {
        // clear under/overflow
        res1    = bid128_to_bid64(&res, rnd_mode, pfpsf);
        *pfpsf |= save_fpsf;
        return res1;
    } // else continue

    // -398 - 34 <= unbexp <= 369 + 15
    if rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY {
        // apply correction, if needed, to make the result rounded to nearest-even
        if is_midpoint_gt_even {
            // res = res - 1
            res1 -= 1; // res1 is now even
        } // else the result is already correctly rounded to nearest-even
    }
    // at this point the result is finite, non-zero canonical normal or subnormal,
    // and in most cases overflow or underflow will not occur

    unsafe {
        // determine the number of digits q in the result
        // q = nr. of decimal digits in x
        // determine first the nr. of bits in x
        if C.w[1] == 0 {
            if C.w[0] >= 0x0020000000000000u64 {  // x >= 2^53
                // split the 64-bit value in two 32-bit halves to avoid rounding errors
                tmp.d   = (C.w[0] >> 32) as f64;    // exact conversion
                nr_bits = (33 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            } else { // if x < 2^53
                tmp.d   = C.w[0] as f64;            // exact conversion
                nr_bits = (1 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
            }
        } else { // C.w[1] != 0 => nr. bits = 64 + nr_bits (C.w[1])
            tmp.d   = C.w[1] as f64;              // exact conversion
            nr_bits = (65 + ((((tmp.i >> 52) as u32) & 0x7ff) - 0x3ff)) as i32;
        }
    }
    q = bid_nr_digits[(nr_bits - 1) as usize].digits as i32;
    if q == 0 {
        q = bid_nr_digits[(nr_bits - 1) as usize].digits1 as i32;
        if C.w[1]  > bid_nr_digits[(nr_bits - 1) as usize].threshold_hi
       || (C.w[1] == bid_nr_digits[(nr_bits - 1) as usize].threshold_hi
        && C.w[0] >= bid_nr_digits[(nr_bits - 1) as usize].threshold_lo) {
            q += 1;
        }
    }
    // if q > 16, round to nearest even to 16 digits (but for underflow it may
    // have to be truncated even more)
    if q > 16 {
        x0 = q - 16;
        if q <= 18 {
            res1 = bid_round64_2_18(
                q, x0, C.w[0], &mut incr_exp,
                &mut is_midpoint_lt_even, &mut is_midpoint_gt_even,
                &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
        } else { // 19 <= q <= 34
            res128 = bid_round128_19_38(
                q, x0, &C, &mut incr_exp,
                &mut is_midpoint_lt_even, &mut is_midpoint_gt_even,
                &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
            res1 = res128.w[0]; // the result fits in 64 bits
        }
        unbexp += x0;
        if incr_exp {
            unbexp += 1;
        }
        q = 16; // need to set in case denormalization is necessary
    } else {
        // the result does not require a second rounding (and it must have
        // been exact in the first rounding, since q <= 16)
        res1 = C.w[0];
    }

    // avoid a double rounding error
    if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even { // double rounding error upward
        // res = res - 1
        res1                  -= 1; // res1 becomes odd
        is_midpoint_lt_even    = false;
        is_inexact_lt_midpoint = true;
        if res1 == 0x00038d7ea4c67fffu64 {  // 10^15 - 1
            res1    = 0x002386f26fc0ffffu64;  // 10^16 - 1
            unbexp -= 1;
        }
    } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even { // double rounding error downward
        // res = res + 1
        res1                  += 1; // res1 becomes odd (so it cannot be 10^16)
        is_midpoint_gt_even    = false;
        is_inexact_gt_midpoint = true;
    } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
        // if this second rounding was exact the result may still be
        // inexact because of the first rounding
        if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
            is_inexact_gt_midpoint = true;
        }
        if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
            is_inexact_lt_midpoint = true;
        }
    } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
        // pu64ed up to a midpoint
        is_inexact_lt_midpoint = true;
        is_inexact_gt_midpoint = false;
        is_midpoint_lt_even    = false;
        is_midpoint_gt_even    = false;
    } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
        // pu64ed down to a midpoint
        is_inexact_lt_midpoint = false;
        is_inexact_gt_midpoint = true;
        is_midpoint_lt_even    = false;
        is_midpoint_gt_even    = false;
    } else {
        // ;
    }
    // this is the result rounded correctly to nearest even, with unbounded exp.

    // check for overflow
    if q + unbexp > P16 + EXP_MAX16_UNBIASED {
        res1    = sign | 0x7800000000000000u64;
        *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_OVERFLOW_EXCEPTION;
        *pfpsf |= save_fpsf;
        return res1
    } else if unbexp > EXP_MAX16_UNBIASED { // q + unbexp <= P16 + expmax16
        // not overflow; the result must be exact, and we can multiply res1 by
        // 10^(unbexp - expmax16) and the product will fit in 16 decimal digits
        scale  = unbexp - EXP_MAX16_UNBIASED;
        res1  *= bid_ten2k64[scale as usize];   // res1 * 10^scale
        unbexp = EXP_MAX16_UNBIASED;                      // unbexp - scale
    } else {
        // continue
    }

    // check for underflow
    if q + unbexp < P16 + EXP_MIN16_UNBIASED {
        if unbexp < EXP_MIN16_UNBIASED {
            // we must truncate more of res
            x0                      = EXP_MIN16_UNBIASED - unbexp; // x0 >= 1
            is_inexact_lt_midpoint0 = is_inexact_lt_midpoint;
            is_inexact_gt_midpoint0 = is_inexact_gt_midpoint;
            is_midpoint_lt_even0    = is_midpoint_lt_even;
            is_midpoint_gt_even0    = is_midpoint_gt_even;
            is_inexact_lt_midpoint  = false;
            is_inexact_gt_midpoint  = false;
            is_midpoint_lt_even     = false;
            is_midpoint_gt_even     = false;
            // the number of decimal digits in res1 is q
            if x0 < q { // 1 <= x0 <= q-1 => round res to q - x0 digits
                // 2 <= q <= 16, 1 <= x0 <= 15
                res1 = bid_round64_2_18(
                    q, x0, res1, &mut incr_exp,
                    &mut is_midpoint_lt_even, &mut is_midpoint_gt_even,
                    &mut is_inexact_lt_midpoint, &mut is_inexact_gt_midpoint);
                if incr_exp {
                    // res1 = 10^(q-x0), 1 <= q - x0 <= q - 1, 1 <= q - x0 <= 15
                    res1 = bid_ten2k64[(q - x0) as usize];
                }
                unbexp += x0; // expmin16
            } else if x0 == q {
                // the second rounding is for 0.d(0)d(1)...d(q-1) * 10^emin
                // determine relationship with 1/2 ulp
                // q <= 16
                match bid_midpoint64[(q - 1) as usize] {
                    val if res1 < val => {  // < 1/2 ulp
                        lt_half_ulp            = true;
                        is_inexact_lt_midpoint = true;
                    },
                    val if res1 == val => { // = 1/2 ulp
                        eq_half_ulp            = true;
                        is_midpoint_gt_even    = true;
                    },
                    _ => {                              // > 1/2 ulp
                        // gt_half_ulp = 1;
                        is_inexact_gt_midpoint = true;
                    }
                }
                res1 = if lt_half_ulp || eq_half_ulp {
                    // res = +0.0 * 10^expmin16
                    0x0000000000000000u64
                } else { // if (gt_half_ulp)
                    // res = +1 * 10^expmin16
                    0x0000000000000001u64
                };
                unbexp = EXP_MIN16_UNBIASED;
            } else { // if (x0 > q)
                // the second rounding is for 0.0...d(0)d(1)...d(q-1) * 10^emin
                res1                   = 0x0000000000000000u64;
                unbexp                 = EXP_MIN16_UNBIASED;
                is_inexact_lt_midpoint = true;
            }
            // avoid a double rounding error
            if (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) && is_midpoint_lt_even { // double rounding error upward
                // res = res - 1
                res1                  -= 1; // res1 becomes odd
                is_midpoint_lt_even    = false;
                is_inexact_lt_midpoint = true;
            } else if (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) && is_midpoint_gt_even { // double rounding error downward
                // res = res + 1
                res1                  += 1; // res1 becomes odd
                is_midpoint_gt_even    = false;
                is_inexact_gt_midpoint = true;
            } else if !is_midpoint_lt_even && !is_midpoint_gt_even && !is_inexact_lt_midpoint && !is_inexact_gt_midpoint {
                // if this rounding was exact the result may still be
                // inexact because of the previous roundings
                if is_inexact_gt_midpoint0 || is_midpoint_lt_even0 {
                    is_inexact_gt_midpoint = true;
                }
                if is_inexact_lt_midpoint0 || is_midpoint_gt_even0 {
                    is_inexact_lt_midpoint = true;
                }
            } else if is_midpoint_gt_even && (is_inexact_gt_midpoint0 || is_midpoint_lt_even0) {
                // pu64ed up to a midpoint
                is_inexact_lt_midpoint = true;
                is_inexact_gt_midpoint = false;
                is_midpoint_lt_even    = false;
                is_midpoint_gt_even    = false;
            } else if is_midpoint_lt_even && (is_inexact_lt_midpoint0 || is_midpoint_gt_even0) {
                // pu64ed down to a midpoint
                is_inexact_lt_midpoint = false;
                is_inexact_gt_midpoint = true;
                is_midpoint_lt_even    = false;
                is_midpoint_gt_even    = false;
            } else {
                // ;
            }
        }
        // else if unbexp >= emin then q < P (because q + unbexp < P16 + expmin16)
        // and the result is tiny and exact

        // check for inexact result
        if is_inexact_lt_midpoint  || is_inexact_gt_midpoint
        || is_midpoint_lt_even     || is_midpoint_gt_even
        || is_inexact_lt_midpoint0 || is_inexact_gt_midpoint0
        || is_midpoint_lt_even0    || is_midpoint_gt_even0 {
            // set the inexact flag and the underflow flag
            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION | StatusFlags::BID_UNDERFLOW_EXCEPTION;
        }
    } else if is_inexact_lt_midpoint || is_inexact_gt_midpoint
           || is_midpoint_lt_even    || is_midpoint_gt_even {
        *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
    }
    // this is the result rounded correctly to nearest, with bounded exponent

    if rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY && is_midpoint_gt_even { // correction
        // res = res + 1
        res1 += 1; // res1 is now odd
    } // else the result is already correct

    // assemble the result
    res1 = if res1 < 0x0020000000000000u64 { // res < 2^53
        sign | (((unbexp + 398) as BID_UINT64) << 53) | res1
    } else { // res1 >= 2^53
        sign | MASK_STEERING_BITS | (((unbexp + 398) as BID_UINT64) << 51) | (res1 & MASK_BINARY_SIG2)
    };
    #[cfg(not(feature = "DECIMAL_TINY_DETECTION_AFTER_ROUNDING"))]
    // correction needed for tininess detection before rounding
    if ((res1 & 0x7fffffffffffffffu64) == 1000000000000000u64) &&
        // 10^15*10^-398
        (((rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST || rnd_mode == RoundingMode::BID_ROUNDING_TIES_AWAY) &&
          (is_midpoint_lt_even || is_inexact_gt_midpoint)) ||
        ((((rnd_mode == RoundingMode::BID_ROUNDING_UP) && (res1 & MASK_SIGN) == 0) ||
          ((rnd_mode == RoundingMode::BID_ROUNDING_DOWN) && (res1 & MASK_SIGN) == MASK_SIGN))
        && (is_midpoint_lt_even    || is_midpoint_gt_even ||
            is_inexact_lt_midpoint || is_inexact_gt_midpoint))) {
        *pfpsf |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
    }
    *pfpsf |= save_fpsf;
    res1
}