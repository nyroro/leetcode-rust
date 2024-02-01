
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.edges.entry(u).or_insert(Vec::new()).push(v);
        self.edges.entry(v).or_insert(Vec::new()).push(u);
    }

    fn dijkstra(&self, start: usize) -> Vec<i32> {
        let mut dist: Vec<i32> = vec![std::i32::MAX; self.edges.len()];
        let mut heap = BinaryHeap::new();

        dist[start - 1] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if cost > dist[position - 1] {
                continue;
            }

            for &next in self.edges[&position].iter() {
                let next_cost = cost + 1;
                if next_cost < dist[next - 1] {
                    dist[next - 1] = next_cost;
                    heap.push(State {
                        cost: next_cost,
                        position: next,
                    });
                }
            }
        }

        dist

    }
}

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let mut graph = Graph::new();
        for edge in edges {
            graph.add_edge(edge[0] as usize, edge[1] as usize);
        }

        let d = graph.dijkstra(n as usize);
        let d2 = graph.dijkstra(1);

        let mut retb = d[0] + 2;
        for edge in edges {
            if (d[edge[0] as usize - 1] + d2[edge[1] as usize - 1] + 1 == retb - 1)
                || (d[edge[1] as usize - 1] + d2[edge[0] as usize - 1] + 1 == retb - 1)
            {
                retb = retb - 1;
                break;
            }
        }

        fn gao(x: i32, time: i32, change: i32) -> i32 {
            let mut t = 0;
            for _ in 0..x {
                if (t / change) % 2 == 1 {
                    t = ((t / change) + 1) * change;
                }
                t += time;
            }
            t

        }

        gao(retb, time, change)
    }
}
