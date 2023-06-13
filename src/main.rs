//! A simple command-line password manager which I wrote for personal usage.
//! Difference to 'pass':
mod generation;
mod interaction;
mod parse;
mod search;

use crate::parse::{Entry, EntryVal};
use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// specifies password file
    #[arg(short, long, value_name = "FILE")]
    path: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// retrieves a password
    Retrieve {
        #[arg(short, long)]
        /// specifies how the password should be output
        output_mode: OutputMode,

        /// the key pattern for which the password should be retrieved
        #[arg(short, long)]
        pattern: String,
    },
    /// generates a password and then writes it to the password file
    Generate {
        /// specifies the key for which a password should be generated
        #[arg(short, long)]
        key: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputMode {
    /// print password to stdout
    Print,
    /// copy password to clipboard
    Clipboard,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Commands::Retrieve {
            output_mode,
            pattern,
        } => {
            handle_retrieve(&args.path, &pattern, output_mode);
        }
        Commands::Generate { key } => {
            handle_generate(&args.path, &key);
        }
    }

    Ok(())
}

fn handle_retrieve(path: &PathBuf, pattern: &str, mode: OutputMode) {
    let mut content = fs::read_to_string(path).expect("Specified file does not exist");

    let entry = match parse::parse(&mut content) {
        Ok(entry) => entry,
        Err(e) => {
            println!("Error parsing file: {:#?}", e);
            return;
        }
    };

    let matching_entries = entry.search_pattern(pattern);
    if !matching_entries.is_empty() {
        let entries_cloned = matching_entries
            .iter()
            .cloned()
            .cloned()
            .collect::<Vec<_>>();
        interaction::present_subentries(&entries_cloned, "", mode);
    } else {
        println!("No entries found for the pattern '{}'", pattern);
    }
}
fn handle_generate(path: &PathBuf, key: &str) {
    let password = generation::generate_password(30);
    write_password(path, key, &password).expect("Could not write to specified file");
    println!("Generated password for '{}'.", key);

    // copy password to clipboard after successfull write
    let entry = vec![Entry {
        key: key.to_string(),
        val: EntryVal::Value(password),
    }];
    interaction::present_subentries(&entry, "", OutputMode::Clipboard);
}
fn write_password(path: &PathBuf, key: &str, password: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open(path)?;
    writeln!(file, "\n{}: {}", key, password)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::parse::Entry;
    use super::parse::EntryVal::{SubEntries, Value};
    use super::search;
    use std::fs;

    #[test]
    fn test_parse() {
        let content = fs::read_to_string("test_file")
            .expect("Test requires test file 'test_file' with specific contents to parse");
        let res = parse::parse_entries(&mut content.lines().peekable(), 0);
        let expected = Ok(vec![
            Entry {
                key: String::from("a"),
                val: Value(String::from("aval")),
            },
            Entry {
                key: String::from("b"),
                val: SubEntries(vec![
                    Entry {
                        key: String::from("b0"),
                        val: Value(String::from("b0val")),
                    },
                    Entry {
                        key: String::from("b1"),
                        val: Value(String::from("b1val")),
                    },
                    Entry {
                        key: String::from("b last"),
                        val: SubEntries(vec![
                            Entry {
                                key: String::from("b20"),
                                val: Value(String::from("b20val")),
                            },
                            Entry {
                                key: String::from("b21"),
                                val: Value(String::from("b21val")),
                            },
                        ]),
                    },
                ]),
            },
            Entry {
                key: String::from("c"),
                val: Value(String::from("c value")),
            },
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_search_leaf() {
        let content = fs::read_to_string("test_file")
            .expect("Test requires test file 'test_file' with specific contents to parse");
        let parsed = parse::parse_entries(&mut content.lines().peekable(), 0).unwrap();
        let res = search::search_pattern("b21", &parsed);
        assert!(res.len() == 1);
        let expected = Entry {
            key: String::from("b21"),
            val: Value(String::from("b21val")),
        };
        assert_eq!(res[0], &expected);
    }

    #[test]
    fn test_search_inner_node() {
        let content = fs::read_to_string("test_file")
            .expect("Test requires test file 'test_file' with specific contents to parse");
        let parsed = parse::parse_entries(&mut content.lines().peekable(), 0).unwrap();
        let res = search::search_pattern("b last", &parsed);
        assert!(res.len() == 1);
        let expected = Entry {
            key: String::from("b last"),
            val: SubEntries(vec![
                Entry {
                    key: String::from("b20"),
                    val: Value(String::from("b20val")),
                },
                Entry {
                    key: String::from("b21"),
                    val: Value(String::from("b21val")),
                },
            ]),
        };
        assert_eq!(res[0], &expected);
    }

    #[test]
    fn test_search_multiple_matches() {
        let content = fs::read_to_string("test_file")
            .expect("Test requires test file 'test_file' with specific contents to parse");
        let parsed = parse::parse_entries(&mut content.lines().peekable(), 0).unwrap();
        let res = search::search_pattern("b2", &parsed);
        let expected = vec![
            Entry {
                key: String::from("b20"),
                val: Value(String::from("b20val")),
            },
            Entry {
                key: String::from("b21"),
                val: Value(String::from("b21val")),
            },
        ];
        assert_eq!(res, expected.iter().collect::<Vec<&Entry>>());
    }
}
