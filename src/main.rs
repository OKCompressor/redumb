// src/main.rs

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  redumb encode <input_file> <dict_dir> <sorted_dict_dir> <encoded_dir>");
    eprintln!("  redumb restore <dict_dir> <encoded_dir> <output_file>");
    eprintln!("  redumb merge-dicts <chunked_dict_dir> <output_dict_file>");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    match args[1].as_str() {
        "encode" => {
            if args.len() != 6 {
                print_usage();
                std::process::exit(1);
            }
            if let Err(e) = redumb::process::transform_chunked(
                &args[2], &args[3], &args[4], &args[5],
            ) {
                eprintln!("Error during encoding: {}", e);
                std::process::exit(1);
            }
        }
        "restore" => {
            if args.len() != 5 {
                print_usage();
                std::process::exit(1);
            }
            if let Err(e) = redumb::process::restore_chunked(
                &args[2], &args[3], &args[4],
            ) {
                eprintln!("Error during restore: {}", e);
                std::process::exit(1);
            }
        }
		"merge-dicts" => {
				    if args.len() != 4 {
				        print_usage();
				        std::process::exit(1);
				    }
				    if let Err(e) = redumb::process::merge_dicts(&args[2], &args[3]) {
				        eprintln!("Error merging dicts: {}", e);
				        std::process::exit(1);
				    }
				}
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

