mod node;
use std::{fs::File, io::Read};

use node::generate_graph;

use crate::node::Node;

const FILENAME: &str = "tree";
fn main() {
    println!("Depth First Search");

    let start = 0;
    let goal = 4;
    let answer = depth_first_search(start, goal);

    println!("answer: {answer:?}");
}

fn depth_first_search(start: usize, goal: usize) -> Vec<usize> {
    let tree: Vec<Node> = tree_from_file(FILENAME);
    let mut answer: Vec<usize> = vec![];
    let mut list: Vec<usize> = vec![start];
    let mut src;
    loop {
        if list.is_empty() {
            break;
        }

        // select
        src = list.remove(0);
        println!("select: {src}, list: {list:?}");

        if src == goal {
            println!("----- reached the goal!! -----");
            let mut p = goal;
            loop {
                if p == start {
                    answer.push(start);
                    break;
                }
                answer.push(p);
                p = tree[p].get_parent();
            }
            break;
        }

        //expand
        let leaves = tree[src].children.clone();
        println!("expand: {leaves:?}");

        list = leaves.iter().cloned().chain(list).collect();
        println!("generate: {list:?}");
    }
    answer // todo
}

fn tree_from_file(filename: &str) -> Vec<Node> {
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("invalid file format");
    generate_graph(contents)
}
