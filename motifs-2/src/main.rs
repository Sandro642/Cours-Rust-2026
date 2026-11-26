fn main() {
    let msg = Message::ChangerCouleur(Couleur::Tsv(0, 160, 255));

    match msg {
        Message::ChangerCouleur(Couleur::Rvb(r, v, b,)) => println!(
            "Changement des taux de rouge à {}, de vert à {} et de bleu {}",
            r, v, b
        ),
        Message::ChangerCouleur(Couleur::Tsv(t, s, v)) => println!(
            "Changement des taux de teinte à {}, de saturation à {} et de valeur à {}",
            t, s, v
        ),
        _ => (),
    }

    fonction(3, 4);

    let mut valeur_du_reglage = Some(5);
    let nouvelle_valeur_du_reglage = Some(10);

    match (valeur_du_reglage, nouvelle_valeur_du_reglage) {
        (Some(_), Some(_)) => {
            println!("Vous ne pouvez pas écraser une valeur déjà existante");
        }
        _ => {
            valeur_du_reglage = nouvelle_valeur_du_reglage;
        }
    }

    println!("Le réglage vaut {:?}", valeur_du_reglage);

    let nombres = (2,4,8,16,32);

    match nombres {
        (premier, _, troisieme, _, cinquieme) => {
            println!("Voici quelques nombres : {}, {}, {}", premier, troisieme, cinquieme);
        }
    }

    let s = Some(String::from("Salutations"));

    if let Some(_) = s {
        println!("J'ai trouvé une chaine de caractères");
    }

    println!("{:?}", s);

    let origine = Point { x:0, y:0, z:0 };

    match origine {
        Point { x, .. } => println!("x vaut {}", x),
    }

    match nombres {
        (premier, .., dernier) => {
            println!("Voici quelques nombres : {}, {}", premier, dernier);
        }
    }

    let nombre = Some(4);

    match nombre {
        Some(x) if x % 2 == 0 => println!("Le nombre {} est pair", x),
        Some(x) => println!("Le nombre {} est impair", x),
        None => (),
    }

    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Nous obtenons 50"),
        Some(n) if n == y => println!("Nous avons une correspondance, n = {}", n),
        _ => println!("Cas par défaut, x = {:?}", x),
    }

    println!("Au final : x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = Message::Hello { id: 5 };

    // match msg {
    //     Message::Hello {
    //         id: id_variable @ 3..=7,
    //     } => println!("Nous avons trouvé un id dans l'intervalle : {}", id_variable),
    //     Message::Hello { id : 10..=12 } => {
    //         println!("Nous avons trouvé un id dans un autre intervalle")
    //     }
    //     Message::Hello { id } => println!("Nous avons trouvé un autre id : {}", id),
    // }

}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn fonction(_: i32, y: i32) {
    println!("Ce code utilise uniquement le paramètre y : {}", y);
}

enum Couleur {
    Rvb(i32, i32, i32),
    Tsv(i32, i32, i32),
}

enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(Couleur),
    Hello { id: i32 },
}