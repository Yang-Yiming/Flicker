use crate::storage;

pub fn run(id: &str, new_body: &str) {
    match storage::read_one(id) {
        Some(mut flicker) => {
            flicker.body = new_body.to_string();
            storage::write(&flicker).unwrap_or_else(|e| eprintln!("Error: {e}"));
            println!("Updated {id}");
        }
        None => eprintln!("Not found: {id}"),
    }
}
