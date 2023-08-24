mod merkle_tree;

use std::path::Path;

fn main() {
    let dir_path = Path::new("demo/");
    let merkle_tree = merkle_tree::build_merkle_tree(dir_path);

    // Now you can use the Merkle tree for verification or any other purpose.
    println!("Root hash: {}", merkle_tree.hash);
}