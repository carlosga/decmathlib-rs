/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_snake_case)]

/*
 Exceptions signaled: invalid
 */

use crate::constants::{MASK_SPECIAL, MASK_STEERING_BITS};
use crate::core::StatusFlags;
use crate::d128::{_IDEC_flags, BID_UINT128};

/// The bid128_llquantexp() functions return the quantum exponent of x.
pub (crate) fn bid128_llquantexp(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> i64 {
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        0x80000000
    } else if (x.w[1] & MASK_STEERING_BITS) == MASK_STEERING_BITS {
        ((x.w[1] >> 47) & 0x3fff) as i64 - 6176
    } else {
        ((x.w[1] >> 49) & 0x3fff) as i64 - 6176
    }
}