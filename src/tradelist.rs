
pub mod Tradelist {

    use std::collections::HashMap;
    use std::fs;
    use serenity::prelude::*;
    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    
    use crate::card::Card;

    pub struct Tradelist {
        cards: Vec<Card>,
        is_public: bool
    }
    
    pub fn new( ) -> Tradelist {
        Tradelist { cards: Vec::new(), is_public: false }
    }

    impl Tradelist {

        pub fn set_public( &self ) {
            self.is_public = true;
        }

        pub fn set_private( &self ) {
            self.is_public = false;
        }

        pub fn add_card( &self, card: Card ) {
            self.cards.push( card );
        }

        pub fn remove_card( &self, card: Card ) {
        }

        pub fn get_card( &self, card_name: &String ) -> Option<Card> {
            for &c in &self.cards {
                if c.card.name == card {
                    card
                }
            }
            None
        }

        pub fn contains_card( &self, card: &Card ) -> bool {
            let digest: bool = false;
            for &c in &self.cards {
                if c.matches( card ) {
                    digest = true;
                }
            }
            digest
        }
    }
}

