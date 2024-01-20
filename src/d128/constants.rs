/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez         */
/* --------------------------------------------------------------------- */
/* Original C source code Copyright (c) 2018, Intel Corp.                */
/* --------------------------------------------------------------------- */

#![allow(unused)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

#[cfg(target_endian = "big")]
pub (crate) const BID_HIGH_128W: usize = 0;

#[cfg(target_endian = "big")]
pub (crate) const BID_LOW_128W: usize = 1;

#[cfg(target_endian = "little")]
pub (crate) const BID_HIGH_128W: usize = 1;

#[cfg(target_endian = "little")]
pub (crate) const BID_LOW_128W: usize = 0;

pub (crate) const P7: i32  = 7;
pub (crate) const P16: i32 = 16;
pub (crate) const P34: i32 = 34;


pub (crate) const MASK_STEERING_BITS: u64         = 0x6000000000000000u64;
pub (crate) const MASK_BINARY_EXPONENT1: u64      = 0x7fe0000000000000u64;
pub (crate) const MASK_BINARY_SIG1: u64           = 0x001fffffffffffffu64;
pub (crate) const MASK_BINARY_EXPONENT2: u64      = 0x1ff8000000000000u64; //used to take G[2:w+3] (sec 3.3)
pub (crate) const MASK_BINARY_SIG2: u64           = 0x0007ffffffffffffu64; //used to mask out G4:T0 (sec 3.3)
pub (crate) const MASK_BINARY_OR2: u64            = 0x0020000000000000u64; //used to prefix 8+G4 to T (sec 3.3)
pub (crate) const UPPER_EXPON_LIMIT: u32          = 51;
pub (crate) const MASK_EXP: u64                   = 0x7ffe000000000000u64;
pub (crate) const MASK_EXP2: u64                  = 0x1fff800000000000u64;
pub (crate) const MASK_SPECIAL: u64               = 0x7800000000000000u64;
pub (crate) const MASK_NAN: u64                   = 0x7c00000000000000u64;
pub (crate) const MASK_SNAN: u64                  = 0x7e00000000000000u64;
pub (crate) const MASK_ANY_INF: u64               = 0x7c00000000000000u64;
pub (crate) const MASK_INF: u64                   = 0x7800000000000000u64;
pub (crate) const MASK_SIGN: u64                  = 0x8000000000000000u64;
pub (crate) const MASK_COEFF: u64                 = 0x0001ffffffffffffu64;
pub (crate) const BIN_EXP_BIAS: u64               = (0x1820u64 << 49);

pub (crate) const EXP_MIN32: u32                  = 0x00000000;
pub (crate) const EXP_MAX32: u32                  = 0x5f800000;
pub (crate) const EXP_MIN: u64                    = 0x0000000000000000;     // EXP_MIN = (-6176 + 6176) << 49
pub (crate) const EXP_MAX: u64                    = 0x5ffe000000000000u64;  // EXP_MAX = (6111 + 6176) << 49
pub (crate) const EXP_MAX_P1: u64                 = 0x6000000000000000u64;  // EXP_MAX + 1 = (6111 + 6176 + 1) << 49
pub (crate) const EXP_P1: u64                     = 0x0002000000000000u64;  // EXP_ P1= 1 << 49
pub (crate) const expmin: i32                     = -6176; // min unbiased exponent
pub (crate) const expmax: i32                     = 6111; // max unbiased exponent
pub (crate) const expmin16: i32                   = -398; // min unbiased exponent
pub (crate) const expmax16: i32                   = 369; // max unbiased exponent
pub (crate) const expmin7: i32                    = -101; // min unbiased exponent
pub (crate) const expmax7: i32                    = 90;  // max unbiased exponent

pub (crate) const MASK_INF32: u32                  = 0x78000000;
pub (crate) const MASK_ANY_INF32: u32              = 0x7c000000;
pub (crate) const MASK_SIGN32: u32                 = 0x80000000;
pub (crate) const MASK_NAN32: u32                  = 0x7c000000;
pub (crate) const MASK_SNAN32: u32                 = 0x7e000000;
pub (crate) const SIGNMASK32: u32                  = 0x80000000;
pub (crate) const BID32_SIG_MAX: u32               = 0x0098967f;
pub (crate) const BID64_SIG_MAX: u64               = 0x002386F26FC0FFFFu64;
pub (crate) const SIGNMASK64: u64                  = 0x8000000000000000u64;
pub (crate) const MASK_STEERING_BITS32: u32        = 0x60000000;
pub (crate) const MASK_BINARY_EXPONENT1_32: u32    = 0x7f800000;
pub (crate) const MASK_BINARY_SIG1_32: u32         = 0x007fffff;
pub (crate) const MASK_BINARY_EXPONENT2_32: u32    = 0x1fe00000; //used to take G[2:w+3] (sec 3.3)
pub (crate) const MASK_BINARY_SIG2_32: u32         = 0x001fffff; //used to mask out G4:T0 (sec 3.3)
pub (crate) const MASK_BINARY_OR2_32: u32          = 0x00800000;
pub (crate) const MASK_SPECIAL32: u32              = 0x78000000;

// TYPE parameters
pub (crate) const BID128_MAXDIGITS: u32 = 34;
pub (crate) const BID64_MAXDIGITS: u32  = 16;
pub (crate) const BID32_MAXDIGITS: u32  = 7;

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

pub (crate) const BID_MODE_MASK:u32        = 0x00001f80;
pub (crate) const BID_INEXACT_MODE:u32     = 0x00001000;
pub (crate) const BID_UNDERFLOW_MODE:u32   = 0x00000800;
pub (crate) const BID_OVERFLOW_MODE:u32    = 0x00000400;
pub (crate) const BID_ZERO_DIVIDE_MODE:u32 = 0x00000200;
pub (crate) const BID_DENORMAL_MODE:u32    = 0x00000100;
pub (crate) const BID_INVALID_MODE:u32     = 0x00000080;

//////////////////////////////////////////////
//      BID Pack/Unpack Macros
//////////////////////////////////////////////

/////////////////////////////////////////
// BID64 definitions
////////////////////////////////////////
pub (crate) const DECIMAL_MAX_EXPON_64: i32  = 767;
pub (crate) const DECIMAL_EXPONENT_BIAS: i32 = 398;
pub (crate) const MAX_FORMAT_DIGITS: u32     = 16;

/////////////////////////////////////////
// BID128 definitions
////////////////////////////////////////
pub (crate) const DECIMAL_MAX_EXPON_128: u32     = 12287;
pub (crate) const DECIMAL_EXPONENT_BIAS_128: i32 = 6176;
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
