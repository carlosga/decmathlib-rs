/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use crate::bid_decimal_data::{BID_POWER10_TABLE_128, BID_RECIP_SCALE, BID_RECIPROCALS10_128, BID_ROUND_CONST_TABLE_128};
use crate::d128::{_IDEC_flags, RoundingMode, StatusFlags};

//////////////////////////////////////////////
// Endianess
//////////////////////////////////////////////

#[cfg(target_endian = "big")]
pub (crate) const BID_HIGH_128W: usize = 0;

#[cfg(target_endian = "big")]
pub (crate) const BID_LOW_128W: usize = 1;

#[cfg(target_endian = "little")]
pub (crate) const BID_HIGH_128W: usize = 1;

#[cfg(target_endian = "little")]
pub (crate) const BID_LOW_128W: usize = 0;

#[cfg(target_endian = "big")]
pub (crate) fn BID_SWAP128(x: &mut BID_UINT128) {
    let sw: BID_UINT64 = x.w[1];
    x.w[1] = x.w[0];
    x.w[0] = sw;
}

//////////////////////////////////////////////
// Structs & Types
//////////////////////////////////////////////

pub (crate) type BID_UINT32 = u32;

pub (crate) type BID_SINT64 = i64;

pub (crate) type BID_UINT64 = u64;

pub (crate) type BID_UINT128 = crate::d128::d128;

#[derive(Debug, Copy, Clone, Default)]
#[repr(align(16))]
pub (crate) struct BID_UINT192 {
    pub (crate) w: [BID_UINT64; 3]
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(align(16))]
pub (crate) struct BID_UINT256 {
    pub (crate) w: [BID_UINT64; 4]
}

#[derive(Debug, Clone, Default)]
#[repr(align(16))]
pub (crate) struct BID_UINT384 {
    pub (crate) w: [BID_UINT64; 6]
}

#[derive(Debug, Clone, Default)]
#[repr(align(16))]
pub (crate) struct BID_UINT512 {
    pub (crate) w: [BID_UINT64; 8]
}

#[derive(Debug, Clone)]
pub (crate) struct DEC_DIGITS {
    pub (crate) digits: u32,
    pub (crate) threshold_hi: BID_UINT64,
    pub (crate) threshold_lo: BID_UINT64,
    pub (crate) digits1: u32
}

pub (crate) union BID_UI32FLOAT {
    pub (crate) ui32: BID_UINT32,
    pub (crate) d: f32
}

impl Default for BID_UI32FLOAT {
    #[must_use]
    fn default() -> Self {
        Self {
            ui32: 0
        }
    }
}

pub (crate) union BID_UI64DOUBLE {
    pub (crate) ui64: BID_UINT64,
    pub (crate) d: f64
}

impl Default for BID_UI64DOUBLE {
    #[must_use]
    fn default() -> Self {
        Self {
            ui64: 0
        }
    }
}

//////////////////////////////////////////////
//      BID Pack/Unpack Macros
//////////////////////////////////////////////

// pub (crate) const P7: i32  = 7;
// pub (crate) const P16: i32 = 16;
pub (crate) const P34: i32 = 34;

pub (crate) const MASK_STEERING_BITS: u64         = 0x6000000000000000u64;
// pub (crate) const MASK_BINARY_EXPONENT1: u64      = 0x7fe0000000000000u64;
// pub (crate) const MASK_BINARY_SIG1: u64           = 0x001fffffffffffffu64;
// pub (crate) const MASK_BINARY_EXPONENT2: u64      = 0x1ff8000000000000u64; //used to take G[2:w+3] (sec 3.3)
// pub (crate) const MASK_BINARY_SIG2: u64           = 0x0007ffffffffffffu64; //used to mask out G4:T0 (sec 3.3)
// pub (crate) const MASK_BINARY_OR2: u64            = 0x0020000000000000u64; //used to prefix 8+G4 to T (sec 3.3)
// pub (crate) const UPPER_EXPON_LIMIT: u32          = 51;
pub (crate) const MASK_EXP: u64                   = 0x7ffe000000000000u64;
pub (crate) const MASK_EXP2: u64                  = 0x1fff800000000000u64;
pub (crate) const MASK_SPECIAL: u64               = 0x7800000000000000u64;
pub (crate) const MASK_NAN: u64                   = 0x7c00000000000000u64;
pub (crate) const MASK_SNAN: u64                  = 0x7e00000000000000u64;
pub (crate) const MASK_ANY_INF: u64               = 0x7c00000000000000u64;
pub (crate) const MASK_INF: u64                   = 0x7800000000000000u64;
pub (crate) const MASK_SIGN: u64                  = 0x8000000000000000u64;
pub (crate) const MASK_COEFF: u64                 = 0x0001ffffffffffffu64;
// pub (crate) const BIN_EXP_BIAS: u64               = 0x1820u64 << 49;

// pub (crate) const EXP_MIN32: u32                  = 0x00000000;
// pub (crate) const EXP_MAX32: u32                  = 0x5f800000;
pub (crate) const EXP_MIN: u64                    = 0x0000000000000000;     // EXP_MIN = (-6176 + 6176) << 49
// pub (crate) const EXP_MAX: u64                    = 0x5ffe000000000000u64;  // EXP_MAX = (6111 + 6176) << 49
pub (crate) const EXP_MAX_P1: u64                 = 0x6000000000000000u64;  // EXP_MAX + 1 = (6111 + 6176 + 1) << 49
pub (crate) const EXP_P1: u64                     = 0x0002000000000000u64;  // EXP_ P1= 1 << 49
pub (crate) const EXP_MIN_UNBIASED: i32           = -6176; // min unbiased exponent
pub (crate) const EXP_MAX_UNBIASED: i32           = 6111; // max unbiased exponent
// pub (crate) const EXP_MIN16_UNBIASED: i32         = -398; // min unbiased exponent
// pub (crate) const EXP_MAX16_UNBIASED: i32         = 369; // max unbiased exponent
// pub (crate) const EXP_MIN7_UNBIASED: i32          = -101; // min unbiased exponent
// pub (crate) const EXP_MAX7_UNBIASED: i32          = 90;

// pub (crate) const MASK_INF32: u32                 = 0x78000000;
// pub (crate) const MASK_ANY_INF32: u32             = 0x7c000000;
// pub (crate) const MASK_SIGN32: u32                = 0x80000000;
// pub (crate) const MASK_NAN32: u32                 = 0x7c000000;
// pub (crate) const MASK_SNAN32: u32                = 0x7e000000;
pub (crate) const SIGNMASK32: u32                 = 0x80000000;
// pub (crate) const BID32_SIG_MAX: u32              = 0x0098967f;
// pub (crate) const BID64_SIG_MAX: u64              = 0x002386F26FC0FFFFu64;
pub (crate) const SIGNMASK64: u64                 = 0x8000000000000000u64;
// pub (crate) const MASK_STEERING_BITS32: u32       = 0x60000000;
// pub (crate) const MASK_BINARY_EXPONENT1_32: u32   = 0x7f800000;
// pub (crate) const MASK_BINARY_SIG1_32: u32        = 0x007fffff;
// pub (crate) const MASK_BINARY_EXPONENT2_32: u32   = 0x1fe00000;
// //used to take G[2:w+3] (sec 3.3)
// pub (crate) const MASK_BINARY_SIG2_32: u32        = 0x001fffff;
// //used to mask out G4:T0 (sec 3.3)
// pub (crate) const MASK_BINARY_OR2_32: u32         = 0x00800000;
// pub (crate) const MASK_SPECIAL32: u32             = 0x78000000;

// TYPE parameters
pub (crate) const BID128_MAXDIGITS: u32 = 34;
// pub (crate) const BID64_MAXDIGITS: u32  = 16;
// pub (crate) const BID32_MAXDIGITS: u32  = 7;

// status
// pub (crate) const BID_FLAG_MASK: u32     = 0x0000003f;
// pub (crate) const DEC_FE_ALL_EXCEPT: u32 = 0x0000003f;
// pub (crate) const BID_IEEE_FLAGS: u32    = 0x0000003d;

pub (crate) const DEC_FE_INVALID:u32    = 0x01;
pub (crate) const DEC_FE_UNNORMAL:u32   = 0x02;
pub (crate) const DEC_FE_DIVBYZERO:u32  = 0x04;
pub (crate) const DEC_FE_OVERFLOW:u32   = 0x08;
pub (crate) const DEC_FE_UNDERFLOW:u32  = 0x10;
pub (crate) const DEC_FE_INEXACT:u32    = 0x20;

