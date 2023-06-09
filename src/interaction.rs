use crate::OutputMode;
use crate::parse::{Entry, EntryVal};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::thread::sleep;
use std::time::Duration;
use std::io;

fn present_entry(entry: Entry, path: &str, mode: &OutputMode) {
    let next_path = if path.is_empty() {
        entry.key.clone()
    } else {
        format!("{}->{}", path, entry.key)
    };
    match (entry.val, mode) {
        (EntryVal::Value(val), OutputMode::Clipboard) => {
            copy_value(val, &next_path);
        },
        (EntryVal::Value(val), OutputMode::Print) => {
            println!("{}", val);
        },
        (EntryVal::SubEntries(entries), _) => {
            present_subentries(&entries, &next_path, mode);
        },
    }
}

fn copy_value(val: String, path: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(val.to_owned()).unwrap();
    let seconds_to_clipboard_clean = 30;
    println!("Copied value to clipboard for key path:\n{}", path);
    println!("Value remains in clipboard for {} seconds", seconds_to_clipboard_clean);
    
    sleep(Duration::from_secs(seconds_to_clipboard_clean));

    ctx.set_contents("".to_string()).unwrap();
    println!("Cleared clipboard");
}

pub fn present_subentries(entries: &[Entry], path: &str, mode: &OutputMode) {
    if entries.len() > 1 {
        let mut input = String::new();
        for (i, entry) in entries.iter().enumerate() {
            println!("{}: {}", i + 1, entry.key);
        }
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<usize>().unwrap();
        if choice > 0 && choice <= entries.len() {
            present_entry(entries[choice - 1].clone(), path, mode);
        }
    } else {
        present_entry(entries[0].clone(), path, mode);
    }
}
