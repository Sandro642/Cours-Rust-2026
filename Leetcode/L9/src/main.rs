fn main() {

    println!("{:?}", is_palindrome(134))

}

pub fn is_palindrome(i: i32) -> bool {
    if format!("{i}").chars().rev().collect::<String>().parse().unwrap_or(0) == i {
        true
    } else {
        false
    }
}
