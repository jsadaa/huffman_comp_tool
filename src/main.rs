use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use crate::huffman_tree::HuffNode;

mod parsing;
mod huffman_tree;
mod file;
mod bit_writer;

fn main() {
    let file = std::fs::read_to_string("./135-0.txt");

    if file.is_err() {
        eprintln!("Error : {}", file.err().unwrap());
        std::process::exit(1);
    }

    let source = file.unwrap();
    let map: HashMap<u8, i32> = parsing::count_bytes(&source);
    let mut heap: BinaryHeap<Reverse<HuffNode>> = BinaryHeap::new();

    for (el, freq) in &map {
        heap.push(Reverse(HuffNode::new_leaf(*el, *freq) ))
    }

    let huff_tree: HuffNode = huffman_tree::build_tree(&mut heap);
    let mut path: Vec<bool> = Vec::new();
    let mut huff_codes: HashMap<u8, Vec<bool>> = HashMap::new();

    for (el, _) in map {
        let code: Option<Vec<bool>> = huff_tree.huff_code(el, &mut path);
        if let Some(code) = code {
            huff_codes.insert(el, code);
        }
        path.clear();
    }

    if let Err(e) = file::write_header("output.bin", &huff_codes) {
        eprintln!("Error while writing header: {}", e);
        return;
    }

    if let Err(e) = file::write_compressed_data("output.bin", &source, &huff_codes) {
        eprintln!("Error while writing compressed data: {}", e);
    }

    let old_size = source.len();
    let new_size = std::fs::metadata("output.bin").unwrap().len() as usize;

    println!("Original Size: {} bytes", old_size);
    println!("Compressed Size: {} bytes", new_size);
}
