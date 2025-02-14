use std::env;
use std::fs;

fn main() {
    // const ENVIRONMENTS = env::args();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: <program> <file_path> <query>");
        std::process::exit(1);
        // return;
    }
    let file_path = &args[1];
    let query = &args[2];
    let file_to_read = fs::read_to_string(file_path).expect(
        "Error trying to read the file. Make sure that it exists and is in the specified directory",
    );
    let mut words = file_to_read.split_whitespace();
    let mut count = 0;
    for word in &mut words {
        if word == query {
            count += 1;
        }
    }

    println!("{query} appears {count} times");
}
