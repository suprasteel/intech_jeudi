mod entities;

use entities::complexe::Complexe;
use entities::translation::Translation;

use crate::entities::Norm;

fn main() {
    let c1 = Complexe::new(1.0, 2.0);
    let c2 = Complexe::new(4.0, 5.0);

    let c3 = c1 + c2;
    // let c3 = Complexe::default();

    
    let translation = Translation::new(4.0, 5.0);

    // Commentaires pour grouper affichage exercice
    // pub trait Norm {
    //     fn norm(&self) -> f64;
    // }

    // impl Norm for Translation {
    //     fn norm(&self) -> f64 {
    //         (self.x * self.x + self.y * self.y).sqrt()
    //     }
    // }

    translation.norm();
    c3.norm();

    // let mut list = vec![];

    // list.push(translation);
    // list.push(c3);

    // list.iter().for_each(
    //     list.norm()
    // );

    println!("{} + {} = {}", c1, c2, c3); // objectif: un comportement cohérent
}

mod test_main {

    use super::entities::complexe::Complexe;
    use super::entities::translation::Translation;

    #[test]
    fn test_complexe_translation() {
        let c1 = Complexe::new(1.0, 2.0);
        let translation = Translation::new(4.0, 5.0);
        let c_valid = Complexe::new(5.0, 7.0);
        assert_eq!(c_valid, c1 + translation);
    }
}
