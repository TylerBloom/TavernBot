pub mod CardEntry {

    use crate::card::Card;
    use crate::utils::Types::*;

    #[derive( Clone )]
    pub struct CardEntry {
        pub count: CardCount,
        pub card: Card::Card
    }

    pub fn new(count: CardCount, card: Card::Card) -> CardEntry {
        CardEntry { count, card }
    }
    
    impl CardEntry {
        pub fn inc_count( &mut self, count: &CardCount ) {
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
        
        pub fn to_string( &self ) -> String {
            format!( "{} {}", self.count, self.card.to_string()  )
        }
    }
}
