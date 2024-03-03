/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */

#[macro_export]
macro_rules! dec_test {
    ($name:ident, bid64_to_bid128, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec64 = decmathlib_rs::d64::d64($input1);
            let res1  = decmathlib_rs::d128::d128::convert_from_decimal64(dec64, &mut status);
            let exp   = decmathlib_rs::d128::d128::from($exp);

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
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::addition(&dec1, &dec2, rnd_mode, &mut status);

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
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::division(&dec1, &dec2, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_fdim, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::fdim(&dec1, &dec2, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_fma, $rnd_mode:expr, $input1:expr, $input2:expr, $input3:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let x        = decmathlib_rs::d128::d128::from($input1);
            let y        = decmathlib_rs::d128::d128::from($input2);
            let z        = decmathlib_rs::d128::d128::from($input3);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::fused_multiply_add(&x, &y, &z, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_fmod, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::fmod(&dec1, &dec2, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_frexp, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let dec1         = decmathlib_rs::d128::d128::from($input1);
            let exp_exp      = $input2;
            let exp          = decmathlib_rs::d128::d128::from($exp);
            let (res1, rexp) = decmathlib_rs::d128::d128::frexp(&dec1);

            assert_eq!(exp_exp, rexp);
            assert_eq!(exp, res1);
        }
    };

    ($name:ident, bid128_from_int64, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec = decmathlib_rs::d128::d128::from($input1);
            let exp = decmathlib_rs::d128::d128::from($expected);

            assert_eq!(exp, dec);
        }
    };

    ($name:ident, bid128_from_string, $rnd_mode:expr, $input1:expr, $expected:expr, $status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec = decmathlib_rs::d128::d128::convert_from_decimal_character($input1, rnd_mode, &mut status);
            let exp = decmathlib_rs::d128::d128::from($expected);

            assert_eq!($status, status);
            assert_eq!(exp, dec);
        }
    };

    ($name:ident, bid128_from_uint64, $input1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let dec = decmathlib_rs::d128::d128::from($input1);
            let exp = decmathlib_rs::d128::d128::from($expected);

            assert_eq!(exp, dec);
        }
    };

    ($name:ident, bid128_ilogb, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.log_b(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_inf, $exp:expr) => {
        #[test]
        fn $name() {
            let exp  = decmathlib_rs::d128::d128::from($exp);

            assert_eq!(exp, decmathlib_rs::d128::INFINITY);
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

            assert_eq!($exp, res1.is_infinite());
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

            assert_eq!($exp, res1.is_sign_minus());
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
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = dec1.ldexp($n, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_llquantexp, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.llquantexp(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_llrint, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let res1     = dec1.llrint(rnd_mode, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_llround, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.llround(&mut status);

            assert_eq!($exp, res1);
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
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let res1     = dec1.lrint(rnd_mode, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_lround, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.lround(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_maxnum, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::max_num(&dec1, &dec2, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_maxnum_mag, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::max_num_mag(&dec1, &dec2, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_minnum, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::min_num(&dec1, &dec2, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_minnum_mag, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::min_num_mag(&dec1, &dec2, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_modf, $rnd_mode:expr, $input1:expr, $exp1:expr, $exp2:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1       = decmathlib_rs::d128::d128::from($input1);
            let exp1       = decmathlib_rs::d128::d128::from($exp1);
            let exp2       = decmathlib_rs::d128::d128::from($exp2);
            let (int, fra) = decmathlib_rs::d128::d128::modf(&dec1, &mut status);

            assert_eq!(exp1, int);
            assert_eq!(exp2, fra);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_mul, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::multiplication(&dec1, &dec2, rnd_mode, &mut status);

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

    ($name:ident, bid128_nearbyint, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::nearbyint(&dec1, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_nextafter, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::next_after(&dec1, &dec2, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_nextdown, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::next_down(&dec1, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_nexttoward, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::next_toward(&dec1, &dec2, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_nextup, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::next_up(&dec1, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quantexp, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = decmathlib_rs::d128::d128::quantexp(&dec1, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quantize, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::quantize(&dec1, &dec2, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quantum, $input1:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = dec1.quantum();

            assert_eq!(exp, res1);
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
            let res1 = decmathlib_rs::d128::d128::compare_quiet_equal(&dec1, &dec2, &mut status);

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
            let res1 = decmathlib_rs::d128::d128::compare_quiet_greater(&dec1, &dec2, &mut status);

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
            let res1 = decmathlib_rs::d128::d128::compare_quiet_greater_equal(&dec1, &dec2, &mut status);

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
            let res1 = decmathlib_rs::d128::d128::compare_quiet_less(&dec1, &dec2, &mut status);

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
            let res1 = decmathlib_rs::d128::d128::compare_quiet_less_equal(&dec1, &dec2, &mut status);

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
            let res1 = decmathlib_rs::d128::d128::compare_quiet_not_equal(&dec1, &dec2, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quiet_ordered, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res1 = decmathlib_rs::d128::d128::compare_quiet_ordered(&dec1, &dec2, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_quiet_unordered, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let res1 = decmathlib_rs::d128::d128::compare_quiet_unordered(&dec1, &dec2, &mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_rem, $rnd_mode:expr, $input1:expr, $input2:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let dec2 = decmathlib_rs::d128::d128::from($input2);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::remainder(&dec1, &dec2, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_round_integral_exact, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::round_to_integral_exact(&dec1, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_round_integral_nearest_away, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::round_to_integral_ties_to_away(&dec1, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_round_integral_nearest_even, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::round_to_integral_ties_to_even(&dec1, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_round_integral_negative, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::round_to_integral_ties_toward_negative(&dec1, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_round_integral_positive, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::round_to_integral_ties_toward_positive(&dec1, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_round_integral_zero, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let exp  = decmathlib_rs::d128::d128::from($exp);
            let res1 = decmathlib_rs::d128::d128::round_to_integral_ties_toward_zero(&dec1, &mut status);

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
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let dec2     = decmathlib_rs::d128::d128::from($input2);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::subtraction(&dec1, &dec2, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_scalbn, $rnd_mode:expr, $input1:expr, $n:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = dec1.scaleb($n, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_scalbln, $rnd_mode:expr, $input1:expr, $n:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = dec1.scalebln($n, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_sqrt, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = dec1.square_root(rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_bid64, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let dec1     = decmathlib_rs::d128::d128::from($input1);
            let res1     = dec1.convert_to_decimal64(rnd_mode, &mut status);

            assert_eq!($exp, res1.0);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_ceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_toward_positive(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_floor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_toward_negative(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_int, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_toward_zero(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_rnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_ties_to_even(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_rninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_ties_to_away(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_exact_toward_positive(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xfloor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_exact_toward_negative(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_exact_toward_zero(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xrnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_exact_ties_to_even(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int32_xrninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i32_exact_ties_to_away(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_ceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i64_toward_positive(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_floor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i64_toward_negative(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_int, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i64_toward_zero(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_rnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i64_ties_to_even(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_rninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i64_ties_to_away(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i64_exact_toward_positive(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xfloor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i64_exact_toward_negative(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i64_exact_toward_zero(&mut status);

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
            let res1 = dec1.convert_to_i64_exact_ties_to_even(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_int64_xrninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_i64_exact_ties_to_away(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_ceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_toward_positive(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_floor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_toward_negative(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_int, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_toward_zero(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_rnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_ties_to_even(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_rninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_ties_to_away(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_xceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_exact_toward_positive(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_xfloor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_exact_toward_negative(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_xint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_exact_toward_zero(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_xrnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_exact_ties_to_even(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint32_xrninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u32_exact_ties_to_away(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_ceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_toward_positive(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_floor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_toward_negative(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_int, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_toward_zero(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_rnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_ties_to_even(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_rninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_ties_to_away(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_xceil, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_exact_toward_positive(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_xfloor, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_exact_toward_negative(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_xint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_exact_toward_zero(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_xrnint, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_exact_ties_to_even(&mut status);

            assert_eq!($exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128_to_uint64_xrninta, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let dec1 = decmathlib_rs::d128::d128::from($input1);
            let res1 = dec1.convert_to_u64_exact_ties_to_away(&mut status);

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

    ($name:ident, bid128ddd_fma, $rnd_mode:expr, $input1:expr, $input2:expr, $input3:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let x        = decmathlib_rs::d64::d64($input1);
            let y        = decmathlib_rs::d64::d64($input2);
            let z        = decmathlib_rs::d64::d64($input3);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::ddd_fma(x, y, z, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128dqd_fma, $rnd_mode:expr, $input1:expr, $input2:expr, $input3:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let x        = decmathlib_rs::d64::d64($input1);
            let y        = decmathlib_rs::d128::d128::from($input2);
            let z        = decmathlib_rs::d64::d64($input3);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::dqd_fma(x, &y, z, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128qdq_fma, $rnd_mode:expr, $input1:expr, $input2:expr, $input3:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let x        = decmathlib_rs::d128::d128::from($input1);
            let y        = decmathlib_rs::d64::d64($input2);
            let z        = decmathlib_rs::d128::d128::from($input3);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::qdq_fma(&x, y, &z, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid128qqd_fma, $rnd_mode:expr, $input1:expr, $input2:expr, $input3:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let mut status: decmathlib_rs::d128::_IDEC_flags = 0;
            let rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let x        = decmathlib_rs::d128::d128::from($input1);
            let y        = decmathlib_rs::d128::d128::from($input2);
            let z        = decmathlib_rs::d64::d64($input3);
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let res1     = decmathlib_rs::d128::d128::qqd_fma(&x, &y, z, rnd_mode, &mut status);

            assert_eq!(exp, res1);
            assert_eq!($exp_status, status)
        }
    };

    ($name:ident, bid_dpd_to_bid128, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let _status: decmathlib_rs::d128::_IDEC_flags = 0;
            let _rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let x        = decmathlib_rs::d128::d128::from($input1);
            let res1     = decmathlib_rs::d128::d128::decode_decimal(&x);

            assert_eq!(exp, res1);
        }
    };

    ($name:ident, bid_to_dpd128, $rnd_mode:expr, $input1:expr, $exp:expr, $exp_status:expr) => {
        #[test]
        fn $name() {
            let _status: decmathlib_rs::d128::_IDEC_flags = 0;
            let _rnd_mode = Some(decmathlib_rs::d128::RoundingMode::from($rnd_mode));
            let exp      = decmathlib_rs::d128::d128::from($exp);
            let x        = decmathlib_rs::d128::d128::from($input1);
            let res1     = decmathlib_rs::d128::d128::encode_decimal(&x);

            assert_eq!(exp, res1);
        }
    };
}