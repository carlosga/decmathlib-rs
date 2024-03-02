/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid_convert_data::{BID_CONVERT_TABLE, BID_FACTORS, BID_PACKED_10000_ZEROS};
use crate::bid_decimal_data::*;
use crate::bid_div_macros::{bid___div_128_by_128, bid___div_256_by_128};
use crate::bid_internal::*;
use crate::d128::{_IDEC_flags, StatusFlags, RoundingMode};

/// Decimal floating-point division
pub (crate) fn bid128_div(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: RoundingMode, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut CA4: BID_UINT256;
    let mut CA4r: BID_UINT256 = Default::default();
    let P256: BID_UINT256;
    let mut CX: BID_UINT128 = Default::default();
    let mut CY: BID_UINT128 = Default::default();
    let mut T128: BID_UINT128 = Default::default();
    let mut CQ: BID_UINT128 = Default::default();
    let CR: BID_UINT128;
    let CA: BID_UINT128;
    let mut TP128: BID_UINT128 = Default::default();
    let Qh: BID_UINT128;
    let mut _Ql: BID_UINT128;
    let mut res: BID_UINT128 = Default::default();
    let mut sign_x: BID_UINT64 = 0;
    let mut sign_y: BID_UINT64 = 0;
    let T: BID_UINT64;
    let mut carry64: BID_UINT64;
    let D: BID_UINT64;
    let Q_high: BID_UINT64;
    let Q_low: BID_UINT64;
    let QX: BID_UINT64;
    let PD: BID_UINT64;
    let mut fx: BID_UI32FLOAT = Default::default();
    let mut fy: BID_UI32FLOAT = Default::default();
    let mut f64: BID_UI32FLOAT = Default::default();
    let mut QX32: BID_UINT32;
    let mut tdigit: [BID_UINT32; 3] = [0u32; 3];
    let mut digit: BID_UINT32;
    let mut digit_h: BID_UINT32;
    let digit_low: BID_UINT32;
    let mut exponent_x: i32 = 0;
    let mut exponent_y: i32 = 0;
    let bin_index: i32;
    let bin_expon: i32;
    let mut diff_expon: i32;
    let mut ed2: i32;
    let mut digits_q: i32;
    let amount: i32;
    let mut nzeros: i32;
    let i: i32;
    let mut j: i32;
    let mut k: i32;
    let d5: i32;
    let mut rmode: RoundingMode;
    let valid_y: BID_UINT64 = unpack_BID128_value(&mut sign_y, &mut exponent_y, &mut CY, y);

    // unpack arguments, check for NaN or Infinity
    if unpack_BID128_value(&mut sign_x, &mut exponent_x, &mut CX, x) == 0 {
        // test if x is NaN
        if (x.w[1] & 0x7c00000000000000u64) == 0x7c00000000000000u64 {
            if (x.w[1] & 0x7e00000000000000u64) == 0x7e00000000000000u64 || // sNaN
                (y.w[1] & 0x7e00000000000000u64) == 0x7e00000000000000u64 {
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            res.w[1] = (CX.w[1]) & QUIET_MASK64;
            res.w[0] = CX.w[0];
            return res;
        }
        // x is Infinity?
        if (x.w[1] & 0x7800000000000000u64) == 0x7800000000000000u64 {
            // check if y is Inf.
            if (y.w[1] & 0x7c00000000000000u64) == 0x7800000000000000u64 { // return NaN
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
                res.w[1] = 0x7c00000000000000u64;
                res.w[0] = 0;
                return res;
            }
            // y is NaN?
            if (y.w[1] & 0x7c00000000000000u64) != 0x7c00000000000000u64 { // return NaN
                // return +/-Inf
                res.w[1] = ((x.w[1] ^ y.w[1]) & 0x8000000000000000u64) | 0x7800000000000000u64;
                res.w[0] = 0;
                return res;
            }
        }
        // x is 0
        if (y.w[1] & 0x7800000000000000u64) < 0x7800000000000000u64 {
            if (CY.w[0] == 0) && (CY.w[1] & 0x0001ffffffffffffu64) != 0x0001ffffffffffffu64 {
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
                // x=y=0, return NaN
                res.w[1] = 0x7c00000000000000u64;
                res.w[0] = 0;
                return res;
            }
            // return 0
            res.w[1] = (x.w[1] ^ y.w[1]) & 0x8000000000000000u64;
            exponent_x = exponent_x - exponent_y + DECIMAL_EXPONENT_BIAS_128;
            if exponent_x > DECIMAL_MAX_EXPON_128 {
                exponent_x = DECIMAL_MAX_EXPON_128;
            } else if exponent_x < 0 {
                exponent_x = 0;
            }
            res.w[1] |= (exponent_x as BID_UINT64) << 49;
            res.w[0]  = 0;
            return res;
        }
    }
    if valid_y == 0 {
        // y is Inf. or NaN

        // test if y is NaN
        if (y.w[1] & 0x7c00000000000000u64) == 0x7c00000000000000u64 {
            if (y.w[1] & 0x7e00000000000000u64) == 0x7e00000000000000u64  { // sNaN
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            res.w[1] = CY.w[1] & QUIET_MASK64;
            res.w[0] = CY.w[0];
            return res;
        }
        // y is Infinity?
        if (y.w[1] & 0x7800000000000000u64) == 0x7800000000000000u64 {
            // return +/-0
            res.w[1] = sign_x ^ sign_y;
            res.w[0] = 0;
            return res;
        }
        // y is 0, return +/-Inf
        __set_status_flags(pfpsf, StatusFlags::BID_ZERO_DIVIDE_EXCEPTION);
        res.w[1] = ((x.w[1] ^ y.w[1]) & 0x8000000000000000u64) | 0x7800000000000000u64;
        res.w[0] = 0;
        return res;
    }

    diff_expon = exponent_x - exponent_y + DECIMAL_EXPONENT_BIAS_128;

    if __unsigned_compare_gt_128(&CY, &CX) {
        unsafe  {
            // CX < CY

            // 2^64
            f64.ui32 = 0x5f800000;

            // fx ~ CX,   fy ~ CY
            fx.d      = (CX.w[1] as f32) * f64.d + (CX.w[0] as f32);
            fy.d      = (CY.w[1] as f32) * f64.d + (CY.w[0] as f32);
            // expon_cy - expon_cx
            bin_index = ((fy.ui32 - fx.ui32) >> 23) as i32;

            CA = if CX.w[1] != 0 {
                T = BID_POWER10_INDEX_BINEXP_128[bin_index as usize].w[0];
                __mul_64x128_short(T, &CX)
            } else {
                T128 = BID_POWER10_INDEX_BINEXP_128[bin_index as usize];
                __mul_64x128_short(CX.w[0], &T128)
            };

            ed2 = 33;
            if __unsigned_compare_gt_128(&CY, &CA) {
                ed2 += 1;
            }

            T128 = BID_POWER10_TABLE_128[ed2 as usize];
            CA4  = __mul_128x128_to_256(&CA, &T128);

            ed2        += BID_ESTIMATE_DECIMAL_DIGITS[bin_index as usize];
            CQ.w[0]     = 0;
            CQ.w[1]     = 0;
            diff_expon -= ed2;
        }
    } else {
        // get CQ = CX/CY
        (CQ, CR) = bid___div_128_by_128(&CX, &CY);

        if CR.w[1] == 0 && CR.w[0] == 0 {
            res = bid_get_BID128(sign_x ^ sign_y, diff_expon, &CQ, rnd_mode, pfpsf);
            return res;
        }

        unsafe  {
            // get number of decimal digits in CQ
            // 2^64
            f64.ui32     = 0x5f800000;
            fx.d      = (CQ.w[1] as f32) * f64.d + (CQ.w[0] as f32);
            // binary expon. of CQ
            bin_expon = ((fx.ui32 - 0x3f800000) >> 23) as i32;

            digits_q   = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon as usize];
            TP128.w[0] = BID_POWER10_INDEX_BINEXP_128[bin_expon as usize].w[0];
            TP128.w[1] = BID_POWER10_INDEX_BINEXP_128[bin_expon as usize].w[1];
            if __unsigned_compare_ge_128(&CQ, &TP128) {
                digits_q += 1;
            }

            ed2         = 34 - digits_q;
            T128.w[0]   = BID_POWER10_TABLE_128[ed2 as usize].w[0];
            T128.w[1]   = BID_POWER10_TABLE_128[ed2 as usize].w[1];
            CA4         = __mul_128x128_to_256(&CR, &T128);
            diff_expon -= ed2;
            CQ          = __mul_128x128_low(&CQ, &T128);
        }
    }

    bid___div_256_by_128(&mut CQ, &mut CA4, &CY);

    if CA4.w[0] != 0 || CA4.w[1] != 0 {
        // set status flags
        __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
    } else {   // check whether result is exact
        // check whether CX, CY are short
        if CX.w[1] == 0 && CY.w[1] == 0 && (CX.w[0] <= 1024) && (CY.w[0] <= 1024) {
            i = (CY.w[0] as i32) - 1;
            j = (CX.w[0] as i32) - 1;
            // difference in powers of 2 BID_FACTORS for Y and X
            nzeros = ed2 - BID_FACTORS[i as usize][0] + BID_FACTORS[j as usize][0];
            // difference in powers of 5 BID_FACTORS
            d5     = ed2 - BID_FACTORS[i as usize][1] + BID_FACTORS[j as usize][1];
            if d5 < nzeros {
                nzeros = d5;
            }
            // get P*(2^M[extra_digits])/10^extra_digits
            (Qh, _Ql) = __mul_128x128_full(&CQ, &BID_RECIPROCALS10_128[nzeros as usize]);

            // now get P/10^extra_digits: shift Q_high right by M[extra_digits]-128
            amount = BID_RECIP_SCALE[nzeros as usize];
            CQ     = __shr_128_long(&Qh, amount);

            diff_expon += nzeros;
        } else {
            // decompose Q as Qh*10^17 + Ql
            // T128 = BID_RECIPROCALS10_128[17];
            T128.w[0] = 0x44909befeb9fad49u64;
            T128.w[1] = 0x000b877aa3236a4bu64;
            P256      = __mul_128x128_to_256(&CQ, &T128);
            // amount = BID_RECIP_SCALE[17];
            Q_high    = (P256.w[2] >> 44) | (P256.w[3] << (64 - 44));
            Q_low     = CQ.w[0] - Q_high * 100000000000000000u64;

            if Q_low == 0 {
                diff_expon += 17;

                tdigit[0] = (Q_high & 0x3ffffff) as BID_UINT32;
                tdigit[1] = 0;
                QX        = Q_high >> 26;
                QX32      = QX as BID_UINT32;
                nzeros    = 0;
                j         = 0;

                // for (j = 0; QX32; j++, QX32 >>= 7) {
                while QX32 != 0 {
                    k          = (QX32 & 127) as i32;
                    tdigit[0] += BID_CONVERT_TABLE[j as usize][k as usize][0];
                    tdigit[1] += BID_CONVERT_TABLE[j as usize][k as usize][1];
                    if tdigit[0] >= 100000000 {
                        tdigit[0] -= 100000000;
                        tdigit[1] += 1;
                    }
                    QX32 >>= 7;
                    j     += 1;
                }

                if tdigit[1] >= 100000000 {
                    tdigit[1] -= 100000000;
                    if tdigit[1] >= 100000000 {
                        tdigit[1] -= 100000000;
                    }
                }

                digit = tdigit[0];
                if digit == 0 && tdigit[1] == 0 {
                    nzeros += 16;
                } else {
                    if digit == 0 {
                        nzeros += 8;
                        digit = tdigit[1];
                    }
                    // decompose digit
                    PD        = (digit as BID_UINT64) * 0x068DB8BBu64;
                    digit_h   = (PD >> 40) as BID_UINT32;
                    digit_low = digit - digit_h * 10000;

                    if digit_low == 0 {
                        nzeros += 4;
                    } else {
                        digit_h = digit_low;
                    }

                    if (digit_h & 1) != 1 {
                        nzeros += (3 & ((BID_PACKED_10000_ZEROS[(digit_h >> 3) as usize] >> (digit_h & 7)) as BID_UINT32)) as i32;
                    }
                }

                if nzeros != 0 {
                    CQ = __mul_64x64_to_128(Q_high, BID_RECIPROCALS10_64[nzeros as usize]);

                    // now get P/10^extra_digits: shift C64 right by M[extra_digits]-64
                    amount = BID_SHORT_RECIP_SCALE[nzeros as usize];
                    CQ.w[0] = CQ.w[1] >> amount;
                } else {
                    CQ.w[0] = Q_high;
                }
                CQ.w[1] = 0;

                diff_expon += nzeros;
            } else {
                tdigit[0] = (Q_low & 0x3ffffff) as BID_UINT32;
                tdigit[1] = 0;
                QX        = Q_low >> 26;
                QX32      = QX as BID_UINT32;
                nzeros    = 0;
                j         = 0;

                // for (j = 0; QX32; j++, QX32 >>= 7) {
                while QX32 != 0 {
                    k          = (QX32 & 127) as i32;
                    tdigit[0] += BID_CONVERT_TABLE[j as usize][k as usize][0];
                    tdigit[1] += BID_CONVERT_TABLE[j as usize][k as usize][1];
                    if tdigit[0] >= 100000000 {
                        tdigit[0] -= 100000000;
                        tdigit[1] += 1;
                    }
                    QX32 >>= 7;
                    j     += 1;
                }

                if tdigit[1] >= 100000000 {
                    tdigit[1] -= 100000000;
                    if tdigit[1] >= 100000000 {
                        tdigit[1] -= 100000000;
                    }
                }

                digit = tdigit[0];
                if digit == 0 && tdigit[1] == 0 {
                    nzeros += 16;
                } else {
                    if digit == 0 {
                        nzeros += 8;
                        digit = tdigit[1];
                    }
                    // decompose digit
                    PD        = (digit as BID_UINT64) * 0x068DB8BBu64;
                    digit_h   = (PD >> 40) as BID_UINT32;
                    digit_low = digit - digit_h * 10000;

                    if digit_low == 0 {
                        nzeros += 4;
                    } else {
                        digit_h = digit_low;
                    }

                    if (digit_h & 1) != 1 {
                        nzeros += (3 & ((BID_PACKED_10000_ZEROS[(digit_h >> 3) as usize] >> (digit_h & 7)) as BID_UINT32)) as i32;
                    }
                }

                if nzeros != 0 {
                    // get P*(2^M[extra_digits])/10^extra_digits
                    (Qh, _Ql) = __mul_128x128_full(&CQ, &BID_RECIPROCALS10_128[nzeros as usize]);

                    // now get P/10^extra_digits: shift Q_high right by M[extra_digits]-128
                    amount = BID_RECIP_SCALE[nzeros as usize];
                    CQ     = __shr_128(&Qh, amount);
                }
                diff_expon += nzeros;
            }
        }
        res = bid_get_BID128(sign_x ^ sign_y, diff_expon, &CQ, rnd_mode, pfpsf);
        return res;
    }

    if diff_expon >= 0 {
        rmode = rnd_mode;
        if (sign_x ^ sign_y) != 0 && ((rmode as u32 - 1u32) < 2) {
            rmode = RoundingMode::from(3 - (rmode as u32));
        }
        match rmode {
            RoundingMode::BID_ROUNDING_TO_NEAREST =>  { // round to nearest code
                // rounding
                // 2*CA4 - CY
                CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
                CA4r.w[0] = CA4.w[0] + CA4.w[0];

                (CA4r.w[0], carry64) = __sub_borrow_out(CA4r.w[0], CY.w[0]);

                CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;
                D         = if (CA4r.w[1] | CA4r.w[0]) != 0 { 1 } else { 0 };
                carry64   = ((1 + ((CA4r.w[1] as BID_SINT64) >> 63)) & (((CQ.w[0]) | D) as BID_SINT64)) as BID_UINT64;
                CQ.w[0]  += carry64;
                if CQ.w[0] < carry64 {
                    CQ.w[1] += 1;
                }
            },
            RoundingMode::BID_ROUNDING_TIES_AWAY => {
                // rounding
                // 2*CA4 - CY
                CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
                CA4r.w[0] = CA4.w[0] + CA4.w[0];

                (CA4r.w[0], carry64) = __sub_borrow_out(CA4r.w[0], CY.w[0]);

                CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;
                D         = if (CA4r.w[1] | CA4r.w[0]) != 0 { 0 } else { 1 };
                carry64   = ((1 + ((CA4r.w[1] as BID_SINT64) >> 63)) | (D as BID_SINT64)) as BID_UINT64;
                CQ.w[0]  += carry64;
                if CQ.w[0] < carry64 {
                    CQ.w[1] += 1;
                }
            },
            RoundingMode::BID_ROUNDING_DOWN | RoundingMode::BID_ROUNDING_TO_ZERO => { },
            _ =>  {
                CQ.w[0] += 1;
                if CQ.w[0] == 0 {
                    CQ.w[1] += 1;
                }
            }
        }
    } else {
        if CA4.w[0] != 0 || CA4.w[1] != 0{
            // set status flags
            __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
        }

        res = bid_handle_UF_128_rem(sign_x ^ sign_y, diff_expon, &CQ, CA4.w[1] | CA4.w[0], rnd_mode, pfpsf);
        return res;
    }

    res = bid_get_BID128(sign_x ^ sign_y, diff_expon, &CQ, rnd_mode, pfpsf);

    res
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/*
//#define LEAVE_TRAILING_ZEROS

BID_TYPE0_FUNCTION_ARGTYPE1_ARGTYPE2(BID_UINT128, bid128dd_div, BID_UINT64, x, BID_UINT64, y)

BID_UINT256 CA4, CA4r, P256;
BID_UINT128 CX, CY, T128, CQ, CR, CA, TP128, Qh, Ql, res;
BID_UINT64 sign_x, sign_y, carry64, D, Q_high, Q_low, QX, PD, valid_y;
int_float fx, fy, f64;
BID_UINT32 QX32, tdigit[3], digit, digit_h, digit_low;
int exponent_x, exponent_y, bin_index, bin_expon, diff_expon, ed2, digits_q, amount;
int nzeros, i, j, k, d5;
unsigned rmode;

BID_OPT_SAVE_BINARY_FLAGS()

valid_y = unpack_BID64(&sign_y, &exponent_y, &CY.w[0], y);

// unpack arguments, check for NaN or Infinity
CX.w[1] = 0;
if (!unpack_BID64(&sign_x, &exponent_x, &CX.w[0], (x))) {
#ifdef BID_SET_STATUS_FLAGS
    if ((y & SNAN_MASK64) == SNAN_MASK64) // y is sNaN
        __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif

    // test if x is NaN
    if ((x & NAN_MASK64) == NAN_MASK64) {
#ifdef BID_SET_STATUS_FLAGS
        if ((x & SNAN_MASK64) == SNAN_MASK64) // sNaN
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
        res.w[0] = (CX.w[0] & 0x0003ffffffffffffu64);
        __mul_64x64_to_128(res, res.w[0], BID_POWER10_TABLE_128[18].w[0]);
        res.w[1] |= ((CX.w[0]) & 0xfc00000000000000u64);
        return res;
    }
    // x is Infinity?
    if (((x)&0x7800000000000000u64) == 0x7800000000000000u64) {
        // check if y is Inf.
        if ((((y)&0x7c00000000000000u64) == 0x7800000000000000u64))
        // return NaN
        {
#ifdef BID_SET_STATUS_FLAGS
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0;
            return res;
        }
        if ((((y)&0x7c00000000000000u64) != 0x7c00000000000000u64)) {
            // otherwise return +/-Inf
            res.w[1] = (((x) ^ (y)) & 0x8000000000000000u64) | 0x7800000000000000u64;
            res.w[0] = 0;
            return res;
        }
    }
    // x is 0
    if ((((y)&0x7800000000000000u64) != 0x7800000000000000u64)) {
        if (!CY.w[0]) {
#ifdef BID_SET_STATUS_FLAGS
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
            // x=y=0, return NaN
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0;
            return res;
        }
        // return 0
        res.w[1] = ((x) ^ (y)) & 0x8000000000000000u64;
        if (((y)&0x6000000000000000u64) == 0x6000000000000000u64)
            exponent_y = ((BID_UINT32)((y) >> 51)) & 0x3ff;
        else
            exponent_y = ((BID_UINT32)((y) >> 53)) & 0x3ff;
        exponent_x = exponent_x - exponent_y + DECIMAL_EXPONENT_BIAS_128;
        res.w[1] |= (((BID_UINT64)exponent_x) << 49);
        res.w[0] = 0;
        return res;
    }
}

CY.w[1] = 0;
if (!valid_y) {
    // y is Inf. or NaN

    // test if y is NaN
    if ((y & NAN_MASK64) == NAN_MASK64) {
#ifdef BID_SET_STATUS_FLAGS
        if ((y & SNAN_MASK64) == SNAN_MASK64) // sNaN
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
        res.w[0] = (CY.w[0] & 0x0003ffffffffffffu64);
        __mul_64x64_to_128(res, res.w[0], BID_POWER10_TABLE_128[18].w[0]);
        res.w[1] |= ((CY.w[0]) & 0xfc00000000000000u64);
        return res;
    }
    // y is Infinity?
    if (((y)&0x7800000000000000u64) == 0x7800000000000000u64) {
        // return +/-0
        res.w[1] = sign_x ^ sign_y;
        res.w[0] = 0;
        return res;
    }
    // y is 0, return +/-Inf
    res.w[1] = (((x) ^ (y)) & 0x8000000000000000u64) | 0x7800000000000000u64;
    res.w[0] = 0;
#ifdef BID_SET_STATUS_FLAGS
    __set_status_flags(pfpsf, StatusFlags::BID_ZERO_DIVIDE_EXCEPTION);
#endif
    return res;
}
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
// (void) fegetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
diff_expon = exponent_x - exponent_y + DECIMAL_EXPONENT_BIAS_128;

if (__unsigned_compare_gt_128(CY, CX)) {
    // CX < CY

    // 2^64
    f64.i = 0x5f800000;

    // fx ~ CX,   fy ~ CY
    fx.d = (float)CX.w[1] * f64.d + (float)CX.w[0];
    fy.d = (float)CY.w[1] * f64.d + (float)CY.w[0];
    // expon_cy - expon_cx
    bin_index = (fy.i - fx.i) >> 23;

    T128 = BID_POWER10_INDEX_BINEXP_128[bin_index as usize];
    __mul_64x128_short(CA, CX.w[0], T128);

    ed2 = 33;
    if (__unsigned_compare_gt_128(CY, CA))
        ed2++;

    T128 = BID_POWER10_TABLE_128[ed2 as usize];
    __mul_128x128_to_256(CA4, CA, T128);

    ed2 += BID_ESTIMATE_DECIMAL_DIGITS[bin_index as usize];
    CQ.w[0] = CQ.w[1] = 0;
    diff_expon = diff_expon - ed2;

} else {
    // get CQ = CX/CY
    bid___div_128_by_128(&CQ, &CR, CX, CY);

    if (!CR.w[1] && !CR.w[0]) {
        bid_get_BID128(&res, sign_x ^ sign_y, diff_expon, CQ, &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
        // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
        return res;
    }
    // get number of decimal digits in CQ
    // 2^64
    f64.i = 0x5f800000;
    fx.d = (float)CQ.w[1] * f64.d + (float)CQ.w[0];
    // binary expon. of CQ
    bin_expon = (fx.i - 0x3f800000) >> 23;

    digits_q = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon as usize];
    TP128.w[0] = BID_POWER10_INDEX_BINEXP_128[bin_expon as usize].w[0];
    TP128.w[1] = BID_POWER10_INDEX_BINEXP_128[bin_expon as usize].w[1];
    if (__unsigned_compare_ge_128(CQ, TP128))
        digits_q++;

    ed2 = 34 - digits_q;
    T128.w[0] = BID_POWER10_TABLE_128[ed2 as usize].w[0];
    T128.w[1] = BID_POWER10_TABLE_128[ed2 as usize].w[1];
    __mul_128x128_to_256(CA4, CR, T128);
    diff_expon = diff_expon - ed2;
    __mul_128x128_low(CQ, CQ, T128);
}

bid___div_256_by_128(&CQ, &CA4, CY);

#ifdef BID_SET_STATUS_FLAGS
if (CA4.w[0] || CA4.w[1]) {
    // set status flags
    __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
}
#ifndef LEAVE_TRAILING_ZEROS
else
#endif
#else
#ifndef LEAVE_TRAILING_ZEROS
if (!CA4.w[0] && !CA4.w[1])
#endif
#endif
#ifndef LEAVE_TRAILING_ZEROS
// check whether result is exact
{
    // check whether CX, CY are short
    if (!CX.w[1] && !CY.w[1] && (CX.w[0] <= 1024) && (CY.w[0] <= 1024)) {
        i = (int)CY.w[0] - 1;
        j = (int)CX.w[0] - 1;
        // difference in powers of 2 BID_FACTORS for Y and X
        nzeros = ed2 - BID_FACTORS[i][0] + BID_FACTORS[j][0];
        // difference in powers of 5 BID_FACTORS
        d5 = ed2 - BID_FACTORS[i][1] + BID_FACTORS[j][1];
        if (d5 < nzeros)
            nzeros = d5;
        // get P*(2^M[extra_digits])/10^extra_digits
        __mul_128x128_full(Qh, Ql, CQ, BID_RECIPROCALS10_128[nzeros as usize]);
        //__mul_128x128_to_256(P256, CQ, BID_RECIPROCALS10_128[nzeros as usize]);Qh.w[1]=P256.w[3];Qh.w[0]=P256.w[2];

        // now get P/10^extra_digits: shift Q_high right by M[extra_digits]-128
        amount = BID_RECIP_SCALE[nzeros as usize];
        __shr_128_long(CQ, Qh, amount);

        diff_expon += nzeros;
    } else {
        // decompose Q as Qh*10^17 + Ql
        // T128 = BID_RECIPROCALS10_128[17];
        T128.w[0] = 0x44909befeb9fad49u64;
        T128.w[1] = 0x000b877aa3236a4bu64;
        __mul_128x128_to_256(P256, CQ, T128);
        // amount = BID_RECIP_SCALE[17];
        Q_high = (P256.w[2] >> 44) | (P256.w[3] << (64 - 44));
        Q_low = CQ.w[0] - Q_high * 100000000000000000u64;

        if (!Q_low) {
            diff_expon += 17;

            tdigit[0] = Q_high & 0x3ffffff;
            tdigit[1] = 0;
            QX = Q_high >> 26;
            QX32 = QX;
            nzeros = 0;

            for (j = 0; QX32; j++, QX32 >>= 7) {
                k = (QX32 & 127);
                tdigit[0] += BID_CONVERT_TABLE[j as usize][k as usize][0];
                tdigit[1] += BID_CONVERT_TABLE[j as usize][k as usize][1];
                if (tdigit[0] >= 100000000) {
                    tdigit[0] -= 100000000;
                    tdigit[1]++;
                }
            }

            if (tdigit[1] >= 100000000) {
                tdigit[1] -= 100000000;
                if (tdigit[1] >= 100000000)
                    tdigit[1] -= 100000000;
            }

            digit = tdigit[0];
            if (!digit && !tdigit[1])
                nzeros += 16;
            else {
                if (!digit) {
                    nzeros += 8;
                    digit = tdigit[1];
                }
                // decompose digit
                PD = (BID_UINT64)digit * 0x068DB8BBu64;
                digit_h = (BID_UINT32)(PD >> 40);
                digit_low = digit - digit_h * 10000;

                if (!digit_low)
                    nzeros += 4;
                else
                    digit_h = digit_low;

                if (!(digit_h & 1))
                    nzeros += 3 & (BID_UINT32)(BID_PACKED_10000_ZEROS[digit_h >> 3] >> (digit_h & 7));
            }

            if (nzeros) {
                __mul_64x64_to_128(CQ, Q_high, BID_RECIPROCALS10_64[nzeros as usize]);

                // now get P/10^extra_digits: shift C64 right by M[extra_digits]-64
                amount = BID_SHORT_RECIP_SCALE[nzeros as usize];
                CQ.w[0] = CQ.w[1] >> amount;
            } else
                CQ.w[0] = Q_high;
            CQ.w[1] = 0;

            diff_expon += nzeros;
        } else {
            tdigit[0] = Q_low & 0x3ffffff;
            tdigit[1] = 0;
            QX = Q_low >> 26;
            QX32 = QX;
            nzeros = 0;

            for (j = 0; QX32; j++, QX32 >>= 7) {
                k = (QX32 & 127);
                tdigit[0] += BID_CONVERT_TABLE[j as usize][k as usize][0];
                tdigit[1] += BID_CONVERT_TABLE[j as usize][k as usize][1];
                if (tdigit[0] >= 100000000) {
                    tdigit[0] -= 100000000;
                    tdigit[1]++;
                }
            }

            if (tdigit[1] >= 100000000) {
                tdigit[1] -= 100000000;
                if (tdigit[1] >= 100000000)
                    tdigit[1] -= 100000000;
            }

            digit = tdigit[0];
            if (!digit && !tdigit[1])
                nzeros += 16;
            else {
                if (!digit) {
                    nzeros += 8;
                    digit = tdigit[1];
                }
                // decompose digit
                PD = (BID_UINT64)digit * 0x068DB8BBu64;
                digit_h = (BID_UINT32)(PD >> 40);
                digit_low = digit - digit_h * 10000;

                if (!digit_low)
                    nzeros += 4;
                else
                    digit_h = digit_low;

                if (!(digit_h & 1))
                    nzeros += 3 & (BID_UINT32)(BID_PACKED_10000_ZEROS[digit_h >> 3] >> (digit_h & 7));
            }

            if (nzeros) {
                // get P*(2^M[extra_digits])/10^extra_digits
                __mul_128x128_full(Qh, Ql, CQ, BID_RECIPROCALS10_128[nzeros as usize]);

                // now get P/10^extra_digits: shift Q_high right by M[extra_digits]-128
                amount = BID_RECIP_SCALE[nzeros as usize];
                __shr_128(CQ, Qh, amount);
            }
            diff_expon += nzeros;
        }
    }
    bid_get_BID128(&res, sign_x ^ sign_y, diff_expon, CQ, &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
    return res;
}
#endif

if (diff_expon >= 0) {
#ifdef IEEE_ROUND_NEAREST
    // rounding
    // 2*CA4 - CY
    CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
    CA4r.w[0] = CA4.w[0] + CA4.w[0];
    __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
    CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;

    D = (CA4r.w[1] | CA4r.w[0]) ? 1 : 0;
    carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) & ((CQ.w[0]) | D);

    CQ.w[0] += carry64;
    if (CQ.w[0] < carry64)
        CQ.w[1]++;
#else
#ifdef IEEE_ROUND_NEAREST_TIES_AWAY
        // rounding
        // 2*CA4 - CY
        CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
        CA4r.w[0] = CA4.w[0] + CA4.w[0];
        __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
        CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;

        D = (CA4r.w[1] | CA4r.w[0]) ? 0 : 1;
        carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) | D;

        CQ.w[0] += carry64;
        if (CQ.w[0] < carry64)
            CQ.w[1]++;
#else
    rmode = rnd_mode;
    if (sign_x ^ sign_y && (unsigned)(rmode - 1) < 2)
        rmode = 3 - rmode;
    switch (rmode) {
        case BID_ROUNDING_TO_NEAREST: // round to nearest code
            // rounding
            // 2*CA4 - CY
            CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
            CA4r.w[0] = CA4.w[0] + CA4.w[0];
            __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
            CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;
            D = (CA4r.w[1] | CA4r.w[0]) ? 1 : 0;
            carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) & ((CQ.w[0]) | D);
            CQ.w[0] += carry64;
            if (CQ.w[0] < carry64)
                CQ.w[1]++;
            break;
        case BID_ROUNDING_TIES_AWAY:
            // rounding
            // 2*CA4 - CY
            CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
            CA4r.w[0] = CA4.w[0] + CA4.w[0];
            __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
            CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;
            D = (CA4r.w[1] | CA4r.w[0]) ? 0 : 1;
            carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) | D;
            CQ.w[0] += carry64;
            if (CQ.w[0] < carry64)
                CQ.w[1]++;
            break;
        case BID_ROUNDING_DOWN:
        case BID_ROUNDING_TO_ZERO:
            break;
        default: // rounding up
            CQ.w[0]++;
            if (!CQ.w[0])
                CQ.w[1]++;
            break;
    }
#endif
#endif

} else {
#ifdef BID_SET_STATUS_FLAGS
    if (CA4.w[0] || CA4.w[1]) {
        // set status flags
        __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
    }
#endif
    bid_handle_UF_128_rem(&res, sign_x ^ sign_y, diff_expon, CQ, CA4.w[1] | CA4.w[0], &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
    return res;
}

bid_get_BID128(&res, sign_x ^ sign_y, diff_expon, CQ, &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
// (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
return res;
}

BID128_FUNCTION_ARGTYPE1_ARG128(bid128dq_div, BID_UINT64, x, y)
BID_UINT256 CA4, CA4r, P256;
BID_UINT128 CX, CY, T128, CQ, CR, CA, TP128, Qh, Ql, res;
BID_UINT64 sign_x, sign_y, carry64, D, Q_high, Q_low, QX, valid_y, PD;
int_float fx, fy, f64;
BID_UINT32 QX32, tdigit[3], digit, digit_h, digit_low;
int exponent_x, exponent_y, bin_index, bin_expon, diff_expon, ed2, digits_q, amount;
int nzeros, i, j, k, d5;
unsigned rmode;

BID_OPT_SAVE_BINARY_FLAGS()

valid_y = unpack_BID128_value(&sign_y, &exponent_y, &CY, y);

// unpack arguments, check for NaN or Infinity
CX.w[1] = 0;
if (!unpack_BID64(&sign_x, &exponent_x, &CX.w[0], x)) {
#ifdef BID_SET_STATUS_FLAGS
    if ((y.w[1] & SNAN_MASK64) == SNAN_MASK64) // y is sNaN
        __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif

    // test if x is NaN
    if ((x & NAN_MASK64) == NAN_MASK64) {
#ifdef BID_SET_STATUS_FLAGS
        if ((x & SNAN_MASK64) == SNAN_MASK64) // sNaN
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
        res.w[0] = (CX.w[0] & 0x0003ffffffffffffu64);
        __mul_64x64_to_128(res, res.w[0], BID_POWER10_TABLE_128[18].w[0]);
        res.w[1] |= ((CX.w[0]) & 0xfc00000000000000u64);
        return res;
    }
    // x is Infinity?
    if ((x & 0x7800000000000000u64) == 0x7800000000000000u64) {
        // check if y is Inf.
        if (((y.w[1] & 0x7c00000000000000u64) == 0x7800000000000000u64))
        // return NaN
        {
#ifdef BID_SET_STATUS_FLAGS
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0;
            return res;
        }
        if (((y.w[1] & 0x7c00000000000000u64) != 0x7c00000000000000u64)) {
            // otherwise return +/-Inf
            res.w[1] = ((x ^ y.w[1]) & 0x8000000000000000u64) | 0x7800000000000000u64;
            res.w[0] = 0;
            return res;
        }
    }
    // x is 0
    if ((y.w[1] & INFINITY_MASK64) != INFINITY_MASK64) {
        if ((!CY.w[0]) && !(CY.w[1] & 0x0001ffffffffffffu64)) {
#ifdef BID_SET_STATUS_FLAGS
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
            // x=y=0, return NaN
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0;
            return res;
        }
        // return 0
        res.w[1] = (x ^ y.w[1]) & 0x8000000000000000u64;
        exponent_x = exponent_x - exponent_y + (DECIMAL_EXPONENT_BIAS_128 << 1) - DECIMAL_EXPONENT_BIAS;
        if (exponent_x > DECIMAL_MAX_EXPON_128)
            exponent_x = DECIMAL_MAX_EXPON_128;
        else if (exponent_x < 0)
            exponent_x = 0;
        res.w[1] |= (((BID_UINT64)exponent_x) << 49);
        res.w[0] = 0;
        return res;
    }
}
exponent_x += (DECIMAL_EXPONENT_BIAS_128 - DECIMAL_EXPONENT_BIAS);

if (!valid_y) {
    // y is Inf. or NaN

    // test if y is NaN
    if ((y.w[1] & 0x7c00000000000000u64) == 0x7c00000000000000u64) {
#ifdef BID_SET_STATUS_FLAGS
        if ((y.w[1] & 0x7e00000000000000u64) == 0x7e00000000000000u64) // sNaN
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
        res.w[1] = CY.w[1] & QUIET_MASK64;
        res.w[0] = CY.w[0];
        return res;
    }
    // y is Infinity?
    if ((y.w[1] & 0x7800000000000000u64) == 0x7800000000000000u64) {
        // return +/-0
        res.w[1] = sign_x ^ sign_y;
        res.w[0] = 0;
        return res;
    }
    // y is 0, return +/-Inf
    res.w[1] = ((x ^ y.w[1]) & 0x8000000000000000u64) | 0x7800000000000000u64;
    res.w[0] = 0;
#ifdef BID_SET_STATUS_FLAGS
    __set_status_flags(pfpsf, StatusFlags::BID_ZERO_DIVIDE_EXCEPTION);
#endif
    return res;
}
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
// (void) fegetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
diff_expon = exponent_x - exponent_y + DECIMAL_EXPONENT_BIAS_128;

if (__unsigned_compare_gt_128(CY, CX)) {
    // CX < CY

    // 2^64
    f64.i = 0x5f800000;

    // fx ~ CX,   fy ~ CY
    fx.d = (float)CX.w[1] * f64.d + (float)CX.w[0];
    fy.d = (float)CY.w[1] * f64.d + (float)CY.w[0];
    // expon_cy - expon_cx
    bin_index = (fy.i - fx.i) >> 23;

    T128 = BID_POWER10_INDEX_BINEXP_128[bin_index as usize];
    __mul_64x128_short(CA, CX.w[0], T128);

    ed2 = 33;
    if (__unsigned_compare_gt_128(CY, CA))
        ed2++;

    T128 = BID_POWER10_TABLE_128[ed2 as usize];
    __mul_128x128_to_256(CA4, CA, T128);

    ed2 += BID_ESTIMATE_DECIMAL_DIGITS[bin_index as usize];
    CQ.w[0] = CQ.w[1] = 0;
    diff_expon = diff_expon - ed2;

} else {
    // get CQ = CX/CY
    bid___div_128_by_128(&CQ, &CR, CX, CY);

    if (!CR.w[1] && !CR.w[0]) {
        bid_get_BID128(&res, sign_x ^ sign_y, diff_expon, CQ, &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
        // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
        return res;
    }
    // get number of decimal digits in CQ
    // 2^64
    f64.i = 0x5f800000;
    fx.d = (float)CQ.w[1] * f64.d + (float)CQ.w[0];
    // binary expon. of CQ
    bin_expon = (fx.i - 0x3f800000) >> 23;

    digits_q = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon as usize];
    TP128.w[0] = BID_POWER10_INDEX_BINEXP_128[bin_expon as usize].w[0];
    TP128.w[1] = BID_POWER10_INDEX_BINEXP_128[bin_expon as usize].w[1];
    if (__unsigned_compare_ge_128(CQ, TP128))
        digits_q++;

    ed2 = 34 - digits_q;
    T128.w[0] = BID_POWER10_TABLE_128[ed2 as usize].w[0];
    T128.w[1] = BID_POWER10_TABLE_128[ed2 as usize].w[1];
    __mul_128x128_to_256(CA4, CR, T128);
    diff_expon = diff_expon - ed2;
    __mul_128x128_low(CQ, CQ, T128);
}

bid___div_256_by_128(&CQ, &CA4, CY);

#ifdef BID_SET_STATUS_FLAGS
if (CA4.w[0] || CA4.w[1]) {
    // set status flags
    __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
}
#ifndef LEAVE_TRAILING_ZEROS
else
#endif
#else
#ifndef LEAVE_TRAILING_ZEROS
if (!CA4.w[0] && !CA4.w[1])
#endif
#endif
#ifndef LEAVE_TRAILING_ZEROS
// check whether result is exact
{
    // printf("ed2=%d,nz=%d,a=%d,CQ="BID_LX16","BID_LX16", RH="BID_LX16",
    // RL="BID_LX16"\n",ed2,nzeros,amount,CQ.w[1],CQ.w[0],reciprocals10_128[nzeros as usize].w[1],reciprocals10_128[nzeros as usize].w[0]);fflush(stdout);
    //  check whether CX, CY are short
    if (!CX.w[1] && !CY.w[1] && (CX.w[0] <= 1024) && (CY.w[0] <= 1024)) {
        i = (int)CY.w[0] - 1;
        j = (int)CX.w[0] - 1;
        // difference in powers of 2 BID_FACTORS for Y and X
        nzeros = ed2 - BID_FACTORS[i][0] + BID_FACTORS[j][0];
        // difference in powers of 5 BID_FACTORS
        d5 = ed2 - BID_FACTORS[i][1] + BID_FACTORS[j][1];
        if (d5 < nzeros)
            nzeros = d5;
        // get P*(2^M[extra_digits])/10^extra_digits
        __mul_128x128_full(Qh, Ql, CQ, BID_RECIPROCALS10_128[nzeros as usize]);
        //__mul_128x128_to_256(P256, CQ, BID_RECIPROCALS10_128[nzeros as usize]);Qh.w[1]=P256.w[3];Qh.w[0]=P256.w[2];

        // now get P/10^extra_digits: shift Q_high right by M[extra_digits]-128
        amount = BID_RECIP_SCALE[nzeros as usize];
        __shr_128_long(CQ, Qh, amount);

        diff_expon += nzeros;
    } else {
        // decompose Q as Qh*10^17 + Ql
        // T128 = BID_RECIPROCALS10_128[17];
        T128.w[0] = 0x44909befeb9fad49u64;
        T128.w[1] = 0x000b877aa3236a4bu64;
        __mul_128x128_to_256(P256, CQ, T128);
        // amount = BID_RECIP_SCALE[17];
        Q_high = (P256.w[2] >> 44) | (P256.w[3] << (64 - 44));
        Q_low = CQ.w[0] - Q_high * 100000000000000000u64;

        if (!Q_low) {
            diff_expon += 17;

            tdigit[0] = Q_high & 0x3ffffff;
            tdigit[1] = 0;
            QX = Q_high >> 26;
            QX32 = QX;
            nzeros = 0;

            for (j = 0; QX32; j++, QX32 >>= 7) {
                k = (QX32 & 127);
                tdigit[0] += BID_CONVERT_TABLE[j as usize][k as usize][0];
                tdigit[1] += BID_CONVERT_TABLE[j as usize][k as usize][1];
                if (tdigit[0] >= 100000000) {
                    tdigit[0] -= 100000000;
                    tdigit[1]++;
                }
            }

            if (tdigit[1] >= 100000000) {
                tdigit[1] -= 100000000;
                if (tdigit[1] >= 100000000)
                    tdigit[1] -= 100000000;
            }

            digit = tdigit[0];
            if (!digit && !tdigit[1])
                nzeros += 16;
            else {
                if (!digit) {
                    nzeros += 8;
                    digit = tdigit[1];
                }
                // decompose digit
                PD = (BID_UINT64)digit * 0x068DB8BBu64;
                digit_h = (BID_UINT32)(PD >> 40);
                // printf("i=%d, nz=%d, digit=%d (%d, %016I64x %016I64x)\n",i,nzeros,digit_h,digit,PD,digit_h);fflush(stdout);
                digit_low = digit - digit_h * 10000;

                if (!digit_low)
                    nzeros += 4;
                else
                    digit_h = digit_low;

                if (!(digit_h & 1))
                    nzeros += 3 & (BID_UINT32)(BID_PACKED_10000_ZEROS[digit_h >> 3] >> (digit_h & 7));
            }

            if (nzeros) {
                __mul_64x64_to_128(CQ, Q_high, BID_RECIPROCALS10_64[nzeros as usize]);

                // now get P/10^extra_digits: shift C64 right by M[extra_digits]-64
                amount = BID_SHORT_RECIP_SCALE[nzeros as usize];
                CQ.w[0] = CQ.w[1] >> amount;
            } else
                CQ.w[0] = Q_high;
            CQ.w[1] = 0;

            diff_expon += nzeros;
        } else {
            tdigit[0] = Q_low & 0x3ffffff;
            tdigit[1] = 0;
            QX = Q_low >> 26;
            QX32 = QX;
            nzeros = 0;

            for (j = 0; QX32; j++, QX32 >>= 7) {
                k = (QX32 & 127);
                tdigit[0] += BID_CONVERT_TABLE[j as usize][k as usize][0];
                tdigit[1] += BID_CONVERT_TABLE[j as usize][k as usize][1];
                if (tdigit[0] >= 100000000) {
                    tdigit[0] -= 100000000;
                    tdigit[1]++;
                }
            }

            if (tdigit[1] >= 100000000) {
                tdigit[1] -= 100000000;
                if (tdigit[1] >= 100000000)
                    tdigit[1] -= 100000000;
            }

            digit = tdigit[0];
            if (!digit && !tdigit[1])
                nzeros += 16;
            else {
                if (!digit) {
                    nzeros += 8;
                    digit = tdigit[1];
                }
                // decompose digit
                PD = (BID_UINT64)digit * 0x068DB8BBu64;
                digit_h = (BID_UINT32)(PD >> 40);
                // printf("i=%d, nz=%d, digit=%d (%d, %016I64x %016I64x)\n",i,nzeros,digit_h,digit,PD,digit_h);fflush(stdout);
                digit_low = digit - digit_h * 10000;

                if (!digit_low)
                    nzeros += 4;
                else
                    digit_h = digit_low;

                if (!(digit_h & 1))
                    nzeros += 3 & (BID_UINT32)(BID_PACKED_10000_ZEROS[digit_h >> 3] >> (digit_h & 7));
            }

            if (nzeros) {
                // get P*(2^M[extra_digits])/10^extra_digits
                __mul_128x128_full(Qh, Ql, CQ, BID_RECIPROCALS10_128[nzeros as usize]);

                // now get P/10^extra_digits: shift Q_high right by M[extra_digits]-128
                amount = BID_RECIP_SCALE[nzeros as usize];
                __shr_128(CQ, Qh, amount);
            }
            diff_expon += nzeros;
        }
    }
    bid_get_BID128(&res, sign_x ^ sign_y, diff_expon, CQ, &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
    return res;
}
#endif

if (diff_expon >= 0) {
#ifdef IEEE_ROUND_NEAREST
    // rounding
    // 2*CA4 - CY
    CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
    CA4r.w[0] = CA4.w[0] + CA4.w[0];
    __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
    CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;

    D = (CA4r.w[1] | CA4r.w[0]) ? 1 : 0;
    carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) & ((CQ.w[0]) | D);

    CQ.w[0] += carry64;
    if (CQ.w[0] < carry64)
        CQ.w[1]++;
#else
#ifdef IEEE_ROUND_NEAREST_TIES_AWAY
        // rounding
        // 2*CA4 - CY
        CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
        CA4r.w[0] = CA4.w[0] + CA4.w[0];
        __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
        CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;

        D = (CA4r.w[1] | CA4r.w[0]) ? 0 : 1;
        carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) | D;

        CQ.w[0] += carry64;
        if (CQ.w[0] < carry64)
            CQ.w[1]++;
#else
    rmode = rnd_mode;
    if (sign_x ^ sign_y && (unsigned)(rmode - 1) < 2)
        rmode = 3 - rmode;
    switch (rmode) {
        case BID_ROUNDING_TO_NEAREST: // round to nearest code
            // rounding
            // 2*CA4 - CY
            CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
            CA4r.w[0] = CA4.w[0] + CA4.w[0];
            __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
            CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;
            D = (CA4r.w[1] | CA4r.w[0]) ? 1 : 0;
            carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) & ((CQ.w[0]) | D);
            CQ.w[0] += carry64;
            if (CQ.w[0] < carry64)
                CQ.w[1]++;
            break;
        case BID_ROUNDING_TIES_AWAY:
            // rounding
            // 2*CA4 - CY
            CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
            CA4r.w[0] = CA4.w[0] + CA4.w[0];
            __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
            CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;
            D = (CA4r.w[1] | CA4r.w[0]) ? 0 : 1;
            carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) | D;
            CQ.w[0] += carry64;
            if (CQ.w[0] < carry64)
                CQ.w[1]++;
            break;
        case BID_ROUNDING_DOWN:
        case BID_ROUNDING_TO_ZERO:
            break;
        default: // rounding up
            CQ.w[0]++;
            if (!CQ.w[0])
                CQ.w[1]++;
            break;
    }
#endif
#endif

} else {
#ifdef BID_SET_STATUS_FLAGS
    if (CA4.w[0] || CA4.w[1]) {
        // set status flags
        __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
    }
#endif
    bid_handle_UF_128_rem(&res, sign_x ^ sign_y, diff_expon, CQ, CA4.w[1] | CA4.w[0], &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
    return res;
}

bid_get_BID128(&res, sign_x ^ sign_y, diff_expon, CQ, &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
// (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
return res;
}

BID128_FUNCTION_ARG128_ARGTYPE2(bid128qd_div, x, BID_UINT64, y)
BID_UINT256 CA4, CA4r, P256;
BID_UINT128 CX, CY, T128, CQ, CR, CA, TP128, Qh, Ql, res;
BID_UINT64 sign_x, sign_y, carry64, D, Q_high, Q_low, QX, PD, valid_y;
int_float fx, fy, f64;
BID_UINT32 QX32, tdigit[3], digit, digit_h, digit_low;
int exponent_x, exponent_y, bin_index, bin_expon, diff_expon, ed2, digits_q, amount;
int nzeros, i, j, k, d5, rmode;

BID_OPT_SAVE_BINARY_FLAGS()

valid_y = unpack_BID64(&sign_y, &exponent_y, &CY.w[0], y);
// unpack arguments, check for NaN or Infinity
if (!unpack_BID128_value(&sign_x, &exponent_x, &CX, x)) {
    // test if x is NaN
    if ((x.w[1] & 0x7c00000000000000u64) == 0x7c00000000000000u64) {
#ifdef BID_SET_STATUS_FLAGS
        if ((x.w[1] & 0x7e00000000000000u64) == 0x7e00000000000000u64 || // sNaN
            (y & 0x7e00000000000000u64) == 0x7e00000000000000u64)
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
        res.w[1] = (CX.w[1]) & QUIET_MASK64;
        res.w[0] = CX.w[0];
        return res;
    }
    // x is Infinity?
    if ((x.w[1] & 0x7800000000000000u64) == 0x7800000000000000u64) {
        // check if y is Inf.
        if (((y & 0x7c00000000000000u64) == 0x7800000000000000u64))
        // return NaN
        {
#ifdef BID_SET_STATUS_FLAGS
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0;
            return res;
        }
        // y is NaN?
        if (((y & 0x7c00000000000000u64) != 0x7c00000000000000u64))
        // return NaN
        {
            // return +/-Inf
            res.w[1] = ((x.w[1] ^ y) & 0x8000000000000000u64) | 0x7800000000000000u64;
            res.w[0] = 0;
            return res;
        }
    }
    // x is 0
    if ((y & 0x7800000000000000u64) < 0x7800000000000000u64) {
        if (!CY.w[0]) {
#ifdef BID_SET_STATUS_FLAGS
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
            // x=y=0, return NaN
            res.w[1] = 0x7c00000000000000u64;
            res.w[0] = 0;
            return res;
        }
        // return 0
        res.w[1] = (x.w[1] ^ y) & 0x8000000000000000u64;
        exponent_x = exponent_x - exponent_y + DECIMAL_EXPONENT_BIAS;
        if (exponent_x > DECIMAL_MAX_EXPON_128)
            exponent_x = DECIMAL_MAX_EXPON_128;
        else if (exponent_x < 0)
            exponent_x = 0;
        res.w[1] |= (((BID_UINT64)exponent_x) << 49);
        res.w[0] = 0;
        return res;
    }
}
CY.w[1] = 0;
if (!valid_y) {
    // y is Inf. or NaN

    // test if y is NaN
    if ((y & NAN_MASK64) == NAN_MASK64) {
#ifdef BID_SET_STATUS_FLAGS
        if ((y & SNAN_MASK64) == SNAN_MASK64) // sNaN
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
#endif
        res.w[0] = (CY.w[0] & 0x0003ffffffffffffu64);
        __mul_64x64_to_128(res, res.w[0], BID_POWER10_TABLE_128[18].w[0]);
        res.w[1] |= ((CY.w[0]) & 0xfc00000000000000u64);
        return res;
    }
    // y is Infinity?
    if ((y & INFINITY_MASK64) == INFINITY_MASK64) {
        // return +/-0
        res.w[1] = ((x.w[1] ^ y) & 0x8000000000000000u64);
        res.w[0] = 0;
        return res;
    }
    // y is 0
#ifdef BID_SET_STATUS_FLAGS
    __set_status_flags(pfpsf, StatusFlags::BID_ZERO_DIVIDE_EXCEPTION);
#endif
    res.w[1] = (sign_x ^ sign_y) | INFINITY_MASK64;
    res.w[0] = 0;
    return res;
}
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
// (void) fegetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
diff_expon = exponent_x - exponent_y + DECIMAL_EXPONENT_BIAS;

if (__unsigned_compare_gt_128(CY, CX)) {
    // CX < CY

    // 2^64
    f64.i = 0x5f800000;

    // fx ~ CX,   fy ~ CY
    fx.d = (float)CX.w[1] * f64.d + (float)CX.w[0];
    fy.d = (float)CY.w[1] * f64.d + (float)CY.w[0];
    // expon_cy - expon_cx
    bin_index = (fy.i - fx.i) >> 23;

    T128 = BID_POWER10_INDEX_BINEXP_128[bin_index as usize];
    __mul_64x128_short(CA, CX.w[0], T128);

    ed2 = 33;
    if (__unsigned_compare_gt_128(CY, CA))
        ed2++;

    T128 = BID_POWER10_TABLE_128[ed2 as usize];
    __mul_128x128_to_256(CA4, CA, T128);

    ed2 += BID_ESTIMATE_DECIMAL_DIGITS[bin_index as usize];
    CQ.w[0] = CQ.w[1] = 0;
    diff_expon = diff_expon - ed2;

} else {
    // get CQ = CX/CY
    bid___div_128_by_128(&CQ, &CR, CX, CY);

    if (!CR.w[1] && !CR.w[0]) {
        bid_get_BID128(&res, sign_x ^ sign_y, diff_expon, CQ, &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
        // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
        return res;
    }
    // get number of decimal digits in CQ
    // 2^64
    f64.i = 0x5f800000;
    fx.d = (float)CQ.w[1] * f64.d + (float)CQ.w[0];
    // binary expon. of CQ
    bin_expon = (fx.i - 0x3f800000) >> 23;

    digits_q = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon as usize];
    TP128.w[0] = BID_POWER10_INDEX_BINEXP_128[bin_expon as usize].w[0];
    TP128.w[1] = BID_POWER10_INDEX_BINEXP_128[bin_expon as usize].w[1];
    if (__unsigned_compare_ge_128(CQ, TP128))
        digits_q++;

    ed2 = 34 - digits_q;
    T128.w[0] = BID_POWER10_TABLE_128[ed2 as usize].w[0];
    T128.w[1] = BID_POWER10_TABLE_128[ed2 as usize].w[1];
    __mul_128x128_to_256(CA4, CR, T128);
    diff_expon = diff_expon - ed2;
    __mul_128x128_low(CQ, CQ, T128);
}

bid___div_256_by_128(&CQ, &CA4, CY);

#ifdef BID_SET_STATUS_FLAGS
if (CA4.w[0] || CA4.w[1]) {
    // set status flags
    __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
}
#ifndef LEAVE_TRAILING_ZEROS
else
#endif
#else
#ifndef LEAVE_TRAILING_ZEROS
if (!CA4.w[0] && !CA4.w[1])
#endif
#endif
#ifndef LEAVE_TRAILING_ZEROS
// check whether result is exact
{
    // check whether CX, CY are short
    if (!CX.w[1] && !CY.w[1] && (CX.w[0] <= 1024) && (CY.w[0] <= 1024)) {
        i = (int)CY.w[0] - 1;
        j = (int)CX.w[0] - 1;
        // difference in powers of 2 BID_FACTORS for Y and X
        nzeros = ed2 - BID_FACTORS[i][0] + BID_FACTORS[j][0];
        // difference in powers of 5 BID_FACTORS
        d5 = ed2 - BID_FACTORS[i][1] + BID_FACTORS[j][1];
        if (d5 < nzeros)
            nzeros = d5;
        // get P*(2^M[extra_digits])/10^extra_digits
        __mul_128x128_full(Qh, Ql, CQ, BID_RECIPROCALS10_128[nzeros as usize]);
        //__mul_128x128_to_256(P256, CQ, BID_RECIPROCALS10_128[nzeros as usize]);Qh.w[1]=P256.w[3];Qh.w[0]=P256.w[2];

        // now get P/10^extra_digits: shift Q_high right by M[extra_digits]-128
        amount = BID_RECIP_SCALE[nzeros as usize];
        __shr_128_long(CQ, Qh, amount);

        diff_expon += nzeros;
    } else {
        // decompose Q as Qh*10^17 + Ql
        // T128 = BID_RECIPROCALS10_128[17];
        T128.w[0] = 0x44909befeb9fad49u64;
        T128.w[1] = 0x000b877aa3236a4bu64;
        __mul_128x128_to_256(P256, CQ, T128);
        // amount = BID_RECIP_SCALE[17];
        Q_high = (P256.w[2] >> 44) | (P256.w[3] << (64 - 44));
        Q_low = CQ.w[0] - Q_high * 100000000000000000u64;

        if (!Q_low) {
            diff_expon += 17;

            tdigit[0] = Q_high & 0x3ffffff;
            tdigit[1] = 0;
            QX = Q_high >> 26;
            QX32 = QX;
            nzeros = 0;

            for (j = 0; QX32; j++, QX32 >>= 7) {
                k = (QX32 & 127);
                tdigit[0] += BID_CONVERT_TABLE[j as usize][k as usize][0];
                tdigit[1] += BID_CONVERT_TABLE[j as usize][k as usize][1];
                if (tdigit[0] >= 100000000) {
                    tdigit[0] -= 100000000;
                    tdigit[1]++;
                }
            }

            if (tdigit[1] >= 100000000) {
                tdigit[1] -= 100000000;
                if (tdigit[1] >= 100000000)
                    tdigit[1] -= 100000000;
            }

            digit = tdigit[0];
            if (!digit && !tdigit[1])
                nzeros += 16;
            else {
                if (!digit) {
                    nzeros += 8;
                    digit = tdigit[1];
                }
                // decompose digit
                PD = (BID_UINT64)digit * 0x068DB8BBu64;
                digit_h = (BID_UINT32)(PD >> 40);
                digit_low = digit - digit_h * 10000;

                if (!digit_low)
                    nzeros += 4;
                else
                    digit_h = digit_low;

                if (!(digit_h & 1))
                    nzeros += 3 & (BID_UINT32)(BID_PACKED_10000_ZEROS[digit_h >> 3] >> (digit_h & 7));
            }

            if (nzeros) {
                __mul_64x64_to_128(CQ, Q_high, BID_RECIPROCALS10_64[nzeros as usize]);

                // now get P/10^extra_digits: shift C64 right by M[extra_digits]-64
                amount = BID_SHORT_RECIP_SCALE[nzeros as usize];
                CQ.w[0] = CQ.w[1] >> amount;
            } else
                CQ.w[0] = Q_high;
            CQ.w[1] = 0;

            diff_expon += nzeros;
        } else {
            tdigit[0] = Q_low & 0x3ffffff;
            tdigit[1] = 0;
            QX = Q_low >> 26;
            QX32 = QX;
            nzeros = 0;

            for (j = 0; QX32; j++, QX32 >>= 7) {
                k = (QX32 & 127);
                tdigit[0] += BID_CONVERT_TABLE[j as usize][k as usize][0];
                tdigit[1] += BID_CONVERT_TABLE[j as usize][k as usize][1];
                if (tdigit[0] >= 100000000) {
                    tdigit[0] -= 100000000;
                    tdigit[1]++;
                }
            }

            if (tdigit[1] >= 100000000) {
                tdigit[1] -= 100000000;
                if (tdigit[1] >= 100000000)
                    tdigit[1] -= 100000000;
            }

            digit = tdigit[0];
            if (!digit && !tdigit[1])
                nzeros += 16;
            else {
                if (!digit) {
                    nzeros += 8;
                    digit = tdigit[1];
                }
                // decompose digit
                PD = (BID_UINT64)digit * 0x068DB8BBu64;
                digit_h = (BID_UINT32)(PD >> 40);
                digit_low = digit - digit_h * 10000;

                if (!digit_low)
                    nzeros += 4;
                else
                    digit_h = digit_low;

                if (!(digit_h & 1))
                    nzeros += 3 & (BID_UINT32)(BID_PACKED_10000_ZEROS[digit_h >> 3] >> (digit_h & 7));
            }

            if (nzeros) {
                // get P*(2^M[extra_digits])/10^extra_digits
                __mul_128x128_full(Qh, Ql, CQ, BID_RECIPROCALS10_128[nzeros as usize]);

                // now get P/10^extra_digits: shift Q_high right by M[extra_digits]-128
                amount = BID_RECIP_SCALE[nzeros as usize];
                __shr_128(CQ, Qh, amount);
            }
            diff_expon += nzeros;
        }
    }
    bid_get_BID128(&res, sign_x ^ sign_y, diff_expon, CQ, &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
    return res;
}
#endif

if (diff_expon >= 0) {
#ifdef IEEE_ROUND_NEAREST
    // rounding
    // 2*CA4 - CY
    CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
    CA4r.w[0] = CA4.w[0] + CA4.w[0];
    __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
    CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;

    D = (CA4r.w[1] | CA4r.w[0]) ? 1 : 0;
    carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) & ((CQ.w[0]) | D);

    CQ.w[0] += carry64;
    if (CQ.w[0] < carry64)
        CQ.w[1]++;
#else
#ifdef IEEE_ROUND_NEAREST_TIES_AWAY
        // rounding
        // 2*CA4 - CY
        CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
        CA4r.w[0] = CA4.w[0] + CA4.w[0];
        __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
        CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;

        D = (CA4r.w[1] | CA4r.w[0]) ? 0 : 1;
        carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) | D;

        CQ.w[0] += carry64;
        if (CQ.w[0] < carry64)
            CQ.w[1]++;
#else
    rmode = rnd_mode;
    if (sign_x ^ sign_y && (unsigned)(rmode - 1) < 2)
        rmode = 3 - rmode;
    switch (rmode) {
        case BID_ROUNDING_TO_NEAREST: // round to nearest code
            // rounding
            // 2*CA4 - CY
            CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
            CA4r.w[0] = CA4.w[0] + CA4.w[0];
            __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
            CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;
            D = (CA4r.w[1] | CA4r.w[0]) ? 1 : 0;
            carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) & ((CQ.w[0]) | D);
            CQ.w[0] += carry64;
            if (CQ.w[0] < carry64)
                CQ.w[1]++;
            break;
        case BID_ROUNDING_TIES_AWAY:
            // rounding
            // 2*CA4 - CY
            CA4r.w[1] = (CA4.w[1] + CA4.w[1]) | (CA4.w[0] >> 63);
            CA4r.w[0] = CA4.w[0] + CA4.w[0];
            __sub_borrow_out(CA4r.w[0], carry64, CA4r.w[0], CY.w[0]);
            CA4r.w[1] = CA4r.w[1] - CY.w[1] - carry64;
            D = (CA4r.w[1] | CA4r.w[0]) ? 0 : 1;
            carry64 = (1 + (((BID_SINT64)CA4r.w[1]) >> 63)) | D;
            CQ.w[0] += carry64;
            if (CQ.w[0] < carry64)
                CQ.w[1]++;
            break;
        case BID_ROUNDING_DOWN:
        case BID_ROUNDING_TO_ZERO:
            break;
        default: // rounding up
            CQ.w[0]++;
            if (!CQ.w[0])
                CQ.w[1]++;
            break;
    }
#endif
#endif

} else {
#ifdef BID_SET_STATUS_FLAGS
    if (CA4.w[0] || CA4.w[1]) {
        // set status flags
        __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
    }
#endif
    bid_handle_UF_128_rem(&res, sign_x ^ sign_y, diff_expon, CQ, CA4.w[1] | CA4.w[0], &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
    // (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
    return res;
}

bid_get_BID128(&res, sign_x ^ sign_y, diff_expon, CQ, &rnd_mode, pfpsf);
#ifdef UNCHANGED_BINARY_STATUS_FLAGS
// (void) fesetexceptflag (&binaryflags, BID_FE_ALL_FLAGS);
#endif
return res;
}
*/