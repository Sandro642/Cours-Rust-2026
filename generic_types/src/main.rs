use crate::complex_type::use_ugly_impl;
use crate::type_parameter::use_impl;

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

    // use_mod();

    use_impl();
    use_ugly_impl();
}

pub mod higher_algo {
    // fn le_plus_grand(list: &[i32]) -> i32 {
    //     let mut the_higher = list[0];
    //
    //     for &element in list {
    //         if element > the_higher {
    //             the_higher = element;
    //         }
    //     }
    //
    //     the_higher
    // }

    fn le_plus_grand_i32(list: &[i32]) -> i32 {
        let mut le_plus_grand = list[0];

        for &element in list.iter() {
            if element > le_plus_grand {
                le_plus_grand = element;
            }
        }

        le_plus_grand
    }

    fn le_plus_grand_caractere(list: &[char]) -> char {
        let mut le_plus_grand = list[0];

        for &element in list.iter() {
            if element > le_plus_grand {
                le_plus_grand = element;
            }
        }

        le_plus_grand
    }

    fn le_plus_grand<T: PartialOrd>(list: &[T]) -> &T {
        let mut le_plus_grand = &list[0];

        for element in list {
            if element > le_plus_grand {
                le_plus_grand = element;
            }
        }

        le_plus_grand
    }

    pub fn use_mod() {
        let number_list_1 = vec![34,50,25,100,65];
        //
        // let result = le_plus_grand(&number_list_1);
        // println!("Le nombre le plus grand est {}", result);
        //
        // let number_list_2 = vec![102,34,6000,89,54,2,43,8];
        //
        // let result = le_plus_grand(&number_list_2);
        // println!("Le nombre le plus grand est {}", result);

        //

        let result = le_plus_grand(&number_list_1);
        println!("Le plus grand nombre est {}", result);

        let char_list_1 = vec!['y', 'm', 'a', 'q'];

        let result = le_plus_grand(&char_list_1);
        println!("Le plus grand caractère est {}", result);
    }
}

pub mod type_parameter {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    fn use_struct() {
        let entiers = Point {x: 5, y: 10};
        let flottants = Point {x: 1.0, y: 4.0};

        // let ne_fonctionne_pas = Point {x: 1, y: 2.0};
    }

    pub fn use_impl() {
        let p = Point {x:5, y:10};

        println!("p.x = {}", p.x());
    }

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

pub mod complex_type {
    struct Point<X1,Y1>  {
        x: X1,
        y: Y1,
    }

    impl<X1,Y1> Point<X1,Y1> {
        fn melange<X2,Y2>(self, other: Point<X2,Y2>) -> Point<X1,Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    pub fn use_ugly_impl() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.melange(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}