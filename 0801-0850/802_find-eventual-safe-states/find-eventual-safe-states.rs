
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut status = vec![0; n];
        let mut result = Vec::new();

        for i in 0..n {
            if Solution::is_safe_node(i as i32, &graph, &mut status) {
                result.push(i as i32);
            }
        }

        result

    }

    fn is_safe_node(node: i32, graph: &Vec<Vec<i32>>, status: &mut Vec<i32>) -> bool {
        if status[node as usize] != 0 {
            return status[node as usize] == 2;
        }

        status[node as usize] = 1;

        for &next_node in &graph[node as usize] {
            if !Solution::is_safe_node(next_node, graph, status) {
                return false;
            }
        }

        status[node as usize] = 2;
        true

    }
}
