/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid_decimal_data::{BID_ESTIMATE_DECIMAL_DIGITS, BID_POWER10_TABLE_128, BID_RECIP_SCALE, BID_RECIPROCALS10_128, BID_ROUND_CONST_TABLE_128};
use crate::bid_internal::*;
use crate::d128::{_IDEC_flags, StatusFlags, RoundingMode};

/// Quantize(x, y) is a floating-point number in the same format that
/// has, if possible, the same numerical value as x and the same quantum
/// (unit-in-the-last-place) as y. If the exponent is being increased, rounding
/// according to the prevailing rounding-direction mode might occur: the result
/// is a different floating-point representation and inexact is signaled if the
/// result does not have the same numerical value as x. If the exponent is being
/// decreased and the significand of the result would have more than 34 digits,
/// invalid is signaled and the result is NaN. If one or both operands are NaN
/// the rules for NaNs are followed. Otherwise if only one operand is
/// infinite then invalid is signaled and the result is NaN. If both operands
/// are infinite then the result is canonical infinity with the sign of x
pub (crate) fn bid128_quantize(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: RoundingMode, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let CT: BID_UINT256;
    let mut CX: BID_UINT128 = Default::default();
    let mut CY: BID_UINT128 = Default::default();
    let T: &BID_UINT128;
    let mut CX2: BID_UINT128 = Default::default();
    let mut CR: BID_UINT128 = Default::default();
    let mut Stemp: BID_UINT128 = Default::default();
    let mut res: BID_UINT128 = Default::default();
    let mut REM_H: BID_UINT128 = Default::default();
    let mut C2N: BID_UINT128 = Default::default();
    let mut sign_x: BID_UINT64 = 0;
    let mut sign_y: BID_UINT64 = 0;
    let remainder_h: BID_UINT64;
    let carry: BID_UINT64;
    let CY64: BID_UINT64;
    let valid_x: BID_UINT64;
    let mut tempx: BID_UI32FLOAT = Default::default();
    let mut exponent_x: i32 = 0;
    let mut exponent_y: i32 = 0;
    let mut digits_x: i32;
    let extra_digits: i32;
    let amount: i32;
    let expon_diff: i32;
    let total_digits: i32;
    let bin_expon_cx: i32;
    let mut rmode: RoundingMode;
    let mut status: _IDEC_flags;

    // BID_OPT_SAVE_BINARY_FLAGS()

    valid_x = unpack_BID128_value(&mut sign_x, &mut exponent_x, &mut CX, x);

    // unpack arguments, check for NaN or Infinity
    if unpack_BID128_value(&mut sign_y, &mut exponent_y, &mut CY, y) == 0 {
        // y is Inf. or NaN
        if (x.w[1] & SNAN_MASK64) == SNAN_MASK64 {	// y is sNaN
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
        }

        // test if y is NaN
        if (y.w[1] & 0x7c00000000000000u64) == 0x7c00000000000000u64 {
            if (y.w[1] & 0x7e00000000000000u64) == 0x7e00000000000000u64 {
                // set status flags
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            if (x.w[1] & 0x7c00000000000000u64) != 0x7c00000000000000u64 {
                res.w[1] = CY.w[1] & QUIET_MASK64;
                res.w[0] = CY.w[0];
            } else {
                res.w[1] = CX.w[1] & QUIET_MASK64;
                res.w[0] = CX.w[0];
            }
            return res;
        }
        // y is Infinity?
        if (y.w[1] & 0x7800000000000000u64) == 0x7800000000000000u64 {
            // check if x is not Inf.
            if (x.w[1] & 0x7c00000000000000u64) < 0x7800000000000000u64 {
                // return NaN
                // set status flags
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
                res.w[1] = 0x7c00000000000000u64;
                res.w[0] = 0;
                return res;
            } else if (x.w[1] & 0x7c00000000000000u64) <= 0x7800000000000000u64 {
                res.w[1] = CX.w[1] & QUIET_MASK64;
                res.w[0] = CX.w[0];
                return res;
            }
        }
    }

    if valid_x == 0 {
        // test if x is NaN or Inf
        if (x.w[1] & 0x7c00000000000000u64) == 0x7800000000000000u64 {
            // set status flags
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0;
            return res;
        } else if (x.w[1] & 0x7c00000000000000u64) == 0x7c00000000000000u64 {
            if (x.w[1] & 0x7e00000000000000u64) == 0x7e00000000000000u64 {
                // set status flags
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            res.w[1] = CX.w[1] & QUIET_MASK64;
            res.w[0] = CX.w[0];
            return res;
        }
        if CX.w[1] == 0 && CX.w[0] == 0 {
            res = bid_get_BID128_very_fast(sign_x, exponent_y, &CX);
            return res;
        }
    }

    unsafe {
        // get number of decimal digits in coefficient_x
        if CX.w[1] != 0 {
            tempx.d      = CX.w[1] as f32;
            bin_expon_cx = (((tempx.ui32 >> 23) & 0xff) - 0x7f + 64) as i32;
        } else {
            tempx.d      = CX.w[0] as f32;
            bin_expon_cx = (((tempx.ui32 >> 23) & 0xff) - 0x7f) as i32;
        }
    }

    digits_x = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon_cx as usize];
    if  CX.w[1]  > BID_POWER10_TABLE_128[digits_x as usize].w[1]
    || (CX.w[1] == BID_POWER10_TABLE_128[digits_x as usize].w[1]
     && CX.w[0] >= BID_POWER10_TABLE_128[digits_x as usize].w[0]) {
        digits_x += 1;
    }

    expon_diff   = exponent_x - exponent_y;
    total_digits = digits_x + expon_diff;

    if (total_digits as BID_UINT32) <= 34 {
        if expon_diff >= 0 {
            T   = &BID_POWER10_TABLE_128[expon_diff as usize];
            CX2 = __mul_128x128_low(T, &CX);
            res = bid_get_BID128_very_fast(sign_x, exponent_y, &CX2);
            return res;
        }
        rmode = rnd_mode;
        if sign_x != 0 && ((rmode as u32 - 1u32) < 2) {
            rmode = RoundingMode::from(3 - (rmode as u32));
        }
        // must round off -expon_diff digits
        extra_digits = -expon_diff;
        CX           = __add_128_128(&CX, &BID_ROUND_CONST_TABLE_128[rmode as usize][extra_digits as usize]);

        // get P*(2^M[extra_digits])/10^extra_digits
        CT = __mul_128x128_to_256(&CX, &BID_RECIPROCALS10_128[extra_digits as usize]);

        // now get P/10^extra_digits: shift C64 right by M[extra_digits]-128
        amount   = BID_RECIP_SCALE[extra_digits as usize];
        CX2.w[0] = CT.w[2];
        CX2.w[1] = CT.w[3];
        if amount >= 64 {
            CR.w[1] = 0;
            CR.w[0] = CX2.w[1] >> (amount - 64);
        } else {
            CR = __shr_128(&CX2, amount);
        }

        if rnd_mode == RoundingMode::BID_ROUNDING_TO_NEAREST && (CR.w[0] & 1) == 1 {
            // check whether fractional part of initial_P/10^extra_digits is
            // exactly .5 this is the same as fractional part of
            // (initial_P + 0.5*10^extra_digits)/10^extra_digits is exactly zero

            // get remainder
            remainder_h = if amount >= 64 {
                 CX2.w[0] | (CX2.w[1] << (128 - amount))
            } else {
                CX2.w[0] << (64 - amount)
            };

            // test whether fractional part is 0
            if remainder_h == 0
            && (CT.w[1]  < BID_RECIPROCALS10_128[extra_digits as usize].w[1]
            || (CT.w[1] == BID_RECIPROCALS10_128[extra_digits as usize].w[1]
             && CT.w[0]  < BID_RECIPROCALS10_128[extra_digits as usize].w[0])) {
                CR.w[0] -= 1;
            }
        }

        status = StatusFlags::BID_INEXACT_EXCEPTION;

        // get remainder
        if amount >= 64 {
            REM_H.w[1] = CX2.w[1] << (128 - amount);
            REM_H.w[0] = CX2.w[0];
        } else {
            REM_H.w[1] = CX2.w[0] << (64 - amount);
            REM_H.w[0] = 0;
        }

        match rmode {
            RoundingMode::BID_ROUNDING_TO_NEAREST | RoundingMode::BID_ROUNDING_TIES_AWAY => {
                // test whether fractional part is 0
                if REM_H.w[1] == 0x8000000000000000u64 && REM_H.w[0] == 0
                && (CT.w[1]  < BID_RECIPROCALS10_128[extra_digits as usize].w[1]
                || (CT.w[1] == BID_RECIPROCALS10_128[extra_digits as usize].w[1]
                 && CT.w[0]  < BID_RECIPROCALS10_128[extra_digits as usize].w[0])) {
                    status = StatusFlags::BID_EXACT_STATUS;
                 }
            },
            RoundingMode::BID_ROUNDING_DOWN | RoundingMode::BID_ROUNDING_TO_ZERO => {
                if (REM_H.w[1] | REM_H.w[0]) == 0
                && (CT.w[1]  < BID_RECIPROCALS10_128[extra_digits as usize].w[1]
                || (CT.w[1] == BID_RECIPROCALS10_128[extra_digits as usize].w[1]
                 && CT.w[0]  < BID_RECIPROCALS10_128[extra_digits as usize].w[0])) {
                    status = StatusFlags::BID_EXACT_STATUS;
                }
            },
            _ => {
                // round up
                (Stemp.w[0], CY64)  = __add_carry_out(CT.w[0], BID_RECIPROCALS10_128[extra_digits as usize].w[0]);
                (Stemp.w[1], carry) = __add_carry_in_out(CT.w[1], BID_RECIPROCALS10_128[extra_digits as usize].w[1], CY64);
                if amount < 64 {
                    C2N.w[1]   = 0;
                    C2N.w[0]   = (1 as BID_UINT64) << amount;
                    REM_H.w[0] = REM_H.w[1] >> (64 - amount);
                    REM_H.w[1] = 0;
                } else {
                    C2N.w[1]     = (1 as BID_UINT64) << (amount - 64);
                    C2N.w[0]     = 0;
                    REM_H.w[1] >>= 128 - amount;
                }
                REM_H.w[0] += carry;
                if REM_H.w[0] < carry {
                    REM_H.w[1] += 1;
                }
                if __unsigned_compare_ge_128(&REM_H, &C2N) {
                    status = StatusFlags::BID_EXACT_STATUS;
                }
            }
        }

        __set_status_flags (pfpsf, status);

        res = bid_get_BID128_very_fast(sign_x, exponent_y, &CR);

        return res;
    }
    if total_digits < 0 {
        CR.w[1] = 0;
        CR.w[0] = 0;
        rmode   = rnd_mode;
        if sign_x != 0 && ((rmode as u32 - 1u32) < 2) {
            rmode = RoundingMode::from(3 - (rmode as u32));
        }
        if rmode == RoundingMode::BID_ROUNDING_UP {
            CR.w[0] = 1;
        }
        __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
        res = bid_get_BID128_very_fast(sign_x, exponent_y, &CR);
        return res;
    }
    // else  more than 34 digits in coefficient
    __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
    res.w[1] = 0x7c00000000000000u64;
    res.w[0] = 0;
    res
}
