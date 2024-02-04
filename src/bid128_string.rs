/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_assignments)]

//////////////////////////////////////////////
//    BID128_to_string
//////////////////////////////////////////////


use crate::bid128::{bid_char_table2, bid_char_table3};
use crate::bid128_2_str_macros::*;
use crate::bid128_2_str_tables::mod10_18_tbl;
use crate::bid_internal::{__mul_64x64_to_128_fast, __set_status_flags, bid_get_BID128};
use crate::constants::{DECIMAL_EXPONENT_BIAS_128, MASK_COEFF, MASK_EXP, MASK_NAN, MASK_SIGN, MASK_SNAN, MASK_SPECIAL};
use crate::core::{RoundingMode, StatusFlags};
use crate::d128::{_IDEC_flags, BID_SINT64, BID_UINT128, BID_UINT32, BID_UINT64};

const MAX_FORMAT_DIGITS_128: usize = 34;
const MAX_STRING_DIGITS_128: usize = 100;
const MAX_SEARCH: usize            = MAX_STRING_DIGITS_128 - MAX_FORMAT_DIGITS_128 - 1;

/// Convert 128-bit decimal floating-point value (binary encoding) to string format
pub (crate) fn bid128_to_string(x: &BID_UINT128) -> String {
    let x_sign: BID_UINT64;
    let mut x_exp: BID_UINT64;
    let mut exp: i32;   // unbiased exponent
    // Note: C1.w[1], C1.w[0] represent x_signif_hi, x_signif_lo (all are BID_UINT64)
    let ind: i32;
    let mut  C1: BID_UINT128 = BID_UINT128::default();
    let d0: u32;
    let d123: u32;
    let mut HI_18Dig: BID_UINT64;
    let mut LO_18Dig: BID_UINT64;
    let mut Tmp: BID_UINT64;
    let mut MiDi: Vec<BID_UINT32> = vec!();
    let mut midi_ind: i32;
    let mut k_lcv: usize;
    let mut str = String::new();

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
        //determine if +/-
        str.push_str(if (x.w[1] & MASK_SIGN) == MASK_SIGN { "-0E" } else { "+0E" });

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
        exp     = ((x_exp >> 49) - 6176) as i32;

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
            Tmp     += C1.w[1] << 5;
            HI_18Dig = 0;
            k_lcv    = 0;

            // Tmp = {C1.w[1]{49:0}, C1.w[0]{63:59}}
            // Lo_18Dig = {C1.w[0]{58:0}}
            while Tmp > 0 {
                midi_ind   = Tmp as i32 & 0x000000000000003fi32;
                midi_ind <<= 1;
                Tmp      >>= 6;
                HI_18Dig  += mod10_18_tbl[k_lcv][midi_ind as usize];
                midi_ind  += 1;
                LO_18Dig  += mod10_18_tbl[k_lcv][midi_ind as usize];
                k_lcv     += 1;
                __L0_Normalize_10to18(&mut HI_18Dig, &mut LO_18Dig);
            }

            if HI_18Dig == 0u64 {
                __L1_Split_MiDi_6_Lead(LO_18Dig, &mut MiDi);
            } else {
                __L1_Split_MiDi_6_Lead(HI_18Dig, &mut MiDi);
                __L1_Split_MiDi_6(LO_18Dig, &mut MiDi);
            }

            /* now convert the MiDi into character strings */
            __L0_MiDi2Str_Lead(MiDi[0], &mut str);
            for midi in MiDi[1..].iter() {
                __L0_MiDi2Str(*midi, &mut str);
            }
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
            ind = (3 * d123) as i32;
            str.push(bid_char_table3[ind as usize]);
            str.push(bid_char_table3[(ind + 1) as usize]);
            str.push(bid_char_table3[(ind + 2) as usize]);
        } else {
            // 0 <= exp <= 999 => d0 = 0
            if d123 < 10 {
                // 0 <= exp <= 9 => 1 digit to return
                str.push(char::from_digit(d123, 10).unwrap()); // ASCII
            } else if d123 < 100 {
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

/// Convert a decimal floating-point value represented in string format
/// (decimal character sequence) to 128-bit decimal floating-point format (binary encoding)
pub (crate) fn bid128_from_string(str: &str, rnd_mode: u32, pfpsf: &mut _IDEC_flags) -> BID_UINT128 {
    let mut CX: BID_UINT128  = BID_UINT128::default();
    let mut res: BID_UINT128 = BID_UINT128::default();
    let sign_x: BID_UINT64;
    let mut coeff_high: BID_UINT64;
    let mut coeff_low: BID_UINT64;
    let mut coeff2: BID_UINT64;
    let mut coeff_l2: BID_UINT64;
    let mut carry: BID_UINT64 = 0x0u64;
    let mut scale_high: BID_UINT64;
    let mut right_radix_leading_zeros: BID_UINT64;
    let mut ndigits_before: usize;
    let mut ndigits_after: usize;
    let mut ndigits_total: usize;
    let mut dec_expon: i32;
    let mut sgn_exp: i32;
    let mut i: usize;
    let mut d2: i32;
    let mut rdx_pt_enc: i32;
    let mut set_inexact: bool = false;
    let mut c: Option<char>;
    let mut buffer: [char; MAX_STRING_DIGITS_128] = [' '; MAX_STRING_DIGITS_128];
    let mut ps: usize = 0;

    right_radix_leading_zeros = 0;
    rdx_pt_enc                = 0;

    // if nu64 string, return NaN
    if str.is_empty() {
        res.w[1] = 0x7c00000000000000u64;
        res.w[0] = 0;
        return res;
    }

    // eliminate leading white space
    while (str.chars().nth(ps) == Some(' ')) || (str.chars().nth(ps) == Some('\t')) {
        ps += 1;
    }

    // c gets first character
    c = str.chars().nth(ps);

    // if c is nu64 or not equal to a (radix point, negative sign, positive sign, or number) it might be SNaN, sNaN, Infinity
    if c.is_none() || (c != Some('.') && c != Some('-') && c != Some('+') && ((c.unwrap() as i32 - '0' as i32) > 9)) {
        let range = &str[ps..];

        res.w[0] = 0;
        res.w[1] = if range.eq_ignore_ascii_case("inf") || range.eq_ignore_ascii_case("infinity") {
            // Infinity
            0x7800000000000000u64
        } else if range.len() >= 4 && range[0..4].eq_ignore_ascii_case("snan") { // return sNaN
            // case-insensitive check for snan
            0x7e00000000000000u64
        } else { // return qNaN
            0x7c00000000000000u64
        };
        return res;
    }

    let range = &str[ps + 1..];

    // if +Inf, -Inf, +Infinity, or -Infinity (case-insensitive check for inf)
    if range.eq_ignore_ascii_case("inf") || range.eq_ignore_ascii_case("infinity") {
        res.w[0] = 0;
        res.w[1] = if c == Some('+') {
            0x7800000000000000u64
        } else if c == Some('-') {
            0xf800000000000000u64
        } else {
            0x7c00000000000000u64
        };
        return res;
    }
    // if +sNaN, +SNaN, -sNaN, or -SNaN
    if range.len() >= 4 && range[0..4].eq_ignore_ascii_case("snan") {
        res.w[0] = 0;
        res.w[1] = if c == Some('-') {
            0xfe00000000000000u64
        } else {
            0x7e00000000000000u64
        };
        return res;
    }

    // set up sign_x to be OR'ed with the upper word later
    sign_x = if c == Some('-') { 0x8000000000000000u64 } else { 0 };

    // go to next character if leading sign
    if c == Some('-') || c == Some('+') {
        ps += 1;
    }

    c = str.chars().nth(ps);

    // if c isn't a decimal point or a decimal digit, return NaN
    if c.is_some() && c != Some('.') && ((c.unwrap() as i32 - '0' as i32) > 9) {
        res.w[1] = 0x7c00000000000000u64 | sign_x;
        res.w[0] = 0;
        return res;
    }

    if c == Some('.') {
        rdx_pt_enc = 1;
        ps        += 1;
    }

    // detect zero (and eliminate/ignore leading zeros)
    if str.chars().nth(ps) == Some('0') {
        // if all numbers are zeros (with possibly 1 radix point, the number is zero
        // should catch cases such as: 000.0
        while str.chars().nth(ps) == Some('0') {
            ps += 1;

            // for numbers such as 0.0000000000000000000000000000000000001001,
            // we want to count the leading zeros
            if rdx_pt_enc != 0 {
              right_radix_leading_zeros += 1;
            }

            // if this character is a radix point, make sure we haven't already
            // encountered one
            if str.chars().nth(ps) == Some('.') {
                if rdx_pt_enc == 0 {
                    rdx_pt_enc = 1;
                    // if this is the first radix point, and the next character is NULL,
                    // we have a zero
                    if str.chars().nth(ps + 1).is_none() {
                      res.w[1] = (0x3040000000000000u64 - (right_radix_leading_zeros << 49)) | sign_x;
                      res.w[0] = 0;
                      return res;
                    }
                    ps += 1;
                } else {
                    // if 2 radix points, return NaN
                    res.w[1] = 0x7c00000000000000u64 | sign_x;
                    res.w[0] = 0;
                    return res;
                }
            } else if str.chars().nth(ps).is_none() {
                if right_radix_leading_zeros > 6176 {
                    right_radix_leading_zeros = 6176;
                }
                res.w[1] = (0x3040000000000000u64 - (right_radix_leading_zeros << 49)) | sign_x;
                res.w[0] = 0;
                return res;
            }
        }
    }

    c = str.chars().nth(ps);

    // initialize local variables
    ndigits_before = 0;
    ndigits_after  = 0;
    ndigits_total  = 0;
    sgn_exp        = 0;
    // pstart_coefficient = ps;

    if rdx_pt_enc == 0 {
        // investigate string (before radix point)
        while c.is_some() && char::is_digit(c.unwrap(), 10) {
            if ndigits_before < MAX_FORMAT_DIGITS_128 {
                buffer[ndigits_before] = c.unwrap();
            } else if ndigits_before < MAX_STRING_DIGITS_128 {
                buffer[ndigits_before] = c.unwrap();
                if c > Some('0') {
                    set_inexact = true;
                }
            } else if c > Some('0') {
                set_inexact = true;
            }
            ps             += 1;
            c               = str.chars().nth(ps);
            ndigits_before += 1;
        }

        ndigits_total = ndigits_before;
        if c == Some('.') {
            ps += 1;
            c   = str.chars().nth(ps);
            if c.is_some() {
                // investigate string (after radix point)
                while c.is_some() && (char::is_digit(c.unwrap(), 10)) /*&& ndigits_total < MAX_STRING_DIGITS_128*/ {
                    if ndigits_total < MAX_FORMAT_DIGITS_128 {
                        buffer[ndigits_total] = c.unwrap();
                    } else if ndigits_total < MAX_STRING_DIGITS_128 {
                        buffer[ndigits_total] = c.unwrap();
                        if c.unwrap() as i32 > '0' as i32 {
                            set_inexact = true;
                        }
                    } else if c.unwrap() as i32 > '0' as i32 {
                        set_inexact = true;
                    }
                    ps            += 1;
                    c              = str.chars().nth(ps);
                    ndigits_total += 1;
                }
                ndigits_after = ndigits_total - ndigits_before;
            }
        }
    } else {
        // we encountered a radix point while detecting zeros
        //if (c = *ps){
        c             = str.chars().nth(ps);
        ndigits_total = 0;

        // investigate string (after radix point)
        while c.is_some() && char::is_digit(c.unwrap(), 10) /*&& ndigits_total < MAX_STRING_DIGITS_128*/ {
            if ndigits_total < MAX_FORMAT_DIGITS_128 {
                buffer[ndigits_total] = c.unwrap();
            }
            else if ndigits_total < MAX_STRING_DIGITS_128  {
                buffer[ndigits_total] = c.unwrap();
                if c.unwrap() as i32 > '0' as i32 {
                    set_inexact = true;
                }
            } else if c.unwrap() as i32 > '0' as i32 {
                set_inexact = true;
            }
            ps            += 1;
            c              = str.chars().nth(ps);
            ndigits_total += 1;
        }
        ndigits_after = ndigits_total - ndigits_before;
    }

    // get exponent
    dec_expon = 0;
    /*if (ndigits_total < MAX_STRING_DIGITS_128)*/
    {
        if c.is_some() {
            if c != Some('e') && c != Some('E') {
                // return NaN
                res.w[1] = 0x7c00000000000000u64;
                res.w[0] = 0;
                return res;
            }
            ps += 1;
            c   = str.chars().nth(ps);

            if c.is_some() && !char::is_digit(c.unwrap(), 10)
                         && ((c != Some('+') && c != Some('-'))
                         || !char::is_digit(str.chars().nth(ps + 1).unwrap(), 10)) {
                // return NaN
                res.w[1] = 0x7c00000000000000u64;
                res.w[0] = 0;
                return res;
            }

            if c == Some('-') {
                sgn_exp = -1;
                ps     += 1;
                c       = str.chars().nth(ps);
            } else if c == Some('+') {
                ps += 1;
                c   = str.chars().nth(ps);
            }

            dec_expon = (c.unwrap() as i32) - ('0' as i32);
            i         = 1;
            ps       += 1;

            if dec_expon == 0 {
                while str.chars().nth(ps) == Some('0') {
                    ps += 1;
                }
            }

            if str.chars().nth(ps).is_some() {
                c = char::from_digit((str.chars().nth(ps).unwrap() as u32) - ('0' as u32), 10);

                while c.is_some() && (char::to_digit(c.unwrap(), 10).unwrap() <= 9 && i < 7) {
                    d2        = dec_expon + dec_expon;
                    dec_expon = (d2 << 2) + d2 + (char::to_digit(c.unwrap(), 10).unwrap() as i32);
                    ps       += 1;

                    c = str.chars().nth(ps);

                    if c.is_some() {
                        c  = char::from_digit((c.unwrap() as u32) - ('0' as u32), 10);
                        i += 1;
                        continue;
                    } else {
                        i += 1;
                    }
                }
            }
        }

        dec_expon = (dec_expon + sgn_exp) ^ sgn_exp;
    }

    if ndigits_total <= MAX_FORMAT_DIGITS_128 {
        dec_expon += DECIMAL_EXPONENT_BIAS_128 - ndigits_after as i32 - right_radix_leading_zeros as i32;
        if dec_expon < 0 {
            res.w[1] = 0 | sign_x;
            res.w[0] = 0;
        }
        if ndigits_total == 0 {
            CX.w[0] = 0;
            CX.w[1] = 0;
        } else if ndigits_total <= 19 {
            coeff_high = ((buffer[0] as i32) - ('0' as i32)) as BID_UINT64;
            i          = 1;
            while i < ndigits_total {
                coeff2     = coeff_high + coeff_high;
                coeff_high = (coeff2 << 2) + coeff2 + (buffer[i] as BID_UINT64) - ('0' as BID_UINT64);
                i         += 1;
            }
            CX.w[0] = coeff_high;
            CX.w[1] = 0;
        } else {
            coeff_high = ((buffer[0] as i32) - ('0' as i32)) as BID_UINT64;
            i          = 1;
            while i < (ndigits_total - 17) {
                coeff2     = coeff_high + coeff_high;
                coeff_high = (coeff2 << 2) + coeff2 + (buffer[i] as BID_UINT64) - ('0' as BID_UINT64);
                i         += 1;
            }
            coeff_low = ((buffer[i] as i32) - ('0' as i32)) as BID_UINT64;
            i        += 1;
            while i < ndigits_total {
                coeff_l2  = coeff_low + coeff_low;
                coeff_low = (coeff_l2 << 2) + coeff_l2 + (buffer[i] as BID_UINT64) - ('0' as BID_UINT64);
                i        += 1;
            }
            // now form the coefficient as coeff_high*10^19+coeff_low+carry
            scale_high = 100000000000000000u64;
            CX         = __mul_64x64_to_128_fast(coeff_high, scale_high);

            CX.w[0] += coeff_low;
            if CX.w[0] < coeff_low {
                CX.w[1] += 1;
            }
        }
        res = bid_get_BID128(sign_x, dec_expon, &CX, rnd_mode, pfpsf);
        return res;
    } else {
        // simply round using the digits that were read

        dec_expon += ndigits_before as i32
                   + DECIMAL_EXPONENT_BIAS_128
                   - MAX_FORMAT_DIGITS_128 as i32
                   - right_radix_leading_zeros as i32;

        if dec_expon < 0 {
            res.w[1] = 0 | sign_x;
            res.w[0] = 0;
        }

        coeff_high = ((buffer[0] as i32) - ('0' as i32)) as BID_UINT64;

        i = 1;
        while i < MAX_FORMAT_DIGITS_128 - 17 {
            coeff2     = coeff_high + coeff_high;
            coeff_high = (coeff2 << 2) + coeff2 + (buffer[i] as BID_UINT64) - ('0' as BID_UINT64);
            i         += 1;
        }
        coeff_low = (buffer[i] as i32 - '0' as i32) as BID_UINT64;

        i += 1;
        while i < MAX_FORMAT_DIGITS_128 {
            coeff_l2  = coeff_low + coeff_low;
            coeff_low = (coeff_l2 << 2) + coeff_l2 + (buffer[i] as BID_UINT64) - ('0' as BID_UINT64);
            i        += 1;
        }

        match rnd_mode {
            RoundingMode::BID_ROUNDING_TO_NEAREST => {
                carry = ((('4' as i32 - buffer[i] as i32) as u32) >> 31) as BID_UINT64;
                if (buffer[i] == '5' && (coeff_low & 1) != 1) || dec_expon < 0 {
                    if dec_expon >= 0 {
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
            RoundingMode::BID_ROUNDING_DOWN => {
                if sign_x != 0 {
                    while i < ndigits_total {
                        if buffer[i] > '0' {
                            carry = 1;
                            break;
                        }
                        i += 1;
                    }
                }
            },
            RoundingMode::BID_ROUNDING_UP => {
                if sign_x == 0 {
                    while i < ndigits_total {
                        if buffer[i] > '0' {
                            carry = 1;
                            break;
                        }
                        i += 1;
                    }
                }
            },
            RoundingMode::BID_ROUNDING_TO_ZERO => {
                carry = 0;
            },
            RoundingMode::BID_ROUNDING_TIES_AWAY => {
                let digit = char::to_digit(buffer[i], 10).unwrap() as i32;
                carry = (((4 - digit) as u32) >> 31) as BID_UINT64;
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
            _ => panic!("bid128_from_string::Unknown rounding mode")
        }
    }

    // now form the coefficient as coeff_high*10^17+coeff_low+carry
    scale_high = 100000000000000000u64;
    if dec_expon < 0 {
        if dec_expon > -(MAX_FORMAT_DIGITS_128 as i32) {
            scale_high = 1000000000000000000u64;
            coeff_low  = (coeff_low << 3) + (coeff_low << 1);
            dec_expon -= 1;
        }
        if dec_expon == -(MAX_FORMAT_DIGITS_128 as i32) && coeff_high > 50000000000000000u64 {
            carry = 0;
        }
    }

    CX         = __mul_64x64_to_128_fast(coeff_high, scale_high);
    coeff_low += carry;
    CX.w[0]   += coeff_low;

    if CX.w[0] < coeff_low {
        CX.w[1] += 1;
    }

    if set_inexact {
        __set_status_flags(pfpsf, StatusFlags::BID_INEXACT_EXCEPTION);
    }

    res = bid_get_BID128(sign_x, dec_expon, &CX, rnd_mode, pfpsf);

    res
}