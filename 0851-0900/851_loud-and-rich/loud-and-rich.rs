
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut graph = vec![Vec::new(); n];
        let mut answer = vec![-1; n];
        
        for edge in richer {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[v].push(u);
        }
        
        for i in 0..n {
            if answer[i] == -1 {
                Self::dfs(i, &graph, &quiet, &mut answer);
            }
        }
        
        answer

    }
    
    fn dfs(u: usize, graph: &Vec<Vec<usize>>, quiet: &Vec<i32>, answer: &mut Vec<i32>) -> i32 {
        if answer[u] != -1 {
            return answer[u];
        }
        
        answer[u] = u as i32;
        
        for &v in &graph[u] {
            let candidate = Self::dfs(v, graph, quiet, answer);
            if quiet[candidate as usize] < quiet[answer[u] as usize] {
                answer[u] = candidate;
            }
        }
        
        answer[u]
    }
}