// pub (crate) const BID_MODE_MASK:u32        = 0x00001f80;
// pub (crate) const BID_INEXACT_MODE:u32     = 0x00001000;
// pub (crate) const BID_UNDERFLOW_MODE:u32   = 0x00000800;
// pub (crate) const BID_OVERFLOW_MODE:u32    = 0x00000400;
// pub (crate) const BID_ZERO_DIVIDE_MODE:u32 = 0x00000200;
// pub (crate) const BID_DENORMAL_MODE:u32    = 0x00000100;
// pub (crate) const BID_INVALID_MODE:u32     = 0x00000080;

//////////////////////////////////////////////
// BID128 definitions
//////////////////////////////////////////////
pub (crate) const DECIMAL_MAX_EXPON_128: i32     = 12287;
pub (crate) const DECIMAL_EXPONENT_BIAS_128: i32 = 6176;
pub (crate) const MAX_FORMAT_DIGITS_128: u32     = 34;

//////////////////////////////////////////////
// Constant Definitions
//////////////////////////////////////////////
pub (crate) const SPECIAL_ENCODING_MASK64: u64  = 0x6000000000000000;
pub (crate) const INFINITY_MASK64: u64          = 0x7800000000000000;
pub (crate) const SINFINITY_MASK64: u64         = 0xf800000000000000;
// pub (crate) const SSNAN_MASK64: u64             = 0xfc00000000000000;
pub (crate) const NAN_MASK64: u64               = 0x7c00000000000000;
pub (crate) const SNAN_MASK64: u64              = 0x7e00000000000000;
pub (crate) const QUIET_MASK64: u64             = 0xfdffffffffffffff;
// pub (crate) const LARGE_COEFF_MASK64: u64       = 0x0007ffffffffffff;
// pub (crate) const LARGE_COEFF_HIGH_BIT64: u64   = 0x0020000000000000;
// pub (crate) const SMALL_COEFF_MASK64: u64       = 0x001fffffffffffff;
// pub (crate) const EXPONENT_MASK64: u64          = 0x3ff;
// pub (crate) const EXPONENT_SHIFT_LARGE64: u64   = 51;
// pub (crate) const EXPONENT_SHIFT_SMALL64: u64   = 53;
// pub (crate) const LARGEST_BID64: u64            = 0x77fb86f26fc0ffff;
// pub (crate) const SMALLEST_BID64: u64           = 0xf7fb86f26fc0ffff;
pub (crate) const SMALL_COEFF_MASK128: u64      = 0x0001ffffffffffff;
pub (crate) const LARGE_COEFF_MASK128: u64      = 0x00007fffffffffff;
pub (crate) const EXPONENT_MASK128: i32         = 0x3fff;
pub (crate) const LARGEST_BID128_HIGH: u64      = 0x5fffed09bead87c0;
pub (crate) const LARGEST_BID128_LOW: u64       = 0x378d8e63ffffffff;
// pub (crate) const SPECIAL_ENCODING_MASK32: u32  = 0x60000000;
// pub (crate) const SINFINITY_MASK32: u32         = 0xf8000000;
// pub (crate) const INFINITY_MASK32: u32          = 0x78000000;
// pub (crate) const LARGE_COEFF_MASK32: u32       = 0x007fffff;
// pub (crate) const LARGE_COEFF_HIGH_BIT32: u32   = 0x00800000;
// pub (crate) const SMALL_COEFF_MASK32: u32       = 0x001fffff;
// pub (crate) const EXPONENT_MASK32: u32          = 0xff;
// pub (crate) const LARGEST_BID32: u32            = 0x77f8967f;
// pub (crate) const NAN_MASK32: u32               = 0x7c000000;
// pub (crate) const SNAN_MASK32: u32              = 0x7e000000;
// pub (crate) const SSNAN_MASK32: u32             = 0xfc000000;
// pub (crate) const QUIET_MASK32: u32             = 0xfdffffff;
pub (crate) const MASK_BINARY_EXPONENT: u64     = 0x7ff0000000000000;
// pub (crate) const BINARY_EXPONENT_BIAS: u32     = 0x3ff;

//////////////////////////////////////////////
// SWAP
//////////////////////////////////////////////

#[inline]
pub (crate) fn swap<T: Copy>(A: &mut T, B: &mut T, T: &mut T) {
    *T = *A;
    *A = *B;
    *B = *T;
}

//////////////////////////////////////////////
// BID128 pack/unpack macros
//////////////////////////////////////////////

///   Macro for handling BID128 underflow
///         sticky bit given as additional argument
#[inline]
pub (crate) fn bid_handle_UF_128_rem(sgn: BID_UINT64, mut expon: i32, CQ: &BID_UINT128, R: BID_UINT64, rnd_mode: RoundingMode, pfpsc: &mut _IDEC_flags) -> BID_UINT128 {
    let T128: &BID_UINT128;
    let TP128: &BID_UINT128;
    let mut Qh: BID_UINT128;
    let Ql: BID_UINT128;
    let mut Qh1: BID_UINT128;
    let mut Stemp: BID_UINT128 = Default::default();
    let mut Tmp: BID_UINT128 = Default::default();
    let Tmp1: BID_UINT128;
    let mut CQ2: BID_UINT128 = Default::default();
    let mut CQ8: BID_UINT128 = Default::default();
    let mut carry: BID_UINT64;
    let CY: BID_UINT64;
    let ed2: i32;
    let amount: i32;
    let mut rmode: RoundingMode;
    let mut status: _IDEC_flags;
    let mut CQ: BID_UINT128 = *CQ;
    let mut pres: BID_UINT128 = Default::default();

    // UF occurs
    if expon + (MAX_FORMAT_DIGITS_128 as i32) < 0 {
        __set_status_flags(pfpsc, StatusFlags::BID_UNDERFLOW_EXCEPTION | StatusFlags::BID_INEXACT_EXCEPTION);
        pres.w[1] = sgn;
        pres.w[0] = 0;
        if  (sgn != 0 && rnd_mode == RoundingMode::Downward)
	     || (sgn == 0 && rnd_mode == RoundingMode::Upward) {
            pres.w[0] = 1u64;
        }
        return pres;
    }

    // CQ *= 10
    CQ2.w[1] = (CQ.w[1] << 1) | (CQ.w[0] >> 63);
    CQ2.w[0] =  CQ.w[0] << 1;
    CQ8.w[1] = (CQ.w[1] << 3) | (CQ.w[0] >> 61);
    CQ8.w[0] =  CQ.w[0] << 3;

    CQ = __add_128_128(&CQ2, &CQ8);

    // add remainder
    if R != 0 {
      CQ.w[0] |= 1;
    }

    ed2 = 1 - expon;
    // add rounding constant to CQ
    rmode = rnd_mode;
    if sgn != 0 && ((rmode as u32 - 1u32) < 2) {
        rmode = RoundingMode::from(3 - (rmode as u32));
    }
    T128             = &BID_ROUND_CONST_TABLE_128[rmode as usize][ed2 as usize];
    (CQ.w[0], carry) = __add_carry_out(T128.w[0], CQ.w[0]);
    CQ.w[1]          = CQ.w[1] + T128.w[1] + carry;
    TP128            = &BID_RECIPROCALS10_128[ed2 as usize];
    (Qh, Ql)         = __mul_128x128_full(&CQ, TP128);
    amount           = BID_RECIP_SCALE[ed2 as usize];

    if amount >= 64 {
        CQ.w[0] = Qh.w[1] >> (amount - 64);
        CQ.w[1] = 0;
    } else {
        CQ = __shr_128(&Qh, amount);
    }

    expon = 0;

    // #ifndef IEEE_ROUND_NEAREST_TIES_AWAY
    // #ifndef IEEE_ROUND_NEAREST
    if rnd_mode == RoundingMode::NearestEven && (CQ.w[0] & 1) == 1{
        // check whether fractional part of initial_P/10^ed1 is exactly .5

        // get remainder
        Qh1 = __shl_128_long(&Qh, 128 - amount);

        if (Qh1.w[1] == 0)
        && (Qh1.w[0] == 0)
        && (Ql.w[1]  < BID_RECIPROCALS10_128[ed2 as usize].w[1]
        || (Ql.w[1] == BID_RECIPROCALS10_128[ed2 as usize].w[1]
         && Ql.w[0]  < BID_RECIPROCALS10_128[ed2 as usize].w[0])) {
            CQ.w[0] -= 1;
        }
    }

    if is_inexact(*pfpsc) {
        __set_status_flags(pfpsc, StatusFlags::BID_UNDERFLOW_EXCEPTION);
    } else {
        status = StatusFlags::BID_INEXACT_EXCEPTION;
        // get remainder
        Qh1 = __shl_128_long(&Qh, 128 - amount);

        match rmode {
            RoundingMode::NearestEven | RoundingMode::NearestAway => {
                // test whether fractional part is 0
                if (Qh1.w[1] == 0x8000000000000000u64)
                && (Qh1.w[0] == 0)
                && (Ql.w[1]  < BID_RECIPROCALS10_128[ed2 as usize].w[1]
                || (Ql.w[1] == BID_RECIPROCALS10_128[ed2 as usize].w[1]
                 && Ql.w[0]  < BID_RECIPROCALS10_128[ed2 as usize].w[0])) {
                    status = StatusFlags::BID_EXACT_STATUS;
                }
            },
            RoundingMode::Downward | RoundingMode::TowardZero => {
                if (Qh1.w[1] == 0)
                && (Qh1.w[0] == 0)
                && (Ql.w[1]  < BID_RECIPROCALS10_128[ed2 as usize].w[1]
                || (Ql.w[1] == BID_RECIPROCALS10_128[ed2 as usize].w[1]
                 && Ql.w[0]  < BID_RECIPROCALS10_128[ed2 as usize].w[0])) {
                    status = StatusFlags::BID_EXACT_STATUS;
                }
            },
            _ => {
                // round up
                (Stemp.w[0], CY)    = __add_carry_out(Ql.w[0], BID_RECIPROCALS10_128[ed2 as usize].w[0]);
                (Stemp.w[1], carry) = __add_carry_in_out (Ql.w[1], BID_RECIPROCALS10_128[ed2 as usize].w[1], CY);
                Qh                  = __shr_128_long(&Qh1, 128 - amount);
                Tmp.w[0]            = 1;
                Tmp.w[1]            = 0;
                Tmp1                = __shl_128_long (&Tmp, amount);
                Qh.w[0]            += carry;
                if Qh.w[0] < carry {
                    Qh.w[1] += 1;
                }
                if __unsigned_compare_ge_128(&Qh, &Tmp1) {
                    status = StatusFlags::BID_EXACT_STATUS;
                }
            }
        }

        if status != StatusFlags::BID_EXACT_STATUS {
            __set_status_flags(pfpsc, StatusFlags::BID_UNDERFLOW_EXCEPTION | status);
        }
    }

    // #endif

    pres.w[1] = sgn | CQ.w[1];
    pres.w[0] = CQ.w[0];

    pres
}

