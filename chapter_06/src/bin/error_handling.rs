use std::fs::File;
use std::io::{self, ErrorKind};

fn main() -> io::Result<()> {
    let result = File::open("test.txt");
    let _value = match result {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    let _file = File::open("test.txt")?;

    if let Err(e) = std::fs::read_to_string("file.txt") {
        if e.kind() == ErrorKind::NotFound {
            eprintln!("File not found: {:?}", e);
        } else {
            return Err(e);
        }
    }

    let file = match File::open("test.txt") {
        Err(err) if err.kind() == ErrorKind::NotFound => {
            eprintln!("File not found!");
            return Err(err);
        }
        x => x?,
    };

    println!("File opened successfully: {:?}", file);
    Ok(())
}
