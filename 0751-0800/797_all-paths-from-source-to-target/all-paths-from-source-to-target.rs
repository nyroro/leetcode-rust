
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        let target = (graph.len() - 1) as i32;
        
        Self::dfs(0, target, &graph, &mut path, &mut result);
        
        result

    }
    
    fn dfs(node: i32, target: i32, graph: &Vec<Vec<i32>>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        path.push(node);
        
        if node == target {
            result.push(path.clone());
        } else {
            for &next_node in &graph[node as usize] {
                Self::dfs(next_node, target, graph, path, result);
            }
        }
        
        path.pop();
    }
}
