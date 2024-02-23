/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_snake_case)]

use crate::bid_internal::{__add_128_128, __set_status_flags, bid_get_BID128, bid_get_BID128_very_fast, BID_SINT64, BID_UINT128, BID_UINT32, BID_UINT64, DECIMAL_MAX_EXPON_128, QUIET_MASK64, SNAN_MASK64, unpack_BID128_value};
use crate::d128::{_IDEC_flags, RoundingMode, StatusFlags};

/// multiply a 128-bit decimal floating-point value by an integral power of 2.
pub (crate) fn bid128_ldexp(x: &BID_UINT128, n: i32, rnd_mode: RoundingMode, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut CX: BID_UINT128 = BID_UINT128::default();
    let mut CX2: BID_UINT128 = BID_UINT128::default();
    let mut CBID_X8: BID_UINT128 = BID_UINT128::default();
    let mut res: BID_UINT128 = BID_UINT128::default();
    let mut exp64: BID_SINT64;
    let mut sign_x: BID_UINT64 = 0;
    let mut exponent_x: i32 = 0;

    // unpack arguments, check for NaN or Infinity
    if unpack_BID128_value(&mut sign_x, &mut exponent_x, &mut CX, x) == 0 {
        // x is Inf. or NaN or 0
        if (x.w[1] & SNAN_MASK64) == SNAN_MASK64 { // y is sNaN
            __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
        }
        res.w[1] = CX.w[1] & QUIET_MASK64;
        res.w[0] = CX.w[0];
        if CX.w[1] == 0 {
            exp64 = (exponent_x as BID_SINT64) + (n as BID_SINT64);
            if exp64 < 0 {
                exp64 = 0;
            }
            if exp64 > DECIMAL_MAX_EXPON_128 as i64 {
                exp64 = DECIMAL_MAX_EXPON_128 as BID_SINT64;
            };
            exponent_x = exp64 as i32;
            res = bid_get_BID128_very_fast(sign_x, exponent_x, &CX);
        }
        return res;
    }

    exp64      = (exponent_x as BID_SINT64) + (n as BID_SINT64);
    exponent_x = exp64 as i32;

    if (exponent_x as BID_UINT32) <= DECIMAL_MAX_EXPON_128 as u32 {
        res = bid_get_BID128_very_fast(sign_x, exponent_x, &CX);
        return res;
    }
    // check for overflow
    if exp64 > DECIMAL_MAX_EXPON_128 as i64 {
        if CX.w[1] < 0x314dc6448d93u64 {
            // try to normalize coefficient
            loop {
                CBID_X8.w[1] = (CX.w[1] << 3) | (CX.w[0] >> 61);
                CBID_X8.w[0] =  CX.w[0] << 3;
                CX2.w[1]     = (CX.w[1] << 1) | (CX.w[0] >> 63);
                CX2.w[0]     =  CX.w[0] << 1;
                CX           = __add_128_128(&CX2, &CBID_X8);

                exponent_x -= 1;
                exp64      -= 1;

                if ! (CX.w[1] < 0x314dc6448d93u64 && exp64 > DECIMAL_MAX_EXPON_128 as i64) {
                    break;
                }
            }
        }
        if exp64 <= DECIMAL_MAX_EXPON_128 as i64 {
            res = bid_get_BID128_very_fast(sign_x, exponent_x, &CX);
            return res;
        } else {
            exponent_x = 0x7fffffff; // overflow
        }
    }
    // exponent < 0
    // the BID pack routine will round the coefficient
    res = bid_get_BID128(sign_x, exponent_x, &CX, rnd_mode, pfpsf);
    res
}