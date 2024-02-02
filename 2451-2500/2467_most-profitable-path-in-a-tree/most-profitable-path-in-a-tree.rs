
use std::collections::{HashMap, HashSet};



impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let n = amount.len();
        let mut graph: Vec<HashSet<i32>> = vec![HashSet::new(); n];

        for edge in edges {
            graph[edge[0] as usize].insert(edge[1]);
            graph[edge[1] as usize].insert(edge[0]);
        }

        let mut bob_path: HashMap<i32, i32> = HashMap::new();
        let mut stop = false;
        let mut visited: Vec<bool> = vec![false; n];

        fn backtrack_bob(node: i32, time: i32, bob_path: &mut HashMap<i32, i32>, stop: &mut bool, visited: &mut Vec<bool>, graph: &Vec<HashSet<i32>>) {
            bob_path.insert(node, time);
            visited[node as usize] = true;
            if node == 0 {
                *stop = true;
                return;
            }
            let mut count = 0;
            for &nei in &graph[node as usize] {
                if !visited[nei as usize] {
                    count += 1;
                    break;
                }
            }
            if count == 0 {
                bob_path.remove(&node);
                return;
            }
            for &nei in &graph[node as usize] {
                if !visited[nei as usize] && !*stop {
                    backtrack_bob(nei, time + 1, bob_path, stop, visited, graph);
                }
            }
            if !*stop {
                bob_path.remove(&node);
            }
        }

        backtrack_bob(bob, 0, &mut bob_path, &mut stop, &mut visited, &graph);

        let mut ans = std::i32::MIN;
        let mut income = 0;
        visited = vec![false; n];

        fn backtrack_alice(node: i32, time: i32, income: &mut i32, ans: &mut i32, visited: &mut Vec<bool>, graph: &Vec<HashSet<i32>>, amount: &Vec<i32>, bob_path: &HashMap<i32, i32>) {
            visited[node as usize] = true;
            let reward = if let Some(&bob_time) = bob_path.get(&node) {
                if time == bob_time {
                    amount[node as usize] / 2

                } else if time < bob_time {
                    amount[node as usize]
                } else {
                    0

                }
            } else {
                amount[node as usize]
            };
            *income += reward;
            let mut count = 0;
            for &nei in &graph[node as usize] {
                if !visited[nei as usize] {
                    count += 1;
                    break;
                }
            }
            if count == 0 {
                *ans = (*ans).max(*income);
                *income -= reward;
                return;
            }
            for &nei in &graph[node as usize] {
                if !visited[nei as usize] {
                    backtrack_alice(nei, time + 1, income, ans, visited, graph, amount, bob_path);
                }
            }
            *income -= reward;
        }

        backtrack_alice(0, 0, &mut income, &mut ans, &mut visited, &graph, &amount, &bob_path);

        ans

    }
}
