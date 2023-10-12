use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    sync::OnceLock,
};

const FILE: &str = include_str!("nam_dict.txt");
pub(crate) static NAMES: OnceLock<HashMap<String, String>> = OnceLock::new();
pub(crate) fn get_names() -> &'static HashMap<String, String> {
    NAMES.get_or_init(|| {
        BufReader::new(FILE.as_bytes())
            .lines()
            .flatten()
            .flat_map(|line| {
                if line.starts_with('#') {
                    return None;
                }
                let mut items = line.split_whitespace();
                let item0 = items.next()?;
                let item1 = items.next()?;
                Some((item1.to_lowercase(), item0.to_lowercase()))
            })
            .collect()
    })
}
