
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<&str, BinaryHeap<Reverse<&str>>> = HashMap::new();

        // 构建图

        for ticket in &tickets {
            graph.entry(&ticket[0])
                .or_insert(BinaryHeap::new())
                .push(Reverse(&ticket[1]));
        }

        let mut result = Vec::new();

        // 深度优先搜索

        fn dfs(graph: &mut HashMap<&str, BinaryHeap<Reverse<&str>>>, airport: &str, result: &mut Vec<String>) {
            while let Some(dest) = graph.get_mut(airport).and_then(|heap| heap.pop().map(|r| r.0)) {
                dfs(graph, dest, result);
            }
            result.push(airport.to_string());
        }

        dfs(&mut graph, "JFK", &mut result);

        result.reverse();
        result

    }
}
