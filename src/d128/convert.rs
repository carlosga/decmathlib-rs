/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez         */
/* --------------------------------------------------------------------- */
/* Original C source code Copyright (c) 2018, Intel Corp.                */
/* --------------------------------------------------------------------- */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::d128::bid_internal::*;
use crate::d128::constants::*;
use crate::d128::bid_decimal_data::*;
use crate::d128::core::{RoundingMode, StatusFlags};
use crate::d128::dec128::{_IDEC_flags, BID_SINT64, BID_UI32FLOAT, BID_UINT128, BID_UINT64};

#[cfg(target_endian = "big")]
use crate::d128::bid_conf::BID_SWAP128;

/// Takes a BID64 as input and converts it to a BID128 and returns it.
pub fn bid64_to_bid128(x: BID_UINT64, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut new_coeff: BID_UINT128    = BID_UINT128::default();
    let mut res: BID_UINT128          = BID_UINT128::default();
    let mut sign_x: BID_UINT64        = 0;
    let mut exponent_x: i32           = 0;
    let mut coefficient_x: BID_UINT64 = 0;

    if unpack_BID64(&mut sign_x, &mut exponent_x, &mut coefficient_x, x) == 0 {
        if ((x) << 1) >= 0xf000000000000000u64 {
            if ((x) & SNAN_MASK64) == SNAN_MASK64 {   // sNaN
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            res.w[0] = coefficient_x & 0x0003ffffffffffffu64;
            let cx = res.w[0];
            res = __mul_64x64_to_128(cx, bid_power10_table_128[18].w[0]);
            res.w[1] |= (coefficient_x) & 0xfc00000000000000u64;
            return res;
        }
    }

    new_coeff.w[0] = coefficient_x;
    new_coeff.w[1] = 0;

    bid_get_BID128_very_fast(
        &mut res, sign_x,
        exponent_x + DECIMAL_EXPONENT_BIAS_128 - DECIMAL_EXPONENT_BIAS,
        &new_coeff);

    res
}

/// Takes a BID128 as input and converts it to a BID64 and returns it.
pub (crate) fn bid128_to_bid64(x: &BID_UINT128, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let mut CX: BID_UINT128    = BID_UINT128::default();
    let mut T128: BID_UINT128;
    let TP128: BID_UINT128;
    let mut Qh: BID_UINT128;
    let Ql: BID_UINT128;
    let mut Qh1: BID_UINT128;
    let mut Stemp: BID_UINT128 = BID_UINT128::default();
    let mut Tmp: BID_UINT128   = BID_UINT128::default();
    let Tmp1: BID_UINT128;
    let mut CX1: BID_UINT128   = BID_UINT128::default();
    let mut sign_x: BID_UINT64 = 0;
    let mut carry: BID_UINT64  = 0;
    let mut cy: BID_UINT64     = 0;
    let res: BID_UINT64;
    let D: BID_SINT64;
    let mut f64: BID_UI32FLOAT = BID_UI32FLOAT::default();
    let mut fx: BID_UI32FLOAT  = BID_UI32FLOAT::default();
    let mut exponent_x: i32    = 0;
    let mut extra_digits: i32;
    let amount: i32;
    let bin_expon_cx: i32;
    let mut rmode: u32;
    let mut status: u32;
    let mut uf_check= 0;

    // TODO
    // BID_OPT_SAVE_BINARY_FLAGS()

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    // unpack arguments, check for NaN or Infinity or 0
    if unpack_BID128(&mut sign_x, &mut exponent_x, &mut CX, x)  == 0 {
        if (x.w[1] << 1) >= 0xf000000000000000u64 {
            Tmp.w[1] = CX.w[1] & 0x00003fffffffffffu64;
            Tmp.w[0] = CX.w[0];
            TP128    = bid_reciprocals10_128[18];
            (Qh, _)  = __mul_128x128_full(&Tmp, &TP128);
            amount   = bid_recip_scale[18];
            Tmp      = __shr_128(&Qh, amount);
            res      = (CX.w[1] & 0xfc00000000000000u64) | Tmp.w[0];

            if (x.w[1] & SNAN_MASK64) == SNAN_MASK64 {
                // sNaN
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }

            return res;
        }

        exponent_x = exponent_x - DECIMAL_EXPONENT_BIAS_128 + DECIMAL_EXPONENT_BIAS;
        if exponent_x < 0 {
            res = sign_x;
            return res;
        }
        if exponent_x > DECIMAL_MAX_EXPON_64 {
            exponent_x = DECIMAL_MAX_EXPON_64;
        }
        res = sign_x | ((exponent_x as BID_UINT64) << 53);
        return res;
    }

    if CX.w[1] != 0 || (CX.w[0] >= 10000000000000000u64) {
        unsafe {
            // find number of digits in coefficient
            // 2^64
            f64.i = 0x5f800000;
            // fx ~ CX
            fx.f         = CX.w[1] as f32 * f64.f + CX.w[0] as f32;
            bin_expon_cx = (((fx.i >> 23) & 0xff) - 0x7f) as i32;
            extra_digits = bid_estimate_decimal_digits[bin_expon_cx as usize] - 16;
            // scale = 38-estimate_decimal_digits[bin_expon_cx];
            D = (CX.w[1] - bid_power10_index_binexp_128[bin_expon_cx as usize].w[1]) as BID_SINT64;
            if D > 0 || (D == 0 && CX.w[0] >= bid_power10_index_binexp_128[bin_expon_cx as usize].w[0]) {
                extra_digits += 1;
            }
        }

        exponent_x += extra_digits;

        rmode = rnd_mode;
        if sign_x != 0 && ((rmode - 1) as u32) < 2 {
            rmode = 3 - rmode;
        }

        if exponent_x < DECIMAL_EXPONENT_BIAS_128 - DECIMAL_EXPONENT_BIAS {
            uf_check = 1;
            if -extra_digits + exponent_x - DECIMAL_EXPONENT_BIAS_128 + DECIMAL_EXPONENT_BIAS + 35 >= 0 {
                if exponent_x == DECIMAL_EXPONENT_BIAS_128 - DECIMAL_EXPONENT_BIAS - 1 {
                    T128 = bid_round_const_table_128[rmode as usize][extra_digits as usize];
                    __add_carry_out(&mut CX1.w[0], &mut carry, T128.w[0], CX.w[0]);
                    CX1.w[1] = CX.w[1] + T128.w[1] + carry;
                    #[cfg(DECIMAL_TINY_DETECTION_AFTER_ROUNDING)]
                    if (__unsigned_compare_ge_128(CX1, bid_power10_table_128[extra_digits + 16])) {
                        uf_check = 0;
                    }
                }
                extra_digits = extra_digits + DECIMAL_EXPONENT_BIAS_128 - DECIMAL_EXPONENT_BIAS - exponent_x;
                exponent_x = DECIMAL_EXPONENT_BIAS_128 - DECIMAL_EXPONENT_BIAS;
                //uf_check = 2;
            } else {
                rmode = RoundingMode::BID_ROUNDING_TO_ZERO;
            }
        }

        T128 = bid_round_const_table_128[rmode as usize][extra_digits as usize];
        let cxy = CX.w[0];
        __add_carry_out(&mut CX.w[0], &mut carry, T128.w[0], cxy);
        CX.w[1] = CX.w[1] + T128.w[1] + carry;

        TP128    = bid_reciprocals10_128[extra_digits as usize];
        (Qh, Ql) = __mul_128x128_full(&CX, &TP128);
        amount   = bid_recip_scale[extra_digits as usize];

        if amount >= 64 {
            CX.w[0] = Qh.w[1] >> (amount - 64);
            CX.w[1] = 0;
        } else {
            CX = __shr_128(&Qh, amount);
        }

        if rmode == 0 {
            if (CX.w[0] & 1) == 1 {
                // check whether fractional part of initial_P/10^ed1 is exactly .5

                // get remainder
                Qh1 = __shl_128_long(&Qh, 128 - amount);

                if Qh1.w[1] == 0 && Qh1.w[0] == 0
                && (Ql.w[1]  < bid_reciprocals10_128[extra_digits as usize].w[1]
                || (Ql.w[1] == bid_reciprocals10_128[extra_digits as usize].w[1]
                 && Ql.w[0]  < bid_reciprocals10_128[extra_digits as usize].w[0])) {
                    CX.w[0] -= 1;
                }
            }
        }

        {
            status = StatusFlags::BID_INEXACT_EXCEPTION;
            // get remainder
            Qh1 = __shl_128_long(&Qh, 128 - amount);
            match rmode {
                RoundingMode::BID_ROUNDING_TO_NEAREST | RoundingMode::BID_ROUNDING_TIES_AWAY => {
                    // test whether fractional part is 0
                    if Qh1.w[1] == 0x8000000000000000u64 && Qh1.w[0] == 0
                    && (Ql.w[1]  < bid_reciprocals10_128[extra_digits as usize].w[1]
                    || (Ql.w[1] == bid_reciprocals10_128[extra_digits as usize].w[1]
                     && Ql.w[0]  < bid_reciprocals10_128[extra_digits as usize].w[0])) {
                        status = BID_EXACT_STATUS;
                    }
                },
                RoundingMode::BID_ROUNDING_DOWN | RoundingMode::BID_ROUNDING_TO_ZERO => {
                    if (Qh1.w[1] == 0) && (Qh1.w[0] == 0)
                    && (Ql.w[1]  < bid_reciprocals10_128[extra_digits as usize].w[1]
                    || (Ql.w[1] == bid_reciprocals10_128[extra_digits as usize].w[1]
                     && Ql.w[0]  < bid_reciprocals10_128[extra_digits as usize].w[0])) {
                        status = BID_EXACT_STATUS;
                    }
                },
                _ => {
                    // round up
                    __add_carry_out(&mut Stemp.w[0], &mut cy, Ql.w[0], bid_reciprocals10_128[extra_digits as usize].w[0]);
                    __add_carry_in_out(&mut Stemp.w[1], &mut carry, Ql.w[1], bid_reciprocals10_128[extra_digits as usize].w[1], cy);
                    Qh = __shr_128_long(&Qh1, 128 - amount);
                    Tmp.w[0] = 1;
                    Tmp.w[1] = 0;
                    Tmp1 = __shl_128_long(&Tmp, amount);
                    Qh.w[0] += carry;
                    if Qh.w[0] < carry {
                        Qh.w[1] += 1;
                    }
                    if __unsigned_compare_ge_128(Qh, Tmp1) {
                        status = BID_EXACT_STATUS;
                    }
                }
            }

            if status != BID_EXACT_STATUS {
                if uf_check != 0 {
                    status |= StatusFlags::BID_UNDERFLOW_EXCEPTION;
                }
                __set_status_flags(pfpsf, status);
            }
        }
    }

    res = get_BID64(sign_x, exponent_x - DECIMAL_EXPONENT_BIAS_128 + DECIMAL_EXPONENT_BIAS, CX.w[0], rnd_mode, pfpsf);

    return res;
}