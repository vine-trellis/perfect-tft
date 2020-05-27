use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, Hash)]
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
impl fmt::Display for Champion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Set {
    min: u8,
    // max: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Trait {
    key: String,
    name: String,
    r#type: String,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Team<'a> {
    champions: &'a Vec<&'a Champion>,
    traits: HashMap<&'a String, u8>,
}

impl<'a> Team<'a> {
    fn get_traits(champions: &'a Vec<&'a Champion>) -> HashMap<&'a String, u8> {
        let mut traits = HashMap::new();
        for champion in champions {
            for r#trait in &champion.traits {
                traits.insert(
                    r#trait,
                    match traits.get(&r#trait) {
                        Some(x) => x + 1u8,
                        None => 1u8,
                    },
                );
            }
        }
        traits
    }
    pub fn from_champions(champions: &'a Vec<&'a Champion>) -> Team<'a> {
        Team {
            champions: champions,
            traits: Team::get_traits(champions),
        }
    }
}
impl fmt::Display for Team<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let team_repr = self
            .champions
            .iter()
            .map(|champion| format!("{} ({})", &champion.name, &champion.cost.to_string()))
            .join(", ");
        let trait_repr = self
            .traits
            .iter()
            .map(|(k, v)| format!("{} -> {}", k, &v.to_string()))
            .join(", ");
        write!(f, "Champions: {} | Traits: {}", team_repr, trait_repr)
    }
}

fn read_json(path: &str) -> String {
    let path = Path::new(&path);
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }
    s
}

fn check_perfect(r#traits: &HashMap<&String, &Trait>, team: &Team) -> bool {
    for key in team.traits.keys() {
        let mut mins: Vec<u8> = Vec::new();
        let _dumb_none: Vec<Set> = Vec::new();
        for set in match r#traits.get(key) {
            Some(x) => &x.sets,
            None => &_dumb_none,
        } {
            mins.push(set.min);
        }
        if !mins.contains(match team.traits.get(key) {
            Some(x) => x,
            None => &0u8,
        }) {
            return false;
        }
    }
    true
}

fn main() {
    // setup
    let champ_data = read_json("src/assets/champions.json");
    let champs: &Vec<Champion> = &serde_json::from_str(&champ_data).unwrap();

    let trait_data = read_json("src/assets/traits.json");
    let trait_vec: Vec<Trait> = serde_json::from_str(&trait_data).unwrap();
    let trait_map: HashMap<&String, &Trait> =
        trait_vec.iter().map(|x| (&x.name, x)).into_iter().collect();

    // enumeration
    for i in 1..=4 {
        println!("Level {} perfect comps:", i);
        let mut champ_combos = champs.into_iter().combinations(i);

        loop {
            let champ_combo = champ_combos.next();
            match champ_combo {
                Some(x) => {
                    let team: Team = Team::from_champions(&x);
                    if check_perfect(&trait_map, &team) {
                        println!("{}", &team);
                    }
                }
                None => break,
            }
        }
    }
}