/// Macro for handling BID128 underflow
#[inline]
pub (crate) fn handle_UF_128(sgn: BID_UINT64, expon: i32, CQ: &BID_UINT128, rnd_mode: RoundingMode, pfpsc: &mut _IDEC_flags) -> BID_UINT128 {
    let T128: BID_UINT128;
    let TP128: BID_UINT128;
    let mut Qh: BID_UINT128;
    let Ql: BID_UINT128;
    let mut Qh1: BID_UINT128;
    let mut Stemp: BID_UINT128 = Default::default();
    let mut Tmp: BID_UINT128 = Default::default();
    let Tmp1: BID_UINT128;
    let mut carry: BID_UINT64;
    let CY: BID_UINT64;
    let ed2: i32;
    let amount: i32;
    let mut rmode: RoundingMode;
    let mut status: _IDEC_flags = StatusFlags::BID_EXACT_STATUS;
    let mut pres: BID_UINT128 = Default::default();
    let mut CQ: BID_UINT128 = *CQ;
    let mut expon = expon;

    // UF occurs
    if (expon + (MAX_FORMAT_DIGITS_128 as i32)) < 0 {
        __set_status_flags(pfpsc, StatusFlags::BID_UNDERFLOW_EXCEPTION | StatusFlags::BID_INEXACT_EXCEPTION);
        pres.w[1] = sgn;
        pres.w[0] = 0;
        if (sgn != 0 && rnd_mode == RoundingMode::Downward)
        || (sgn == 0 && rnd_mode == RoundingMode::Upward) {
          pres.w[0] = 1u64;
        }
        return pres;
    }

    ed2 = 0 - expon;

    // add rounding constant to CQ
    rmode = rnd_mode;
    if sgn != 0 && ((rmode as u32 - 1u32) < 2) {
        rmode = RoundingMode::from(3 - (rmode as u32));
    }

    T128             = BID_ROUND_CONST_TABLE_128[rmode as usize][ed2 as usize];
    (CQ.w[0], carry) = __add_carry_out(T128.w[0], CQ.w[0]);
    CQ.w[1]          = CQ.w[1] + T128.w[1] + carry;
    TP128            = BID_RECIPROCALS10_128[ed2 as usize];
    (Qh, Ql)         = __mul_128x128_full(&CQ, &TP128);
    amount           = BID_RECIP_SCALE[ed2 as usize];

    if amount >= 64 {
        CQ.w[0] = Qh.w[1] >> (amount - 64);
        CQ.w[1] = 0;
    } else {
        CQ = __shr_128(&Qh, amount);
    }

    expon = 0;

    if rnd_mode == RoundingMode::NearestEven && (CQ.w[0] & 1) == 1 {
        // check whether fractional part of initial_P/10^ed1 is exactly .5

        // get remainder
        Qh1 = __shl_128_long(&Qh, 128 - amount);

        if  Qh1.w[1] == 0
         && Qh1.w[0] == 0
        && (Ql.w[1]  < BID_RECIPROCALS10_128[ed2 as usize].w[1]
        || (Ql.w[1] == BID_RECIPROCALS10_128[ed2 as usize].w[1]
         && Ql.w[0]  < BID_RECIPROCALS10_128[ed2 as usize].w[0])) {
            CQ.w[0] -= 1;
        }
    }

    if is_inexact(*pfpsc) {
        __set_status_flags(pfpsc, StatusFlags::BID_UNDERFLOW_EXCEPTION);
    } else {
        status = StatusFlags::BID_INEXACT_EXCEPTION;
        // get remainder
        Qh1 = __shl_128_long(&Qh, 128 - amount);

        match rmode {
            RoundingMode::NearestEven | RoundingMode::NearestAway => {
                // test whether fractional part is 0
                if  Qh1.w[1] == 0x8000000000000000u64
                && (Qh1.w[0] == 0)
                && (Ql.w[1]  < BID_RECIPROCALS10_128[ed2 as usize].w[1]
                || (Ql.w[1] == BID_RECIPROCALS10_128[ed2 as usize].w[1]
                 && Ql.w[0]  < BID_RECIPROCALS10_128[ed2 as usize].w[0])) {
                    status = StatusFlags::BID_EXACT_STATUS;
                }
            },
            RoundingMode::Downward | RoundingMode::TowardZero=> {
                if (Qh1.w[1] == 0) && (Qh1.w[0] == 0)
                && (Ql.w[1]  < BID_RECIPROCALS10_128[ed2 as usize].w[1]
                || (Ql.w[1] == BID_RECIPROCALS10_128[ed2 as usize].w[1]
                 && Ql.w[0]  < BID_RECIPROCALS10_128[ed2 as usize].w[0])) {
                    status = StatusFlags::BID_EXACT_STATUS;
                }
            },
            _ =>  {
                // round up
                (Stemp.w[0], CY)    = __add_carry_out(Ql.w[0], BID_RECIPROCALS10_128[ed2 as usize].w[0]);
                (Stemp.w[1], carry) = __add_carry_in_out(Ql.w[1],BID_RECIPROCALS10_128[ed2 as usize].w[1], CY);
                Qh                  = __shr_128_long(&Qh1, 128 - amount);
                Tmp.w[0]            = 1;
                Tmp.w[1]            = 0;
                Tmp1                = __shl_128_long(&Tmp, amount);
                Qh.w[0]            += carry;
                if Qh.w[0] < carry {
                    Qh.w[1] += 1;
                }
                if __unsigned_compare_ge_128(&Qh, &Tmp1) {
                    status = StatusFlags::BID_EXACT_STATUS;
                }
            }
        }
    }

    if status != StatusFlags::BID_EXACT_STATUS {
        __set_status_flags(pfpsc, StatusFlags::BID_UNDERFLOW_EXCEPTION | status);
    }

    pres.w[1] = sgn | CQ.w[1];
    pres.w[0] = CQ.w[0];

    pres
}

