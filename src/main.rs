
mod card;
mod cardDB;

//use std::env;

pub use card::Card;
pub use cardDB::CardDB;

fn check_none( thing: Option<&Card::Card> ) -> bool {
    match thing {
        Some(p) => true,
        None => false
    }
}

fn main() {
    let mut db = CardDB::new( );
    db.read_json( "AtomicCards.json".to_string() );
    //db.read_json( "testing.json".to_string() );
    
    let mut card = db.get_card( "Izzet Charm".to_string() ); 
    println!( "Is 'Izzet Charm' here? {}", ! card.is_none() );
    if !card.is_none() {
        println!( "\t- Look, it's an {}", card.unwrap().name );
    }
    card = db.get_card( "Izzet Charms".to_string() ); 
    println!( "Is 'Izzet Charms' here? {}", ! card.is_none() );
    //if ! is_optional(card) {
    //    println!( "{}", card.name );
    //}
}
