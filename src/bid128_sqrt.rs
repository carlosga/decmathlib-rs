/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid_decimal_data::{BID_ESTIMATE_DECIMAL_DIGITS, BID_POWER10_INDEX_BINEXP_128, BID_POWER10_TABLE_128};
use crate::bid_internal::*;
use crate::bid_sqrt_macros::{bid_long_sqrt128, short_sqrt128};
use crate::d128::{_IDEC_flags, StatusFlags, RoundingMode};

/// Decimal floating-point square root, UINT128 -> UINT128
pub(crate) fn bid128_sqrt(x: &BID_UINT128, rnd_mode: RoundingMode, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut M256: BID_UINT256 = Default::default();
    let C256: BID_UINT256;
    let mut C4: BID_UINT256 = Default::default();
    let mut C8: BID_UINT256 = Default::default();
    let mut CX: BID_UINT128 = Default::default();
    let CX1: BID_UINT128;
    let mut CX2: BID_UINT128 = Default::default();
    let mut A10: BID_UINT128;
    let S2: BID_UINT128;
    let T128: &BID_UINT128;
    let TP128: &BID_UINT128;
    let mut CS: BID_UINT128 = Default::default();
    let mut CSM: BID_UINT128 = Default::default();
    let mut res: BID_UINT128 = Default::default();
    let mut sign_x: BID_UINT64 = 0;
    let mut Carry: BID_UINT64;
    let D: BID_SINT64;
    let mut fx: BID_UI32FLOAT = Default::default();
    let mut f64: BID_UI32FLOAT = Default::default();
    let mut exponent_x: i32 = 0;
    let bin_expon_cx: i32;
    let mut digits: i32;
    let mut scale: i32;
    let exponent_q: i32;

    // unpack arguments, check for NaN or Infinity
    if unpack_BID128_value(&mut sign_x, &mut exponent_x, &mut CX, x) == 0 {
        res.w[1] = CX.w[1];
        res.w[0] = CX.w[0];
        // NaN ?
        if (x.w[1] & 0x7c00000000000000u64) == 0x7c00000000000000u64 {
            if (x.w[1] & 0x7e00000000000000u64) == 0x7e00000000000000u64 {
                // sNaN
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            res.w[1] = CX.w[1] & QUIET_MASK64;
            return res;
        }
        // x is Infinity?
        if (x.w[1] & 0x7800000000000000u64) == 0x7800000000000000u64 {
            res.w[1] = CX.w[1];
            if sign_x != 0 {
                // -Inf, return NaN
                res.w[1] = 0x7c00000000000000u64;
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            return res;
        }
        // x is 0 otherwise

        res.w[1] = sign_x | ((((exponent_x + DECIMAL_EXPONENT_BIAS_128) as BID_UINT64) >> 1) << 49);
        res.w[0] = 0;
        return res;
    }
    if sign_x != 0 {
        res.w[1] = 0x7c00000000000000u64;
        res.w[0] = 0;
        __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
        return res;
    }

    unsafe {
        // 2^64
        f64.ui32 = 0x5f800000;

        // fx ~ CX
        fx.d         = (CX.w[1] as f32) * f64.d + (CX.w[0] as f32);
        bin_expon_cx = (((fx.ui32 >> 23) & 0xff) - 0x7f) as i32;
        digits       = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon_cx as usize];
    }

    A10 = CX;
    if (exponent_x & 1) == 1 {
        A10.w[1] = (CX.w[1] << 3) | (CX.w[0] >> 61);
        A10.w[0] = CX.w[0] << 3;
        CX2.w[1] = (CX.w[1] << 1) | (CX.w[0] >> 63);
        CX2.w[0] = CX.w[0] << 1;
        A10      = __add_128_128(&A10, &CX2);
    }

    CS.w[0] = short_sqrt128(&A10);
    CS.w[1] = 0;
    // check for exact result
    if CS.w[0] * CS.w[0] == A10.w[0] {
        S2 = __mul_64x64_to_128_fast(CS.w[0], CS.w[0]);
        if S2.w[1] == A10.w[1] {
            // && S2.w[0]==A10.w[0])
            res = bid_get_BID128_very_fast(0, (exponent_x + DECIMAL_EXPONENT_BIAS_128) >> 1, &CS);
            return res;
        }
    }
    // get number of digits in CX
    D = (CX.w[1] - BID_POWER10_INDEX_BINEXP_128[bin_expon_cx as usize].w[1]) as BID_SINT64;
    if D > 0 || (D == 0 && CX.w[0] >= BID_POWER10_INDEX_BINEXP_128[bin_expon_cx as usize].w[0]) {
        digits += 1;
    }

    // if exponent is odd, scale coefficient by 10
    scale      = 67 - digits;
    exponent_q = exponent_x - scale;
    scale     += exponent_q & 1; // exp. bias is even

    if scale > 38 {
        T128 = &BID_POWER10_TABLE_128[(scale - 37) as usize];
        CX1  = __mul_128x128_low(&CX, T128);

        TP128 = &BID_POWER10_TABLE_128[37];
        C256  = __mul_128x128_to_256(&CX1, TP128);
    } else {
        T128 = &BID_POWER10_TABLE_128[scale as usize];
        C256 = __mul_128x128_to_256(&CX, T128);
    }

    // 4*C256
    C4.w[3] = (C256.w[3] << 2) | (C256.w[2] >> 62);
    C4.w[2] = (C256.w[2] << 2) | (C256.w[1] >> 62);
    C4.w[1] = (C256.w[1] << 2) | (C256.w[0] >> 62);
    C4.w[0] = C256.w[0] << 2;

    bid_long_sqrt128(&mut CS, &C256);
    //printf("C256=%016I64x %016I64x %016I64x %016I64x, CS=%016I64x %016I64x \n",C256.w[3],C256.w[2],C256.w[1],C256.w[0],CS.w[1],CS.w[0]);

    if ((rnd_mode as u32) & 3) == 0 {
        // compare to midpoints
        CSM.w[1] = (CS.w[1] << 1) | (CS.w[0] >> 63);
        CSM.w[0] = (CS.w[0] + CS.w[0]) | 1;
        // CSM^2
        //__mul_128x128_to_256(M256, CSM, CSM);
        __sqr128_to_256(&mut M256, &CSM);

        if  C4.w[3]  > M256.w[3]
        || (C4.w[3] == M256.w[3]
        && (C4.w[2]  > M256.w[2]
        || (C4.w[2] == M256.w[2]
        && (C4.w[1]  > M256.w[1]
        || (C4.w[1] == M256.w[1]
         && C4.w[0]  > M256.w[0]))))) {
            // round up
            CS.w[0] += 1;
            if CS.w[0] == 0 {
                CS.w[1] += 1;
            }
        } else {
            C8.w[1] = (CS.w[1] << 3) | (CS.w[0] >> 61);
            C8.w[0] = CS.w[0] << 3;
            // M256 - 8*CSM
            (M256.w[0], Carry) = __sub_borrow_out(M256.w[0], C8.w[0]);
            (M256.w[1], Carry) = __sub_borrow_in_out(M256.w[1], C8.w[1], Carry);
            (M256.w[2], Carry) = __sub_borrow_in_out(M256.w[2], 0, Carry);
            M256.w[3]         -= Carry;

            // if CSM' > C256, round up
            if  M256.w[3]  > C4.w[3]
            || (M256.w[3] == C4.w[3]
            && (M256.w[2]  > C4.w[2]
            || (M256.w[2] == C4.w[2]
            && (M256.w[1]  > C4.w[1]
            || (M256.w[1] == C4.w[1]
             && M256.w[0]  > C4.w[0]))))) {
                // round down
                if CS.w[0] == 0 {
                    CS.w[1] -= 1;
                }
                CS.w[0] -= 1;
            }
        }
    } else {
        __sqr128_to_256(&mut M256, &CS);
        C8.w[1] = (CS.w[1] << 1) | (CS.w[0] >> 63);
        C8.w[0] = CS.w[0] << 1;
        if  M256.w[3]  > C256.w[3]
        || (M256.w[3] == C256.w[3]
        && (M256.w[2]  > C256.w[2]
        || (M256.w[2] == C256.w[2]
        && (M256.w[1]  > C256.w[1]
        || (M256.w[1] == C256.w[1]
         && M256.w[0]  > C256.w[0]))))) {
            (M256.w[0], Carry) = __sub_borrow_out(M256.w[0], C8.w[0]);
            (M256.w[1], Carry) = __sub_borrow_in_out(M256.w[1], C8.w[1], Carry);
            (M256.w[2], Carry) = __sub_borrow_in_out(M256.w[2], 0, Carry);
            M256.w[3]         -= Carry;
            M256.w[0]         += 1;
            if M256.w[0] == 0 {
                M256.w[1] += 1;
                if M256.w[1] == 0 {
                    M256.w[2] += 1;
                    if M256.w[2] == 0 {
                        M256.w[3] += 1;
                    }
                }
            }

            if CS.w[0] == 0 {
                CS.w[1] -= 1;
            }
            CS.w[0] -= 1;

            if  M256.w[3]  > C256.w[3]
            || (M256.w[3] == C256.w[3]
            && (M256.w[2]  > C256.w[2]
            || (M256.w[2] == C256.w[2]
            && (M256.w[1]  > C256.w[1]
            || (M256.w[1] == C256.w[1]
             && M256.w[0]  > C256.w[0]))))) {
                if CS.w[0] == 0 {
                    CS.w[1] -= 1;
                }
                CS.w[0] -= 1;
            }
        } else {
            (M256.w[0], Carry) = __add_carry_out(M256.w[0], C8.w[0]);
            (M256.w[1], Carry) = __add_carry_in_out(M256.w[1], C8.w[1], Carry);
            (M256.w[2], Carry) = __add_carry_in_out(M256.w[2], 0, Carry);
            M256.w[3]         += Carry;
            M256.w[0]         += 1;
            if M256.w[0] == 0 {
                M256.w[1] += 1;
                if M256.w[1] == 0 {
                    M256.w[2] += 1;
                    if M256.w[2] == 0 {
                        M256.w[3] += 1;
                    }
                }
            }
            if  M256.w[3]  < C256.w[3]
            || (M256.w[3] == C256.w[3]
            && (M256.w[2]  < C256.w[2]
            || (M256.w[2] == C256.w[2]
            && (M256.w[1]  < C256.w[1]
            || (M256.w[1] == C256.w[1]
             && M256.w[0] <= C256.w[0]))))) {
                CS.w[0] += 1;
                if CS.w[0] != 0 {
                    CS.w[1] += 1;
                }
            }
        }
        // RU?
        if (rnd_mode) == RoundingMode::Upward {
            CS.w[0] += 1;
            if CS.w[0] == 0 {
                CS.w[1] += 1;
            }
        }
    }

    __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
    let mut expon = (exponent_q + DECIMAL_EXPONENT_BIAS_128) >> 1;
    res = bid_get_BID128_fast(0, &mut expon, &mut CS);
    res
}

/*

    BID128_FUNCTION_ARGTYPE1 (bid128d_sqrt, BID_UINT64, x)

    BID_UINT256 M256, C256, C4, C8;
    BID_UINT128 CX, CX1, CX2, A10, S2, T128, TP128, CS, CSM, res;
    BID_UINT64 sign_x, Carry;
    BID_SINT64 D;
    int_float fx, f64;
    int exponent_x, bin_expon_cx;
    int digits, scale, exponent_q;

    BID_OPT_SAVE_BINARY_FLAGS()

    // unpack arguments, check for NaN or Infinity
    // unpack arguments, check for NaN or Infinity
    CX.w[1] = 0;
    if (!unpack_BID64 (&sign_x, &exponent_x, &CX.w[0], x)) {
        res.w[1] = CX.w[0];
        res.w[0] = 0;
        // NaN ?
        if ((x & 0x7c00000000000000u64) == 0x7c00000000000000u64) {
    #ifdef BID_SET_STATUS_FLAGS
            if ((x & SNAN_MASK64) == SNAN_MASK64)	// sNaN
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
    #endif
            res.w[0] = (CX.w[0] & 0x0003ffffffffffffu64);
            __mul_64x64_to_128 (res, res.w[0], BID_POWER10_TABLE_128[18].w[0]);
            res.w[1] |= ((CX.w[0]) & 0xfc00000000000000u64);
            return res;
        }
        // x is Infinity?
        if ((x & 0x7800000000000000u64) == 0x7800000000000000u64) {
            if (sign_x) {
                // -Inf, return NaN
                res.w[1] = 0x7c00000000000000u64;
    #ifdef BID_SET_STATUS_FLAGS
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
    #endif
            }
            return res;
        }
        // x is 0 otherwise

        exponent_x =
                exponent_x - DECIMAL_EXPONENT_BIAS + DECIMAL_EXPONENT_BIAS_128;
        res.w[1] =
                sign_x | ((((BID_UINT64) (exponent_x + DECIMAL_EXPONENT_BIAS_128)) >> 1)
                          << 49);
        res.w[0] = 0;
        return res;
    }
    if (sign_x) {
        res.w[1] = 0x7c00000000000000u64;
        res.w[0] = 0;
    #ifdef BID_SET_STATUS_FLAGS
        __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
    #endif
        return res;
    }
    #ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fegetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
    #endif
    exponent_x =
            exponent_x - DECIMAL_EXPONENT_BIAS + DECIMAL_EXPONENT_BIAS_128;

    // 2^64
    f64.i = 0x5f800000;

    // fx ~ CX
    fx.d = (float) CX.w[1] * f64.d + (float) CX.w[0];
    bin_expon_cx = ((fx.i >> 23) & 0xff) - 0x7f;
    digits = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon_cx];

    A10 = CX;
    if (exponent_x & 1) {
        A10.w[1] = (CX.w[1] << 3) | (CX.w[0] >> 61);
        A10.w[0] = CX.w[0] << 3;
        CX2.w[1] = (CX.w[1] << 1) | (CX.w[0] >> 63);
        CX2.w[0] = CX.w[0] << 1;
        __add_128_128 (A10, A10, CX2);
    }

    CS.w[0] = short_sqrt128 (A10);
    CS.w[1] = 0;
    // check for exact result
    if (CS.w[0] * CS.w[0] == A10.w[0]) {
        __mul_64x64_to_128_fast (S2, CS.w[0], CS.w[0]);
        if (S2.w[1] == A10.w[1]) {
            bid_get_BID128_very_fast (&res, 0,
                                      (exponent_x + DECIMAL_EXPONENT_BIAS_128) >> 1,
                                      CS);
    #ifdef UNCHANGED_BINARY_STATUS_FLAGS
            // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
    #endif
            return res;
        }
    }
    // get number of digits in CX
    D = CX.w[1] - BID_POWER10_INDEX_BINEXP_128[bin_expon_cx].w[1];
    if (D > 0
    || (!D && CX.w[0] >= BID_POWER10_INDEX_BINEXP_128[bin_expon_cx].w[0]))
    digits++;

    // if exponent is odd, scale coefficient by 10
    scale = 67 - digits;
    exponent_q = exponent_x - scale;
    scale += (exponent_q & 1);	// exp. bias is even

    if (scale > 38) {
        T128 = BID_POWER10_TABLE_128[scale - 37];
        __mul_128x128_low (CX1, CX, T128);

        TP128 = BID_POWER10_TABLE_128[37];
        __mul_128x128_to_256 (C256, CX1, TP128);
    } else {
    T128 = BID_POWER10_TABLE_128[scale];
    __mul_128x128_to_256 (C256, CX, T128);
    }


    // 4*C256
    C4.w[3] = (C256.w[3] << 2) | (C256.w[2] >> 62);
    C4.w[2] = (C256.w[2] << 2) | (C256.w[1] >> 62);
    C4.w[1] = (C256.w[1] << 2) | (C256.w[0] >> 62);
    C4.w[0] = C256.w[0] << 2;

    bid_long_sqrt128 (&CS, C256);

    #ifndef IEEE_ROUND_NEAREST
    #ifndef IEEE_ROUND_NEAREST_TIES_AWAY
    if (!((rnd_mode) & 3)) {
    #endif
    #endif
        // compare to midpoints
        CSM.w[1] = (CS.w[1] << 1) | (CS.w[0] >> 63);
        CSM.w[0] = (CS.w[0] + CS.w[0]) | 1;
        // CSM^2
        //__mul_128x128_to_256(M256, CSM, CSM);
        __sqr128_to_256 (M256, CSM);

        if (C4.w[3] > M256.w[3]
                || (C4.w[3] == M256.w[3]
                    && (C4.w[2] > M256.w[2]
                        || (C4.w[2] == M256.w[2]
                            && (C4.w[1] > M256.w[1]
                                || (C4.w[1] == M256.w[1]
                                    && C4.w[0] > M256.w[0])))))) {
            // round up
            CS.w[0]++;
            if (!CS.w[0])
                CS.w[1]++;
        } else {
            C8.w[1] = (CS.w[1] << 3) | (CS.w[0] >> 61);
            C8.w[0] = CS.w[0] << 3;
            // M256 - 8*CSM
            __sub_borrow_out (M256.w[0], Carry, M256.w[0], C8.w[0]);
            __sub_borrow_in_out (M256.w[1], Carry, M256.w[1], C8.w[1], Carry);
            __sub_borrow_in_out (M256.w[2], Carry, M256.w[2], 0, Carry);
            M256.w[3] = M256.w[3] - Carry;

            // if CSM' > C256, round up
            if (M256.w[3] > C4.w[3]
                    || (M256.w[3] == C4.w[3]
                        && (M256.w[2] > C4.w[2]
                            || (M256.w[2] == C4.w[2]
                                && (M256.w[1] > C4.w[1]
                                    || (M256.w[1] == C4.w[1]
                                        && M256.w[0] > C4.w[0])))))) {
                // round down
                if (!CS.w[0])
                    CS.w[1]--;
                CS.w[0]--;
            }
        }
    #ifndef IEEE_ROUND_NEAREST
    #ifndef IEEE_ROUND_NEAREST_TIES_AWAY
    } else {
    __sqr128_to_256 (M256, CS);
    C8.w[1] = (CS.w[1] << 1) | (CS.w[0] >> 63);
    C8.w[0] = CS.w[0] << 1;
    if (M256.w[3] > C256.w[3]
    || (M256.w[3] == C256.w[3]
    && (M256.w[2] > C256.w[2]
    || (M256.w[2] == C256.w[2]
    && (M256.w[1] > C256.w[1]
    || (M256.w[1] == C256.w[1]
    && M256.w[0] > C256.w[0])))))) {
        __sub_borrow_out (M256.w[0], Carry, M256.w[0], C8.w[0]);
        __sub_borrow_in_out (M256.w[1], Carry, M256.w[1], C8.w[1], Carry);
        __sub_borrow_in_out (M256.w[2], Carry, M256.w[2], 0, Carry);
        M256.w[3] = M256.w[3] - Carry;
        M256.w[0]++;
        if (!M256.w[0]) {
            M256.w[1]++;
            if (!M256.w[1]) {
                M256.w[2]++;
                if (!M256.w[2])
                    M256.w[3]++;
            }
        }

        if (!CS.w[0])
            CS.w[1]--;
        CS.w[0]--;

        if (M256.w[3] > C256.w[3]
                || (M256.w[3] == C256.w[3]
                    && (M256.w[2] > C256.w[2]
                        || (M256.w[2] == C256.w[2]
                            && (M256.w[1] > C256.w[1]
                                || (M256.w[1] == C256.w[1]
                                    && M256.w[0] > C256.w[0])))))) {

            if (!CS.w[0])
                CS.w[1]--;
            CS.w[0]--;
        }
    }

    else {
    __add_carry_out (M256.w[0], Carry, M256.w[0], C8.w[0]);
    __add_carry_in_out (M256.w[1], Carry, M256.w[1], C8.w[1], Carry);
    __add_carry_in_out (M256.w[2], Carry, M256.w[2], 0, Carry);
    M256.w[3] = M256.w[3] + Carry;
    M256.w[0]++;
    if (!M256.w[0]) {
        M256.w[1]++;
        if (!M256.w[1]) {
            M256.w[2]++;
            if (!M256.w[2])
                M256.w[3]++;
        }
    }
    if (M256.w[3] < C256.w[3]
    || (M256.w[3] == C256.w[3]
    && (M256.w[2] < C256.w[2]
    || (M256.w[2] == C256.w[2]
    && (M256.w[1] < C256.w[1]
    || (M256.w[1] == C256.w[1]
    && M256.w[0] <= C256.w[0])))))) {

        CS.w[0]++;
        if (!CS.w[0])
            CS.w[1]++;
    }
    }
    // RU?
    if ((rnd_mode) == BID_ROUNDING_UP) {
        CS.w[0]++;
        if (!CS.w[0])
            CS.w[1]++;
    }

    }
    #endif
    #endif

    #ifdef BID_SET_STATUS_FLAGS
    __set_status_flags (pfpsf, BID_INEXACT_EXCEPTION);
    #endif
    bid_get_BID128_fast (&res, 0, (exponent_q + DECIMAL_EXPONENT_BIAS_128) >> 1,
                         CS);
    #ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
    #endif
    return res;
}
*/
