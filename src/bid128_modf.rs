/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid128_add::bid128_sub;
use crate::bid128_round_integral::bid128_round_integral_zero;
use crate::bid_conf::{BID_HIGH_128W, BID_LOW_128W};
use crate::core::RoundingMode;
use crate::d128::{_IDEC_flags, BID_UINT128};

/// Decomposes given decimal floating point value num into integral and fractional parts.
pub (crate) fn bid128_modf(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> (BID_UINT128, BID_UINT128) {
    let mut res: BID_UINT128;
    let mut xi: BID_UINT128 = bid128_round_integral_zero(x, pfpsf);

	// check for Infinity
	if (x.w[BID_HIGH_128W] & 0x7c00000000000000u64) == 0x7800000000000000u64 {
	    res = BID_UINT128::default();
		res.w[BID_HIGH_128W]= (x.w[BID_HIGH_128W] & 0x8000000000000000u64)|0x5ffe000000000000u64;
		res.w[BID_LOW_128W] = 0;
	} else {
	    res = bid128_sub(x, &xi, RoundingMode::BID_ROUNDING_TO_NEAREST, pfpsf);
	}

	xi.w[BID_HIGH_128W]  |=  x.w[BID_HIGH_128W] & 0x8000000000000000u64;
	res.w[BID_HIGH_128W] |=  x.w[BID_HIGH_128W] & 0x8000000000000000u64;

	(xi, res)
}