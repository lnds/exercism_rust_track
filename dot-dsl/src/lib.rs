#[macro_use]
extern crate maplit;

pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                pub init: String,
                pub end: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(init: &str, end: &str) -> Self {
                    Edge {
                        init: init.to_string(),
                        end: end.to_string(),
                        attrs: hashmap![],
                    }
                }

                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|x| (x.0.to_string(), x.1.to_string()))
                        .collect::<HashMap<_, _>>();
                    self.clone()
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;


            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: hashmap![]
                    }
                }

                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|x| (x.0.to_string(), x.1.to_string()))
                        .collect::<HashMap<_, _>>();
                    self.clone()
                }

                pub fn get_attr(&mut self, name: &str) -> Option<&str> {
                    match self.attrs.get(name) {
                        None => None,
                        Some(s) => Some(s)
                    }
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap![],
            }
        }

        pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|x| (x.0.to_string(), x.1.to_string()))
                .collect::<HashMap<_, _>>();
            self.clone()
        }

        pub fn with_nodes(&mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
            self.nodes = nodes.to_vec();
            self.clone()
        }

        pub fn with_edges(&mut self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            self.edges = edges.to_vec();
            self.clone()
        }

        pub fn get_node(&self, name: &str) -> Option<graph_items::node::Node> {
            let n = self.nodes.iter().find(|n| n.name == name);
            match n {
                None => None,
                Some(n) => Some(n.clone()),
            }
        }
    }
}
