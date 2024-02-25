use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};

pub(crate) fn read_header(file_path: &str) -> io::Result<(HashMap<u8, Vec<bool>>, u64, u64)> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut cursor = Cursor::new(buffer);

    // Lire la longueur totale des données compressées en bits
    let total_bits = cursor.read_u64::<LittleEndian>()?;

    let mut huff_codes = HashMap::new();

    // Assurez-vous de commencer la lecture des codes après avoir lu la longueur totale
    while let Ok(key) = cursor.read_u8() {
        if key == 0u8 { break; } // Fin de l'en-tête détectée

        let code_length = cursor.read_u16::<LittleEndian>()? as usize;
        let mut code_bits = Vec::with_capacity(code_length);
        let mut bits_read = 0;

        while bits_read < code_length {
            let byte = cursor.read_u8()?;
            for i in 0..8 {
                if bits_read == code_length { break; }
                let bit = (byte >> (7 - i)) & 1;
                code_bits.push(bit == 1);
                bits_read += 1;
            }
        }

        huff_codes.insert(key, code_bits);
    }

    // Retourner également la position actuelle du curseur pour savoir où commencent les données compressées
    Ok((huff_codes, total_bits, cursor.position()))
}
