use std::thread;
use std::time::Duration;

struct Cache<T>
where
T: Fn(u32) -> u32,
{
    calcul: T,
    valeur: Option<u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32
{
    fn new(calcul: T) -> Cache<T> {
        Cache {
            calcul,
            valeur: None,
        }
    }

    fn valeur(&mut self, arg: u32) -> u32 {
        match self.valeur {
            Some(v) => v,
            None => {
                let v = (self.calcul) (arg);
                self.valeur = Some(v);
                v
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let utilisateur_nombre_simule = 10;
    let nombre_aleatoire_simule = 7;
}

fn simuler_gros_calcul(intensite: u32) -> u32 {
    println!("Calcul très lent...");
    thread::sleep(Duration::from_secs(2));
    intensite
}

// fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {
//     if intensite < 25 {
//         println!(
//             "Ajourd'hui, faire {} pompes",
//             simuler_gros_calcul(intensite)
//         );
//         println!(
//             "Ensuite, faire {} abdominaux",
//             simuler_gros_calcul(intensite)
//         );
//     } else {
//         if nombre_aleatoire == 3 {
//             println!("Faites une pause aujourd'hui ! Rappelez-vous de bien vous hydrater !");
//         } else {
//             println!(
//                 "Aujourd'hui, courrez pendant {} minutes",
//                 simuler_gros_calcul(intensite)
//             );
//         }
//     }
// }

fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {
    let fermeture_lente = |nombre| {
        println!("Calcul très lent ...");
        thread::sleep(Duration::from_secs(2));
        nombre
    };

    if intensite < 25 {
        println!("Aujourd'hui, faire {} pompes", fermeture_lente(intensite));
        println!("Ensuite faire {} abdominaux", fermeture_lente(intensite));
    } else {
        if nombre_aleatoire == 3 {
            println!("Faites une pause aujourd'hui ! Rappelez vous de bien vous hydrater");
        } else {
            println!("Aujourd'hui, courrez pendant {} minutes", fermeture_lente(intensite));
        }
    }
}
