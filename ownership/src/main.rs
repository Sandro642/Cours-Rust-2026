mod main2;

fn main() {
    let s = String::from("Hello");

    println!("{s}");

    let x = 5;
    let y = x;

    println!("x = {x} and y = {y}");
    
    //let s1 = String::from("Hello");
    //let s2 = s1;

    //println!("Test {s1}");

    let s = String::from("Hello");

    //s = String::from("ahoy");
    let s3 = s.clone();

    println!("{s3}, world!");

    //

    let s = String::from("Hello");

    takes_ownership(s);

    //println!("Where is s ? : {s}");

    let x = 5;

    makes_copy(x);

    println!("Where is x ? : {x}, How, i can use it again ??");


    //

    let s1 = gives_ownership();

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);


    //

    let s1 = String::from("Hello");

    //let (s2, len) = calculate_length(s1);

    //println!("The length of '{s2}' is {len}");


    //

    let s1 = String::from("Hello");

    //let len = calculate_length(&s1);

    //println!("The length of {s1} is {len}");

    //change(&s);


    //

    let mut s = String::from("Hello");

    change(&mut s);
    {
        let r1 = &mut s;

        println!("{r1}");
    }
    let r2 = &mut s;

    println!("{r2}");

    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");


    //

    let reference_to_nothing = dangle();


    //


    let mut s = String::from("hello world");
    let word = first_word(&s);

    s.clear();


    //

    let s = String::from("hello world");

    let length = s.len();

    let hello = &s[..5];
    let world = &s[6..]; // Ou
    let world = &s[6..length];

    //let (first_word, index) = first_word_updated(&s);

    //s.clear();

    //println!("The first word of '{s}' is '{first_word}' and this is the index '{index}'");

    let my_string = String::from("Hello world");

    let word = first_word_updated(&my_string[0..6]);
    let word = first_word_updated(&my_string[..]);

    let word = first_word_updated(&my_string);

    let my_string_literal = "Hello world";

    let word = first_word_updated(&my_string_literal[0..6]);
    let word = first_word_updated(&my_string_literal[..]);

    let word = first_word_updated(&my_string_literal);


    //

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word_updated(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn dangle() -> String {
    let s = String::from("Hello");

    s
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}




// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//
//     (s, length)
// }

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
