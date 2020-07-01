use std::cmp::Ordering;

#[derive(Eq, Clone)]
pub enum Node {
    Leaf(u8, usize),
    Branch(usize, Box<Node>, Box<Node>),
}

pub fn frequency(node: &Node) -> usize {
    match node {
        Node::Leaf(_, frequency) => *frequency,
        Node::Branch(frequency, _, _) => *frequency,
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.frequency.cmp(&other.frequency)
        frequency(&self).cmp(&frequency(&other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        frequency(&self) == frequency(&other)
    }
}
