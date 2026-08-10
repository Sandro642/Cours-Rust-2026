use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (recherche, nom_fichier) = interpreter_config(&args);

    println!("On recherche : {}", recherche);
    println!("Dans le fichier : {}", nom_fichier);

    let contenu = fs::read_to_string(nom_fichier)
        .expect("Quelque chose s'est mal passé lors de la lecture du fichier");

    println!("Dans le texte :\n{}", contenu);
}

fn interpreter_config(args: &[String]) -> (&str, &str) {
    let recherche = &args[1];
    let nom_fichier = &args[2];

    (recherche, nom_fichier)
}
