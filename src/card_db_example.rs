
mod card;
mod cardDB;

//use std::env;

pub use card::Card;
pub use cardDB::CardDB;

fn main() {
    let db = CardDB::create( String::from( "AtomicCards" ) );
    let mut test_card_name = String::from( "Izzet Charm" );
    
    //db.read_json( "AtomicCards.json".to_string() );
    //db.read_json( "testing.json".to_string() );
    
    let mut card = db.get_card( &test_card_name ); 
    println!( "Is {:?} here? {}", test_card_name, card.is_some() );
    if card.is_some() {
        println!( "\t- Look, it's {}", card.unwrap().name );
    }
    test_card_name = String::from( "Izzet Charms" );
    card = db.get_card( &test_card_name ); 
    println!( "Is {:?} here? {}", test_card_name, card.is_some() );
}
