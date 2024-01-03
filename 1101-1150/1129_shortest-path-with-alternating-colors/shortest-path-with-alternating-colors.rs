
impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![vec![vec![], vec![]]; n as usize];
        let mut queue = std::collections::VecDeque::new();
        let mut visited = vec![vec![false; 2]; n as usize];
        let mut shortest_paths = vec![vec![-1; 2]; n as usize];
        
        for edge in red_edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u][0].push(v);
        }
        
        for edge in blue_edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u][1].push(v);
        }
        
        queue.push_back((0, 0, 0));
        queue.push_back((0, 1, 0));
        visited[0][0] = true;
        visited[0][1] = true;
        shortest_paths[0][0] = 0;
        shortest_paths[0][1] = 0;
        
        while let Some((node, color, distance)) = queue.pop_front() {
            let next_color = 1 - color;
            
            for &next_node in &graph[node][next_color] {
                if !visited[next_node][next_color] {
                    visited[next_node][next_color] = true;
                    shortest_paths[next_node][next_color] = distance + 1;
                    queue.push_back((next_node, next_color, distance + 1));
                }
            }
        }
        
        let mut result = vec![];
        
        for path in shortest_paths {
            if path[0] == -1 && path[1] == -1 {
                result.push(-1);
            } else if path[0] == -1 {
                result.push(path[1]);
            } else if path[1] == -1 {
                result.push(path[0]);
            } else {
                result.push(path[0].min(path[1]));
            }
        }
        
        result

    }
}
