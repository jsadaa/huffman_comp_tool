use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::huffman::node::HuffNode;

pub struct HuffTree {
    // keep the root of the tree even if it is not used, plan to manipulate the tree later
    #[allow(dead_code)]
    pub(crate) root: HuffNode,
    pub(crate) prefix_code_table: HashMap<u8, Vec<bool>>,
}

impl HuffTree {
    pub fn new(bytes_map: HashMap<u8, i32>) -> HuffTree {
        let heap: BinaryHeap<Reverse<HuffNode>> = BinaryHeap::new();
        let mut tree_generator = TreeGenerator::new(heap, bytes_map);
        let root: HuffNode = tree_generator.build_tree();
        let prefix_code_table: HashMap<u8, Vec<bool>> = tree_generator.build_prefix_code_table(&root);

        HuffTree {
            root,
            prefix_code_table,
        }
    }

    pub fn get_prefix_code_table(&self) -> HashMap<u8, Vec<bool>> {
        self.prefix_code_table.clone()
    }
}

struct TreeGenerator {
    heap: BinaryHeap<Reverse<HuffNode>>,
    bytes_map: HashMap<u8, i32>,
}

impl TreeGenerator {
    pub fn new(heap: BinaryHeap<Reverse<HuffNode>>, bytes_map: HashMap<u8, i32>) -> TreeGenerator {
        TreeGenerator {
            heap,
            bytes_map,
        }
    }

    pub fn build_tree(&mut self) -> HuffNode {
        for (el, freq) in &self.bytes_map {
            self.heap.push(Reverse(HuffNode::new_leaf(*el, *freq)))
        }

        while self.heap.len() > 1 {
            let small_node1: HuffNode = self.heap.pop().unwrap().0;
            let small_node2: HuffNode = self.heap.pop().unwrap().0;

            let new_internal_node: HuffNode = HuffNode::new_internal(Box::new(small_node1), Box::new(small_node2));

            self.heap.push(Reverse(new_internal_node));
        }

        self.heap.pop().unwrap().0
    }

    fn build_prefix_code_table(&mut self, root_node: &HuffNode) -> HashMap<u8, Vec<bool>> {
        let mut path: Vec<bool> = Vec::new();
        let mut huff_codes: HashMap<u8, Vec<bool>> = HashMap::new();

        for el in self.bytes_map.keys() {
            let code: Option<Vec<bool>> = root_node.huff_code(el, &mut path);
            if let Some(code) = code {
                huff_codes.insert(*el, code);
            }
            path.clear();
        }

        huff_codes
    }
}

