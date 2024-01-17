/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* Ported to rust-lang by Carlos Guzmán Álvarez                          */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */
/* Original C source code Copyright (c) 2018, Intel Corp.                */
/* --------------------------------------------------------------------- */

#![allow(unused)]
#![allow(dead_code)]

#[cfg(target_endian = "big")]
pub (crate) const BID_HIGH_128W: usize = 0;
#[cfg(target_endian = "big")]
pub (crate) const BID_LOW_128W: usize = 1;
#[cfg(target_endian = "little")]
pub (crate) const BID_HIGH_128W: usize = 1;
#[cfg(target_endian = "little")]
pub (crate) const BID_LOW_128W: usize = 0;

// TYPE parameters
pub (crate) const BID128_MAXDIGITS: u32 = 34;
pub (crate) const BID64_MAXDIGITS: u32  = 16;
pub (crate) const BID32_MAXDIGITS: u32  = 7;

// rounding modes
pub (crate) const BID_ROUNDING_TO_NEAREST: u32  = 0x00000;
pub (crate) const BID_ROUNDING_DOWN: u32        = 0x00001;
pub (crate) const BID_ROUNDING_UP: u32          = 0x00002;
pub (crate) const BID_ROUNDING_TO_ZERO: u32     = 0x00003;
pub (crate) const BID_ROUNDING_TIES_AWAY: u32   = 0x00004;

pub (crate) const BID_RMODE_MASK: u32  = BID_ROUNDING_TO_NEAREST
                                       | BID_ROUNDING_DOWN
                                       | BID_ROUNDING_UP
                                       | BID_ROUNDING_TO_ZERO
                                       | BID_ROUNDING_TIES_AWAY;

// status
pub (crate) const BID_FLAG_MASK: u32     = 0x0000003f;
pub (crate) const DEC_FE_ALL_EXCEPT: u32 = 0x0000003f;
pub (crate) const BID_IEEE_FLAGS: u32    = 0x0000003d;
pub (crate) const BID_EXACT_STATUS:u32   = 0x00000000;

pub (crate) const DEC_FE_INVALID:u32    = 0x01;
pub (crate) const DEC_FE_UNNORMAL:u32   = 0x02;
pub (crate) const DEC_FE_DIVBYZERO:u32  = 0x04;
pub (crate) const DEC_FE_OVERFLOW:u32   = 0x08;
pub (crate) const DEC_FE_UNDERFLOW:u32  = 0x10;
pub (crate) const DEC_FE_INEXACT:u32    = 0x20;

////////////////////////////////////////////////////////

pub (crate) const BID_INEXACT_EXCEPTION: u32            = DEC_FE_INEXACT;
pub (crate) const BID_UNDERFLOW_EXCEPTION: u32          = DEC_FE_UNDERFLOW;
pub (crate) const BID_OVERFLOW_EXCEPTION: u32           = DEC_FE_OVERFLOW;
pub (crate) const BID_ZERO_DIVIDE_EXCEPTION: u32        = DEC_FE_DIVBYZERO;
pub (crate) const BID_DENORMAL_EXCEPTION: u32           = DEC_FE_UNNORMAL;
pub (crate) const BID_INVALID_EXCEPTION: u32            = DEC_FE_INVALID;
pub (crate) const BID_UNDERFLOW_INEXACT_EXCEPTION: u32  = DEC_FE_UNDERFLOW | DEC_FE_INEXACT;
pub (crate) const BID_OVERFLOW_INEXACT_EXCEPTION: u32   = DEC_FE_OVERFLOW | DEC_FE_INEXACT;

pub (crate) const BID_MODE_MASK:u32        = 0x00001f80;
pub (crate) const BID_INEXACT_MODE:u32     = 0x00001000;
pub (crate) const BID_UNDERFLOW_MODE:u32   = 0x00000800;
pub (crate) const BID_OVERFLOW_MODE:u32    = 0x00000400;
pub (crate) const BID_ZERO_DIVIDE_MODE:u32 = 0x00000200;
pub (crate) const BID_DENORMAL_MODE:u32    = 0x00000100;
pub (crate) const BID_INVALID_MODE:u32     = 0x00000080;

/*********************************************************************
 *
 *      BID Pack/Unpack Macros
 *
 *********************************************************************/
