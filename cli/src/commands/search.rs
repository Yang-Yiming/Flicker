use flicker_core::Status;
use flicker_core::storage;

pub fn run(query: &str) {
    let q = query.to_lowercase();
    for f in storage::read_all().iter().filter(|f| f.meta.status != Status::Deleted) {
        if f.body.to_lowercase().contains(&q) {
            let preview: String = f.body.lines().next().unwrap_or("").chars().take(60).collect();
            println!("{} [{:8}] {}", f.meta.id, f.meta.status, preview);
        }
    }
}
