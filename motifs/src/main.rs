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


    // fn fonction(x:i32) {
    //     todo!()
    // }

    // fonction(12);


    let point = (3,5);
    afficher_coordonnees(&point);

    let x = 1;

    match x {
        1 => println!("un"),
        2 => println!("deux"),
        3 => println!("trois"),
        _ => println!("n'importe quoi")
    }

    let x = Some(5);
    let _y = 10;

    match x {
        Some(50) => println!("On a 50"),
        Some(y) => println!("Correspondance, y = {:?}", y),
        _ => println!("Cas par défaut, x = {:?}", x),
    }

    let x = 1;

    match x {
        1 | 2 => println!("1 ou 2"),
        3 => println!("3"),
        _ => println!("quelque chose d'autre"),
    }

    let x = 5;

    match x {
        1..5 => println!("de un à cinq"),
        _ => println!("quelque chose d'autre"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("lettre ASCII du début"),
        'k'..='z' => println!("Lettre ASCII de la fin"),
        _ => println!("autre chose"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    println!("A la fin : x = {:?}, y = {:?}", x, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0} => println!("Sur l'axe x à la position {}", x),
        Point { x: 0, y} => println!("Sur l'axe à la position {}", y),
        Point {x, y} => println!("Sur aucun des axes : ({}, {})", x, y),
    }



    let msg = Message::ChangerCouleur(0, 160, 255);

    match msg {
        Message::Quitter => {
            println!("La variante Quitter n'a pas de données à déstructurer")
        }
        Message::Deplacer { x, y} => {
            println!(
                "Déplacement de {} sur l'axe x et de {} sur l'axe y",
                x, y
            )
        }
        Message::Ecrire(text) => println!("Message textuel : {}", text),
        Message::ChangerCouleur(r, g, b) => println!(
            "Changement des taux de rouge à {}, de vert à {} et de bleu à {}",
            r, g, b
        )
    }

    let ((pieds, pouces), Point {x, y}) = ((3, 10), Point {x: 3, y: -10});

    
}
fn afficher_coordonnees(&(x,y): &(i32, i32)) {
    println!("Coordonnées actuelles: ({}, {})", x, y);
}

enum Message {
    Quitter,
    Deplacer { x: i32, y: i32},
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}
