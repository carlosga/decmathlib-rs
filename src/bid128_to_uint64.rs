/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#![allow(overflowing_literals)]

use crate::bid128::*;
use crate::bid_internal::*;
use crate::d128::{_IDEC_flags, StatusFlags};

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-to-nearest-even mode; inexact exceptions not signaled
pub (crate) fn bid128_to_uint64_rnint(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let tmp64: BID_UINT64;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let mut fstar: BID_UINT256 = BID_UINT256::default();
    let P256: BID_UINT256;

    // unpack x
    x_sign  = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp   = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        // if (x.w[1] & MASK_NAN) == MASK_NAN {	    // x is NAN
        //     if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if x_sign == 0 {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        unsafe {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	        // exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                tmp1.d    = C1.w[1] as f64;	            // exact conversion
                x_nr_bits = 65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
            }
        }
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1]  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
             }
        }
        exp = ((x_exp >> 49) - 6176) as i32;
        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                if x_sign != 0 {	// if n < 0 and q + exp = 20
                    // if n < -1/2 then n cannot be converted to uint64 with RN
                    // too large if c(0)c(1)...c(19).c(20)...c(q-1) > 1/2
                    // <=> 0.c(0)c(1)...c(q-1) * 10^21 > 0x05, 1<=q<=34
                    // <=> C * 10^(21-q) > 0x05, 1<=q<=34
                    if q == 21 {
                        // C > 5
                        if C1.w[1] != 0 || C1.w[0] > 0x05u64 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to 64-bit unsigned int fall through
                        // to '1 <= q + exp <= 20'
                    } else {
                        // if 1 <= q <= 20
                        //   C * 10^(21-q) > 5 is true because C >= 1 and 10^(21-q) >= 10
                        // if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        //   C > 5 * 10^(q-21) is true because C > 2^64 and 5*10^(q-21) < 2^64
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// if n > 0 and q + exp = 20
                    // if n >= 2^64 - 1/2 then n is too large
                    // <=> c(0)c(1)...c(19).c(20)...c(q-1) >= 2^64-1/2
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 >= 2^64-1/2
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 >= 5*(2^65-1)
                    // <=> C * 10^(21-q) >= 0x9fffffffffffffffb, 1<=q<=34
                    if q == 1 {
                        // C * 10^20 >= 0x9fffffffffffffffb
                        C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q <= 19 {
                        // C * 10^(21-q) >= 0x9fffffffffffffffb
                        C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 20 {
                        // C * 10 >= 0x9fffffffffffffffb <=> C * 2 > 1ffffffffffffffff
                        C.w[0] = C1.w[0] + C1.w[0];
                        C.w[1] = C1.w[1] + C1.w[1];
                        if C.w[0] < C1.w[0] {
                            C.w[1] += 1;
                        }
                        if C.w[1] > 0x01 || (C.w[1] == 0x01 && C.w[0] >= 0xffffffffffffffffu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 21 {
                        // C >= 0x9fffffffffffffffb
                        if C1.w[1] > 0x09 || (C1.w[1] == 0x09 && C1.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        // C  >= 10^(q-21) * 0x9fffffffffffffffb max 44 bits x 68 bits
                        C.w[1] = 0x09;
                        C.w[0] = 0xfffffffffffffffbu64;
                        C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                        if C1.w[1] > C.w[1]
                                || (C1.w[1] == C.w[1] && C1.w[0] >= C.w[0]) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    }
                }
            },
            _ => { }
        }
        // n is not too large to be converted to int64 if -1/2 <= n < 2^64 - 1/2
        // Note: some of the cases tested for above fall through to this point
        match q + exp {
            value if value < 0 => {	// n = +/-0.0...c(0)c(1)...c(q-1)
                // return 0
                res = 0x0000000000000000u64;
                return res;
            },
            value if value == 0 => {	// n = +/-0.c(0)c(1)...c(q-1)
                // if 0.c(0)c(1)...c(q-1) <= 0.5 <=> c(0)c(1)...c(q-1) <= 5 * 10^(q-1)
                //   res = 0
                // else if x > 0
                //   res = +1
                // else // if x < 0
                //   invalid exc
                ind = q - 1;
                if ind <= 18 {	// 0 <= ind <= 18
                    res = if (C1.w[1] == 0) && (C1.w[0] <= BID_MIDPOINT64[ind as usize]) {
                        0x0000000000000000u64	// return 0
                    } else if x_sign == 0 {	// n > 0
                        0x00000001	// return +1
                    } else {
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        0x8000000000000000u64
                    };
                } else {	// 19 <= ind <= 33
                    if  (C1.w[1]  < BID_MIDPOINT128[(ind - 19) as usize].w[1])
                    || ((C1.w[1] == BID_MIDPOINT128[(ind - 19) as usize].w[1])
                     && (C1.w[0] <= BID_MIDPOINT128[(ind - 19) as usize].w[0])) {
                        res = 0x0000000000000000u64;	// return 0
                    } else if x_sign == 0 {	// n > 0
                        res = 0x00000001;	// return +1
                    } else {
                        res = 0x8000000000000000u64;
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                    }
                }
            },
             _ => {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
                // x <= -1 or 1 <= x < 2^64-1/2 so if positive x can be rounded
                // to nearest to a 64-bit unsigned signed integer
                if x_sign != 0 {	// x <= -1
                    // set invalid flag
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                    // return Integer Indefinite
                    res = 0x8000000000000000u64;
                    return res;
                }
                // 1 <= x < 2^64-1/2 so x can be rounded
                // to nearest to a 64-bit unsigned integer
                match exp {
                    value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                        ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                        // chop off ind digits from the lower part of C1
                        // C1 = C1 + 1/2 * 10^ind where the result C1 fits in 127 bits
                        tmp64 = C1.w[0];
                        if ind <= 19 {
                            C1.w[0] += BID_MIDPOINT64[(ind - 1) as usize];
                        } else {
                            C1.w[0] += BID_MIDPOINT128[(ind - 20) as usize].w[0];
                            C1.w[1] += BID_MIDPOINT128[(ind - 20) as usize].w[1];
                        }
                        if C1.w[0] < tmp64 {
                            C1.w[1] += 1;
                        }
                        // calculate C* and f*
                        // C* is actually floor(C*) in this case
                        // C* and f* need shifting and masking, as shown by
                        // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                        // 1 <= x <= 33
                        // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                        // C* = (C1 + 1/2 * 10^x) * 10^(-x)
                        // the approximation of 10^(-x) was rounded up to 118 bits
                        P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                        if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                            Cstar.w[1] = P256.w[3];
                            Cstar.w[0] = P256.w[2];
                            fstar.w[3] = 0;
                            fstar.w[2] = P256.w[2] & BID_MASKHIGH128[(ind - 1) as usize];
                            fstar.w[1] = P256.w[1];
                            fstar.w[0] = P256.w[0];
                        } else {	// 22 <= ind - 1 <= 33
                            Cstar.w[1] = 0;
                            Cstar.w[0] = P256.w[3];
                            fstar.w[3] = P256.w[3] & BID_MASKHIGH128[(ind - 1) as usize];
                            fstar.w[2] = P256.w[2];
                            fstar.w[1] = P256.w[1];
                            fstar.w[0] = P256.w[0];
                        }
                        // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                        // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                        // if (0 < f* < 10^(-x)) then the result is a midpoint
                        //   if floor(C*) is even then C* = floor(C*) - logical right
                        //       shift; C* has p decimal digits, correct by Prop. 1)
                        //   else if floor(C*) is odd C* = floor(C*)-1 (logical right
                        //       shift; C* has p decimal digits, correct by Pr. 1)
                        // else
                        //   C* = floor(C*) (logical right shift; C has p decimal digits,
                        //       correct by Property 1)
                        // n = C* * 10^(e+x)

                        // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                        shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                        Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                            (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                            // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                        } else {	// 22 <= ind - 1 <= 33
                            Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                        };
                        // if the result was a midpoint it was rounded away from zero, so
                        // it will need a correction
                        // check for midpoints
                        if (fstar.w[3] == 0) && (fstar.w[2] == 0)
                        && (fstar.w[1] != 0 || fstar.w[0] != 0)
                        && (fstar.w[1]  < BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0] <= BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0])) {
                            // the result is a midpoint; round to nearest
                            if (Cstar.w[0] & 0x01) == 0x01 {	// Cstar.w[0] is odd; MP in [EVEN, ODD]
                                // if floor(C*) is odd C = floor(C*) - 1; the result >= 1
                                Cstar.w[0] -= 1;	// Cstar.w[0] is now even
                            }	// else MP in [ODD, EVEN]
                        }
                        res = Cstar.w[0];	// the result is positive
                    },
                    value if value == 0 => {
                        // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                        // res = C (exact)
                        res = C1.w[0];
                    },
                    _ => {
                        // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                        // res = C * 10^exp (exact) - must fit in 64 bits
                        res = C1.w[0] * BID_TEN2K64[exp as  usize];
                    }
                }
            }
        }
    }

    res
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-to-nearest-even mode; inexact exceptions signaled
pub (crate) fn bid128_to_uint64_xrnint(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let mut tmp64: BID_UINT64;
    let mut tmp64A: BID_UINT64;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let mut fstar: BID_UINT256 = BID_UINT256::default();
    let P256: BID_UINT256;

    // unpack x
    x_sign  = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp   = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        // if (x.w[1] & MASK_NAN) == MASK_NAN {	        // x is NAN
        //     if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if x_sign == 0 {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        unsafe {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	        // exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
            tmp1.d    = C1.w[1] as f64;	// exact conversion
            x_nr_bits = 65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
            }
        }
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1]  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
             }
        }
        exp = ((x_exp >> 49) - 6176) as i32;

        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                if x_sign != 0 {	// if n < 0 and q + exp = 20
                    // if n < -1/2 then n cannot be converted to uint64 with RN
                    // too large if c(0)c(1)...c(19).c(20)...c(q-1) > 1/2
                    // <=> 0.c(0)c(1)...c(q-1) * 10^21 > 0x05, 1<=q<=34
                    // <=> C * 10^(21-q) > 0x05, 1<=q<=34
                    if q == 21 {
                        // C > 5
                        if C1.w[1] != 0 || C1.w[0] > 0x05u64 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to 64-bit unsigned int fall through
                        // to '1 <= q + exp <= 20'
                    } else {
                        // if 1 <= q <= 20
                        //   C * 10^(21-q) > 5 is true because C >= 1 and 10^(21-q) >= 10
                        // if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        //   C > 5 * 10^(q-21) is true because C > 2^64 and 5*10^(q-21) < 2^64
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// if n > 0 and q + exp = 20
                    // if n >= 2^64 - 1/2 then n is too large
                    // <=> c(0)c(1)...c(19).c(20)...c(q-1) >= 2^64-1/2
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 >= 2^64-1/2
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 >= 5*(2^65-1)
                    // <=> C * 10^(21-q) >= 0x9fffffffffffffffb, 1<=q<=34
                    if q == 1 {
                        // C * 10^20 >= 0x9fffffffffffffffb
                        C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q <= 19 {
                        // C * 10^(21-q) >= 0x9fffffffffffffffb
                        C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 20 {
                        // C * 10 >= 0x9fffffffffffffffb <=> C * 2 > 1ffffffffffffffff
                        C.w[0] = C1.w[0] + C1.w[0];
                        C.w[1] = C1.w[1] + C1.w[1];
                        if C.w[0] < C1.w[0] {
                            C.w[1] += 1;
                        }
                        if C.w[1] > 0x01 || (C.w[1] == 0x01 && C.w[0] >= 0xffffffffffffffffu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 21 {
                        // C >= 0x9fffffffffffffffb
                        if C1.w[1] > 0x09 || (C1.w[1] == 0x09 && C1.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        // C  >= 10^(q-21) * 0x9fffffffffffffffb max 44 bits x 68 bits
                        C.w[1] = 0x09;
                        C.w[0] = 0xfffffffffffffffbu64;
                        C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                        if C1.w[1] > C.w[1] || (C1.w[1] == C.w[1] && C1.w[0] >= C.w[0]) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    }
                }
            }
            _ => { }
        }
        // n is not too large to be converted to int64 if -1/2 <= n < 2^64 - 1/2
        // Note: some of the cases tested for above fall through to this point
        match q + exp {
            value if value < 0 => {	// n = +/-0.0...c(0)c(1)...c(q-1)
                // set inexact flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                // return 0
                res = 0x0000000000000000u64;
                return res;
            },
            value if value == 0 => {	// n = +/-0.c(0)c(1)...c(q-1)
                // if 0.c(0)c(1)...c(q-1) <= 0.5 <=> c(0)c(1)...c(q-1) <= 5 * 10^(q-1)
                //   res = 0
                // else if x > 0
                //   res = +1
                // else // if x < 0
                //   invalid exc
                ind = q - 1;
                if ind <= 18 {	// 0 <= ind <= 18
                    if (C1.w[1] == 0) && (C1.w[0] <= BID_MIDPOINT64[ind as usize]) {
                        res = 0x0000000000000000u64;	// return 0
                    } else if x_sign == 0 {	// n > 0
                        res = 0x00000001;	// return +1
                    } else {
                        res = 0x8000000000000000u64;
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        return res;
                    }
                } else {	// 19 <= ind <= 33
                    if  (C1.w[1]  < BID_MIDPOINT128[(ind - 19) as usize].w[1])
                    || ((C1.w[1] == BID_MIDPOINT128[(ind - 19) as usize].w[1])
                     && (C1.w[0] <= BID_MIDPOINT128[(ind - 19) as usize].w[0])) {
                        res = 0x0000000000000000u64;	// return 0
                    } else if x_sign == 0 {	// n > 0
                        res = 0x00000001;	// return +1
                    } else {
                        res = 0x8000000000000000u64;
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        return res;
                    }
                }
                // set inexact flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            },
            _ => {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
                // x <= -1 or 1 <= x < 2^64-1/2 so if positive x can be rounded
                // to nearest to a 64-bit unsigned signed integer
                if x_sign != 0 {	// x <= -1
                    // set invalid flag
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                    // return Integer Indefinite
                    res = 0x8000000000000000u64;
                    return res;
                }
                // 1 <= x < 2^64-1/2 so x can be rounded
                // to nearest to a 64-bit unsigned integer
                match exp {
                    value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                        ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                        // chop off ind digits from the lower part of C1
                        // C1 = C1 + 1/2 * 10^ind where the result C1 fits in 127 bits
                        tmp64 = C1.w[0];
                        if ind <= 19 {
                            C1.w[0] += BID_MIDPOINT64[(ind - 1) as usize];
                        } else {
                            C1.w[0] += BID_MIDPOINT128[(ind - 20) as usize].w[0];
                            C1.w[1] += BID_MIDPOINT128[(ind - 20) as usize].w[1];
                        }
                        if C1.w[0] < tmp64 {
                            C1.w[1] += 1;
                        }
                        // calculate C* and f*
                        // C* is actually floor(C*) in this case
                        // C* and f* need shifting and masking, as shown by
                        // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                        // 1 <= x <= 33
                        // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                        // C* = (C1 + 1/2 * 10^x) * 10^(-x)
                        // the approximation of 10^(-x) was rounded up to 118 bits
                        P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                        if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                            Cstar.w[1] = P256.w[3];
                            Cstar.w[0] = P256.w[2];
                            fstar.w[3] = 0;
                            fstar.w[2] = P256.w[2] & BID_MASKHIGH128[(ind - 1) as usize];
                            fstar.w[1] = P256.w[1];
                            fstar.w[0] = P256.w[0];
                        } else {	// 22 <= ind - 1 <= 33
                            Cstar.w[1] = 0;
                            Cstar.w[0] = P256.w[3];
                            fstar.w[3] = P256.w[3] & BID_MASKHIGH128[(ind - 1) as usize];
                            fstar.w[2] = P256.w[2];
                            fstar.w[1] = P256.w[1];
                            fstar.w[0] = P256.w[0];
                        }
                        // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                        // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                        // if (0 < f* < 10^(-x)) then the result is a midpoint
                        //   if floor(C*) is even then C* = floor(C*) - logical right
                        //       shift; C* has p decimal digits, correct by Prop. 1)
                        //   else if floor(C*) is odd C* = floor(C*)-1 (logical right
                        //       shift; C* has p decimal digits, correct by Pr. 1)
                        // else
                        //   C* = floor(C*) (logical right shift; C has p decimal digits,
                        //       correct by Property 1)
                        // n = C* * 10^(e+x)

                        // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                        shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                        Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                            (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                            // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                        } else {	// 22 <= ind - 1 <= 33
                            Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                        };
                        // determine inexactness of the rounding of C*
                        // if (0 < f* - 1/2 < 10^(-x)) then
                        //   the result is exact
                        // else // if (f* - 1/2 > T*) then
                        //   the result is inexact
                        if ind - 1 <= 2 {
                            if  fstar.w[1]  > 0x8000000000000000u64
                            || (fstar.w[1] == 0x8000000000000000u64
                             && fstar.w[0]  > 0x0u64) {
                                // f* > 1/2 and the result may be exact
                                tmp64 = fstar.w[1] - 0x8000000000000000u64;	// f* - 1/2
                                if  tmp64  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                || (tmp64 == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                 && fstar.w[0] >= BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                }	// else the result is exact
                            } else {	// the result is inexact; f2* <= 1/2
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            }
                        } else if ind - 1 <= 21 {	// if 3 <= ind <= 21
                            if  fstar.w[3] > 0x0
                            || (fstar.w[3] == 0x0 && fstar.w[2] > BID_ONEHALF128[(ind - 1) as usize])
                            || (fstar.w[3] == 0x0 && fstar.w[2] == BID_ONEHALF128[(ind - 1) as usize]
                            && (fstar.w[1] != 0 || fstar.w[0] != 0)) {
                                // f2* > 1/2 and the result may be exact
                                // Calculate f2* - 1/2
                                tmp64 = fstar.w[2] - BID_ONEHALF128[(ind - 1) as usize];
                                tmp64A = fstar.w[3];
                                if tmp64 > fstar.w[2] {
                                    tmp64A -= 1;
                                }
                                if  tmp64A != 0 || tmp64 != 0
                                 || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                 && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                }	// else the result is exact
                            } else {	// the result is inexact; f2* <= 1/2
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            }
                        } else {	// if 22 <= ind <= 33
                            if  fstar.w[3] > BID_ONEHALF128[(ind - 1) as usize]
                            || (fstar.w[3] == BID_ONEHALF128[(ind - 1) as usize]
                            && (fstar.w[2] != 0 || fstar.w[1] != 0 || fstar.w[0] != 0)) {
                                // f2* > 1/2 and the result may be exact
                                // Calculate f2* - 1/2
                                tmp64 = fstar.w[3] - BID_ONEHALF128[(ind - 1) as usize];
                                if  tmp64 != 0 || fstar.w[2] != 0
                                 || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                 && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                }	// else the result is exact
                            } else {	// the result is inexact; f2* <= 1/2
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            }
                        }

                        // if the result was a midpoint it was rounded away from zero, so
                        // it will need a correction
                        // check for midpoints
                        if (fstar.w[3] == 0) && (fstar.w[2] == 0)
                        && (fstar.w[1] != 0 || fstar.w[0] != 0)
                        && (fstar.w[1]  < BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0] <= BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0])) {
                            // the result is a midpoint; round to nearest
                            if (Cstar.w[0] & 0x01) == 0x01 {	// Cstar.w[0] is odd; MP in [EVEN, ODD]
                                // if floor(C*) is odd C = floor(C*) - 1; the result >= 1
                                Cstar.w[0] -= 1;	// Cstar.w[0] is now even
                            }	// else MP in [ODD, EVEN]
                        }
                        res = Cstar.w[0];	// the result is positive
                    },
                    value if value == 0 => {
                        // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                        // res = C (exact)
                        res = C1.w[0];
                    },
                    _ => {
                        // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                        // res = C * 10^exp (exact) - must fit in 64 bits
                        res = C1.w[0] * BID_TEN2K64[exp as  usize];
                    }
                }
            }
        }
    }

    res
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-down mode; inexact exceptions not signaled
pub (crate) fn bid128_to_uint64_floor(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let P256: BID_UINT256;

    // unpack x
    x_sign  = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp   = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        // if (x.w[1] & MASK_NAN) == MASK_NAN {	// x is NAN
        //     if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if x_sign == 0 {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        // if n < 0 then n cannot be converted to uint64 with RM
        if x_sign != 0 {	// if n < 0 and q + exp = 20
            // too large if c(0)c(1)...c(19).c(20)...c(q-1) > 0
            // set invalid flag
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
            // return Integer Indefinite
            res = 0x8000000000000000u64;
            return res;
        }
        unsafe  {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	// exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                tmp1.d    = C1.w[1] as f64;	// exact conversion
                x_nr_bits =
                        65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
            }
        }
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1]  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
             }
        }
        exp = ((x_exp >> 49) - 6176) as i32;
        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                // if n > 0 and q + exp = 20
                // if n >= 2^64 then n is too large
                // <=> c(0)c(1)...c(19).c(20)...c(q-1) >= 2^64
                // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 >= 2^64
                // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 >= 5*2^65
                // <=> C * 10^(21-q) >= 0xa0000000000000000, 1<=q<=34
                if q == 1 {
                    // C * 10^20 >= 0xa0000000000000000
                    C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                    if C.w[1] >= 0x0a {
                        // actually C.w[1] == 0x0a && C.w[0] >= 0x0000000000000000u64) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                } else if q <= 19 {
                    // C * 10^(21-q) >= 0xa0000000000000000
                    C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                    if C.w[1] >= 0x0a {
                        // actually C.w[1] == 0x0a && C.w[0] >= 0x0000000000000000u64) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                } else if q == 20 {
                    // C >= 0x10000000000000000
                    if C1.w[1] >= 0x01 {
                        // actually C1.w[1] == 0x01 && C1.w[0] >= 0x0000000000000000u64) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                } else if q == 21 {
                    // C >= 0xa0000000000000000
                    if C1.w[1] >= 0x0a {
                        // actually C1.w[1] == 0x0a && C1.w[0] >= 0x0000000000000000u64) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                    // C  >= 10^(q-21) * 0xa0000000000000000 max 44 bits x 68 bits
                    C.w[1] = 0x0a;
                    C.w[0] = 0x0000000000000000u64;
                    C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                    if C1.w[1] > C.w[1] || (C1.w[1] == C.w[1] && C1.w[0] >= C.w[0]) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                }
            },
            _ => { }
        }
        // n is not too large to be converted to int64 if 0 <= n < 2^64
        // Note: some of the cases tested for above fall through to this point
        if (q + exp) <= 0 {	// n = +0.[0...0]c(0)c(1)...c(q-1)
            // return 0
            res = 0x0000000000000000u64;
            return res;
        } else {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
            // 1 <= x < 2^64 so x can be rounded
            // down to a 64-bit unsigned signed integer
            match exp {
                value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                    ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                    // chop off ind digits from the lower part of C1
                    // C1 fits in 127 bits
                    // calculate C* and f*
                    // C* is actually floor(C*) in this case
                    // C* and f* need shifting and masking, as shown by
                    // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                    // 1 <= x <= 33
                    // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                    // C* = C1 * 10^(-x)
                    // the approximation of 10^(-x) was rounded up to 118 bits
                    P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                    if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        Cstar.w[1] = P256.w[3];
                        Cstar.w[0] = P256.w[2];
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[1] = 0;
                        Cstar.w[0] = P256.w[3];
                    }
                    // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                    // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                    // C* = floor(C*) (logical right shift; C has p decimal digits,
                    //     correct by Property 1)
                    // n = C* * 10^(e+x)

                    // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                    shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                    Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                        // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                    };
                    res = Cstar.w[0];	// the result is positive
                },
                value if value == 0 => {
                    // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                    // res = C (exact)
                    res = C1.w[0];
                },
                _ => {
                    // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                    // res = C * 10^exp (exact) - must fit in 64 bits
                    res = C1.w[0] * BID_TEN2K64[exp as  usize];
                }
            }
        }
    }

    res
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-down mode; inexact exceptions signaled
pub (crate) fn bid128_to_uint64_xfloor(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let mut fstar: BID_UINT256 = BID_UINT256::default();
    let P256: BID_UINT256;

    // unpack x
    x_sign = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        // if (x.w[1] & MASK_NAN) == MASK_NAN {	// x is NAN
        //     if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if x_sign == 0 {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        // if n < 0 then n cannot be converted to uint64 with RM
        if x_sign != 0 {	// if n < 0 and q + exp = 20
            // too large if c(0)c(1)...c(19).c(20)...c(q-1) > 0
            // set invalid flag
            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
            // return Integer Indefinite
            res = 0x8000000000000000u64;
            return res;
        }
        unsafe {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	// exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                tmp1.d    = C1.w[1] as f64;	// exact conversion
                x_nr_bits = 65 + ((((tmp1.i >> 52) as u32)  & 0x7ff) - 0x3ff);
            }
        }
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1]  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
             }
        }
        exp = ((x_exp >> 49) - 6176) as i32;
        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                // if n > 0 and q + exp = 20
                // if n >= 2^64 then n is too large
                // <=> c(0)c(1)...c(19).c(20)...c(q-1) >= 2^64
                // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 >= 2^64
                // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 >= 5*2^65
                // <=> C * 10^(21-q) >= 0xa0000000000000000, 1<=q<=34
                if q == 1 {
                    // C * 10^20 >= 0xa0000000000000000
                    C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                    if C.w[1] >= 0x0a {
                        // actually C.w[1] == 0x0a && C.w[0] >= 0x0000000000000000u64) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                } else if q <= 19 {
                    // C * 10^(21-q) >= 0xa0000000000000000
                    C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                    if C.w[1] >= 0x0a {
                        // actually C.w[1] == 0x0a && C.w[0] >= 0x0000000000000000u64) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                } else if q == 20 {
                    // C >= 0x10000000000000000
                    if C1.w[1] >= 0x01 {
                        // actually C1.w[1] == 0x01 && C1.w[0] >= 0x0000000000000000u64) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                } else if q == 21 {
                    // C >= 0xa0000000000000000
                    if C1.w[1] >= 0x0a {
                        // actually C1.w[1] == 0x0a && C1.w[0] >= 0x0000000000000000u64) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                    // C  >= 10^(q-21) * 0xa0000000000000000 max 44 bits x 68 bits
                    C.w[1] = 0x0a;
                    C.w[0] = 0x0000000000000000u64;
                    C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                    if C1.w[1] > C.w[1] || (C1.w[1] == C.w[1] && C1.w[0] >= C.w[0]) {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                    // else cases that can be rounded to a 64-bit int fall through
                    // to '1 <= q + exp <= 20'
                }
            },
            _ => { }
        }
        // n is not too large to be converted to int64 if 0 <= n < 2^64
        // Note: some of the cases tested for above fall through to this point
        if (q + exp) <= 0 {	// n = +0.[0...0]c(0)c(1)...c(q-1)
            // set inexact flag
            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            // return 0
            res = 0x0000000000000000u64;
            return res;
        } else {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
            // 1 <= x < 2^64 so x can be rounded
            // down to a 64-bit unsigned signed integer
            match exp {
                value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                    ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                    // chop off ind digits from the lower part of C1
                    // C1 fits in 127 bits
                    // calculate C* and f*
                    // C* is actually floor(C*) in this case
                    // C* and f* need shifting and masking, as shown by
                    // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                    // 1 <= x <= 33
                    // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                    // C* = C1 * 10^(-x)
                    // the approximation of 10^(-x) was rounded up to 118 bits
                    P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                    if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        Cstar.w[1] = P256.w[3];
                        Cstar.w[0] = P256.w[2];
                        fstar.w[3] = 0;
                        fstar.w[2] = P256.w[2] & BID_MASKHIGH128[(ind - 1) as usize];
                        fstar.w[1] = P256.w[1];
                        fstar.w[0] = P256.w[0];
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[1] = 0;
                        Cstar.w[0] = P256.w[3];
                        fstar.w[3] = P256.w[3] & BID_MASKHIGH128[(ind - 1) as usize];
                        fstar.w[2] = P256.w[2];
                        fstar.w[1] = P256.w[1];
                        fstar.w[0] = P256.w[0];
                    }
                    // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                    // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                    // C* = floor(C*) (logical right shift; C has p decimal digits,
                    //     correct by Property 1)
                    // n = C* * 10^(e+x)

                    // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                    shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                    Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                        // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                    };
                    // determine inexactness of the rounding of C*
                    // if (0 < f* < 10^(-x)) then
                    //   the result is exact
                    // else // if (f* > T*) then
                    //   the result is inexact
                    if ind - 1 <= 2 {
                        if  fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                        }	// else the result is exact
                    } else if ind - 1 <= 21 {	// if 3 <= ind <= 21
                        if  fstar.w[2] != 0 || fstar.w[1] > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                        }	// else the result is exact
                    } else {	// if 22 <= ind <= 33
                        if  fstar.w[3] != 0 || fstar.w[2] != 0
                         || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                        }	// else the result is exact
                    }

                    res = Cstar.w[0];	// the result is positive
                },
                value if value == 0 => {
                    // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                    // res = C (exact)
                    res = C1.w[0];
                },
                _ => {
                    // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                    // res = C * 10^exp (exact) - must fit in 64 bits
                    res = C1.w[0] * BID_TEN2K64[exp as  usize];
                }
            }
        }
    }

    res
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-up mode; inexact exceptions not signaled
pub (crate) fn bid128_to_uint64_ceil(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let mut fstar: BID_UINT256 = BID_UINT256::default();
    let P256: BID_UINT256;

    // unpack x
    x_sign  = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp   = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        // if (x.w[1] & MASK_NAN) == MASK_NAN {	// x is NAN
        //     if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if x_sign == 0 {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        unsafe {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	        // exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
            tmp1.d    = C1.w[1] as f64;	                // exact conversion
            x_nr_bits = 65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
            }
        }

        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1]  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
            }
        }
        exp = ((x_exp >> 49) - 6176) as i32;
        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                if x_sign != 0 {	// if n < 0 and q + exp = 20
                    // if n <= -1 then n cannot be converted to uint64 with RZ
                    // too large if c(0)c(1)...c(19).c(20)...c(q-1) >= 1
                    // <=> 0.c(0)c(1)...c(q-1) * 10^21 >= 0x0a, 1<=q<=34
                    // <=> C * 10^(21-q) >= 0x0a, 1<=q<=34
                    if q == 21 {
                        // C >= a
                        if C1.w[1] != 0 || C1.w[0] >= 0x0au64 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to 64-bit unsigned int fall through
                        // to '1 <= q + exp <= 20'
                    } else {
                        // if 1 <= q <= 20
                        //   C * 10^(21-q) >= a is true because C >= 1 and 10^(21-q) >= 10
                        // if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        //  C >= a * 10^(q-21) is true because C > 2^64 and a*10^(q-21) < 2^64
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// if n > 0 and q + exp = 20
                    // if n > 2^64 - 1 then n is too large
                    // <=> c(0)c(1)...c(19).c(20)...c(q-1) > 2^64 - 1
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 > 2^64 - 1
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 > 10 * (2^64 - 1)
                    // <=> C * 10^(21-q) > 0x9fffffffffffffff6, 1<=q<=34
                    if q == 1 {
                        // C * 10^20 > 0x9fffffffffffffff6
                        C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] > 0xfffffffffffffff6u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q <= 19 {
                        // C * 10^(21-q) > 0x9fffffffffffffff6
                        C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] > 0xfffffffffffffff6u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 20 {
                        // C > 0xffffffffffffffff
                        if C1.w[1] != 0 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 21 {
                        // C > 0x9fffffffffffffff6
                        if C1.w[1] > 0x09 || (C1.w[1] == 0x09 && C1.w[0] > 0xfffffffffffffff6u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        // C  > 10^(q-21) * 0x9fffffffffffffff6 max 44 bits x 68 bits
                        C.w[1] = 0x09;
                        C.w[0] = 0xfffffffffffffff6u64;
                        C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                        if C1.w[1] > C.w[1] || (C1.w[1] == C.w[1] && C1.w[0] > C.w[0]) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    }
                }
            },
            _ => { }
        }
        // n is not too large to be converted to int64 if -1 < n <= 2^64 - 1
        // Note: some of the cases tested for above fall through to this point
        if (q + exp) <= 0 {	// n = +/-0.[0...0]c(0)c(1)...c(q-1)
            // return 0 or 1
            res = if x_sign != 0 { 0x0000000000000000u64 } else { 0x0000000000000001u64 };
            return res;
        } else {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
            // x <= -1 or 1 <= x < 2^64 so if positive x can be rounded
            // to zero to a 64-bit unsigned signed integer
            if x_sign != 0 {	// x <= -1
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            }
            // 1 <= x <= 2^64 - 1 so x can be rounded
            // to zero to a 64-bit unsigned integer
            match exp {
                value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                    ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                    // chop off ind digits from the lower part of C1
                    // C1 fits in 127 bits
                    // calculate C* and f*
                    // C* is actually floor(C*) in this case
                    // C* and f* need shifting and masking, as shown by
                    // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                    // 1 <= x <= 33
                    // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                    // C* = C1 * 10^(-x)
                    // the approximation of 10^(-x) was rounded up to 118 bits
                    P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                    if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        Cstar.w[1] = P256.w[3];
                        Cstar.w[0] = P256.w[2];
                        fstar.w[3] = 0;
                        fstar.w[2] = P256.w[2] & BID_MASKHIGH128[(ind - 1) as usize];
                        fstar.w[1] = P256.w[1];
                        fstar.w[0] = P256.w[0];
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[1] = 0;
                        Cstar.w[0] = P256.w[3];
                        fstar.w[3] = P256.w[3] & BID_MASKHIGH128[(ind - 1) as usize];
                        fstar.w[2] = P256.w[2];
                        fstar.w[1] = P256.w[1];
                        fstar.w[0] = P256.w[0];
                    }
                    // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                    // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                    // C* = floor(C*) (logical right shift; C has p decimal digits,
                    //     correct by Property 1)
                    // n = C* * 10^(e+x)

                    // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                    shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                    Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                        // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                    };
                    // if the result is positive and inexact, need to add 1 to it

                    // determine inexactness of the rounding of C*
                    // if (0 < f* < 10^(-x)) then
                    //   the result is exact
                    // else // if (f* > T*) then
                    //   the result is inexact
                    if ind - 1 <= 2 {
                        if  fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            if x_sign == 0 {	// positive and inexact
                                Cstar.w[0] += 1;
                                if Cstar.w[0] == 0x0 {
                                    Cstar.w[1] += 1;
                                }
                            }
                        }	// else the result is exact
                    } else if ind - 1 <= 21 {	// if 3 <= ind <= 21
                        if  fstar.w[2] != 0
                         || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            if x_sign == 0 {	// positive and inexact
                                Cstar.w[0] += 1;
                                if Cstar .w[0] == 0x0 {
                                    Cstar.w[1] += 1;
                                }
                            }
                        }	// else the result is exact
                    } else {	// if 22 <= ind <= 33
                        if  fstar.w[3] != 0 || fstar.w[2] != 0
                         || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            if x_sign == 0 {	// positive and inexact
                                Cstar.w[0] += 1;
                                if Cstar.w[0] == 0x0 {
                                    Cstar.w[1] += 1;
                                }
                            }
                        }	// else the result is exact
                    }

                    res = Cstar.w[0];	// the result is positive
                },
                value if value == 0 => {
                    // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                    // res = C (exact)
                    res = C1.w[0];
                },
                _ => {
                    // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                    // res = C * 10^exp (exact) - must fit in 64 bits
                    res = C1.w[0] * BID_TEN2K64[exp as  usize];
                }
            }
        }
    }

    res
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-up mode; inexact exceptions signaled
pub (crate) fn bid128_to_uint64_xceil(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let mut fstar: BID_UINT256 = BID_UINT256::default();
    let P256: BID_UINT256;

    // unpack x
    x_sign = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // // x is special
        // if (x.w[1] & MASK_NAN) == MASK_NAN {	// x is NAN
        //     if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if x_sign == 0 {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        unsafe {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	        // exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                tmp1.d    = C1.w[1] as f64;	// exact conversion
                x_nr_bits = 65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
            }
        }

        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1]  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
             }
        }
        exp = ((x_exp >> 49) - 6176) as i32;
        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                if x_sign != 0 {	// if n < 0 and q + exp = 20
                    // if n <= -1 then n cannot be converted to uint64 with RZ
                    // too large if c(0)c(1)...c(19).c(20)...c(q-1) >= 1
                    // <=> 0.c(0)c(1)...c(q-1) * 10^21 >= 0x0a, 1<=q<=34
                    // <=> C * 10^(21-q) >= 0x0a, 1<=q<=34
                    if q == 21 {
                        // C >= a
                        if C1.w[1] != 0 || C1.w[0] >= 0x0au64 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to 64-bit unsigned int fall through
                        // to '1 <= q + exp <= 20'
                    } else {
                        // if 1 <= q <= 20
                        //   C * 10^(21-q) >= a is true because C >= 1 and 10^(21-q) >= 10
                        // if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        //  C >= a * 10^(q-21) is true because C > 2^64 and a*10^(q-21) < 2^64
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// if n > 0 and q + exp = 20
                    // if n > 2^64 - 1 then n is too large
                    // <=> c(0)c(1)...c(19).c(20)...c(q-1) > 2^64 - 1
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 > 2^64 - 1
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 > 10 * (2^64 - 1)
                    // <=> C * 10^(21-q) > 0x9fffffffffffffff6, 1<=q<=34
                    if q == 1 {
                        // C * 10^20 > 0x9fffffffffffffff6
                        C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] > 0xfffffffffffffff6u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q <= 19 {
                        // C * 10^(21-q) > 0x9fffffffffffffff6
                        C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] > 0xfffffffffffffff6u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 20 {
                        // C > 0xffffffffffffffff
                        if C1.w[1] != 0 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 21 {
                        // C > 0x9fffffffffffffff6
                        if C1.w[1] > 0x09 || (C1.w[1] == 0x09 && C1.w[0] > 0xfffffffffffffff6u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        // C  > 10^(q-21) * 0x9fffffffffffffff6 max 44 bits x 68 bits
                        C.w[1] = 0x09;
                        C.w[0] = 0xfffffffffffffff6u64;
                        C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                        if C1.w[1] > C.w[1] || (C1.w[1] == C.w[1] && C1.w[0] > C.w[0]) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    }
                }
            },
            _ => { }
        }
        // n is not too large to be converted to int64 if -1 < n <= 2^64 - 1
        // Note: some of the cases tested for above fall through to this point
        if (q + exp) <= 0 {	// n = +/-0.[0...0]c(0)c(1)...c(q-1)
            // set inexact flag
            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            // return 0 or 1
            res = if x_sign != 0 { 0x0000000000000000u64 } else { 0x0000000000000001u64 };
            return res;
        } else {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
            // x <= -1 or 1 <= x < 2^64 so if positive x can be rounded
            // to zero to a 64-bit unsigned signed integer
            if x_sign != 0 {	// x <= -1
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            }
            // 1 <= x <= 2^64 - 1 so x can be rounded
            // to zero to a 64-bit unsigned integer
            match exp {
                value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                    ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                    // chop off ind digits from the lower part of C1
                    // C1 fits in 127 bits
                    // calculate C* and f*
                    // C* is actually floor(C*) in this case
                    // C* and f* need shifting and masking, as shown by
                    // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                    // 1 <= x <= 33
                    // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                    // C* = C1 * 10^(-x)
                    // the approximation of 10^(-x) was rounded up to 118 bits
                    P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                    if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        Cstar.w[1] = P256.w[3];
                        Cstar.w[0] = P256.w[2];
                        fstar.w[3] = 0;
                        fstar.w[2] = P256.w[2] & BID_MASKHIGH128[(ind - 1) as usize];
                        fstar.w[1] = P256.w[1];
                        fstar.w[0] = P256.w[0];
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[1] = 0;
                        Cstar.w[0] = P256.w[3];
                        fstar.w[3] = P256.w[3] & BID_MASKHIGH128[(ind - 1) as usize];
                        fstar.w[2] = P256.w[2];
                        fstar.w[1] = P256.w[1];
                        fstar.w[0] = P256.w[0];
                    }
                    // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                    // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                    // C* = floor(C*) (logical right shift; C has p decimal digits,
                    //     correct by Property 1)
                    // n = C* * 10^(e+x)

                    // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                    shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                    Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                        // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                    };
                    // if the result is positive and inexact, need to add 1 to it

                    // determine inexactness of the rounding of C*
                    // if (0 < f* < 10^(-x)) then
                    //   the result is exact
                    // else // if (f* > T*) then
                    //   the result is inexact
                    if ind - 1 <= 2 {
                        if  fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            if x_sign == 0 {	// positive and inexact
                                Cstar.w[0] += 1;
                                if Cstar.w[0] == 0x0 {
                                    Cstar.w[1] += 1;
                                }
                            }
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                        }	// else the result is exact
                    } else if ind - 1 <= 21 {	// if 3 <= ind <= 21
                        if  fstar.w[2] != 0
                         || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            if x_sign == 0 {	// positive and inexact
                                Cstar.w[0] += 1;
                                if Cstar.w[0] == 0x0 {
                                    Cstar.w[1] += 1;
                                }
                            }
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                        }	// else the result is exact
                    } else {	// if 22 <= ind <= 33
                        if  fstar.w[3] != 0 || fstar.w[2] != 0
                         || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            if x_sign == 0 {	// positive and inexact
                                Cstar.w[0] += 1;
                                if Cstar.w[0] == 0x0 {
                                    Cstar.w[1] += 1;
                                }
                            }
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                        }	// else the result is exact
                    }

                    res = Cstar.w[0];	// the result is positive
                },
                value if value == 0 => {
                    // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                    // res = C (exact)
                    res = C1.w[0];
                },
                _ => {
                    // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                    // res = C * 10^exp (exact) - must fit in 64 bits
                    res = C1.w[0] * BID_TEN2K64[exp as  usize];
                }
            }
        }
    }

    res
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-to-zero; inexact exceptions not signaled
pub (crate) fn bid128_to_uint64_int(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let P256: BID_UINT256;

    // unpack x
    x_sign  = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp   = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        // if (x.w[1] & MASK_NAN) == MASK_NAN {	// x is NAN
        //     if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if x_sign == 0 {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        unsafe {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	// exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                tmp1.d    = C1.w[1] as f64;	// exact conversion
                x_nr_bits = 65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
            }
        }
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1] > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
             }
        }
        exp = ((x_exp >> 49) - 6176) as i32;
        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                if x_sign != 0 {	// if n < 0 and q + exp = 20
                    // if n <= -1 then n cannot be converted to uint64 with RZ
                    // too large if c(0)c(1)...c(19).c(20)...c(q-1) >= 1
                    // <=> 0.c(0)c(1)...c(q-1) * 10^21 >= 0x0a, 1<=q<=34
                    // <=> C * 10^(21-q) >= 0x0a, 1<=q<=34
                    if q == 21 {
                        // C >= a
                        if C1.w[1] != 0 || C1.w[0] >= 0x0au64 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to 64-bit unsigned int fall through
                        // to '1 <= q + exp <= 20'
                    } else {
                        // if 1 <= q <= 20
                        //   C * 10^(21-q) >= a is true because C >= 1 and 10^(21-q) >= 10
                        // if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        //  C >= a * 10^(q-21) is true because C > 2^64 and a*10^(q-21) < 2^64
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// if n > 0 and q + exp = 20
                    // if n >= 2^64 then n is too large
                    // <=> c(0)c(1)...c(19).c(20)...c(q-1) >= 2^64
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 >= 2^64
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 >= 5*2^65
                    // <=> C * 10^(21-q) >= 0xa0000000000000000, 1<=q<=34
                    if q == 1 {
                        // C * 10^20 >= 0xa0000000000000000
                        C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                        if C.w[1] >= 0x0a {
                            // actually C.w[1] == 0x0a && C.w[0] >= 0x0000000000000000u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q <= 19 {
                        // C * 10^(21-q) >= 0xa0000000000000000
                        C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                        if C.w[1] >= 0x0a {
                            // actually C.w[1] == 0x0a && C.w[0] >= 0x0000000000000000u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 20 {
                        // C >= 0x10000000000000000
                        if C1.w[1] >= 0x01 {
                            // actually C1.w[1] == 0x01 && C1.w[0] >= 0x0000000000000000u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 21 {
                        // C >= 0xa0000000000000000
                        if C1.w[1] >= 0x0a {
                            // actually C1.w[1] == 0x0a && C1.w[0] >= 0x0000000000000000u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        // C  >= 10^(q-21) * 0xa0000000000000000 max 44 bits x 68 bits
                        C.w[1] = 0x0a;
                        C.w[0] = 0x0000000000000000u64;
                        C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                        if C1.w[1] > C.w[1]
                                || (C1.w[1] == C.w[1] && C1.w[0] >= C.w[0]) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    }
                }
            },
            _ => { }
        }
        // n is not too large to be converted to int64 if -1 < n < 2^64
        // Note: some of the cases tested for above fall through to this point
        if (q + exp) <= 0 {	// n = +/-0.[0...0]c(0)c(1)...c(q-1)
            // return 0
            res = 0x0000000000000000u64;
            return res;
        } else {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
            // x <= -1 or 1 <= x < 2^64 so if positive x can be rounded
            // to zero to a 64-bit unsigned signed integer
            if x_sign != 0 {	// x <= -1
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            }
            // 1 <= x < 2^64 so x can be rounded
            // to zero to a 64-bit unsigned integer
            match exp {
                value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                    ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                    // chop off ind digits from the lower part of C1
                    // C1 fits in 127 bits
                    // calculate C* and f*
                    // C* is actually floor(C*) in this case
                    // C* and f* need shifting and masking, as shown by
                    // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                    // 1 <= x <= 33
                    // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                    // C* = C1 * 10^(-x)
                    // the approximation of 10^(-x) was rounded up to 118 bits
                    P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                    if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        Cstar.w[1] = P256.w[3];
                        Cstar.w[0] = P256.w[2];
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[1] = 0;
                        Cstar.w[0] = P256.w[3];
                    }
                    // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                    // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                    // C* = floor(C*) (logical right shift; C has p decimal digits,
                    //     correct by Property 1)
                    // n = C* * 10^(e+x)

                    // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                    shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                    Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                        // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                    };
                    res = Cstar.w[0];	// the result is positive
                },
                value if value == 0 => {
                    // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                    // res = C (exact)
                    res = C1.w[0];
                },
                _ => {
                    // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                    // res = C * 10^exp (exact) - must fit in 64 bits
                    res = C1.w[0] * BID_TEN2K64[exp as  usize];
                }
            }
        }
    }

    res
}

