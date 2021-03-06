#[macro_use]
extern crate itertools;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    println!("input: {:?}", input);
    if input.is_empty() {
        return Some(vec![]);
    }
    if input.len() == 1 {
        if input[0].0 == input[0].1 {
            return Some(input.to_vec());
        }
        return None;
    }
    let dominoes: Vec<Domino> = input
        .iter()
        .enumerate()
        .map(|(i, &p)| Domino::new(i, p))
        .chain(input.iter().enumerate().map(|(i, &p)| Domino::rev(i, p)))
        .collect();

    let mut graph = DiGraph::<Domino, u32>::new();
    let indices = dominoes
        .iter()
        .map(|d| graph.add_node(d.clone()))
        .collect::<Vec<NodeIndex<_>>>();

    for (&x, &y) in iproduct!(indices.iter(), indices.iter()) {
        let a = &graph[x].clone();
        let b = &graph[y];
        if a.connect(b) {
            graph.add_edge(x, y, 1);
        }
    }

    for (&from, &to) in iproduct!(indices.iter(), indices.iter()) {
        for path in all_simple_paths::<Vec<NodeIndex>, _>(
            &graph,
            from,
            to,
            input.len() - 2,
            Some(input.len() - 1),
        ) {
            let dom_path: Vec<Domino> = path.iter().map(|&pi| &graph[pi]).cloned().collect();
            if dom_path.iter().map(|d| d.id).sorted().dedup().count() == input.len() {
                let doms = dom_path.iter().map(|d| d.pair).collect::<Vec<(u8, u8)>>();
                if doms[0].0 == doms[doms.len() - 1].1 {
                    return Some(doms);
                }
            }
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Domino {
    id: usize, // because could use multiple domino sets
    pair: (u8, u8),
}

impl Domino {
    fn new(id: usize, pair: (u8, u8)) -> Self {
        Domino { id, pair }
    }

    fn rev(id: usize, pair: (u8, u8)) -> Self {
        Domino {
            id,
            pair: (pair.1, pair.0),
        }
    }

    fn connect(&self, other: &Domino) -> bool {
        self.pair.1 == other.pair.0 && self.id != other.id
    }
}
