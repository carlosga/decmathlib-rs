/* -------------------------------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

#[macro_export]
macro_rules! serde_test {
    ($name:ident, serde_serialize, $input1:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let expected = dec1.to_string();
            let serialized = serde_json::to_string(&dec1).expect("serialization error");

            assert_eq!(format!("\"{}\"", expected), serialized);
        }
    };

    ($name:ident, serde_deserialize, $input1:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            let expected = decmathlib_rs::d128::d128::from($input1);
            let serialized = format!("\"{}\"", expected.to_string());
            let deserialized: decmathlib_rs::d128::d128 = serde_json::from_str(&serialized).expect("deserialization error");

            assert_eq!(expected, deserialized);
        }
    };
}

serde_test!(serde_serialize_001, serde_serialize, "0e6176");
serde_test!(serde_serialize_002, serde_serialize, "12345678901234567890123456789012345");
serde_test!(serde_serialize_003, serde_serialize, "12345678901234567890123456789012345");
serde_test!(serde_serialize_004, serde_serialize, "12345678901234567890123456789012345");
serde_test!(serde_serialize_005, serde_serialize, "12345678901234567890123456789012345");
serde_test!(serde_serialize_006, serde_serialize, "12345678901234567890123456789012345");
serde_test!(serde_serialize_007, serde_serialize, "1.0");
serde_test!(serde_serialize_008, serde_serialize, "-9.9999999999999999999999999999999995");
serde_test!(serde_serialize_009, serde_serialize, "-9.9999999999999999999999999999999995");
serde_test!(serde_serialize_010, serde_serialize, "9.9999999999999999999999999999999995");
serde_test!(serde_serialize_011, serde_serialize, "9.9999999999999999999999999999999995");
serde_test!(serde_serialize_012, serde_serialize, "9.9999999999999999999999999999999995");
serde_test!(serde_serialize_013, serde_serialize, "9.9999999999999999999999999999999995");
serde_test!(serde_serialize_014, serde_serialize, "9.9999999999999999999999999999999995");
serde_test!(serde_serialize_015, serde_serialize, "1.0000000000000000000000000000000015");
serde_test!(serde_serialize_016, serde_serialize, "1.0000000000000000000000000000000015");
serde_test!(serde_serialize_017, serde_serialize, "1.0000000000000000000000000000000015");
serde_test!(serde_serialize_018, serde_serialize, "1.0000000000000000000000000000000015");
serde_test!(serde_serialize_019, serde_serialize, "1.0000000000000000000000000000000015");
serde_test!(serde_serialize_020, serde_serialize, "000.0");

serde_test!(serde_deserialize_001, serde_deserialize, "0e6176");
serde_test!(serde_deserialize_002, serde_deserialize, "12345678901234567890123456789012345");
serde_test!(serde_deserialize_003, serde_deserialize, "12345678901234567890123456789012345");
serde_test!(serde_deserialize_004, serde_deserialize, "12345678901234567890123456789012345");
serde_test!(serde_deserialize_005, serde_deserialize, "12345678901234567890123456789012345");
serde_test!(serde_deserialize_006, serde_deserialize, "12345678901234567890123456789012345");
serde_test!(serde_deserialize_007, serde_deserialize, "1.0");
serde_test!(serde_deserialize_008, serde_deserialize, "-9.9999999999999999999999999999999995");
serde_test!(serde_deserialize_009, serde_deserialize, "-9.9999999999999999999999999999999995");
serde_test!(serde_deserialize_010, serde_deserialize, "9.9999999999999999999999999999999995");
serde_test!(serde_deserialize_011, serde_deserialize, "9.9999999999999999999999999999999995");
serde_test!(serde_deserialize_012, serde_deserialize, "9.9999999999999999999999999999999995");
serde_test!(serde_deserialize_013, serde_deserialize, "9.9999999999999999999999999999999995");
serde_test!(serde_deserialize_014, serde_deserialize, "9.9999999999999999999999999999999995");
serde_test!(serde_deserialize_015, serde_deserialize, "1.0000000000000000000000000000000015");
serde_test!(serde_deserialize_016, serde_deserialize, "1.0000000000000000000000000000000015");
serde_test!(serde_deserialize_017, serde_deserialize, "1.0000000000000000000000000000000015");
serde_test!(serde_deserialize_018, serde_deserialize, "1.0000000000000000000000000000000015");
serde_test!(serde_deserialize_019, serde_deserialize, "1.0000000000000000000000000000000015");
serde_test!(serde_deserialize_020, serde_deserialize, "000.0");

// serde_test!(serde_deserialize_999, serde_deserialize, 0x0001ed09bead87c0378d8e64ffffffffu128);
