use crate::higher_algo::use_mod;

fn main() {
    let number_list = vec![34,50,25,100,65];

    let mut higher = number_list[0];

    for number in number_list {
        if number > higher {
            higher = number;
        }
    }

    println!("Le nombre le plus grand est {}", higher);


    let liste_de_nombres = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut le_plus_grand = liste_de_nombres[0];

    for nombre in liste_de_nombres {
        if nombre > le_plus_grand {
            le_plus_grand = nombre;
        }
    }

    println!("Le nombre le plus grand est {}", le_plus_grand);

    use_mod();
}

pub mod higher_algo {
    fn le_plus_grand(list: &[i32]) -> i32 {
        let mut the_higher = list[0];

        for &element in list {
            if element > the_higher {
                the_higher = element;
            }
        }

        the_higher
    }

    pub fn use_mod() {
        let number_list_1 = vec![34,50,25,100,65];

        let result = le_plus_grand(&number_list_1);
        println!("Le nombre le plus grand est {}", result);

        let number_list_2 = vec![102,34,6000,89,54,2,43,8];

        let result = le_plus_grand(&number_list_2);
        println!("Le nombre le plus grand est {}", result);
    }
}