/*
///  BID128 unpack, input passed by value  (used in transcendental functions only)
#[inline]
pub (crate) fn unpack_BID128_value_BLE(psign_x: &mut BID_UINT64, pexponent_x: &mut i32, pcoefficient_x: &mut BID_UINT128, x: &BID_UINT128) -> BID_UINT64 {
    let mut coeff: BID_UINT128 = Default::default();
    let T33: &BID_UINT128;
    let T34: &BID_UINT128;
    let ex: BID_UINT64;

    *psign_x = (x.w[BID_HIGH_128W]) & 0x8000000000000000u64;

    // special encodings
    if (x.w[BID_HIGH_128W] & INFINITY_MASK64) >= SPECIAL_ENCODING_MASK64 {
        if (x.w[BID_HIGH_128W] & INFINITY_MASK64) < INFINITY_MASK64 {
            // non-canonical input
            pcoefficient_x.w[BID_LOW_128W]  = 0;
            pcoefficient_x.w[BID_HIGH_128W] = 0;
            ex                              = (x.w[BID_HIGH_128W]) >> 47;
            *pexponent_x                    = (ex as i32) & EXPONENT_MASK128;
            return 0;
        }
        // 10^33
        T33 = &BID_POWER10_TABLE_128[33];

        pcoefficient_x.w[BID_LOW_128W]  = x.w[BID_LOW_128W];
        pcoefficient_x.w[BID_HIGH_128W] = (x.w[BID_HIGH_128W]) & 0x00003fffffffffffu64;

	    if  (pcoefficient_x.w[BID_HIGH_128W]  > T33.w[1])
	    || ((pcoefficient_x.w[BID_HIGH_128W] == T33.w[1])
	     && (pcoefficient_x.w[BID_LOW_128W]  >= T33.w[0])) { // non-canonical
            pcoefficient_x.w[BID_HIGH_128W] = (x.w[BID_HIGH_128W]) & 0xfe00000000000000u64;
            pcoefficient_x.w[BID_LOW_128W]  = 0;
        } else {
            pcoefficient_x.w[BID_HIGH_128W] = (x.w[BID_HIGH_128W]) & 0xfe003fffffffffffu64;
        }

        if (x.w[BID_HIGH_128W] & NAN_MASK64) == INFINITY_MASK64 {
            pcoefficient_x.w[BID_LOW_128W]  = 0;
            pcoefficient_x.w[BID_HIGH_128W] = x.w[BID_HIGH_128W] & SINFINITY_MASK64;
        }
        *pexponent_x = 0;
        return 0;	// NaN or Infinity
    }

    coeff.w[BID_LOW_128W]  = x.w[BID_LOW_128W];
    coeff.w[BID_HIGH_128W] = (x.w[BID_HIGH_128W]) & SMALL_COEFF_MASK128;

    // 10^34
    T34 = &BID_POWER10_TABLE_128[34];

    // check for non-canonical values
    if  (coeff.w[BID_HIGH_128W]  > T34.w[1])
    || ((coeff.w[BID_HIGH_128W] == T34.w[1])
     && (coeff.w[BID_LOW_128W]  >= T34.w[0])) {
	  coeff.w[BID_LOW_128W]  = 0;
	  coeff.w[BID_HIGH_128W] = 0;
    }

    pcoefficient_x.w[BID_LOW_128W]  = coeff.w[BID_LOW_128W];
    pcoefficient_x.w[BID_HIGH_128W] = coeff.w[BID_HIGH_128W];

    ex           = (x.w[BID_HIGH_128W]) >> 49;
    *pexponent_x = (ex as i32) & EXPONENT_MASK128;

    coeff.w[BID_LOW_128W] | coeff.w[BID_HIGH_128W]
}
*/

///  BID128 unpack, input passed by value
#[inline]
pub (crate) fn unpack_BID128_value(psign_x: &mut BID_UINT64, pexponent_x: &mut i32, pcoefficient_x: &mut BID_UINT128, x: &BID_UINT128) -> BID_UINT64 {
    let mut coeff: BID_UINT128 = Default::default();
    let T33: &BID_UINT128;
    let T34: &BID_UINT128;
    let ex: BID_UINT64;

    *psign_x = (x.w[1]) & 0x8000000000000000u64;

    // special encodings
    if (x.w[1] & INFINITY_MASK64) >= SPECIAL_ENCODING_MASK64 {
        if (x.w[1] & INFINITY_MASK64) < INFINITY_MASK64 {
            // non-canonical input
            pcoefficient_x.w[0] = 0;
            pcoefficient_x.w[1] = 0;
            ex                  = (x.w[1]) >> 47;
            *pexponent_x        = (ex as i32) & EXPONENT_MASK128;
            return 0;
        }
        // 10^33
        T33 = &BID_POWER10_TABLE_128[33];
        /*coeff.w[0] = x.w[0];
           coeff.w[1] = (x.w[1]) & LARGE_COEFF_MASK128;
           pcoefficient_x->w[0] = x.w[0];
           pcoefficient_x->w[1] = x.w[1];
           if (__unsigned_compare_ge_128 (coeff, T33)) // non-canonical
           pcoefficient_x->w[1] &= (~LARGE_COEFF_MASK128); */

        pcoefficient_x.w[0] = x.w[0];
        pcoefficient_x.w[1] = (x.w[1]) & 0x00003fffffffffffu64;
        if __unsigned_compare_ge_128(pcoefficient_x, T33) { // non-canonical
            pcoefficient_x.w[1] = (x.w[1]) & 0xfe00000000000000u64;
            pcoefficient_x.w[0] = 0;
        } else {
            pcoefficient_x.w[1] = (x.w[1]) & 0xfe003fffffffffffu64;
        }
        if (x.w[1] & NAN_MASK64) == INFINITY_MASK64 {
            pcoefficient_x.w[0] = 0;
            pcoefficient_x.w[1] = x.w[1] & SINFINITY_MASK64;
        }
        *pexponent_x = 0;
        return 0;	// NaN or Infinity
    }

    coeff.w[0] = x.w[0];
    coeff.w[1] = (x.w[1]) & SMALL_COEFF_MASK128;

    // 10^34
    T34 = &BID_POWER10_TABLE_128[34];
    // check for non-canonical values
    if __unsigned_compare_ge_128(&coeff, T34) {
        coeff.w[0] = 0;
        coeff.w[1] = 0;
    }

    pcoefficient_x.w[0] = coeff.w[0];
    pcoefficient_x.w[1] = coeff.w[1];

    ex           = (x.w[1]) >> 49;
    *pexponent_x = (ex as i32) & EXPONENT_MASK128;

    coeff.w[0] | coeff.w[1]
}

///  BID128 unpack, input pased by reference
#[inline]
pub (crate) fn unpack_BID128(psign_x: &mut BID_UINT64, pexponent_x: &mut i32, pcoefficient_x: &mut BID_UINT128, px: &BID_UINT128) -> BID_UINT64 {
    let mut coeff: BID_UINT128 = Default::default();
    let T33: &BID_UINT128;
    let T34: &BID_UINT128;
    let ex: BID_UINT64;

    *psign_x = (px.w[1]) & 0x8000000000000000u64;

    // special encodings
    if (px.w[1] & INFINITY_MASK64) >= SPECIAL_ENCODING_MASK64 {
        if (px.w[1] & INFINITY_MASK64) < INFINITY_MASK64 {
            // non-canonical input
            pcoefficient_x.w[0] = 0;
            pcoefficient_x.w[1] = 0;
            ex                  = (px.w[1]) >> 47;
            *pexponent_x        = (ex as i32) & EXPONENT_MASK128;
            return 0;
        }
        // 10^33
        T33                 = &BID_POWER10_TABLE_128[33];
        coeff.w[0]          = px.w[0];
        coeff.w[1]          = (px.w[1]) & LARGE_COEFF_MASK128;
        pcoefficient_x.w[0] = px.w[0];
        pcoefficient_x.w[1] = px.w[1];
        if __unsigned_compare_ge_128(&coeff, T33) { // non-canonical
            pcoefficient_x.w[1] &= !LARGE_COEFF_MASK128;
            pcoefficient_x.w[0] = 0;
        }
        *pexponent_x = 0;
        return 0; // NaN or Infinity
    }

    coeff.w[0] = px.w[0];
    coeff.w[1] = (px.w[1]) & SMALL_COEFF_MASK128;

    // 10^34
    T34 = &BID_POWER10_TABLE_128[34];
    // check for non-canonical values
    if __unsigned_compare_ge_128 (&coeff, T34) {
        coeff.w[0] = 0;
        coeff.w[1] = 0;
    }

    pcoefficient_x.w[0] = coeff.w[0];
    pcoefficient_x.w[1] = coeff.w[1];

    ex           = (px.w[1]) >> 49;
    *pexponent_x = (ex as i32) & EXPONENT_MASK128;

    coeff.w[0] | coeff.w[1]
}

