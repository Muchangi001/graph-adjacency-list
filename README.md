
---

# Graph with Metadata in Rust

This Rust project implements a simple **directed graph** where each edge holds additional metadata like `label`, `to`, and `timestamp`.

It uses:
- `HashMap` for the **adjacency list**
- `Vec<Edge>` to hold outgoing edges per node
- Custom `Metadata` struct to store detailed edge info

## 📦 Features

- Add nodes to the graph
- Add edges with metadata
- Display the graph structure in a readable format

## 📚 Data Structures

```rust
struct Metadata {
    to: String,
    label: String,
    timestamp: String,
}

struct Edge {
    metadata: Metadata,
}

struct Graph {
    adj_list: HashMap<String, Vec<Edge>>,
}
```

## 🛠️ How It Works

1. **Create a Graph:**
   ```rust
   let mut g = Graph::new();
   ```

2. **Add Nodes:**
   ```rust
   g.add_node("A");
   g.add_node("B");
   g.add_node("C");
   ```

3. **Add Edges with Metadata:**
   ```rust
   g.add_edge("A", Metadata {
       to: "B".to_string(),
       label: "wife".to_string(),
       timestamp: "01/01/1980".to_string(),
   });
   ```

4. **Display the Graph:**
   ```rust
   g.display();
   ```

## 🧪 Sample Output

```
Node(A) =>  Edge(to -> B, label : wife, timestamp : 01/01/1980)
Node(B) =>  Edge(to -> C, label : daughter, timestamp : 01/01/2000)
Node(C) =>  Edge(to -> A, label : father, timestamp : 01/01/2000)
```

## 📎 Use Cases

- Representing family trees
- Tracking time-based relationships
- Any domain where edges carry rich metadata (e.g., transactions, message logs)

---