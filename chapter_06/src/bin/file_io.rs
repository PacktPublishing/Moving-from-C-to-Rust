use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let contents: String = fs::read_to_string("file.txt")?;
    println!("File contents: {}", contents);

    let contents: Vec<u8> = fs::read("file.bin")?;
    println!("Binary contents length: {}", contents.len());

    fs::write("out.txt", "Hello World\n")?;

    let _file = File::open("test.txt")?;

    let _file = File::create("new_file.txt")?;

    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open("file.txt")?;

    file.write_all(b"Appended text\n")?;

    println!("File operations completed successfully");
    Ok(())
}
