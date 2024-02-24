use std::collections::HashMap;
use std::io;
use std::io::{Write};
use byteorder::{LittleEndian, WriteBytesExt};
use crate::file::bit_writer::BitWriter;

pub fn write_header(filename: &str, huff_codes: &HashMap<u8, Vec<bool>>) -> std::io::Result<()> {

    // Ouvrir le fichier en écriture
    let mut file = std::fs::File::create(filename)?;

    // Écrire chaque entrée de la table huff_codes dans le fichier
    for (key, value) in huff_codes {
        // Écrire la clé
        file.write_all(&[*key])?;

        // Écrire la longueur du vecteur en utilisant 2 octets (16 bits)
        file.write_u16::<LittleEndian>(value.len() as u16)?;

        // Écrire le vecteur lui-même
        let mut writer = BitWriter::new(&file);
        for &bit in value {
            writer.write(bit as u8)?;
        }
    }

    // Écrire un octet de séparation entre l'en-tête et le corps
    file.write_all(&[0u8])?;

    Ok(())
}

pub fn write_comp_data(filename: &str, compressed_data: Vec<bool>) -> io::Result<()> {
    // Ouvrir le fichier en mode d'ajout
    let file = std::fs::OpenOptions::new().append(true).open(filename)?;
    let mut writer = BitWriter::new(file);

    for &bit in &compressed_data {
        writer.write(bit as u8)?;
    }

    // Flusher les bits restants dans le buffer
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_decomp_data(output_path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = std::fs::File::create(output_path)?;
    file.write_all(data)?;
    Ok(())
}
