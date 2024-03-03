/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid_b2d::{BID_B2D, BID_D2B};
use crate::bid_internal::{__add_128_64, __mul_128x128_high, __mul_64x128_full, __mul_64x64_to_128, __sub_128_128, BID_HIGH_128W, BID_LOW_128W, BID_UINT128, BID_UINT32, BID_UINT64};

/// Convert a 128-bit decimal floating-point value encoded in BID format
/// to the same value encoded in DPD
pub (crate) fn bid_to_dpd128(ba: &BID_UINT128) -> BID_UINT128 {
    let mut res: BID_UINT128 = Default::default();
    let mut sign: BID_UINT128 = Default::default();
    let comb: BID_UINT32;
    let exp: BID_UINT32;
    let mut trailing: BID_UINT128 = Default::default();
    let d0: BID_UINT128;
    let d1: BID_UINT128;
    let d2: BID_UINT128;
    let d3: BID_UINT128;
    let d4: BID_UINT128;
    let d5: BID_UINT128;
    let d6: BID_UINT128;
    let d7: BID_UINT128;
    let d8: BID_UINT128;
    let d9: BID_UINT128;
    let d10: BID_UINT128;
    let d11: BID_UINT128;
    let mut bcoeff: BID_UINT128 = Default::default();
    let mut dcoeff: BID_UINT128 = Default::default();
    let mut nanb: BID_UINT64 = 0;

    sign.w[1]     = ba.w[BID_HIGH_128W] & 0x8000000000000000u64;
    sign.w[0]     = 0;
    comb          = ((ba.w[BID_HIGH_128W] & 0x7fffc00000000000u64) >> 46) as BID_UINT32;
    trailing.w[1] = ba.w[BID_HIGH_128W] & 0x00003fffffffffffu64;
    trailing.w[0] = ba.w[BID_LOW_128W];
    // exp           = 0;

    if (comb & 0x1f000) == 0x1e000 {	// G0..G4 = 11110 -> Inf
        res.w[BID_HIGH_128W] = ba.w[BID_HIGH_128W] & 0xf800000000000000u64;
        res.w[BID_LOW_128W] = 0;
        return res;
        // Detect NaN, and canonicalize trailing
    } else if (comb & 0x1f000) == 0x1f000 {
        if (trailing.w[1]  > 0x0000314dc6448d93u64)        // significand is non-canonical
        || ((trailing.w[1] == 0x0000314dc6448d93u64)
         && (trailing.w[0] >= 0x38c15b0a00000000u64)) {    // significand is non-canonical
            trailing.w[1] = 0u64;
            trailing.w[0] = 0u64;
        }
        bcoeff.w[1] = trailing.w[1];
        bcoeff.w[0] = trailing.w[0];
        nanb = ba.w[BID_HIGH_128W] & 0xfe00000000000000u64;
        exp = 0;
    } else {	// Normal number
        if (comb & 0x18000) == 0x18000 {	// G0..G1 = 11 -> exp is G2..G11
            exp         = (comb >> 1) & 0x3fff;
            bcoeff.w[1] = (((8 + (comb & 1)) as u64) << 46u64) | trailing.w[1];
            bcoeff.w[0] = trailing.w[0];
        } else {
            exp         = (comb >> 3) & 0x3fff;
            bcoeff.w[1] = (((comb & 7) as u64) << 46u64) | trailing.w[1];
            bcoeff.w[0] = trailing.w[0];
        }
        // Zero the coefficient if non-canonical (>= 10^34)
        if bcoeff.w[1]  > 0x1ed09bead87c0u64
        || (bcoeff.w[1] == 0x1ed09bead87c0u64
         && bcoeff.w[0] >= 0x378D8E6400000000u64) {
            bcoeff.w[0] = 0;
            bcoeff.w[1] = 0;
        }
    }
    // Constant 2^128 / 1000 + 1
    {
        let mut t: BID_UINT128;
        let mut _t2: BID_UINT64;
        let mut d1000: BID_UINT128 = Default::default();
        let b11: BID_UINT128;
        let b10: BID_UINT128;
        let b9: BID_UINT128;
        let b8: BID_UINT128;
        let b7: BID_UINT128;
        let b6: BID_UINT128;
        let b5: BID_UINT128;
        let b4: BID_UINT128;
        let b3: BID_UINT128;
        let b2: BID_UINT128;
        let b1: BID_UINT128;

        d1000.w[1] = 0x4189374BC6A7EFu64;
        d1000.w[0] = 0x9DB22D0E56041894u64;

        b11 = __mul_128x128_high(&bcoeff, &d1000);
        b10 = __mul_128x128_high(&b11, &d1000);
        b9  = __mul_128x128_high(&b10, &d1000);
        b8  = __mul_128x128_high(&b9, &d1000);
        b7  = __mul_128x128_high(&b8, &d1000);
        b6  = __mul_128x128_high(&b7, &d1000);
        b5  = __mul_128x128_high(&b6, &d1000);
        b4  = __mul_128x128_high(&b5, &d1000);
        b3  = __mul_128x128_high(&b4, &d1000);
        b2  = __mul_128x128_high(&b3, &d1000);
        b1  = __mul_128x128_high(&b2, &d1000);

        (_t2, t) = __mul_64x128_full(1000u64, &b11);
        d11      = __sub_128_128(&bcoeff, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b10);
        d10      = __sub_128_128(&b11, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b9);
        d9       = __sub_128_128(&b10, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b8);
        d8       = __sub_128_128(&b9, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b7);
        d7       = __sub_128_128(&b8, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b6);
        d6       = __sub_128_128(&b7, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b5);
        d5       = __sub_128_128(&b6, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b4);
        d4       = __sub_128_128(&b5, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b3);
        d3       = __sub_128_128(&b4, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b2);
        d2       = __sub_128_128(&b3, &t);
        (_t2, t) = __mul_64x128_full(1000u64, &b1);
        d1       = __sub_128_128(&b2, &t);
        d0       = b1;
     }

    dcoeff.w[0] = BID_B2D[d11.w[0] as usize]
               | (BID_B2D[d10.w[0] as usize] << 10)
               | (BID_B2D[d9.w[0] as usize] << 20)
               | (BID_B2D[d8.w[0] as usize] << 30)
               | (BID_B2D[d7.w[0] as usize] << 40)
               | (BID_B2D[d6.w[0] as usize] << 50)
               | (BID_B2D[d5.w[0] as usize] << 60);
    dcoeff.w[1] = (BID_B2D[d5.w[0] as usize] >> 4)
                | (BID_B2D[d4.w[0] as usize] << 6)
                | (BID_B2D[d3.w[0] as usize] << 16)
                | (BID_B2D[d2.w[0] as usize] << 26)
                | (BID_B2D[d1.w[0] as usize] << 36);

    res.w[0] = dcoeff.w[0];
    if d0.w[0] >= 8 {
        res.w[1] = sign.w[1] | (((0x18000 | ((exp >> 12) << 13) | ((d0.w[0] & 1) << 12) as u32 | (exp & 0xfff)) as u64) << 46) | dcoeff.w[1];
    } else {
        res.w[1] = sign.w[1] | (((((exp >> 12) << 15) | (d0.w[0] << 12) as u32 | (exp & 0xfff)) as u64) << 46) | dcoeff.w[1];
    }

    res.w[1] |= nanb;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&res);

    res
}

