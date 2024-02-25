#[derive(PartialEq, Eq, Debug)]
pub enum HuffNode {
    Internal { weight: i32, left: Box<HuffNode>, right: Box<HuffNode> },
    Leaf { weight: i32, element: u8 },
}

impl HuffNode {
    pub(crate) fn new_leaf(element: u8, weight: i32) -> Self {
        HuffNode::Leaf { element, weight }
    }

    pub(crate) fn new_internal(left: Box<HuffNode>, right: Box<HuffNode>) -> Self {
        let weight = left.weight() + right.weight();
        HuffNode::Internal { weight, left, right }
    }

    pub(crate) fn weight(&self) -> i32 {
        match self {
            HuffNode::Internal { weight, .. } | HuffNode::Leaf { weight, .. } => *weight,
        }
    }

    pub(crate) fn is_leaf(&self) -> bool {
        matches!(self, HuffNode::Leaf { .. })
    }

    pub(crate) fn left_right(&self) -> Option<(&HuffNode, &HuffNode)> {
        match self {
            HuffNode::Internal { left, right, .. } => Some((left, right)),
            HuffNode::Leaf { .. } => None,
        }
    }

    pub(crate) fn element(&self) -> Option<u8> {
        match self {
            HuffNode::Leaf { element, .. } => Some(*element),
            HuffNode::Internal { .. } => None,
        }
    }

    pub(crate) fn huff_code(&self, element: u8, path: &mut Vec<bool>) -> Option<Vec<bool>> {
        if self.is_leaf() {
            if self.element().unwrap() == element {
                return Some(path.clone());
            }
            return None;
        }

        let (left, right) = self.left_right().unwrap();

        // Tentative de trouver le code dans le sous-arbre gauche
        path.push(false); // Ajoute un faux bit pour le chemin vers la gauche
        if let Some(code) = left.huff_code(element, path) {
            return Some(code);
        }
        path.pop(); // Important : supprime le dernier bit du chemin car il n'a pas mené à l'élément

        // Tentative de trouver le code dans le sous-arbre droit
        path.push(true); // Ajoute un vrai bit pour le chemin vers la droite
        let result = right.huff_code(element, path);
        path.pop(); // Supprime le dernier bit du chemin après l'appel récursif

        result
    }
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