/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(non_camel_case_types)]

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;
use crate::bid128_add::{bid128_add, bid128_sub};
use crate::bid128_compare::{bid128_quiet_equal, bid128_quiet_greater, bid128_quiet_greater_equal, bid128_quiet_less, bid128_quiet_less_equal, bid128_quiet_not_equal};
use crate::bid128_div::bid128_div;
use crate::bid128_fdim::bid128_fdim;
use crate::bid128_fmod::bid128_fmod;
use crate::bid128_frexp::bid128_frexp;
use crate::bid128_ilogb::bid128_ilogb;
use crate::bid128_ldexp::bid128_ldexp;
use crate::bid128_logb::bid128_logb;
use crate::bid128_lrint::bid128_lrint;
use crate::bid128_lround::bid128_lround;
use crate::bid128_minmax::{bid128_maxnum, bid128_maxnum_mag, bid128_minnum, bid128_minnum_mag};
use crate::bid128_modf::bid128_modf;
use crate::bid128_mul::bid128_mul;
use crate::bid128_nearbyint::bid128_nearbyint;
use crate::bid128_noncomp::*;
use crate::bid128_rem::bid128_rem;
use crate::bid128_scalbln::bid128_scalbln;
use crate::bid128_scalbn::bid128_scalbn;
use crate::bid128_string::{bid128_from_string, bid128_to_string};
use crate::bid128_to_int32::*;
use crate::bid128_to_int64::*;
use crate::bid_conf::{BID_HIGH_128W, BID_LOW_128W};
use crate::bid_from_int::{bid128_from_int32, bid128_from_int64, bid128_from_uint32, bid128_from_uint64};use crate::convert::{bid128_to_bid64, bid64_to_bid128};
use crate::core::{ClassTypes, DEFAULT_ROUNDING_MODE, StatusFlags};
use crate::d64::d64;

// BID_FPSC
pub type _IDEC_flags = u32;

#[derive(Debug, Clone)]
pub (crate) struct DEC_DIGITS {
    pub (crate) digits: u32,
    pub (crate) threshold_hi: BID_UINT64,
    pub (crate) threshold_lo: BID_UINT64,
    pub (crate) digits1: u32
}

pub (crate) union BID_UI32FLOAT {
    pub (crate) i: BID_UINT32,
    pub (crate) d: f32
}

impl Default for BID_UI32FLOAT {
    fn default() -> Self {
        Self {
            i: 0
        }
    }
}

pub (crate) union BID_UI64DOUBLE {
    pub (crate) i: BID_UINT64,
    pub (crate) d: f64
}

