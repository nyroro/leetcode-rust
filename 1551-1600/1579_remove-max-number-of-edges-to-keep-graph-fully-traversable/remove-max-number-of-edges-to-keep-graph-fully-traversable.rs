
struct UnionFind {
    parent: Vec<i32>,
    size: Vec<i32>,
    count: i32,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n as i32).collect();
        let size = vec![1; n];
        let count = n as i32;
        UnionFind { parent, size, count }
    }

    fn find(&mut self, mut p: i32) -> i32 {
        while self.parent[p as usize] != p {
            self.parent[p as usize] = self.parent[self.parent[p as usize] as usize];
            p = self.parent[p as usize];
        }
        p

    }

    fn union(&mut self, mut p: i32, mut q: i32) -> bool {
        let root_p = self.find(p);
        let root_q = self.find(q);
        if root_p == root_q {
            return false;
        }
        let (smaller, larger) = if self.size[root_p as usize] > self.size[root_q as usize] {
            (root_q, root_p)
        } else {
            (root_p, root_q)
        };
        self.parent[smaller as usize] = larger;
        self.size[larger as usize] += self.size[smaller as usize];
        self.count -= 1;
        true

    }

    fn count(&self) -> i32 {
        self.count

    }
}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut alice = UnionFind::new(n);
        let mut bob = UnionFind::new(n);
        let mut res = 0;

        for edge in &edges {
            let t = edge[0];
            let u = (edge[1] - 1) as i32;
            let v = (edge[2] - 1) as i32;
            if t == 3 {
                if !alice.union(u, v) || !bob.union(u, v) {
                    res += 1;
                }
            }
        }

        for edge in &edges {
            let t = edge[0];
            let u = (edge[1] - 1) as i32;
            let v = (edge[2] - 1) as i32;
            if t == 1 {
                if !alice.union(u, v) {
                    res += 1;
                }
            } else if t == 2 {
                if !bob.union(u, v) {
                    res += 1;
                }
            }
        }

        if alice.count() != 1 || bob.count() != 1 {
            return -1;
        }

        res

    }
}
