pub trait Affichable {
    fn afficher(&self);
}

pub struct Ecran {
    pub composants: Vec<Box<dyn Affichable>>,
}

impl Ecran {
    pub fn executer(&self) {
        for composant in self.composants.iter() {
            composant.afficher();
        }
    }
}

// 17.6

// pub trait Affichable {
//     fn afficher(&self);
// }
// 
// pub struct Ecran<T: Affichable> {
//     pub composants: Vec<T>,
// }
// 
// impl<T> Ecran<T>
// where
//     T: Affichable,
// {
//     pub fn executer(&self) {
//         for composant in self.composants.iter() {
//             composant.afficher();
//         }
//     }
// }