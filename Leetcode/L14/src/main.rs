fn main() {
    let n = String::from("Hello everyone");
    println!("{:?}", hello());

                println!("{}", n);
}


pub fn hello() -> String {
    return String::from("Hello everyone");
}
