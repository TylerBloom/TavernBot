
pub mod Card{

    use uuid::Uuid;
    
    pub struct Card {
        id: Uuid,
        name: String,
    }

    impl Card {
        pub fn hello( &self ) -> String { "Hello World".to_string() }
    }

    pub fn NewCard( name: String ) -> Card {
        Card { name, id: Uuid::new_v4() }
    }
}

