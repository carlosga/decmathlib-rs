use std::str::FromStr;
use byteorder::{BigEndian, ByteOrder};
use sqlx::Decode;
use sqlx::encode::Encode;
use sqlx::postgres::PgValueFormat;
use crate::d128::d128;

const POSITIVE_MASK: i16      = 0x0000;
const NEGATIVE_MASK: i16      = 0x4000;
const NAN_MASK: i32           = 0xC000;
const DECIMAL_SCALE_MASK: i32 = 0x3FFF;
const NBASE: i32              = 10000;
const MAX_PRECISION: i32      = 1000;
const MAX_SCALE: i32          = 28;

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
    d128 { w: [0x1, 0x3078000000000000] }   // 1E+2
];

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::postgres::Postgres> for d128 {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("numeric")
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for d128 {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_numeric")
    }
}

#[cfg(feature = "sqlx")]
impl Encode<'_, sqlx::postgres::Postgres> for d128 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        todo!("sqlx encode_by_ref");
        sqlx::encode::IsNull::No
    }
}

#[cfg(feature = "sqlx")]
impl Decode<'_, sqlx::postgres::Postgres> for d128 {
    fn decode(value: <sqlx::postgres::Postgres as sqlx::database::HasValueRef<'_>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let res = match value.format() {
            PgValueFormat::Binary => {
                let buf: &[u8]       = value.as_bytes()?;
                let mut pos: usize   = 0;
                let mut binary: d128 = d128::default();
                let mut ndigits: i16 = 0; // # of digits in digits[] - can be 0!
                let mut weight: i16  = 0; // weight of first digit
                let mut sign: i16    = 0; // NUMERIC_POS, NUMERIC_NEG, or NUMERIC_NAN
                let mut dscale: i16  = 0; // display scale
                let mut digit: i16   = 0;

                ndigits = BigEndian::read_i16(&buf[pos..]);
                pos    += 2;
                weight  = BigEndian::read_i16(&buf[pos..]) + 7;
                pos    += 2;
                sign    = BigEndian::read_i16(&buf[pos..]);
                pos    += 2;
                dscale  = BigEndian::read_i16(&buf[pos..]);
                pos    += 2;

                // base-NBASE digits
                for i in 0..ndigits {
                    digit   = BigEndian::read_i16(&buf[pos..]);
                    pos    += 2;
                    binary += d128::from(digit as i32) * WEIGHTS[(weight - i) as usize];
                }

                match sign {
                    NEGATIVE_MASK => -binary,
                    _             => binary
                }
            },
            PgValueFormat::Text => d128::from_str(value.as_str()?).unwrap()
        };

        Ok(res)
    }
}
