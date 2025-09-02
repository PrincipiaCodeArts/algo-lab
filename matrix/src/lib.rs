#![allow(non_snake_case)]

macro_rules! get {
    ($Arr:expr, $ldArr:expr, $i:expr, $j:expr) => {
        $Arr[$j * $ldArr + $i]
    };
}

pub fn gemm_IJP(
    m: usize,
    n: usize,
    k: usize,
    A: &[f64],
    ldA: usize,
    B: &[f64],
    ldB: usize,
    C: &mut [f64],
    ldC: usize,
) {
    for i in 0..m {
        for j in 0..n {
            for p in 0..k {
                get!(C, ldC, i, j) += get!(A, ldA, i, p) * get!(B, ldB, p, j);
            }
        }
    }
}

pub fn gemm_IPJ(
    m: usize,
    n: usize,
    k: usize,
    A: &[f64],
    ldA: usize,
    B: &[f64],
    ldB: usize,
    C: &mut [f64],
    ldC: usize,
) {
    for i in 0..m {
        for p in 0..k {
            for j in 0..n {
                get!(C, ldC, i, j) += get!(A, ldA, i, p) * get!(B, ldB, p, j);
            }
        }
    }
}

pub fn gemm_JIP(
    m: usize,
    n: usize,
    k: usize,
    A: &[f64],
    ldA: usize,
    B: &[f64],
    ldB: usize,
    C: &mut [f64],
    ldC: usize,
) {
    for j in 0..n {
        for i in 0..m {
            for p in 0..k {
                get!(C, ldC, i, j) += get!(A, ldA, i, p) * get!(B, ldB, p, j);
            }
        }
    }
}

pub fn gemm_JPI(
    m: usize,
    n: usize,
    k: usize,
    A: &[f64],
    ldA: usize,
    B: &[f64],
    ldB: usize,
    C: &mut [f64],
    ldC: usize,
) {
    for j in 0..n {
        for p in 0..k {
            for i in 0..m {
                get!(C, ldC, i, j) += get!(A, ldA, i, p) * get!(B, ldB, p, j);
            }
        }
    }
}

pub fn gemm_PIJ(
    m: usize,
    n: usize,
    k: usize,
    A: &[f64],
    ldA: usize,
    B: &[f64],
    ldB: usize,
    C: &mut [f64],
    ldC: usize,
) {
    for p in 0..k {
        for i in 0..m {
            for j in 0..n {
                get!(C, ldC, i, j) += get!(A, ldA, i, p) * get!(B, ldB, p, j);
            }
        }
    }
}

pub fn gemm_PJI(
    m: usize,
    n: usize,
    k: usize,
    A: &[f64],
    ldA: usize,
    B: &[f64],
    ldB: usize,
    C: &mut [f64],
    ldC: usize,
) {
    for p in 0..k {
        for j in 0..n {
            for i in 0..m {
                get!(C, ldC, i, j) += get!(A, ldA, i, p) * get!(B, ldB, p, j);
            }
        }
    }
}
