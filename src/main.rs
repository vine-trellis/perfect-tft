use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize, Clone, Hash)]
#[serde(rename_all = "camelCase")]
struct Champion {
    name: String,
    champion_id: String,
    cost: u8,
    traits: Vec<String>,
}
impl PartialEq for Champion {
    fn eq(&self, other: &Self) -> bool {
        self.champion_id == other.champion_id
    }
}
impl Eq for Champion {}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Set {
    min: Option<u8>,
    max: Option<u8>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Trait {
    key: String,
    name: String,
    r#type: String,
    sets: Vec<Set>,
}

fn read_json(path: &str) -> String {
    let path = Path::new(&path);
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                    why),
        Ok(_) => println!("successfully read {}", display),
    }
    s
}

fn main() {
    let champ_data = read_json("src/assets/champions.json");
    let champs: Vec<Champion> = serde_json::from_str(&champ_data).unwrap();
    println!("{}", champs.len());

    let trait_data = read_json("src/assets/traits.json");
    let r#traits: Vec<Trait> = serde_json::from_str(&trait_data).unwrap();
    println!("{}", traits.len());

    let champ_combos = champs.into_iter().combinations(4);
    let mut counter = 0;
    for champ_combo in champ_combos {
        counter = counter + 1;
    }
    println!("{}", counter);
}
