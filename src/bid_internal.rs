/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::bid_decimal_data::{bid_power10_table_128, bid_recip_scale, bid_reciprocals10_128, bid_round_const_table};
use crate::constants::*;
use crate::core::{RoundingMode, StatusFlags};
use crate::dec128::{_IDEC_flags, BID_UINT128, BID_UINT192, BID_UINT256, BID_UINT32, BID_UINT384, BID_UINT512, BID_UINT64};

///  BID32 unpack, input pased by reference
pub (crate) fn unpack_BID32(psign_x: &mut BID_UINT32, pexponent_x: &mut i32, pcoefficient_x: &mut BID_UINT32, x: BID_UINT32) -> BID_UINT32 {
    let tmp: BID_UINT32;

    *psign_x = x & 0x80000000;

    if (x & SPECIAL_ENCODING_MASK32) == SPECIAL_ENCODING_MASK32 {
        // special encodings
        if (x & INFINITY_MASK32) == INFINITY_MASK32 {
            *pcoefficient_x = x & 0xfe0fffff;
            if (x & 0x000fffff) >= 1000000 {
                *pcoefficient_x = x & 0xfe000000;
            }
            if (x & NAN_MASK32) == INFINITY_MASK32 {
                *pcoefficient_x = x & 0xf8000000;
            }
            *pexponent_x = 0;

            return 0; // NaN or Infinity
        }
        // coefficient
        *pcoefficient_x = (x & SMALL_COEFF_MASK32) | LARGE_COEFF_HIGH_BIT32;
        // check for non-canonical value
        if *pcoefficient_x >= 10000000 {
            *pcoefficient_x = 0;
        }
        // get exponent
        tmp = x >> 21;
        *pexponent_x = (tmp & EXPONENT_MASK32) as i32;
        return *pcoefficient_x;
    }
    // exponent
    tmp = x >> 23;
    *pexponent_x    = (tmp & EXPONENT_MASK32) as i32;
    // coefficient
    *pcoefficient_x = x & LARGE_COEFF_MASK32;

    *pcoefficient_x
}

///  BID64 unpack, input pased by reference
pub (crate) fn unpack_BID64(psign_x: &mut BID_UINT64, pexponent_x: &mut i32, pcoefficient_x: &mut BID_UINT64, x: BID_UINT64) -> BID_UINT64 {
    let tmp: BID_UINT64;
    let mut coeff: BID_UINT64;

    *psign_x = x & 0x8000000000000000u64;

    if (x & SPECIAL_ENCODING_MASK64) == SPECIAL_ENCODING_MASK64 {
        // special encodings
        // coefficient
        coeff = (x & LARGE_COEFF_MASK64) | LARGE_COEFF_HIGH_BIT64;

        if (x & INFINITY_MASK64) == INFINITY_MASK64 {
            *pexponent_x    = 0i32;
            *pcoefficient_x = x & 0xfe03ffffffffffffu64;
            if (x & 0x0003ffffffffffffu64) >= 1000000000000000u64 {
                *pcoefficient_x = x & 0xfe00000000000000u64;
            }
            if (x & NAN_MASK64) == INFINITY_MASK64 {
                *pcoefficient_x = x & SINFINITY_MASK64;
            }
            return 0; // NaN or Infinity
        }
        // check for non-canonical values
        if coeff >= 10000000000000000u64 {
            coeff = 0;
        }
        *pcoefficient_x = coeff;
        // get exponent
        tmp          = x >> EXPONENT_SHIFT_LARGE64;
        *pexponent_x = (tmp & EXPONENT_MASK64) as i32;
        return coeff;
    }
    // exponent
    tmp             = x >> EXPONENT_SHIFT_SMALL64;
    *pexponent_x    = (tmp & EXPONENT_MASK64) as i32;
    // coefficient
    *pcoefficient_x = x & SMALL_COEFF_MASK64;

    *pcoefficient_x
}

