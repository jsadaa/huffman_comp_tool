use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::huffman::node::HuffNode;

pub fn build_tree(heap: &mut BinaryHeap<Reverse<HuffNode>>) -> HuffNode {
    while heap.len() > 1 {
        // Utilisation de pop pour extraire les deux noeuds les plus légers
        let small_node1: HuffNode = heap.pop().unwrap().0; // unwrap Reverse
        let small_node2: HuffNode = heap.pop().unwrap().0; // unwrap Reverse

        // Création d'un nouveau noeud interne
        let new_internal_node: HuffNode = HuffNode::new_internal(Box::new(small_node1), Box::new(small_node2));

        // On réintroduit le noeud interne dans le tas
        heap.push(Reverse(new_internal_node));
    }

    // Le dernier noeud restant dans le tas est l'arbre de Huffman complet
    heap.pop().unwrap().0
}