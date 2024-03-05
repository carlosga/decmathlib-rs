/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

//! A 128-bit decimal floating point type (IEEE Standard 754-2008 compliant).

#![allow(non_camel_case_types)]

use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, LowerExp, UpperExp};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;
use forward_ref::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};

use crate::bid128_add::{bid128_add, bid128_sub, bid64dq_add};
use crate::bid128_compare::*;
use crate::bid128_div::bid128_div;
use crate::bid128_fdim::bid128_fdim;
use crate::bid128_fma::{bid128_fma, bid128ddd_fma, bid128dqd_fma, bid128qdq_fma, bid128qqd_fma};
use crate::bid128_fmod::bid128_fmod;
use crate::bid128_frexp::bid128_frexp;
use crate::bid128_ilogb::bid128_ilogb;
use crate::bid128_ldexp::bid128_ldexp;
use crate::bid128_llquantexp::bid128_llquantexp;
use crate::bid128_llrint::bid128_llrint;
use crate::bid128_llround::bid128_llround;
use crate::bid128_logb::bid128_logb;
use crate::bid128_lrint::bid128_lrint;
use crate::bid128_lround::bid128_lround;
use crate::bid128_minmax::{bid128_maxnum, bid128_maxnum_mag, bid128_minnum, bid128_minnum_mag};
use crate::bid128_modf::bid128_modf;
use crate::bid128_mul::bid128_mul;
use crate::bid128_nearbyint::bid128_nearbyint;
use crate::bid128_next::{bid128_nextafter, bid128_nextdown, bid128_nextup};
use crate::bid128_nexttoward::bid128_nexttoward;
use crate::bid128_noncomp::*;
use crate::bid128_quantexp::bid128_quantexp;
use crate::bid128_quantize::bid128_quantize;
use crate::bid128_quantum::bid128_quantum;
use crate::bid128_rem::bid128_rem;
use crate::bid128_round_integral::{bid128_round_integral_exact, bid128_round_integral_nearest_away, bid128_round_integral_nearest_even, bid128_round_integral_negative, bid128_round_integral_positive, bid128_round_integral_zero};
use crate::bid128_scalbln::bid128_scalbln;
use crate::bid128_scalbn::bid128_scalbn;
use crate::bid128_sqrt::bid128_sqrt;
use crate::bid128_string::{bid128_from_string, bid128_to_string};
use crate::bid128_to_int32::*;
use crate::bid128_to_int64::*;
use crate::bid128_to_uint32::*;
use crate::bid128_to_uint64::*;
use crate::bid64_to_bid128::{bid128_to_bid64, bid64_to_bid128};
use crate::bid_dpd::{bid_dpd_to_bid128, bid_to_dpd128};
use crate::bid_from_int::{bid128_from_int32, bid128_from_int64, bid128_from_uint32, bid128_from_uint64};
use crate::bid_internal::*;
use crate::d64::d64;

/// A classification of decimal floating point numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassTypes {
    /// Signaling NaN (not a number).
    SignalingNaN,

    /// Quiet NaN (not a number).
    QuietNaN,

    /// Negative infinity.
    NegativeInfinity,

    /// Negative normal.
    NegativeNormal,

    /// Negative subnormal.
    NegativeSubnormal,

    /// Negative zero.
    NegativeZero,

    /// Positive zero.
    PositiveZero,

    /// Positive subnormal.
    PositiveSubnormal,

    /// Positive normal.
    PositiveNormal,

    /// Positive infinity.
    PositiveInfinity
}

/// Rounding mode.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RoundingMode {
    /// Rounding towards nearest representable value.
    BID_ROUNDING_TO_NEAREST  = 0x00000,

    /// Rounding towards negative infinity.
    BID_ROUNDING_DOWN        = 0x00001,

    /// Rounding towards positive infinity.
    BID_ROUNDING_UP          = 0x00002,

    /// Rounding towards zero.
    BID_ROUNDING_TO_ZERO     = 0x00003,

    /// Rounding towards the nearest value, breaks ties by rounding away from zero.
    BID_ROUNDING_TIES_AWAY   = 0x00004
}

impl From<u32> for RoundingMode {
    fn from(value: u32) -> Self {
        match value {
            0x00000 => RoundingMode::BID_ROUNDING_TO_NEAREST,
            0x00001 => RoundingMode::BID_ROUNDING_DOWN,
            0x00002 => RoundingMode::BID_ROUNDING_UP,
            0x00003 => RoundingMode::BID_ROUNDING_TO_ZERO,
            0x00004 => RoundingMode::BID_ROUNDING_TIES_AWAY,
            _ => panic!("Unknown rounding mode")
        }
    }
}

pub const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::BID_ROUNDING_TO_NEAREST;

/// Status flags.
pub struct StatusFlags;

// BID_FPSC
pub type _IDEC_flags = u32;

