/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

//////////////////////////////////////////////
//    BID128_to_string
//////////////////////////////////////////////

// #define BID_128RES
// #include "bid128_2_str.h"
// #include "bid128_2_str_macros.h"

use std::ops::BitAnd;
use log::info;
use crate::bid128::{bid_char_table2, bid_char_table3};
use crate::bid128_2_str_macros::*;
use crate::bid128_2_str_tables::mod10_18_tbl;
use crate::constants::{MASK_COEFF, MASK_EXP, MASK_NAN, MASK_SIGN, MASK_SNAN, MASK_SPECIAL};
use crate::dec128::{_IDEC_flags, BID_SINT64, BID_UINT128, BID_UINT32, BID_UINT64};

const MAX_FORMAT_DIGITS_128: u32 = 34u32;
const MAX_STRING_DIGITS_128: u32 = 100u32;
const MAX_SEARCH: u32            = MAX_STRING_DIGITS_128 - MAX_FORMAT_DIGITS_128 - 1;

pub (crate) fn bid128_to_string(x: &BID_UINT128, pfpsf: &mut _IDEC_flags) -> String {
    let x_sign: BID_UINT64;
    let mut x_exp: BID_UINT64;
    let mut exp: i32;   // unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let ind: i32;
    let mut  C1: BID_UINT128 = BID_UINT128::default();
    let k: u32 = 0; // pointer in the string
    let d0: u32;
    let d123: u32;
    let mut HI_18Dig: BID_UINT64;
    let mut LO_18Dig: BID_UINT64;
    let mut Tmp: BID_UINT64;
    let mut MiDi: String = String::new();
    let mut midi_ind: i32;
    let mut k_lcv: i32;
    let len: i32;
    let save_fpsf: _IDEC_flags;
    let mut str = String::new();

    save_fpsf = *pfpsf; // dummy

    #[cfg(target_endian = "big")]
    let mut x = *x;

    #[cfg(target_endian = "big")]
    BID_SWAP128(&mut x);

    // check for NaN or Infinity
    if (x.w[1] & MASK_SPECIAL) == MASK_SPECIAL {
        // x is special
        return if (x.w[1] & MASK_NAN) == MASK_NAN {   // x is NAN
            if (x.w[1] & MASK_SNAN) == MASK_SNAN {    // x is SNAN
                // set invalid flag
                if (x.w[1] as BID_SINT64) < 0 { String::from("-SNaN") } else { String::from("+SNaN") }
            } else {
                // x is QNaN
                if (x.w[1] as BID_SINT64) < 0 { String::from("-NaN") } else { String::from("+NaN") }
            }
        } else { // x is not a NaN, so it must be infinity
            if (x.w[1] & MASK_SIGN) == 0x0u64 { String::from("+Inf") } else { String::from("-Inf") }
        };
    } else if ((x.w[1] & MASK_COEFF) == 0x0u64) && (x.w[0] == 0x0u64) {
        // x is 0
        len = 0;

        //determine if +/-
        str.push(if (x.w[1] & MASK_SIGN) == MASK_SIGN { '-' } else { '+' });
        str.push('0');
        str.push('E');

        // extract the exponent and print
        exp = (((x.w[1] & MASK_EXP) >> 49) - 6176) as i32;

        if exp > (((0x5ffe) >> 1) - (6176)) {
            exp = ((((x.w[1] << 2) & MASK_EXP) >> 49) as i32) - 6176;
        }
        if exp >= 0 {
            str.push('+');
        }
        str.push_str(&exp.to_string());

        return str;
    } else { // x is not special and is not zero
        // unpack x
        x_sign = x.w[1] & MASK_SIGN;// 0 for positive, MASK_SIGN for negative
        x_exp  = x.w[1] & MASK_EXP;// biased and shifted left 49 bit positions

        if (x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64 {
            x_exp = (x.w[1] << 2) & MASK_EXP;// biased and shifted left 49 bit positions
        }

        C1.w[1] = x.w[1] & MASK_COEFF;
        C1.w[0] = x.w[0];
        exp     = ((x_exp >> 49).wrapping_sub(6176)) as i32;

        // determine sign's representation as a char
        str.push(if x_sign != 0 { '-' /* negative number */ } else { '+' /* positive number */ });

        // determine coefficient's representation as a decimal string

        // if zero or non-canonical, set coefficient to '0'
        if (C1.w[1] > 0x0001ed09bead87c0u64)
        || (C1.w[1] == 0x0001ed09bead87c0u64 && (C1.w[0] > 0x378d8e63ffffffffu64))
        || ((x.w[1] & 0x6000000000000000u64) == 0x6000000000000000u64) || ((C1.w[1] == 0) && (C1.w[0] == 0)) {
            str.push('0');
        } else {
            /* ****************************************************
              This takes a bid coefficient in C1.w[1],C1.w[0]
              and put the converted character sequence at location
              starting at &(str[k]). The function returns the number
              of MiDi returned. Note that the character sequence
              does not have leading zeros EXCEPT when the input is of
              zero value. It will then output 1 character '0'
              The algorithm essentailly tries first to get a sequence of
              Millenial Digits "MiDi" and then uses table lookup to get the
              character strings of these MiDis.
              **************************************************** */
            /* Algorithm first decompose possibly 34 digits in hi and lo
              18 digits. (The high can have at most 16 digits). It then
              uses macro that handle 18 digit portions.
              The first step is to get hi and lo such that
              2^(64) C1.w[1] + C1.w[0] = hi * 10^18  + lo,   0 <= lo < 10^18.
              We use a table lookup method to obtain the hi and lo 18 digits.
              [C1.w[1],C1.w[0]] = c_8 2^(107) + c_7 2^(101) + ... + c_0 2^(59) + d
              where 0 <= d < 2^59 and each c_j has 6 bits. Because d fits in
              18 digits,  we set hi = 0, and lo = d to begin with.
              We then retrieve from a table, for j = 0, 1, ..., 8
              that gives us A and B where c_j 2^(59+6j) = A * 10^18 + B.
              hi += A ; lo += B; After each accumulation into lo, we normalize
              immediately. So at the end, we have the decomposition as we need. */

            Tmp      = C1.w[0] >> 59;
            LO_18Dig = (C1.w[0] << 5) >> 5;
            Tmp     += (C1.w[1] << 5);
            HI_18Dig = 0;
            k_lcv    = 0;

            // Tmp = {C1.w[1]{49:0}, C1.w[0]{63:59}}
            // Lo_18Dig = {C1.w[0]{58:0}}
            while Tmp > 0 {
                midi_ind   = Tmp as i32 & 0x000000000000003fi32;
                midi_ind <<= 1;
                Tmp      >>= 6;
                HI_18Dig  += mod10_18_tbl[k_lcv as usize][midi_ind as usize];
                midi_ind  += 1;
                LO_18Dig  += mod10_18_tbl[k_lcv as usize][midi_ind as usize];
                k_lcv     += 1;
                __L0_Normalize_10to18(&mut HI_18Dig, &mut LO_18Dig);
            }

            if (HI_18Dig == 0u64) {
                __L1_Split_MiDi_6_Lead(LO_18Dig, &mut str);
            } else {
                __L1_Split_MiDi_6_Lead(HI_18Dig, &mut str);
                __L1_Split_MiDi_6(LO_18Dig, &mut str);
            }

            // TODO: Port
            // len = ptr - MiDi;
            // c_ptr_start = &(str[k]);
            // c_ptr = c_ptr_start;
            //
            // /* now convert the MiDi into character strings */
            // __L0_MiDi2Str_Lead (MiDi[0], c_ptr);
            // for (k_lcv = 1; k_lcv < len; k_lcv++) {
            //     __L0_MiDi2Str (MiDi[k_lcv], c_ptr);
            // }
            // k = k + (c_ptr - c_ptr_start);
        }

        // print E and sign of exponent
        str.push('E');
        if exp < 0 {
            exp = -exp;
            str.push('-');
        } else {
            str.push('+');
        }

        // determine exponent's representation as a decimal string
        // d0 = exp / 1000;
        // Use Property 1
        d0   = ((exp * 0x418a) >> 24) as u32;  // 0x418a * 2^-24 = (10^(-3))RP,15
        d123 = (exp - 1000 * (d0 as i32)) as u32;

        if d0 != 0 {
            // 1000 <= exp <= 6144 => 4 digits to return
            str.push(char::from_digit(d0, 10).unwrap());     // ASCII for decimal digit d0
            ind      = (3 * d123) as i32;
            str.push(bid_char_table3[ind as usize]);
            str.push(bid_char_table3[(ind + 1) as usize]);
            str.push(bid_char_table3[(ind + 2) as usize]);
        } else {
            // 0 <= exp <= 999 => d0 = 0
            if d123 < 10 {
                // 0 <= exp <= 9 => 1 digit to return
                str.push(char::from_digit(d123, 10).unwrap()); // ASCII
            } else if (d123 < 100) {
                // 10 <= exp <= 99 => 2 digits to return
                ind = (2 * (d123 - 10)) as i32;
                str.push(bid_char_table2[ind as usize]);
                str.push(bid_char_table2[(ind + 1) as usize]);
            } else {
                // 100 <= exp <= 999 => 3 digits to return
                ind = (3 * d123) as i32;
                str.push(bid_char_table3[ind as usize]);
                str.push(bid_char_table3[(ind + 1) as usize]);
                str.push(bid_char_table3[(ind + 2) as usize]);
            }
        }
    }

    str
}

/*
pub (crate) fn bid128_from_string(str: &str, rnd_mode: u32, pfpsf: &mut _IDEC_Flags) -> BID_UINT128 {
    let CX: BID_UINT128  = BID_UINT128::default();
    let res: BID_UINT128 = BID_UINT128::default();
    let sign_x: BID_UINT64;
    let coeff_high: BID_UINT64;
    let coeff_low: BID_UINT64;
    let coeff2: BID_UINT64;
    let coeff_l2: BID_UINT64;
    let carry: BID_UINT64 = 0x0u64,
    let scale_high: BID_UINT64;
    let right_radix_leading_zeros: BID_UINT64;
    let ndigits_before: i32;
    let ndigits_after: i32;
    let ndigits_total: i32;
    let dec_expon: i32;
    let sgn_exp: i32;
    let i: i32;
    let d2: i32;
    let rdx_pt_enc: i32;
    let set_inexact: i32 = 0;
    let c: string;
    let buffer[char; MAX_STRING_DIGITS_128];
    let save_rnd_mode: u32;
    let save_fpsf: u32;
    let c: char;
    let ps: usize = 0;

// #if DECIMAL_CALL_BY_REFERENCE
// #if !DECIMAL_GLOBAL_ROUNDING
//   _IDEC_round rnd_mode = *prnd_mode;
// #endif
// #endif

    save_rnd_mode             = rnd_mode; // dummy
    save_fpsf                 = *pfpsf; // dummy
    right_radix_leading_zeros = 0;
    rdx_pt_enc                = 0;

    // if nu64 string, return NaN
    if ps.is_empty() {
        res.w[1] = 0x7c00000000000000u64;
        res.w[0] = 0;
        return res;
    }

    // eliminate leading white space
    while (str[ps] == ' ') || (str[ps] == '\t') {
        ps += 1;
    }

    // c gets first character
    c = str[ps];

    // if c is nu64 or not equal to a (radix point, negative sign,
    // positive sign, or number) it might be SNaN, sNaN, Infinity
    if (!c || (c != '.' && c != '-' && c != '+' && (((c - '0') as u32) > 9))) {
        res.w[0] = 0;
        // Infinity?
        if ((tolower_macro(ps[0]) == 'i' && tolower_macro (ps[1]) == 'n' && tolower_macro(ps[2]) == 'f')
            && (!ps[3]
                || (tolower_macro (ps[3]) == 'i'
                 && tolower_macro (ps[4]) == 'n'
                 && tolower_macro (ps[5]) == 'i'
                 && tolower_macro (ps[6]) == 't'
                 && tolower_macro (ps[7]) == 'y' && !ps[8])
            )) {
          res.w[1] = 0x7800000000000000u64;
          return res;
        }
        // return sNaN
        res.w[1] = if  tolower_macro(ps[0]) == 's' && tolower_macro(ps[1]) == 'n'
                    && tolower_macro(ps[2]) == 'a' && tolower_macro(ps[3]) == 'n' {
            // case insensitive check for snan
            0x7e00000000000000u64
        } else {
            // return qNaN
            0x7c00000000000000u64
        };

        return res;
    }
    // if +Inf, -Inf, +Infinity, or -Infinity (case insensitive check for inf)
    if ((tolower_macro (ps[1]) == 'i'
      && tolower_macro (ps[2]) == 'n'
      && tolower_macro (ps[3]) == 'f') && (!ps[4] ||
        (tolower_macro (ps[4]) == 'i'
      && tolower_macro (ps[5]) == 'n'
      && tolower_macro (ps[6]) == 'i'
      && tolower_macro (ps[7]) == 't' &&
         tolower_macro (ps[8]) == 'y' && !ps[9]))) { // ci check for infinity
        res.w[0] = 0;
        res.w[1] = if c == '+' {
            0x7800000000000000u64
        } else if (c == '-') {
            0xf800000000000000u64
        } else {
            0x7c00000000000000u64
        };

        return res;
    }
    // if +sNaN, +SNaN, -sNaN, or -SNaN
    if  tolower_macro (ps[1]) == 's' && tolower_macro (ps[2]) == 'n'
     && tolower_macro (ps[3]) == 'a' && tolower_macro (ps[4]) == 'n' {
        res.w[0] = 0;
        res.w[1] = if (c == '-') {
            0xfe00000000000000u64
        } else {
            0x7e00000000000000u64
        };
        return res;;
    }
    // set up sign_x to be OR'ed with the upper word later
    sign_x = if c == '-' {
        0x8000000000000000u64;
    } else {
        0
    };

    // go to next character if leading sign
    if (c == '-' || c == '+') {
        ps += 1;
    }

    c = str[ps];

    // if c isn't a decimal point or a decimal digit, return NaN
    if c != '.' && (((c - '0') as u32) > 9) {
        res.w[1] = 0x7c00000000000000u64 | sign_x;
        res.w[0] = 0;
        return res;
    }

    if c == '.' {
        rdx_pt_enc = 1;
        ps        += 1;
    }

    // detect zero (and eliminate/ignore leading zeros)
    if str[ps] == '0' {
        // if all numbers are zeros (with possibly 1 radix point, the number is zero
        // should catch cases such as: 000.0
        while str[ps] == '0' {
            ps += 1;

            // for numbers such as 0.0000000000000000000000000000000000001001,
            // we want to count the leading zeros
            if rdx_pt_enc {
              right_radix_leading_zeros += 1;
            }

            // if this character is a radix point, make sure we haven't already
            // encountered one
            if str[ps] == '.' {
                if rdx_pt_enc == 0 {
                    rdx_pt_enc = 1;
                    // if this is the first radix point, and the next character is Nu64,
                    // we have a zero
                    if (ps + 1) > str.len() {
                      res.w[1] = (0x3040000000000000u64 - (right_radix_leading_zeros << 49)) | sign_x;
                      res.w[0] = 0;
                      return res;
                    }
                    ps = ps + 1;
                } else {
                    // if 2 radix points, return NaN
                    res.w[1] = 0x7c00000000000000u64 | sign_x;
                    res.w[0] = 0;
                    return res;
                }
            } else if ps > str.len() {
                if right_radix_leading_zeros > 6176 {
                    right_radix_leading_zeros = 6176;
                }
                res.w[1] = (0x3040000000000000u64 - (right_radix_leading_zeros << 49)) | sign_x;
                res.w[0] = 0;
                return res;
            }
        }
    }

    c = str[ps];

    // initialize local variables
    ndigits_before = 0;
    ndigits_after  = 0;
    ndigits_total  = 0;
    sgn_exp        = 0;
    // pstart_coefficient = ps;

    if !rdx_pt_enc {
        // investigate string (before radix point)
        while (((c - '0') as u2) <= 9 /*&& ndigits_before < MAX_STRING_DIGITS_128*/) {
            if (ndigits_before < MAX_FORMAT_DIGITS_128) {
                buffer[ndigits_before] = c;
            } else if (ndigits_before < MAX_STRING_DIGITS_128) {
                buffer[ndigits_before] = c;
                if c > '0' {
                    set_inexact = 1;
                }
            } else if c > '0' {
                set_inexact = 1;
            }
            ps             += 1;
            c               = str[ps];
            ndigits_before += 1;
        }

        ndigits_total = ndigits_before;
        if c == '.' {
            ps += 1;
            if ((c = *ps)) {
                // investigate string (after radix point)
                while (c - '0') as u32 <= 9 /*&& ndigits_total < MAX_STRING_DIGITS_128*/ {
                    if ndigits_total < MAX_FORMAT_DIGITS_128 {
                        buffer[ndigits_total] = c;
                    } else if (ndigits_total < MAX_STRING_DIGITS_128 {
                        buffer[ndigits_total] = c;
                        if c > '0' {
                            set_inexact = 1;
                        }
                    } else if c > '0' {
                        set_inexact=1;
                    }
                    ps            += 1;
                    c              = str[ps];
                    ndigits_total += 1;
                }
                ndigits_after = ndigits_total - ndigits_before;
            }
        }
    } else {
        // we encountered a radix point while detecting zeros
        //if (c = *ps){
        c             = str[ps];
        ndigits_total = 0;

        // investigate string (after radix point)
        while ((c - '0') as u32) <= 9 /*&& ndigits_total < MAX_STRING_DIGITS_128*/ {
            if ndigits_total < MAX_FORMAT_DIGITS_128 {
                buffer[ndigits_total] = c;
            }
            else if (ndigits_total < MAX_STRING_DIGITS_128)  {
                buffer[ndigits_total] = c;
                if c > '0' {
                    set_inexact = 1;
                }
            } else if c > '0' {
                set_inexact = 1;
            }
            ps+= 1;
            c = *ps;
            ndigits_total+= 1;
        }
        ndigits_after = ndigits_total - ndigits_before;
    }

    // get exponent
    dec_expon = 0;
    /*if (ndigits_total < MAX_STRING_DIGITS_128)*/
    {
        if (c) {
            if c != 'e' && c != 'E' {
                // return NaN
                res.w[1] = 0x7c00000000000000u64;
                res.w[0] = 0;
                return res;
            }
            ps += 1;
            c   = str[ps];

            if ((((c - '0') as u32) > 9) && ((c != '+' && c != '-') || ((ps[1] - '0') as u32) > 9)) {
                // return NaN
                res.w[1] = 0x7c00000000000000u64;
                res.w[0] = 0;
                return res;
            }

            if c == '-' {
                sgn_exp = -1;
                ps     += 1;
                c       = str[ps];
            } else if c == '+' {
                ps += 1;
                c   = str[ps];
            }

            dec_expon = c - '0';
            i         = 1;
            ps       += 1;

            if !dec_expon {
                while str[ps] == '0' {
                    ps += 1;
                }
            }
            c = str[ps] - '0'; // TODO

            while ((c as unsigned)) <= 9 && i < 7 {
                d2        = dec_expon + dec_expon;
                dec_expon = (d2 << 2) + d2 + c;
                ps       += 1;
                c         = str[ps] - '0';  // TODO
                i        += 1;
            }
        }

        dec_expon = (dec_expon + sgn_exp) ^ sgn_exp;
    }

    if ndigits_total <= MAX_FORMAT_DIGITS_128 {
        dec_expon += DECIMAL_EXPONENT_BIAS_128 - ndigits_after - right_radix_leading_zeros;
        if dec_expon < 0 {
            res.w[1] = 0 | sign_x;
            res.w[0] = 0;
        }
        if ndigits_total == 0 {
            CX.w[0] = 0;
            CX.w[1] = 0;
        } else if ndigits_total <= 19 {
            coeff_high = buffer[0] - '0';
            for (i = 1; i < ndigits_total; i++) {
                coeff2     = coeff_high + coeff_high;
                coeff_high = (coeff2 << 2) + coeff2 + buffer[i] - '0';
            }
            CX.w[0] = coeff_high;
            CX.w[1] = 0;
        } else {
            coeff_high = buffer[0] - '0';
            for (i = 1; i < ndigits_total - 17; i++) {
                coeff2 = coeff_high + coeff_high;
                coeff_high = (coeff2 << 2) + coeff2 + buffer[i] - '0';
            }
            coeff_low = buffer[i] - '0';
            i+= 1;
            for (; i < ndigits_total; i++) {
                coeff_l2  = coeff_low + coeff_low;
                coeff_low = (coeff_l2 << 2) + coeff_l2 + buffer[i] - '0';
            }
            // now form the coefficient as coeff_high*10^19+coeff_low+carry
            scale_high = 100000000000000000u64;
            __mul_64x64_to_128_fast (CX, coeff_high, scale_high);

            CX.w[0] += coeff_low;
            if (CX.w[0] < coeff_low) {
                CX.w[1] += 1;
            }
        }
        bid_get_BID128(&res, sign_x, dec_expon, CX, &rnd_mode, pfpsf);
        return res;
    } else {
        // simply round using the digits that were read

        dec_expon += ndigits_before + DECIMAL_EXPONENT_BIAS_128 - MAX_FORMAT_DIGITS_128 - right_radix_leading_zeros;

        if (dec_expon < 0) {
            res.w[1] = 0 | sign_x;
            res.w[0] = 0;
        }

        coeff_high = buffer[0] - '0';

        i = 1;
        while i < MAX_FORMAT_DIGITS_128 - 17 {
            coeff2     = coeff_high + coeff_high;
            coeff_high = (coeff2 << 2) + coeff2 + buffer[i] - '0';
            i         += 1;
        }
        coeff_low = buffer[i] - '0';

        i += 1;
        while i < MAX_FORMAT_DIGITS_128 {
            coeff_l2  = coeff_low + coeff_low;
            coeff_low = (coeff_l2 << 2) + coeff_l2 + buffer[i] - '0';
            i        += 1;
        }

        match rnd_mode {
            BID_ROUNDING_TO_NEAREST => {
                carry = (('4' - buffer[i]) as u32) >> 31;
                if (buffer[i] == '5' && !(coeff_low & 1)) || dec_expon < 0 {
                    if (dec_expon >= 0) {
                        carry = 0;
                        i    += 1;
                    }
                    while i < ndigits_total {
                        if buffer[i] > '0' {
                            carry = 1;
                            break;
                        }
                        i += 1;
                    }
                }
            },
            BID_ROUNDING_DOWN => {
                if sign_x {
                    while i < ndigits_total {
                        if buffer[i] > '0' {
                            carry = 1;
                            break;
                        }
                        i += 1;
                    }
                }
            },
            BID_ROUNDING_UP => {
                if !sign_x {
                    while i < ndigits_total {
                        if buffer[i] > '0' {
                            carry = 1;
                            break;
                        }
                        i += 1;
                    }
                }
            },
            BID_ROUNDING_TO_ZERO => {
                carry = 0;
            },
            BID_ROUNDING_TIES_AWAY => {
                carry = (('4' - buffer[i]) as u32) >> 31;
                if dec_expon < 0 {
                    while i < ndigits_total {
                        if buffer[i] > '0' {
                            carry = 1;
                            break;
                        }
                        i += 1;
                    }
                }
            },
        }
    }

    // now form the coefficient as coeff_high*10^17+coeff_low+carry
    scale_high = 100000000000000000u64;
    if dec_expon < 0 {
        if dec_expon > -MAX_FORMAT_DIGITS_128 {
            scale_high = 1000000000000000000u64;
            coeff_low  = (coeff_low << 3) + (coeff_low << 1);
            dec_expon -= 1;
        }
        if dec_expon == -MAX_FORMAT_DIGITS_128 && coeff_high > 50000000000000000u64 {
            carry = 0;
        }
    }

    CX = __mul_64x64_to_128_fast(coeff_high, scale_high);

    coeff_low += carry;
    CX.w[0]   += coeff_low;

    if CX.w[0] < coeff_low {
        CX.w[1] += 1;
    }

    if set_inexact {
        __set_status_flags(pfpsf, BID_INEXACT_EXCEPTION);
    }

    res = bid_get_BID128(sign_x, dec_expon, &CX, &rnd_mode, pfpsf);

    return res;
}
*/