///  BID128 unpack, input pased by reference
pub (crate) fn unpack_BID128(psign_x: &mut BID_UINT64, pexponent_x: &mut i32, pcoefficient_x: &mut BID_UINT128, px: &BID_UINT128) -> BID_UINT64 {
    let mut coeff: BID_UINT128 = BID_UINT128::default();
    let T33: BID_UINT128;
    let T34: BID_UINT128;
    let ex: BID_UINT64;

    *psign_x = (px.w[1]) & 0x8000000000000000u64;

    // special encodings
    if (px.w[1] & INFINITY_MASK64) >= SPECIAL_ENCODING_MASK64 {
        if (px.w[1] & INFINITY_MASK64) < INFINITY_MASK64 {
            // non-canonical input
            pcoefficient_x.w[0] = 0;
            pcoefficient_x.w[1] = 0;
            ex                  = (px.w[1]) >> 47;
            *pexponent_x        = (ex as i32) & EXPONENT_MASK128;
            return 0;
        }
        // 10^33
        T33                 = bid_power10_table_128[33];
        coeff.w[0]          = px.w[0];
        coeff.w[1]          = (px.w[1]) & LARGE_COEFF_MASK128;
        pcoefficient_x.w[0] = px.w[0];
        pcoefficient_x.w[1] = px.w[1];
        if __unsigned_compare_ge_128(coeff, T33) { // non-canonical
            pcoefficient_x.w[1] &= !LARGE_COEFF_MASK128;
            pcoefficient_x.w[0] = 0;
        }
        *pexponent_x = 0;
        return 0; // NaN or Infinity
    }

    coeff.w[0] = px.w[0];
    coeff.w[1] = (px.w[1]) & SMALL_COEFF_MASK128;

    // 10^34
    T34 = bid_power10_table_128[34];
    // check for non-canonical values
    if __unsigned_compare_ge_128 (coeff, T34) {
        coeff.w[0] = 0;
        coeff.w[1] = 0;
    }

    pcoefficient_x.w[0] = coeff.w[0];
    pcoefficient_x.w[1] = coeff.w[1];

    ex           = (px.w[1]) >> 49;
    *pexponent_x = (ex as i32) & EXPONENT_MASK128;

    coeff.w[0] | coeff.w[1]
}

