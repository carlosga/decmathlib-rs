/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(overflowing_literals)]

use crate::bid_internal::{BID_UINT128, MASK_SPECIAL, MASK_STEERING_BITS};
use crate::d128::{_IDEC_flags, StatusFlags};

/// The quantexp() functions return the quantum exponent of x.
pub (crate) fn bid128_quantexp(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> i32 {
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        0x80000000
    } else if (x.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS {
        (((x.w[1] >> 47) as i32) & 0x3fff) - 6176
    } else {
        (((x.w[1] >> 49) as i32) & 0x3fff) - 6176
    }
}

