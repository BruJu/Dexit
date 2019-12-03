use crate::pokemon::Pokemon;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

mod pokemon;

enum KindOfMove {
    None,
    LevelUp,
    EggMove,
    TM,
    TR
}

impl KindOfMove {
    fn is_none(&self) -> bool {
        match *self {
            KindOfMove::None => true,
            _ => false,
        }
    }
}

fn filter_attack_name(line: &String) -> String {
    let close_position = line.find("]");

    let from = match close_position {
        None => 2,
        Some(x) => x + 2,
    };

    return line.chars().skip(from).collect()
}


fn read_new_pokemon(filename: &str, pokemons: &mut Vec<Pokemon>) {
    let file = File::open(filename);

    if file.is_err() {
        return;
    }

    let reader = BufReader::new(file.unwrap());

    let mut current_pokemon : Option<Pokemon> = Option::None;
    let mut state_separator= 1;
    let mut type_of_move : KindOfMove = KindOfMove::None;
    let mut last_seen_pokemon_name : Option<String> = Option::None;

    let array_text = [String::from("======"),
        String::from("Level Up Moves:"),
        String::from("Egg Moves:"),
        String::from("TMs:"),
        String::from("TRs:")
    ];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.len() == 0 {
            continue;
        }

        // TODO : find how to match line with the texts
        if line == array_text[0] {
            state_separator = (state_separator + 1) % 2;
            type_of_move = KindOfMove::None;

            if state_separator == 0 {
                match current_pokemon {
                    None => {},
                    Some(x) => {
                        pokemons.push(x);
                    },
                }

                current_pokemon = Option::None;
            }
        } else if line == array_text[1] {
            type_of_move = KindOfMove::LevelUp;
        } else if line == array_text[2] {
            type_of_move = KindOfMove::EggMove;
        } else if line == array_text[3] {
            type_of_move = KindOfMove::TM;
        } else if line == array_text[4] {
            type_of_move = KindOfMove::TR;
        } else {
            if state_separator == 0 {
                last_seen_pokemon_name = Option::Some(line.clone());
            } else {
                if !type_of_move.is_none() && line.chars().nth(0).unwrap() == '-' {
                    match current_pokemon.as_mut() {
                        Some(p) => {
                            p.add_move(&filter_attack_name(&line));
                        },
                        _ => {},
                    }
                } else {
                    type_of_move = KindOfMove::None;

                    if line.starts_with("Type: ") {
                        let actual_type = String::from(&line[6..]);

                        current_pokemon = Option::Some(Pokemon::new(&last_seen_pokemon_name.as_ref().unwrap(), &actual_type));
                    }
                }
            }
        }
    }

    match current_pokemon {
        None => {},
        Some(x) => {
            pokemons.push(x)
        },
    }

    // println!("{:#?}", pokemons);
}



fn main() {
    let bulbasaur = Pokemon::new(&String::from("Bulbasaur"),
                                 &String::from("Grass / Poison"));

    println!("=== BULBASAUR DESCRIPTION ===");
    println!("{:?}", bulbasaur);

    println!("Is grass {}", bulbasaur.has_type(&String::from("Grass")));
    println!("Is poison {}", bulbasaur.has_type(&String::from("Poison")));
    println!("Is fire {}", bulbasaur.has_type(&String::from("Fire")));

    println!("=== LOOK FOR WATER TYPE WITH SHELL SMASH ===");

    let water_type = String::from("Water");
    let shell_smash = String::from("Shell Smash");

    let mut pokemons : Vec<Pokemon> = Vec::new();

    read_new_pokemon("Move1.txt", &mut pokemons);
    read_new_pokemon("Move2.txt", &mut pokemons);

    for pokemon in pokemons.iter()
        .filter(|p| p.has_type(&water_type) && p.has_move(&shell_smash)) {
        println!("{}", pokemon.get_name());
    }
}
