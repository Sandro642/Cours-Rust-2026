use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    {
        let r;

        {
            let x = 5;
            r = &x;


            println!("r: {r}");
        }
    }

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";


        let resultat = la_plus_longue(string1.as_str(), string2);

        println!("La plus grande chaîne est {}", resultat)
    }

    {
        // fn la_plus_longue<'a>(x: &'a str, y: &str) -> &'a str {
        //     let resultat = String::from("Très longue chaîne");
        //     resultat.as_str()
        // }
    }

    struct ExtraitImportant<'a> {
        partie: &'a str,
    }

    impl<'a> ExtraitImportant<'a> {
        fn niveau(&self) -> i32 {
            3
        }

        fn annoncer_et_retourner_partie(&self, annonce: &str) -> &str {
            println!("Vottre attention s'il vous plaît : {}", annonce);
            self.partie
        }
    }

    let roman = String::from("Appellez-moi Ismaël. Il y a quelques années...");
    let premiere_phrase = roman.split('.')
        .next()
        .unwrap();

    let i = ExtraitImportant { partie: premiere_phrase };

    {
        fn premier_mot<'a>(s: &'a str) -> &'a str {
            let octets = s.as_bytes();

            for (i, &element) in octets.iter().enumerate() {
                if element == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
    }

    {
        let s: &'static str = "j'ai une durée de vie statique.";
    }
}

fn la_plus_longue<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn la_plus_longue_avec_annonce<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Annonce : {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}