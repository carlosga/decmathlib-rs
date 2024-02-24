/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use std::fmt::Formatter;
use crate::bid128_2_str_tables::*;
use crate::bid_internal::{BID_UINT32, BID_UINT64};

pub (crate) fn __l0_normalize_10to18(x_hi: &mut BID_UINT64, x_lo: &mut BID_UINT64) {
    let l0_tmp: BID_UINT64 = *x_lo as BID_UINT64 + BID_TWOTO60_M_10TO18;
    if (l0_tmp & BID_TWOTO60) == BID_TWOTO60 {
        *x_hi += 1 ;
        *x_lo  = (l0_tmp << 4) >> 4;
    }
}

pub (crate) fn __l0_normalize_10to9(x_hi: &mut BID_UINT32, x_lo: &mut BID_UINT32) {
    let l0_tmp: BID_UINT32 = *x_lo + BID_TWOTO30_M_10TO9;
    if (l0_tmp & 0x40000000) == 0x40000000 {
        *x_hi += 1;
        *x_lo  = (l0_tmp << 2) >> 2;
    }
}

pub (crate) fn __l0_split_midi_2(x: BID_UINT32, vec: &mut Vec<BID_UINT32>) {
    let mut l0_head: BID_UINT32 = x >> 10;
    let mut l0_tail: BID_UINT32 = (x & (0x03FF)) + (l0_head << 5) - (l0_head << 3);
    let l0_tmp: BID_UINT32      = l0_tail >> 10;
    l0_head                    += l0_tmp;
    l0_tail                     = (l0_tail & (0x03FF)) + (l0_tmp <<5) - (l0_tmp << 3);

    if l0_tail > 999 {
        l0_tail -= 1000;
        l0_head += 1;
    }

    vec.push(l0_head);
    vec.push(l0_tail);
}

pub (crate) fn __l0_split_midi_3(x: BID_UINT32, vec: &mut Vec<BID_UINT32>) {
    let mut l0_x: BID_UINT32    = x as BID_UINT32;
    let mut l0_head: BID_UINT32 = ((l0_x >> 17) * 34359) >> 18;
    l0_x                       -= l0_head * 1000000;

    if l0_x >= 1000000 {
        l0_x    -= 1000000;
        l0_head += 1;
    }

    let mut l0_mid: BID_UINT32  = l0_x >> 10;
    let mut l0_tail: BID_UINT32 = (l0_x & (0x03FF)) + (l0_mid << 5) - (l0_mid << 3);
    let l0_tmp: BID_UINT32      = (l0_tail) >> 10;
    l0_mid                     += l0_tmp;
    l0_tail                     = (l0_tail & (0x3FF)) + (l0_tmp << 5) - (l0_tmp << 3);

    if l0_tail > 999 {
        l0_tail -= 1000;
        l0_mid  += 1;
    }

    vec.push(l0_head);
    vec.push(l0_mid);
    vec.push(l0_tail);
}

pub (crate) fn __l1_split_midi_6(x: BID_UINT64, vec: &mut Vec<BID_UINT32>) {
    let mut l1_xhi_64: BID_UINT64 = ((x >> 28) * (BID_INV_TENTO9 as BID_UINT64)) >> 33;
    let mut l1_xlo_64: BID_UINT64 = x as BID_UINT64 - l1_xhi_64 * (BID_TENTO9 as BID_UINT64);

    if l1_xlo_64 >= (BID_TENTO9 as BID_UINT64) {
        l1_xlo_64 -= BID_TENTO9 as BID_UINT64;
        l1_xhi_64 += 1;
    }

    let l1_x_hi: BID_UINT32 = l1_xhi_64 as BID_UINT32;
    let l1_x_lo: BID_UINT32 = l1_xlo_64 as BID_UINT32;

    __l0_split_midi_3(l1_x_hi, vec);
    __l0_split_midi_3(l1_x_lo, vec);
}

pub (crate) fn __l1_split_midi_6_lead(x: BID_UINT64, vec: &mut Vec<BID_UINT32>) {
    let l1_x_hi: BID_UINT32;
    let l1_x_lo: BID_UINT32;
    let mut l1_xhi_64: BID_UINT64;
    let mut l1_xlo_64: BID_UINT64;

    if x >= (BID_TENTO9 as BID_UINT64) {
        l1_xhi_64 = ((x >> 28) * (BID_INV_TENTO9 as BID_UINT64)) >> 33;
        l1_xlo_64 = x - l1_xhi_64 * BID_TENTO9 as BID_UINT64;

        if l1_xlo_64 >= (BID_TENTO9 as BID_UINT64) {
            l1_xlo_64 -= BID_TENTO9 as BID_UINT64;
            l1_xhi_64 += 1;
        }

        l1_x_hi = l1_xhi_64 as BID_UINT32;
        l1_x_lo = l1_xlo_64 as BID_UINT32;

        if l1_x_hi >= BID_TENTO6 {
            __l0_split_midi_3(l1_x_hi, vec);
            __l0_split_midi_3(l1_x_lo, vec);
        } else if l1_x_hi >= BID_TENTO3 {
            __l0_split_midi_2(l1_x_hi, vec);
            __l0_split_midi_3(l1_x_lo, vec);
        } else {
            vec.push(l1_x_hi);
            __l0_split_midi_3(l1_x_lo, vec);
        }
    } else {
        l1_x_lo = x as BID_UINT32;
        if l1_x_lo >= BID_TENTO6 {
            __l0_split_midi_3(l1_x_lo, vec);
        } else if l1_x_lo >= BID_TENTO3 {
            __l0_split_midi_2(l1_x_lo, vec);
        } else {
            vec.push(l1_x_lo);
        }
    }
}

pub (crate) fn __l0_midi_2_str(x: BID_UINT32, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    fmt.write_str(BID_MIDI_TBL[x as usize])
}

pub (crate) fn __l0_midi_2_str_lead(x: BID_UINT32, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    if x >= 100 {
        fmt.write_str(BID_MIDI_TBL[x as usize])
    } else if x >= 10 {
        fmt.write_str(&BID_MIDI_TBL[x as usize][1..])
    } else {
        fmt.write_str(&BID_MIDI_TBL[x as usize][2..])
    }
}
