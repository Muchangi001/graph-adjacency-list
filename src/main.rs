use std::collections::HashMap;

#[derive(Debug)]
struct Metadata {
    to: String,
    label: String,
    timestamp: String,
}

#[derive(Debug)]
struct Edge {
    metadata: Metadata,
}

#[derive(Debug)]
struct Graph {
    adj_list: HashMap<String, Vec<Edge>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    fn add_node(&mut self, n: &str) {
        self.adj_list.entry(n.to_string()).or_insert(vec![]);
    }

    fn add_edge(&mut self, src: &str, md: Metadata) {
        if let Some(v) = self.adj_list.get_mut(src) {
            v.push(Edge { metadata: md });
        }
    }

    fn display(&self) {
        for (node, edges) in &self.adj_list {
            print!("Node({}) => ", node);
            for edge in edges {
                println!(
                    "Edge(to -> {}, label : {}, timestamp : {})",
                    edge.metadata.to, edge.metadata.label, edge.metadata.timestamp
                );
            }
        }
    }
}

fn main() {
    let mut g = Graph::new();

    let n1 = "A";
    let n2 = "B";
    let n3 = "C";

    g.add_node(n1);
    g.add_node(n2);
    g.add_node(n3);

    g.add_edge(
        n1,
        Metadata {
            to: n2.to_string(),
            label: "wife".to_string(),
            timestamp: "01/01/1980".to_string(),
        },
    );
    g.add_edge(
        n2,
        Metadata {
            to: n3.to_string(),
            label: "daughter".to_string(),
            timestamp: "01/01/2000".to_string(),
        },
    );
    g.add_edge(
        n3,
        Metadata {
            to: n1.to_string(),
            label: "father".to_string(),
            timestamp: "01/01/2000".to_string(),
        },
    );

    g.display();
}
