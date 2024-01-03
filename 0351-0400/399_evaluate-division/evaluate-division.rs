
use std::collections::HashMap;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        // Step 1: Create an empty graph

        let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();
        
        // Step 2: Build the graph

        for i in 0..equations.len() {
            let equation = &equations[i];
            let value = values[i];
            let a = &equation[0];
            let b = &equation[1];
            
            graph.entry(a.to_string()).or_insert(HashMap::new()).insert(b.to_string(), value);
            graph.entry(b.to_string()).or_insert(HashMap::new()).insert(a.to_string(), 1.0 / value);
        }
        
        // Step 3: Perform queries

        let mut results: Vec<f64> = Vec::new();
        
        for query in queries {
            let c = &query[0];
            let d = &query[1];
            
            if let Some(result) = Self::dfs(c, d, &mut HashMap::new(), &graph) {
                results.push(result);
            } else {
                results.push(-1.0);
            }
        }
        
        results

    }
    
    fn dfs(start: &str, end: &str, visited: &mut HashMap<String, bool>, graph: &HashMap<String, HashMap<String, f64>>) -> Option<f64> {
        if !graph.contains_key(start) || !graph.contains_key(end) {
            return None;
        }
        
        if start == end {
            return Some(1.0);
        }
        
        visited.insert(start.to_string(), true);
        
        if let Some(neighbors) = graph.get(start) {
            for (neighbor, weight) in neighbors {
                if visited.contains_key(neighbor) {
                    continue;
                }
                
                if let Some(result) = Self::dfs(neighbor, end, visited, graph) {
                    return Some(result * weight);
                }
            }
        }
        
        visited.remove(start);
        
        None

    }
}