/// BID64 pack macro (general form)
pub (crate) fn get_BID64(sgn: BID_UINT64, mut expon: i32, mut coeff: BID_UINT64, mut rmode: u32, fpsc: &mut _IDEC_flags) -> BID_UINT64 {
    let mut Stemp: BID_UINT128 = BID_UINT128::default();
    let Q_low: BID_UINT128;
    let QH: BID_UINT64;
    let mut r: BID_UINT64;
    let mut mask: BID_UINT64;
    let mut _C64: BID_UINT64 = 0;
    let mut remainder_h: BID_UINT64;
    let CY: BID_UINT64;
    let carry: BID_UINT64;
    let extra_digits: i32;
    let amount: i32;
    let amount2: i32;
    let mut status: u32;

    if coeff > 9999999999999999u64 {
        expon += 1;
        coeff  = 1000000000000000u64;
    }
    // check for possible underflow/overflow
    if (expon as u32) >= 3 * 256 {
        if expon < 0 {
            // underflow
            if expon + (MAX_FORMAT_DIGITS as i32) < 0 {
                __set_status_flags(fpsc, StatusFlags::BID_UNDERFLOW_EXCEPTION | StatusFlags::BID_INEXACT_EXCEPTION);

                if rmode == RoundingMode::BID_ROUNDING_DOWN && sgn != 0 {
                    return 0x8000000000000001u64;
                }
                if rmode == RoundingMode::BID_ROUNDING_UP && sgn == 0 {
                    return 1u64;
                }
                // result is 0
                return sgn;
            }
            if sgn != 0 && (rmode - 1) < 2 {
                rmode = 3 - rmode;
            }

            // get digits to be shifted out
            extra_digits = -expon;
            coeff       += bid_round_const_table[rmode as usize][extra_digits as usize];

            // get coeff*(2^M[extra_digits])/10^extra_digits
            (QH, Q_low) = __mul_64x128_full(coeff, &bid_reciprocals10_128[extra_digits as usize]);

            // now get P/10^extra_digits: shift Q_high right by M[extra_digits]-128
            amount = bid_recip_scale[extra_digits as usize];

            _C64 = QH >> amount;

            if rmode == 0 {    //BID_ROUNDING_TO_NEAREST
                if (_C64 & 1) == 1 {
                    // check whether fractional part of initial_P/10^extra_digits is exactly .5

                    // get remainder
                    amount2       = 64 - amount;
                    remainder_h   = 0;
                    remainder_h   = remainder_h.wrapping_sub(1);
                    remainder_h >>= amount2;
                    remainder_h   = remainder_h & QH;

                    if remainder_h == 0
                        && (Q_low.w[1]  < bid_reciprocals10_128[extra_digits as usize].w[1]
                        || (Q_low.w[1] == bid_reciprocals10_128[extra_digits as usize].w[1]
                         && Q_low.w[0]  < bid_reciprocals10_128[extra_digits as usize].w[0])) {
                        _C64 -= 1;
                    }
                }

                if is_inexact(*fpsc) {
                    __set_status_flags(fpsc, StatusFlags::BID_UNDERFLOW_EXCEPTION);
                } else {
                    status = StatusFlags::BID_INEXACT_EXCEPTION;
                    // get remainder
                    remainder_h = QH << (64 - amount);

                    match rmode {
                        RoundingMode::BID_ROUNDING_TO_NEAREST | RoundingMode::BID_ROUNDING_TIES_AWAY => {
                            if remainder_h == 0x8000000000000000u64
                            && (Q_low.w[1]  < bid_reciprocals10_128[extra_digits as usize].w[1]
                            || (Q_low.w[1] == bid_reciprocals10_128[extra_digits as usize].w[1]
                             && Q_low.w[0]  < bid_reciprocals10_128[extra_digits as usize].w[0])) {
                                status = BID_EXACT_STATUS;
                            }
                        },
                        RoundingMode::BID_ROUNDING_DOWN | RoundingMode::BID_ROUNDING_TO_ZERO => { // test whether fractional part is 0
                            if remainder_h == 0
                            && (Q_low.w[1]  < bid_reciprocals10_128[extra_digits as usize].w[1]
                            || (Q_low.w[1] == bid_reciprocals10_128[extra_digits as usize].w[1]
                             && Q_low.w[0]  < bid_reciprocals10_128[extra_digits as usize].w[0])) {
                                status = BID_EXACT_STATUS;
                            }
                        },
                        _ => { // round up
                            (Stemp.w[0], CY) = __add_carry_out(Q_low.w[0], bid_reciprocals10_128[extra_digits as usize].w[0]);
                            (Stemp.w[1], carry) = __add_carry_in_out(Q_low.w[1], bid_reciprocals10_128[extra_digits as usize].w[1], CY);
                            if (remainder_h >> (64 - amount)) + carry >= ((1u64) << amount) {
                                status = BID_EXACT_STATUS;
                            }
                        }
                    };

                    if status != BID_EXACT_STATUS {
                        __set_status_flags(fpsc, StatusFlags::BID_UNDERFLOW_EXCEPTION | status);
                    }
                }

                return sgn | _C64;
            }
        }
        if coeff == 0 {
            if expon > DECIMAL_MAX_EXPON_64 {
                expon = DECIMAL_MAX_EXPON_64;
            }
        }
        while coeff < 1000000000000000u64 && expon >= 3 * 256 {
            expon -= 1;
            coeff = (coeff << 3) + (coeff << 1);
        }
        if expon > DECIMAL_MAX_EXPON_64 {
            __set_status_flags(fpsc, StatusFlags::BID_OVERFLOW_EXCEPTION | StatusFlags::BID_INEXACT_EXCEPTION);
            // overflow
            r = sgn | INFINITY_MASK64;
            match rmode {
                RoundingMode::BID_ROUNDING_DOWN => {
                    if sgn == 0 {
                        r = LARGEST_BID64;
                    }
                },
                RoundingMode::BID_ROUNDING_TO_ZERO => r = sgn | LARGEST_BID64,
                RoundingMode::BID_ROUNDING_UP => { // round up
                    if sgn != 0 {
                        r = SMALLEST_BID64;
                    }
                },
                _ => {}
            };
            return r;
        }
    }

    mask   = 1;
    mask <<= EXPONENT_SHIFT_SMALL64;

    // check whether coefficient fits in 10*5+3 bits
    if coeff < mask {
        r   = expon as BID_UINT64;
        r <<= EXPONENT_SHIFT_SMALL64;
        r  |= coeff | sgn;
        return r;
    }
    // special format

    // eliminate the case coeff==10^16 after rounding
    if coeff == 10000000000000000u64 {
        r = (expon + 1) as BID_UINT64;
        r <<= EXPONENT_SHIFT_SMALL64;
        r |= 1000000000000000u64 | sgn;
        return r;
    }

    r      = expon as BID_UINT64;
    r    <<= EXPONENT_SHIFT_LARGE64;
    r     |= sgn | SPECIAL_ENCODING_MASK64;
    // add coeff, without leading bits
    mask   = (mask >> 2) - 1;
    coeff &= mask;
    r     |= coeff;

    r
}

/// No overflow/underflow checks
/// No checking for coefficient == 10^34 (rounding artifact)
pub (crate) fn bid_get_BID128_very_fast(pres: &mut BID_UINT128, sgn: BID_UINT64, expon: i32, coeff: &BID_UINT128) -> BID_UINT128 {
    let mut tmp: BID_UINT64;

    pres.w[0] = coeff.w[0];
    tmp       = expon as BID_UINT64;
    tmp     <<= 49;
    pres.w[1] = sgn | tmp | coeff.w[1];

    *pres
}

//////////////////////////////////////////////
//  Status Flag Handling
/////////////////////////////////////////////
#[inline(always)]
pub (crate) fn __set_status_flags(fpsc: &mut _IDEC_flags, status: _IDEC_flags) {
    *fpsc |= status;
}

#[inline(always)]
pub (crate) fn is_inexact(fpsc: _IDEC_flags) -> bool{
    fpsc & StatusFlags::BID_INEXACT_EXCEPTION == StatusFlags::BID_INEXACT_EXCEPTION
}

