use super::{translation::Translation, Norm};
use std::{fmt::Display, ops::Add};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Complexe {
    re: f64,
    im: f64,
}

impl Complexe {
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self {
            re: real,
            im: imaginary,
        }
    }
}

impl Display for Complexe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}+i{})", self.re, self.im)
    }
}

impl Add for Complexe {
    // associated type
    type Output = Complexe; // De quel type est le resultat d'un + sur deux complexes

    fn add(self, rhs: Self) -> Self::Output {
        Complexe::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Add<Translation> for Complexe {
    // associated type
    type Output = Complexe;

    fn add(self, rhs: Translation) -> Self::Output {
        Complexe::new(self.re + rhs.x, self.im + rhs.y)
    }
}

impl Norm for Complexe {
    fn norm(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

mod test {

    use crate::entities::Norm;
    use super::Complexe;

    #[test]
    fn test_complexe_addition() {
        let c1 = Complexe::new(1.0, 2.0);
        let c2 = Complexe::new(4.0, 5.0);
        let c_valid = Complexe::new(5.0, 7.0);
        assert_eq!(c_valid, c1 + c2);
    }

    
    #[test]
    fn test_norm () {
        let c1 = Complexe::new(1.0, 1.0);
        assert_eq!(c1.norm(), 2.0_f64.sqrt());
    }
    
    
}
