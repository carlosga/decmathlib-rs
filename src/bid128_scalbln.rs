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
#![allow(overflowing_literals)]

use crate::bid128_scalbn::bid128_scalbn;
use crate::d128::{_IDEC_flags, BID_UINT128};

pub (crate) fn bid128_scalbln(x: &BID_UINT128, n: i64, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut n1: i32 = n as i32;
    n1 = match n1 as i64 {
        val if val < n => 0x7fffffffi32,
        val if val > n => 0x80000000i32,
        _ => n1
    };

    bid128_scalbn(x, n1, rnd_mode, pfpsf)
}