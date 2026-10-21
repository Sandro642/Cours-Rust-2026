fn main() {
    println!("Hello, world!");
    let b = Box::new(5);

    println!("b = {}", b);
}

enum List {
    Cons(i32, List),
    Nil,
}

enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}
