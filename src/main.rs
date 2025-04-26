use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
struct Metadata {
    to: String,
    label: String,
    timestamp: String,
}

#[derive(Debug, Clone)]
struct Edge {
    metadata: Metadata,
}

#[derive(Debug, Clone)]
struct Graph {
    adj_list: HashMap<String, Person>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    fn add_node(&mut self, p: Person) {
        self.adj_list.entry(p.name.to_string()).or_insert(p);
    }

    fn add_edge(&mut self, src: Person, md: Metadata) {
        if let Some(p) = self.adj_list.get_mut(&src.name) {
            p.edges.push(Edge { metadata: md});
        }
    }

    fn display(&self) {
        for (id, node) in &self.adj_list {
            print!("Node({}) => ", id);
            for edge in &node.edges {
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

    let n1 = Person {
        name: "John".to_string(),
        edges: vec![],
    };
    let n2 = Person {
        name: "Jane".to_string(),
        edges: vec![],
    };
    let n3 = Person {
        name: "Alice".to_string(),
        edges: vec![],
    };

    g.add_node(n1.clone());
    g.add_node(n2.clone());
    g.add_node(n3.clone());

    g.add_edge(
        n1.clone(),
        Metadata {
            to: n2.name.to_string(),
            label: "husband of".to_string(),
            timestamp: "01/01/1980".to_string(),
        },
    );
    g.add_edge(
        n2.clone(),
        Metadata {
            to: n3.name.to_string(),
            label: "mother of".to_string(),
            timestamp: "01/01/2000".to_string(),
        },
    );
    g.add_edge(
        n3.clone(),
        Metadata {
            to: n1.name.to_string(),
            label: "daughter of".to_string(),
            timestamp: "01/01/2000".to_string(),
        },
    );

    g.display();
}
