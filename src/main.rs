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
    let mut open_list: Vec<usize> = vec![start];
    let mut close_list: Vec<usize> = vec![];
    let mut src;
    loop {
        if open_list.is_empty() {
            break;
        }

        // select
        src = open_list.remove(0);
        close_list.push(src);
        println!("select: {src}, open_list: {open_list:?}, close_list: {close_list:?}");

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
        let leaves: Vec<usize> = tree[src]
            .children
            .clone()
            .iter()
            .cloned()
            .filter(|leaf| !(close_list.contains(leaf) || open_list.contains(leaf)))
            .collect();
        println!("expand: {leaves:?}");

        open_list = leaves.into_iter().chain(open_list).collect();
        println!("generate: {open_list:?}");
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
