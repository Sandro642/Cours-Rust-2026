use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};
use std::net::IpAddr;
use rand::*;
use std::cmp::Ordering;

fn main() {
    // let v = vec![1,2,3];
    //
    // v[99];

    // let f = File::open("hello.txt");


    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Echec de l'ouverture de hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Erreur de création du fichier: {:?}", e),
    //         },
    //         autre_erreur => {
    //             panic!("Erreur d'ouverture du fichier : {:?}", autre_erreur)
    //         }
    //     }
    // };

    // Optimised algo

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Erreur de création du fichier: {:?}", error);
            })
        } else {
            panic!("Erreur d'ouverture du fichier : {:?}", error);
        }
    });

    fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(fichier) => fichier,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // Optimised Algo

    fn lire_pseudo_depuis_fichier_2() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    fn lire_pseudo_depuis_fichier_3(file_name: String) -> Result<String, io::Error> {
        let mut s = String::new();
        File::open(file_name)?.read_to_string(&mut s)?;
        Ok(s)
    }

    fn lire_pseudo_depuis_fichier_4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    let f = File::open("hello.txt");

    fn dernier_caractere_de_la_premiere_ligne(texte: &str) -> Option<char> {
        texte.lines().next()?.chars().last()
    }

    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // let nombre_secret = rand::thread_rng().gen_range(1..101);

    // loop {
    //
    //     let mut supposition = String::new();
    //
    //     let supposition: i32 = match supposition.trim().parse() {
    //         Ok(nombre) => nombre,
    //         Err(_) => continue,
    //     };
    //
    //     if supposition < 1 || supposition > 100 {
    //         println!("Le nombre secret est entre 1 et 100.");
    //         continue;
    //     }
    //
    //     match supposition.cmp(&nombre_secret) {
    //
    //     }
    // }

    pub struct Supposition {
        valeur: i32,
    }

    impl Supposition {
        pub fn new(valeur:i32) -> Supposition {
            if valeur < 1 || valeur > 100 {
                panic!("Supposition valeur doit être entre 1 et 100, obtenu {}.", valeur);
            }

            Supposition { valeur }
        }

        pub fn valeur(&self) -> i32 {
            self.valeur
        }
    }

}

