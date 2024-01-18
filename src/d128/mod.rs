/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez         */
/* --------------------------------------------------------------------- */

pub mod dec128;
mod data;
mod constants;
mod bid_internal;
mod bid128;
mod bid_conf;
mod bid128_noncomp;
mod bid128_mul;