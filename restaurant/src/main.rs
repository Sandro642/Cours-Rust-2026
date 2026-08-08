use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IOResult;
use rand::Rng;

//use std::{cmp::Ordering, io};
use std::io::{self, Write};
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..= 100);
}

// fn function1() -> Result {}
//
// fn function2() -> IOResult<()> {}
