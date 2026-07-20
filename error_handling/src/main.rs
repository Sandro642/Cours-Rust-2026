use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let v = vec![1,2,3];
    //
    // v[99];

    // let f = File::open("hello.txt");

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
}

