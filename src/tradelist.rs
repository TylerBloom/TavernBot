
pub mod Tradelist {

    use std::collections::HashMap;
    
    use crate::utils::Types::*;
    use crate::card::Card;
    use crate::card_entry::CardEntry;

    pub struct Tradelist<'a> {
        cards: HashMap<String, Vec<CardEntry::CardEntry<'a>>>,
        is_public: bool
    }
    
    pub fn new<'a>( ) -> Tradelist<'a> {
        Tradelist { cards: HashMap::new(), is_public: false }
    }

    impl<'a> Tradelist<'a> {

        pub fn set_public( &mut self ) {
            self.is_public = true;
        }

        pub fn set_private( &mut self ) {
            self.is_public = false;
        }

        pub fn add_card( &mut self, count: CardCount, card: &Card::Card<'a> ) {
            if count == 0 {
                ()
            }
            
            let card_name = card.get_name();
            let mut entry = self.cards.get_mut( &card_name );
            if entry.is_none() {
                let mut new_vec: Vec<CardEntry::CardEntry<'a>> = Vec::new();
                new_vec.push( CardEntry::new(count, *card) );
                self.cards.insert( card_name, new_vec );
                ()
            }
            let mut listing = entry.unwrap();
            for c in listing {
                if c.card == *card {
                    c.count += count
                }
            }
            listing.push( CardEntry::new(count, *card) );
            ()
        }

        pub fn remove_card( &mut self, entry: CardEntry::CardEntry<'a> ) {
            if entry.count == 0 {
                ()
            }
            
            let card_name = entry.card.get_name();
            let entry_list = self.cards.get( &card_name );
            if entry_list.is_none() {
                ()
            }
            let mut listing = entry_list.unwrap();
            let mut needs_removed: bool = false;
            let mut index: usize = 0 ;
            for &(mut c) in listing {
                if c.card == entry.card {
                    c.dec_count( &entry.count );
                    needs_removed = c.count == 0;
                    break
                }
                index += 1;
            }
            if needs_removed {
                listing.remove( index );
            }
            if listing.len() == 0 {
                self.cards.remove( &card_name );
            }
            ()
        }

        pub fn contains_card( &self, card: Card::Card<'a> ) -> bool {
            let mut digest: bool = false;
            let card_name = card.get_name();
            let entry = self.cards.get( &card_name );
            if entry.is_none() {
                ()
            }
            let listing = entry.unwrap();
            for c in listing {
                if c.card.matches( &card ) {
                    digest = true;
                    break;
                }
            }
            digest
        }
    }
}

