mod parse;
mod search;
mod interaction;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please provide a pattern to search for and a file to search in as arguments.");
        return;
    }

    let pattern = &args[1];
    let filename = &args[2];

    let content = fs::read_to_string(filename)
        .expect(format!("Specified file {} does not exist", filename).as_str());

    let entries = parse::parse_entries(&mut content.lines().peekable(), 0).expect("Content was ill formatted");

    let found_entries = search::search_pattern(pattern, &entries);

    if !found_entries.is_empty() {
        for entry in found_entries {
            interaction::present_entry(entry.clone(), "");
        }
    } else {
        println!("No entries found for the pattern '{}'", pattern);
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
        let expected = Some(vec![
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