impl StatusFlags {
    pub const BID_INEXACT_EXCEPTION: _IDEC_flags            = DEC_FE_INEXACT;
    pub const BID_UNDERFLOW_EXCEPTION: _IDEC_flags          = DEC_FE_UNDERFLOW;
    pub const BID_OVERFLOW_EXCEPTION: _IDEC_flags           = DEC_FE_OVERFLOW;
    pub const BID_ZERO_DIVIDE_EXCEPTION: _IDEC_flags        = DEC_FE_DIVBYZERO;
    pub const BID_DENORMAL_EXCEPTION: _IDEC_flags           = DEC_FE_UNNORMAL;
    pub const BID_INVALID_EXCEPTION: _IDEC_flags            = DEC_FE_INVALID;
    pub const BID_UNDERFLOW_INEXACT_EXCEPTION: _IDEC_flags  = DEC_FE_UNDERFLOW | DEC_FE_INEXACT;
    pub const BID_OVERFLOW_INEXACT_EXCEPTION: _IDEC_flags   = DEC_FE_OVERFLOW | DEC_FE_INEXACT;
    pub const BID_EXACT_STATUS:_IDEC_flags                  = 0x00000000;
}

/// The 128-bit decimal type.
#[derive(Copy, Clone)]
#[repr(align(16))]
pub struct d128 {
    pub (crate) w: [BID_UINT64; 2]
}

/// The radix or base of the internal representation of `d128`.
pub const RADIX: i32 = 10;

/// The number minus one (-1).
pub const MINUS_ONE: d128 = d128 { w: [0x0000000000000001u64, 0xb040000000000000u64] };

/// The number zero (0).
pub const ZERO: d128 = d128 { w: [0x0000000000000000u64, 0x3040000000000000u64] };

/// The number one (1).
pub const ONE: d128 = d128 { w: [0x0000000000000001u64, 0x3040000000000000u64] };

/// Not a Number (NaN).
pub const NAN: d128 = d128 { w: [0x0000000000000000u64, 0x7c00000000000000u64] };

/// Negative Not a Number (Nan).
pub const NEG_NAN: d128 = d128 { w: [0x0000000000000000u64, 0xFC00000000000000u64] };

/// Signaling Not a Number (Nan).
pub const SNAN: d128 = d128 { w: [0x0000000000000000u64, 0x7E00000000000000u64] };

/// Negative signaling Not a Number (Nan).
pub const NEG_SNAN: d128 = d128 { w: [0x0000000000000000u64, 0xFE00000000000000u64] };

/// Infinity.
pub const INFINITY: d128 = d128 { w: [0x0000000000000000u64, 0x7800000000000000u64] };

/// Negative Infinity.
pub const NEGATIVE_INFINITY: d128 = d128 { w: [0x0000000000000000u64, 0xF800000000000000u64] };

/// The number of digits in the coefficient.
pub const MANTISSA_DIGITS: u32 = 34;

/// The difference between 1 and the least value greater than 1 that is representable in the given floating point type (1E-33).
pub const EPSILON: d128 = d128 { w: [0x1u64, 0x2FFE000000000000u64] };

/// Smallest finite value (1E-6143).
pub const MIN: d128 = d128 { w: [0x1u64, 0x42000000000000u64] };

/// Largest finite value (9.999999999999999999999999999999999E6144).
pub const MAX: d128 = d128 { w: [0x378D8E63FFFFFFFFu64, 0x5FFFED09BEAD87C0u64] };

/// The minimum exponent.
pub const MIN_EXP: i32 = -6142;

/// The maximum exponent.
pub const MAX_EXP: i32 = 6145;

/// Macro to simplify the creation of instances of 128-bit decimal floating point numbers.
#[macro_export]
macro_rules! dec128 {
    ($t:tt) => {{
        use std::str::FromStr;
        $crate::d128::d128::from_str(stringify!($t)).expect("Invalid decimal number literal")
    }}
}

impl Default for d128 {
    /// Returns the default value of 0.
    #[must_use]
    fn default() -> Self {
        Self::new(0x3040000000000000u64, 0x0)
    }
}

impl d128 {
    #[must_use]
    pub (crate) fn new(h: u64, l: u64) -> Self {
        #[cfg(target_endian = "big")]
        return Self { w: [h, l] };

        #[cfg(target_endian = "little")]
        return Self { w: [l, h] };
    }

    /// Convert a 128-bit decimal floating-point value encoded in BID format
    /// to the same value encoded in DPD
    pub fn encode_decimal(&self) -> Self {
        bid_to_dpd128(self)
    }

    /// Convert a 128-bit decimal floating-point value encoded in DPD format
    /// to the same value encoded in BID format
    pub fn decode_decimal(&self) -> Self {
        bid_dpd_to_bid128(self)
    }

    /// Copies a 128-bit decimal floating-point operand x to a destination in the same format, changing the sign to positive
    #[must_use]
    pub fn abs(&self) -> Self {
        bid128_abs(self)
    }

    /// Tells which of the following ten classes x falls into (details in the IEEE Standard 754-2008):
    /// signalingNaN, quietNaN, negativeInfinity, negativeNormal, negativeSubnormal, negativeZero, positiveZero,
    /// positiveSubnormal, positiveNormal, positiveInfinity
    #[must_use]
    pub fn class(&self) -> ClassTypes {
        bid128_class(self)
    }

    /// Copies a decimal floating-point operand x to a destination in the same format, with no change
    #[must_use]
    pub fn copy(&self) -> Self { bid128_copy(self) }

