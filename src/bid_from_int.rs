/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

use crate::bid_conf::{BID_HIGH_128W, BID_LOW_128W};
use crate::constants::{SIGNMASK32, SIGNMASK64};
use crate::d128::{BID_UINT128, BID_UINT64};

pub (crate) fn bid128_from_int32(x: i32) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::default();

    // if integer is negative, use the absolute value
    if (x & (SIGNMASK32 as i32)) == (SIGNMASK32 as i32)  {
        res.w[BID_HIGH_128W] = 0xb040000000000000u64;
        res.w[BID_LOW_128W]  = !(x as u64) + 1;	// 2's complement of x
    } else {
        res.w[BID_HIGH_128W] = 0x3040000000000000u64;
        res.w[BID_LOW_128W]  = x as u64;
    }

    res
}

pub (crate) fn bid128_from_uint32(x: u32) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::default();

    res.w[BID_HIGH_128W] = 0x3040000000000000u64;
    res.w[BID_LOW_128W]  = x as u64;

    res
}

pub (crate) fn bid128_from_int64(x: i64) -> BID_UINT128 {
    let mut res: BID_UINT128 = BID_UINT128::default();

    // if integer is negative, use the absolute value
    if (x & (SIGNMASK64 as i64)) == (SIGNMASK64 as i64) {
        res.w[BID_HIGH_128W] = 0xb040000000000000u64;
        res.w[BID_LOW_128W]  = (!x + 1) as BID_UINT64;	// 2's complement of x
    } else {
        res.w[BID_HIGH_128W] = 0x3040000000000000u64;
        res.w[BID_LOW_128W]  = x as BID_UINT64;
    }

    res
}

pub (crate) fn bid128_from_uint64(x: u64) -> BID_UINT128 {
    let mut res : BID_UINT128 = BID_UINT128::default();

    res.w[BID_HIGH_128W] = 0x3040000000000000u64;
    res.w[BID_LOW_128W]  = x;

    res
}
