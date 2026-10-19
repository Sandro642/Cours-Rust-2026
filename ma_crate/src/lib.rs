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


//! # Ma ma_crate
//!
//! 'ma_crate' est un regroupement d'utilitaires pour rendre plus pratique
//! certains calculs.

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

