pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod export {
    pub fn addition_interne(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn ajouter_deux(a: i32) -> i32 {
        addition_interne(a, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn interne() {
        assert_eq!(4, export::addition_interne(2, 2));
    }
}
