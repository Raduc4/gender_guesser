// https://discord.com/channels/442252698964721669/448238009733742612/1033324112132710481
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("load_names.rs");
    let contents = r##"use once_cell::sync::Lazy;

pub static NAMES: Lazy<Vec<(String, String)>> = Lazy::new(|| {
let mut vec_of_items:Vec<(String, String)> = Vec::new();
let p = env::current_dir().unwrap();
let file = File::open(format!("{}/src/nam_dict.txt", p.display())).unwrap();
let lines = BufReader::new(file).lines();

for line in lines {
  let line = line.unwrap(); 
    if !line.starts_with("#") {
        let item_vec = line.split_whitespace().collect::<Vec<&str>>();
        vec_of_items.push((item_vec[1].clone().to_string(), item_vec[0].clone().to_string()));
      };
    }
  vec_of_items
});"##;
    fs::write(&dest_path, contents).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
