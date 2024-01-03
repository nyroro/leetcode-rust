
struct Graph {
    adj_list: Vec<Vec<i32>>,
}

impl Graph {
    fn new(n: i32, edges: &Vec<Vec<i32>>) -> Self {
        let mut adj_list = vec![vec![]; n as usize];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj_list[u].push(v as i32);
            adj_list[v].push(u as i32);
        }
        Graph { adj_list }
    }

    fn dfs(&self, v: usize, visited: &mut Vec<bool>, component: &mut Vec<i32>) {
        visited[v] = true;
        component.push(v as i32);
        for &u in &self.adj_list[v] {
            if !visited[u as usize] {
                self.dfs(u as usize, visited, component);
            }
        }
    }

    fn is_complete(&self, component: &Vec<i32>) -> bool {
        for &u in component {
            for &v in component {
                if u != v && !self.adj_list[u as usize].contains(&v) {
                    return false;
                }
            }
        }
        true

    }
}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let graph = Graph::new(n, &edges);
        let mut visited = vec![false; n as usize];
        let mut count = 0;
        for v in 0..n as usize {
            if !visited[v] {
                let mut component = Vec::new();
                graph.dfs(v, &mut visited, &mut component);
                if graph.is_complete(&component) {
                    count += 1;
                }
            }
        }
        count

    }
}
