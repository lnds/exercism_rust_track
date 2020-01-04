use itertools::Itertools;
use petgraph::algo::{all_simple_paths, is_cyclic_directed};
use petgraph::graph::{DiGraph, NodeIndex};

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    println!("input: {:?}", input);
    if input.is_empty() {
        Some(vec![])
    } else if input.len() == 1 {
        if input[0].0 == input[0].1 {
            Some(input.to_vec())
        } else {
            None
        }
    } else {
        let dominoes: Vec<Domino> = input
            .iter()
            .enumerate()
            .map(|(i, &p)| Domino::new(i, p))
            .chain(input.iter().enumerate().map(|(i, &p)| Domino::rev(i, p)))
            .collect();
        println!("dominoes = {:?}", dominoes);

        let mut graph = DiGraph::<Domino, u32>::new();
        let indices = dominoes.iter().map(|d|graph.add_node(d.clone())).collect::<Vec<NodeIndex<_>>>();
        for index in indices.iter() {
            let domino: &Domino = &graph[*index].clone();
            for other_index in indices.iter() {
                let other_domino: &Domino = &graph[*other_index];
                if *domino != *other_domino {
                    if domino.connect(&other_domino) {
                        graph.add_edge(*index, *other_index, 1);
                    }
                }
            }
        }

        if is_cyclic_directed(&graph) {
            for &from in indices.iter() {
                for &to in indices.iter() {
                    if from == to {
                        continue;
                    }
                    for path in all_simple_paths::<Vec<NodeIndex>, _>(
                        &graph,
                        from,
                        to,
                        input.len() - 2,
                        Some(input.len() - 1),
                    ) {
                        let dom_path: Vec<Domino> =
                            path.iter().map(|&pi| &graph[pi]).cloned().collect();
                        if dom_path.iter().map(|d| d.id).sorted().dedup().count() == input.len() {
                            println!("dom path => {:?}", dom_path);
                            let doms = dom_path
                                .iter()
                                .map(|d| d.pair)
                                .collect::<Vec<(u8, u8)>>();
                            if doms[0].0 == doms[doms.len() - 1].1 {
                                return Some(doms);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Domino{id:usize, pair:(u8, u8)}

impl Domino {
    fn new(id: usize, pair: (u8, u8)) -> Self {
        Domino{id, pair}
    }

    fn rev(id: usize, pair: (u8, u8)) -> Self {
        Domino{id, pair: (pair.1, pair.0)}
    }

    fn connect(&self, other: &Domino) -> bool {
        self.pair.1 == other.pair.0 && self.id != other.id
    }
}
