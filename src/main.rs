use crate::pokemon::Pokemon;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

mod pokemon;

enum KindOfMove {
    NONE,
    LEVEL_UP,
    EGG_MOVE,
    MOVE_TUTOR,
    TM,
    TR
}


fn read_new_pokemon(filename: &str, pokemons: &mut Vec<Pokemon>) {
    let f = File::open(filename);

    if f.is_err() {
        return;
    }

    let file = BufReader::new(&f.unwrap());

    let mut current_pokemon : Option<Pokemon> = Option::None;
    let mut state_seperator : 0;
    let mut type_of_move : KindOfMove = KindOfMove::NONE;

    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();


        match l {
            String::from("======") => {
                state_seperator = (state_seperator + 1) % 2;
                type_of_move = KindOfMove::NONE;

                if state_seperator == 0 {
                    current_pokemon = Option::None;
                }
            },
            String::from("Level Up Moves:") => {
                type_of_move = KindOfMove::LEVEL_UP;
            },
            String::from("Egg Moves:") => {
                type_of_move = KindOfMove::EGG_MOVE;
            },
            String::from("TMs:") => {
                type_of_move = KindOfMove::TM;
            },
            String::from("TRs:") => {
                type_of_move = KindOfMove::TR;
            },
            _ => {
                if state_seperator == 0 {


                }



            }


        }








    }




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





}
