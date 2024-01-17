/* --------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.         */
/* --------------------------------------------------------------------- */
/* decmathlib-rs                                                         */
/* Copyright (C) 2023-2024 Carlos Guzmán Álvarez                         */
/* --------------------------------------------------------------------- */

#[macro_export]
macro_rules! dec_test {
    ($name:ident, bid64_to_bid128, $input1:expr, $x:expr, $y:expr) => {
        #[test]
        fn $name() {
            let res1     = decmathlib_rs::d128::dec128::bid64_to_bid128($input1);
            let expected = decmathlib_rs::d128::dec128::BID_UINT128::new($x, $y);

            assert_eq!(expected, res1);
        }
    };
}