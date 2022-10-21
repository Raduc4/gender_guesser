use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

type Names = HashMap<String, String>;

#[derive(Debug)]
enum Gender {
    Male,
    Famale,
    NotSure,
    MayBeFamale,
    MayBeMale,
    NotFound,
}
pub struct Detector {
    case_sensitive: bool,
    names: Names,
}

impl Detector {
    pub fn new(case_sensitive: bool) -> Self {
        Self {
            case_sensitive,
            names: HashMap::default(),
        }
    }
    fn parse(self, name_to_find: &str) -> Gender {
        let file = File::open("src/gender_guesser/data/nam_dict.txt").unwrap();
        let lines = io::BufReader::new(file);

        // lines.lines().into_iter().for_each(|item| {
        //     let line = item.unwrap();

        //     match Self::find_name(&line, name_to_find) {
        //         Some(x) => x,
        //         _ => {}
        //     }
        // });
        lines.lines().into_iter().filter(|item| {
            let line: Vec<&str> = item.unwrap().split(" ").collect();
            line[1] == name_to_find
        });
        // println!("{x:?}");
        Gender::Famale
    }
    // fn find_name<'a>(lines: BufReader<File>, name_to_find: &str) -> Option<(&'a str, &'a str)> {
    //     let somet = lines.lines().for_each(|item| {
    //         let x = match item.unwrap().split(" ").collect::<Vec<&str>>() {
    //             sex if sex[1] == name_to_find => Some((sex[0], sex[1])),
    //             _ => None,
    //         };
    //         x
    //     });
    // if !line.starts_with("#") {
    // Some(("mama", "Mama"))
    //     let x = line.split(" ").collect::<Vec<&str>>();
    //     match x {
    //         name if name[1] == neme_to_find => Some((name[0], name[1])),
    //         _ => None,
    //     }
    // } else {
    //     None
    // }

    fn get_gender(self, name: &str) -> Gender {
        Self::parse(self, name)
    }
}

#[cfg(test)]
mod tests_gender_guesser {
    use super::*;
    #[test]
    fn prints_the_line() {
        let d = Detector::new(false);
        println!("{:?}", d.parse("Cristian"));
    }
}
