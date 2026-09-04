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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Il n'y a pas assez d'arguments.");
        }

        let recherche = args[1].clone();
        let nom_fichier = args[2].clone();

        let ignorer_majuscules = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            recherche,
            nom_fichier,
            ignorer_majuscules,
        })
    }
}

pub fn search_case_insensitivity<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    let mut resultats = Vec::new();

    for ligne in contenu.lines() {
        if ligne.contains(recherche) {
            resultats.push(ligne);
        }
    }
    resultats
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
        )
    }
}
