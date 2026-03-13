use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::Cursor;

fn main() -> std::io::Result<()> {
    let mut cursor = Cursor::new([0xaa, 0xbb, 0xcc, 0xdd]);
    let val = cursor.read_u32::<LittleEndian>()?;
    println!("Little endian: {:x}", val);

    let mut cursor = Cursor::new([0xaa, 0xbb, 0xcc, 0xdd]);
    let val = cursor.read_u32::<BigEndian>()?;
    println!("Big endian: {:x}", val);

    use byteorder::WriteBytesExt;
    let mut buf = vec![];
    buf.write_u32::<LittleEndian>(0xddccbbaa)?;
    println!("Written bytes (LE): {:x?}", buf);

    let mut buf = vec![];
    buf.write_u32::<BigEndian>(0xaabbccdd)?;
    println!("Written bytes (BE): {:x?}", buf);

    Ok(())
}
