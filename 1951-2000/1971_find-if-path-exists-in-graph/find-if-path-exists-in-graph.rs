
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut adjacency_list = vec![Vec::new(); n as usize];
        let mut visited = vec![false; n as usize];
        
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adjacency_list[u].push(v);
            adjacency_list[v].push(u);
        }
        
        Self::dfs(source as usize, destination as usize, &adjacency_list, &mut visited)
    }
    
    fn dfs(node: usize, destination: usize, adjacency_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> bool {
        if node == destination {
            return true;
        }
        
        visited[node] = true;
        
        for &next_node in &adjacency_list[node] {
            if !visited[next_node] && Self::dfs(next_node, destination, adjacency_list, visited) {
                return true;
            }
        }
        
        false

    }
}
