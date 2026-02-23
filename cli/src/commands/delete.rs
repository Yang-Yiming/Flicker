use crate::model::Status;
use crate::storage;

pub fn run(id: &str) {
    match storage::read_one(id) {
        Some(mut f) => {
            f.meta.status = Status::Deleted;
            storage::write(&f).unwrap();
            println!("deleted {id}");
        }
        None => eprintln!("flicker not found: {id}"),
    }
}
