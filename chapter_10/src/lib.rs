#![cfg_attr(feature = "nightly", feature(portable_simd))]

use rand::Rng;

pub type Matrix = Vec<Vec<f32>>;

pub fn matrix_multiply_original(a: Matrix, b: Matrix) -> Matrix {
    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_cols = b[0].len();

    let mut out = vec![vec![0.0; b_cols]; a_rows];

    for i in 0..a_rows {
        for j in 0..b_cols {
            for k in 0..a_cols {
                out[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    out
}

pub fn matrix_multiply_indexing_opt(a: Matrix, b: Matrix) -> Matrix {
    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_cols = b[0].len();

    let mut out = vec![vec![0.0; b_cols]; a_rows];

    for i in 0..a_rows {
        for j in 0..b_cols {
            let mut sum = 0.0;
            for k in 0..a_cols {
                sum += a[i][k] * b[k][j];
            }
            out[i][j] = sum;
        }
    }
    out
}

pub fn matrix_multiply_unsafe_indexing(a: Matrix, b: Matrix) -> Matrix {
    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_cols = b[0].len();

    let mut out = vec![vec![0.0; b_cols]; a_rows];

    for i in 0..a_rows {
        for j in 0..b_cols {
            let mut sum = 0.0;
            for p in 0..a_cols {
                sum += unsafe {
                    a.get_unchecked(i).get_unchecked(p) * b.get_unchecked(p).get_unchecked(j)
                };
            }
            unsafe { *out.get_unchecked_mut(i).get_unchecked_mut(j) = sum };
        }
    }
    out
}

pub fn matrix_multiply_unsafe_init(a: Matrix, b: Matrix) -> Matrix {
    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_cols = b[0].len();

    let mut out = Vec::with_capacity(a_rows);

    for i in 0..a_rows {
        let mut row = Vec::with_capacity(b_cols);
        for j in 0..b_cols {
            let mut sum = 0.0;
            for p in 0..a_cols {
                sum += unsafe {
                    a.get_unchecked(i).get_unchecked(p) * b.get_unchecked(p).get_unchecked(j)
                };
            }
            row.push(sum);
        }
        out.push(row);
    }
    out
}

#[cfg(feature = "nightly")]
mod simd;

#[cfg(feature = "nightly")]
pub use simd::matrix_multiply_simd as matrix_multiply;

#[cfg(not(feature = "nightly"))]
pub use matrix_multiply_unsafe_indexing as matrix_multiply;

pub fn make_random(rows: usize, columns: usize) -> Matrix {
    let mut rng = rand::thread_rng();
    (0..rows)
        .map(|_| (0..columns).map(|_| rng.gen::<f32>()).collect())
        .collect()
}
