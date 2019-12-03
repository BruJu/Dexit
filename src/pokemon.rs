
use std::collections::HashSet;

#[derive(Debug)]
pub struct Pokemon {
    name: String,
    type1: String,
    type2: Option<String>,
    attacks: HashSet<String>,
}

impl Pokemon {
    pub fn new(name: &String, types: &String) -> Pokemon {
        let slash_pos = types.find("/");

        let type1: String;
        let type2: Option<String>;

        match slash_pos {
            Some(position) => {
                type1 = String::from(&types[0..position - 1]);
                type2 = Option::Some(String::from(&types[position + 2..]));
            },
            _ => {
                type1 = types.clone();
                type2 = None;
            }
        };

        Pokemon {
            name: name.clone(),
            type1,
            type2,
            attacks: HashSet::new()
        }
    }

    pub fn add_move(&mut self, attack: &String) {
        self.attacks.insert(attack.clone());
    }

    pub fn has_move(&self, attack: &String) -> bool {
        self.attacks.contains(attack)
    }

    pub fn has_type(&self, typing: &String) -> bool {
        if typing == &self.type1 {
            true
        } else {
            match &self.type2 {
                Some(x) => x.eq(typing),
                None => false
            }
        }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}
