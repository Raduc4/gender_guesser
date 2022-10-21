use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type Names = HashMap<String, String>;

#[derive(Debug)]
pub enum Gender {
    Male,
    Famale,
    NotSure,
    MayBeFamale,
    MayBeMale,
    NotFound,
}
pub struct Detector {
    names: Names,
}

impl Detector {
    pub fn new() -> Self {
        Self {
            names: HashMap::default(),
        }
    }
    pub fn parse(&mut self, name_to_find: &str) -> () {
        let file = File::open("src/gender_guesser/data/nam_dict.txt").unwrap();
        let lines = BufReader::new(file).lines();

        for line in lines {
            let item = line.unwrap();
            Self::eat_the_line(self, &item);
        }
    }

    pub fn eat_the_line<'a>(&mut self, line: &str) -> () {
        if !line.starts_with("#") {
            let item = line.split(" ").collect::<Vec<&str>>();
            Self::set(self, item);
        };
    }

    pub fn set(&mut self, item_vec: Vec<&str>) -> () {
        self.names
            .insert(item_vec[1].to_string(), item_vec[0].to_owned());
    }
    pub fn get_gender(&mut self, name: &str) -> Gender {
        Self::parse(self, name);

        if self.names.contains_key(name) {
            Gender::Male
        } else {
            Gender::NotFound
        }
    }
}

#[cfg(test)]
mod tests_gender_guesser {
    use super::*;
    #[test]
    fn prints_the_line() {
        let mut d = Detector::new();
        println!("{:?}", d.get_gender("Enna"));
        println!("{:?}", d.names.capacity());
    }
}
