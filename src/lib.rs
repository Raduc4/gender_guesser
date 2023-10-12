//! # Guessing Gender
//!
//! `get_gender()` - use this to get the gender  

mod load_names;

use std::{collections::HashMap, marker::PhantomData};

use load_names::get_names;

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
/// let d = Detector::new();
/// d.get_gender("name");
///```
#[derive(Debug, Clone, Copy)]
pub struct Detector(PhantomData<Names>);

impl Default for Detector {
    fn default() -> Self {
        Self::new()
    }
}
impl Detector {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
    fn names(&self) -> &'static Names {
        get_names()
    }
    pub fn get_gender(&self, name: &str) -> Gender {
        self.names()
            .get(name.to_lowercase().as_str())
            .map(|letter| match letter.as_str() {
                "m" => Gender::Male,
                "f" => Gender::Female,
                "?m" => Gender::MayBeMale,
                "?f" => Gender::MayBeFemale,
                "=" => Gender::BothMaleFemale,
                _ => Gender::NotSure,
            })
            .unwrap_or(Gender::NotFound)
    }
}

#[cfg(test)]
mod tests_gender_guesser {
    use super::*;
    #[test]
    fn display_male() {
        let d = Detector::new();
        assert_eq!(Gender::Male, d.get_gender("Radu"));
    }

    #[test]
    fn display_female() {
        let d = Detector::new();
        assert_eq!(Gender::Female, d.get_gender("Ana"));
    }

    #[test]
    fn display_female_oposite() {
        let d = Detector::new();
        assert_ne!(Gender::Female, d.get_gender("Radu"));
    }

    #[test]
    fn display_male_oposite() {
        let d = Detector::new();
        assert_ne!(Gender::Male, d.get_gender("Ana"));
    }

    #[test]
    fn display_may_be_male() {
        let d = Detector::new();
        assert_eq!(Gender::MayBeMale, d.get_gender("Adair"));
    }
    #[test]
    fn display_may_be_female() {
        let d = Detector::new();
        assert_eq!(Gender::MayBeFemale, d.get_gender("Aaf"));
    }

    #[test]
    fn display_not_found() {
        let d = Detector::new();
        assert_eq!(Gender::NotFound, d.get_gender("Adadadasdadasd"));
    }
    #[test]
    fn display_not_sure() {
        let d = Detector::new();
        assert_eq!(Gender::NotSure, d.get_gender("Abbe"));
    }
}
