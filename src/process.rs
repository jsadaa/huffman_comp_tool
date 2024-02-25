use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::compression::compressor;
use crate::file::{counter, reader, writer};
use crate::huffman::node::HuffNode;
use crate::huffman::tree::build_tree;

pub fn compress(source: &str) -> (HashMap<u8, Vec<bool>>, Vec<bool>, u64) {
    let huff_codes = gen_prefix_code_tab(source);
    let (compressed_data, total_bits) = compressor::compress(source, &huff_codes);
    (huff_codes, compressed_data, total_bits)
}

pub fn decompress(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let (huff_codes, total_bits, start_pos) = reader::read_header(file_path)?;
    compressor::decompress(file_path, &huff_codes, start_pos, total_bits as usize)
}

fn gen_prefix_code_tab(source: &str) -> HashMap<u8, Vec<bool>> {
    let map: HashMap<u8, i32> = counter::count_bytes(source);
    let mut heap: BinaryHeap<Reverse<HuffNode>> = BinaryHeap::new();

    for (el, freq) in &map {
        heap.push(Reverse(HuffNode::new_leaf(*el, *freq)))
    }

    let huff_tree: HuffNode = build_tree(&mut heap);
    let mut path: Vec<bool> = Vec::new();
    let mut huff_codes: HashMap<u8, Vec<bool>> = HashMap::new();

    for (el, _) in map {
        let code: Option<Vec<bool>> = huff_tree.huff_code(el, &mut path);
        if let Some(code) = code {
            huff_codes.insert(el, code);
        }
        path.clear();
    }

    huff_codes
}

pub fn write_comp_file(filename: &str, comp_data: Vec<bool>, huff_codes: HashMap<u8, Vec<bool>>, total_bits: u64) -> Result<(), std::io::Error> {
    if let Err(e) = writer::write_header(filename, &huff_codes, total_bits) {
        eprintln!("Error while writing header: {}", e);
    }

    if let Err(e) = writer::write_comp_data(filename, comp_data) {
        eprintln!("Error while writing compressed data: {}", e);
    }

    Ok(())
}

pub fn write_dec_file(output_path: &str, data: &[u8]) -> Result<(), std::io::Error> {
    if let Err(e) = writer::write_decomp_data(output_path, data) {
        eprintln!("Error while writing decompressed data: {}", e);
    }

    Ok(())
}
