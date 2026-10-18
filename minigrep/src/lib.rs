use std::{env, result};
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenu = fs::read_to_string(config.nom_fichier)?;

    let results = if config.ignorer_majuscules {
        search_case_insensitivity(&config.recherche, &contenu)
    } else {
        rechercher(&config.recherche, &contenu)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub recherche: String,
    pub nom_fichier: String,
    pub ignorer_majuscules: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let recherche = match args.next() {
            Some(arg) => arg,
            None => return Err("Nous n'avons pas de chaîne de caractères."),
        };

        let nom_fichier = match args.next() {
            Some(arg) => arg,
            None => return Err("Nous n'avons pas de nom de fichier."),
        };

        let ignorer_majuscules = env::var("MINIGREP_INSENSIBLE_CASSE").is_err();

        Ok(Config {
            recherche,
            nom_fichier,
            ignorer_majuscules,
        })
    }
}

pub fn search_case_insensitivity<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    contenu
        .lines()
        .filter(|ligne| ligne.to_lowercase().contains(recherche))
        .collect()
}

pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    contenu
        .lines()
        .filter(|ligne| ligne.contains(recherche))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_resultat() {
        let recherche = "duct";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.";

        assert_eq!(
            vec!["sécurité, rapidité, productivité."],
            rechercher(recherche, contenu)
        );
    }

    #[test]
    fn case_insensitivity() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitivity(query, content)
        );
    }
}
