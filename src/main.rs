
mod card;
mod cardDB;

//use std::env;

pub use card::Card;
pub use cardDB::CardDB;

fn main() {
    let mut db = CardDB::new( );
    db.read_json( "AtomicCards.json".to_string() );
    //db.read_json( "testing.json".to_string() );
}
