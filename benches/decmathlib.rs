/* ----------------------------------------------------------------------------- */
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.*/
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* mpdecimal bench - https://www.bytereef.org/mpdecimal/                         */
/* Copyright (c) 2009, Stefan Krah                                               */
/* ----------------------------------------------------------------------------- */

#![allow(non_snake_case)]

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use decmathlib_rs::d128::{_IDEC_flags, d128};#[derive(Debug, Copy, Clone)]

struct CarbonGasInputs {
    iterations: u32,
    k: d128,
    T: d128,
    a: d128,
    b: d128,
    N: d128,
    p: d128,
    V: d128,
}

impl Display for CarbonGasInputs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {}, {}, {})", self.iterations, self.k, self.T, self.a, self.b, self.N, self.p, self.V)
    }
}

// https://fpbench.org/benchmarks.html
fn carbon_gas_benchmark(c: &mut Criterion) {
    let mut pfpsf: _IDEC_flags = 0;
    let inputs = CarbonGasInputs {
        iterations: 10_000_000,
        k: d128::convert_from_decimal_character("1.3806503e-23", None, &mut pfpsf),
        T: d128::convert_from_decimal_character("300.0"        , None, &mut pfpsf),
        a: d128::convert_from_decimal_character("0.401"        , None, &mut pfpsf),
        b: d128::convert_from_decimal_character("42.7e-6"      , None, &mut pfpsf),
        N: d128::convert_from_decimal_character("1000"         , None, &mut pfpsf),
        p: d128::convert_from_decimal_character("3.5e7"        , None, &mut pfpsf),
        V: d128::convert_from_decimal_character("0.5"          , None, &mut pfpsf),
    };

    let mut group = c.benchmark_group("decmathlib");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    group.bench_with_input(BenchmarkId::new("carbon_gas", inputs), &inputs, |b, &i| {
        b.iter(|| carbon_gas(black_box(i.k), black_box(i.T), black_box(i.a), black_box(i.b), black_box(i.N), black_box(i.p), black_box(i.V), black_box(i.iterations)));
    });
    group.finish();
}

fn carbon_gas(k: d128, T: d128, a: d128, b: d128, N: d128, p: d128, V: d128, maxiter: u32) -> d128 {
    // Constants
    //   k = 1.3806503e-23;
    //
    // Variables
    //   T in [300.0    , 300.0];
    //   a in [0.401    , 0.401];
    //   b in [42.7e-6  , 42.7e-6];
    //   N in [1000     , 1000];
    //   p in [3.5e7    , 3.5e7];
    //   V in [0.1      , 0.5];
    //
    // Definitions
    //   res rnd64= (p + a * (N / V) * (N / V)) * (V - N * b) - k * N * T;
    //
    // Expressions
    //   crate::carbon_gas = res;

    let mut rnd128 = d128::default();

    for _ in 0..maxiter {
        rnd128 += (p + a * (N / V) * (N / V)) * (V - N * b) - k * N * T;
    }

    black_box(rnd128)
}

#[derive(Debug, Copy, Clone)]
struct KeplerInputs {
    iterations: u32,
    x1: d128,
    x2: d128,
    x3: d128,
    x4: d128,
}

impl Display for KeplerInputs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {})", self.iterations, self.x1, self.x2, self.x3, self.x4)
    }
}

fn kepler_benchmark(c: &mut Criterion) {
    let mut pfpsf: _IDEC_flags = 0;
    let inputs = KeplerInputs {
        iterations: 10_000_000,
        x1: d128::convert_from_decimal_character("636000000000000031974423109204508364200592041015625E-50", None, &mut pfpsf),
        x2: d128::convert_from_decimal_character("636000000000000031974423109204508364200592041015625E-50", None, &mut pfpsf),
        x3: d128::convert_from_decimal_character("636000000000000031974423109204508364200592041015625E-50", None, &mut pfpsf),
        x4: d128::convert_from_decimal_character("636000000000000031974423109204508364200592041015625E-50", None, &mut pfpsf)
    };

    let mut group = c.benchmark_group("decmathlib");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    group.bench_with_input(BenchmarkId::new("kepler", inputs), &inputs, |b, &i| {
        b.iter(|| kepler(black_box(i.x1), black_box(i.x2), black_box(i.x3), black_box(i.x4), black_box(i.iterations)));
    });
    group.finish();
}

fn kepler(x1: d128, x2: d128, x3: d128, x4: d128, maxiter: u32) -> d128 {
    // {
    // Variables
    // 	real x1 in [4, 636000000000000031974423109204508364200592041015625e-50];
    // 	real x2 in [4, 636000000000000031974423109204508364200592041015625e-50];
    // 	real x3 in [4, 636000000000000031974423109204508364200592041015625e-50];
    // 	real x4 in [4, 636000000000000031974423109204508364200592041015625e-50];
    //
    // Expressions
    // 	kepler1 rnd64= ((((((((x1 * x4) * (((-x1 + x2) + x3) - x4)) + (x2 * (((x1 - x2) + x3) + x4))) + (x3 * (((x1 + x2) - x3) + x4))) - ((x2 * x3) * x4)) - (x1 * x3)) - (x1 * x2)) - x4);
    // }

    let mut rnd128= d128::default();

    for _ in 0..maxiter {
        rnd128 += (((((((x1 * x4) * (((-x1 + x2) + x3) - x4)) + (x2 * (((x1 - x2) + x3) + x4))) + (x3 * (((x1 + x2) - x3) + x4))) - ((x2 * x3) * x4)) - (x1 * x3)) - (x1 * x2)) - x4;
    }

    black_box(rnd128)
}

fn from_string_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("decmathlib");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    group.bench_function("from_string", |b| b.iter(|| d128::from_str("9.999999999999999999999999999999999E6144")));
    group.finish();
}

criterion_group!(benches, from_string_benchmark, kepler_benchmark, carbon_gas_benchmark);
criterion_main!(benches);