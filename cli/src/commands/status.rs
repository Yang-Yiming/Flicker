use crate::model::Status;
use crate::storage;

pub fn run() {
    let flickers = storage::read_all();
    for s in [Status::Inbox, Status::Kept, Status::Archived, Status::Deleted] {
        let count = flickers.iter().filter(|f| f.meta.status == s).count();
        println!("{:8}: {}", s, count);
    }
}