/// No overflow/underflow checks
/// No checking for coefficient == 10^34 (rounding artifact)
#[inline]
pub (crate) fn bid_get_BID128_very_fast(sgn: BID_UINT64, expon: i32, coeff: &BID_UINT128) -> BID_UINT128 {
    let mut tmp: BID_UINT64;
    let mut res: BID_UINT128 = Default::default();

    res.w[0] = coeff.w[0];
    tmp      = expon as BID_UINT64;
    tmp    <<= 49;
    res.w[1] = sgn | tmp | coeff.w[1];

    res
}

/*
// same as above, but for either endian mode
#[inline]
pub (crate) fn bid_get_BID128_very_fast_BLE(sgn: BID_UINT64, expon: i32, coeff: &BID_UINT128) -> BID_UINT128 {
    let mut tmp: BID_UINT64;
    let mut res: BID_UINT128 = Default::default();

    res.w[BID_LOW_128W] = coeff.w[BID_LOW_128W];
    tmp                 = expon as BID_UINT64;
    tmp               <<= 49;
    res.w[BID_HIGH_128W] = sgn | tmp | coeff.w[BID_HIGH_128W];

    res
}
*/

/// No overflow/underflow checks
#[inline]
pub (crate) fn bid_get_BID128_fast(sgn: BID_UINT64, expon: &mut i32, coeff: &mut BID_UINT128) -> BID_UINT128 {
    let mut tmp: BID_UINT64;
    let mut res: BID_UINT128 = Default::default();
    // coeff==10^34?
    if coeff.w[1] == 0x0001ed09bead87c0u64 && coeff.w[0] == 0x378d8e6400000000u64 {
      *expon += 1;
      // set coefficient to 10^33
      coeff.w[1] = 0x0000314dc6448d93u64;
      coeff.w[0] = 0x38c15b0a00000000u64;
    }
    res.w[0] = coeff.w[0];
    tmp      = *expon as BID_UINT64;
    tmp    <<= 49;
    res.w[1] = sgn | tmp | coeff.w[1];

    res
}

/// General BID128 pack macro
pub (crate) fn bid_get_BID128(sgn: BID_UINT64, expon: i32, coeff: &BID_UINT128, rnd_mode: RoundingMode, pfpsc: &mut _IDEC_flags) -> BID_UINT128 {
    let T: &BID_UINT128;
    let mut tmp: BID_UINT64;
    let mut tmp2: BID_UINT64;
    let mut expon: i32 = expon;
    let mut coeff: BID_UINT128 = *coeff;
    let mut pres: BID_UINT128 = Default::default();

    // coeff==10^34?
    if coeff.w[1] == 0x0001ed09bead87c0u64 && coeff.w[0] == 0x378d8e6400000000u64 {
        expon += 1;
        // set coefficient to 10^33
        coeff.w[1] = 0x0000314dc6448d93u64;
        coeff.w[0] = 0x38c15b0a00000000u64;
    }

    // check OF, UF
    // if expon < 0 || expon > DECIMAL_MAX_EXPON_128 {
    if !(0..=DECIMAL_MAX_EXPON_128).contains(&expon) {
        // check UF
        if expon < 0 {
            return handle_UF_128(sgn, expon, &coeff, rnd_mode, pfpsc);
        }

        if expon - (MAX_FORMAT_DIGITS_128 as i32) <= (DECIMAL_MAX_EXPON_128) {
            T = &BID_POWER10_TABLE_128[(MAX_FORMAT_DIGITS_128 - 1) as usize];
            while __unsigned_compare_gt_128(T, &coeff) && expon > DECIMAL_MAX_EXPON_128 {
                coeff.w[1] = (coeff.w[1] << 3) + (coeff.w[1] << 1) + (coeff.w[0] >> 61) + (coeff.w[0] >> 63);
                tmp2       = coeff.w[0] << 3;
                coeff.w[0] = (coeff.w[0] << 1) + tmp2;
                if coeff.w[0] < tmp2 {
                    coeff.w[1] += 1;
                }
                expon -= 1;
            }
        }
        if expon > DECIMAL_MAX_EXPON_128 {
            if (coeff.w[1] | coeff.w[0]) == 0 {
                pres.w[1] = sgn | ((DECIMAL_MAX_EXPON_128 as BID_UINT64) << 49);
                pres.w[0] = 0;
                return pres;
            }
            // OF
            __set_status_flags (pfpsc, StatusFlags::BID_OVERFLOW_EXCEPTION | StatusFlags::BID_INEXACT_EXCEPTION);
            if rnd_mode == RoundingMode::TowardZero
            || (sgn != 0 && rnd_mode == RoundingMode::Upward)
            || (sgn == 0 && rnd_mode == RoundingMode::Downward) {
                pres.w[1] = sgn | LARGEST_BID128_HIGH;
                pres.w[0] = LARGEST_BID128_LOW;
            } else {
                pres.w[1] = sgn | INFINITY_MASK64;
                pres.w[0] = 0;
            }
            return pres;
        }
    }

    pres.w[0] = coeff.w[0];
    tmp       = expon as BID_UINT64;
    tmp     <<= 49;
    pres.w[1] = sgn | tmp | coeff.w[1];

    pres
}

//////////////////////////////////////////////
//  Status Flag Handling
//////////////////////////////////////////////

#[inline]
pub (crate) fn __set_status_flags(fpsc: &mut _IDEC_flags, status: _IDEC_flags) {
    *fpsc |= status;
}

#[inline]
pub (crate) fn is_inexact(fpsc: _IDEC_flags) -> bool{
    fpsc & StatusFlags::BID_INEXACT_EXCEPTION == StatusFlags::BID_INEXACT_EXCEPTION
}

//////////////////////////////////////////////
// Logical Shift Macros
//////////////////////////////////////////////

#[inline]
pub (crate) fn __shr_128(A: &BID_UINT128, k: i32) -> BID_UINT128 {
    let mut Q: BID_UINT128 = Default::default();

    Q.w[0]  = A.w[0] >> k;
    Q.w[0] |= A.w[1] << (64 - k);
    Q.w[1]  = A.w[1] >> k;

    Q
}

#[inline]
pub (crate) fn __shr_256(A: &BID_UINT256, k: i32) -> BID_UINT256 {
    let mut Q: BID_UINT256 = Default::default();

    Q.w[0]  = A.w[0] >> k;
    Q.w[0] |= A.w[1] << (64 - k);
    Q.w[1]  = A.w[1] >> k;

    Q
}

#[inline]
pub (crate) fn __shr_128_long(A: &BID_UINT128, k: i32) -> BID_UINT128 {
    let mut Q: BID_UINT128 = Default::default();

    if k < 64 {
        Q.w[0]  = A.w[0] >> k;
        Q.w[0] |= A.w[1] << (64 - k);
        Q.w[1]  = A.w[1] >> k;
    } else {
        Q.w[0] = A.w[1] >> ((k) - 64);
        Q.w[1] = 0;
    }

    Q
}

#[inline]
pub (crate) fn __shl_128_long(A: &BID_UINT128, k: i32) -> BID_UINT128 {
    let mut Q: BID_UINT128 = Default::default();

    if k < 64 {
        Q.w[1]  = A.w[1] << k;
        Q.w[1] |= A.w[0] >> (64 - k);
        Q.w[0]  = A.w[0] << k;
    }  else {
        Q.w[1] = A.w[0] << ((k) - 64);
        Q.w[0] = 0;
    }

    Q
}

//////////////////////////////////////////////
// Add/Subtract Macros
//////////////////////////////////////////////

/// add 64-bit value to 128-bit
#[inline]
pub (crate) fn __add_128_64(A128: &BID_UINT128, B64: BID_UINT64) -> BID_UINT128 {
    let mut R64H: BID_UINT64  = A128.w[1];
    let mut R128: BID_UINT128 = Default::default();
    R128.w[0] = B64 + A128.w[0];
    if R128.w[0] < B64 {
        R64H += 1;
    }
    R128.w[1] = R64H;
    R128
}

/// subtract 64-bit value from 128-bi
#[inline]
pub (crate) fn __sub_128_64(A128: &BID_UINT128, B64: BID_UINT64) -> BID_UINT128 {
    let mut R64H: BID_UINT64 = A128.w[1];
    let mut R128: BID_UINT128 = Default::default();
    if A128.w[0] < B64 {
        R64H -= 1;
    }
    R128.w[1] = R64H;
    R128.w[0] = A128.w[0] - B64;
    R128
}

