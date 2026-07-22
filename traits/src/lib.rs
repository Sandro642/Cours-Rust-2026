use std::fmt::Display;

pub trait Resumable {
    fn resumer(&self) -> String {
        format!("(Lire plus d'éléments de {} ...)", self.resumer_auteur())
    }

    fn resumer_auteur(&self) -> String;
}

pub struct ArticleDePresse {
    pub titre: String,
    pub lieu: String,
    pub auteur: String,
    pub contenu: String,
}

impl Resumable for ArticleDePresse {
    // fn resumer(&self) -> String {
    //     //format!("{}, par {} ({})", self.titre, self.auteur, self.lieu)
    //     todo!()
    // }

    fn resumer_auteur(&self) -> String {
        todo!()
    }
}

pub struct Tweet {
    pub username: String,
    pub contenu: String,
    pub reponse: bool,
    pub retweet: bool,
}

impl Resumable for Tweet {
    // fn resumer(&self) -> String {
    //     format!("{} : {}", self.username, self.contenu)
    // }

    fn resumer_auteur(&self) -> String {
        format!("@{}", self.username)
    }
}

struct Paire<T> {
    x: T,
    y: T,
}

impl<T> Paire<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Paire<T> {
    fn afficher_comparaison(&self) {
        if self.x >= self.y {
            println!("Le plus grand élément est x = {}", self.x);
        } else {
            println!("Le plus grand élément est y = {}", self.y);
        }
    }
}

fn test() {
    let s = 3.to_string();
}

// Example //

// impl<T: Display> ToString for T {
//
// }