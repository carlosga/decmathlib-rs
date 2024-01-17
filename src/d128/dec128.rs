/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* Ported to rust-lang by Carlos Guzmán Álvarez                          */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */
/* Original C source code Copyright (c) 2018, Intel Corp.                */
/* --------------------------------------------------------------------- */

#![allow(non_camel_case_types)]

use crate::d128::constants::*;
use crate::d128::data::bid_power10_table_128;
use crate::d128::internal::{__mul_64x64_to_128, bid_get_BID128_very_fast, unpack_BID64};

pub type BID_UINT32 = u32;

pub type BID_UINT64 = u64;

#[derive(Copy, Clone, Debug, Default)]
pub struct BID_UINT128 {
    pub w: [BID_UINT64; 2]
}

impl BID_UINT128 {
    pub fn new(l: BID_UINT64, h: BID_UINT64) -> Self {
        #[cfg(target_endian = "big")]
        return Self { w: [l, h] };
        #[cfg(target_endian = "little")]
        return Self { w: [h, l] };
    }
}

impl Eq for BID_UINT128 {}

impl PartialEq for BID_UINT128 {
    fn eq(&self, other: &Self) -> bool {
        self.w[BID_HIGH_128W] == other.w[BID_HIGH_128W] && self.w[BID_LOW_128W]  == other.w[BID_LOW_128W]
    }
}

/// Takes a BID64 as input and converts it to a BID128 and returns it.
pub fn bid64_to_bid128(x: BID_UINT64) -> BID_UINT128 {
    let mut new_coeff: BID_UINT128    = BID_UINT128::default();
    let mut res: BID_UINT128          = BID_UINT128::default();
    let mut sign_x: BID_UINT64        = 0;
    let mut exponent_x: i32           = 0;
    let mut coefficient_x: BID_UINT64 = 0;

    if unpack_BID64(&mut sign_x, &mut exponent_x, &mut coefficient_x, x) == 0 {
        if ((x) << 1) >= 0xf000000000000000u64 {
            #[cfg(BID_SET_STATUS_FLAGS)]
            if (((x) & SNAN_MASK64) == SNAN_MASK64) {   // sNaN
                __set_status_flags(pfpsf, BID_INVALID_EXCEPTION);
            }
            res.w[0] = coefficient_x & 0x0003ffffffffffffu64;
            let cx = res.w[0];
            __mul_64x64_to_128(&mut res, cx, bid_power10_table_128[18].w[0]);
            res.w[1] |= (coefficient_x) & 0xfc00000000000000u64;
            return res;
        }
    }

    new_coeff.w[0] = coefficient_x;
    new_coeff.w[1] = 0;

    bid_get_BID128_very_fast(
        &mut res, sign_x,
        exponent_x + DECIMAL_EXPONENT_BIAS_128 as i32 - DECIMAL_EXPONENT_BIAS as i32,
        &new_coeff);

    res
} // convert_bid64_to_bid128