//------------------------ desde aqui --

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-to-zero; inexact exceptions signaled
pub (crate) fn bid128_to_uint64_xint(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let mut fstar: BID_UINT256 = BID_UINT256::default();
    let P256: BID_UINT256;

    // unpack x
    x_sign  = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp   = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        // if (x.w[1] & MASK_NAN) == MASK_NAN {	// x is NAN
        //     if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if x_sign == 0 {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        unsafe {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	// exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                tmp1.d    = C1.w[1] as f64;	// exact conversion
                x_nr_bits = 65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
            }
        }
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1]  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
             }
        }
        exp = ((x_exp >> 49) - 6176) as i32;
        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                if x_sign != 0 {	// if n < 0 and q + exp = 20
                    // if n <= -1 then n cannot be converted to uint64 with RZ
                    // too large if c(0)c(1)...c(19).c(20)...c(q-1) >= 1
                    // <=> 0.c(0)c(1)...c(q-1) * 10^21 >= 0x0a, 1<=q<=34
                    // <=> C * 10^(21-q) >= 0x0a, 1<=q<=34
                    if q == 21 {
                        // C >= a
                        if C1.w[1] != 0 || C1.w[0] >= 0x0au64 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to 64-bit unsigned int fall through
                        // to '1 <= q + exp <= 20'
                    } else {
                        // if 1 <= q <= 20
                        //   C * 10^(21-q) >= a is true because C >= 1 and 10^(21-q) >= 10
                        // if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        //  C >= a * 10^(q-21) is true because C > 2^64 and a*10^(q-21) < 2^64
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// if n > 0 and q + exp = 20
                    // if n >= 2^64 then n is too large
                    // <=> c(0)c(1)...c(19).c(20)...c(q-1) >= 2^64
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 >= 2^64
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 >= 5*2^65
                    // <=> C * 10^(21-q) >= 0xa0000000000000000, 1<=q<=34
                    if q == 1 {
                        // C * 10^20 >= 0xa0000000000000000
                        C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                        if C.w[1] >= 0x0a {
                            // actually C.w[1] == 0x0a && C.w[0] >= 0x0000000000000000u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q <= 19 {
                        // C * 10^(21-q) >= 0xa0000000000000000
                        C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                        if C.w[1] >= 0x0a {
                            // actually C.w[1] == 0x0a && C.w[0] >= 0x0000000000000000u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 20 {
                        // C >= 0x10000000000000000
                        if C1.w[1] >= 0x01 {
                            // actually C1.w[1] == 0x01 && C1.w[0] >= 0x0000000000000000u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 21 {
                        // C >= 0xa0000000000000000
                        if C1.w[1] >= 0x0a {
                            // actually C1.w[1] == 0x0a && C1.w[0] >= 0x0000000000000000u64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        // C  >= 10^(q-21) * 0xa0000000000000000 max 44 bits x 68 bits
                        C.w[1] = 0x0a;
                        C.w[0] = 0x0000000000000000u64;
                        C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                        if C1.w[1] > C.w[1] || (C1.w[1] == C.w[1] && C1.w[0] >= C.w[0]) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    }
                }
            },
            _ => { }
        }
        // n is not too large to be converted to int64 if -1 < n < 2^64
        // Note: some of the cases tested for above fall through to this point
        if (q + exp) <= 0 {	// n = +/-0.[0...0]c(0)c(1)...c(q-1)
            // set inexact flag
            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            // return 0
            res = 0x0000000000000000u64;
            return res;
        } else {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
            // x <= -1 or 1 <= x < 2^64 so if positive x can be rounded
            // to zero to a 64-bit unsigned signed integer
            if x_sign != 0 {	// x <= -1
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            }
            // 1 <= x < 2^64 so x can be rounded
            // to zero to a 64-bit unsigned integer
            match exp {
                value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                    ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                    // chop off ind digits from the lower part of C1
                    // C1 fits in 127 bits
                    // calculate C* and f*
                    // C* is actually floor(C*) in this case
                    // C* and f* need shifting and masking, as shown by
                    // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                    // 1 <= x <= 33
                    // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                    // C* = C1 * 10^(-x)
                    // the approximation of 10^(-x) was rounded up to 118 bits
                    P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                    if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        Cstar.w[1] = P256.w[3];
                        Cstar.w[0] = P256.w[2];
                        fstar.w[3] = 0;
                        fstar.w[2] = P256.w[2] & BID_MASKHIGH128[(ind - 1) as usize];
                        fstar.w[1] = P256.w[1];
                        fstar.w[0] = P256.w[0];
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[1] = 0;
                        Cstar.w[0] = P256.w[3];
                        fstar.w[3] = P256.w[3] & BID_MASKHIGH128[(ind - 1) as usize];
                        fstar.w[2] = P256.w[2];
                        fstar.w[1] = P256.w[1];
                        fstar.w[0] = P256.w[0];
                    }
                    // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                    // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                    // C* = floor(C*) (logical right shift; C has p decimal digits,
                    //     correct by Property 1)
                    // n = C* * 10^(e+x)

                    // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                    shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                    Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                        (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                        // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                    } else {	// 22 <= ind - 1 <= 33
                        Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                    };
                    // determine inexactness of the rounding of C*
                    // if (0 < f* < 10^(-x)) then
                    //   the result is exact
                    // else // if (f* > T*) then
                    //   the result is inexact
                    if ind - 1 <= 2 {
                        if  fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                        }	// else the result is exact
                    } else if ind - 1 <= 21 {	// if 3 <= ind <= 21
                        if  fstar.w[2] != 0
                         || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                        }	// else the result is exact
                    } else {	// if 22 <= ind <= 33
                        if  fstar.w[3] != 0 || fstar.w[2] != 0
                         || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                        || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                         && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                            // set the inexact flag
                            *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                        }	// else the result is exact
                    }

                    res = Cstar.w[0];	// the result is positive
                },
                value if value == 0 => {
                    // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                    // res = C (exact)
                    res = C1.w[0];
                },
                _ => {
                    // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                    // res = C * 10^exp (exact) - must fit in 64 bits
                    res = C1.w[0] * BID_TEN2K64[exp as  usize];
                }
            }
        }
    }

    res
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-to-nearest-away; inexact exceptions not signaled
pub (crate) fn bid128_to_uint64_rninta(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let tmp64: BID_UINT64;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let P256: BID_UINT256;

    // unpack x
    x_sign  = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp   = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        // if (x.w[1] & MASK_NAN) == MASK_NAN {	        // x is NAN
        //     if (x.w[1] & MASK_SNAN) == MASK_SNAN {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if x_sign == 0 {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        unsafe {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	// exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                tmp1.d    = C1.w[1] as f64;	// exact conversion
                x_nr_bits = 65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
            }
        }
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1]  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
            }
        }
        exp = ((x_exp >> 49) - 6176) as i32;
        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                if x_sign != 0 {	// if n < 0 and q + exp = 20
                    // if n <= -1/2 then n cannot be converted to uint64 with RN
                    // too large if c(0)c(1)...c(19).c(20)...c(q-1) >= 1/2
                    // <=> 0.c(0)c(1)...c(q-1) * 10^21 >= 0x05, 1<=q<=34
                    // <=> C * 10^(21-q) >= 0x05, 1<=q<=34
                    if q == 21 {
                        // C >= 5
                        if C1.w[1] != 0 || C1.w[0] >= 0x05u64 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to 64-bit unsigned int fall through
                        // to '1 <= q + exp <= 20'
                    } else {
                        // if 1 <= q <= 20
                        //   C * 10^(21-q) >= 5 is true because C >= 1 and 10^(21-q) >= 10
                        // if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        //  C >= 5 * 10^(q-21) is true because C > 2^64 and 5*10^(q-21) < 2^64
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// if n > 0 and q + exp = 20
                    // if n >= 2^64 - 1/2 then n is too large
                    // <=> c(0)c(1)...c(19).c(20)...c(q-1) >= 2^64-1/2
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 >= 2^64-1/2
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 >= 5*(2^65-1)
                    // <=> C * 10^(21-q) >= 0x9fffffffffffffffb, 1<=q<=34
                    if q == 1 {
                        // C * 10^20 >= 0x9fffffffffffffffb
                        C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q <= 19 {
                        // C * 10^(21-q) >= 0x9fffffffffffffffb
                        C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 20 {
                        // C * 10 >= 0x9fffffffffffffffb <=> C * 2 > 1ffffffffffffffff
                        C.w[0] = C1.w[0] + C1.w[0];
                        C.w[1] = C1.w[1] + C1.w[1];
                        if C.w[0] < C1.w[0] {
                            C.w[1] += 1;
                        }
                        if C.w[1] > 0x01 || (C.w[1] == 0x01 && C.w[0] >= 0xffffffffffffffffu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 21 {
                        // C >= 0x9fffffffffffffffb
                        if C1.w[1] > 0x09 || (C1.w[1] == 0x09 && C1.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        // C  >= 10^(q-21) * 0x9fffffffffffffffb max 44 bits x 68 bits
                        C.w[1] = 0x09;
                        C.w[0] = 0xfffffffffffffffbu64;
                        C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                        if C1.w[1] > C.w[1] || (C1.w[1] == C.w[1] && C1.w[0] >= C.w[0]) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    }
                }
            },
            _ => { }
        }
        // n is not too large to be converted to int64 if -1/2 < n < 2^64 - 1/2
        // Note: some of the cases tested for above fall through to this point
        match q + exp {
            value if value < 0 => {	// n = +/-0.0...c(0)c(1)...c(q-1)
                // return 0
                res = 0x0000000000000000u64;
                return res;
            },
            value if value == 0 => {	// n = +/-0.c(0)c(1)...c(q-1)
                // if 0.c(0)c(1)...c(q-1) < 0.5 <=> c(0)c(1)...c(q-1) < 5 * 10^(q-1)
                //   res = 0
                // else if x > 0
                //   res = +1
                // else // if x < 0
                //   invalid exc
                ind = q - 1;
                if ind <= 18 {	// 0 <= ind <= 18
                    if (C1.w[1] == 0) && (C1.w[0] < BID_MIDPOINT64[ind as usize]) {
                        res = 0x0000000000000000u64;	// return 0
                    } else if x_sign == 0 {	// n > 0
                        res = 0x00000001;	// return +1
                    } else {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// 19 <= ind <= 33
                    if  (C1.w[1]  < BID_MIDPOINT128[(ind - 19) as usize].w[1])
                    || ((C1.w[1] == BID_MIDPOINT128[(ind - 19) as usize].w[1])
                     && (C1.w[0]  < BID_MIDPOINT128[(ind - 19) as usize].w[0])) {
                        res = 0x0000000000000000u64;	// return 0
                    } else if x_sign == 0 {	// n > 0
                        res = 0x00000001;	// return +1
                    } else {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                }
            },
            _ => {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
                // x <= -1 or 1 <= x < 2^64-1/2 so if positive x can be rounded
                // to nearest to a 64-bit unsigned signed integer
                if x_sign != 0 {	// x <= -1
                    // set invalid flag
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                    // return Integer Indefinite
                    res = 0x8000000000000000u64;
                    return res;
                }
                // 1 <= x < 2^64-1/2 so x can be rounded
                // to nearest to a 64-bit unsigned integer
                match exp {
                    value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                        ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                        // chop off ind digits from the lower part of C1
                        // C1 = C1 + 1/2 * 10^ind where the result C1 fits in 127 bits
                        tmp64 = C1.w[0];
                        if ind <= 19 {
                            C1.w[0] += BID_MIDPOINT64[(ind - 1) as usize];
                        } else {
                            C1.w[0] += BID_MIDPOINT128[(ind - 20) as usize].w[0];
                            C1.w[1] += BID_MIDPOINT128[(ind - 20) as usize].w[1];
                        }
                        if C1.w[0] < tmp64 {
                            C1.w[1] += 1;
                        }
                        // calculate C* and f*
                        // C* is actually floor(C*) in this case
                        // C* and f* need shifting and masking, as shown by
                        // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                        // 1 <= x <= 33
                        // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                        // C* = (C1 + 1/2 * 10^x) * 10^(-x)
                        // the approximation of 10^(-x) was rounded up to 118 bits
                        P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                        if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                            Cstar.w[1] = P256.w[3];
                            Cstar.w[0] = P256.w[2];
                        } else {	// 22 <= ind - 1 <= 33
                            Cstar.w[1] = 0;
                            Cstar.w[0] = P256.w[3];
                        }
                        // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                        // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                        // if (0 < f* < 10^(-x)) then the result is a midpoint
                        //   if floor(C*) is even then C* = floor(C*) - logical right
                        //       shift; C* has p decimal digits, correct by Prop. 1)
                        //   else if floor(C*) is odd C* = floor(C*)-1 (logical right
                        //       shift; C* has p decimal digits, correct by Pr. 1)
                        // else
                        //   C* = floor(C*) (logical right shift; C has p decimal digits,
                        //       correct by Property 1)
                        // n = C* * 10^(e+x)

                        // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                        shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                        Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                            (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                            // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                        } else {	// 22 <= ind - 1 <= 33
                            Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                        };

                        // if the result was a midpoint it was rounded away from zero
                        res = Cstar.w[0];	// the result is positive
                    },
                    value if value == 0 => {
                        // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                        // res = C (exact)
                        res = C1.w[0];
                    },
                    _ => {
                        // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                        // res = C * 10^exp (exact) - must fit in 64 bits
                        res = C1.w[0] * BID_TEN2K64[exp as  usize];
                    }
                }
            }
        }
    }

    res
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned
/// integer in rounding-to-nearest-away; inexact exceptions signaled
pub (crate) fn bid128_to_uint64_xrninta(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> BID_UINT64 {
    let res: BID_UINT64;
    let x_sign: BID_UINT64;
    let x_exp: BID_UINT64;
    let exp: i32;			// unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let mut tmp64: BID_UINT64;
    let mut tmp64A: BID_UINT64;
    let mut tmp1: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let x_nr_bits: u32;
    let mut q: i32;
    let ind: i32;
    let shift: i32;
    let mut C1: BID_UINT128 = BID_UINT128::default();
    let mut C: BID_UINT128 = BID_UINT128::default();
    let mut Cstar: BID_UINT128 = BID_UINT128::default();    // C* represents up to 34 decimal digits ~ 113 bits
    let mut fstar: BID_UINT256 = BID_UINT256::default();
    let P256: BID_UINT256;

    // unpack x
    x_sign = x.w[1] & MASK_SIGN;	// 0 for positive, MASK_SIGN for negative
    x_exp = x.w[1] & MASK_EXP;	// biased and shifted left 49 bit positions
    C1.w[1] = x.w[1] & MASK_COEFF;
    C1.w[0] = x.w[0];

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        // if ((x.w[1] & MASK_NAN) == MASK_NAN) {	    // x is NAN
        //     if ((x.w[1] & MASK_SNAN) == MASK_SNAN) {	// x is SNAN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is QNaN
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // } else {	// x is not a NaN, so it must be infinity
        //     if (!x_sign) {	// x is +inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     } else {	// x is -inf
        //         // set invalid flag
        //         *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        //         // return Integer Indefinite
        //         res = 0x8000000000000000u64;
        //     }
        //     return res;
        // }
        // set invalid flag
        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
        // return Integer Indefinite
        res = 0x8000000000000000u64;
        return res;
    }
    // check for non-canonical values (after the check for special values)
    if (C1.w[1]  > 0x0001ed09bead87c0u64)
    || (C1.w[1] == 0x0001ed09bead87c0u64
    && (C1.w[0]  > 0x378d8e63ffffffffu64))
    || ((x.w[1]  & 0x6000000000000000u64) == 0x6000000000000000u64) {
        res = 0x0000000000000000u64;
        return res;
    } else if (C1.w[1] == 0x0u64) && (C1.w[0] == 0x0u64) {
        // x is 0
        res = 0x0000000000000000u64;
        return res;
    } else {	// x is not special and is not zero
        unsafe {
            // q = nr. of decimal digits in x
            //  determine first the nr. of bits in x
            if C1.w[1] == 0 {
                if C1.w[0] >= 0x0020000000000000u64 {	// x >= 2^53
                    // split the 64-bit value in two 32-bit halves to avoid rounding errors
                    tmp1.d    = (C1.w[0] >> 32) as f64;	// exact conversion
                    x_nr_bits = 33 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                } else {	// if x < 2^53
                    tmp1.d    = C1.w[0] as f64;	// exact conversion
                    x_nr_bits = 1 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
                }
            } else {	// C1.w[1] != 0 => nr. bits = 64 + nr_bits (C1.w[1])
                tmp1.d    = C1.w[1] as f64;	// exact conversion
                x_nr_bits = 65 + ((((tmp1.i >> 52) as u32) & 0x7ff) - 0x3ff);
            }
        }
        q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits as i32;
        if q == 0 {
            q = BID_NR_DIGITS[(x_nr_bits - 1) as usize].digits1 as i32;
            if  C1.w[1]  > BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
            || (C1.w[1] == BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_hi
             && C1.w[0] >= BID_NR_DIGITS[(x_nr_bits - 1) as usize].threshold_lo) {
                q += 1;
             }
        }
        exp = ((x_exp >> 49) - 6176) as i32;
        match q + exp {
            value if value > 20 => {	// x >= 10^20 ~= 2^66.45... (cannot fit in 64 bits)
                // set invalid flag
                *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                // return Integer Indefinite
                res = 0x8000000000000000u64;
                return res;
            },
            value if value == 20 => {	// x = c(0)c(1)...c(19).c(20)...c(q-1)
                // in this case 2^63.11... ~= 10^19 <= x < 10^20 ~= 2^66.43...
                // so x rounded to an integer may or may not fit in an unsigned 64-bit int
                // the cases that do not fit are identified here; the ones that fit
                // fall through and will be handled with other cases further,
                // under '1 <= q + exp <= 20'
                if x_sign != 0 {	// if n < 0 and q + exp = 20
                    // if n <= -1/2 then n cannot be converted to uint64 with RN
                    // too large if c(0)c(1)...c(19).c(20)...c(q-1) >= 1/2
                    // <=> 0.c(0)c(1)...c(q-1) * 10^21 >= 0x05, 1<=q<=34
                    // <=> C * 10^(21-q) >= 0x05, 1<=q<=34
                    if q == 21 {
                        // C >= 5
                        if C1.w[1] != 0 || C1.w[0] >= 0x05u64 {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to 64-bit unsigned int fall through
                        // to '1 <= q + exp <= 20'
                    } else {
                        // if 1 <= q <= 20
                        //   C * 10^(21-q) >= 5 is true because C >= 1 and 10^(21-q) >= 10
                        // if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        //  C >= 5 * 10^(q-21) is true because C > 2^64 and 5*10^(q-21) < 2^64
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// if n > 0 and q + exp = 20
                    // if n >= 2^64 - 1/2 then n is too large
                    // <=> c(0)c(1)...c(19).c(20)...c(q-1) >= 2^64-1/2
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^20 >= 2^64-1/2
                    // <=> 0.c(0)c(1)...c(19)c(20)...c(q-1) * 10^21 >= 5*(2^65-1)
                    // <=> C * 10^(21-q) >= 0x9fffffffffffffffb, 1<=q<=34
                    if q == 1 {
                        // C * 10^20 >= 0x9fffffffffffffffb
                        C = __mul_128x64_to_128(C1.w[0], &BID_TEN2K128[0]);	// 10^20 * C
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q <= 19 {
                        // C * 10^(21-q) >= 0x9fffffffffffffffb
                        C = __mul_64x64_to_128MACH(C1.w[0], BID_TEN2K64[(21 - q) as usize]);
                        if C.w[1] > 0x09 || (C.w[1] == 0x09 && C.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 20 {
                        // C * 10 >= 0x9fffffffffffffffb <=> C * 2 > 1ffffffffffffffff
                        C.w[0] = C1.w[0] + C1.w[0];
                        C.w[1] = C1.w[1] + C1.w[1];
                        if C.w[0] < C1.w[0] {
                            C.w[1] += 1;
                        }
                        if C.w[1] > 0x01 || (C.w[1] == 0x01 && C.w[0] >= 0xffffffffffffffffu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else if q == 21 {
                        // C >= 0x9fffffffffffffffb
                        if C1.w[1] > 0x09 || (C1.w[1] == 0x09 && C1.w[0] >= 0xfffffffffffffffbu64) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    } else {	// if 22 <= q <= 34 => 1 <= q - 21 <= 13
                        // C  >= 10^(q-21) * 0x9fffffffffffffffb max 44 bits x 68 bits
                        C.w[1] = 0x09;
                        C.w[0] = 0xfffffffffffffffbu64;
                        C = __mul_128x64_to_128(BID_TEN2K64[(q - 21) as usize], &C);
                        if C1.w[1] > C.w[1] || (C1.w[1] == C.w[1] && C1.w[0] >= C.w[0]) {
                            // set invalid flag
                            *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                            // return Integer Indefinite
                            res = 0x8000000000000000u64;
                            return res;
                        }
                        // else cases that can be rounded to a 64-bit int fall through
                        // to '1 <= q + exp <= 20'
                    }
                }
            },
            _ => { }
        }
        // n is not too large to be converted to int64 if -1/2 < n < 2^64 - 1/2
        // Note: some of the cases tested for above fall through to this point
        match q + exp {
            value if value < 0 => {	// n = +/-0.0...c(0)c(1)...c(q-1)
                // set inexact flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                // return 0
                res = 0x0000000000000000u64;
                return res;
            },
            value if value == 0 => {	// n = +/-0.c(0)c(1)...c(q-1)
                // if 0.c(0)c(1)...c(q-1) < 0.5 <=> c(0)c(1)...c(q-1) < 5 * 10^(q-1)
                //   res = 0
                // else if x > 0
                //   res = +1
                // else // if x < 0
                //   invalid exc
                ind = q - 1;
                if ind <= 18 {	// 0 <= ind <= 18
                    if (C1.w[1] == 0) && (C1.w[0] < BID_MIDPOINT64[ind as usize]) {
                        res = 0x0000000000000000u64;	// return 0
                    } else if x_sign == 0 {	// n > 0
                        res = 0x00000001;	// return +1
                    } else {
                        // set invalid flag
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                } else {	// 19 <= ind <= 33
                    if  (C1.w[1]  < BID_MIDPOINT128[(ind - 19) as usize].w[1])
                    || ((C1.w[1] == BID_MIDPOINT128[(ind - 19) as usize].w[1])
                     && (C1.w[0]  < BID_MIDPOINT128[(ind - 19) as usize].w[0])) {
                        res = 0x0000000000000000u64;	// return 0
                    } else if x_sign == 0 {	// n > 0
                        res = 0x00000001;	// return +1
                    } else {
                        *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                        // return Integer Indefinite
                        res = 0x8000000000000000u64;
                        return res;
                    }
                }
                // set inexact flag
                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
            },
            _ => {	// if (1 <= q + exp <= 20, 1 <= q <= 34, -33 <= exp <= 19)
                // x <= -1 or 1 <= x < 2^64-1/2 so if positive x can be rounded
                // to nearest to a 64-bit unsigned signed integer
                if x_sign != 0 {	// x <= -1
                    // set invalid flag
                    *pfpsf |= StatusFlags::BID_INVALID_EXCEPTION;
                    // return Integer Indefinite
                    res = 0x8000000000000000u64;
                    return res;
                }
                // 1 <= x < 2^64-1/2 so x can be rounded
                // to nearest to a 64-bit unsigned integer
                match exp {
                    value if value < 0 => {	// 2 <= q <= 34, -33 <= exp <= -1, 1 <= q + exp <= 20
                        ind = -exp;	// 1 <= ind <= 33; ind is a synonym for 'x'
                        // chop off ind digits from the lower part of C1
                        // C1 = C1 + 1/2 * 10^ind where the result C1 fits in 127 bits
                        tmp64 = C1.w[0];
                        if ind <= 19 {
                            C1.w[0] += BID_MIDPOINT64[(ind - 1) as usize];
                        } else {
                            C1.w[0] += BID_MIDPOINT128[(ind - 20) as usize].w[0];
                            C1.w[1] += BID_MIDPOINT128[(ind - 20) as usize].w[1];
                        }
                        if C1.w[0] < tmp64 {
                            C1.w[1] += 1;
                        }
                        // calculate C* and f*
                        // C* is actually floor(C*) in this case
                        // C* and f* need shifting and masking, as shown by
                        // BID_SHIFTRIGHT128[] and BID_MASKHIGH128[]
                        // 1 <= x <= 33
                        // kx = 10^(-x) = BID_TEN2MK128[(ind - 1) as usize]
                        // C* = (C1 + 1/2 * 10^x) * 10^(-x)
                        // the approximation of 10^(-x) was rounded up to 118 bits
                        P256 = __mul_128x128_to_256(&C1, &BID_TEN2MK128[(ind - 1) as usize]);
                        if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                            Cstar.w[1] = P256.w[3];
                            Cstar.w[0] = P256.w[2];
                            fstar.w[3] = 0;
                            fstar.w[2] = P256.w[2] & BID_MASKHIGH128[(ind - 1) as usize];
                            fstar.w[1] = P256.w[1];
                            fstar.w[0] = P256.w[0];
                        } else {	// 22 <= ind - 1 <= 33
                            Cstar.w[1] = 0;
                            Cstar.w[0] = P256.w[3];
                            fstar.w[3] = P256.w[3] & BID_MASKHIGH128[(ind - 1) as usize];
                            fstar.w[2] = P256.w[2];
                            fstar.w[1] = P256.w[1];
                            fstar.w[0] = P256.w[0];
                        }
                        // the top Ex bits of 10^(-x) are T* = BID_TEN2MK128TRUNC[ind as usize], e.g.
                        // if x=1, T*=BID_TEN2MK128TRUNC[0]=0x19999999999999999999999999999999
                        // if (0 < f* < 10^(-x)) then the result is a midpoint
                        //   if floor(C*) is even then C* = floor(C*) - logical right
                        //       shift; C* has p decimal digits, correct by Prop. 1)
                        //   else if floor(C*) is odd C* = floor(C*)-1 (logical right
                        //       shift; C* has p decimal digits, correct by Pr. 1)
                        // else
                        //   C* = floor(C*) (logical right shift; C has p decimal digits,
                        //       correct by Property 1)
                        // n = C* * 10^(e+x)

                        // shift right C* by Ex-128 = BID_SHIFTRIGHT128[ind as usize]
                        shift = BID_SHIFTRIGHT128[(ind - 1) as usize];	// 0 <= shift <= 102
                        Cstar.w[0] = if ind - 1 <= 21 {	// 0 <= ind - 1 <= 21
                            (Cstar.w[0] >> shift) | (Cstar.w[1] << (64 - shift))
                            // redundant, it will be 0! Cstar.w[1] = (Cstar.w[1] >> shift);
                        } else {	// 22 <= ind - 1 <= 33
                            Cstar.w[0] >> (shift - 64)	// 2 <= shift - 64 <= 38
                        };
                        // determine inexactness of the rounding of C*
                        // if (0 < f* - 1/2 < 10^(-x)) then
                        //   the result is exact
                        // else // if (f* - 1/2 > T*) then
                        //   the result is inexact
                        if ind - 1 <= 2 {
                            if  fstar.w[1]  > 0x8000000000000000u64
                            || (fstar.w[1] == 0x8000000000000000u64
                             && fstar.w[0]  > 0x0u64) {
                                // f* > 1/2 and the result may be exact
                                tmp64 = fstar.w[1] - 0x8000000000000000u64;	// f* - 1/2
                                if  tmp64  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                || (tmp64 == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                 && fstar.w[0] >= BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                }	// else the result is exact
                            } else {	// the result is inexact; f2* <= 1/2
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            }
                        } else if ind - 1 <= 21 {	// if 3 <= ind <= 21
                            if  fstar.w[3]  > 0x0
                            || (fstar.w[3] == 0x0 && fstar.w[2] > BID_ONEHALF128[(ind - 1) as usize])
                            || (fstar.w[3] == 0x0 && fstar.w[2] == BID_ONEHALF128[(ind - 1) as usize]
                            && (fstar.w[1] != 0 || fstar.w[0] != 0)) {
                                // f2* > 1/2 and the result may be exact
                                // Calculate f2* - 1/2
                                tmp64 = fstar.w[2] - BID_ONEHALF128[(ind - 1) as usize];
                                tmp64A = fstar.w[3];
                                if tmp64 > fstar.w[2] {
                                    tmp64A -= 1;
                                }
                                if  tmp64A != 0 || tmp64 != 0
                                 || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                 && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                }	// else the result is exact
                            } else {	// the result is inexact; f2* <= 1/2
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            }
                        } else {	// if 22 <= ind <= 33
                            if  fstar.w[3]  > BID_ONEHALF128[(ind - 1) as usize]
                            || (fstar.w[3] == BID_ONEHALF128[(ind - 1) as usize]
                            && (fstar.w[2] != 0 || fstar.w[1] != 0 || fstar.w[0] != 0)) {
                                // f2* > 1/2 and the result may be exact
                                // Calculate f2* - 1/2
                                tmp64 = fstar.w[3] - BID_ONEHALF128[(ind - 1) as usize];
                                if  tmp64 != 0 || fstar.w[2] != 0
                                 || fstar.w[1]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                || (fstar.w[1] == BID_TEN2MK128TRUNC[(ind - 1) as usize].w[1]
                                 && fstar.w[0]  > BID_TEN2MK128TRUNC[(ind - 1) as usize].w[0]) {
                                    // set the inexact flag
                                    *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                                }	// else the result is exact
                            } else {	// the result is inexact; f2* <= 1/2
                                // set the inexact flag
                                *pfpsf |= StatusFlags::BID_INEXACT_EXCEPTION;
                            }
                        }

                        // if the result was a midpoint it was rounded away from zero
                        res = Cstar.w[0];	// the result is positive
                    },
                    value if value == 0 => {
                        // 1 <= q <= 20, but x < 2^64 - 1/2 so in this case C1.w[1] has to be 0
                        // res = C (exact)
                        res = C1.w[0];
                    },
                    _ => {
                        // if (exp > 0) => 1 <= exp <= 19, 1 <= q < 19, 2 <= q + exp <= 20
                        // res = C * 10^exp (exact) - must fit in 64 bits
                        res = C1.w[0] * BID_TEN2K64[exp as  usize];
                    }
                }
            }
        }
    }

    res
}