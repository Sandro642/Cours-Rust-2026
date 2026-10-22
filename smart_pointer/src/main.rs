use std::ops::Deref;

use crate::List::{Cons, Nil};

fn main() {
    println!("Hello, world!");
    let b = Box::new(5);

    println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MaBoite::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));

    let m = MaBoite::new(String::from("Rust"));
    saluer(&m);
    saluer(&(*m)[..]);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}

struct MaBoite<T>(T);

impl<T> MaBoite<T> {
    fn new(x: T) -> MaBoite<T> {
        MaBoite(x)
    }
}

impl<T> Deref for MaBoite<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn saluer(nom: &str) {
    println!("Salutations, {}", nom);
}
