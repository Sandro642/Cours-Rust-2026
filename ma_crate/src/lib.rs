pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/// Ajoute 1 au nombre donné.
///
/// #Exemples
///
/// ```
/// let argument = 5;
/// let reponse = ma_crate::ajouter_un(argument);
///
/// assert_eq!(6, reponse);
/// ```
pub fn ajouter_un(x: i32) -> i32 {
    x + 1
}

pub use self::types::CouleurPrimaire;
pub use self::types::CouleurSecondaire;
pub use self::utilitaires::mixer;

pub mod types {
    pub enum CouleurPrimaire {
        Rouge,
        Jaune,
        Bleu,
    }

    pub enum CouleurSecondaire {
        Orange,
        Vert,
        Violet,
    }
}

pub mod utilitaires {
    use crate::types::{CouleurSecondaire::Orange, *};

    pub fn mixer(c1: CouleurPrimaire, c2: CouleurPrimaire) -> CouleurSecondaire {
        return Orange;
    }
}
