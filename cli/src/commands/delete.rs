use flicker_core::Status;
use flicker_core::storage;

pub fn run(id: &str) {
    match storage::read_one(id) {
        Some(mut f) => {
            f.meta.status = Status::Deleted;
            storage::write(&mut f).unwrap();
            println!("deleted {id}");
        }
        None => eprintln!("flicker not found: {id}"),
    }
}
