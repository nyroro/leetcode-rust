
impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut graph = vec![vec![]; n as usize + 1];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        
        let mut visited = vec![false; n as usize + 1];
        visited[1] = true;
        
        let mut probability = 1.0;
        let result = Self::dfs(1, t, target, &graph, &mut visited, &mut probability);
        
        if result {
            probability

        } else {
            0.0

        }
    }
    
    fn dfs(
        node: usize,
        time: i32,
        target: i32,
        graph: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        probability: &mut f64,
    ) -> bool {
        if time == 0 {
            return node == target as usize;
        }
        
        let mut count = 0;
        for &next in &graph[node] {
            if !visited[next] {
                count += 1;
            }
        }
        
        if count == 0 {
            return node == target as usize;
        }
        
        for &next in &graph[node] {
            if !visited[next] {
                visited[next] = true;
                *probability *= 1.0 / count as f64;
                if Self::dfs(next, time - 1, target, graph, visited, probability) {
                    return true;
                }
                *probability *= count as f64;
                visited[next] = false;
            }
        }
        
        false

    }
}
