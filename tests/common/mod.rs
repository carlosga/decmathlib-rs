/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

#[macro_export]
macro_rules! dec_test {
    ($name:ident, bid64_to_bid128, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let res1 = decmathlib_rs::d128::d128::from_decimal64($input1, &mut status);
            let exp  = decmathlib_rs::d128::d128::from($exp);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status);
        }
    };

    ($name:ident, bid128_abs, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);

            assert_eq!(exp, res1.abs());
        }
    };

    ($name:ident, bid128_add, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::add(&dec1, &dec2, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_class, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec   = decmathlib_rs::d128::d128::from($input1);
            let class = dec.class();

            assert!(matches!($expected, class));
        }
    };

    ($name:ident, bid128_copy, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec  = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($expected);
            let copy = dec.copy();

            assert_eq!(exp, copy);
        }
    };

    ($name:ident, bid128_copy_sign, $input1:expr, $input2:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let x    = decmathlib_rs::d128::d128::from($input1);
            let y    = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($expected);
            let copy = x.copy_sign(&y);

            assert_eq!(exp, copy);
        }
    };

    ($name:ident, bid128_div, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::divide(&dec1, &dec2, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_from_int64, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec = decmathlib_rs::d128::d128::from_i64($input1);
            let exp = decmathlib_rs::d128::d128::from($expected);

            assert_eq!(exp, dec);
        }
    };

    ($name:ident, bid128_from_string, $rnd_mode:expr, $input1:expr, $expected:expr, $status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec = decmathlib_rs::d128::d128::from_string($input1, rnd_mode, &mut status);
            let exp = decmathlib_rs::d128::d128::from($expected);

            assert_eq!($status, status);
            assert_eq!(exp, dec);
        }
    };

    ($name:ident, bid128_from_uint64, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec = decmathlib_rs::d128::d128::from_u64($input1);
            let exp = decmathlib_rs::d128::d128::from($expected);

            assert_eq!(exp, dec);
        }
    };

    ($name:ident, bid128_ilogb, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.ilogb(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_inf, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::infinity();
            let exp  = decmathlib_rs::d128::d128::from($exp);

            assert_eq!(exp, res1);
        }
    };

    ($name:ident, bid128_is_canonical, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);

            assert_eq!($exp, res1.is_canonical());
        }
    };

    ($name:ident, bid128_is_finite, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);

            assert_eq!($exp, res1.is_finite());
        }
    };

    ($name:ident, bid128_is_infinity, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);

            assert_eq!($exp, res1.is_infinity());
        }
    };

    ($name:ident, bid128_is_nan, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);

            assert_eq!($exp, res1.is_nan());
        }
    };

    ($name:ident, bid128_is_normal, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);

            assert_eq!($exp, res1.is_normal());
        }
    };

    ($name:ident, bid128_is_signaling, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);

            assert_eq!($exp, res1.is_signaling());
        }
    };

    ($name:ident, bid128_is_signed, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);

            assert_eq!($exp, res1.is_signed());
        }
    };

    ($name:ident, bid128_is_subnormal, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);

            assert_eq!($exp, res1.is_subnormal());
        }
    };

    ($name:ident, bid128_is_zero, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res1 = decmathlib_rs::d128::d128::from($input1);

            assert_eq!($exp, res1.is_zero());
        }
    };

    ($name:ident, bid128_ldexp, $rnd_mode:expr, $input1:expr, $n:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = dec1.ldexp($n, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_logb, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = dec1.logb(&mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_lrint, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let res1     = dec1.lrint(rnd_mode, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_mul, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::multiply(&dec1, &dec2, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_nan, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let res1 = decmathlib_rs::d128::d128::nan($input1, &mut status);
            let exp  = decmathlib_rs::d128::d128::from($exp);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_negate, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res  = decmathlib_rs::d128::d128::negate(&dec1);

            assert_eq!(exp, res);
        }
    };

    ($name:ident, bid128_quiet_equal, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res1 = decmathlib_rs::d128::d128::quiet_equal(&dec1, &dec2, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quiet_greater, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res1 = decmathlib_rs::d128::d128::quiet_greater(&dec1, &dec2, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quiet_greater_equal, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res1 = decmathlib_rs::d128::d128::quiet_greater_equal(&dec1, &dec2, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quiet_less, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res1 = decmathlib_rs::d128::d128::quiet_less(&dec1, &dec2, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quiet_less_equal, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res1 = decmathlib_rs::d128::d128::quiet_less_equal(&dec1, &dec2, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quiet_not_equal, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res1 = decmathlib_rs::d128::d128::quiet_not_equal(&dec1, &dec2, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_rem, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            // let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::remainder(&dec1, &dec2, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_same_quantum, $input1:expr, $input2:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res  = decmathlib_rs::d128::d128::same_quantum(&dec1, &dec2);

            assert_eq!($exp, res);
        }
    };

    ($name:ident, bid128_sub, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::subtract(&dec1, &dec2, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_scalbn, $rnd_mode:expr, $input1:expr, $n:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = dec1.scalbn($n, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_scalbln, $rnd_mode:expr, $input1:expr, $n:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = dec1.scalbln($n, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_bid64, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some($rnd_mode);
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let res1     = dec1.to_decimal64(rnd_mode, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_ceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_ceil(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_floor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_floor(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_int, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_int(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_rnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_rnint(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_rninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_rninta(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_xceil(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xfloor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_xfloor(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_xint(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xrnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_xrnint(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xrninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i32_xrninta(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_ceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_ceil(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_floor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_floor(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_int, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_int(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_rnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_rnint(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_rninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_rninta(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_xceil(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xfloor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_xfloor(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_xint(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xinta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_xinta(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xrnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_xrnint(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xrninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_i64_xrninta(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_string, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.to_string();

            assert_eq!($exp, res1);
        }
    };

    ($name:ident, bid128_total_order, $input1:expr, $input2:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res  = decmathlib_rs::d128::d128::total_order(&dec1, &dec2);

            assert_eq!($exp, res);
        }
    };

    ($name:ident, bid128_total_order_mag, $input1:expr, $input2:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res  = decmathlib_rs::d128::d128::total_order_mag(&dec1, &dec2);

            assert_eq!($exp, res);
        }
    };
}