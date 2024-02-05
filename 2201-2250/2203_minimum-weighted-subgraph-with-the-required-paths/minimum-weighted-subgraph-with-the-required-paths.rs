
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;



struct Graph {
    edges: HashMap<i32, HashMap<i32, i64>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: i32, to: i32, weight: i64) {
        self.edges.entry(from).or_insert(HashMap::new()).entry(to).and_modify(|e| *e = (*e).min(weight)).or_insert(weight);
    }
}

impl Solution {
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let mut graph = Graph::new();
        let mut dual_graph = Graph::new();

        for edge in edges {
            let from = edge[0];
            let to = edge[1];
            let weight = edge[2] as i64;
            graph.add_edge(from, to, weight);
            dual_graph.add_edge(to, from, weight);
        }

        let src1_distance = Solution::dijkstra(&graph.edges, src1);
        let src2_distance = Solution::dijkstra(&graph.edges, src2);

        if !src1_distance.contains_key(&dest) && !src2_distance.contains_key(&dest) {
            return -1;
        }

        let dest_distance = Solution::dijkstra(&dual_graph.edges, dest);

        let mut travel_costs = HashSet::new();

        for (&k, &cost) in &src1_distance {
            if let Some(&cost2) = src2_distance.get(&k) {
                if let Some(&cost3) = dest_distance.get(&k) {
                    travel_costs.insert(cost + cost2 + cost3);
                }
            }
        }

        if let Some(&min_cost) = travel_costs.iter().min() {
            return min_cost;
        }

        -1

    }

    fn dijkstra(graph: &HashMap<i32, HashMap<i32, i64>>, src: i32) -> HashMap<i32, i64> {
        let mut distance = HashMap::new();
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), src));

        while let Some((Reverse(cost), node)) = pq.pop() {
            if !distance.contains_key(&node) {
                distance.insert(node, cost);
                if let Some(neighbors) = graph.get(&node) {
                    for (&neighbor, &weight) in neighbors {
                        if !distance.contains_key(&neighbor) {
                            pq.push((Reverse(cost + weight), neighbor));
                        }
                    }
                }
            }
        }

        distance

    }
}
