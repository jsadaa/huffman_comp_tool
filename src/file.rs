use std::collections::HashMap;
use crate::bit_writer::BitWriter;

pub(crate) fn write_header(filename: &str, huff_codes: &HashMap<u8, Vec<u8>>) -> std::io::Result<()> {
    use std::io::Write;
    use byteorder::{WriteBytesExt, LittleEndian};

    // Ouvrir le fichier en écriture
    let mut file = std::fs::File::create(filename)?;

    // Écrire chaque entrée de la table huff_codes dans le fichier
    for (key, value) in huff_codes {
        // Écrire la clé
        file.write_all(&[*key])?;

        // Écrire la longueur du vecteur en utilisant 4 octets (32 bits)
        file.write_u32::<LittleEndian>(value.len() as u32)?;

        // Écrire le vecteur lui-même
        file.write_all(value)?;
    }

    // Écrire un octet de séparation entre l'en-tête et le corps
    file.write_all(&[0u8])?;

    Ok(())
}

pub(crate) fn write_compressed_data(filename: &str, source: &str, huff_codes: &HashMap<u8, Vec<u8>>) -> std::io::Result<()> {
    // Ouvrir le fichier en mode d'ajout
    let file = std::fs::OpenOptions::new().append(true).open(filename)?;
    let mut writer = BitWriter::new(file);

    // Parcourir chaque caractère dans la source
    for byte in source.bytes() {
        // Obtenir le vecteur de code Huffman pour ce caractère
        if let Some(code) = huff_codes.get(&byte) {
            // Écrire le code de Huffman dans le fichier
            for &bit in code {
                writer.write(bit)?;    // writing bit by bit
            }
        }
    }

    // Flusher les bits restants dans le buffer
    writer.flush()?;
    Ok(())
}