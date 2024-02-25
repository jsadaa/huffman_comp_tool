use std::collections::HashMap;
use std::io;
use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};

use crate::file::bit_writer::BitWriter;

pub fn write_header(filename: &str, huff_codes: &HashMap<u8, Vec<bool>>, total_bits: u64) -> std::io::Result<()> {
    let mut file = std::fs::File::create(filename)?;

    // Écrire la longueur totale des données compressées en bits au début
    file.write_u64::<LittleEndian>(total_bits)?;

    for (key, value) in huff_codes {
        file.write_all(&[*key])?;
        file.write_u16::<LittleEndian>(value.len() as u16)?;

        let mut writer = BitWriter::new(&file);
        for &bit in value {
            writer.write(bit as u8)?;
        }
    }

    file.write_all(&[0u8])?; // Octet de séparation

    Ok(())
}

pub fn write_comp_data(filename: &str, compressed_data: Vec<bool>) -> io::Result<()> {
    // Ouvrir le fichier en mode d'ajout
    let file = std::fs::OpenOptions::new().append(true).open(filename)?;
    let mut writer = BitWriter::new(file);

    for &bit in &compressed_data {
        writer.write(bit as u8)?;
    }

    // Flusher les bits restants dans le buffer pour s'assurer que tout est écrit
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_decomp_data(output_path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = std::fs::File::create(output_path)?;
    file.write_all(data)?;
    Ok(())
}