    /// Copies a 128-bit decimal floating-point operand x to a destination in the same format as x, but with the sign of y
    #[must_use]
    pub fn copy_sign(&self, other: &Self) -> Self { bid128_copy_sign(self, other) }

    /// Return true if and only if x is a finite number, infinity, or NaN that is canonical
    #[must_use]
    pub fn is_canonical(&self) -> bool {
        bid128_is_canonical(self)
    }

    /// Return true if and only if x is zero, subnormal or normal (not infinite or NaN)
    #[must_use]
    pub fn is_finite(&self) -> bool {
        bid128_is_finite(self)
    }

    /// Return true if and only if x is infinite
    #[must_use]
    pub fn is_infinite(&self) -> bool {
        bid128_is_inf(self)
    }

    /// Return true if and only if x is a NaN
    #[must_use]
    pub fn is_nan(&self) -> bool {
        bid128_is_nan(self)
    }

    /// Return true if and only if x is normal (not zero, subnormal, infinite, or NaN)
    #[must_use]
    pub fn is_normal(&self) -> bool {
        bid128_is_normal(self)
    }

    /// Return true if and only if x is a signaling NaN
    #[must_use]
    pub fn is_signaling(&self) -> bool {
        bid128_is_signaling(self)
    }

    /// Return true if and only if x has negative sign
    #[must_use]
    pub fn is_sign_minus(&self) -> bool {
        bid128_is_signed(self)
    }

    /// Return true if and only if x is subnormal
    #[must_use]
    pub fn is_subnormal(&self) -> bool {
        bid128_is_subnormal(self)
    }

    /// Return true if and only if x is +0 or -0
    #[must_use]
    pub fn is_zero(&self) -> bool {
        bid128_is_zero(self)
    }

    /// Copies a 128-bit decimal floating-point operand x to a destination
    /// in the same format, reversing the sign
    #[must_use]
    pub fn negate(x: &Self) -> Self {
        bid128_negate(x)
    }

    /// `same_quantum` is true if the exponents of x and y are the same,
    /// and false otherwise; `same_quantum(NaN, NaN)` and `same_quantum(inf, inf)` are
    /// true; if exactly one operand is infinite or exactly one operand is NaN,
    /// sameQuantum is false
    #[must_use]
    pub fn same_quantum(x: &Self, y: &Self) -> bool {
        bid128_same_quantum(x, y)
    }

    /// Return true if the absolute values of x and y are ordered (see the IEEE Standard 754-2008)
    #[must_use]
    pub fn total_order(x: &Self, y: &Self) -> bool {
        bid128_total_order(x, y)
    }

    /// Return true if the absolute values of x and y are ordered (see the IEEE Standard 754-2008)
    #[must_use]
    pub fn total_order_mag(x: &Self, y: &Self) -> bool {
        bid128_total_order_mag(x, y)
    }

    #[must_use]
    pub fn nan(tagp: &str, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
        bid128_nan(tagp, pfpsf)
    }

