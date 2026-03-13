use crate::{make_random, Matrix};
use std::simd::{f32x4, simd_swizzle};

pub fn matrix_multiply_simd(a: Matrix, b: Matrix) -> Matrix {
    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_cols = b[0].len();

    let mut out = vec![vec![0.0; b_cols]; a_rows];

    for i in 0..a_rows {
        for j in 0..b_cols {
            let mut sum = f32x4::splat(0.0);

            for p in (0..a_cols).step_by(4) {
                if p + 4 <= a_cols {
                    let a_vec = f32x4::from_slice(&a[i][p..p + 4]);
                    let b_vec = f32x4::from_array(unsafe {
                        [
                            *b.get_unchecked(p + 0).get_unchecked(j),
                            *b.get_unchecked(p + 1).get_unchecked(j),
                            *b.get_unchecked(p + 2).get_unchecked(j),
                            *b.get_unchecked(p + 3).get_unchecked(j),
                        ]
                    });

                    sum += a_vec * b_vec;
                }
            }

            sum += simd_swizzle!(sum, [1, 0, 3, 2]);
            sum += simd_swizzle!(sum, [2, 2, 0, 0]);
            unsafe {
                *out.get_unchecked_mut(i).get_unchecked_mut(j) = sum[0];
            }
        }
    }
    out
}
