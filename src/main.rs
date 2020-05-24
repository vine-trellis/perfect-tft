use std::path::Path;
use std::fs::File;
use std::fmt;
use std::io::prelude::*;
use std::collections::HashMap;
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
impl fmt::Display for Champion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}", self.name)
    }
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Set {
    min: u8,
    // max: Option<u8>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Trait {
    key: String,
    name: String,
    r#type: String,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Team {
    champions: Vec<Champion>,
}

impl Team {
    fn get_traits(&self) -> HashMap<&String, u8> {
        let mut traits = HashMap::new();
        for champion in &self.champions {
            for r#trait in &champion.traits {
                traits.insert(r#trait, match traits.get(&r#trait) {
                    Some(x) => x + 1u8,
                    None => 1u8,
                }
            );
            }
        }
        traits
    }
}
// impl fmt::Display for Team {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut repr = String::from("");
//         for champion in self.champions {
//             repr.push_str(format!("{}", champion.name));
//         }
//         write!("{}",repr)
//     }
// }

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
        Ok(_) => {}
        // println!("successfully read {}", display),
    }
    s
}

fn check_perfect(r#traits: &HashMap<&String, &Trait>, team: &Team ) -> bool {
    let team_traits = team.get_traits();
    for key in team_traits.keys() {
        let mut mins: Vec<u8> = Vec::new();
        let _dumb_none: Vec<Set> = Vec::new();
        for set in match r#traits.get(key) {
            Some(x) => &x.sets,
            None => &_dumb_none
        }  
        {
            mins.push(set.min);
        }
        if !mins.contains(match team_traits.get(key)  {
            Some(x) => x,
            None => &0u8,
        }){
            return false
        }
    }
    true
}

fn main() {
    for i in 1..10 {
        println!("{}", i);
        let champ_data = read_json("src/assets/champions.json");
        let champs: Vec<Champion> = serde_json::from_str(&champ_data).unwrap();
        // println!("{}", champs.len());

        let trait_data = read_json("src/assets/traits.json");
        let trait_vec: Vec<Trait> = serde_json::from_str(&trait_data).unwrap();
        let mut trait_map: HashMap<&String, &Trait> = HashMap::new();
        for r#trait in &trait_vec {
            trait_map.insert(&r#trait.name, r#trait);
        }

        let mut champ_combos = champs.into_iter().combinations(i);
        // let mut teams: Vec<Team> = Vec::new();
        loop {
            let champ_combo = champ_combos.next();
            match champ_combo {
                Some(x) => {
                    let team: Team = Team {champions : x};
                    if check_perfect(&trait_map, &team) {
                        println!("{:?}", &team);
                    }
                },
                None => {
                    break
                }
            }

        }
        // for champ_combo in champ_combos {
        //     let team: Team = Team {champions : champ_combo};
        //     teams.push(team);
        // }
        // for team in &teams {
        //     if check_perfect(&trait_map, team) {
        //         println!("{:?}", team);
        //     }
        // }
    }
}
