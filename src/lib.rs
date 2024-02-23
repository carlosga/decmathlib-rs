/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */

#![allow(clippy::needless_late_init)]
#![allow(clippy::redundant_guards)]

mod bid64_to_bid128;
mod bid128;
mod bid128_2_str_tables;
mod bid128_2_str_macros;
mod bid128_add;
mod bid128_compare;
mod bid128_div;
mod bid128_fdim;
mod bid128_fma;
mod bid128_fmod;
mod bid128_frexp;
mod bid128_ilogb;
mod bid128_ldexp;
mod bid128_llquantexp;
mod bid128_llrint;
mod bid128_llround;
mod bid128_logb;
mod bid128_lrint;
mod bid128_lround;
mod bid128_minmax;
mod bid128_modf;
mod bid128_mul;
mod bid128_nearbyint;
mod bid128_next;
mod bid128_nexttoward;
mod bid128_noncomp;
mod bid128_quantexp;
mod bid128_quantize;
mod bid128_quantum;
mod bid128_rem;
mod bid128_round_integral;
mod bid128_scalbln;
mod bid128_scalbn;
mod bid128_sqrt;
mod bid128_string;
mod bid128_to_int32;
mod bid128_to_int64;
mod bid128_to_uint32;
mod bid128_to_uint64;
mod bid_conf;
mod bid_convert_data;
mod bid_decimal_data;
mod bid_div_macros;
mod bid_from_int;
mod bid_internal;
mod bid_round;
mod bid_sqrt_macros;
mod constants;
pub mod d64;
pub mod d128;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "sqlx")]
pub mod sqlx;