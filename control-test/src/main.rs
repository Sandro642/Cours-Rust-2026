fn main() {
    println!("Hello, world!");
}

fn ajouter_deux(a: i32) -> i32 {
    a + 2
}

fn affiche_et_retourne_10(a: i32) -> i32 {
    println!("J'ai obtenu la valeur {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ce_test_reussit() {
        let valeur = affiche_et_retourne_10(4);
        assert_eq!(10, valeur);
    }

    #[test]
    fn ajouter_deux_a_deux() {
        assert_eq!(4, ajouter_deux(2));
    }

    #[test]
    fn ajouter_deux_a_trois() {
        assert_eq!(5, ajouter_deux(3));
    }

    #[test]
    fn cent() {
        assert_eq!(102, ajouter_deux(100));
    }

    #[test]
    #[ignore]
    fn fonction_gavé_long() {}

    #[test]
    fn ce_test_echoue() {
        let valeur = affiche_et_retourne_10(8);
        assert_eq!(5, valeur);
    }
}
