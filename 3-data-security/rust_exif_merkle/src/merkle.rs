use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleNode {
    pub hash: Vec<u8>,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    pub fn new(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize().to_vec();
        
        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }

    pub fn from_children(left: MerkleNode, right: MerkleNode) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&left.hash);
        hasher.update(&right.hash);
        let hash = hasher.finalize().to_vec();

        MerkleNode {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn save_to_file(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filepath, json)?;
        Ok(())
    }

    pub fn load_from_file(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(filepath)?;
        let node: MerkleNode = serde_json::from_str(&json)?;
        Ok(node)
    }

    pub fn verify(&self, data: &[Vec<u8>]) -> bool {
        // Rebuild a new tree from the data
        if let Some(new_tree) = build_merkle_tree(data.to_vec()) {
            // Compare the root hashes
            self.hash == new_tree.hash
        } else {
            false
        }
    }
}

pub fn build_merkle_tree(leaves: Vec<Vec<u8>>) -> Option<MerkleNode> {
    if leaves.is_empty() {
        return None;
    }

    let mut nodes: Vec<MerkleNode> = leaves.iter()
        .map(|data| MerkleNode::new(data))
        .collect();

    while nodes.len() > 1 {
        let mut new_nodes = Vec::new();
        for chunk in nodes.chunks(2) {
            match chunk {
                [left, right] => {
                    new_nodes.push(MerkleNode::from_children(
                        left.clone(),
                        right.clone()
                    ));
                }
                [left] => {
                    // If odd number of nodes, duplicate the last one
                    new_nodes.push(MerkleNode::from_children(
                        left.clone(),
                        left.clone()
                    ));
                }
                _ => unreachable!(),
            }
        }
        nodes = new_nodes;
    }

    Some(nodes.pop().unwrap())
} 