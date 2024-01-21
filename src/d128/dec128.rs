/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ops::{Mul, MulAssign, Neg};
use crate::d128::bid128_mul::bid128_mul;

use crate::d128::bid128_noncomp::*;
use crate::d128::constants::*;
use crate::d128::convert::{bid128_to_bid64, bid64_to_bid128};
use crate::d128::core::{ClassTypes, DEFAULT_ROUNDING_MODE, RoundingMode};

pub type _IDEC_flags = u32;       // could be a struct with diagnostic info

#[derive(Debug, Copy, Clone, Default)]
pub (crate) struct DEC_DIGITS {
    pub (crate) digits: u32,
    pub (crate) threshold_hi: BID_UINT64,
    pub (crate) threshold_lo: BID_UINT64,
    pub (crate) digits1: u32
}

#[derive(Copy, Clone)]
pub (crate) union BID_UI32FLOAT {
    pub (crate) i: BID_UINT32,
    pub (crate) f: f32
}

impl Default for BID_UI32FLOAT {
    fn default() -> Self {
        Self {
            i: 0
        }
    }
}

#[derive(Copy, Clone)]
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

#[derive(Debug, Copy, Clone, Default)]
pub (crate) struct BID_UINT384 {
    pub (crate) w: [BID_UINT64; 6]
}

#[derive(Debug, Copy, Clone, Default)]
pub (crate) struct BID_UINT512 {
    pub (crate) w: [BID_UINT64; 8]
}

pub (crate) type BID_UINT32 = u32;

pub (crate) type BID_SINT64 = i64;

pub (crate) type BID_UINT64 = u64;

pub type decimal64 = BID_UINT64;

#[derive(Copy, Clone, Debug, Default)]
pub struct BID_UINT128 {
    pub (crate) w: [BID_UINT64; 2]
}

pub type decimal128 = BID_UINT128;

impl decimal128 {
    pub (crate) fn new(l: u64, h: u64) -> Self {
        #[cfg(target_endian = "big")]
        return Self { w: [l, h] };

        #[cfg(target_endian = "little")]
        return Self { w: [h, l] };
    }

    pub fn class(&self) -> ClassTypes {
        bid128_class(self)
    }

    pub fn copy(&self) -> Self { bid128_copy(self) }

    pub fn copy_sign(&self, other: &Self) -> Self { bid128_copySign(self, other) }

    pub fn infinity() -> Self {
        bid128_inf()
    }

    pub fn is_canonical(&self) -> bool {
        bid128_isCanonical(self)
    }

    pub fn is_finite(&self) -> bool {
        bid128_isFinite(self)
    }

    pub fn is_infinity(&self) -> bool {
        bid128_isInf(self)
    }

    pub fn is_nan(&self) -> bool {
        bid128_isNaN(self)
    }

    pub fn is_normal(&self) -> bool {
        bid128_isNormal(self)
    }

    pub fn is_signaling(&self) -> bool {
        bid128_isSignaling(self)
    }

    pub fn is_signed(&self) -> bool {
        bid128_isSigned(self)
    }

    pub fn is_subnormal(&self) -> bool {
        bid128_isSubnormal(self)
    }

    pub fn is_zero(&self) -> bool {
        bid128_isZero(self)
    }

    pub fn negate(x: &Self) -> Self {
        bid128_negate(x)
    }

    pub fn same_quantum(x: &Self, y: &Self) -> bool {
        bid128_sameQuantum(x, y)
    }

    pub fn total_order(x: &Self, y: &Self) -> bool {
        bid128_totalOrder(x, y)
    }

    pub fn total_order_mag(x: &Self, y: &Self) -> bool {
        bid128_totalOrderMag(x, y)
    }

    pub fn from_i64(value: i64) -> Self {
        let mut res = Self::default();

        // if integer is negative, use the absolute value
        if (value & SIGNMASK64 as i64) == SIGNMASK64 as i64 {
            res.w[BID_HIGH_128W] = 0xb040000000000000u64;
            res.w[BID_LOW_128W]  = (!value + 1) as BID_UINT64;	// 2's complement of x
        } else {
            res.w[BID_HIGH_128W] = 0x3040000000000000u64;
            res.w[BID_LOW_128W]  = value as u64;
        }

        res
    }

