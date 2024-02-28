/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */

use std::str::FromStr;
use byteorder::{BigEndian, ByteOrder};
use sqlx::Decode;
use sqlx::encode::Encode;
use sqlx::postgres::PgValueFormat;
use crate::bid_decimal_data::{BID_ESTIMATE_DECIMAL_DIGITS, BID_POWER10_TABLE_128};
use crate::bid_internal::{__unsigned_compare_ge_128, BID_UI64DOUBLE, BID_UINT128, BID_UINT64, MASK_BINARY_EXPONENT, unpack_BID128};
use crate::d128::{d128, INFINITY, NAN, NEGATIVE_INFINITY};

// https://doxygen.postgresql.org/backend_2utils_2adt_2numeric_8c_source.html

const NUMERIC_POS: i32      = 0x0000;
const NUMERIC_NEG: i32      = 0x4000;
const NUMERIC_NAN: i32      = 0xC000;
const NUMERIC_PINF: i32     = 0xD000;
const NUMERIC_NINF: i32     = 0xF000;
const BI_TEN_THOUSAND: u128 = 10000u128;
const BI_MAX_LONG: u128     = 9223372036854775807u128;

const WEIGHTS: [d128; 15] = [
    d128 { w: [0x1, 0x3008000000000000] },  // 1E-28
    d128 { w: [0x1, 0x3010000000000000] },  // 1E-24
    d128 { w: [0x1, 0x3018000000000000] },  // 1E-20
    d128 { w: [0x1, 0x3020000000000000] },  // 1E-16
    d128 { w: [0x1, 0x3028000000000000] },  // 1E-12
    d128 { w: [0x1, 0x3030000000000000] },  // 1E-8
    d128 { w: [0x1, 0x3038000000000000] },  // 1E-4
    d128 { w: [0x1, 0x3040000000000000] },  // 1
    d128 { w: [0x1, 0x3048000000000000] },  // 1E+4
    d128 { w: [0x1, 0x3050000000000000] },  // 1E+8
    d128 { w: [0x1, 0x3058000000000000] },  // 1E+12
    d128 { w: [0x1, 0x3060000000000000] },  // 1E+16
    d128 { w: [0x1, 0x3068000000000000] },  // 1E+20
    d128 { w: [0x1, 0x3070000000000000] },  // 1E+24
    d128 { w: [0x1, 0x3078000000000000] }   // 1E+28
];

const BID_POWER10_TABLE: [u128; 39] = [
    0x00000000000000000000000000000001u128,	// 10^0
    0x0000000000000000000000000000000au128,	// 10^1
    0x00000000000000000000000000000064u128,	// 10^2
    0x000000000000000000000000000003e8u128,	// 10^3
    0x00000000000000000000000000002710u128,	// 10^4
    0x000000000000000000000000000186a0u128,	// 10^5
    0x000000000000000000000000000f4240u128,	// 10^6
    0x00000000000000000000000000989680u128,	// 10^7
    0x00000000000000000000000005f5e100u128,	// 10^8
    0x0000000000000000000000003b9aca00u128,	// 10^9
    0x000000000000000000000002540be400u128,	// 10^10
    0x0000000000000000000000174876e800u128,	// 10^11
    0x0000000000000000000000e8d4a51000u128,	// 10^12
    0x0000000000000000000009184e72a000u128,	// 10^13
    0x000000000000000000005af3107a4000u128,	// 10^14
    0x000000000000000000038d7ea4c68000u128,	// 10^15
    0x0000000000000000002386f26fc10000u128,	// 10^16
    0x0000000000000000016345785d8a0000u128,	// 10^17
    0x00000000000000000de0b6b3a7640000u128,	// 10^18
    0x00000000000000008ac7230489e80000u128,	// 10^19
    0x00000000000000056bc75e2d63100000u128,	// 10^20
    0x000000000000003635c9adc5dea00000u128,	// 10^21
    0x000000000000021e19e0c9bab2400000u128,	// 10^22
    0x000000000000152d02c7e14af6800000u128,	// 10^23
    0x000000000000d3c21bcecceda1000000u128,	// 10^24
    0x0000000000084595161401484a000000u128,	// 10^25
    0x000000000052b7d2dcc80cd2e4000000u128,	// 10^26
    0x00000000033b2e3c9fd0803ce8000000u128,	// 10^27
    0x00000000204fce5e3e25026110000000u128,	// 10^28
    0x00000001431e0fae6d7217caa0000000u128,	// 10^29
    0x0000000c9f2c9cd04674edea40000000u128,	// 10^30
    0x0000007e37be2022c0914b2680000000u128,	// 10^31
    0x000004ee2d6d415b85acef8100000000u128,	// 10^32
    0x0000314dc6448d9338c15b0a00000000u128,	// 10^33
    0x0001ed09bead87c0378d8e6400000000u128,	// 10^34
    0x0013426172c74d822b878fe800000000u128,	// 10^35
    0x00c097ce7bc90715b34b9f1000000000u128,	// 10^36
    0x0785ee10d5da46d900f436a000000000u128,	// 10^37
    0x4b3b4ca85a86c47a098a224000000000u128	// 10^38
];

