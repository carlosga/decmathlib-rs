/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(non_snake_case)]

use crate::bid_internal::*;
use crate::d128::{BID_SINT64, BID_UI64DOUBLE, BID_UINT128, BID_UINT256, BID_UINT64};

pub (crate) fn bid___div_128_by_128(CX0: &BID_UINT128, CY: &BID_UINT128) -> (BID_UINT128, BID_UINT128){
    let mut CY36: BID_UINT128 = BID_UINT128::default();
    let mut CY51: BID_UINT128 = BID_UINT128::default();
    let mut CQ: BID_UINT128 = BID_UINT128::default();
    let mut A2: BID_UINT128;
    let mut CX: BID_UINT128 = BID_UINT128::default();
    let mut CQT: BID_UINT128 = BID_UINT128::default();
    let mut Q: BID_UINT64;
    let mut t64: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let mut d49: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let mut d60: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let mut lx: f64;
    let ly: f64;
    let mut lq: f64;
    let mut pCQ: BID_UINT128 = BID_UINT128::default();
    let mut pCR: BID_UINT128 = BID_UINT128::default();

    if CX0.w[1] == 0 && CY.w[1] == 0 {
        pCQ.w[0] = CX0.w[0] / CY.w[0];
        pCQ.w[1] = 0;
        pCR.w[1] = 0;
        pCR.w[0] = 0;
        pCR.w[0] = CX0.w[0] - pCQ.w[0] * CY.w[0];
        return (pCQ, pCR);
    }

    CX.w[1] = CX0.w[1];
    CX.w[0] = CX0.w[0];

    // 2^64
    unsafe {
        t64.i = 0x43f0000000000000u64;
        lx = (CX.w[1] as f64) * t64.d + (CX.w[0] as f64);
        ly = (CY.w[1] as f64) * t64.d + (CY.w[0] as f64);
        lq = lx / ly;
    }

    CY36.w[1] = CY.w[0] >> (64 - 36);
    CY36.w[0] = CY.w[0] << 36;

    CQ.w[1] = 0;
    CQ.w[0] = 0;

    // Q >= 2^100 ?
    if CY.w[1] == 0 && CY36.w[1] == 0 && (CX.w[1] >= CY36.w[0]) {
        // then Q >= 2^100

        unsafe {
            // 2^(-60)*CX/CY
            d60.i = 0x3c30000000000000u64;
            lq   *= d60.d;
            Q     = (lq as BID_UINT64) - 4u64;

            // Q*CY
            A2 = __mul_64x64_to_128(Q, CY.w[0]);

            // A2 <<= 60
            A2.w[1]   = (A2.w[1] << 60) | (A2.w[0] >> (64 - 60));
            A2.w[0] <<= 60;

            CX = __sub_128_128(&CX, &A2);

            lx = (CX.w[1] as f64) * t64.d + (CX.w[0] as f64);
            lq = lx / ly;

            CQ.w[1] = Q >> (64 - 60);
            CQ.w[0] = Q << 60;
        }
    }

    CY51.w[1] = (CY.w[1] << 51) | (CY.w[0] >> (64 - 51));
    CY51.w[0] = CY.w[0] << 51;

    if CY.w[1] < ((1 << (64 - 51)) as BID_UINT64) && (__unsigned_compare_gt_128(&CX, &CY51)) {
        unsafe {
            // Q > 2^51

            // 2^(-49)*CX/CY
            d49.i = 0x3ce0000000000000u64;
            lq   *= d49.d;

            Q = (lq as BID_UINT64) - 1u64;

            // Q*CY
            A2       = __mul_64x64_to_128(Q, CY.w[0]);
            A2.w[1] += Q * CY.w[1];

            // A2 <<= 49
            A2.w[1] = (A2.w[1] << 49) | (A2.w[0] >> (64 - 49));
            A2.w[0] <<= 49;

            CX = __sub_128_128(&CX, &A2);

            CQT.w[1] = Q >> (64 - 49);
            CQT.w[0] = Q << 49;
            CQ = __add_128_128(&CQ, &CQT);

            lx = (CX.w[1] as f64) * t64.d + (CX.w[0] as f64);
            lq = lx / ly;
        }
    }

    Q = lq as BID_UINT64;

    A2       = __mul_64x64_to_128(Q, CY.w[0]);
    A2.w[1] += Q * CY.w[1];

    CX = __sub_128_128(&CX, &A2);
    if (CX.w[1] as BID_SINT64) < 0 {
        Q       -= 1;
        CX.w[0] += CY.w[0];
        if CX.w[0] < CY.w[0] {
            CX.w[1] += 1;
        }
        CX.w[1] += CY.w[1];
        if (CX.w[1] as BID_SINT64) < 0 {
            Q       -= 1;
            CX.w[0] += CY.w[0];
            if CX.w[0] < CY.w[0] {
                CX.w[1] += 1;
            }
            CX.w[1] += CY.w[1];
        }
    } else if __unsigned_compare_ge_128(&CX, &CY) {
        Q += 1;
        CX = __sub_128_128(&CX, &CY);
    }

    CQ = __add_128_64(&CQ, Q);

    pCQ.w[1] = CQ.w[1];
    pCQ.w[0] = CQ.w[0];
    pCR.w[1] = CX.w[1];
    pCR.w[0] = CX.w[0];

    return (pCQ, pCR);
}

