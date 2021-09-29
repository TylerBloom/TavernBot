pub mod CardEntry {

    use crate::card::Card;
    use crate::utils::Types::*;

    #[derive(Clone)]
    pub struct CardEntry<'a> {
        pub count: CardCount,
        pub card: Card::Card<'a>,
    }

    pub fn new(count: CardCount, card: Card::Card) -> CardEntry {
        CardEntry { count, card }
    }

    impl<'a> CardEntry<'a> {
        pub fn inc_count(&mut self, count: &CardCount) {
            self.count += count;
        }

        pub fn dec_count(&mut self, count: &CardCount) {
            if self.count >= *count {
                self.count -= count;
            } else {
                self.count = 0;
            }
        }

        pub fn update_count(&mut self, count: CardCount) {
            self.count = count;
        }
    }
}
