use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    // Set up command-line argument parsing
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <pairing_file_path> <log_file_path>", args[0]);
        std::process::exit(1);
    }

    let pairing_file_path = &args[1];
    let log_file_path = &args[2];

    // Read the pairing file
    let pairing_file_contents = match fs::read_to_string(pairing_file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading pairing file: {}", e);
            std::process::exit(1);
        }
    };

    // Ensure the log file directory exists
    if let Some(parent) = PathBuf::from(log_file_path).parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Error creating log directory: {}", e);
            std::process::exit(1);
        }
    }

    // Start minimuxer
    match minimuxer::muxer::start(pairing_file_contents, log_file_path.to_string()) {
        Ok(_) => println!("Minimuxer started successfully"),
        Err(e) => {
            eprintln!("Error starting minimuxer: {:?}", e);
            std::process::exit(1);
        }
    }

    // Set the target address for minimuxer
    minimuxer::muxer::target_minimuxer_address();

    println!("Minimuxer is running. Press Ctrl+C to exit.");

    // Keep the program running
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}