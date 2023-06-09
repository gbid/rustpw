mod parse;
mod search;
mod interaction;

use std::env;
use std::fs;

pub enum OutputMode {
    Print,
    Clipboard,
}
enum Mode {
    Retrieve(OutputMode),
    Generate,
}

fn handle_retrieve(args: &[String], mode: &OutputMode) {
    if args.len() < 4 {
        eprintln!("Please provide a file to search in and a pattern to search for as arguments.");
        return;
    }

    let filename = &args[2];
    let pattern = &args[3];
    let mut content = fs::read_to_string(filename)
        .expect(format!("Specified file {} does not exist", filename).as_str());

    let entry = match parse::parse(&mut content) {
        Ok(entry) => entry,
        Err(e) => {
            println!("Error parsing file: {:#?}", e);
            return;
        },
    };

    let matching_entries = entry.search_pattern(pattern);
    if !matching_entries.is_empty() {
        let entries_cloned = matching_entries.iter().cloned().cloned().collect::<Vec<_>>();
        interaction::present_subentries(&entries_cloned, "", mode);
    } else {
        println!("No entries found for the pattern '{}'", pattern);
    }
}
fn handle_generate(args: &[String]) {
    todo!("{:?}", args)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a mode to supply from 'clipboard', 'print', 'generate' as first argument");
        return;
    }

    let mode = match args[1].as_str() {
        "clipboard" => Mode::Retrieve(OutputMode::Clipboard),
        "print" => Mode::Retrieve(OutputMode::Print),
        "generate" => Mode::Generate,
        _ => {
            eprintln!("Please provide a mode to supply from 'clipboard', 'print', 'generate' as first argument");
            return;
        }
    };

    match mode {
        Mode::Retrieve(OutputMode::Clipboard) => {
            handle_retrieve(&args, &OutputMode::Clipboard);
        },
        Mode::Retrieve(OutputMode::Print) => {
            handle_retrieve(&args, &OutputMode::Print);
        },
        Mode::Generate => {
            handle_generate(&args);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::parse::EntryVal::{SubEntries, Value};
    use super::parse::Entry;
    use super::parse;
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
