//! # Guessing Gender
//!
//! `get_gender()` - use this to get the gender  

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
include!(concat!(env!("OUT_DIR"), "/load_names.rs"));

impl Detector {
    pub fn new() -> Self {
        let names = &NAMES;
        let mut hmap: HashMap<String, String> = HashMap::new();
        for (u1, u2) in names.iter() {
            hmap.insert(u1.to_owned(), u2.to_owned());
        }
        Self { names: hmap }
    }

    pub fn get_gender(&mut self, name: &str) -> Gender {
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