impl sqlx::Type<sqlx::postgres::Postgres> for d128 {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("numeric")
    }
}

impl sqlx::postgres::PgHasArrayType for d128 {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_numeric")
    }
}

fn __get_dec_digits64(X: &BID_UINT128) -> i32 {
    let mut tempx: BID_UI64DOUBLE = BID_UI64DOUBLE:: default();
    let mut digits_x: i32;
    let bin_expon_cx: usize;

    if X.w[1] == 0 {
        if X.w[0] == 0 {
            return 0;
        }
        unsafe {
            //--- get number of bits in the coefficients of x and y ---
            tempx.d      = X.w[0] as f64;
            bin_expon_cx = (((tempx.ui64 & MASK_BINARY_EXPONENT) >> 52) - 0x3ff) as usize;
        }
        // get number of decimal digits in the coeff_x
        digits_x = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon_cx];
        if X.w[0] >= BID_POWER10_TABLE_128[digits_x as usize].w[0] {
            digits_x += 1;
        }
        return digits_x;
    }
    unsafe {
        tempx.d      = X.w[1] as f64;
        bin_expon_cx = (((tempx.ui64 & MASK_BINARY_EXPONENT) >> 52) - 0x3ff) as usize;
    }
    // get number of decimal digits in the coeff_x
    digits_x = BID_ESTIMATE_DECIMAL_DIGITS[bin_expon_cx + 64];
    if __unsigned_compare_ge_128(&X, &BID_POWER10_TABLE_128[digits_x as usize]) {
        digits_x += 1;
    }
    return digits_x;
}

