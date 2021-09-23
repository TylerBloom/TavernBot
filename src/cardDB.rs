
pub mod CardDB {
    use std::collections::HashMap;
    use std::fs;
    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    
    use crate::card::Card;

    #[derive(Serialize, Deserialize)]
    struct RawCardDB {
        data: HashMap<String, serde_json::Value>
    }

    pub struct CardDB {
        cards: Vec<Card::Card>
    }
    
    pub fn new() -> CardDB {
        CardDB { cards: Vec::new() }
    }

    impl CardDB {

        pub fn get_card( &self, card_name: String ) -> Option<&Card::Card> {
            let mut digest: Option<&Card::Card> = None;
            for card in &self.cards {
                if card_name == card.name {
                    digest = Some(card);
                } else {
                    continue;
                };
            }
            digest
        }
        
        pub fn read_json( &mut self, filename: String ) -> Result<()> {
            println!( "Getting ready to read: {}", filename );

            let file_data = fs::read_to_string( filename ).expect( "Something went wrong..." );

            let file_json: RawCardDB = serde_json::from_str( &file_data )?;
            
            let mut types: Vec<String>;
            let mut printings: Vec<String>;
            
            for (name, data) in file_json.data {
                if ! data[0]["printings"].is_array() {
                    continue;
                }
                if ! data[0]["types"].is_array() {
                    continue;
                }
                
                printings = Vec::new();
                for p in data[0]["printings"].as_array().unwrap() {
                    printings.push( p.to_string() );
                }
             
                types = Vec::new();
                for tp in data[0]["types"].as_array().unwrap() {
                    types.push( tp.to_string() );
                }

                self.cards.push( Card::Card { name, printings, types } );
            }

            Ok(())
        }
    }
}

