use std::fmt::Debug;

pub mod complexe;
pub mod translation;

pub trait Norm: Debug {
    fn norm(&self) -> f64;
}
