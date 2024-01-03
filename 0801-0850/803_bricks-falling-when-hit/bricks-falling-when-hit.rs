
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parent = vec![0; n];
        let mut size = vec![1; n];
        for i in 0..n {
            parent[i] = i;
        }
        UnionFind { parent, size }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x

    }

    fn union(&mut self, mut x: usize, mut y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];
    }

    fn size_of_connected_component(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut copy = grid.clone();
        for hit in &hits {
            copy[hit[0] as usize][hit[1] as usize] = 0;
        }

        let mut uf = UnionFind::new(m * n + 1);
        for i in 0..n {
            if copy[0][i] == 1 {
                uf.union(i, m * n);
            }
        }

        let directions = vec![-1, 0, 1, 0, -1];
        for i in 1..m {
            for j in 0..n {
                if copy[i][j] == 1 {
                    for k in 0..4 {
                        let x = i as i32 + directions[k];
                        let y = j as i32 + directions[k + 1];
                        if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 && copy[x as usize][y as usize] == 1 {
                            uf.union(i * n + j, x as usize * n + y as usize);
                        }
                    }
                }
            }
        }

        let mut result = vec![];
        for i in (0..hits.len()).rev() {
            let x = hits[i][0] as usize;
            let y = hits[i][1] as usize;
            if grid[x][y] == 0 {
                result.push(0);
                continue;
            }
            let prev_size = uf.size_of_connected_component(m * n);
            if x == 0 {
                uf.union(y, m * n);
            }
            for k in 0..4 {
                let new_x = x as i32 + directions[k];
                let new_y = y as i32 + directions[k + 1];
                if new_x >= 0 && new_x < m as i32 && new_y >= 0 && new_y < n as i32 && copy[new_x as usize][new_y as usize] == 1 {
                    uf.union(x * n + y, new_x as usize * n + new_y as usize);
                }
            }
            let current_size = uf.size_of_connected_component(m * n);
            result.push(std::cmp::max(0, current_size as i32 - prev_size as i32 - 1));
            copy[x][y] = 1;
        }
        result.reverse();
        result

    }
}
