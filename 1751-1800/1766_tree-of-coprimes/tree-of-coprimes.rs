
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut coprimes: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut ancestors: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut result: Vec<i32> = vec![-1; nums.len()];

        // Populate coprimes HashMap

        let unique_nums: HashSet<i32> = nums.iter().cloned().collect();
        for &n1 in unique_nums.iter() {
            for &n2 in unique_nums.iter() {
                if gcd(n1, n2) == 1 {
                    coprimes.entry(n1).or_insert(Vec::new()).push(n2);
                }
            }
        }

        // Populate adjacency list

        for edge in edges.iter() {
            adjacency_list.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
            adjacency_list.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
        }

        // DFS traversal

        fn dfs(
            nums: &Vec<i32>,
            adjacency_list: &HashMap<i32, Vec<i32>>,
            coprimes: &HashMap<i32, Vec<i32>>,
            ancestors: &mut HashMap<i32, Vec<(i32, i32)>>,
            result: &mut Vec<i32>,
            node: usize, // Change the type to usize

            parent: i32,
            level: i32,
        ) {
            let mut max_level = -1;
            if let Some(cop_values) = coprimes.get(&nums[node]) {
                for &cop in cop_values.iter() {
                    if let Some(ancestor_list) = ancestors.get(&cop) {
                        if let Some(last_ancestor) = ancestor_list.last() {
                            if last_ancestor.0 > max_level {
                                max_level = last_ancestor.0;
                                result[node] = last_ancestor.1;
                            }
                        }
                    }
                }
            }
            ancestors.entry(nums[node]).or_insert(Vec::new()).push((level, node as i32)); // Convert node to i32

            if let Some(neighbors) = adjacency_list.get(&(node as i32)) { // Convert node to i32

                for &child in neighbors.iter() {
                    if child != parent {
                        dfs(nums, adjacency_list, coprimes, ancestors, result, child as usize, node as i32, level + 1); // Convert child and node to usize and i32

                    }
                }
            }
            ancestors.get_mut(&nums[node]).unwrap().pop();
        }

        dfs(&nums, &adjacency_list, &coprimes, &mut ancestors, &mut result, 0, 0, 0); // Pass 0 as usize

        result

    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a

    } else {
        gcd(b, a % b)
    }
}
