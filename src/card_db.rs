pub mod CardDB {

    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    use serenity::prelude::*;
    use std::collections::HashMap;
    use std::fs;

    use crate::card::Card;

    #[derive(Serialize, Deserialize)]
    struct RawCardDB {
        data: HashMap<String, serde_json::Value>,
    }

    pub struct CardDB {
        cards: HashMap<String, Card::AtomicCard>,
    }

    pub fn new() -> CardDB {
        CardDB {
            cards: HashMap::new(),
        }
    }

    pub fn create(filename: String) -> CardDB {
        let mut new_database: CardDB = new();
        new_database.read_json(filename);
        new_database
    }

    impl TypeMapKey for CardDB {
        type Value = CardDB;
    }

    impl CardDB {
        pub fn get_card(&self, card_name: &String) -> Option<&Card::AtomicCard> {
            self.cards.get(card_name)
        }

        pub fn read_json(&mut self, filename: String) -> Result<()> {
            println!("Getting ready to read: {}", filename);

            // Read in the data from the file
            let file_data = fs::read_to_string(filename).expect("Something went wrong...");

            // Use serde's serialize functionality to convert the json object to something a bit
            // more managable
            let raw_card_data: RawCardDB = serde_json::from_str(&file_data)?;

            // Each card struct has both a list of types and a list of printings. These need to be
            // pull from the json data.
            let mut types: Vec<String>;
            let mut printings: Vec<String>;

            // For each datum in the json data, we check that datum has a list of printings and
            // types (if not, we don't want it).
            for (name, data) in raw_card_data.data {
                if !data[0]["printings"].is_array() {
                    continue;
                }
                if !data[0]["types"].is_array() {
                    continue;
                }

                // Converts the json value struct to a list of strings
                printings = Vec::new();
                for p in data[0]["printings"].as_array().unwrap() {
                    printings.push(p.to_string());
                }

                // Converts the json value struct to a list of strings
                types = Vec::new();
                for tp in data[0]["types"].as_array().unwrap() {
                    types.push(tp.to_string());
                }

                // Adds a card to the HashMap index by the card's name
                self.cards.insert(
                    name.clone(),
                    Card::AtomicCard {
                        name,
                        printings,
                        types,
                    },
                );
            }

            Ok(())
        }
    }
}
