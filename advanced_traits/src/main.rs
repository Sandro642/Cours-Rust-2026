fn main() {}

pub trait Iterator<T> {
    type Item;

    fn next(&mut self) -> Option<T>;
}
