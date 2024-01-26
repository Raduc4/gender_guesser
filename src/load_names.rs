use std::{
    collections::HashMap,
    io::BufRead,
    sync::OnceLock,
};

const FILE: &[u8; 4493536] = include_bytes!("nam_dict.txt");
pub(crate) static NAMES: OnceLock<HashMap<String, String>> = OnceLock::new();
pub(crate) fn get_names() -> &'static HashMap<String, String> {
    NAMES.get_or_init(|| {
        FILE
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