pub (crate) fn bid___div_256_by_128(pCQ: &mut BID_UINT128, pCA4: &mut BID_UINT256, CY: &BID_UINT128) {
    let mut CA4: BID_UINT256 = BID_UINT256::default();
    let mut CA2: BID_UINT256 = BID_UINT256::default();
    let mut CY51: BID_UINT256 = BID_UINT256::default();
    let mut CY36: BID_UINT256 = BID_UINT256::default();
    let mut CQ: BID_UINT128 = BID_UINT128::default();
    let mut A2: BID_UINT128;
    let mut A2h: BID_UINT128;
    let mut CQT: BID_UINT128 = BID_UINT128::default();
    let mut Q: BID_UINT64;
    let mut carry64: BID_UINT64;
    let mut t64: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let mut d49: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let mut d60: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let mut lx: f64;
    let ly: f64;
    let mut lq: f64;
    let d128: f64;
    let d192: f64;

    // the quotient is assumed to be at most 113 bits,
    // as needed by BID128 divide routines

    // initial dividend
    CA4.w[3] = pCA4.w[3];
    CA4.w[2] = pCA4.w[2];
    CA4.w[1] = pCA4.w[1];
    CA4.w[0] = pCA4.w[0];
    CQ.w[1]  = pCQ.w[1];
    CQ.w[0]  = pCQ.w[0];

    unsafe {
        // 2^64
        t64.i = 0x43f0000000000000u64;
        d128  = t64.d * t64.d;
        d192  = d128 * t64.d;
        lx    = (CA4.w[3] as f64) * d192 + ((CA4.w[2] as f64) * d128 + ((CA4.w[1] as f64) * t64.d + (CA4.w[0] as f64)));
        ly    = (CY.w[1] as f64) * t64.d + (CY.w[0] as f64);
        lq    = lx / ly;
    }

    CY36.w[2] = CY.w[1] >> (64 - 36);
    CY36.w[1] = (CY.w[1] << 36) | (CY.w[0] >> (64 - 36));
    CY36.w[0] = CY.w[0] << 36;

    // CQ.w[1] = (*pCQ).w[1];
    // CQ.w[0] = (*pCQ).w[0];

    // Q >= 2^100 ?
    if CA4.w[3] > CY36.w[2] || (CA4.w[3] == CY36.w[2] && (CA4.w[2] > CY36.w[1] || (CA4.w[2] == CY36.w[1] && CA4.w[1] >= CY36.w[0]))) {
        unsafe  {
            // 2^(-60)*CA4/CY
            d60.i = 0x3c30000000000000u64;
            lq   *= d60.d;
            Q     = (lq as BID_UINT64) - 4u64;

            // Q*CY
            CA2 = __mul_64x128_to_256(Q, &CY);

            // __mul_64x128_to_192(A: BID_UINT64, B: &BID_UINT128) -> BID_UINT192 {

            // CA2 <<= 60
            // CA2.w[3] = CA2.w[2] >> (64-60);
            CA2.w[2]   = (CA2.w[2] << 60) | (CA2.w[1] >> (64 - 60));
            CA2.w[1]   = (CA2.w[1] << 60) | (CA2.w[0] >> (64 - 60));
            CA2.w[0] <<= 60;

            // CA4 -= CA2
            (CA4.w[0], carry64) = __sub_borrow_out(CA4.w[0], CA2.w[0]);
            (CA4.w[1], carry64) = __sub_borrow_in_out(CA4.w[1], CA2.w[1], carry64);
            CA4.w[2]            = CA4.w[2] - CA2.w[2] - carry64;

            lx = (CA4.w[2] as f64) * d128 + ((CA4.w[1] as f64) * t64.d + (CA4.w[0] as f64));
            lq = lx / ly;

            CQT.w[1] = Q >> (64 - 60);
            CQT.w[0] = Q << 60;
            CQ = __add_128_128(&CQ, &CQT);
        }
    }

    CY51.w[2] = CY.w[1] >> (64 - 51);
    CY51.w[1] = (CY.w[1] << 51) | (CY.w[0] >> (64 - 51));
    CY51.w[0] = CY.w[0] << 51;

    if CA4.w[2] > CY51.w[2] || ((CA4.w[2] == CY51.w[2]) && (__unsigned_compare_gt_256_as_128(&CA4, &CY51))) {
        unsafe {
            // Q > 2^51

            // 2^(-49)*CA4/CY
            d49.i = 0x3ce0000000000000u64;
            lq   *= d49.d;

            Q = (lq as BID_UINT64) - 1u64;

            // Q*CY
            A2  = __mul_64x64_to_128(Q, CY.w[0]);
            A2h = __mul_64x64_to_128(Q, CY.w[1]);
            A2.w[1] += A2h.w[0];
            if A2.w[1] < A2h.w[0] {
                A2h.w[1] += 1;
            }

            // A2 <<= 49
            CA2.w[2] = (A2h.w[1] << 49) | (A2.w[1] >> (64 - 49));
            CA2.w[1] = (A2.w[1]  << 49) | (A2.w[0] >> (64 - 49));
            CA2.w[0] =  A2.w[0]  << 49;

            (CA4.w[0], carry64) = __sub_borrow_out(CA4.w[0], CA2.w[0]);
            (CA4.w[1], carry64) = __sub_borrow_in_out(CA4.w[1], CA2.w[1], carry64);
            CA4.w[2]            = CA4.w[2] - CA2.w[2] - carry64;

            CQT.w[1] = Q >> (64 - 49);
            CQT.w[0] = Q << 49;
            CQ       = __add_128_128(&CQ, &CQT);

            lx = (CA4.w[2] as f64) * d128 + ((CA4.w[1] as f64) * t64.d + (CA4.w[0] as f64));
            lq = lx / ly;
        }
    }

    Q = lq as BID_UINT64;
    A2 = __mul_64x64_to_128(Q, CY.w[0]);
    A2.w[1] += Q * CY.w[1];

    CA4 = __sub_256_128(&CA4, &A2);
    if (CA4.w[1] as BID_SINT64) < 0 {
        Q -= 1;
        CA4.w[0] += CY.w[0];
        if CA4.w[0] < CY.w[0] {
            CA4.w[1] += 1;
        }
        CA4.w[1] += CY.w[1];
        if (CA4.w[1] as BID_SINT64) < 0 {
            Q -= 1;
            CA4.w[0] += CY.w[0];
            if CA4.w[0] < CY.w[0] {
                CA4.w[1] += 1;
            }
            CA4.w[1] += CY.w[1];
        }
    } else if __unsigned_compare_ge_256_128(&CA4, &CY) {
        Q   += 1;
        CA4  = __sub_256_128(&CA4, &CY);
    }

    CQ = __add_128_64(&CQ, Q);

    pCQ.w[1]  = CQ.w[1];
    pCQ.w[0]  = CQ.w[0];
    pCA4.w[1] = CA4.w[1];
    pCA4.w[0] = CA4.w[0];
}