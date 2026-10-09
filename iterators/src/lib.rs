#[derive(PartialEq, Debug)]
struct Chaussure {
    pointure: u32,
    style: String,
}

struct Compteur {
    compteur: u32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { compteur: 0 }
    }
}

impl Iterator for Compteur {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.compteur < 5 {
            self.compteur += 1;
            Some(self.compteur)
        } else {
            None
        }
    }
}

fn chaussures_a_la_pointure(chaussures: Vec<Chaussure>, pointure_chaussure: u32) -> Vec<Chaussure> {
    chaussures
        .into_iter()
        .filter(|s| s.pointure == pointure_chaussure)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtres_par_pointure() {
        let chaussures = vec![
            Chaussure {
                pointure: 10,
                style: String::from("baskets"),
            },
            Chaussure {
                pointure: 13,
                style: String::from("sandale"),
            },
            Chaussure {
                pointure: 10,
                style: String::from("bottes"),
            },
        ];

        let a_ma_pointure = chaussures_a_la_pointure(chaussures, 10);

        assert_eq!(
            a_ma_pointure,
            vec![
                Chaussure {
                    pointure: 10,
                    style: String::from("baskets"),
                },
                Chaussure {
                    pointure: 10,
                    style: String::from("bottes"),
                },
            ]
        );
    }

    #[test]
    fn appel_direct_a_next() {
        let mut compteur = Compteur::new();

        assert_eq!(compteur.next(), Some(1));
        assert_eq!(compteur.next(), Some(2));
        assert_eq!(compteur.next(), Some(3));
        assert_eq!(compteur.next(), Some(4));
        assert_eq!(compteur.next(), Some(5));
        assert_eq!(compteur.next(), None);
    }

    #[test]
    fn utilisation_des_autres_methodes_du_trait_iterator() {
        let some: u32 = Compteur::new()
            .zip(Compteur::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, some);
    }
}
