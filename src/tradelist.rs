pub mod Tradelist {

    use std::collections::HashMap;

    use crate::card::Card;
    use crate::card_entry::CardEntry;
    use crate::utils::Types::*;

    pub struct Tradelist<'a> {
        cards: HashMap<String, Vec<CardEntry::CardEntry<'a>>>,
        is_public: bool,
    }

    pub fn new<'a>() -> Tradelist<'a> {
        Tradelist {
            cards: HashMap::new(),
            is_public: false,
        }
    }

    impl<'a> Tradelist<'a> {
        pub fn set_public(&mut self) {
            self.is_public = true;
        }

        pub fn set_private(&mut self) {
            self.is_public = false;
        }

        fn add_new_entry(&mut self, entry: CardEntry::CardEntry<'a>) {
            let mut new_vec: Vec<CardEntry::CardEntry> = Vec::new();
            new_vec.push(entry.clone());
            self.cards.insert(entry.card.get_name(), new_vec);
        }

        fn increase_entry(&mut self, entry: CardEntry::CardEntry<'a>) {
            for c in self.cards.get_mut(&entry.card.get_name()).unwrap() {
                if c.card == entry.card {
                    c.count += entry.count
                }
            }
            self.cards
                .get_mut(&entry.card.get_name())
                .unwrap()
                .push(entry);
        }

        fn decrease_entry(&mut self, entry: CardEntry::CardEntry<'a>) {
            let listing = self.cards.get_mut(&entry.card.get_name()).unwrap();
            let mut i: usize = 0;
            while i < listing.len() {
                if listing[i].card == entry.card {
                    listing[i].dec_count(&entry.count);
                    listing.remove(i);
                    if listing.len() == 0 {
                        self.cards.remove(&entry.card.get_name());
                    }
                    break;
                }
                i += 1;
            }
        }

        pub fn add_card(&mut self, count: CardCount, card: Card::Card<'a>) {
            if count == 0 {
                ()
            }

            match self.cards.get(&card.get_name()) {
                None => self.add_new_entry(CardEntry::new(count, card)),
                Some(_) => self.increase_entry(CardEntry::new(count, card)),
            }
        }

        pub fn remove_card(&mut self, count: CardCount, card: Card::Card<'a>) {
            if count == 0 {
                ()
            }

            match self.cards.get(&card.get_name()) {
                None => (),
                Some(_) => self.decrease_entry(CardEntry::new(count, card)),
            }
        }

        pub fn contains_card(&self, card: Card::Card<'a>) -> bool {
            let mut digest: bool = false;
            let card_name = card.get_name();
            let entry = self.cards.get(&card_name);
            if entry.is_none() {
                ()
            }
            let listing = entry.unwrap();
            for c in listing {
                if c.card.matches(&card) {
                    digest = true;
                    break;
                }
            }
            digest
        }
    }
}
