
use std::collections::HashMap;

struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind { parent: vec![-1; size] }
    }

    fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] >= 0 {
            root = self.parent[root] as usize;
        }
        while self.parent[x] >= 0 {
            let parent = self.parent[x] as usize;
            self.parent[x] = root as i32;
            x = parent;
        }
        root

    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.parent[root_x] < self.parent[root_y] {
                self.parent[root_x] += self.parent[root_y];
                self.parent[root_y] = root_x as i32;
            } else {
                self.parent[root_y] += self.parent[root_x];
                self.parent[root_x] = root_y as i32;
            }
        }
    }
}

impl Solution {
    pub fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut queries_with_index: Vec<(usize, i32, i32, i32)> = queries

            .iter()
            .enumerate()
            .map(|(i, q)| (i, q[0], q[1], q[2]))
            .collect();
        queries_with_index.sort_by_key(|&(_, _, _, limit)| limit);

        let mut edge_list_with_index: Vec<(usize, i32, i32, i32)> = edge_list

            .iter()
            .enumerate()
            .map(|(i, e)| (i, e[0], e[1], e[2]))
            .collect();
        edge_list_with_index.sort_by_key(|&(_, _, _, dis)| dis);

        let mut uf = UnionFind::new(n as usize);
        let mut result = vec![false; queries.len()];
        let mut edge_index = 0;

        for (i, p, q, limit) in queries_with_index {
            while edge_index < edge_list_with_index.len() && edge_list_with_index[edge_index].3 < limit {
                let (_, u, v, _) = edge_list_with_index[edge_index];
                uf.union(u as usize, v as usize);
                edge_index += 1;
            }
            result[i] = uf.find(p as usize) == uf.find(q as usize);
        }

        result

    }
}
