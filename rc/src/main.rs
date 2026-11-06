enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Compteur après la création de a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("Compteur après la création de b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("Compteur après la création de c = {}", Rc::strong_count(&a));
    }

    println!(
        "Compteur après que c'est sorti de la portée {}",
        Rc::strong_count(&a)
    );
}
