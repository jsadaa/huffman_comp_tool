use std::collections::HashMap;
use std::fs::File;

use crate::compression::compressor;
use crate::file::{counter, reader, writer};
use crate::huffman::tree::HuffTree;

pub fn compress(source: &Vec<u8>) -> (HashMap<u8, Vec<bool>>, Vec<bool>, u64) {
    let bytes_map: HashMap<u8, i32> = counter::count_bytes(source);
    let huff_tree: HuffTree = HuffTree::new(bytes_map);
    let prefix_code_table = huff_tree.get_prefix_code_table();

    let (compressed_data, total_bits) = compressor::compress(source, &prefix_code_table);

    (prefix_code_table, compressed_data, total_bits)
}

pub fn decompress(file: File) -> Result<Vec<u8>, std::io::Error> {
    let (prefix_code_table, total_bits, start_pos) = reader::read_header(&file)?;
    compressor::decompress(&file, &prefix_code_table, start_pos, total_bits as usize)
}

pub fn write_comp_file(filename: &str, comp_data: Vec<bool>, huff_codes: HashMap<u8, Vec<bool>>, total_bits: u64) -> Result<(), std::io::Error> {
    if let Err(e) = writer::write_header(filename, &huff_codes, total_bits) {
        eprintln!("Error while writing header: {}", e);
    }

    if let Err(e) = writer::write_comp_file(filename, comp_data) {
        eprintln!("Error while writing compressed data: {}", e);
    }

    Ok(())
}

pub fn write_dec_file(output_path: &str, data: &[u8]) -> Result<(), std::io::Error> {
    if let Err(e) = writer::write_dec_file(output_path, data) {
        eprintln!("Error while writing decompressed data: {}", e);
    }

    Ok(())
}
