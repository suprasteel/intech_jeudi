pub mod complexe;
pub mod translation;

pub trait Norm {
    fn norm(&self) -> f64;
}