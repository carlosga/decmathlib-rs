/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(non_snake_case)]

#[cfg(target_endian = "big")]
use crate::bid_conf::BID_SWAP128;

use crate::bid128_ilogb::bid128_ilogb;
use crate::bid_conf::{BID_HIGH_128W, BID_LOW_128W};
use crate::bid_internal::{__set_status_flags, unpack_BID128_value};
use crate::constants::QUIET_MASK64;
use crate::core::StatusFlags;
use crate::d128::{_IDEC_flags, BID_UINT128, BID_UINT64};

pub (crate) fn bid128_logb(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut exponent_x: i32 = 0;
    let mut sign_x: BID_UINT64 = 0;
    let mut res: BID_UINT128 = BID_UINT128::default();
    let mut CX: BID_UINT128 = BID_UINT128::default();

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    if unpack_BID128_value(&mut sign_x, &mut exponent_x, &mut CX, &x) == 0 {
        // test if x is NaN/Inf
        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut x);

        if (x.w[BID_HIGH_128W] & 0x7800000000000000u64) == 0x7800000000000000u64 {
            if (x.w[BID_HIGH_128W] & 0x7e00000000000000u64) == 0x7e00000000000000u64 { // sNaN
                __set_status_flags(pfpsf, StatusFlags::BID_INVALID_EXCEPTION);
            }
            res.w[BID_HIGH_128W] = (CX.w[1]) & QUIET_MASK64;
            res.w[BID_LOW_128W]  =  CX.w[0];
            if (x.w[BID_HIGH_128W] & 0x7c00000000000000u64) == 0x7800000000000000u64 {
                res.w[BID_HIGH_128W] &= 0x7fffffffffffffffu64;
            }
            return res;
        }

        // x is 0
        __set_status_flags(pfpsf, StatusFlags::BID_ZERO_DIVIDE_EXCEPTION);
        res.w[BID_HIGH_128W] = 0xf800000000000000u64;
        res.w[BID_LOW_128W]  = 0;
        return res;
    }

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    #[cfg(target_endian = "big")]
    let ires = bid128_ilogb(&x, pfpsf);

    #[cfg(target_endian = "little")]
    let ires = bid128_ilogb(x, pfpsf);

    if (ires & (0x80000000u32 as i32)) == (0x80000000u32 as i32) {
      res.w[BID_HIGH_128W] = 0xb040000000000000u64;
      res.w[BID_LOW_128W]  = -ires as BID_UINT64;
    } else {
      res.w[BID_HIGH_128W] = 0x3040000000000000u64;
      res.w[BID_LOW_128W]  = ires as BID_UINT64;
    }
    res
}