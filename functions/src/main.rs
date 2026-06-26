fn main() {
    println!("Hello, world!");

    another_function(12, 'c');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is : {y}");

    let x = five();

    println!("The value of x is: {x}");

    let a: i32 = another_plus_one(5);

    println!("5 + 1 is : {a} ");
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: char) {
    println!("Hello from here : {x} and {y}");
}

fn another_plus_one(y: i32) -> i32 {
    y + 1
}