    /// Convert a decimal floating-point value represented in string format
    /// (decimal character sequence) to 128-bit decimal floating-point format (binary encoding)
    #[must_use]
    pub fn convert_from_decimal_character(value: &str, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_from_string(value, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Convert 64-bit decimal floating-point value to 128-bit decimal floating-point format (binary encoding)
    #[must_use]
    pub fn convert_from_decimal64(bid: d64, status: &mut _IDEC_flags) -> Self {
        bid64_to_bid128(bid.0, status)
    }

    /// fdim returns x - y if x > y, and +0 is x <= y
    #[must_use]
    pub fn fdim(&self, rhs: &BID_UINT128, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_fdim(self, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Computes x * y + z as if to infinite precision and rounded only once to fit the result type.
    #[must_use]
    pub fn fused_multiply_add(x: &Self, y: &Self, z: &Self, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_fma(x, y, z, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Decimal floating-point fused multiply-add, UINT64 * UINT64 + UINT64 -> UINT128
    #[must_use]
    pub fn ddd_fma(x: d64, y: d64, z: d64, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128ddd_fma(x.0, y.0, z.0, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Decimal floating-point fused multiply-add d64 * d128 + d64 -> d128
    #[must_use]
    pub fn dqd_fma(x: d64, y: &Self, z: d64, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128dqd_fma(x.0, y, z.0, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Decimal floating-point fused multiply-add, d128 * d64 + d128 -> d128
    #[must_use]
    pub fn qdq_fma(x: &Self, y: d64, z: &Self, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128qdq_fma(x, y.0, z, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Decimal floating-point fused multiply-add, UINT128 * UINT128 + UINT64
    #[must_use]
    pub fn qqd_fma(x: &Self, y: &Self, z: d64, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128qqd_fma(x, y, z.0, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Computes the decimal floating point remainder of the division operation x / y.
    #[must_use]
    pub fn fmod(&self, rhs: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_fmod(self, rhs, pfpsf)
    }

    /// Decomposes given decimal floating point value num into a normalized fraction and an integral power of two.
    #[must_use]
    pub fn frexp(&self) -> (Self, i32) {
        bid128_frexp(self)
    }

    /// multiply a 128-bit decimal floating-point value by an integral power of 2.
    #[must_use]
    pub fn ldexp(&self, n: i32, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_ldexp(self, n, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// The `llquantexp()` functions return the quantum exponent of x.
    #[must_use]
    pub fn llquantexp(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_llquantexp(self, pfpsf)
    }

    /// Returns the exponent e of x, a signed integral value, determined
    /// as though x were represented with infinite range and minimum exponent
    #[must_use]
    pub fn logb(&self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_logb(self, pfpsf)
    }

    /// The lrint function rounds its argument to the nearest integer value of
    /// type long int, rounding according to the current rounding direction.
    #[must_use]
    pub fn lrint(&self, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_lrint(self, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// The llrint function rounds its argument to the nearest integer value of
    /// type long long int, rounding according to the current rounding direction.
    #[must_use]
    pub fn llrint(&self, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_llrint(self, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// The lround function rounds its argument to the nearest integer value of
    /// type long int, using rounding to nearest-away
    #[must_use]
    pub fn lround(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_lround(self, pfpsf)
    }

    /// The llround function rounds its argument to the nearest integer value of
    /// type long int, using rounding to nearest-away
    #[must_use]
    pub fn llround(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_llround(self, pfpsf)
    }

    /// Returns the exponent e of x, a signed integral value, determined
    /// as though x were represented with infinite range and minimum exponent
    #[must_use]
    pub fn log_b(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_ilogb(self, pfpsf)
    }

    /// Returns the canonicalized floating-point number y if x < y,
    /// x if y < x, the canonicalized floating-point number if one operand is a
    /// floating-point number and the other a quiet NaN.
    #[must_use]
    pub fn max_num(x: &Self, y: &Self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_maxnum(x, y, pfpsf)
    }

    /// Returns the canonicalized floating-point number x if |x| > |y|,
    /// y if |y| > |x|, otherwise this function is identical to `max`
    #[must_use]
    pub fn max_num_mag(x: &Self, y: &Self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_maxnum_mag(x, y, pfpsf)
    }

    /// Returns the canonicalized floating-point number x if x < y,
    /// y if y < x, the canonicalized floating-point number if one operand is
    /// a floating-point number and the other a quiet NaN. Otherwise it is either x or y, canonicalized.
    #[must_use]
    pub fn min_num(x: &Self, y: &Self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_minnum(x, y, pfpsf)
    }

    /// Returns the canonicalized floating-point number x if |x| < |y|,
    /// y if |y| < |x|, otherwise this function is identical to `min`
    #[must_use]
    pub fn min_num_mag(x: &Self, y: &Self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_minnum_mag(x, y, pfpsf)
    }

    /// Decomposes given decimal floating point value num into integral and fractional parts.
    #[must_use]
    pub fn modf(&self, pfpsf: &mut _IDEC_flags) -> (Self, Self) {
        bid128_modf(self, pfpsf)
    }

    /// Rounds the decimal floating-point value num to an integer value in decicmal floating-point format, using the given rounding mode.
    #[must_use]
    pub fn nearbyint(&self, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_nearbyint(self, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Returns the next 128-bit decimal floating-point number that neighbors
    /// the first operand in the direction toward the second operand
    #[must_use]
    pub fn next_after(x: &Self, y:&Self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_nextafter(x, y, pfpsf)
    }

    /// Returns the greatest 128-bit decimal floating-point number that
    /// compares less than the operand
    #[must_use]
    pub fn next_down(&self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_nextdown(self, pfpsf)
    }

    /// Returns the next representable value after x in the direction of y.
    #[must_use]
    pub fn next_toward(x: &Self, y: &Self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_nexttoward(x, y, pfpsf)
    }

    /// Returns the least 128-bit decimal floating-point number that
    /// compares greater than the operand
    #[must_use]
    pub fn next_up(&self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_nextup(self, pfpsf)
    }

    /// The `quantexp()` functions return the quantum exponent of x.
    #[must_use]
    pub fn quantexp(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_quantexp(self, pfpsf)
    }

    /// `quantize` is a floating-point number in the same format that
    /// has, if possible, the same numerical value as x and the same quantum
    /// (unit-in-the-last-place) as y. If the exponent is being increased, rounding
    /// according to the prevailing rounding-direction mode might occur: the result
    /// is a different floating-point representation and inexact is signaled if the
    /// result does not have the same numerical value as x. If the exponent is being
    /// decreased and the significand of the result would have more than 34 digits,
    /// invalid is signaled and the result is NaN. If one or both operands are NaN
    /// the rules for NaNs are followed. Otherwise if only one operand is
    /// infinite then invalid is signaled and the result is NaN. If both operands
    /// are infinite then the result is canonical infinity with the sign of x
    #[must_use]
    pub fn quantize(x: &Self, y: &Self, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_quantize(x, y, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// The `quantum` functions compute the quantum of a finite argument.
    /// If x is infinite, the result is +Inf. If x is NaN, the result is NaN.
    #[must_use]
    pub fn quantum(&self) -> Self {
        bid128_quantum(self)
    }

    #[allow(unused)]
    #[must_use]
    pub (crate) fn scale(&self) -> i32 {
        let mut x_exp: BID_UINT64;
        let mut exp: i32;   // unbiased exponent

        #[cfg(target_endian = "big")]
        let mut x = *x;

        #[cfg(target_endian = "big")]
        BID_SWAP128(&mut x);

        // check for NaN or Infinity
        if ((self.w[1] & MASK_COEFF) == 0x0u64) && (self.w[0] == 0x0u64) {
            exp = (((self.w[1] & MASK_EXP) >> 49) - 6176) as i32;

            if exp > (((0x5ffe) >> 1) - (6176)) {
                exp = ((((self.w[1] << 2) & MASK_EXP) >> 49) as i32) - 6176;
            }

            exp
        } else { // x is not special and is not zero
            // unpack x
            x_exp = self.w[1] & MASK_EXP;             // biased and shifted left 49 bit positions

            if (self.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
                x_exp = (self.w[1] << 2) & MASK_EXP;   // biased and shifted left 49 bit positions
            }

            ((x_exp >> 49) - 6176) as i32
        }
    }

    /// Returns x * 10^N
    #[must_use]
    pub fn scaleb(&self, n: i32, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_scalbn(self, n, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Returns x * 10^N
    #[must_use]
    pub fn scalebln(&self, n: i64, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_scalbln(self, n, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Decimal floating-point square root
    #[must_use]
    pub fn square_root(&self, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_sqrt(self, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit decimal floating-point format (binary encoding)
    #[must_use]
    pub fn convert_to_decimal64(&self, rnd_mode: Option<RoundingMode>, status: &mut _IDEC_flags) -> d64 {
        d64(bid128_to_bid64(self, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status))
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed
    /// integer in rounding-to-nearest-even mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i32_ties_to_even(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_rnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-nearest-even mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_i32_exact_ties_to_even(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xrnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-down mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i32_toward_negative(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_floor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-down mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_i32_exact_toward_negative(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xfloor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-up mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i32_toward_positive(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_ceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-up mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_i32_exact_toward_positive(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-zero; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i32_toward_zero(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_int(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-zero; inexact exceptions signale
    #[must_use]
    pub fn convert_to_i32_exact_toward_zero(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-nearest-away; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i32_ties_to_away(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_rninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-nearest-away; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_i32_exact_ties_to_away(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xrninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-up mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i64_toward_positive(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_ceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-down mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i64_toward_negative(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_floor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-to-zero; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i64_toward_zero(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_int(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed
    /// integer in rounding-to-nearest-even mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i64_ties_to_even(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_rnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed
    /// integer in rounding-to-nearest-away; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_i64_ties_to_away(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_rninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-up mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_i64_exact_toward_positive(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-down mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_i64_exact_toward_negative(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xfloor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-to-zero; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_i64_exact_toward_zero(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed
    /// integer in rounding-to-nearest-even mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_i64_exact_ties_to_even(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xrnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed
    /// integer in rounding-to-nearest-away; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_i64_exact_ties_to_away(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xrninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-up mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u32_toward_positive(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_ceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-down mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u32_toward_negative(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_floor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-to-zero; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u32_toward_zero(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_int(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-to-nearest-even mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u32_ties_to_even(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_rnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-to-nearest-away; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u32_ties_to_away(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_rninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-up mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u32_exact_toward_positive(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_xceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-down mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u32_exact_toward_negative(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_xfloor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-to-zero; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u32_exact_toward_zero(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_xint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-to-nearest-even mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u32_exact_ties_to_even(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_xrnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit unsigned
    /// integer in rounding-to-nearest-away; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u32_exact_ties_to_away(&self, pfpsf: &mut _IDEC_flags) -> u32 {
        bid128_to_uint32_xrninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-up mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u64_toward_positive(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_ceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-down mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u64_toward_negative(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_floor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-to-zero; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u64_toward_zero(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_int(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-to-nearest-even mode; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u64_ties_to_even(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_rnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-to-nearest-away; inexact exceptions not signaled
    #[must_use]
    pub fn convert_to_u64_ties_to_away(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_rninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-up mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u64_exact_toward_positive(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_xceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-down mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u64_exact_toward_negative(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_xfloor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-to-zero; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u64_exact_toward_zero(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_xint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-to-nearest-even mode; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u64_exact_ties_to_even(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_xrnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit unsigned
    /// integer in rounding-to-nearest-away; inexact exceptions signaled
    #[must_use]
    pub fn convert_to_u64_exact_ties_to_away(&self, pfpsf: &mut _IDEC_flags) -> u64 {
        bid128_to_uint64_xrninta(self, pfpsf)
    }

    /// Decimal floating-point addition, d128 + d128 -> d128
    #[must_use]
    pub fn addition(lhs: &Self, rhs: &Self, rnd_mode: Option<RoundingMode>, status: &mut _IDEC_flags) -> Self {
        bid128_add(lhs, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status)
    }

    /// Decimal floating-point addition, UINT64 + UINT128 -> UINT64
    #[must_use]
    pub fn add_dq(lhs: &d64, rhs: &Self, rnd_mode: Option<RoundingMode>, status: &mut _IDEC_flags) -> d64 {
        let res = bid64dq_add(lhs.0, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status);

        d64(res)
    }

    /// Decimal floating-point division, d128 / d128 -> d128
    #[must_use]
    pub fn division(lhs: &Self, rhs: &Self, rnd_mode: Option<RoundingMode>, status: &mut _IDEC_flags) -> Self {
        bid128_div(lhs, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status)
    }

    /// Decimal floating-point multiplication, d128 / d128 -> d128
    #[must_use]
    pub fn multiplication(lhs: &Self, rhs: &Self, rnd_mode: Option<RoundingMode>, status: &mut _IDEC_flags) -> Self {
        bid128_mul(lhs, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status)
    }

    /// Decimal floating-point remainder
    #[must_use]
    pub fn remainder(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> Self {
        bid128_rem(lhs, rhs, status)
    }

    /// Decimal floating-point subtraction, d128 - d128 -> d128
    #[must_use]
    pub fn subtraction(lhs: &Self, rhs: &Self, rnd_mode: Option<RoundingMode>, status: &mut _IDEC_flags) -> Self {
        bid128_sub(lhs, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_equal(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_equal(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_greater(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_greater(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_unordered(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_unordered(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_ordered(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_ordered(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_greater_equal(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_greater_equal(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_less(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_less(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_less_equal(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_less_equal(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_not_equal(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_not_equal(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_not_greater(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_not_greater(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_quiet_not_less(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_not_less(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_signaling_greater(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_signaling_greater(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_signaling_greater_equal(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_signaling_greater_equal(lhs, rhs, status)
    }

    /// Compare 128-bit decimal floating-point numbers for specified relation;
    /// do not signal invalid exception for quiet NaNs
    #[must_use]
    pub fn compare_signaling_greater_unordered(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_signaling_greater_unordered(lhs, rhs, status)
    }

    /// Round 128-bit decimal floating-point value to integral-valued decimal
    /// floating-point value in the same format, using the current rounding mode;
    /// signal inexact exceptions
    #[must_use]
    pub fn round_to_integral_exact(x: &BID_UINT128, rnd_mode: Option<RoundingMode>, pfpsf: &mut _IDEC_flags) -> d128 {
        bid128_round_integral_exact(x, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Round 128-bit decimal floating-point value to integral-valued decimal
    /// floating-point value in the same format, using the rounding-to-nearest-away mode;
    /// do not signal inexact exceptions
    #[must_use]
    pub fn round_to_integral_ties_to_away(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> d128 {
        bid128_round_integral_nearest_away(x, pfpsf)
    }

    /// Round 128-bit decimal floating-point value to integral-valued decimal
    /// floating-point value in the same format, using the rounding-to-nearest-even mode;
    /// do not signal inexact exceptions
    #[must_use]
    pub fn round_to_integral_ties_to_even(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> d128 {
        bid128_round_integral_nearest_even(x, pfpsf)
    }

    /// Round 128-bit decimal floating-point value to integral-valued decimal
    /// floating-point value in the same format, using the rounding-down mode; do not
    /// signal inexact exceptions
    #[must_use]
    pub fn round_to_integral_ties_toward_negative(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> d128 {
        bid128_round_integral_negative(x, pfpsf)
    }

    /// Round 128-bit decimal floating-point value to integral-valued decimal
    /// floating-point value in the same format, using the rounding-up mode; do not
    /// signal inexact exceptions
    #[must_use]
    pub fn round_to_integral_ties_toward_positive(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> d128 {
        bid128_round_integral_positive(x, pfpsf)
    }

    /// Round 128-bit decimal floating-point value to integral-valued decimal
    /// floating-point value in the same format, using the rounding-to-zero mode;
    /// do not signal inexact exceptions
    #[must_use]
    pub fn round_to_integral_ties_toward_zero(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> d128 {
        bid128_round_integral_zero(x, pfpsf)
    }
}

impl Eq for d128 { }

impl PartialEq for d128 {
    fn eq(&self, other: &Self) -> bool {
        let mut status: _IDEC_flags = StatusFlags::BID_EXACT_STATUS;
        let s_nan: bool = self.is_nan();
        let o_nan: bool = other.is_nan();

        if s_nan && o_nan {
            bid128_quiet_unordered(self, other, &mut status)
        } else if s_nan || o_nan {
            bid128_quiet_ordered(self, other, &mut status)
        } else {
            bid128_quiet_equal(self, other, &mut status)
        }
    }
}

impl PartialOrd for d128 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut status: _IDEC_flags = StatusFlags::BID_EXACT_STATUS;
        let equal: bool = self.w[BID_HIGH_128W] == other.w[BID_HIGH_128W]
                       && self.w[BID_LOW_128W]  == other.w[BID_LOW_128W];
        if equal {
            return Some(Ordering::Equal)
        }
        let less = bid128_quiet_less(self, other, &mut status);
        if less {
            return Some(Ordering::Less);
        }
        let greater = bid128_quiet_greater(self, other, &mut status);
        if greater {
            return Some(Ordering::Greater);
        }
        None
    }

    fn lt(&self, other: &Self) -> bool {
        let mut status: _IDEC_flags = StatusFlags::BID_EXACT_STATUS;
        bid128_quiet_less(self, other, &mut status)
    }

    fn le(&self, other: &Self) -> bool {
        let mut status: _IDEC_flags = StatusFlags::BID_EXACT_STATUS;
        bid128_quiet_less_equal(self, other, &mut status)
    }

    fn gt(&self, other: &Self) -> bool {
        let mut status: _IDEC_flags = StatusFlags::BID_EXACT_STATUS;
        bid128_quiet_greater(self, other, &mut status)
    }

    fn ge(&self, other: &Self) -> bool {
        let mut status: _IDEC_flags = StatusFlags::BID_EXACT_STATUS;
        bid128_quiet_greater_equal(self, other, &mut status)
    }
}

impl Debug for d128 {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        bid128_to_string(self, fmt, true)
    }
}

impl Display for d128 {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        bid128_to_string(self, fmt, true)
    }
}

impl LowerExp for d128 {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        bid128_to_string(self, fmt, false)
    }
}

impl UpperExp for d128 {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        bid128_to_string(self, fmt, true)
    }
}

impl FromStr for d128 {
    type Err = u32;

    /// Converts a decimal string value to decimal128.
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// let dec1 = decmathlib_rs::d128::d128::from_str("+100000.00000000E6107");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
         let mut status: _IDEC_flags = 0;
         let dec: BID_UINT128 = bid128_from_string(s, DEFAULT_ROUNDING_MODE, &mut status);

         match status {
            StatusFlags::BID_EXACT_STATUS | StatusFlags::BID_INEXACT_EXCEPTION => Ok(dec),
            _ => Err(status)
         }
    }
}

impl From<i32> for d128 {
    /// Converts i32 to decimal128.
    /// # Examples
    ///
    /// ```
    /// let value = 0;
    /// let dec1 = decmathlib_rs::d128::d128::from(value);
    /// ```
    fn from(value: i32) -> Self {
        bid128_from_int32(value)
    }
}

impl From<u32> for d128 {
    /// Converts u32 to decimal128.
    /// # Examples
    ///
    /// ```
    /// let value = 0;
    /// let dec1 = decmathlib_rs::d128::d128::from(value);
    /// ```
    fn from(value: u32) -> Self {
        bid128_from_uint32(value)
    }
}

impl From<i64> for d128 {
    /// Converts i64 to decimal128.
    /// # Examples
    ///
    /// ```
    /// let value = 0;
    /// let dec1 = decmathlib_rs::d128::d128::from(value);
    /// ```
    fn from(value: i64) -> Self {
        bid128_from_int64(value)
    }
}

impl From<u64> for d128 {
    /// Converts i64 to decimal128.
    /// # Examples
    ///
    /// ```
    /// let value = 0;
    /// let dec1 = decmathlib_rs::d128::d128::from(value);
    /// ```
    fn from(value: u64) -> Self {
        bid128_from_uint64(value)
    }
}

impl From<d64> for d128 {
    /// Converts decimal64 to decimal128.
    /// # Examples
    ///
    /// ```
    /// let dec64 = decmathlib_rs::d64::d64(0x002462d53c8abac0u64);
    /// let dec1 = decmathlib_rs::d128::d128::from(dec64);
    /// ```
    fn from(value: d64) -> Self {
        let mut status: _IDEC_flags = 0;
        bid64_to_bid128(value.0, &mut status)
    }
}

impl TryInto<d64> for d128 {
    type Error = _IDEC_flags;

    /// Tries to convert decimal128 to decimal64
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::d64::d64;
    /// use decmathlib_rs::d128::{_IDEC_flags, d128};
    /// let res: d128 = decmathlib_rs::d128::d128::from(0x2cffed09bead87c0378d8e63ffffffffu128);
    /// let dec64: Result<d64, _IDEC_flags> = res.try_into();
    /// ```
    fn try_into(self) -> Result<d64, Self::Error> {
         let mut status: _IDEC_flags = 0;
         let dec64: BID_UINT64 = bid128_to_bid64(&self, DEFAULT_ROUNDING_MODE, &mut status);

         match status {
            0 => Ok(d64(dec64)),
            _ => Err(status)
         }
    }
}

impl From<u128> for d128 {
    /// Converts an i128 encoded decimal.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// ```
    fn from(value: u128) -> Self {
        Self::new((value >> 64) as u64, value as u64, )
    }
}

impl From<&str> for d128 {
    fn from(value: &str) -> Self {
        let mut status:_IDEC_flags = StatusFlags::BID_EXACT_STATUS;
        bid128_from_string(value, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

impl Neg for d128 {
    type Output = Self;

    /// Performs the unary - operation
    /// # Examples
    ///
    /// ```
    /// use std::ops::Neg;
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let neg1 = -dec1;
    /// let neg2 = -&dec1;
    /// ```
    fn neg(self) -> Self::Output {
        bid128_negate(&self)
    }
}

forward_ref_unop! { impl Neg, neg for d128 }

impl Add for d128 {
    type Output = Self;

    /// Performs the + operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = dec1 + dec2;
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_add(&self, &rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

forward_ref_binop!(impl Add, add for d128, d128);

impl AddAssign for d128 {
    /// Performs the += operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::d128::RoundingMode;
    /// let mut dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2     = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// dec1        += dec2;
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        let mut status: _IDEC_flags = 0;
        let dec: BID_UINT128 = bid128_add(self, &rhs, DEFAULT_ROUNDING_MODE, &mut status);

        self.w[0] = dec.w[0];
        self.w[1] = dec.w[1];
    }
}

forward_ref_op_assign! { impl AddAssign, add_assign for d128, d128 }

impl Div for d128 {
    type Output = Self;

    /// Performs the * operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = dec1 / dec2;
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_div(&self, &rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

forward_ref_binop!(impl Div, div for d128, d128);

impl DivAssign for d128 {
    /// Performs the /= operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::d128::RoundingMode;
    /// let mut dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2     = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// dec1        /= dec2;
    /// ```
    fn div_assign(&mut self, rhs: Self) {
        let mut status: _IDEC_flags = 0;
        let dec: BID_UINT128 = bid128_div(self, &rhs, DEFAULT_ROUNDING_MODE, &mut status);

        self.w[0] = dec.w[0];
        self.w[1] = dec.w[1];
    }
}

forward_ref_op_assign! { impl DivAssign, div_assign for d128, d128 }

impl Mul for d128 {
    type Output = Self;

    /// Performs the * operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = dec1 * dec2;
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_mul(&self, &rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

forward_ref_binop!(impl Mul, mul for d128, d128);

impl MulAssign for d128 {
    /// Performs the *= operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::d128::RoundingMode;
    /// let mut dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2     = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// dec1        *= dec2;
    /// ```
    fn mul_assign(&mut self, rhs: Self) {
        let mut status: _IDEC_flags = 0;
        let dec: BID_UINT128 = bid128_mul(self, &rhs, DEFAULT_ROUNDING_MODE, &mut status);

        self.w[0] = dec.w[0];
        self.w[1] = dec.w[1];
    }
}

forward_ref_op_assign! { impl MulAssign, mul_assign for d128, d128 }

impl Rem for d128 {
    type Output = Self;

    /// Performs the - operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = dec1 % dec2;
    /// ```
    fn rem(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_rem(&self, &rhs, &mut status)
    }
}

forward_ref_binop!(impl Rem, rem for d128, d128);

impl RemAssign for d128 {
    /// Performs the %= operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::d128::RoundingMode;
    /// let mut dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2     = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// dec1        %= dec2;
    /// ```
    fn rem_assign(&mut self, rhs: Self) {
        let mut status: _IDEC_flags = 0;
        let dec: BID_UINT128 = bid128_rem(self, &rhs, &mut status);

        self.w[0] = dec.w[0];
        self.w[1] = dec.w[1];
    }
}

forward_ref_op_assign! { impl RemAssign, rem_assign for d128, d128 }

impl Sub for d128 {
    type Output = Self;

    /// Performs the - operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = dec1 - dec2;
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_sub(&self, &rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

forward_ref_binop!(impl Sub, sub for d128, d128);

impl SubAssign for d128 {
    /// Performs the *= operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::d128::RoundingMode;
    /// let mut dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2     = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// dec1        -= dec2;
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        let mut status: _IDEC_flags = 0;
        let dec: BID_UINT128 = bid128_sub(self, &rhs, DEFAULT_ROUNDING_MODE, &mut status);

        self.w[0] = dec.w[0];
        self.w[1] = dec.w[1];
    }
}

forward_ref_op_assign! { impl SubAssign, sub_assign for d128, d128 }

impl std::iter::Sum for d128 {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(ZERO, |a, b| a + b)
    }
}

impl std::iter::Product for d128 {
    fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(ONE,|a, b| a * b)
    }
}

impl<'a> std::iter::Sum<&'a d128> for d128 {
    fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
        iter.fold(ZERO, |a, b| a + b)
    }
}

impl<'a> std::iter::Product<&'a d128> for d128 {
    fn product<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
        iter.fold(ONE, |a, b| a * b)
    }
}

impl std::hash::Hash for d128 {
    /// Computes the hash of a decimal floating point number.
    /// ```
    /// use std::hash::{DefaultHasher, Hash, Hasher};
    ///
    /// let mut hasher = DefaultHasher::new();
    /// decmathlib_rs::dec128!(7920).hash(&mut hasher);
    /// assert_eq!(6912922690305470905, hasher.finish());
    /// ```
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.w[0]);
        state.write_u64(self.w[1]);
    }

    /// Computes the hash of a sequence of decimal floating point numbers.
    /// ```
    /// use std::hash::{DefaultHasher, Hash, Hasher};
    ///
    /// let mut hasher = DefaultHasher::new();
    /// let numbers = [decmathlib_rs::dec128!(6), decmathlib_rs::dec128!(28), decmathlib_rs::dec128!(496), decmathlib_rs::dec128!(8128)];
    /// Hash::hash_slice(&numbers, &mut hasher);
    /// assert_eq!(16555189424726162492, hasher.finish());
    /// ```
    #[inline]
    fn hash_slice<H: std::hash::Hasher>(data: &[d128], state: &mut H) {
        let newlen: usize  = std::mem::size_of_val(data);
        let ptr: *const u8 = data.as_ptr() as *const u8;
        state.write(unsafe { std::slice::from_raw_parts(ptr, newlen) })
    }
}