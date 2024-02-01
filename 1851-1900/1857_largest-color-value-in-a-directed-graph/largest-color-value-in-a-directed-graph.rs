


impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let mut graph = vec![vec![]; n];
        let mut colors_count = vec![vec![0; 26]; n];
        let mut visited = vec![0; n];
        let mut on_path = vec![0; n];
        let mut result = 0;

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
        }

        let colors: Vec<u8> = colors.bytes().map(|c| (c - b'a') as u8).collect();

        for u in 0..n {
            if !Solution::dfs(u, &graph, &colors, &mut colors_count, &mut visited, &mut on_path, &mut result) {
                return -1;
            }
        }

        result

    }

    fn dfs(u: usize, graph: &Vec<Vec<usize>>, colors: &Vec<u8>, colors_count: &mut Vec<Vec<i32>>, visited: &mut Vec<i32>, on_path: &mut Vec<i32>, result: &mut i32) -> bool {
        if visited[u] == 1 {
            return on_path[u] == 0;
        }

        visited[u] = 1;
        on_path[u] = 1;

        for &v in &graph[u] {
            if !Solution::dfs(v, graph, colors, colors_count, visited, on_path, result) {
                return false;
            }
            for i in 0..26 {
                colors_count[u][i] = colors_count[u][i].max(colors_count[v][i]);
            }
        }

        colors_count[u][colors[u] as usize] += 1;
        *result = result.max(colors_count[u][colors[u] as usize]);

        on_path[u] = 0;
        true

    }
}
