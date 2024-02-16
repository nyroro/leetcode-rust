
// Define a struct to represent the graph

struct Graph {
    adj_list: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    // Implement a function to add edges to the graph

    fn add_edge(&mut self, u: usize, v: usize, w: i32) {
        self.adj_list[u].push((v, w));
        self.adj_list[v].push((u, w));
    }

    // Implement a depth-first search function to find the minimum score

    fn dfs(&self, node: usize, vis: &mut Vec<bool>, mini: &mut i32) {
        vis[node] = true;
        for &(neighbor, weight) in &self.adj_list[node] {
            if !vis[neighbor] {
                *mini = (*mini).min(weight);
                self.dfs(neighbor, vis, mini);
            } else {
                *mini = (*mini).min(weight);
            }
        }
    }
}

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut graph = Graph { adj_list: vec![vec![]; n as usize + 1] };
        
        for road in &roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let w = road[2];
            graph.add_edge(u, v, w);
        }

        let mut vis = vec![false; n as usize + 1];
        let mut mini = i32::MAX;
        graph.dfs(1, &mut vis, &mut mini);

        mini

    }
}