impl Default for BID_UI64DOUBLE {
    fn default() -> Self {
        Self {
            i: 0
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub (crate) struct BID_UINT192 {
    pub (crate) w: [BID_UINT64; 3]
}

#[derive(Debug, Copy, Clone, Default)]
pub (crate) struct BID_UINT256 {
    pub (crate) w: [BID_UINT64; 4]
}

#[derive(Debug, Clone, Default)]
pub (crate) struct BID_UINT384 {
    pub (crate) w: [BID_UINT64; 6]
}

#[derive(Debug, Clone, Default)]
pub (crate) struct BID_UINT512 {
    pub (crate) w: [BID_UINT64; 8]
}

pub (crate) type BID_UINT32 = u32;

pub (crate) type BID_SINT64 = i64;

pub (crate) type BID_UINT64 = u64;

/// The 128-bit decimal type.
#[derive(Copy, Clone, Debug, Default)]
pub struct d128 {
    pub (crate) w: [BID_UINT64; 2]
}

pub (crate) type BID_UINT128 = d128;

#[macro_export]
macro_rules! dec128 {
    ($t:tt) => {{
        $crate::d128::d128::from_str(stringify!($t)).expect("Invalid decimal number literal")
    }}
}

impl d128 {
    pub (crate) fn new(h: u64, l: u64) -> Self {
        #[cfg(target_endian = "big")]
        return Self { w: [h, l] };

        #[cfg(target_endian = "little")]
        return Self { w: [l, h] };
    }

    /// Copies a 128-bit decimal floating-point operand x to a destination in the same format, changing the sign to positive
    pub fn abs(&self) -> BID_UINT128 {
        bid128_abs(self)
    }

    /// Tells which of the following ten classes x falls into (details in the IEEE Standard 754-2008):
    /// signalingNaN, quietNaN, negativeInfinity, negativeNormal, negativeSubnormal, negativeZero, positiveZero,
    /// positiveSubnormal, positiveNormal, positiveInfinity
    pub fn class(&self) -> ClassTypes {
        bid128_class(self)
    }

    /// Copies a decimal floating-point operand x to a destination in the same format, with no change
    pub fn copy(&self) -> Self { bid128_copy(self) }

    /// Copies a 128-bit decimal floating-point operand x to a destination in the same format as x, but with the sign of y
    pub fn copy_sign(&self, other: &Self) -> Self { bid128_copy_sign(self, other) }

    pub fn infinity() -> Self {
        bid128_inf()
    }

    /// Return true if and only if x is a finite number, infinity, or NaN that is canonical
    pub fn is_canonical(&self) -> bool {
        bid128_is_canonical(self)
    }

    /// Return true if and only if x is zero, subnormal or normal (not infinite or NaN)
    pub fn is_finite(&self) -> bool {
        bid128_is_finite(self)
    }

    /// Return true if and only if x is infinite
    pub fn is_infinity(&self) -> bool {
        bid128_is_inf(self)
    }

    /// Return true if and only if x is a NaN
    pub fn is_nan(&self) -> bool {
        bid128_is_nan(self)
    }

    /// Return true if and only if x is normal (not zero, subnormal, infinite, or NaN)
    pub fn is_normal(&self) -> bool {
        bid128_is_normal(self)
    }

    /// Return true if and only if x is a signaling NaN
    pub fn is_signaling(&self) -> bool {
        bid128_is_signaling(self)
    }

    /// Return true if and only if x has negative sign
    pub fn is_signed(&self) -> bool {
        bid128_is_signed(self)
    }

    /// Return true if and only if x is subnormal
    pub fn is_subnormal(&self) -> bool {
        bid128_is_subnormal(self)
    }

    /// Copies a 128-bit decimal floating-point operand x to a destination in the same format, reversing the sign
    pub fn is_zero(&self) -> bool {
        bid128_is_zero(self)
    }

    pub fn negate(x: &Self) -> Self {
        bid128_negate(x)
    }

    /// same_quantum(x, y) is true if the exponents of x and y are the same,
    /// and false otherwise; same_quantum(NaN, NaN) and same_quantum(inf, inf) are
    /// true; if exactly one operand is infinite or exactly one operand is NaN,
    /// sameQuantum is false
    pub fn same_quantum(x: &Self, y: &Self) -> bool {
        bid128_same_quantum(x, y)
    }

    /// Return true if the absolute values of x and y are ordered (see the IEEE Standard 754-2008)
    pub fn total_order(x: &Self, y: &Self) -> bool {
        bid128_total_order(x, y)
    }

    /// Return true if the absolute values of x and y are ordered (see the IEEE Standard 754-2008)
    pub fn total_order_mag(x: &Self, y: &Self) -> bool {
        bid128_total_order_mag(x, y)
    }

    pub fn nan(tagp: &str, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
        bid128_nan(tagp, pfpsf)
    }

    /// Convert a decimal floating-point value represented in string format
    /// (decimal character sequence) to 128-bit decimal floating-point format (binary encoding)
    pub fn from_string(value: &str, rnd_mode: Option<u32>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_from_string(value, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Convert 64-bit decimal floating-point value to 128-bit decimal floating-point format (binary encoding)
    pub fn from_decimal64(bid: d64, status: &mut _IDEC_flags) -> Self {
        bid64_to_bid128(bid.0, status)
    }

    /// fdim returns x - y if x > y, and +0 is x <= y
    pub fn fdim(&self, rhs: &BID_UINT128, rnd_mode: Option<u32>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_fdim(self, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    pub fn fmod(&self, rhs: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_fmod(self, rhs, pfpsf)
    }

    pub fn frexp(&self, exp: i32) -> (Self, i32) {
        bid128_frexp(self, exp)
    }

    /// multiply a 128-bit decimal floating-point value by an integral power of 2.
    pub fn ldexp(&self, n: i32, rnd_mode: Option<u32>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_ldexp(self, n, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Returns the exponent e of x, a signed integral value, determined
    /// as though x were represented with infinite range and minimum exponent
    pub fn logb(&self, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_logb(self, pfpsf)
    }

    /// The lrint function rounds its argument to the nearest integer value of
    /// type long int, rounding according to the current rounding direction.
    pub fn lrint(&self, rnd_mode: Option<u32>, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_lrint(self, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// The lround function rounds its argument to the nearest integer value of
    /// type long int, using rounding to nearest-away
    pub fn lround(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_lround(self, pfpsf)
    }

    /// Returns the exponent e of x, a signed integral value, determined
    /// as though x were represented with infinite range and minimum exponent
    pub fn ilogb(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_ilogb(self, pfpsf)
    }

    /// Returns the canonicalized floating-point number y if x < y,
    /// x if y < x, the canonicalized floating-point number if one operand is a
    /// floating-point number and the other a quiet NaN.
    pub fn max(x: &Self, y: &Self, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
        bid128_maxnum(x, y, pfpsf)
    }

    /// Returns the canonicalized floating-point number x if |x| > |y|,
    /// y if |y| > |x|, otherwise this function is identical to __bid128_maxnum
    pub fn maxnum_mag(x: &Self, y: &Self, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
        bid128_maxnum_mag(x, y, pfpsf)
    }

    /// Returns the canonicalized floating-point number x if x < y,
    /// y if y < x, the canonicalized floating-point number if one operand is
    /// a floating-point number and the other a quiet NaN. Otherwise it is either x or y, canonicalized.
    pub fn min(x: &Self, y: &Self, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
        bid128_minnum(x, y, pfpsf)
    }

    /// Returns the canonicalized floating-point number x if |x| < |y|,
    /// y if |y| < |x|, otherwise this function is identical to __bid64_minnum
    pub fn minnum_mag(x: &Self, y: &Self, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
        bid128_minnum_mag(x, y, pfpsf)
    }

    /// Decomposes given decimal floating point value num into integral and fractional parts,
    pub fn modf(&self, pfpsf: &mut _IDEC_flags) -> (Self, Self) {
        bid128_modf(self, pfpsf)
    }

    /// Rounds the decimal floating-point value num to an integer value in deicmal floating-point format, using the given rounding mode.
    pub fn nearbyint(&self, rnd_mode: Option<u32>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_nearbyint(self, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Returns x * 10^N
    pub fn scalbn(&self, n: i32, rnd_mode: Option<u32>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_scalbn(self, n, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Returns x * 10^N
    pub fn scalbln(&self, n: i64, rnd_mode: Option<u32>, pfpsf: &mut _IDEC_flags) -> Self {
        bid128_scalbln(self, n, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit decimal floating-point format (binary encoding)
    pub fn to_decimal64(&self, rnd_mode: Option<u32>, status: &mut _IDEC_flags) -> d64 {
        d64(bid128_to_bid64(self, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status))
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed
    /// integer in rounding-to-nearest-even mode; inexact exceptions not signaled
    pub fn to_i32_rnint(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_rnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-nearest-even mode; inexact exceptions signaled
    pub fn to_i32_xrnint(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xrnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-down mode; inexact exceptions not signaled
    pub fn to_i32_floor(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_floor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-down mode; inexact exceptions signaled
    pub fn to_i32_xfloor(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xfloor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-up mode; inexact exceptions not signaled
    pub fn to_i32_ceil(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_ceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-up mode; inexact exceptions signaled
    pub fn to_i32_xceil(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-zero; inexact exceptions not signaled
    pub fn to_i32_int(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_int(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-zero; inexact exceptions signale
    pub fn to_i32_xint(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-nearest-away; inexact exceptions not signaled
    pub fn to_i32_rninta(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_rninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 32-bit signed integer
    /// in rounding-to-nearest-away; inexact exceptions signaled
    pub fn to_i32_xrninta(&self, pfpsf: &mut _IDEC_flags) -> i32 {
        bid128_to_int32_xrninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-up mode; inexact exceptions not signaled
    pub fn to_i64_ceil(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_ceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-down mode; inexact exceptions not signaled
    pub fn to_i64_floor(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_floor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-to-zero; inexact exceptions not signaled
    pub fn to_i64_int(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_int(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed
    /// integer in rounding-to-nearest-even mode; inexact exceptions not signaled
    pub fn to_i64_rnint(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_rnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed
    /// integer in rounding-to-nearest-away; inexact exceptions not signaled
    pub fn to_i64_rninta(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_rninta(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-up mode; inexact exceptions signaled
    pub fn to_i64_xceil(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xceil(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-down mode; inexact exceptions signaled
    pub fn to_i64_xfloor(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xfloor(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed integer
    /// in rounding-to-zero; inexact exceptions signaled
    pub fn to_i64_xint(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed
    /// integer in rounding-to-nearest-even mode; inexact exceptions signaled
    pub fn to_i64_xrnint(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xrnint(self, pfpsf)
    }

    /// Convert 128-bit decimal floating-point value to 64-bit signed
    /// integer in rounding-to-nearest-away; inexact exceptions signaled
    pub fn to_i64_xrninta(&self, pfpsf: &mut _IDEC_flags) -> i64 {
        bid128_to_int64_xrninta(self, pfpsf)
    }

    pub fn add(lhs: &Self, rhs: &Self, rnd_mode: Option<u32>, status: &mut _IDEC_flags) -> Self {
        bid128_add(lhs, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status)
    }

    pub fn divide(lhs: &Self, rhs: &Self, rnd_mode: Option<u32>, status: &mut _IDEC_flags) -> Self {
        bid128_div(lhs, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status)
    }

    pub fn multiply(lhs: &Self, rhs: &Self, rnd_mode: Option<u32>, status: &mut _IDEC_flags) -> Self {
        bid128_mul(lhs, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status)
    }

    pub fn remainder(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> Self {
        bid128_rem(lhs, rhs, status)
    }

    pub fn subtract(lhs: &Self, rhs: &Self, rnd_mode: Option<u32>, status: &mut _IDEC_flags) -> Self {
        bid128_sub(lhs, rhs, rnd_mode.unwrap_or(DEFAULT_ROUNDING_MODE), status)
    }

    pub fn quiet_equal(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_equal(lhs, rhs, status)
    }

    pub fn quiet_greater(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_greater(lhs, rhs, status)
    }

    pub fn quiet_greater_equal(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_greater_equal(lhs, rhs, status)
    }

    pub fn quiet_less(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_less(lhs, rhs, status)
    }

    pub fn quiet_less_equal(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_less_equal(lhs, rhs, status)
    }

    pub fn quiet_not_equal(lhs: &Self, rhs: &Self, status: &mut _IDEC_flags) -> bool {
        bid128_quiet_not_equal(lhs, rhs, status)
    }
}

impl Eq for d128 { }

impl PartialEq for d128 {
    fn eq(&self, other: &Self) -> bool {
        self.w[BID_HIGH_128W] == other.w[BID_HIGH_128W] && self.w[BID_LOW_128W] == other.w[BID_LOW_128W]
    }
}

impl PartialOrd for d128 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut status: _IDEC_flags = StatusFlags::BID_EXACT_STATUS;
        let equal = self.w[BID_HIGH_128W] == other.w[BID_HIGH_128W]
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

impl Display for d128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", bid128_to_string(self))
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
            0 => Ok(dec),
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
    /// let neg  = dec1.neg();
    /// ```
    fn neg(self) -> Self::Output {
        bid128_negate(&self)
    }
}

impl Neg for &d128 {
    type Output = d128;

    /// Performs the unary - operation
    /// # Examples
    ///
    /// ```
    /// use std::ops::Neg;
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let neg  = &dec1.neg();
    /// ```
    fn neg(self) -> Self::Output {
        bid128_negate(self)
    }
}

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

impl Add for &d128 {
    type Output = d128;

    /// Performs the + operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = &dec1 + &dec2;
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_add(self, rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

impl AddAssign for d128 {
    /// Performs the += operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::core::RoundingMode;
    /// let mut dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2     = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// dec1        += dec2;
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        let mut status: _IDEC_flags = 0;
        let dec: BID_UINT128 = bid128_mul(self, &rhs, DEFAULT_ROUNDING_MODE, &mut status);

        self.w[0] = dec.w[0];
        self.w[1] = dec.w[1];
    }
}

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

impl Div for &d128 {
    type Output = d128;

    /// Performs the / operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = &dec1 / &dec2;
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_mul(self, rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

impl DivAssign for d128 {
    /// Performs the /= operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::core::RoundingMode;
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

impl Mul for &d128 {
    type Output = d128;

    /// Performs the * operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = &dec1 * &dec2;
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_mul(self, rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

impl MulAssign for d128 {
    /// Performs the *= operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::core::RoundingMode;
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

impl Rem for &d128 {
    type Output = d128;

    /// Performs the % operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = &dec1 % &dec2;
    /// ```
    fn rem(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_rem(self, rhs, &mut status)
    }
}

impl RemAssign for d128 {
    /// Performs the %= operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::core::RoundingMode;
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

impl Sub for &d128 {
    type Output = d128;

    /// Performs the - operation.
    /// # Examples
    ///
    /// ```
    /// let dec1 = decmathlib_rs::d128::d128::from(0x150a2e0d6728de4e95595bd43d654036u128);
    /// let dec2 = decmathlib_rs::d128::d128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
    /// let res  = &dec1 - &dec2;
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_sub(self, rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

impl SubAssign for d128 {
    /// Performs the *= operation.
    /// # Examples
    ///
    /// ```
    /// use decmathlib_rs::core::RoundingMode;
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
