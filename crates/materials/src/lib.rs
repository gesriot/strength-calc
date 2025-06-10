#![forbid(unsafe_code)]
use serde::{Deserialize, Serialize};
use strength_calc_core::Material;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Steel {
    e: f64,
    area: f64,
}

impl Default for Steel {
    fn default() -> Self {
        Self {
            e: 210e9,   // Pa
            area: 1e-4, // m² (dummy)
        }
    }
}

impl Material for Steel {
    fn e(&self) -> f64 {
        self.e
    }
    fn area(&self) -> Option<f64> {
        Some(self.area)
    }
}
