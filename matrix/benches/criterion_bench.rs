#![allow(non_snake_case)]
#![allow(dead_code)]

use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};
use matrix::{gemm_IJP, gemm_IPJ, gemm_JIP, gemm_JPI, gemm_PIJ, gemm_PJI};
use std::hint;

// TODO (checkpoint):
// - Implement tests
// - Create a doubling experiment with square matrices and one with generic ones
// using randomly generated matrices: 1, 10, 100, 1000

macro_rules! bench_with_input {
    ($group: expr, $gemm:ident, $input:expr) => {
        $group.bench_with_input(
            BenchmarkId::new(
                stringify!($gemm),
                format!("m({})Xk({})Xn({})", $input.0, $input.2, $input.1,),
            ),
            &$input,
            |b, input| {
                let (m, n, k, A, ldA, B, ldB, C, ldC) = *input;
                let mut C = Vec::from(C);
                b.iter(|| {
                    $gemm(
                        hint::black_box(m),
                        hint::black_box(n),
                        hint::black_box(k),
                        hint::black_box(&A),
                        hint::black_box(ldA),
                        hint::black_box(&B),
                        hint::black_box(ldB),
                        hint::black_box(&mut C),
                        hint::black_box(ldC),
                    )
                });
            },
        );
    };
}

pub fn basic_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("Basic test");
    let inputs: Vec<(
        usize,
        usize,
        usize,
        &[f64],
        usize,
        &[f64],
        usize,
        &[f64],
        usize,
    )> = vec![
        // m, n, k, A, ldA, B, ldB, C, ldC
        (
            2,
            2,
            2,
            &[1.0, 2.0, 3.0, 4.0],
            2,
            &[1.0, 2.0, 1.0, 2.0],
            2,
            &[2.0, 2.0, 3.0, 3.0],
            2,
        ),
        // Square matrix with non-minimal leading dimensions
        (
            3,
            3,
            3,
            &[
                1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0, 7.0, 8.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            4,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            4,
            &[
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            4,
        ),
        // Non-square with varying leading dimensions
        (
            2,
            4,
            3,
            &[1.0, 2.0, 0.0, 3.0, 4.0, 0.0, 5.0, 6.0, 0.0],
            3,
            &[
                1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 5.0, 6.0, 7.0, 8.0, 0.0, 0.0, 9.0, 10.0, 11.0, 12.0,
                0.0, 0.0, 0.0, 0.0,
            ],
            5,
            &[
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            5,
        ),
    ];

    for input in inputs {
        bench_with_input!(group, gemm_IJP, input);
        bench_with_input!(group, gemm_IPJ, input);
        bench_with_input!(group, gemm_PIJ, input);
        bench_with_input!(group, gemm_PJI, input);
        bench_with_input!(group, gemm_JIP, input);
        bench_with_input!(group, gemm_JPI, input);
    }

    group.finish();
}

criterion_group!(benches, basic_test);
criterion_main!(benches);
