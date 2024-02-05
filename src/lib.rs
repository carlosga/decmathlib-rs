/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

mod bid128;
mod bid128_2_str_tables;
mod bid128_2_str_macros;
mod bid128_add;
mod bid128_compare;
mod bid128_div;
mod bid128_fma;
mod bid128_ilogb;
mod bid128_ldexp;
mod bid128_logb;
mod bid128_lrint;
mod bid128_lround;
mod bid128_minmax;
mod bid128_mul;
mod bid128_noncomp;
mod bid128_rem;
mod bid128_scalbln;
mod bid128_scalbn;
mod bid128_string;
mod bid128_to_int32;
mod bid128_to_int64;
mod bid128_to_uint32;
mod bid128_to_uint64;
mod bid_conf;
mod bid_convert_data;
mod bid_decimal_data;
mod bid_div_macros;
mod bid_internal;
mod bid_round;
mod constants;
mod convert;
pub mod core;
pub mod d64;
pub mod d128;