/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* Ported to rust-lang by Carlos Guzmán Álvarez                          */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */
/* Original C source code Copyright (c) 2018, Intel Corp.                */
/* --------------------------------------------------------------------- */

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::d128::constants::*;
use crate::d128::data::bid_power10_table_128;
use crate::d128::dec128::{BID_UINT128, BID_UINT32, BID_UINT64};

///  BID32 unpack, input pased by reference
pub (crate) fn unpack_BID32(psign_x: &mut BID_UINT32, pexponent_x: &mut i32, pcoefficient_x: &mut BID_UINT32, x: BID_UINT32) -> BID_UINT32{
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

    return *pcoefficient_x;
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

    return *pcoefficient_x;
}

///  BID128 unpack, input pased by reference
pub (crate) fn unpack_BID128 (psign_x: &mut BID_UINT64, pexponent_x: &mut i32, pcoefficient_x: &mut BID_UINT128, px: &BID_UINT128) -> BID_UINT64{
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

    return coeff.w[0] | coeff.w[1];
}

/// No overflow/underflow checks
/// No checking for coefficient == 10^34 (rounding artifact)
pub (crate) fn bid_get_BID128_very_fast(pres: &mut BID_UINT128, sgn: BID_UINT64, expon: i32, coeff: &BID_UINT128) -> BID_UINT128 {
    let mut tmp: BID_UINT64;

    pres.w[0] = coeff.w[0];
    tmp       = expon as BID_UINT64;
    tmp     <<= 49;
    pres.w[1] = sgn | tmp | coeff.w[1];

    return *pres;
}

//////////////////////////////////////////////
//  Status Flag Handling
/////////////////////////////////////////////

pub (crate) fn __set_status_flags(fpsc: &mut BID_UINT64, status: BID_UINT64)
{
    *(fpsc) |= status;
}

/*********************************************************************
 *
 *      Multiply Macros
 *
 *********************************************************************/

/*****************************************************
 *      Unsigned Multiply Macros
 *****************************************************/

/// get full 64x64bit product
pub (crate) fn __mul_64x64_to_128(p: &mut BID_UINT128, cx: BID_UINT64, cy: BID_UINT64) {
    let cxh: BID_UINT64;
    let cxl: BID_UINT64;
    let cyh: BID_UINT64;
    let cyl: BID_UINT64;
    let pl: BID_UINT64;
    let mut ph: BID_UINT64;
    let mut pm: BID_UINT64;
    let pm2: BID_UINT64;

    cxh = cx >> 32;
    cxl = (cx as BID_UINT32) as BID_UINT64;
    cyh = cy >> 32;
    cyl = (cy as BID_UINT32) as BID_UINT64;

    pm = cxh * cyl;
    ph = cxh * cyh;
    pl = cxl * cyl;
    pm2 = cxl * cyh;
    ph += pm >> 32;
    pm = ((pm as BID_UINT32) as BID_UINT64) + pm2 + (pl >> 32) as BID_UINT64;

    p.w[1] = ph + (pm >> 32);
    p.w[0] = (pm << 32) + ((pl as BID_UINT32) as BID_UINT64);
}

/*********************************************************************
 *
 *      Compare Macros
 *
 *********************************************************************/
// greater than
//  return 0 if A<=B
//  non-zero if A>B
pub (crate) fn __unsigned_compare_gt_128(A: BID_UINT128, B: BID_UINT128) -> bool  {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] > B.w[0]))
}

// greater-or-equal
pub (crate) fn __unsigned_compare_ge_128(A: BID_UINT128, B: BID_UINT128) -> bool {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] >= B.w[0]))
}

pub (crate) fn __test_equal_128(A: BID_UINT128, B: BID_UINT128) -> bool {
    (A.w[1] == B.w[1]) & & (A.w[0] == B.w[0])
}