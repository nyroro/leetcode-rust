
impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n];
        for i in 0..n {
            if edges[i] != -1 {
                graph[i as usize].push(edges[i as usize]);
            }
        }
        
        let mut visited: Vec<bool> = vec![false; n];
        let mut on_stack: Vec<bool> = vec![false; n];
        let mut max_cycle_len = -1;
        
        for i in 0..n {
            if !visited[i] {
                let mut stack: Vec<i32> = vec![];
                Solution::dfs(i as i32, &graph, &mut visited, &mut on_stack, &mut stack, &mut max_cycle_len);
            }
        }
        
        max_cycle_len

    }
    
    fn dfs(node: i32, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, on_stack: &mut Vec<bool>, stack: &mut Vec<i32>, max_cycle_len: &mut i32) {
        visited[node as usize] = true;
        on_stack[node as usize] = true;
        stack.push(node);
        
        for &neighbor in &graph[node as usize] {
            if !visited[neighbor as usize] {
                Solution::dfs(neighbor, graph, visited, on_stack, stack, max_cycle_len);
            } else if on_stack[neighbor as usize] {
                let cycle_len = stack.len() as i32 - stack.iter().position(|&x| x == neighbor).unwrap() as i32;
                *max_cycle_len = (*max_cycle_len).max(cycle_len);
            }
        }
        
        on_stack[node as usize] = false;
        stack.pop();
    }
}
