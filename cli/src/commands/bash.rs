pub fn run(cmd: &str) {
    let config = flicker_core::config::load();
    let status = std::process::Command::new(&config.shell)
        .arg("-c")
        .arg(cmd)
        .status()
        .unwrap_or_else(|e| { eprintln!("Error: {e}"); std::process::exit(1); });
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
