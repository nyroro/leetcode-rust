
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, bool)>> = vec![vec![]; n];
        for connection in &connections {
            let (a, b) = (connection[0] as usize, connection[1] as usize);
            graph[a].push((b, true));
            graph[b].push((a, false));
        }
        
        let mut count = 0;
        let mut visited = vec![false; n];
        
        fn dfs(node: usize, graph: &Vec<Vec<(usize, bool)>>, visited: &mut Vec<bool>, count: &mut i32) {
            visited[node] = true;
            for &(neighbor, direction) in &graph[node] {
                if !visited[neighbor] {
                    if direction {
                        *count += 1;
                    }
                    dfs(neighbor, graph, visited, count);
                }
            }
        }
        
        dfs(0, &graph, &mut visited, &mut count);
        count

    }
}
