#[derive(Debug, PartialEq, Clone)]
enum EntryVal {
    Value(String),
    SubEntries(Vec<Entry>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
    key: String,
    val: EntryVal,
}

struct PartialEntry {
    key: String,
    val: Option<EntryVal>
}
impl Entry {
    pub fn search_pattern(&self, pattern: &str) -> Vec<&Entry> {
        if self.key.contains(pattern) {
            vec![self]
        } else if let EntryVal::SubEntries(entries) = &self.val {
            entries.iter().flat_map(|entry| entry.search_pattern(pattern)).collect()
        } else {
            Vec::new()
        }
    }
}

pub fn search_pattern<'a>(pattern: &str, entries: &'a [Entry]) -> Vec<&'a Entry> {
    entries.iter().flat_map(|entry| entry.search_pattern(pattern)).collect()
}
pub fn parse_entries(lines: &mut std::str::Lines, indent_lvl: usize) -> Option<Vec<Entry>> {
    let mut entries = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let line_indent_lvl = line.chars().take_while(|c| *c == '\t').count();
        if indent_lvl ==  line_indent_lvl {
            let partial_entry: PartialEntry = parse_entry(line)?;
            let key = partial_entry.key;
            let val = if let Some(atom_val) = partial_entry.val {
                atom_val
            } else {
                EntryVal::SubEntries(parse_entries(lines, indent_lvl + 1)?)
            };
            entries.push(Entry {
                key,
                val,
            });
        }
    }
    Some(entries)
}

fn parse_entry(line: &str) -> Option<PartialEntry> {
    let mut parts = line.splitn(2, ':');
    let key = parts.next()?.trim().to_string();
    let val = parts.next()?.trim().to_string();
    let val = if val == String::from("") {
        None
    } else {
        Some(EntryVal::Value(val))
    };
    Some(PartialEntry {
        key,
        val,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::EntryVal::{Value, SubEntries};
    use std::fs;

    #[test]
    fn test_parse() {
        let content = fs::read_to_string("test_file").expect("Test requires test file 'test_file' with specific contents to parse");
        let res = parse_entries(&mut content.lines(), 0); 
        let expected = Some(
            vec![
            Entry {
                key: String::from("a"),
                val: Value(
                    String::from("aval"),
                    ),
            },
            Entry {
                key: String::from("b"),
                val: SubEntries(
                    vec![
                    Entry {
                        key: String::from("b0"),
                        val: Value(
                            String::from("b0val"),
                            ),
                    },
                    Entry {
                        key: String::from("b1"),
                        val: Value(
                            String::from("b1val"),
                            ),
                    },
                    Entry {
                        key: String::from("b_last"),
                        val: SubEntries(
                            vec![
                            Entry {
                                key: String::from("b20"),
                                val: Value(
                                    String::from("b20val"),
                                    ),
                            },
                            Entry {
                                key: String::from("b21"),
                                val: Value(
                                    String::from("b21val"),
                                    ),
                            },
                            ],
                            ),
                    },
                    ],
                    ),
            },
            ],
            );
            assert_eq!(res, expected);
    }

    #[test]
    fn test_search_leaf() {
        let content = fs::read_to_string("test_file").expect("Test requires test file 'test_file' with specific contents to parse");
        let parsed = parse_entries(&mut content.lines(), 0).unwrap();
        let res = search_pattern("b21", &parsed);
        assert!(res.len() == 1);
        let expected = Entry {
            key: String::from("b21"),
            val: EntryVal::Value(String::from("b21val")),
        };
        assert_eq!(res[0], &expected);
    }

    #[test]
    fn test_search_inner_node() {
        let content = fs::read_to_string("test_file").expect("Test requires test file 'test_file' with specific contents to parse");
        let parsed = parse_entries(&mut content.lines(), 0).unwrap();
        let res = search_pattern("b_last", &parsed);
        assert!(res.len() == 1);
        let expected = Entry {
            key: String::from("b_last"),
            val: SubEntries(
                vec![
                Entry {
                    key: String::from("b20"),
                    val: Value(
                        String::from("b20val"),
                        ),
                },
                Entry {
                    key: String::from("b21"),
                    val: Value(
                        String::from("b21val"),
                        ),
                },
                ],
                ),
        };

        assert_eq!(res[0], &expected);
    }

    #[test]
    fn test_search_multiple_matches() {
        let content = fs::read_to_string("test_file").expect("Test requires test file 'test_file' with specific contents to parse");
        let parsed = parse_entries(&mut content.lines(), 0).unwrap();
        let res = search_pattern("b2", &parsed);
        //assert!(res.len() == 2);
        let expected = vec![
            Entry {
                key: String::from("b20"),
                val: Value(
                    String::from("b20val"),
                    ),
            },
            Entry {
                key: String::from("b21"),
                val: Value(
                    String::from("b21val"),
                    ),
            },
        ];
        assert_eq!(res, expected.iter().collect::<Vec<&Entry>>());
    }
}
