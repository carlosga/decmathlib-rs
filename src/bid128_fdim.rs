/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid128_add::bid128_sub;
use crate::bid128_compare::bid128_quiet_greater;
use crate::bid_internal::{MASK_NAN, BID_UINT128, BID_HIGH_128W};
use crate::d128::{_IDEC_flags, RoundingMode};

/// fdim returns x - y if x > y, and +0 is x <= y
/// Exceptions: P, O, I (U could only be unmasked, which is not supported)
pub (crate) fn bid128_fdim(x: &BID_UINT128, y: &BID_UINT128, rnd_mode: RoundingMode, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let tmp_fpsf: _IDEC_flags = *pfpsf; // dummy fpsf for calls to comparison functions
    let cmpres: bool          = bid128_quiet_greater(x, y, pfpsf);

    *pfpsf = tmp_fpsf;    // restore fpsf

    if ((x.w[BID_HIGH_128W] & MASK_NAN) != MASK_NAN) && ((y.w[BID_HIGH_128W] & MASK_NAN) != MASK_NAN) && !cmpres {
        // if x != NaN and y != NaN and x <= y return +0
        return BID_UINT128::new(0x3040000000000000u64, 0x0000000000000000u64);
    }

    // else if x = NaN or y = NaN or x > y return x - y

    bid128_sub(x, y, rnd_mode, pfpsf)
}
