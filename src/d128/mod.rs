/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez         */
/* --------------------------------------------------------------------- */

mod bid128;
mod bid128_fma;
mod bid128_mul;
mod bid128_noncomp;
mod bid_conf;
mod bid_decimal_data;
mod bid_internal;
mod constants;
mod convert;
pub mod dec128;
