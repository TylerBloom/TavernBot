
pub mod Card{

    use std::fmt;

    pub struct Card {
        pub name: String,
        pub printings: Vec<String>,
        pub types: Vec<String>
    }

    impl std::fmt::Display for Card {

        fn fmt ( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
            write!( f, "{}", self.name )
        }
    }

}

