#![forbid(unsafe_code)]

use nalgebra::{DMatrix, DVector};
use sprs::CsMat;

/// Naïve dense solver for Sprint 0.
pub fn solve(global: CsMat<f64>, mut f: Vec<f64>, fixed_dofs: &[usize]) -> Vec<f64> {
    let n = global.rows();
    assert_eq!(n, global.cols());

    // 1) конвертируем разреженную CSR → плотную матрицу
    let mut dense = DMatrix::<f64>::zeros(n, n);
    for (row, vec) in global.outer_iterator().enumerate() {
        for (col, &val) in vec.iter() {
            dense[(row, col)] = val;
        }
    }

    // 2) применяем Дирихле (фиксированные DOF)
    for &dof in fixed_dofs {
        for c in 0..n {
            dense[(dof, c)] = 0.0;
        }
        for r in 0..n {
            dense[(r, dof)] = 0.0;
        }
        dense[(dof, dof)] = 1.0;
        f[dof] = 0.0;
    }

    // 3) решаем
    let b = DVector::from_vec(f);
    let u = dense.lu().solve(&b).expect("Singular matrix");
    u.data.as_vec().clone()
}
