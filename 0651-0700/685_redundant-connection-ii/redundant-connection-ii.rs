
struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind { parent: (0..=n as i32).collect() }
    }

    fn find(&mut self, mut x: i32) -> i32 {
        while x != self.parent[x as usize] {
            x = self.parent[x as usize];
        }
        x

    }

    fn union(&mut self, x: i32, y: i32) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            false

        } else {
            self.parent[root_x as usize] = root_y;
            true

        }
    }
}

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut uf = UnionFind::new(n);
        let mut parent = vec![0; n + 1];
        let mut conflict_edge = vec![];
        let mut cycle_edge = vec![];

        for edge in &edges {
            let u = edge[0];
            let v = edge[1];
            if parent[v as usize] != 0 {
                conflict_edge = vec![parent[v as usize], v];
                conflict_edge.extend_from_slice(edge);
            } else {
                parent[v as usize] = u;
                if !uf.union(u, v) {
                    cycle_edge = edge.clone();
                }
            }
        }

        if !conflict_edge.is_empty() {
            if !cycle_edge.is_empty() {
                return vec![parent[conflict_edge[3] as usize], conflict_edge[3]];
            } else {
                return vec![conflict_edge[2], conflict_edge[3]];
            }
        } else {
            cycle_edge

        }
    }
}
