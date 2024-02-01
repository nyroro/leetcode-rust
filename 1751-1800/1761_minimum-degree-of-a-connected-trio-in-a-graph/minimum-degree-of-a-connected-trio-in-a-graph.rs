
impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize; // Convert n to usize

        let mut degree = vec![0; n + 1];
        let mut graph = vec![vec![false; n + 1]; n + 1];
        
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u][v] = true;
            graph[v][u] = true;
            degree[u] += 1;
            degree[v] += 1;
        }
        
        let mut min_degree = std::i32::MAX;
        
        for i in 1..=n {
            for j in (i + 1)..=n {
                if graph[i][j] {
                    for k in (j + 1)..=n {
                        if graph[i][k] && graph[j][k] {
                            let trio_degree = degree[i] + degree[j] + degree[k] - 6;
                            min_degree = min_degree.min(trio_degree);
                        }
                    }
                }
            }
        }
        
        if min_degree == std::i32::MAX {
            -1

        } else {
            min_degree

        }
    }
}