//////////////////////////////////////////////
// Logical Shift Macros
//////////////////////////////////////////////
pub (crate) fn __shr_128(A: &BID_UINT128, k: i32) -> BID_UINT128 {
    let mut Q: BID_UINT128 = BID_UINT128::default();

    Q.w[0]  = A.w[0] >> k;
    Q.w[0] |= A.w[1] << (64 - k);
    Q.w[1]  = A.w[1] >> k;

    Q
}

pub (crate) fn __shr_128_long(A: &BID_UINT128, k: i32) -> BID_UINT128 {
    let mut Q: BID_UINT128 = BID_UINT128::default();

    if k < 64 {
        Q.w[0]  = A.w[0] >> k;
        Q.w[0] |= A.w[1] << (64 - k);
        Q.w[1]  = A.w[1] >> k;
    } else {
        Q.w[0] = A.w[1] >> ((k) - 64);
        Q.w[1] = 0;
    }

    Q
}

pub (crate) fn __shl_128_long(A: &BID_UINT128, k: i32) -> BID_UINT128 {
    let mut Q: BID_UINT128 = BID_UINT128::default();

    if k < 64 {
        Q.w[1]  = A.w[1] << k;
        Q.w[1] |= A.w[0] >> (64 - k);
        Q.w[0]  = A.w[0] << k;
    }  else {
        Q.w[1] = A.w[0] << ((k) - 64);
        Q.w[0] = 0;
    }

    Q
}

//////////////////////////////////////////////
// Add/Subtract Macros
//////////////////////////////////////////////

/// add 64-bit value to 128-bit
pub (crate) fn __add_128_64(A128: &BID_UINT128, B64: BID_UINT64) -> BID_UINT128 {
    let mut R64H: BID_UINT64  = A128.w[1];
    let mut R128: BID_UINT128 = BID_UINT128::default();
    R128.w[0] = BID_UINT64::wrapping_add(B64, A128.w[0]);
    if R128.w[0] < B64 {
        R64H += 1;
    }
    R128.w[1] = R64H;
    R128
}

/// subtract 64-bit value from 128-bi
pub (crate) fn __sub_128_64(A128: &BID_UINT128, B64: BID_UINT64) -> BID_UINT128 {
    let mut R64H: BID_UINT64 = A128.w[1];
    let mut R128: BID_UINT128 = BID_UINT128::default();
    if A128.w[0] < B64 {
        R64H -= 1;
    }
    R128.w[1] = R64H;
    R128.w[0] = A128.w[0] - B64;
    R128
}

// add 128-bit value to 128-bit - assume no carry-out
pub (crate) fn __add_128_128(A128: &BID_UINT128, B128: &BID_UINT128) -> BID_UINT128 {
    let mut Q128: BID_UINT128 = BID_UINT128::default();
    let mut R128: BID_UINT128 = BID_UINT128::default();
    Q128.w[1] = A128.w[1].wrapping_add(B128.w[1]);
    Q128.w[0] = B128.w[0].wrapping_add(A128.w[0]);
    if Q128.w[0] < B128.w[0] {
        Q128.w[1] += 1;
    }
    R128.w[1] = Q128.w[1];
    R128.w[0] = Q128.w[0];

    R128
}

pub (crate) fn __sub_128_128(A128: &BID_UINT128, B128: &BID_UINT128) -> BID_UINT128 {
    let mut Q128: BID_UINT128 = BID_UINT128::default();
    let mut R128: BID_UINT128 = BID_UINT128::default();
    Q128.w[1] = A128.w[1] - B128.w[1];
    Q128.w[0] = A128.w[0] - B128.w[0];
    if A128.w[0] < B128.w[0] {
        Q128.w[1] -= 1;
    }
    R128.w[1] = Q128.w[1];
    R128.w[0] = Q128.w[0];

    R128
}

/// Returns (sum, carry)
pub (crate) fn __add_carry_out(X: BID_UINT64, Y: BID_UINT64) -> (BID_UINT64, BID_UINT64) {
    let S         = BID_UINT64::wrapping_add(X, Y);
    let CY  = if S < X { 1 } else { 0 };
    (S, CY)
}

/// Returns (sum, carry)
pub (crate) fn __add_carry_in_out(X: BID_UINT64, Y: BID_UINT64, CI: BID_UINT64) -> (BID_UINT64, BID_UINT64) {
    let X1: BID_UINT64 = X + CI;
    let S: BID_UINT64  = BID_UINT64::wrapping_add(X1, Y);
    let CY: BID_UINT64 = if S < X1 || X1 < CI { 1u64 } else { 0 };
    (S, CY)
}

//////////////////////////////////////////////
// Multiply Macros
//////////////////////////////////////////////

pub (crate) fn __mul_64x64_to_64(CX: BID_UINT64, CY: BID_UINT64) -> BID_UINT64 {
    CX * CY
}

