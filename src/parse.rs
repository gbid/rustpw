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
    val: Option<EntryVal>,
}
impl Entry {
    pub fn search_pattern(&self, pattern: &str) -> Vec<&Entry> {
        if self.key.contains(pattern) {
            vec![self]
        } else if let EntryVal::SubEntries(entries) = &self.val {
            entries
                .iter()
                .flat_map(|entry| entry.search_pattern(pattern))
                .collect()
        } else {
            Vec::new()
        }
    }
}

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::thread::sleep;
use std::time::Duration;
use std::io;
fn copy_value(val: String, path: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(val.to_owned()).unwrap();
    let seconds_to_clipboard_clean = 10;
    println!("Copied value to clipboard for key path:\n{}", path);
    println!("Value remains in clipboard for {} seconds", seconds_to_clipboard_clean);
    
    // wait for 10 seconds
    sleep(Duration::from_secs(seconds_to_clipboard_clean));

    // clear the clipboard
    ctx.set_contents("".to_string()).unwrap();
    println!("Cleared clipboard");
}

fn present_subentries(entries: &[Entry], path: &str) {
    let mut input = String::new();
    for (i, entry) in entries.iter().enumerate() {
        println!("{}: {}", i + 1, entry.key);
    }
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().parse::<usize>().unwrap();
    if choice > 0 && choice <= entries.len() {
        present_entry(entries[choice - 1].clone(), path);
    }
}

pub fn present_entry(entry: Entry, path: &str) {
    let next_path = if path.is_empty() {
        entry.key.clone()
    } else {
        format!("{}->{}", path, entry.key)
    };
    match entry.val {
        EntryVal::Value(val) => {
            copy_value(val, &next_path);
        },
        EntryVal::SubEntries(entries) => {
            present_subentries(&entries, &next_path);
        },
    }
}


//fn copy_value(val: String) {
//    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
//    ctx.set_contents(val.to_owned()).unwrap();
//    
//    // wait for 10 seconds
//    sleep(Duration::from_secs(10));
//
//    // clear the clipboard
//    ctx.set_contents("".to_string()).unwrap();
//}
//
//fn present_subentries(entries: &[Entry]) {
//    let mut input = String::new();
//    for (i, entry) in entries.iter().enumerate() {
//        println!("{}: {}", i + 1, entry.key);
//    }
//    io::stdin().read_line(&mut input).unwrap();
//    let choice = input.trim().parse::<usize>().unwrap();
//    if choice > 0 && choice <= entries.len() {
//        present_entry(entries[choice - 1].clone());
//    }
//}
//
//pub fn present_entry(entry: Entry) {
//    match entry.val {
//        EntryVal::Value(val) => {
//            copy_value(val);
//        },
//        EntryVal::SubEntries(entries) => {
//            present_subentries(&entries);
//        },
//    }
//}


pub fn search_pattern<'a>(pattern: &str, entries: &'a [Entry]) -> Vec<&'a Entry> {
    entries
        .iter()
        .flat_map(|entry| entry.search_pattern(pattern))
        .collect()
}

use std::iter::Peekable;

pub fn parse_entries(lines: &mut Peekable<std::str::Lines>, indent_lvl: usize) -> Option<Vec<Entry>> {
    let mut entries = Vec::new();
    while let Some(line) = lines.peek().cloned() { // clone the line in peek
        if line.is_empty() {
            lines.next(); // consume the line
            continue;
        }
        let line_indent_lvl = line.chars().take_while(|c| *c == '\t').count();
        if line_indent_lvl < indent_lvl {
            break;
        } else if indent_lvl == line_indent_lvl {
            lines.next(); // consume the line
            let partial_entry: PartialEntry = parse_entry(&line)?;
            let key = partial_entry.key;
            let val = if let Some(atom_val) = partial_entry.val {
                atom_val
            } else {
                EntryVal::SubEntries(parse_entries(lines, indent_lvl + 1)?)
            };
            entries.push(Entry { key, val });
        } else {
            lines.next(); // consume the line
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
    Some(PartialEntry { key, val })
}

#[cfg(test)]
mod tests {
    use super::EntryVal::{SubEntries, Value};
    use super::*;
    use std::fs;

    #[test]
    fn test_parse() {
        let content = fs::read_to_string("test_file")
            .expect("Test requires test file 'test_file' with specific contents to parse");
        let res = parse_entries(&mut content.lines(), 0);
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
                        key: String::from("b_last"),
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
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_search_leaf() {
        let content = fs::read_to_string("test_file")
            .expect("Test requires test file 'test_file' with specific contents to parse");
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
        let content = fs::read_to_string("test_file")
            .expect("Test requires test file 'test_file' with specific contents to parse");
        let parsed = parse_entries(&mut content.lines(), 0).unwrap();
        let res = search_pattern("b_last", &parsed);
        assert!(res.len() == 1);
        let expected = Entry {
            key: String::from("b_last"),
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
        let parsed = parse_entries(&mut content.lines(), 0).unwrap();
        let res = search_pattern("b2", &parsed);
        //assert!(res.len() == 2);
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
