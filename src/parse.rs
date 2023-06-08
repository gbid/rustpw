#[derive(Debug, PartialEq, Clone)]
pub enum EntryVal {
	Value(String),
	SubEntries(Vec<Entry>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
	pub key: String,
	pub val: EntryVal,
}

struct PartialEntry {
	key: String,
	val: Option<EntryVal>,
}

use std::iter::Peekable;
use std::str::Lines;

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String),
}

pub fn parse(content: &mut String) -> Result<Entry, ParseError> {
    let entries = parse_entries(&mut content.lines().peekable(), 0)?;
    Ok(Entry {
        key: String::from(""),
        val: EntryVal::SubEntries(entries),
    })
}
pub fn parse_entries(lines: &mut Peekable<Lines>, indent_lvl: usize) -> Result<Vec<Entry>, ParseError> {
    let mut entries = Vec::new();
    while let Some(line) = lines.peek().cloned() {
        if line.is_empty() {
            lines.next();
            continue;
        }
        let line_indent_lvl = line.chars().take_while(|c| *c == '\t').count();
        if line_indent_lvl < indent_lvl {
            break;
        } else if indent_lvl == line_indent_lvl {
            lines.next();
            let partial_entry: PartialEntry = parse_entry(&line)
                .ok_or(ParseError::InvalidFormat(format!("Cannot parse line:{}", line)))?;
            let key = partial_entry.key;
            let val = if let Some(atom_val) = partial_entry.val {
                atom_val
            } else {
                EntryVal::SubEntries(parse_entries(lines, indent_lvl + 1)?)
            };
            entries.push(Entry { key, val });
        } else {
            return Err(ParseError::InvalidFormat(format!("Invalid indentation level on line: {}", line)));
        }
    }
    Ok(entries)
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
