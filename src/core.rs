/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

use crate::constants::{DEC_FE_DIVBYZERO, DEC_FE_INEXACT, DEC_FE_INVALID, DEC_FE_OVERFLOW, DEC_FE_UNDERFLOW, DEC_FE_UNNORMAL};
use crate::d128::_IDEC_flags;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassTypes {
    SignalingNaN,
    QuietNaN,
    NegativeInfinity,
    NegativeNormal,
    NegativeSubnormal,
    NegativeZero,
    PositiveZero,
    PositiveSubnormal,
    PositiveNormal,
    PositiveInfinity
}

pub struct RoundingMode;

impl RoundingMode {
    // rounding modes
    pub const BID_ROUNDING_TO_NEAREST: u32  = 0x00000;
    pub const BID_ROUNDING_DOWN: u32        = 0x00001;
    pub const BID_ROUNDING_UP: u32          = 0x00002;
    pub const BID_ROUNDING_TO_ZERO: u32     = 0x00003;
    pub const BID_ROUNDING_TIES_AWAY: u32   = 0x00004;

    pub const BID_RMODE_MASK: u32 = RoundingMode::BID_ROUNDING_TO_NEAREST
        | RoundingMode::BID_ROUNDING_DOWN
        | RoundingMode::BID_ROUNDING_UP
        | RoundingMode::BID_ROUNDING_TO_ZERO
        | RoundingMode::BID_ROUNDING_TIES_AWAY;
}

#[cfg(DECIMAL_DEFAULT_ROUNDING_MODE = "TO_NEAREST")]
pub const DEFAULT_ROUNDING_MODE: u32 = RoundingMode::BID_ROUNDING_TO_NEAREST;

#[cfg(DECIMAL_DEFAULT_ROUNDING_MODE = "DOWN")]
pub const DEFAULT_ROUNDING_MODE: u32 = RoundingMode::BID_ROUNDING_DOWN;

#[cfg(DECIMAL_DEFAULT_ROUNDING_MODE = "UP")]
pub const DEFAULT_ROUNDING_MODE: u32 = RoundingMode::BID_ROUNDING_UP;

#[cfg(DECIMAL_DEFAULT_ROUNDING_MODE = "TO_ZERO")]
pub const DEFAULT_ROUNDING_MODE: u32 = RoundingMode::BID_ROUNDING_TO_ZERO;

#[cfg(DECIMAL_DEFAULT_ROUNDING_MODE = "TIES_AWAY")]
pub const DEFAULT_ROUNDING_MODE: u32 = RoundingMode::BID_ROUNDING_TIES_AWAY;

pub struct StatusFlags;

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
