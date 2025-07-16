// src/main.rs (example binary)
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: dumb_preprocessor <input> <dict> <output>");
        std::process::exit(1);
    }
    if let Err(e) = dumb_preprocessor::process::transform(&args[1], &args[2], &args[3]) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// TODO: Add restore pipeline and tests