use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Seek};

pub(crate) fn compress(source: &str, huff_codes: &HashMap<u8, Vec<bool>>) -> Vec<bool> {
    // stock the compressed data
    let mut compressed_data = Vec::new();

    for byte in source.bytes() {
        // Obtenir le vecteur de code Huffman pour ce caractère
        if let Some(code) = huff_codes.get(&byte) {
            // Écrire le code de Huffman dans le fichier
            for &bit in code {
                compressed_data.push(bit);
            }
        }
    }

    compressed_data
}

pub(crate) fn decompress(file_path: &str, huff_codes: &HashMap<u8, Vec<bool>>, start_pos: u64) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    file.seek(io::SeekFrom::Start(start_pos))?;
    let mut compressed_data = Vec::new();
    file.read_to_end(&mut compressed_data)?;

    let mut decompressed_data = Vec::new();
    let mut bit_pos = 0;
    let mut current_bits = Vec::new();

    // Inversion de la table de codage pour la recherche de bits à octet
    let mut codes_to_byte = HashMap::new();
    for (byte, bits) in huff_codes.iter() {
        codes_to_byte.insert(bits.clone(), *byte);
    }

    while bit_pos < compressed_data.len() * 8 {
        let byte_pos = bit_pos / 8;
        let bit_index = bit_pos % 8;
        let bit = (compressed_data[byte_pos] >> (7 - bit_index)) & 1;
        current_bits.push(bit == 1);

        if let Some(&byte) = codes_to_byte.get(&current_bits) {
            decompressed_data.push(byte);
            current_bits.clear();
        }

        bit_pos += 1;
    }

    Ok(decompressed_data)
}