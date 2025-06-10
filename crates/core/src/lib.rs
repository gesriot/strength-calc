#![forbid(unsafe_code)]

pub mod prelude;

use sprs::{CsMat, TriMat};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Topology {
    Bar2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Physics {
    Structural,
}

/// Generic material description (isotropic elastic for now).
pub trait Material: Send + Sync {
    fn e(&self) -> f64;
    fn area(&self) -> Option<f64> {
        None
    }
}

/// Generic finite-element entity.
pub trait Element: dyn_clone::DynClone + Send + Sync {
    fn topology(&self) -> Topology;
    fn physics(&self) -> Physics;
    /// Возвращает локальную матрицу жёсткости в triplet-формате.
    fn stiffness(&self) -> TriMat<f64>;
}

dyn_clone::clone_trait_object!(Element);

/// Trait responsible for assembling global matrices/vectors.
pub trait Assembler {
    /// Returns global K (CSR) and global force F (initially zeros).
    fn assemble(&self, dof: usize, elements: &[Box<dyn Element>]) -> (CsMat<f64>, Vec<f64>);
}

/// Serial implementation ― `iter()` → `par_iter()` в будущем
pub struct SerialAssembler;

impl Assembler for SerialAssembler {
    fn assemble(&self, dof: usize, elements: &[Box<dyn Element>]) -> (CsMat<f64>, Vec<f64>) {
        // 1) собираем «тройную» матрицу
        let mut tri = TriMat::with_capacity((dof, dof), elements.len() * 4);

        for el in elements {
            let k_local = el.stiffness(); // TriMat<f64>
            // triplet_iter() выдаёт (&value, (row, col))
            for (v, (r, c)) in k_local.triplet_iter() {
                tri.add_triplet(r, c, *v);
            }
        }

        // 2) конвертируем triplet → CSR
        let global = tri.to_csr();
        (global, vec![0.0; dof])
    }
}

// Small helper for doctest ― speed of sound in steel.
/// Calculates speed of sound c = sqrt(E/ρ)
///
/// ```
/// use strength_calc_core::prelude::*;
/// use strength_calc_materials::Steel;
/// let steel = Steel::default();
/// let c = (steel.e() / 7850.0).sqrt();
/// assert!((c - 5100.0).abs() < 100.0);
/// ```
pub fn sound_speed(e_modulus: f64, density: f64) -> f64 {
    (e_modulus / density).sqrt()
}