// ///  Signed, Fu64 64x64-bit Multiply
// pub (crate) fn __imul_64x64_to_128(CX: BID_UINT64, CY: BID_UINT64) -> BID_UINT128 {
//     let mut SX: BID_UINT64;
//     let mut SY: BID_UINT64;
//     let P: BID_UINT128 = __mul_64x64_to_128(P, CX, CY);

//     SX  = ((CX as BID_SINT64)) >> 63;
//     SY  = ((CY as BID_SINT64)) >> 63;
//     SX &= CY;
//     SY &= CX;

//     P.w[1] = P.w[1] - SX - SY;

//     P
// }

// ///  Signed, Fu64 64x128-bit Multiply
// pub (crate) __imul_64x128_full(Ph, Ql, A, B) -> BID_UINT64 {
//     BID_UINT128 ALBL, ALBH, QM2, QM;

//     let ALBH: BID_UINT128 = __imul_64x64_to_128(A, B.w[1]);
//     let ALBL: BID_UINT128 = __imul_64x64_to_128(A, B.w[0]);
//     Ql.w[0] = ALBL.w[0];
//     QM.w[0] = ALBL.w[1];
//     QM.w[1] = ((BID_SINT64)ALBL.w[1]) >> 63;
//     __add_128_128(QM2, ALBH, QM);
//     (Ql).w[1] = QM2.w[0];
//     Ph = QM2.w[1];
// }

/*****************************************************
 *      Unsigned Multiply Macros
 *****************************************************/

/// get fu64 64x64bit product
pub (crate) fn __mul_64x64_to_128(CX: BID_UINT64, CY: BID_UINT64) -> BID_UINT128 {
    let CXH: BID_UINT64;
    let CXL: BID_UINT64;
    let CYH: BID_UINT64;
    let CYL: BID_UINT64;
    let PL: BID_UINT64;
    let mut PH: BID_UINT64;
    let mut PM: BID_UINT64;
    let PM2: BID_UINT64;
    let mut P: BID_UINT128 = BID_UINT128::default();

    CXH = CX >> 32;
    CXL = (CX as BID_UINT32) as BID_UINT64;
    CYH = CY >> 32;
    CYL = (CY as BID_UINT32) as BID_UINT64;

    PM  = CXH * CYL;
    PH  = CXH * CYH;
    PL  = CXL * CYL;
    PM2 = CXL * CYH;
    PH += PM >> 32;
    PM  = ((PM as BID_UINT32) as BID_UINT64) + PM2 + (PL >> 32) as BID_UINT64;

    P.w[1] = PH + (PM >> 32);
    P.w[0] = (PM << 32) + ((PL as BID_UINT32) as BID_UINT64);

    P
}

/// get fu64 64x64bit product
/// Note: This macro is used for CX < 2^61, CY < 2^61
pub (crate) fn __mul_64x64_to_128_fast(CX: BID_UINT64, CY: BID_UINT64) -> BID_UINT128 {
    let CXH: BID_UINT64;
    let CXL: BID_UINT64;
    let CYH: BID_UINT64;
    let CYL: BID_UINT64;
    let PL: BID_UINT64;
    let PH: BID_UINT64;
    let mut PM: BID_UINT64;
    let mut P: BID_UINT128 = BID_UINT128::default();

    CXH = CX >> 32;
    CXL = (CX as BID_UINT32) as BID_UINT64;
    CYH = CY >> 32;
    CYL = (CY as BID_UINT32) as BID_UINT64;

    PM  = CXH * CYL;
    PL  = CXL * CYL;
    PH  = CXH * CYH;
    PM += CXL * CYH;
    PM += PL >> 32;

    P.w[1] = PH + (PM >> 32);
    P.w[0] = (PM << 32) + ((PL as BID_UINT32) as BID_UINT64);

    P
}

/// get fu64 64x64bit product
pub (crate) fn __mul_64x64_to_128_full(CX: BID_UINT64, CY: BID_UINT64) -> BID_UINT128 {
    let CXH:BID_UINT64;
    let CXL:BID_UINT64;
    let CYH:BID_UINT64;
    let CYL:BID_UINT64;
    let PL:BID_UINT64;
    let mut PH:BID_UINT64;
    let mut PM:BID_UINT64;
    let PM2:BID_UINT64;
    let mut P: BID_UINT128 = BID_UINT128::default();

    CXH = CX >> 32;
    CXL = (CX as BID_UINT32) as BID_UINT64;
    CYH = CY >> 32;
    CYL = (CY as BID_UINT32) as BID_UINT64;

    PM  = CXH * CYL;
    PH  = CXH * CYH;
    PL  = CXL * CYL;
    PM2 = CXL * CYH;
    PH += PM >> 32;
    PM  = (PM as BID_UINT32) as BID_UINT64 + PM2 + (PL >> 32);

    P.w[1] = PH + (PM >> 32);
    P.w[0] = (PM << 32) + ((PL as BID_UINT32) as BID_UINT64);

    P
}

