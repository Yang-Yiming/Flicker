use flicker_core::Status;
use flicker_core::storage;
use std::str::FromStr;

pub fn run(status_filter: Option<&str>) {
    let mut flickers = storage::read_all();

    if let Some(s) = status_filter {
        let f = Status::from_str(s).unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(1); });
        flickers.retain(|fl| fl.meta.status == f);
    } else {
        flickers.retain(|fl| fl.meta.status != Status::Deleted);
    }

    flickers.sort_by(|a, b| b.meta.created_at.cmp(&a.meta.created_at));

    for f in &flickers {
        let preview: String = f.body.lines().next().unwrap_or("").chars().take(60).collect();
        println!("{} [{:8}] {}", f.meta.id, f.meta.status, preview);
    }
}
