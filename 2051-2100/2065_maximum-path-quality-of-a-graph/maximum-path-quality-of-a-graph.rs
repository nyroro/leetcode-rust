


struct Graph {
    n: usize,
    values: Vec<i32>,
    adj_list: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    fn new(n: usize, values: Vec<i32>) -> Self {
        Graph {
            n,
            values,
            adj_list: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, wt: i32) {
        self.adj_list[u].push((v, wt));
        self.adj_list[v].push((u, wt));
    }

    fn maximal_path_quality(&self, max_time: i32) -> i32 {
        let mut ans = 0;
        let mut vis = vec![0; self.n];
        let mut sum = 0;
        self.dfs(0, max_time, &mut vis, &mut sum, &mut ans);
        ans

    }

    fn dfs(&self, node: usize, max_time: i32, vis: &mut Vec<i32>, sum: &mut i32, ans: &mut i32) {
        vis[node] += 1;
        if vis[node] == 1 {
            *sum += self.values[node];
        }
        if node == 0 {
            *ans = (*ans).max(*sum);
        }
        for &(v, wt) in &self.adj_list[node] {
            if wt <= max_time {
                self.dfs(v, max_time - wt, vis, sum, ans);
            }
        }
        vis[node] -= 1;
        if vis[node] == 0 {
            *sum -= self.values[node];
        }
    }
}

impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let n = values.len();
        let mut graph = Graph::new(n, values);
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let wt = edge[2];
            graph.add_edge(u, v, wt);
        }
        graph.maximal_path_quality(max_time)
    }
}
