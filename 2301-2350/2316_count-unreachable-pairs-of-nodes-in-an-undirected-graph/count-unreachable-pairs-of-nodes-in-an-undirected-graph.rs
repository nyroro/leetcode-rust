
struct Graph {
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize, edges: &Vec<Vec<usize>>) -> Graph {
        let mut adj_list = vec![Vec::new(); n];
        for edge in edges {
            adj_list[edge[0]].push(edge[1]);
            adj_list[edge[1]].push(edge[0]);
        }
        Graph { adj_list }
    }

    fn dfs(&self, visited: &mut Vec<bool>, curr_node: usize) -> usize {
        visited[curr_node] = true;
        let mut node_count = 1;
        for &adj_node in &self.adj_list[curr_node] {
            if !visited[adj_node] {
                node_count += self.dfs(visited, adj_node);
            }
        }
        node_count

    }
}
