use std::fmt::Display;

pub mod binary_tree;
pub mod visualize;

use binary_tree::{BinarySearchTree, node::Node};
use bodium_core::data_types::KvError;

fn main() {
    let mut tree: BinarySearchTree<u32, String> = BinarySearchTree::new();
    println!("tree is empty {}", tree.is_empty());

    tree.put(18, String::from("Na"));
    tree.put(25, String::from("James"));
    tree.put(16, String::from("Simon"));
    tree.put(27, String::from("Andy"));
    tree.put(21, String::from("Liz"));
    tree.put(9, String::from("Helen"));
    tree.put(12, String::from("Tom"));
    tree.put(11, String::from("Sally"));
    tree.put(14, String::from("Bella"));
    tree.put(7, String::from("Cee"));
    tree.put(3, String::from("Colin"));
    tree.put(8, String::from("Nicolas"));
    tree.put(32, String::from("George"));
    tree.put(23, String::from("Kate"));
    tree.put(26, String::from("Ed"));
    tree.put(19, String::from("Dean"));
    tree.put(17, String::from("Joanna"));
    println!("Tree size: {}", tree.size());
    println!("tree is empty {}", tree.is_empty());

    let _ = visualize::draw_tree_graph("tree_view.png", &tree).map_err(KvError::from);

    println!("BST contains key 3 - {}", tree.contains(3));
    tree.contains(3);

    display(tree.get(&9), "get key");
    display(tree.get(&18), "get key");

    for key in tree.iter_keys() {
        print!("{}, ", key);
    }
    println!();

    for key in tree.keys().iter() {
        print!("{}, ", key);
    }
    println!();

    for node in tree.nodes().iter() {
        print!("{}", node);
    }
    println!();

    display_node(tree.floor(18), "floor of key 18");
    display_node(tree.floor(15), "floor of key 15");
    display_node(tree.floor(22), "floor of key 22");

    println!("*************  Select *****************");
    display(tree.select(16).as_deref(), "select 16: ");
    println!("rank_iter 32: {:?}", tree.rank(32));
    println!();

    display(tree.get(&3), "get key");
    display(tree.get(&11), "get key");
    println!("Delete keys 3, 7 & 25.");
    tree.delete(3);
    tree.delete(7);

    tree.delete(25);

    println!("Tree size: {}", tree.size());

    println!("\n{:?}", tree.keys());
    // println!("{tree:#?}");
}

fn display<K: Display, V: Display>(opt: Option<&Node<K, V>>, msg: &str) {
    if opt.is_some() {
        println!("{}: {}", msg, opt.unwrap());
    } else {
        println!("{}: None", msg);
    }
}

fn display_node<K: Display>(opt: Option<K>, msg: &str) {
    if opt.is_some() {
        println!("{}: {}", msg, opt.unwrap());
    } else {
        println!("{}: None", msg);
    }
}
