mod parse;

use std::env;
use std::fs;
use parse::*;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a pattern was provided
    if args.len() < 3 {
        println!("Please provide a pattern to search for and a file to serach in as arguments.");
        return;
    }

    // The second argument is the pattern to search for
    let pattern = &args[1];
    let filename = &args[2];

    // Read and parse the file
    //let content = fs::read_to_string("/home/gbid/test_pws")
    let content = fs::read_to_string(filename)
        .expect(format!("Specified file {} does not exist", filename).as_str());

    let entries = parse_entries(&mut content.lines().peekable(), 0).expect("Content was ill formatted");

    // Search for the pattern
    let found_entries = search_pattern(pattern, &entries);

    // If we found some entries, present them to the user
    if !found_entries.is_empty() {
        for entry in found_entries {
            present_entry(entry.clone(), "");
        }
    } else {
        println!("No entries found for the pattern '{}'", pattern);
    }
}

