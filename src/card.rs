pub mod Card {

    use std::fmt;

    #[derive(Clone)]
    pub struct AtomicCard {
        pub name: String,
        pub printings: Vec<String>,
        pub types: Vec<String>,
    }

    impl std::fmt::Display for AtomicCard {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.name)
        }
    }

    impl PartialEq for AtomicCard {
        fn eq(&self, other: &AtomicCard) -> bool {
            let mut digest: bool = self.name == other.name;
            digest |= self.types == other.types;
            // Due to issues with Cards keeping a reference to an Atomic, this isn't being checked
            //digest |= self.printings == other.printings;
            digest
        }
    }

    #[derive(Clone)]
    pub struct Card {
        pub card: AtomicCard,
        pub printing: String,
    }

    pub fn new(card: AtomicCard, printing: String) -> Card {
        if card.printings.contains(&printing) {
            Card {
                card,
                printing: printing,
            }
        } else {
            Card {
                card,
                printing: printing,
            }
        }
    }

    impl Card {
        pub fn matches(&self, other: &Card) -> bool {
            let mut digest: bool = self.card == other.card;
            if (!self.printing.is_empty() && !other.printing.is_empty()) {
                digest &= self.printing == other.printing;
            }
            digest
        }

        pub fn get_name(&self) -> String {
            self.card.name.clone()
        }
        
        pub fn to_string( &self ) -> String {
            if( self.printing.is_empty() )
            {
                format!( "{}", self.card.to_string() )
            } else {
                format!( "{} [{}]", self.card.to_string(), self.printing )
            }
        }
    }

    impl PartialEq for Card {
        fn eq(&self, other: &Card) -> bool {
            self.card == other.card && self.printing == other.printing
        }
    }
}
