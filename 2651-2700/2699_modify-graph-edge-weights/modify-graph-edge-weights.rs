
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    node: usize,
    weight: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    cost: i64,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn modified_graph_edges(n: i32, mut edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;
        let target = target as i64; // Convert target to i64


        let mut adj_list: HashMap<usize, Vec<Edge>> = HashMap::new();

        for edge in &edges {
            let (a, b, w) = (edge[0] as usize, edge[1] as usize, edge[2] as i64); // Convert edge weight to i64

            if w != -1 {
                adj_list.entry(a).or_insert(Vec::new()).push(Edge { node: b, weight: w });
                adj_list.entry(b).or_insert(Vec::new()).push(Edge { node: a, weight: w });
            }
        }

        let mut heap = BinaryHeap::new();
        let mut dist: Vec<i64> = vec![std::i64::MAX; n];
        dist[source] = 0;
        heap.push(State { cost: 0, position: source });

        while let Some(State { cost, position }) = heap.pop() {
            if position == destination {
                break;
            }
            if cost > dist[position] {
                continue;
            }
            if let Some(neighbors) = adj_list.get(&position) {
                for &Edge { node, weight } in neighbors {
                    let next = State { cost: cost + weight, position: node };
                    if next.cost < dist[next.position] {
                        heap.push(next);
                        dist[next.position] = next.cost;
                    }
                }
            }
        }

        if dist[destination] < target {
            return Vec::new();
        }

        if dist[destination] == target {
            for edge in &mut edges {
                if edge[2] == -1 {
                    edge[2] = 1_000_000_000;
                }
            }
            return edges;
        }

        for i in 0..edges.len() {
            let (a, b, w) = (edges[i][0] as usize, edges[i][1] as usize, edges[i][2] as i64); // Convert edge weight to i64

            if w == -1 {
                edges[i][2] = 1;
                adj_list.entry(a).or_insert(Vec::new()).push(Edge { node: b, weight: 1 });
                adj_list.entry(b).or_insert(Vec::new()).push(Edge { node: a, weight: 1 });

                let mut heap = BinaryHeap::new();
                let mut dist: Vec<i64> = vec![std::i64::MAX; n];
                dist[source] = 0;
                heap.push(State { cost: 0, position: source });

                while let Some(State { cost, position }) = heap.pop() {
                    if position == destination {
                        break;
                    }
                    if cost > dist[position] {
                        continue;
                    }
                    if let Some(neighbors) = adj_list.get(&position) {
                        for &Edge { node, weight } in neighbors {
                            let next = State { cost: cost + weight, position: node };
                            if next.cost < dist[next.position] {
                                heap.push(next);
                                dist[next.position] = next.cost;
                            }
                        }
                    }
                }

                if dist[destination] <= target {
                    edges[i][2] += (target - dist[destination]) as i32; // Convert to i32 for addition

                    for j in i + 1..edges.len() {
                        if edges[j][2] == -1 {
                            edges[j][2] = 1_000_000_000;
                        }
                    }
                    return edges;
                }
            }
        }

        Vec::new()
    }
}
