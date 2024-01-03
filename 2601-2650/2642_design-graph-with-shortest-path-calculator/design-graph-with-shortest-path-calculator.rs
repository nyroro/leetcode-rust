
struct Graph {
    n: i32,
    edges: Vec<Vec<i32>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        Graph {
            n,
            edges,
        }
    }
    
    fn add_edge(&mut self, edge: Vec<i32>) {
        self.edges.push(edge);
    }
    
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut dist = vec![std::i32::MAX; self.n as usize];
        dist[node1 as usize] = 0;
        
        for _ in 0..self.n - 1 {
            for edge in &self.edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let cost = edge[2];
                
                if dist[u] != std::i32::MAX && dist[u] + cost < dist[v] {
                    dist[v] = dist[u] + cost;
                }
            }
        }
        
        if dist[node2 as usize] == std::i32::MAX {
            -1

        } else {
            dist[node2 as usize]
        }
    }
}
