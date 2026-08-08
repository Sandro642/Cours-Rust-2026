use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is : {x}");
    x = x + 1;
    println!("The value of x is : {x}");

    const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;


    // Test
    let x = ONE_HOUR_IN_SECONDS;

    {
        let x = ONE_HOUR_IN_SECONDS * 10;

        println!("The value of x in inner scope is : {x}");
    }

    println!("The value of x is : {x}");

    {
        let space = "      ";
        let space = space.len();

        println!("The value of space is : {space}");
    }

    {
        let y: u16 = 256;

        let y: u64 = 10000000000000000000;

        let y: u128 = 100000000000000000000000000000000000000;

        println!("The value of y is {y}");
    }

    {
        let status: bool = false;

        if (!status) {
            println!("The value of status is false");
        }

        let status: bool = true;

        if (status) {
            println!("The value of status is true");
        }
    }

    {
        let tup: (i8, u32, f64) = (1, 1600, 2.0);

        let x = tup.0;
        println!("The value of x is: {x}");

        let tup = (1, 1600, 2.0);

        let (x, y, z) = tup;

        println!("{x}, {y}, {z}");
    }

    {
        let table = [4; 5];
        let table = table.len();

        println!("The value of table is : {table}");
    }

    println!("The value of ONE_HOUR_IN_SECONDS is {ONE_HOUR_IN_SECONDS}");


    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is : {element}");

    another_function();
}

fn another_function() {
    println!("I'm the new function rn");
}