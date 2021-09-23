
pub mod CardDB {
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use uuid::Uuid;
    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    
    use crate::card::Card;

    #[derive(Serialize, Deserialize)]
    struct RawCardDB {
        data: HashMap<String, serde_json::Value>
    }

    pub struct CardDB {
        cards: HashMap<Uuid, Card::Card>
    }
    
    pub fn new() -> CardDB {
        CardDB { cards: HashMap::new() }
    }

    impl CardDB {
        
        pub fn read_json( mut self, filename: String ) -> Result<()> {
            println!( "Getting ready to read: {}", filename );

            let file_data = fs::read_to_string( filename ).expect( "Something went wrong..." );

            let file_json: RawCardDB = serde_json::from_str( &file_data )?;
            
            let mut cards: HashMap<Uuid, Card::Card> = HashMap::new();
            let mut types: Vec<String>;
            let mut printings: Vec<String>;
            let mut card_id: String;
            let mut card_uuid: Uuid;
            
            for (name, data) in file_json.data {
                types = Vec::new();
                printings = Vec::new();
                
                card_id = data[0]["identifiers"]["scryfallOracleId"].to_string();
                card_id = card_id[1..card_id.len()-1].to_string();
                println!( "{}: {}", name, card_id );
                card_uuid = Uuid::parse_str( &card_id ).unwrap();

                if ! data[0]["printings"].is_array() {
                    continue;
                }
                if ! data[0]["types"].is_array() {
                    continue;
                }
                
                for p in data[0]["printings"].as_array().unwrap() {
                    printings.push( p.to_string() );
                }
             
                for tp in data[0]["types"].as_array().unwrap() {
                    types.push( tp.to_string() );
                }

                cards.insert( card_uuid, Card::Card { name, printings, types } );
            }

            self.cards = cards;

            Ok(())
        }
    }
}