/// Convert a 128-bit decimal floating-point value encoded in DPD format
/// to the same value encoded in BID format
pub (crate) fn bid_dpd_to_bid128(da: &BID_UINT128) -> BID_UINT128 {
    let mut res: BID_UINT128 = Default::default();
    let mut sign: BID_UINT128 = Default::default();
    let mut exp: BID_UINT64;
    let comb: BID_UINT64;
    let mut trailing: BID_UINT128 = Default::default();
    let d0: BID_UINT64;
    let d1: BID_UINT64;
    let d2: BID_UINT64;
    let d3: BID_UINT64;
    let d4: BID_UINT64;
    let d5: BID_UINT64;
    let d6: BID_UINT64;
    let d7: BID_UINT64;
    let d8: BID_UINT64;
    let d9: BID_UINT64;
    let d10: BID_UINT64;
    let d11: BID_UINT64;
    let mut bcoeff: BID_UINT128;
    let tl: BID_UINT64;
    let th: BID_UINT64;
    let mut nanb: BID_UINT64 = 0;
    let da: BID_UINT128 = *da;

    sign.w[1]     = da.w[BID_HIGH_128W] & 0x8000000000000000u64;
    sign.w[0]     = 0;
    comb          = (da.w[BID_HIGH_128W] & 0x7fffc00000000000u64) >> 46;
    trailing.w[1] = da.w[BID_HIGH_128W] & 0x00003fffffffffffu64;
    trailing.w[0] = da.w[BID_LOW_128W];
    // exp           = 0;

    if (comb & 0x1f000) == 0x1e000 {	// G0..G4 = 11110 -> Inf
        res.w[BID_HIGH_128W] = da.w[BID_HIGH_128W] & 0xf800000000000000u64;
        res.w[BID_LOW_128W] = 0u64;
        return res;
    } else if (comb & 0x1f000) == 0x1f000 {	// G0..G4 = 11111 -> NaN
        nanb = da.w[BID_HIGH_128W] & 0xfe00000000000000u64;
        exp  = 0;
        d0   = 0;
    } else {	// Normal number
        if (comb & 0x18000) == 0x18000 {	// G0..G1 = 11 -> d0 = 8 + G4
            d0  = 8 + if (comb & 0x01000) == 0x01000 { 1 } else { 0 };
            exp = if (comb & 0x04000) == 0x04000 { 1 } else { 0 } * 0x2000
                + if (comb & 0x02000) == 0x02000 { 1 } else { 0 } * 0x1000;
            // exp leading bits are G2..G3
        } else {
            d0 = 4 * if (comb & 0x04000) == 0x04000 { 1 } else { 0 }
               + 2 * if (comb & 0x2000)  == 0x2000 { 1 } else { 0 }
               +     if (comb & 0x1000)  == 0x1000 { 1 } else { 0 };
            exp = if (comb & 0x10000) == 0x10000 { 1 } else { 0 } * 0x2000
                + if (comb & 0x08000) == 0x08000 { 1 } else { 0 } * 0x1000;
            // exp loading bits are G0..G1
        }
    }

    d11 = BID_D2B[((trailing.w[0]) & 0x3ff) as usize];
    d10 = BID_D2B[((trailing.w[0] >> 10) & 0x3ff) as usize];
    d9  = BID_D2B[((trailing.w[0] >> 20) & 0x3ff) as usize];
    d8  = BID_D2B[((trailing.w[0] >> 30) & 0x3ff) as usize];
    d7  = BID_D2B[((trailing.w[0] >> 40) & 0x3ff) as usize];
    d6  = BID_D2B[((trailing.w[0] >> 50) & 0x3ff) as usize];
    d5  = BID_D2B[((trailing.w[0] >> 60) | ((trailing.w[1] & 0x3f) << 4)) as usize];
    d4  = BID_D2B[((trailing.w[1] >> 6) & 0x3ff) as usize];
    d3  = BID_D2B[((trailing.w[1] >> 16) & 0x3ff) as usize];
    d2  = BID_D2B[((trailing.w[1] >> 26) & 0x3ff) as usize];
    d1  = BID_D2B[((trailing.w[1] >> 36) & 0x3ff) as usize];

    tl = d11 + (d10 * 1000u64)   + (d9 * 1000000u64) + (d8 * 1000000000u64)
       + (d7 * 1000000000000u64) + (d6 * 1000000000000000u64);
    th = d5  + (d4 * 1000u64)    + (d3 * 1000000u64) + (d2 * 1000000000u64)
       + (d1 * 1000000000000u64) + (d0 * 1000000000000000u64);

    bcoeff = __mul_64x64_to_128(th, 1000000000000000000u64);
    bcoeff = __add_128_64(&bcoeff, tl);

    if nanb == 0 {
        exp += comb & 0xfff;
    }

    res.w[0] = bcoeff.w[0];
    res.w[1] = (exp << 49) | sign.w[1] | bcoeff.w[1];

    res.w[1] |= nanb;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&res);

    res
}