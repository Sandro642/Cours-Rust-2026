fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1,2,3];

    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    //

    // mod another {
    //     fn chapter_4() {
    //         let mut v = vec![1,2,3,4,5];
    //         let first = &v[0];
    //
    //         v.push(6);
    //
    //         println!("The first element is {first}")
    //     }
    // }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    mod spreadsheet {
        pub enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        fn row_fn() {
            let row = vec![
                SpreadsheetCell::Int(3),
                SpreadsheetCell::Text(String::from("Blue")),
                SpreadsheetCell::Float(10.12),
            ];
        }
    }

    //

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // The same

    let s = "Initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");

    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l'); // Only a char

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    let s2 = String::from("Tout le monde!");
    let s4 = format!("{s1} {s2}");
    println!("{s2}");

    //

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // let s1 = String::from("hi");
    // let h = s1[0];

    let hello = String::from("Hola");
    let s: &str = &hello[..2];
    println!("{s}");


    for c in hello.bytes() {
        println!("{c}");
    }
}
