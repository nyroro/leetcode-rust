
struct UnionFind {
    parent: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        UnionFind { parent, count: n }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x

    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
            self.count -= 1;
        }
    }

    fn get_count(&self) -> usize {
        self.count

    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut uf = UnionFind::new(n);
        let mut x_map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        let mut y_map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();

        for i in 0..n {
            let x = stones[i][0];
            let y = stones[i][1];
            if x_map.contains_key(&x) {
                uf.union(i, *x_map.get(&x).unwrap());
            } else {
                x_map.insert(x, i);
            }
            if y_map.contains_key(&y) {
                uf.union(i, *y_map.get(&y).unwrap());
            } else {
                y_map.insert(y, i);
            }
        }
        (n - uf.get_count()) as i32

    }
}
