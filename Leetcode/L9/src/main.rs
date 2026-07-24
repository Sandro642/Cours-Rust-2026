fn main() {

    println!("{:?}", is_palindrome(134))

}

pub fn is_palindrome(i: i32) -> bool {
    if reverse(i) == i {
        true
    } else {
        false
    }
}

pub fn reverse(i: i32) -> i32 {
    format!("{i}").chars().rev().collect::<String>().parse().unwrap_or(0)
}
