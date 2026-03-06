use flicker_core::Flicker;
use flicker_core::storage;

pub fn run(text: &str) {
    let mut flicker = Flicker::new(text.to_string(), "cli");
    let id = flicker.meta.id.clone();
    storage::write(&mut flicker).unwrap();
    println!("{id}");
}
