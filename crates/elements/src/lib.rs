#![forbid(unsafe_code)]

use sprs::TriMat;
use strength_calc_core::{Element, Physics, Topology};

/// Bar2 ― 2-node axial element (1 DOF per node).
#[derive(Clone, Debug)]
pub struct Bar2 {
    /// Глобальные номера узлов
    pub node_ids: [usize; 2],
    /// Модуль Юнга
    pub e: f64,
    /// Площадь поперечного сечения
    pub area: f64,
    /// Длина элемента
    pub length: f64,
}

impl Element for Bar2 {
    fn topology(&self) -> Topology {
        Topology::Bar2
    }

    fn physics(&self) -> Physics {
        Physics::Structural
    }

    fn stiffness(&self) -> TriMat<f64> {
        // строитель тройной матрицы 2×2 с 4 потенциальными ненулями
        let k = self.e * self.area / self.length;
        let mut tri = TriMat::with_capacity((2, 2), 4);

        tri.add_triplet(0, 0, k);
        tri.add_triplet(0, 1, -k);
        tri.add_triplet(1, 0, -k);
        tri.add_triplet(1, 1, k);

        tri
    }
}
