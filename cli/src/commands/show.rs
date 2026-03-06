use flicker_core::storage;

pub fn run(id: &str) {
    match storage::read_one(id) {
        Some(f) => {
            println!("id:         {}", f.meta.id);
            println!("created_at: {}", f.meta.created_at);
            println!("source:     {}", f.meta.source);
            println!("status:     {}", f.meta.status);
            if let Some(audio) = &f.meta.audio_file {
                println!("audio:      {audio}");
            }
            println!("\n{}", f.body);
        }
        None => eprintln!("flicker not found: {id}"),
    }
}
