
use std::collections::{HashMap, VecDeque};



struct Graph {
    adj_list: HashMap<i32, Vec<i32>>,
    indegree: HashMap<i32, i32>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
            indegree: HashMap::new(),
        }
    }

    fn add_edge(&mut self, a: i32, b: i32) {
        self.adj_list.entry(a).or_insert(Vec::new()).push(b);
        *self.indegree.entry(b).or_insert(0) += 1;
    }
}

impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; k as usize]; k as usize];
        let row_order = Solution::topological_sort(&row_conditions, k);
        let col_order = Solution::topological_sort(&col_conditions, k);

        if row_order.len() < k as usize || col_order.len() < k as usize {
            return Vec::new();
        }

        for (row, &val) in row_order.iter().enumerate() {
            if let Some(col) = col_order.iter().position(|&x| x == val) {
                matrix[row][col] = val;
            }
        }

        matrix

    }

    fn topological_sort(condition: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut graph = Graph::new();

        for pair in condition {
            graph.add_edge(pair[0], pair[1]);
        }

        let mut queue = VecDeque::new();
        for i in 1..=k {
            if !graph.indegree.contains_key(&i) {
                queue.push_back(i);
            }
        }

        let mut order = Vec::new();

        while let Some(node) = queue.pop_front() {
            order.push(node);

            if let Some(neighbors) = graph.adj_list.get(&node) {
                for &neighbor in neighbors {
                    *graph.indegree.get_mut(&neighbor).unwrap() -= 1;
                    if *graph.indegree.get(&neighbor).unwrap() == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        order

    }
}
