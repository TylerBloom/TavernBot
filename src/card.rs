
pub mod Card{

    use std::fmt;

    #[derive( Clone )]
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

    #[derive( Clone )]
    pub struct Card<'a>  {
        pub card: &'a AtomicCard,
        pub printing: String
    }

    pub fn new<'a>( card: &'a AtomicCard, printing: Option<String> ) -> Card<'a> {
        match printing {
            Some(p) => Card { card, printing: p },
            None => Card { card, printing: String::from("") }
        }
    }

    impl<'a> Card<'a> {
        pub fn matches( &self, other: &Card ) -> bool {
            self.card == other.card && self.printing == other.printing
        }
        
        pub fn get_name( &self ) -> String {
            self.card.name.clone()
        }
    }

    impl<'a> PartialEq for Card<'a> {
        fn eq( &self, other: &Card ) -> bool {
            let mut digest : bool  = self.printing == other.printing;
            digest |= self.card == other.card;
            digest
        }
    }

}
