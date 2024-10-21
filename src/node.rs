use std::slice::Iter;

#[derive(Clone, Debug)]
pub struct Node {
    pub num: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl Node {
    pub fn get_parent(&self) -> usize {
        self.parent.unwrap_or(0)
    }
}

const RADIX: u32 = 10;
// 1行ごとにNode1個の情報
// 先頭がNodeの序数
// それ以外がNodeの子
// 今回は枝に重みはつかない
pub fn generate_graph(contents: String) -> Vec<Node> {
    let mut graph = vec![];
    for node_info in contents.split('\n') {
        if node_info.starts_with("//") {
            continue;
        }
        let node_info: Vec<usize> = node_info
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| usize::from_str_radix(&c.to_string(), RADIX).unwrap())
            .collect();

        let mut node_info: Iter<usize> = node_info.iter();
        let num = node_info.next().map_or(0, |n| *n);
        let children: Vec<usize> = node_info.copied().collect();

        graph.push(Node {
            num,
            children,
            parent: None,
        })
    }
    for i in 0..graph.len() {
        if graph[i].parent.is_some() {
            println!("parent is already registered");
            continue;
        }

        let num = graph[i].num;

        for j in 0..i {
            let children = graph[j].children.clone();
            if children.contains(&num) {
                graph[i].parent = Some(j);
            }
        }
    }

    graph
}