/////////////////////////////////////////
// BID64 definitions
////////////////////////////////////////
pub (crate) const DECIMAL_MAX_EXPON_64: u32  = 767;
pub (crate) const DECIMAL_EXPONENT_BIAS: u32 = 398;
pub (crate) const MAX_FORMAT_DIGITS: u32     = 16;
/////////////////////////////////////////
// BID128 definitions
////////////////////////////////////////
pub (crate) const DECIMAL_MAX_EXPON_128: u32     = 12287;
pub (crate) const DECIMAL_EXPONENT_BIAS_128: u32 = 6176;
pub (crate) const MAX_FORMAT_DIGITS_128: u32     = 34;
/////////////////////////////////////////
// BID32 definitions
////////////////////////////////////////
pub (crate) const DECIMAL_MAX_EXPON_32: u32     = 191;
pub (crate) const DECIMAL_EXPONENT_BIAS_32: u32 = 101;
pub (crate) const MAX_FORMAT_DIGITS_32: u32     = 7;
////////////////////////////////////////
// Constant Definitions
///////////////////////////////////////
pub (crate) const SPECIAL_ENCODING_MASK64: u64  = 0x6000000000000000;
pub (crate) const INFINITY_MASK64: u64          = 0x7800000000000000;
pub (crate) const SINFINITY_MASK64: u64         = 0xf800000000000000;
pub (crate) const SSNAN_MASK64: u64             = 0xfc00000000000000;
pub (crate) const NAN_MASK64: u64               = 0x7c00000000000000;
pub (crate) const SNAN_MASK64: u64              = 0x7e00000000000000;
pub (crate) const QUIET_MASK64: u64             = 0xfdffffffffffffff;
pub (crate) const LARGE_COEFF_MASK64: u64       = 0x0007ffffffffffff;
pub (crate) const LARGE_COEFF_HIGH_BIT64: u64   = 0x0020000000000000;
pub (crate) const SMALL_COEFF_MASK64: u64       = 0x001fffffffffffff;
pub (crate) const EXPONENT_MASK64: u64          = 0x3ff;
pub (crate) const EXPONENT_SHIFT_LARGE64: u64   = 51;
pub (crate) const EXPONENT_SHIFT_SMALL64: u64   = 53;
pub (crate) const LARGEST_BID64: u64            = 0x77fb86f26fc0ffff;
pub (crate) const SMALLEST_BID64: u64           = 0xf7fb86f26fc0ffff;
pub (crate) const SMALL_COEFF_MASK128: u64      = 0x0001ffffffffffff;
pub (crate) const LARGE_COEFF_MASK128: u64      = 0x00007fffffffffff;
pub (crate) const EXPONENT_MASK128: i32         = 0x3fff;
pub (crate) const LARGEST_BID128_HIGH: u64      = 0x5fffed09bead87c0;
pub (crate) const LARGEST_BID128_LOW: u64       = 0x378d8e63ffffffff;
pub (crate) const SPECIAL_ENCODING_MASK32: u32  = 0x60000000;
pub (crate) const SINFINITY_MASK32: u32         = 0xf8000000;
pub (crate) const INFINITY_MASK32: u32          = 0x78000000;
pub (crate) const LARGE_COEFF_MASK32: u32       = 0x007fffff;
pub (crate) const LARGE_COEFF_HIGH_BIT32: u32   = 0x00800000;
pub (crate) const SMALL_COEFF_MASK32: u32       = 0x001fffff;
pub (crate) const EXPONENT_MASK32: u32          = 0xff;
pub (crate) const LARGEST_BID32: u32            = 0x77f8967f;
pub (crate) const NAN_MASK32: u32               = 0x7c000000;
pub (crate) const SNAN_MASK32: u32              = 0x7e000000;
pub (crate) const SSNAN_MASK32: u32             = 0xfc000000;
pub (crate) const QUIET_MASK32: u32             = 0xfdffffff;
pub (crate) const MASK_BINARY_EXPONENT: u64     = 0x7ff0000000000000;
pub (crate) const BINARY_EXPONENT_BIAS: u32     = 0x3ff;
pub (crate) const UPPER_EXPON_LIMIT: u32        = 51;
