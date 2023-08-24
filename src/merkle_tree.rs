use crypto_hash::{Algorithm, hex_digest};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Define the basic Merkle tree data structure
#[derive(Clone)]
pub struct MerkleNode {
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
    pub hash: String,
}

impl MerkleNode {
    pub fn new(hash: String) -> MerkleNode {
        MerkleNode {
            left: None,
            right: None,
            hash,
        }
    }
}

pub fn compute_hash(data: &[u8]) -> String {
    hex_digest(Algorithm::SHA256, data)
}

pub fn hash_file(path: &Path) -> String {
    let mut file = File::open(path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    compute_hash(&buffer)
}

pub fn build_merkle_tree(dir_path: &Path) -> MerkleNode {
    if dir_path.is_file() {
        return MerkleNode::new(hash_file(dir_path));
    }

    let mut children = Vec::new();
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            children.push(MerkleNode::new(hash_file(path)));
        }
    }

    while children.len() > 1 {
        let mut parents = Vec::new();
        for pair in children.chunks(2) {
            let left = pair[0].clone();
            let right = pair.get(1).cloned().unwrap_or_else(|| left.clone());
            let combined_hash = format!("{}{}", left.hash, right.hash);
            parents.push(MerkleNode::new(compute_hash(combined_hash.as_bytes())));
        }
        children = parents;
    }

    children.pop().expect("Root node not found")
}