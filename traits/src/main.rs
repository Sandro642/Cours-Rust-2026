use std::fmt::{Debug, Display};
use traits::{ArticleDePresse, Resumable, Tweet};
fn main() {
    let tweet = Tweet {
        username: String::from("Jean"),
        contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
        reponse: false,
        retweet: false,
    };

    println!("1 Nouveau tweet : {}", tweet.resumer());

    let article = ArticleDePresse {
        titre: String::from("Les Penguins ont remporté la Coupe Stanley !"),
        lieu: String::from("Pittsburgh, PA, USA"),
        auteur: String::from("Iceburgh"),
        contenu: String::from(
            "Les Penguins de Pittsburgh sont une nouvelle fois la meilleure \
            équipe de hockey de la LNH.",
        ),
    };

    println!("Nouvelle article disponible ! {}", article.resumer());
}

pub fn notifier(element: &impl Resumable) {
    println!("Flash info ! {}", element.resumer());
}

pub fn notifier_2<T: Resumable>(element: &T) {
    println!("Flash info ! {}", element.resumer());
}

pub fn notifier_3<T: Resumable>(element1: &T, element2: &T) {}

pub fn notifier_4(element: &(impl Resumable + Display)) {}

pub fn notifier_5<T: Resumable + Display>(element: &T) {}

//

fn une_fonction<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {0}

fn une_fonction_2<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
      U: Clone + Display
{0}

//

fn retourne_resumable() -> impl Resumable {
    Tweet {
        username: String::from("Jean"),
        contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
        reponse: false,
        retweet: false,
    }
}

// fn retourne_resumable(estArticle: bool) -> impl Resumable {
//     if estArticle {
//         ArticleDePresse {
//             titre: String::from("Les Penguins ont remporté la Coupe Stanley !"),
//             lieu: String::from("Pittsburgh, PA, USA"),
//             auteur: String::from("Iceburgh"),
//             contenu: String::from(
//                 "Les Penguins de Pittsburgh sont une nouvelle fois la meilleure \
//             équipe de hockey de la LNH.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("Jean"),
//             contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
//             reponse: false,
//             retweet: false,
//         }
//     }
// } Not possible for the moment.

