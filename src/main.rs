mod parse;
use std::{io, fs};
use parse::{parse_entries, Entry};



fn main() -> io::Result<()> {
    let content = fs::read_to_string("test_file")?;
    let res: Option<Vec<Entry>> = parse_entries(&mut content.lines(), 0); 
    println!("{:#?}", res);

    Ok(())
}
