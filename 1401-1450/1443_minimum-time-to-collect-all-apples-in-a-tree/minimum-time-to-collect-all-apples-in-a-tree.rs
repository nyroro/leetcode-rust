
impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut adj_list = vec![Vec::new(); n as usize];
        let mut visited = vec![false; n as usize];
        
        // 构建邻接表

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj_list[u].push(v);
            adj_list[v].push(u);
        }
        
        // 深度优先搜索

        fn dfs(v: usize, adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, has_apple: &Vec<bool>) -> i32 {
            visited[v] = true;
            let mut path_len = 0;
            
            for &u in &adj_list[v] {
                if !visited[u] {
                    path_len += dfs(u, adj_list, visited, has_apple);
                }
            }
            
            if v != 0 && (has_apple[v] || path_len > 0) {
                path_len += 2;
            }
            
            path_len

        }
        
        dfs(0, &adj_list, &mut visited, &has_apple)
    }
}
