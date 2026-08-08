use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let recherche = &args[1];
    let nom_du_fichier = &args[2];

    println!("On recherche {}", recherche);
    println!("Dans le fichier {}", nom_du_fichier);

    let contenu = fs::read_to_string(nom_du_fichier)
        .expect("Quel que chose s'est mal passé durant la lecture du fichier.");

    println!("Dans le texte :\n {}", contenu);
}
