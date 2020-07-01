use std::collections::HashMap;

use counter::Counter;
use rust_heap::heap::heapify;

use crate::node::{frequency, Node};

pub fn encode(message: Vec<u8>) {
    // Get the frequency of each unique byte in the file
    let frequencies: Counter<u8, usize> =
        message.clone().into_iter().collect::<Counter<u8, usize>>();

    let mut tree: Vec<Node> = frequencies
        .iter()
        .map(|(&k, &v)| Node::Leaf(k, v))
        .collect();

    heapify(&mut tree);

    while tree.len() > 1 {
        let left_node: Node = tree.pop().unwrap();
        let right_node: Node = tree.pop().unwrap();

        let left_frequency = frequency(&left_node);
        let right_frequency = frequency(&right_node);

        let subtree = Node::Branch(
            left_frequency + right_frequency,
            Box::new(left_node),
            Box::new(right_node),
        );

        tree.push(subtree);

        heapify(&mut tree);
    }

    // Dictionary to hold the abbreviated codes for each unique byte
    let mut ring: HashMap<u8, String> = HashMap::new();

    generate_codes(tree.pop().unwrap(), String::from(""), &mut ring);

    let encoded_message = message.into_iter().map(|k| ring.get(&k).unwrap()).collect::<Vec<_>>().join("");
}

fn generate_codes(node: Node, code: String, ring: &mut HashMap<u8, String>) {
    match node {
        Node::Leaf(byte, _) => {
            ring.insert(byte, code);
            return;
        }
        Node::Branch(_, left, right) => {
            let left_code: String = code.clone() + "0";
            let right_code: String = code + "1";

            generate_codes(*left, left_code, ring);
            generate_codes(*right, right_code, ring);
        }
    }
}
