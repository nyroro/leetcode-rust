
impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = edges.len();
        let mut sorted_edges = edges.iter().enumerate().collect::<Vec<_>>();
        sorted_edges.sort_by(|a, b| a.1[2].cmp(&b.1[2]));

        let mut uf = UnionFind::new(n as usize);
        let mut mst_weight = 0;
        for (i, edge) in sorted_edges.iter() {
            if uf.union(edge[0] as usize, edge[1] as usize) {
                mst_weight += edge[2];
            }
        }

        let mut result = vec![vec![], vec![]];
        for (i, edge) in sorted_edges.iter().enumerate() {
            let mut uf = UnionFind::new(n as usize);
            let mut weight = 0;
            for (j, e) in sorted_edges.iter().enumerate() {
                if i != j && uf.union(e.1[0] as usize, e.1[1] as usize) {
                    weight += e.1[2];
                }
            }

            if uf.count() != 1 || (uf.count() == 1 && weight > mst_weight) {
                result[0].push(edge.0 as i32);
            } else {
                let mut uf = UnionFind::new(n as usize);
                uf.union(edge.1[0] as usize, edge.1[1] as usize);
                weight = edge.1[2];
                for (j, e) in sorted_edges.iter().enumerate() {
                    if i != j && uf.union(e.1[0] as usize, e.1[1] as usize) {
                        weight += e.1[2];
                    }
                }
                if weight == mst_weight {
                    result[1].push(edge.0 as i32);
                }
            }
        }

        result

    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let mut size = vec![1; n];
        for i in 0..n {
            parent[i] = i;
        }
        UnionFind {
            parent,
            size,
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        self.count -= 1;
        true

    }

    fn count(&self) -> usize {
        self.count

    }
}
