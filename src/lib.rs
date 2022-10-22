//! # Guessing Gender
//!
//! `get_gender` - main method

use std::{
    collections::HashMap,
    env,
    fs::File,
    include,
    io::{BufRead, BufReader},
    path::Path,
};

type Names = HashMap<String, String>;

/// # Possible results
#[derive(Debug, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
    MayBeMale,
    MayBeFemale,
    BothMaleFemale,
    NotSure,
    NotFound,
}
/// Implementation struct for guessing the gender
///
/// # Examples
///```
/// use gender_guesser::Detector;
///
/// let mut d = Detector::new();
/// d.get_gender("name");
///```
pub struct Detector {
    names: Names,
}

impl Detector {
    pub fn new() -> Self {
        Self {
            names: HashMap::default(),
        }
    }
    fn parse(&mut self, name_to_find: &str) -> () {
        let p = env::current_dir().unwrap();
        println!("{}", std::env::var("OUT_DIR").unwrap());
        println!("{}", p.display());
        let file = File::open(format!("{}/src/nam_dict.txt", p.display())).unwrap();
        let lines = BufReader::new(file).lines();

        for line in lines {
            let item = line.unwrap();
            Self::eat_the_line(self, &item);
        }
    }

    fn eat_the_line<'a>(&mut self, line: &str) -> () {
        if !line.starts_with("#") {
            let item = line.split_whitespace().collect::<Vec<&str>>();
            Self::set(self, item);
        };
    }

    fn set(&mut self, item_vec: Vec<&str>) -> () {
        self.names
            .insert(item_vec[1].to_string(), item_vec[0].to_owned());
    }
    pub fn get_gender(&mut self, name: &str) -> Gender {
        Self::parse(self, name);

        if self.names.contains_key(name) {
            let letter = self.names.get(name).unwrap();

            match letter {
                letter if letter == "M" => Gender::Male,
                letter if letter == "F" => Gender::Female,
                letter if letter == "?M" => Gender::MayBeMale,
                letter if letter == "?F" => Gender::MayBeFemale,
                _ => Gender::NotSure,
            }
        } else {
            Gender::NotFound
        }
    }
}

#[cfg(test)]
mod tests_gender_guesser {
    use super::*;
    #[test]
    fn display_male() {
        let mut d = Detector::new();
        assert_eq!(Gender::Male, d.get_gender("Radu"));
    }

    #[test]
    fn display_female() {
        let mut d = Detector::new();
        assert_eq!(Gender::Female, d.get_gender("Ana"));
    }

    #[test]
    fn display_female_oposite() {
        let mut d = Detector::new();
        assert_ne!(Gender::Female, d.get_gender("Radu"));
    }

    #[test]
    fn display_male_oposite() {
        let mut d = Detector::new();
        assert_ne!(Gender::Male, d.get_gender("Ana"));
    }

    #[test]
    fn display_may_be_male() {
        let mut d = Detector::new();
        assert_eq!(Gender::MayBeMale, d.get_gender("Adair"));
    }
    #[test]
    fn display_may_be_female() {
        let mut d = Detector::new();
        assert_eq!(Gender::MayBeFemale, d.get_gender("Aaf"));
    }

    #[test]
    fn display_not_found() {
        let mut d = Detector::new();
        assert_eq!(Gender::NotFound, d.get_gender("Adadadasdadasd"));
    }
    #[test]
    fn display_not_sure() {
        let mut d = Detector::new();
        assert_eq!(Gender::NotSure, d.get_gender("Abbe"));
    }
}