    pub fn from_u64(value: u64) -> Self {
        let mut res = Self::default();

        res.w[BID_HIGH_128W] = 0x3040000000000000u64;
        res.w[BID_LOW_128W]  = value;

        res
    }

    pub fn from_decimal64(bid: BID_UINT64, status: &mut _IDEC_flags) -> Self {
        bid64_to_bid128(bid, status)
    }

    pub fn to_decimal64(&self, rnd_mode: Option<u32>, status: &mut _IDEC_flags) -> decimal64 {
        bid128_to_bid64(self, rnd_mode.unwrap_or(RoundingMode::BID_ROUNDING_UP), status)
    }

    pub fn multiply(lhs: &Self, rhs: &Self, rnd_mode: Option<u32>, status: &mut _IDEC_flags) -> Self {
        bid128_mul(lhs, rhs, rnd_mode.unwrap_or(RoundingMode::BID_ROUNDING_UP), status)
    }
}

impl Eq for decimal128 {}

impl PartialEq for decimal128 {
    fn eq(&self, other: &Self) -> bool {
        self.w[BID_HIGH_128W] == other.w[BID_HIGH_128W] && self.w[BID_LOW_128W]  == other.w[BID_LOW_128W]
    }
}

/// Tries to convert decimal128 to decimal64
impl TryInto<decimal64> for decimal128 {
    type Error = _IDEC_flags;

    fn try_into(self) -> Result<BID_UINT64, Self::Error> {
         let mut status: _IDEC_flags = 0;
         let dec64 = bid128_to_bid64(&self, DEFAULT_ROUNDING_MODE, &mut status);

         match status {
            0 => Ok(dec64),
            _ => Err(status)
         }
    }
}

/// Converts decimal64 to decimal128
impl From<decimal64> for decimal128 {
    fn from(value: BID_UINT64) -> Self {
        let mut status: _IDEC_flags = 0;
        bid64_to_bid128(value, &mut status)
    }
}

/// Converts an i128 encoded decimal
impl From<i128> for decimal128 {
    fn from(value: i128) -> Self {
        Self::new((value >> 64) as u64, value as u64)
    }
}

/// Converts an i128 encoded decimal
impl From<u128> for decimal128 {
    fn from(value: u128) -> Self {
        Self::new((value >> 64) as u64, value as u64)
    }
}

impl Neg for decimal128 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        bid128_negate(&self)
    }
}

impl Neg for &decimal128 {
    type Output = decimal128;

    fn neg(self) -> Self::Output {
        bid128_negate(self)
    }
}

/// Performs the * operation.
/// # Examples
///
/// ```
/// let dec1 = decmathlib_rs::d128::dec128::decimal128::from(0x150a2e0d6728de4e95595bd43d654036u128);
/// let dec2 = decmathlib_rs::d128::dec128::decimal128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
/// let res  = dec1 * dec2;
/// ```
impl Mul for decimal128 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_mul(&self, &rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

/// Performs the * operation.
/// # Examples
///
/// ```
/// let dec1 = decmathlib_rs::d128::dec128::decimal128::from(0x150a2e0d6728de4e95595bd43d654036u128);
/// let dec2 = decmathlib_rs::d128::dec128::decimal128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
/// let res  = dec1 * dec2;
/// ```
impl Mul for &decimal128 {
    type Output = decimal128;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut status: _IDEC_flags = 0;
        bid128_mul(self, rhs, DEFAULT_ROUNDING_MODE, &mut status)
    }
}

/// Performs the *= operation.
/// # Examples
///
/// ```
/// use decmathlib_rs::d128::core::RoundingMode;
/// let mut dec1 = decmathlib_rs::d128::dec128::decimal128::from(0x150a2e0d6728de4e95595bd43d654036u128);
/// let dec2     = decmathlib_rs::d128::dec128::decimal128::from(0xc47aef17e9919a5569aaaf503275e8f4u128);
/// dec1        *= dec2;
/// ```
impl MulAssign for decimal128 {
    fn mul_assign(&mut self, rhs: Self) {
        let mut status: _IDEC_flags = 0;
        let dec = bid128_mul(self, &rhs, DEFAULT_ROUNDING_MODE, &mut status);

        self.w[0] = dec.w[0];
        self.w[1] = dec.w[1];
    }
}