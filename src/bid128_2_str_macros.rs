/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(unused_assignments)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::bid128_2_str_tables::*;
use crate::dec128::{BID_UINT32, BID_UINT64};
pub (crate) fn __L0_Normalize_10to18(X_hi: &mut BID_UINT64, X_lo: &mut BID_UINT64) {
    let L0_tmp: BID_UINT64 = *X_lo as BID_UINT64 + bid_Twoto60_m_10to18;
    if (L0_tmp & bid_Twoto60) == bid_Twoto60 {
        *X_hi = *X_hi + 1 ;
        *X_lo = (L0_tmp << 4) >> 4;
    }
}

pub (crate) fn __L0_Normalize_10to9(X_hi: &mut BID_UINT32, X_lo: &mut BID_UINT32) {
    let L0_tmp: BID_UINT32 = *X_lo + bid_Twoto30_m_10to9;
    if (L0_tmp & 0x40000000) == 0x40000000 {
        *X_hi = *X_hi + 1;
        *X_lo = (L0_tmp << 2) >> 2;
    }
}

pub (crate) fn __L0_Split_MiDi_2(X: BID_UINT32, str: &mut String) {
    let mut L0_head: BID_UINT32 = X >> 10;
    let mut L0_tail: BID_UINT32 = (X & (0x03FF)) + (L0_head << 5) - (L0_head << 3);
    let L0_tmp: BID_UINT32      = L0_tail >> 10;
    L0_head                    += L0_tmp;
    L0_tail                     = (L0_tail & (0x03FF)) + (L0_tmp<<5) - (L0_tmp << 3);

    if L0_tail > 999 {
        L0_tail -= 1000;
        L0_head += 1;
    }

    str.push_str(&L0_head.to_string());
    str.push_str(&L0_tail.to_string());
}

pub (crate) fn __L0_Split_MiDi_3(X: BID_UINT32, str: &mut String) {
    let mut L0_X: BID_UINT32    = X as BID_UINT32;
    let mut L0_head: BID_UINT32 = ((L0_X >> 17) * 34359) >> 18;
    L0_X                       -= L0_head * 1000000;

    if L0_X >= 1000000 {
        L0_X    -= 1000000;
        L0_head += 1;
    }

    let mut L0_mid: BID_UINT32  = L0_X >> 10;
    let mut L0_tail: BID_UINT32 = (L0_X & (0x03FF)) + (L0_mid << 5) - (L0_mid << 3);
    let L0_tmp: BID_UINT32      = (L0_tail) >> 10;
    L0_mid                     += L0_tmp;
    L0_tail                     = (L0_tail & (0x3FF)) + (L0_tmp << 5) - (L0_tmp << 3);

    if L0_tail > 999 {
        L0_tail -= 1000;
        L0_mid  += 1;
    }

    // TODO: Fix
    // str.push_str(&L0_head.to_string());
    // str.push_str(&L0_mid.to_string()));
    // str.push_str(&L0_tail.to_string()));

    str.push_str(&L0_head.to_string());
    str.push_str(&format!("{:0>3}", L0_mid.to_string()));
    str.push_str(&format!("{:0>3}", L0_tail.to_string()));
}

pub (crate) fn __L1_Split_MiDi_6(X: BID_UINT64, str: &mut String) {
    let mut  L1_Xhi_64: BID_UINT64 = ((X >> 28) * (bid_Inv_Tento9 as BID_UINT64)) >> 33;
    let mut  L1_Xlo_64: BID_UINT64 = X as BID_UINT64 - L1_Xhi_64 * (bid_Tento9 as BID_UINT64);

    if L1_Xlo_64 >= (bid_Tento9 as BID_UINT64) {
        L1_Xlo_64 -= bid_Tento9 as BID_UINT64;
        L1_Xhi_64 += 1;
    }

    let L1_X_hi: BID_UINT32 = L1_Xhi_64 as BID_UINT32;
    let L1_X_lo: BID_UINT32 = L1_Xlo_64 as BID_UINT32;

    __L0_Split_MiDi_3(L1_X_hi, str);
    __L0_Split_MiDi_3(L1_X_lo, str);
}

pub (crate) fn __L1_Split_MiDi_6_Lead(X: BID_UINT64, str: &mut String) {
    let mut L1_X_hi: BID_UINT32;
    let mut L1_X_lo: BID_UINT32;
    let mut L1_Xhi_64: BID_UINT64;
    let mut L1_Xlo_64: BID_UINT64;

    if X >= (bid_Tento9 as BID_UINT64) {
        L1_Xhi_64 = ((X >> 28) * (bid_Inv_Tento9 as BID_UINT64)) >> 33;
        L1_Xlo_64 = X - L1_Xhi_64 * bid_Tento9 as BID_UINT64;

        if L1_Xlo_64 >= (bid_Tento9 as BID_UINT64) {
            L1_Xlo_64 -= bid_Tento9 as BID_UINT64;
            L1_Xhi_64 += 1;
        }

        L1_X_hi = L1_Xhi_64 as BID_UINT32;
        L1_X_lo = L1_Xlo_64 as BID_UINT32;

        if L1_X_hi >= bid_Tento6 {
            __L0_Split_MiDi_3(L1_X_hi, str);
            __L0_Split_MiDi_3(L1_X_lo, str);
        } else if L1_X_hi >= bid_Tento3 {
            __L0_Split_MiDi_2(L1_X_hi, str);
            __L0_Split_MiDi_3(L1_X_lo, str);
        } else {
            str.push_str(&L1_X_hi.to_string());
            __L0_Split_MiDi_3(L1_X_lo, str);
        }
    } else {
        L1_X_lo = X as BID_UINT32;
        if L1_X_lo >= bid_Tento6 {
            __L0_Split_MiDi_3(L1_X_lo, str);
        } else if L1_X_lo >= bid_Tento3 {
            __L0_Split_MiDi_2(L1_X_lo, str);
        } else {
            str.push_str(&L1_X_lo.to_string());
        }
    }
}

pub (crate) fn __L0_MiDi2Str(X: BID_UINT32, str: &mut String) {
    let mut  i = X as usize;
    str.push_str(bid_midi_tbl[i]);
    i += 1;
    str.push_str(bid_midi_tbl[i]);
    i += 1;
    str.push_str(bid_midi_tbl[i]);
}

pub (crate) fn __L0_MiDi2Str_Lead(X: BID_UINT32, str: &mut String) {
    let mut  i = X as usize;
    if X >= 100 {
        str.push_str(bid_midi_tbl[i]);
        i += 1;
        str.push_str(bid_midi_tbl[i]);
        i += 1;
        str.push_str(bid_midi_tbl[i]);
        i += 1;
        str.push_str(bid_midi_tbl[i]);
    } else if X >= 10 {
        i += 1;
        str.push_str(bid_midi_tbl[i]);
        i += 1;
        str.push_str(bid_midi_tbl[i]);
    } else {
        i += 1;
        i += 1;
        str.push_str(bid_midi_tbl[i]);
    }
}