// add 128-bit value to 128-bit - assume no carry-out
#[inline]
pub (crate) fn __add_128_128(A128: &BID_UINT128, B128: &BID_UINT128) -> BID_UINT128 {
    let mut Q128: BID_UINT128 = Default::default();
    let mut R128: BID_UINT128 = Default::default();
    Q128.w[1] = A128.w[1] + B128.w[1];
    Q128.w[0] = B128.w[0] + A128.w[0];
    if Q128.w[0] < B128.w[0] {
        Q128.w[1] += 1;
    }
    R128.w[1] = Q128.w[1];
    R128.w[0] = Q128.w[0];

    R128
}

#[inline]
pub (crate) fn __sub_128_128(A128: &BID_UINT128, B128: &BID_UINT128) -> BID_UINT128 {
    let mut Q128: BID_UINT128 = Default::default();
    let mut R128: BID_UINT128 = Default::default();
    Q128.w[1] = A128.w[1] - B128.w[1];
    Q128.w[0] = A128.w[0] - B128.w[0];
    if A128.w[0] < B128.w[0] {
        Q128.w[1] -= 1;
    }
    R128.w[1] = Q128.w[1];
    R128.w[0] = Q128.w[0];

    R128
}

#[inline]
pub (crate) fn __sub_256_128_to_256(A128: &BID_UINT256, B128: &BID_UINT128) -> BID_UINT256 {
    let mut Q128: BID_UINT128 = Default::default();
    let mut R256: BID_UINT256 = Default::default();
    Q128.w[1] = A128.w[1] - B128.w[1];
    Q128.w[0] = A128.w[0] - B128.w[0];
    if A128.w[0] < B128.w[0] {
        Q128.w[1] -= 1;
    }
    R256.w[1] = Q128.w[1];
    R256.w[0] = Q128.w[0];

    R256
}

/// Returns (sum, carry)
#[inline]
pub (crate) fn __add_carry_out(X: BID_UINT64, Y: BID_UINT64) -> (BID_UINT64, BID_UINT64) {
    let S: BID_UINT64  = X + Y;
    let CY: BID_UINT64 = if S < X { 1 } else { 0 };
    (S, CY)
}

/// Returns (sum, carry)
#[inline]
pub (crate) fn __add_carry_in_out(X: BID_UINT64, Y: BID_UINT64, CI: BID_UINT64) -> (BID_UINT64, BID_UINT64) {
    let X1: BID_UINT64 = X + CI;
    let S: BID_UINT64  = X1 + Y;
    let CY: BID_UINT64 = if S < X1 || X1 < CI { 1u64 } else { 0 };
    (S, CY)
}

#[inline]
pub (crate) fn __sub_borrow_out(X: BID_UINT64, Y: BID_UINT64) -> (BID_UINT64, BID_UINT64) {
    let X1: BID_UINT64 = X;
	let S: BID_UINT64  = X - Y;
	let CY: BID_UINT64 = if S > X1 { 1  } else { 0 };
	(S, CY)
}

#[inline]
pub (crate) fn __sub_borrow_in_out(X: BID_UINT64, Y: BID_UINT64, CI: BID_UINT64) -> (BID_UINT64, BID_UINT64) {
    let X0: BID_UINT64 = X;
	let X1: BID_UINT64 = X - CI;
	let S: BID_UINT64  = X1 - Y;
	let CY: BID_UINT64 = if (S > X1) || (X1 > X0) { 1 } else { 0 };
	(S, CY)
}

//////////////////////////////////////////////
// Multiply Macros
//////////////////////////////////////////////

#[inline]
pub (crate) fn __mul_64x64_to_64(CX: BID_UINT64, CY: BID_UINT64) -> BID_UINT64 {
    CX * CY
}

//////////////////////////////////////////////
//      Unsigned Multiply Macros
//////////////////////////////////////////////

/// get fu64 64x64bit product
#[inline]
pub (crate) fn __mul_64x64_to_128(CX: BID_UINT64, CY: BID_UINT64) -> BID_UINT128 {
    let CXH: BID_UINT64 = CX >> 32;
    let CXL: BID_UINT64 = (CX as BID_UINT32) as BID_UINT64;
    let CYH: BID_UINT64 = CY >> 32;
    let CYL: BID_UINT64 = (CY as BID_UINT32) as BID_UINT64;

    let mut PM: BID_UINT64 = CXH * CYL;
    let mut PH: BID_UINT64 = CXH * CYH;
    let PL: BID_UINT64     = CXL * CYL;
    let PM2: BID_UINT64    = CXL * CYH;
    PH += PM >> 32;
    PM  = ((PM as BID_UINT32) as BID_UINT64) + PM2 + (PL >> 32) as BID_UINT64;

    BID_UINT128::new(PH + (PM >> 32), (PM << 32) + ((PL as BID_UINT32) as BID_UINT64))
}

/// get fu64 64x64bit product
/// Note: This macro is used for CX < 2^61, CY < 2^61
#[inline]
pub (crate) fn __mul_64x64_to_128_fast(CX: BID_UINT64, CY: BID_UINT64) -> BID_UINT128 {
    let CXH: BID_UINT64 = CX >> 32;
    let CXL: BID_UINT64 = (CX as BID_UINT32) as BID_UINT64;
    let CYH: BID_UINT64 = CY >> 32;
    let CYL: BID_UINT64 = (CY as BID_UINT32) as BID_UINT64;

    let mut PM: BID_UINT64 = CXH * CYL;
    let PL: BID_UINT64     = CXL * CYL;
    let PH: BID_UINT64     = CXH * CYH;
    PM += CXL * CYH;
    PM += PL >> 32;

    BID_UINT128::new(PH + (PM >> 32), (PM << 32) + ((PL as BID_UINT32) as BID_UINT64))
}

/// get fu64 64x64bit product
#[inline]
pub (crate) fn __mul_64x64_to_128_full(CX: BID_UINT64, CY: BID_UINT64) -> BID_UINT128 {
    let CXH:BID_UINT64 = CX >> 32;
    let CXL:BID_UINT64 = (CX as BID_UINT32) as BID_UINT64;
    let CYH:BID_UINT64 = CY >> 32;
    let CYL:BID_UINT64 = (CY as BID_UINT32) as BID_UINT64;

    let mut PM:BID_UINT64 = CXH * CYL;
    let mut PH:BID_UINT64 = CXH * CYH;
    let PL:BID_UINT64     = CXL * CYL;
    let PM2:BID_UINT64    = CXL * CYH;
    PH += PM >> 32;
    PM  = (PM as BID_UINT32) as BID_UINT64 + PM2 + (PL >> 32);

    BID_UINT128::new(PH + (PM >> 32), (PM << 32) + ((PL as BID_UINT32) as BID_UINT64))
}

#[inline]
pub (crate) fn __mul_128x128_high(A: &BID_UINT128, B: &BID_UINT128) -> BID_UINT128 {
    let ALBH: BID_UINT128 = __mul_64x64_to_128(A.w[0], B.w[1]);
    let AHBL: BID_UINT128 = __mul_64x64_to_128(B.w[0], A.w[1]);
    let ALBL: BID_UINT128 = __mul_64x64_to_128(A.w[0], B.w[0]);
    let AHBH: BID_UINT128 = __mul_64x64_to_128(A.w[1], B.w[1]);
    let QM: BID_UINT128   = __add_128_128(&ALBH, &AHBL);
    let QM2: BID_UINT128  = __add_128_64(&QM, ALBL.w[1]);

    __add_128_64(&AHBH, QM2.w[1])
}

#[inline]
pub (crate) fn __mul_128x128_full(A: &BID_UINT128, B: &BID_UINT128) -> (BID_UINT128, BID_UINT128) {
    let mut Ql: BID_UINT128 = Default::default();
    let ALBH: BID_UINT128 = __mul_64x64_to_128(A.w[0], B.w[1]);
    let AHBL: BID_UINT128 = __mul_64x64_to_128(B.w[0], A.w[1]);
    let ALBL: BID_UINT128 = __mul_64x64_to_128(A.w[0], B.w[0]);
    let AHBH: BID_UINT128 = __mul_64x64_to_128(A.w[1], B.w[1]);
    let QM: BID_UINT128   = __add_128_128(&ALBH, &AHBL);
    Ql.w[0] = ALBL.w[0];
    let QM2: BID_UINT128  = __add_128_64(&QM, ALBL.w[1]);
    let Qh: BID_UINT128   = __add_128_64(&AHBH, QM2.w[1]);
    Ql.w[1] = QM2.w[0];

    (Qh, Ql)
}

