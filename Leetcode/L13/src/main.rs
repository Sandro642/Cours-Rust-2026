fn main() {
    println!("Hello, world!");

    println!("{:?}", roman_to_int(String::from("MCMXCIV")));
}

pub fn roman_to_int(s: String) -> i32 {
    let mut value = 0;
    let mut prev = 0;

    for ch in s.chars() {
        match ch {
            'I' => { value += 1; prev = 1;},
            'V' => { value += if prev == 1 { 3 } else { 5 }; prev = 5;},
            'X' => { value += if prev == 1 { 8 } else { 10 }; prev = 10;},
            'L' => { value += if prev == 10 { 30 } else { 50 }; prev = 50;},
            'C' => { value += if prev == 10 { 80 } else { 100 }; prev = 100;},
            'D' => { value += if prev == 100 { 300 } else { 500 }; prev = 500;},
            'M' => { value += if prev == 100 { 800 } else { 1000 }; prev = 1000;},
            _ => {}
        }
    }
    value
}
