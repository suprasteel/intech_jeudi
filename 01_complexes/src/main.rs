mod entities;

use std::rc::Rc;

use entities::complexe::Complexe;
use entities::translation::Translation;

use crate::entities::Norm;

fn main() {
    let c1 = Complexe::new(1.0, 2.0);
    let c2 = Complexe::new(4.0, 5.0);

    let c3 = c1 + c2;
    // let c3 = Complexe::default();

    let translation = Translation::new(4.0, 5.0);

    translation.norm();
    c3.norm();

    // utiliser box, allouer en heap
    let box_complexe_3 = Box::new(c3);
    let box_translation = Box::new(translation);

    // un vecteur qui travaille sur des types implémentants Norm
    let mut list: Vec<Box<dyn Norm>> = vec![];

    let _slice = &list[..];

    // pousser deux type différents dans notre liste
    list.push(box_complexe_3);
    list.push(box_translation);

    // un itérateur mutablqui prend l'ownership des valeurs de la liste
    list.into_iter().for_each(|a| println!("{:?}", a));

    println!("{} + {} = {}", c1, c2, c3); // objectif: un comportement cohérent

    // présentation du smartpointer Rc
    let rc_1 = Rc::new("hello word");

    {
        let _rc_2 = rc_1.clone();
    }

    dbg!(rc_1);
}

mod test_main {
    #[allow(unused_imports)]
    use super::entities::complexe::Complexe;
    #[allow(unused_imports)]
    use super::entities::translation::Translation;

    #[test]
    fn test_complexe_translation() {
        let c1 = Complexe::new(1.0, 2.0);
        let translation = Translation::new(4.0, 5.0);
        let c_valid = Complexe::new(5.0, 7.0);
        assert_eq!(c_valid, c1 + translation);
    }
}
