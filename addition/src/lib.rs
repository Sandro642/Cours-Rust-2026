#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("deux plus deux ne vaut pas quatre"))
        }
    }

    // #[test]
    // fn another_one() {
    //     panic!("Fait échouer ce test");
    // }

    #[derive(Debug)]
    struct Rectangle {
        largeur: u32,
        hauteur: u32,
    }

    impl Rectangle {
        fn peut_contenir(&self, other: &Rectangle) -> bool {
            self.largeur <  other.largeur && self.hauteur > other.hauteur
        }
    }

    // #[test]
    // fn un_grand_peut_contenir_un_petit() {
    //     let le_grand = Rectangle {largeur: 8, hauteur: 7};
    //     let le_petit = Rectangle {largeur: 5, hauteur: 1};
    //
    //     assert!(le_grand.peut_contenir(&le_petit));
    // }

    #[test]
    fn un_petit_peut_contenir_un_plus_grand() {
        let le_grand = Rectangle {largeur: 8, hauteur: 7};
        let le_petit = Rectangle {largeur: 5, hauteur: 1};

        assert!(!le_petit.peut_contenir(&le_grand));
    }

    #[test]
    fn ceci_ajoute_deux() {
        assert_ne!(4, ajouter_deux(2));
    }

    #[test]
    fn accueil_contient_le_nom() {
        let resultat = accueil("Caroline");

        assert!(
            resultat.contains("Caroline"),
            "Le message d'accueil ne contient pas le nom, il vaut `{}`",
            resultat
        );
    }

    #[test]
    #[should_panic(expected = "La supposition doit être plus petite ou égale à 100")]
    fn plus_grand_que_100() {
        Supposition::new(200);
    }

}


pub struct Supposition {
    valeur: i32,
}

impl Supposition {
    pub fn new(valeur: i32) -> Supposition {
        if valeur < 1 {
            panic!(
                "La supposition doit être plus petite ou égale à 100, et nous avons {}",
                valeur
            );
        } else if valeur > 100 {
            panic!(
                "La supposition doit être plus grande ou égale à 1, et nous avons {}",
                valeur
            );
        }

        Supposition {valeur}
    }
}

pub fn accueil(nom: &str) -> String {
    format!("Salut ! {}", nom)
}

pub fn ajouter_deux(a: i32) -> i32 {
    a + 3
}

