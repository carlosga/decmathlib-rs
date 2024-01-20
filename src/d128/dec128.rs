/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez         */
/* --------------------------------------------------------------------- */
/* Original C source code Copyright (c) 2018, Intel Corp.                */
/* --------------------------------------------------------------------- */

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ops::Neg;
use crate::d128::bid128_noncomp::*;
use crate::d128::constants::*;
use crate::d128::convert::{bid128_to_bid64, bid64_to_bid128};
use crate::d128::core::{ClassTypes, RoundingMode};

pub (crate) type _IDEC_round = u32;

pub type _IDEC_flags = u32;       // could be a struct with diagnostic info

pub (crate) type BID_UINT32 = u32;

pub (crate) type BID_SINT64 = i64;

pub (crate) type BID_UINT64 = u64;

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

#[derive(Copy, Clone, Debug, Default)]
pub struct BID_UINT128 {
    pub (crate)  w: [BID_UINT64; 2]
}

#[derive(Debug, Copy, Clone, Default)]
pub (crate) struct BID_UINT192 {
    pub (crate) w: [BID_UINT64; 3]
}

#[derive(Debug, Copy, Clone, Default)]
pub (crate) struct BID_UINT256 {
    pub (crate) w: [BID_UINT64; 4]
}

impl BID_UINT128 {
    pub fn new(l: u64, h: u64) -> Self {
        #[cfg(target_endian = "big")]
        return Self { w: [l, h] };

        #[cfg(target_endian = "little")]
        return Self { w: [h, l] };
    }
}

impl BID_UINT128 {
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

    pub fn from_decimal64(bid: BID_UINT64, status: &mut _IDEC_flags) -> Self {
        bid64_to_bid128(bid, status)
    }

    pub fn to_decimal64(&self, rnd_mode: Option<u32>, status: &mut _IDEC_flags) -> BID_UINT64 {
        bid128_to_bid64(self, rnd_mode.unwrap_or(RoundingMode::BID_ROUNDING_UP), status)
    }
}

impl Eq for BID_UINT128 {}

impl PartialEq for BID_UINT128 {
    fn eq(&self, other: &Self) -> bool {
        self.w[BID_HIGH_128W] == other.w[BID_HIGH_128W] && self.w[BID_LOW_128W]  == other.w[BID_LOW_128W]
    }
}

impl Neg for BID_UINT128 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        bid128_negate(&self)
    }
}

impl Neg for &BID_UINT128 {
    type Output = BID_UINT128;

    fn neg(self) -> Self::Output {
        bid128_negate(self)
    }
}

impl From<i64> for BID_UINT128 {
    fn from(value: i64) -> Self {
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
}

impl From<u64> for BID_UINT128 {
    fn from(value: u64) -> Self {
        let mut res = Self::default();

        res.w[BID_HIGH_128W] = 0x3040000000000000u64;
        res.w[BID_LOW_128W]  = value;

        res
    }
}

impl From<i128> for BID_UINT128 {
    fn from(value: i128) -> Self {
        Self::new((value >> 64) as u64, value as u64)
    }
}

impl From<u128> for BID_UINT128 {
    fn from(value: u128) -> Self {
        Self::new((value >> 64) as u64, value as u64)
    }
}