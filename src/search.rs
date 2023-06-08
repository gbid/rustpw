use crate::parse::{Entry, EntryVal};

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

pub fn _search_pattern<'a>(pattern: &str, entries: &'a [Entry]) -> Vec<&'a Entry> {
    entries
        .iter()
        .flat_map(|entry| entry.search_pattern(pattern))
        .collect()
}
