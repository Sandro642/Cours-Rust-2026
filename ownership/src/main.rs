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

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}");

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

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
