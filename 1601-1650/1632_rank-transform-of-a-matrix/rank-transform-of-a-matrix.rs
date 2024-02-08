
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> DisjointSet {
        let parent = (0..=n).collect();
        let size = vec![1; n + 1];
        DisjointSet { parent, size }
    }

    fn find(&mut self, node: usize) -> usize {
        if node == self.parent[node] {
            return node;
        }
        self.parent[node] = self.find(self.parent[node]);
        self.parent[node]
    }

    fn union(&mut self, u: usize, v: usize) {
        let root_u = self.find(u);
        let root_v = self.find(v);
        if root_u == root_v {
            return;
        }
        if self.size[root_u] < self.size[root_v] {
            self.parent[root_u] = root_v;
            self.size[root_v] += self.size[root_u];
        } else {
            self.parent[root_v] = root_u;
            self.size[root_u] += self.size[root_v];
        }
    }
}



impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut mp: std::collections::BTreeMap<i32, Vec<(usize, usize)>> = std::collections::BTreeMap::new();

        for i in 0..n {
            for j in 0..m {
                mp.entry(matrix[i][j]).or_insert(vec![]).push((i, j));
            }
        }

        let mut ans: Vec<Vec<i32>> = vec![vec![0; m]; n];
        let mut row: Vec<i32> = vec![0; n];
        let mut col: Vec<i32> = vec![0; m];
        let mut rank: Vec<i32> = vec![0; m + n];

        for (_, v) in &mp {
            let mut ds = DisjointSet::new(10005);
            for &(i, j) in v {
                ds.union(i, j + n);
            }
            for &(i, j) in v {
                let root = ds.find(i);
                rank[root] = rank[root].max(row[i]).max(col[j]);
            }
            for &(i, j) in v {
                let root = ds.find(i);
                let r = rank[root] + 1;
                ans[i][j] = r;
                row[i] = r;
                col[j] = r;
            }
        }

        ans

    }
}
