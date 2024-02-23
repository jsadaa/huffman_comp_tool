use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug)]
pub enum HuffNode {
    Internal { weight: i32, left: Box<HuffNode>, right: Box<HuffNode> },
    Leaf { weight: i32, element: u8 },
}

impl HuffNode {
    // Constructeur pour les feuilles
    pub(crate) fn new_leaf(element: u8, weight: i32) -> Self {
        HuffNode::Leaf { element, weight }
    }

    // Constructeur pour les noeuds internes
    pub(crate) fn new_internal(left: Box<HuffNode>, right: Box<HuffNode>) -> Self {
        let weight = left.weight() + right.weight();
        HuffNode::Internal { weight, left, right }
    }

    // Retourne le poids du noeud
    pub(crate) fn weight(&self) -> i32 {
        match self {
            HuffNode::Internal { weight, .. } | HuffNode::Leaf { weight, .. } => *weight,
        }
    }

    // Teste si le noeud est une feuille
    pub(crate) fn is_leaf(&self) -> bool {
        matches!(self, HuffNode::Leaf { .. })
    }

    // Retourne les fils gauche et droite d'un noeud interne
    pub(crate) fn left_right(&self) -> Option<(&HuffNode, &HuffNode)> {
        match self {
            HuffNode::Internal { left, right, .. } => Some((left, right)),
            HuffNode::Leaf { .. } => None,
        }
    }

    // Retourne l'élément d'une feuille
    pub(crate) fn element(&self) -> Option<u8> {
        match self {
            HuffNode::Leaf { element, .. } => Some(*element),
            HuffNode::Internal { .. } => None,
        }
    }

    // Retourne le code de Huffman associé à un élément
    pub(crate) fn huff_code(&self, element: u8, path: &mut Vec<bool>) -> Option<Vec<bool>> {
        if self.is_leaf() {
            if self.element().unwrap() == element {
                return Some(path.clone());
            }
            return None;
        }

        let (left, right) = self.left_right().unwrap();
        path.push(false);

        let mut left_path = path.clone();
        let mut right_path = path.clone();

        let left_code = left.huff_code(element, &mut left_path);
        if left_code.is_some() {
            return left_code;
        }

        path.pop();
        path.push(true);

        let right_code = right.huff_code(element, &mut right_path);
        if right_code.is_some() {
            return right_code;
        }

        None
    }

    // retourne les huff codes pour les bits (bool) pour la décompression
}

// Trait Ord nécessaire pour l'utilisation dans un BinaryHeap
impl Ord for HuffNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight().cmp(&other.weight())
    }
}

impl PartialOrd for HuffNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Fonction de construction de l'arbre de Huffman
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