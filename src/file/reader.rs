use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};

pub(crate) fn read_header(file_path: &str) -> io::Result<(HashMap<u8, Vec<bool>>, u64)> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut cursor = Cursor::new(buffer);

    let mut huff_codes = HashMap::new();
    let mut read_byte = cursor.read_u8()?;
    while read_byte != 0u8 {
        let key = read_byte;
        let code_length = cursor.read_u16::<LittleEndian>()? as usize;
        let mut code = Vec::with_capacity(code_length);

        for _ in 0..code_length {
            let bit = cursor.read_u8()?;
            code.push(bit == 1);
        }

        huff_codes.insert(key, code);
        read_byte = cursor.read_u8()?;
    }

    Ok((huff_codes, cursor.position()))
}