use crate::parse::{Entry, EntryVal};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::thread::sleep;
use std::time::Duration;
use std::io;

fn present_entry(entry: Entry, path: &str) {
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

pub fn present_subentries(entries: &[Entry], path: &str) {
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
