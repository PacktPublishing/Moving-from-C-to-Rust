use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter, Cursor, SeekFrom};

fn main() -> io::Result<()> {
    let mut file = File::open("file.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("Read to string: {}", buffer);

    let mut f = File::open("file.txt")?;
    let mut buf = [0; 10];
    let bytes_read = f.read(&mut buf)?;
    println!("Bytes read: {}", bytes_read);

    let mut f = File::open("file.txt")?;
    let mut buf = vec![0; 10];
    f.read_exact(&mut buf)?;
    println!("Read exact: {:?}", buf);

    let file = File::open("file.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    println!("Buffered read line: {}", line);

    let mut file = File::create("test.txt")?;
    file.write_all("Hello World\n".as_bytes())?;

    file.flush()?;

    let file = File::create("buffered.txt")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"Buffered write\n")?;
    writer.flush()?;

    let mut file = File::open("test.txt")?;
    file.seek(SeekFrom::End(-10))?;
    let mut buf = vec![0; 10];
    file.read_exact(&mut buf)?;
    println!("Last 10 bytes: {:?}", buf);

    let pos = file.seek(SeekFrom::Current(0))?;
    println!("Current position: {}", pos);

    let mut cursor = Cursor::new(b"test buffer content");
    cursor.seek(SeekFrom::End(-10))?;
    let mut buf = vec![0; 10];
    cursor.read_exact(&mut buf)?;
    println!("Cursor read: {:?}", String::from_utf8_lossy(&buf));

    pub trait ReadExt: std::io::Read {
        fn read_ext(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.read(buf)
        }
    }

    impl<T> ReadExt for T where T: Read {}

    let mut file = File::open("file.txt")?;
    let mut buf = [0; 10];
    let _bytes = file.read_ext(&mut buf)?;

    println!("All I/O operations completed successfully");
    Ok(())
}