pub (crate) fn __mul_128x128_high(A: &BID_UINT128, B: &BID_UINT128) -> BID_UINT128 {
    let ALBH: BID_UINT128 = __mul_64x64_to_128(A.w[0], B.w[1]);
    let AHBL: BID_UINT128 = __mul_64x64_to_128(B.w[0], A.w[1]);
    let ALBL: BID_UINT128 = __mul_64x64_to_128(A.w[0], B.w[0]);
    let AHBH: BID_UINT128 = __mul_64x64_to_128(A.w[1], B.w[1]);
    let QM: BID_UINT128   = __add_128_128(&ALBH, &AHBL);
    let QM2: BID_UINT128  = __add_128_64(&QM, ALBL.w[1]);
    let Q: BID_UINT128    = __add_128_64(&AHBH, QM2.w[1]);

    Q
}

pub (crate) fn __mul_128x128_full(A: &BID_UINT128, B: &BID_UINT128) -> (BID_UINT128, BID_UINT128) {
	let ALBH: BID_UINT128   = __mul_64x64_to_128(A.w[0], B.w[1]);
	let AHBL: BID_UINT128   = __mul_64x64_to_128(B.w[0], A.w[1]);
	let ALBL: BID_UINT128   = __mul_64x64_to_128(A.w[0], B.w[0]);
	let AHBH: BID_UINT128   = __mul_64x64_to_128(A.w[1], B.w[1]);
    let QM: BID_UINT128     = __add_128_128(&ALBH, &AHBL);
    let mut Ql: BID_UINT128 = __add_128_128(&ALBH, &AHBL);
	Ql.w[0] = ALBL.w[0];
    let QM2: BID_UINT128    = __add_128_64(&QM, ALBL.w[1]);
    let Qh: BID_UINT128     = __add_128_64(&AHBH, QM2.w[1]);
	Ql.w[1] = QM2.w[0];

    (Qh, Ql)
}

pub (crate) fn __mul_128x128_low(A: &BID_UINT128, B: &BID_UINT128) -> BID_UINT128 {
    let mut Ql: BID_UINT128 = BID_UINT128::default();
    let ALBL: BID_UINT128   = __mul_64x64_to_128(A.w[0], B.w[0]);
    let QM64: BID_UINT64    = B.w[0] * A.w[1] + A.w[0] * B.w[1];

    Ql.w[0] = ALBL.w[0];
    Ql.w[1] = QM64 + ALBL.w[1];

    Ql
}

