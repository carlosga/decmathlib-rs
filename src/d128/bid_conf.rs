/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* Ported to rust-lang by Carlos Guzmán Álvarez                          */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */
/* Original C source code - Copyright (c) 2018, Intel Corp.              */
/* --------------------------------------------------------------------- */

#[cfg(target_endian = "big")]
pub (crate) fn BID_SWAP128(x: &mut BID_UINT128) {
    let sw: BID_UINT64 = x.w[1];
    x.w[1] = x.w[0];
    x.w[0] = sw;
}