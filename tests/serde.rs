/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */

#[cfg(feature = "serde")]
#[test]
fn serde_serialize_001() {
    let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
    let expected = "+1234567890123456789012345678901234E+1";
    let dec = decmathlib_rs::d128::d128::from_string(expected, None, &mut status);
    let serialized = serde_json::to_string(&dec).expect("serialization error");

    assert_eq!(format!("\"{}\"", expected), serialized);
}

#[cfg(feature = "serde")]
#[test]
fn serde_deserialize_001() {
    let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
    let serialized = "+1234567890123456789012345678901234E+1";
    let expected = decmathlib_rs::d128::d128::from_string(serialized, None, &mut status);
    let deserialized: decmathlib_rs::d128::d128 = serde_json::from_str(&format!("\"{}\"", serialized)).expect("deserialization error");

    assert_eq!(expected, deserialized);
}