#[inline]
pub (crate) fn __mul_128x128_low(A: &BID_UINT128, B: &BID_UINT128) -> BID_UINT128 {
    let mut Ql: BID_UINT128 = Default::default();
    let ALBL: BID_UINT128   = __mul_64x64_to_128(A.w[0], B.w[0]);
    let QM64: BID_UINT64    = B.w[0] * A.w[1] + A.w[0] * B.w[1];

    Ql.w[0] = ALBL.w[0];
    Ql.w[1] = QM64 + ALBL.w[1];

    Ql
}

#[inline]
pub (crate) fn __mul_64x128_low(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT128 {
    let mut Ql: BID_UINT128 = Default::default();
    let ALBH: BID_UINT128   = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128   = __mul_64x64_to_128(A, B.w[0]);
    Ql.w[0] = ALBL.w[0];
    let QM2: BID_UINT128    = __add_128_64(&ALBH, ALBL.w[1]);
    Ql.w[1] = QM2.w[0];

    Ql
}

#[inline]
pub (crate) fn __mul_64x128_full(A: BID_UINT64, B: &BID_UINT128) -> (BID_UINT64, BID_UINT128) {
    let mut Ql: BID_UINT128 = Default::default();
    let ALBH: BID_UINT128   = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128   = __mul_64x64_to_128(A, B.w[0]);

    Ql.w[0]              = ALBL.w[0];
    let QM2: BID_UINT128 = __add_128_64(&ALBH, ALBL.w[1]);
    Ql.w[1]              = QM2.w[0];
    let Ph: BID_UINT64   = QM2.w[1];

    (Ph, Ql)
}

#[inline]
pub (crate) fn __mul_64x128_to_192(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT192 {
    let mut Q: BID_UINT192 = Default::default();
    let ALBH: BID_UINT128  = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128  = __mul_64x64_to_128(A, B.w[0]);

    Q.w[0] = ALBL.w[0];
    let QM2: BID_UINT128 = __add_128_64(&ALBH, ALBL.w[1]);
    Q.w[1] = QM2.w[0];
    Q.w[2] = QM2.w[1];

    Q
}

#[inline]
pub (crate) fn __mul_64x128_to_256(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT256 {
    let mut Q: BID_UINT256 = Default::default();
    let ALBH: BID_UINT128  = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128  = __mul_64x64_to_128(A, B.w[0]);

    Q.w[0] = ALBL.w[0];
    let QM2: BID_UINT128 = __add_128_64(&ALBH, ALBL.w[1]);
    Q.w[1] = QM2.w[0];
    Q.w[2] = QM2.w[1];

    Q
}

#[inline]
pub (crate) fn __mul_64x128_to192(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT192 {
    let mut Q: BID_UINT192 = Default::default();
    let ALBH: BID_UINT128  = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128  = __mul_64x64_to_128(A, B.w[0]);

    Q.w[0] = ALBL.w[0];
    let QM2: BID_UINT128  = __add_128_64(&ALBH, ALBL.w[1]);
    Q.w[1] = QM2.w[0];
    Q.w[2] = QM2.w[1];

    Q
}

#[inline]
pub (crate) fn __mul_128x128_to_256(A: &BID_UINT128, B: &BID_UINT128) -> BID_UINT256 {
    let CY1: BID_UINT64;
    let CY2: BID_UINT64;
    let (Phl, Qll) = __mul_64x128_full(A.w[0], B);
    let (Phh, Qlh) = __mul_64x128_full(A.w[1], B);
    let mut P256: BID_UINT256 = Default::default();

    P256.w[0] = Qll.w[0];
    (P256.w[1], CY1) = __add_carry_out(Qlh.w[0], Qll.w[1]);
    (P256.w[2], CY2) = __add_carry_in_out(Qlh.w[1], Phl, CY1);
    P256.w[3] = Phh + CY2;

    P256
}

#[inline]
pub (crate) fn __mul_64x64_to_128MACH(CX64: BID_UINT64, CY64: BID_UINT64) -> BID_UINT128 {
    let CXH: BID_UINT64    = CX64 >> 32;
    let CXL: BID_UINT64    = (CX64 as BID_UINT32) as BID_UINT64;
    let CYH: BID_UINT64    = CY64 >> 32;
    let CYL: BID_UINT64    = (CY64 as BID_UINT32) as BID_UINT64;
    let mut PM: BID_UINT64 = CXH * CYL;
    let mut PH: BID_UINT64 = CXH * CYH;
    let PL: BID_UINT64     = CXL * CYL;
    let PM2: BID_UINT64    = CXL * CYH;

    PH += PM >> 32;
    PM  = ((PM as BID_UINT32) as BID_UINT64 + PM2 + (PL >> 32)) as BID_UINT64;

    BID_UINT128::new(PH + (PM >> 32), (PM << 32) + ((PL as BID_UINT32) as BID_UINT64))
}

// 64x64-bit product
#[inline]
pub (crate) fn __mul_64x64_to_128HIGH(CX64: BID_UINT64, CY64: BID_UINT64) -> BID_UINT64 {
    let CXH: BID_UINT64    = CX64 >> 32;
    let CXL: BID_UINT64    = (CX64 as BID_UINT32) as BID_UINT64;
    let CYH: BID_UINT64    = CY64 >> 32;
    let CYL: BID_UINT64    = (CY64 as BID_UINT32) as BID_UINT64;
    let mut PM: BID_UINT64 = CXH*CYL;
    let mut PH: BID_UINT64 = CXH*CYH;
    let PL:BID_UINT64      = CXL * CYL;
    let PM2: BID_UINT64    = CXL * CYH;

    PH += PM >> 32;
    PM  = ((PM as BID_UINT32) as BID_UINT64 + PM2 + (PL >> 32)) as BID_UINT64;

    PH + (PM >> 32) // P64
}

#[inline]
pub (crate) fn __mul_64x192_to_256(lA: BID_UINT64, lB: &BID_UINT192) -> BID_UINT256 {
    let mut lC: BID_UINT64;
    let mut lP: BID_UINT256 = Default::default();
    let lP0: BID_UINT128    = __mul_64x64_to_128(lA, lB.w[0]);
    let lP1: BID_UINT128    = __mul_64x64_to_128(lA, lB.w[1]);
    let lP2: BID_UINT128    = __mul_64x64_to_128(lA, lB.w[2]);
    lP.w[0]       = lP0.w[0];
    (lP.w[1], lC) = __add_carry_out(lP1.w[0], lP0.w[1]);
    (lP.w[2], lC) = __add_carry_in_out(lP2.w[0],lP1.w[1], lC);
    lP.w[3]       = lP2.w[1] + lC;

    lP
}

#[inline]
pub (crate) fn __mul_64x256_to_256(lA: BID_UINT64, lB: &BID_UINT256) -> BID_UINT256 {
    let mut lC: BID_UINT64;
    let mut lP: BID_UINT256 = Default::default();
    let lP0: BID_UINT128    = __mul_64x64_to_128(lA, lB.w[0]);
    let lP1: BID_UINT128    = __mul_64x64_to_128(lA, lB.w[1]);
    let lP2: BID_UINT128    = __mul_64x64_to_128(lA, lB.w[2]);
    lP.w[0]       = lP0.w[0];
    (lP.w[1], lC) = __add_carry_out(lP1.w[0], lP0.w[1]);
    (lP.w[2], lC) = __add_carry_in_out(lP2.w[0],lP1.w[1], lC);
    lP.w[3]       = lP2.w[1] + lC;

    lP
}

#[inline]
pub (crate) fn __mul_128x64_to_128(A64: BID_UINT64, B128: &BID_UINT128) -> BID_UINT128 {
    let ALBH_L: BID_UINT64    = A64 * B128.w[1];
    let mut Q128: BID_UINT128 = __mul_64x64_to_128MACH(A64, B128.w[0]);
    Q128.w[1] += ALBH_L;
    Q128
}

/// might simplify by calculating just QM2.w[0]
#[inline]
pub (crate) fn __mul_64x128_to_128(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT128 {
    let mut Ql: BID_UINT128 = Default::default();
    let ALBH: BID_UINT128   = __mul_64x64_to_128(A, B.w[1]);
    let ALBL: BID_UINT128   = __mul_64x64_to_128(A, B.w[0]);
    Ql.w[0] = ALBL.w[0];
    let QM2: BID_UINT128    = __add_128_64(&ALBH, ALBL.w[1]);
    Ql.w[1] = QM2.w[0];

    Ql
}

#[inline]
pub (crate) fn __mul_64x256_to_320(A: BID_UINT64, B: &BID_UINT256) -> BID_UINT512 {
    let mut lC: BID_UINT64;
    let mut P: BID_UINT512 = BID_UINT512::default();
    let lP0: BID_UINT128   = __mul_64x64_to_128(A, B.w[0]);
    let lP1: BID_UINT128   = __mul_64x64_to_128(A, B.w[1]);
    let lP2: BID_UINT128   = __mul_64x64_to_128(A, B.w[2]);
    let lP3: BID_UINT128   = __mul_64x64_to_128(A, B.w[3]);
    P.w[0] = lP0.w[0];
    (P.w[1], lC) = __add_carry_out(lP1.w[0],lP0.w[1]);
    (P.w[2], lC) = __add_carry_in_out(lP2.w[0],lP1.w[1], lC);
    (P.w[3], lC) = __add_carry_in_out(lP3.w[0],lP2.w[1], lC);
    P.w[4] = lP3.w[1] + lC;

    P
}

#[inline]
pub (crate) fn __mul_192x192_to_384(A: &BID_UINT192, B: &BID_UINT192) -> BID_UINT384 {
    let mut CY: BID_UINT64;
    let mut P: BID_UINT384 = BID_UINT384::default();
    let P0: BID_UINT256    = __mul_64x192_to_256(A.w[0], B);
    let P1: BID_UINT256    = __mul_64x192_to_256(A.w[1], B);
    let P2: BID_UINT256    = __mul_64x192_to_256(A.w[2], B);
    P.w[0] = P0.w[0];
    (P.w[1], CY) = __add_carry_out(P1.w[0], P0.w[1]);
    (P.w[2], CY) = __add_carry_in_out(P1.w[1], P0.w[2], CY);
    (P.w[3], CY) = __add_carry_in_out(P1.w[2], P0.w[3], CY);
    P.w[4] = P1.w[3] + CY;
    (P.w[2], CY) = __add_carry_out(P2.w[0], P.w[2]);
    (P.w[3], CY) = __add_carry_in_out(P2.w[1], P.w[3],CY);
    (P.w[4], CY) = __add_carry_in_out(P2.w[2], P.w[4],CY);
    P.w[5] = P2.w[3] + CY;

    P
}

// Full 128x128-bit product
#[inline]
pub (crate) fn __sqr128_to_256(P256: &mut BID_UINT256, A: &BID_UINT128) {
    // BID_UINT128 Qll, Qlh, Qhh;
    let TMP_C1: BID_UINT64;
    let TMP_C2: BID_UINT64;

    let mut Qhh: BID_UINT128 = __mul_64x64_to_128(A.w[1], A.w[1]);
    let mut Qlh: BID_UINT128 = __mul_64x64_to_128(A.w[0], A.w[1]);

    Qhh.w[1] += Qlh.w[1] >> 63;
    Qlh.w[1]  = (Qlh.w[1] + Qlh.w[1]) | (Qlh.w[0] >> 63);
    Qlh.w[0] += Qlh.w[0];

    let Qll: BID_UINT128 = __mul_64x64_to_128(A.w[0], A.w[0]);

    (P256.w[1], TMP_C1) = __add_carry_out(Qlh.w[0], Qll.w[1]);
    P256.w[0]           = Qll.w[0];
    (P256.w[2], TMP_C2) = __add_carry_in_out(Qlh.w[1], Qhh.w[0], TMP_C1);
    P256.w[3]           = Qhh.w[1]+TMP_C2;
}

#[inline]
pub (crate) fn __mul_64x320_to_512(A: BID_UINT64, B: &BID_UINT512) -> BID_UINT512 {
    let mut P: BID_UINT512 = BID_UINT512::default();
    let mut lC: BID_UINT64;
	let lP0: BID_UINT128 = __mul_64x64_to_128(A, B.w[0]);
	let lP1: BID_UINT128 = __mul_64x64_to_128(A, B.w[1]);
	let lP2: BID_UINT128 = __mul_64x64_to_128(A, B.w[2]);
	let lP3: BID_UINT128 = __mul_64x64_to_128(A, B.w[3]);
	let lP4: BID_UINT128 = __mul_64x64_to_128(A, B.w[4]);
	P.w[0] = lP0.w[0];
	(P.w[1], lC) = __add_carry_out(lP1.w[0],lP0.w[1]);
	(P.w[2], lC) = __add_carry_in_out(lP2.w[0],lP1.w[1],lC);
	(P.w[3], lC) = __add_carry_in_out(lP3.w[0],lP2.w[1],lC);
	(P.w[4], lC) = __add_carry_in_out(lP4.w[0],lP3.w[1],lC);
	P.w[5]       = lP4.w[1] + lC;

	P
}

#[inline]
pub (crate) fn __mul_256x256_to_512(A: &BID_UINT256, B: &BID_UINT256) -> BID_UINT512 {
    let mut CY: BID_UINT64;
    let mut P: BID_UINT512 = BID_UINT512::default();
    let P0: BID_UINT512    = __mul_64x256_to_320(A.w[0], B);
    let P1: BID_UINT512    = __mul_64x256_to_320(A.w[1], B);
    let P2: BID_UINT512    = __mul_64x256_to_320(A.w[2], B);
    let P3: BID_UINT512    = __mul_64x256_to_320(A.w[3], B);
    P.w[0] = P0.w[0];
    (P.w[1], CY) = __add_carry_out(P1.w[0], P0.w[1]);
    (P.w[2], CY) = __add_carry_in_out(P1.w[1], P0.w[2], CY);
    (P.w[3], CY) = __add_carry_in_out(P1.w[2], P0.w[3], CY);
    (P.w[4], CY) = __add_carry_in_out(P1.w[3], P0.w[4], CY);
    P.w[5] = P1.w[4] + CY;
    (P.w[2], CY) = __add_carry_out(P2.w[0], P.w[2]);
    (P.w[3], CY) = __add_carry_in_out(P2.w[1], P.w[3], CY);
    (P.w[4], CY) = __add_carry_in_out(P2.w[2], P.w[4], CY);
    (P.w[5], CY) = __add_carry_in_out(P2.w[3], P.w[5], CY);
    P.w[6] = P2.w[4] + CY;
    (P.w[3], CY) = __add_carry_out(P3.w[0], P.w[3]);
    (P.w[4], CY) = __add_carry_in_out(P3.w[1], P.w[4], CY);
    (P.w[5], CY) = __add_carry_in_out(P3.w[2], P.w[5], CY);
    (P.w[6], CY) = __add_carry_in_out(P3.w[3], P.w[6], CY);
    P.w[7] = P3.w[4] + CY;

    P
}

#[inline]
pub (crate) fn __mul_64x128_short(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT128 {
	let ALBH_L: BID_UINT64  = __mul_64x64_to_64(A,B.w[1]);
	let mut Ql: BID_UINT128 = __mul_64x64_to_128(A, B.w[0]);

	Ql.w[1] += ALBH_L;

	Ql
}

//////////////////////////////////////////////
// Compare Macros
//////////////////////////////////////////////

/// greater than
///  return 0 if A<=B
///  non-zero if A>B
#[inline]
pub (crate) fn __unsigned_compare_gt_128(A: &BID_UINT128, B: &BID_UINT128) -> bool  {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] > B.w[0]))
}

#[inline]
pub (crate) fn __unsigned_compare_gt_128_256(A: &BID_UINT128, B: &BID_UINT256) -> bool  {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] > B.w[0]))
}

#[inline]
pub (crate) fn __unsigned_compare_gt_256_128(A: &BID_UINT256, B: &BID_UINT128) -> bool  {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] > B.w[0]))
}

#[inline]
pub (crate) fn __unsigned_compare_gt_256_as_128(A: &BID_UINT256, B: &BID_UINT256) -> bool  {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] > B.w[0]))
}

/// greater-or-equal
#[inline]
pub (crate) fn __unsigned_compare_ge_128(A: &BID_UINT128, B: &BID_UINT128) -> bool {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] >= B.w[0]))
}

#[inline]
pub (crate) fn __unsigned_compare_ge_256_128(A: &BID_UINT256, B: &BID_UINT128) -> bool {
    (A.w[1] > B.w[1]) || ((A.w[1] == B.w[1]) && (A.w[0] >= B.w[0]))
}

#[inline]
pub (crate) fn __test_equal_128(A: &BID_UINT128, B: &BID_UINT128) -> bool {
    (A.w[1] == B.w[1]) && (A.w[0] == B.w[0])
}
