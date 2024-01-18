/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

#[macro_export]
macro_rules! dec_test {
    ($name:ident, bid128_class, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec   = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);
            let class = dec.class();

            assert!(matches!($expected, class));
        }
    };

    ($name:ident, bid128_copy, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec      = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);
            let expected = decmathlib_rs::d128::dec128::BID_UINT128::from($expected);
            let copy     = dec.copy();

            assert_eq!(expected, copy);
        }
    };

    ($name:ident, bid128_copySign, $input1:expr, $input2:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let x        = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);
            let y        = decmathlib_rs::d128::dec128::BID_UINT128::from($input2);
            let expected = decmathlib_rs::d128::dec128::BID_UINT128::from($expected);
            let copy     = x.copy_sign(&y);

            assert_eq!(expected, copy);
        }
    };

    ($name:ident, bid64_to_bid128, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1     = decmathlib_rs::d128::dec128::bid64_to_bid128($input1);
            let expected = decmathlib_rs::d128::dec128::BID_UINT128::from($exp);

            assert_eq!(expected, res1);
        }
    };

    ($name:ident, bid128_from_int64, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec      = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);
            let expected = decmathlib_rs::d128::dec128::BID_UINT128::from($expected);

            assert_eq!(expected, dec);
        }
    };

    ($name:ident, bid128_from_uint64, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec      = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);
            let expected = decmathlib_rs::d128::dec128::BID_UINT128::from($expected);

            assert_eq!(expected, dec);
        }
    };

    ($name:ident, bid128_inf, $exp:expr) => {
        #[test]
        fn $name() {
            let res1     = decmathlib_rs::d128::dec128::BID_UINT128::infinity();
            let expected = decmathlib_rs::d128::dec128::BID_UINT128::from($exp);

            assert_eq!(expected, res1);
        }
    };

    ($name:ident, bid128_is_canonical, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);

            assert_eq!($exp, res1.is_canonical());
        }
    };

    ($name:ident, bid128_is_finite, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);

            assert_eq!($exp, res1.is_finite());
        }
    };

    ($name:ident, bid128_is_infinity, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);

            assert_eq!($exp, res1.is_infinity());
        }
    };

    ($name:ident, bid128_is_nan, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);

            assert_eq!($exp, res1.is_nan());
        }
    };

    ($name:ident, bid128_is_normal, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);

            assert_eq!($exp, res1.is_normal());
        }
    };

    ($name:ident, bid128_is_signaling, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);

            assert_eq!($exp, res1.is_signaling());
        }
    };

    ($name:ident, bid128_is_signed, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);

            assert_eq!($exp, res1.is_signed());
        }
    };

    ($name:ident, bid128_is_subnormal, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);

            assert_eq!($exp, res1.is_subnormal());
        }
    };

    ($name:ident, bid128_is_zero, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);

            assert_eq!($exp, res1.is_zero());
        }
    };

    ($name:ident, bid128_same_quantum, $input1:expr, $input2:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);
            let dec2 = decmathlib_rs::d128::dec128::BID_UINT128::from($input2);
            let res  = decmathlib_rs::d128::dec128::BID_UINT128::same_quantum(&dec1, &dec2);

            assert_eq!($exp, res);
        }
    };

    ($name:ident, bid128_total_order, $input1:expr, $input2:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);
            let dec2 = decmathlib_rs::d128::dec128::BID_UINT128::from($input2);
            let res  = decmathlib_rs::d128::dec128::BID_UINT128::total_order(&dec1, &dec2);

            assert_eq!($exp, res);
        }
    };

    ($name:ident, bid128_total_order_mag, $input1:expr, $input2:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::dec128::BID_UINT128::from($input1);
            let dec2 = decmathlib_rs::d128::dec128::BID_UINT128::from($input2);
            let res  = decmathlib_rs::d128::dec128::BID_UINT128::total_order_mag(&dec1, &dec2);

            assert_eq!($exp, res);
        }
    };
}