impl Encode<'_, sqlx::postgres::Postgres> for d128 {
    // adapted from https://github.com/pgjdbc/pgjdbc/blob/master/pgjdbc/src/main/java/org/postgresql/util/ByteConverter.java
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        let mut coeff: BID_UINT128 = BID_UINT128::default();
        let mut sign: BID_UINT64   = 0;
        let mut exponent: i32      = 0;
        let mut weight: i32        = -1;
        let mut digits: Vec<i16>   = vec![0i16; 0];

        unpack_BID128(&mut sign, &mut exponent, &mut coeff, self);

        let mut unscaled: u128 = (coeff.w[1] as u128) << 64 | (coeff.w[0] as u128);
        let mut scale: i32     = self.scale();

        if unscaled == 0 {
            // final byte[] bytes = new byte[]{0, 0, -1, -1, 0, 0, 0, 0};
            // ByteConverter.int2(bytes, 6, Math.max(0, scale));
            // return bytes;
        }

        if scale >= 0 {
            // this means we have an integer
            // adjust unscaled and weight
            if scale > 0 {
                // scale     = scale.abs();
                // weight value covers 4 digits
                weight   += scale / 4;
                // whatever remains needs to be incorporated to the unscaled value
                let mods: usize = (scale % 4) as usize;
                unscaled *= BID_POWER10_TABLE[mods];
                scale     = 0;
            }

            while unscaled > BI_MAX_LONG {
                let quot = unscaled / BI_TEN_THOUSAND;
                let rema = unscaled % BI_TEN_THOUSAND;
                unscaled = quot;
                let short_value = rema as i16;
                if short_value != 0 || !digits.is_empty() {
                    digits.push(short_value);
                }
                weight += 1;
            }

            let mut unscaled_long = unscaled as u64;

            loop {
                let short_value = (unscaled_long % 10000) as i16;
                if short_value != 0 || !digits.is_empty() {
                    digits.push(short_value);
                }
                unscaled_long /= 10000;
                weight        += 1;
                if unscaled_long == 0 {
                    break;
                }
            }

        } else {

            let scale: i32 = scale.abs();
            let mut wholes: u128  = unscaled / BID_POWER10_TABLE[scale as usize];
            let mut decimal: u128 = unscaled % BID_POWER10_TABLE[scale as usize];

            weight = -1;

            if decimal != 0 {
                let mods: i32         = scale % 4;
                let mut segments: i32 = scale / 4;
                if mods != 0 {
                    decimal  *= 10u128.pow((4 - mods) as u32);
                    segments += 1;
                }
                loop {
                    let quot = decimal / BI_TEN_THOUSAND;
                    let rema = decimal % BI_TEN_THOUSAND;
                    decimal = quot;
                    let short_value = rema as i16;
                    if short_value != 0 || !digits.is_empty() {
                        digits.push(short_value);
                    }
                    segments -= 1;
                    if decimal == 0 {
                        break;
                    }
                }

                // for the leading 0 shorts we either adjust weight (if no wholes) or push shorts
                if wholes == 0 {
                    weight -= segments;
                } else {
                    // now add leading 0 shorts
                    digits[0..(segments as usize)].fill(0i16);
                }
            }

            loop {
                weight += 1;
                let quot: u128 = wholes / BI_TEN_THOUSAND;
                let rema: u128 = wholes % BI_TEN_THOUSAND;
                wholes = quot;
                let short_value = rema as i16;
                if short_value != 0 || !digits.is_empty() {
                    digits.push(short_value);
                }
                if wholes == 0 {
                    break;
                }
            }
        }

        // Encode the value

        let size_in_bytes: usize = 8 + (digits.len() * std::mem::size_of::<i16>());
        let mut buffer: Vec<u8>  = vec![0u8; size_in_bytes];
        let mut pos: usize = 0;
        let sign = if self.is_infinity() && !self.is_signed() {
            NUMERIC_PINF
        } else if self.is_infinity() && self.is_signed() {
            NUMERIC_NINF
        } else if self.is_nan() || self.is_signaling() {
            NUMERIC_NAN
        } else if self.is_signed() {
            NUMERIC_NEG
        } else {
            NUMERIC_POS
        } as i16;

        BigEndian::write_i16(&mut buffer[pos..], digits.len() as i16);
        pos += 2;
        BigEndian::write_i16(&mut buffer[pos..], weight as i16);
        pos += 2;
        BigEndian::write_i16(&mut buffer[pos..], sign);
        pos += 2;
        BigEndian::write_i16(&mut buffer[pos..], scale.abs() as i16);
        pos += 2;

        for digit in digits.iter().rev() {
            BigEndian::write_i16(&mut buffer[pos..], *digit);
            pos += 2;
        }

        buf.extend(&buffer);

        sqlx::encode::IsNull::No
    }
}

impl Decode<'_, sqlx::postgres::Postgres> for d128 {
    // adapted from https://github.com/carlosga/pgsqlclient-core/blob/main/src/PostgreSql.Data.SqlClient/src/PostgreSql/Data/Frontend/MessageReader.Numeric.cs
    fn decode(value: <sqlx::postgres::Postgres as sqlx::database::HasValueRef<'_>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let res = match value.format() {
            PgValueFormat::Binary => {
                let buf: &[u8]       = value.as_bytes()?;
                let mut pos: usize   = 0;
                let mut binary: d128 = d128::default();
                let ndigits: i16;    // # of digits in digits[] - can be 0!
                let weight: i16;     // weight of first digit
                let sign: i16;       // NUMERIC_POS, NUMERIC_NEG, or NUMERIC_NAN
                let _dscale: i16;    // display scale
                let mut digit: i16;

                ndigits = BigEndian::read_i16(&buf[pos..]);
                pos    += 2;
                weight  = BigEndian::read_i16(&buf[pos..]) + 7;
                pos    += 2;
                sign    = BigEndian::read_i16(&buf[pos..]);
                pos    += 2;
                _dscale = BigEndian::read_i16(&buf[pos..]);
                pos    += 2;

                // base-NBASE digits
                for i in 0..ndigits {
                    digit   = BigEndian::read_i16(&buf[pos..]);
                    pos    += 2;
                    binary += d128::from(digit as i32) * WEIGHTS[(weight - i) as usize];
                }

                match sign as i32 {
                    NUMERIC_NEG  => -binary,
                    NUMERIC_NAN  => NAN,
                    NUMERIC_PINF => INFINITY,
                    NUMERIC_NINF => NEGATIVE_INFINITY,
                    _            => binary
                }
            },
            PgValueFormat::Text => d128::from_str(value.as_str()?).unwrap()
        };

        Ok(res)
    }
}