pub (crate) fn __mul_64x128_low(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT128 {
    let mut Ql: BID_UINT128 = BID_UINT128::default();
    let ALBH: BID_UINT128   = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128   = __mul_64x64_to_128(A, B.w[0]);
    Ql.w[0] = ALBL.w[0];
    let QM2: BID_UINT128    = __add_128_64(&ALBH, ALBL.w[1]);
    Ql.w[1] = QM2.w[0];

    Ql
}

pub (crate) fn __mul_64x128_full(A: BID_UINT64, B: &BID_UINT128) -> (BID_UINT64, BID_UINT128) {
    let mut Ql: BID_UINT128 = BID_UINT128::default();
    let ALBH: BID_UINT128   = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128   = __mul_64x64_to_128(A, B.w[0]);
    let Ph: BID_UINT64;

    Ql.w[0] = ALBL.w[0];
    let QM2: BID_UINT128 = __add_128_64(&ALBH, ALBL.w[1]);
    Ql.w[1] = QM2.w[0];
    Ph      = QM2.w[1];

    (Ph, Ql)
}

pub (crate) fn __mul_64x128_to_192(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT192 {
    let mut Q: BID_UINT192 = BID_UINT192::default();
    let ALBH: BID_UINT128  = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128  = __mul_64x64_to_128(A, B.w[0]);

    Q.w[0] = ALBL.w[0];
    let QM2: BID_UINT128 = __add_128_64(&ALBH, ALBL.w[1]);
    Q.w[1] = QM2.w[0];
    Q.w[2] = QM2.w[1];

    Q
}

pub (crate) fn __mul_64x128_to192(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT192 {
    let mut Q: BID_UINT192    = BID_UINT192::default();
    let ALBH: BID_UINT128 = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128 = __mul_64x64_to_128(A, B.w[0]);

    Q.w[0] = ALBL.w[0];
    let QM2: BID_UINT128  = __add_128_64(&ALBH, ALBL.w[1]);
    Q.w[1] = QM2.w[0];
    Q.w[2] = QM2.w[1];

    Q
}

pub (crate) fn __mul_128x128_to_256(A: &BID_UINT128, B: &BID_UINT128) -> BID_UINT256 {
    let CY1: BID_UINT64;
    let CY2: BID_UINT64;
    let (Phl, Qll) = __mul_64x128_full(A.w[0], B);
    let (Phh, Qlh) = __mul_64x128_full(A.w[1], B);
    let mut P256: BID_UINT256 = BID_UINT256::default();

    P256.w[0] = Qll.w[0];
    (P256.w[1], CY1) = __add_carry_out(Qlh.w[0], Qll.w[1]);
    (P256.w[2], CY2) = __add_carry_in_out(Qlh.w[1], Phl, CY1);
    P256.w[3] = Phh + CY2;

    P256
}

pub (crate) fn __mul_64x64_to_128MACH(CX64: BID_UINT64, CY64: BID_UINT64) -> BID_UINT128 {
    let CXH: BID_UINT64;
    let CXL: BID_UINT64;
    let CYH: BID_UINT64;
    let CYL: BID_UINT64;
    let PL: BID_UINT64;
    let mut PH: BID_UINT64;
    let mut PM: BID_UINT64;
    let PM2: BID_UINT64;

    CXH = CX64 >> 32;
    CXL = (CX64 as BID_UINT32) as BID_UINT64;
    CYH = CY64 >> 32;
    CYL = (CY64 as BID_UINT32) as BID_UINT64;
    PM  = CXH * CYL;
    PH  = CXH * CYH;
    PL  = CXL * CYL;
    PM2 = CXL * CYH;
    PH += PM >> 32;
    PM  = ((PM as BID_UINT32) as BID_UINT64 + PM2 + (PL >> 32)) as BID_UINT64;

    let mut P128 = BID_UINT128::default();
    P128.w[1] = PH + (PM >> 32);
    P128.w[0] = (PM << 32) + ((PL as BID_UINT32) as BID_UINT64);

    P128
}

// 64x64-bit product
pub (crate) fn __mul_64x64_to_128HIGH(CX64: BID_UINT64, CY64: BID_UINT64) -> BID_UINT64 {
    let CXH: BID_UINT64;
    let CXL: BID_UINT64;
    let CYH: BID_UINT64;
    let CYL: BID_UINT64;
    let PL: BID_UINT64;
    let mut PH: BID_UINT64;
    let mut PM: BID_UINT64;
    let PM2: BID_UINT64;
    let P64: BID_UINT64;

    CXH = CX64 >> 32;
    CXL = (CX64 as BID_UINT32) as BID_UINT64;
    CYH = CY64 >> 32;
    CYL = (CY64 as BID_UINT32) as BID_UINT64;
    PM  = CXH*CYL;
    PH  = CXH*CYH;
    PL  = CXL*CYL;
    PM2 = CXL*CYH;
    PH += PM >> 32;
    PM  = ((PM as BID_UINT32) as BID_UINT64 + PM2 + (PL >> 32)) as BID_UINT64;
    P64 = PH + (PM >> 32);
    P64
}

pub (crate) fn __mul_64x192_to_256(lA: BID_UINT64, lB: &BID_UINT192) -> BID_UINT256 {
    let mut lC: BID_UINT64;
	let mut lP: BID_UINT256 = BID_UINT256::default();
	let lP0: BID_UINT128 = __mul_64x64_to_128(lA, lB.w[0]);
	let lP1: BID_UINT128 = __mul_64x64_to_128(lA, lB.w[1]);
	let lP2: BID_UINT128 = __mul_64x64_to_128(lA, lB.w[2]);
	lP.w[0] = lP0.w[0];
	(lP.w[1], lC) = __add_carry_out(lP1.w[0], lP0.w[1]);
	(lP.w[2], lC) = __add_carry_in_out(lP2.w[0],lP1.w[1], lC);
	lP.w[3] = lP2.w[1] + lC;

	lP
}

pub (crate) fn __mul_128x64_to_128(A64: BID_UINT64, B128: &BID_UINT128) -> BID_UINT128 {
    let ALBH_L: BID_UINT64    = A64 * B128.w[1];
    let mut Q128: BID_UINT128 = __mul_64x64_to_128MACH(A64, B128.w[0]);
    Q128.w[1] += ALBH_L;
    Q128
}

/// might simplify by calculating just QM2.w[0]
pub (crate) fn __mul_64x128_to_128(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT128 {
    let mut Ql: BID_UINT128 = BID_UINT128::default();
    let ALBH = __mul_64x64_to_128(A, B.w[1]);
    let ALBL = __mul_64x64_to_128(A, B.w[0]);
    Ql.w[0] = ALBL.w[0];
    let QM2 = __add_128_64(&ALBH, ALBL.w[1]);
    Ql.w[1] = QM2.w[0];

    Ql
}

pub (crate) fn __mul_64x256_to_320(A: BID_UINT64, B: &BID_UINT256) -> BID_UINT512 {
    let mut lC: BID_UINT64;
    let mut P: BID_UINT512 = BID_UINT512::default();
	let lP0: BID_UINT128 = __mul_64x64_to_128(A, B.w[0]);
	let lP1: BID_UINT128 = __mul_64x64_to_128(A, B.w[1]);
	let lP2: BID_UINT128 = __mul_64x64_to_128(A, B.w[2]);
	let lP3: BID_UINT128 = __mul_64x64_to_128(A, B.w[3]);
	P.w[0] = lP0.w[0];
	(P.w[1], lC) = __add_carry_out(lP1.w[0],lP0.w[1]);
	(P.w[2], lC) = __add_carry_in_out(lP2.w[0],lP1.w[1], lC);
	(P.w[3], lC) = __add_carry_in_out(lP3.w[0],lP2.w[1], lC);
	P.w[4] = lP3.w[1] + lC;

	P
}

pub (crate) fn __mul_192x192_to_384(A: &BID_UINT192, B: &BID_UINT192) -> BID_UINT384
{
    let mut CY: BID_UINT64;
	let mut P: BID_UINT384  = BID_UINT384::default();
	let P0: BID_UINT256 = __mul_64x192_to_256(A.w[0], B);
	let P1: BID_UINT256 = __mul_64x192_to_256(A.w[1], B);
	let P2: BID_UINT256 = __mul_64x192_to_256(A.w[2], B);
	P.w[0] = P0.w[0];
	(P.w[1], CY) = __add_carry_out(P1.w[0], P0.w[1]);
	(P.w[2], CY) = __add_carry_in_out(P1.w[1], P0.w[2], CY);
	(P.w[3], CY) = __add_carry_in_out(P1.w[2], P0.w[3], CY);
	P.w[4] = P1.w[3] + CY;
	(P.w[2], CY) = __add_carry_out(P2.w[0], P.w[2]);
	(P.w[3], CY) = __add_carry_in_out(P2.w[1], P.w[3],CY);
	(P.w[4], CY) = __add_carry_in_out(P2.w[2], P.w[4],CY);
	P.w[5] = P2.w[3] + CY;

	P
}

pub (crate) fn __mul_256x256_to_512(A: &BID_UINT256, B: &BID_UINT256) -> BID_UINT512
{
    let mut CY: BID_UINT64;
    let mut P: BID_UINT512 = BID_UINT512::default();
	let P0: BID_UINT512 = __mul_64x256_to_320(A.w[0], B);
	let P1: BID_UINT512 = __mul_64x256_to_320(A.w[1], B);
	let P2: BID_UINT512 = __mul_64x256_to_320(A.w[2], B);
	let P3: BID_UINT512 = __mul_64x256_to_320(A.w[3], B);
	P.w[0] = P0.w[0];
	(P.w[1], CY) = __add_carry_out(P1.w[0], P0.w[1]);
	(P.w[2], CY) = __add_carry_in_out(P1.w[1], P0.w[2], CY);
	(P.w[3], CY) = __add_carry_in_out(P1.w[2], P0.w[3], CY);
	(P.w[4], CY) = __add_carry_in_out(P1.w[3], P0.w[4], CY);
	P.w[5] = P1.w[4] + CY;
	(P.w[2], CY) = __add_carry_out(P2.w[0], P.w[2]);
	(P.w[3], CY) = __add_carry_in_out(P2.w[1], P.w[3], CY);
	(P.w[4], CY) = __add_carry_in_out(P2.w[2], P.w[4], CY);
	(P.w[5], CY) = __add_carry_in_out(P2.w[3], P.w[5], CY);
	P.w[6] = P2.w[4] + CY;
	(P.w[3], CY) = __add_carry_out(P3.w[0], P.w[3]);
	(P.w[4], CY) = __add_carry_in_out(P3.w[1], P.w[4], CY);
	(P.w[5], CY) = __add_carry_in_out(P3.w[2], P.w[5], CY);
	(P.w[6], CY) = __add_carry_in_out(P3.w[3], P.w[6], CY);
	P.w[7] = P3.w[4] + CY;

	P
}

//////////////////////////////////////////////
// Compare Macros
//////////////////////////////////////////////

/// greater than
///  return 0 if A<=B
///  non-zero if A>B
pub (crate) fn __unsigned_compare_gt_128(A: BID_UINT128, B: BID_UINT128) -> bool  {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] > B.w[0]))
}

/// greater-or-equal
pub (crate) fn __unsigned_compare_ge_128(A: BID_UINT128, B: BID_UINT128) -> bool {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] >= B.w[0]))
}

pub (crate) fn __test_equal_128(A: BID_UINT128, B: BID_UINT128) -> bool {
    (A.w[1] == B.w[1]) && (A.w[0] == B.w[0])
}
