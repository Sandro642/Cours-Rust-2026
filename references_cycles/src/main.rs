use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn parcourir(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// Part 2:

#[derive(Debug)]
struct Noeud {
    valeur: i32,
    parent: RefCell<Weak<Noeud>>,
    enfants: RefCell<Vec<Rc<Noeud>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("compteur initial de a = {}", Rc::strong_count(&a));
    println!("prochain élément de a = {:?}", a.parcourir());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("compteur de a après création de b = {}", Rc::strong_count(&a));
    println!("compteur intial de b = {}", Rc::strong_count(&b));
    println !("prochain élément de b = {:?}", b.parcourir());

    if let Some(lien) = a.parcourir()  {
        *lien.borrow_mut() = Rc::clone(&b);
    }

    println!("compteur de b après avoir changé a = {}", Rc::strong_count(&b));
    println!("compteur de a après avoir changé a = {}", Rc::strong_count(&a));


    // Part 2:

    let feuille = Rc::new(Noeud {
        valeur : 3,
        parent: RefCell::new(Weak::new()),
        enfants: RefCell::new(vec![]),
    });

    println!("Feuille strong = {}, weak = {}",
    Rc::strong_count(&feuille),
    Rc::weak_count(&feuille),
    );

    {
        let branche = Rc::new(Noeud {
            valeur: 5,
            parent: RefCell::new(Weak::new()),
            enfants: RefCell::new(vec![Rc::clone(&feuille)]),
        });

        *feuille.parent.borrow_mut() = Rc::downgrade(&branche);

        println!("branche strong = {}, weak = {}",
        Rc::strong_count(&branche),
        Rc::weak_count(&branche),
        );

        println!("feuille strong = {}, weak = {}",
        Rc::strong_count(&feuille),
        Rc::weak_count(&feuille),
        );
    }

    println!("parent de la feuille = {:?}", feuille.parent.borrow().upgrade());
    println!("feuille strong = {}, weak = {}",
    Rc::strong_count(&feuille),
    Rc::weak_count(&feuille),
    );
}