fn main() {
    let couleur_favorite: Option<&str> = None;
    let on_est_mardi = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(couleur) = couleur_favorite {
        println!("Utilisaton de votre couleur favorite, {}, comme couleur de fond", couleur);
    } else if on_est_mardi {
        println!("Mardi, c'est le jour du vert");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Utilisaton du violet comme couleur de fond");
        } else {
            println!("Utilisation de l'orange comme couleur de fond");
        }
    } else {
        println!("Utilisation du bleu comme couleur de fond");
    }

    let mut pile: Vec<u32> = Vec::new();
    pile.push(1);
    pile.push(2);
    pile.push(3);

    while let Some(donnee_du_haut) = pile.pop() {
        println!("{}", donnee_du_haut);
    }

    let v = vec!['a', 'b', 'c'];

    for (indice, valeur) in v.iter().enumerate() {
        println!("{} est à l'indice {}", valeur, indice);
    }

    let _x = 5;

    let (_x,_y,_z) = (1,2,3);


    fn fonction(x:i32) {
        todo!()
    }

    fonction(12);


    let point = (3,5);
    afficher_coordonnees(&point);
}

fn afficher_coordonnees(&(x,y): &(i32, i32)) {
    println!("Coordonnées actuelles: ({}, {})", x, y);
}
