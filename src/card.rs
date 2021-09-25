
pub mod Card{

    use std::fmt;

    pub struct AtomicCard {
        pub name: String,
        pub printings: Vec<String>,
        pub types: Vec<String>
    }

    impl std::fmt::Display for AtomicCard {
        fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
            write!( f, "{}", self.name )
        }
    }

    impl PartialEq for AtomicCard {
        fn eq( &self, other: &AtomicCard ) -> bool {
            let mut digest : bool  = self.name == other.name;
            digest |= self.printings == other.printings;
            digest |= self.types == other.types;
            digest
        }
    }

    pub struct Card<'a> {
        pub card: &'a AtomicCard,
        pub printing: Option<String>
    }

    pub fn new<'a>( card: &'a AtomicCard, printing: Option<String> ) -> Card {
        Card { card, printing }
    }

    impl<'a> Card<'a> {
        fn matches( &'a self, other: &'a Card ) -> bool {
            self.card == other.card && self.printing == other.printing
        }
    }

}

