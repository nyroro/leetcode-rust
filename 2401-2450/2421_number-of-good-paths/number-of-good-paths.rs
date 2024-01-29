
use std::collections::{HashMap, HashSet};

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union_set(&mut self, x: usize, y: usize) {
        let xset = self.find(x);
        let yset = self.find(y);
        if xset == yset {
            return;
        } else if self.rank[xset] < self.rank[yset] {
            self.parent[xset] = yset;
        } else if self.rank[xset] > self.rank[yset] {
            self.parent[yset] = xset;
        } else {
            self.parent[yset] = xset;
            self.rank[xset] += 1;
        }
    }
}

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = vals.len();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in edges {
            adj[edge[0] as usize].push(edge[1] as usize);
            adj[edge[1] as usize].push(edge[0] as usize);
        }

        let mut values_to_nodes: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &value) in vals.iter().enumerate() {
            values_to_nodes.entry(value).or_insert(Vec::new()).push(i);
        }

        let mut dsu = UnionFind::new(n);
        let mut good_paths = 0;

        let mut sorted_values: Vec<i32> = values_to_nodes.keys().cloned().collect();
        sorted_values.sort();

        for &value in &sorted_values {
            for &node in values_to_nodes[&value].iter() {
                for &neig in adj[node].iter() {
                    if vals[node] >= vals[neig] {
                        dsu.union_set(node, neig);
                    }
                }
            }

            let mut group: HashMap<usize, usize> = HashMap::new();
            for &node in values_to_nodes[&value].iter() {
                *group.entry(dsu.find(node)).or_insert(0) += 1;
            }

            for &size in group.values() {
                good_paths += (size * (size + 1)) / 2;
            }
        }

        good_paths as i32

    }
}
