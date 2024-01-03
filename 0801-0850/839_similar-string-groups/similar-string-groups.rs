
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            for j in (i + 1)..n {
                if Solution::is_similar(&strs[i], &strs[j]) {
                    uf.union(i, j);
                }
            }
        }

        uf.count() as i32

    }

    fn is_similar(s1: &String, s2: &String) -> bool {
        let mut diff = 0;
        let mut chars1: Vec<char> = s1.chars().collect();
        let mut chars2: Vec<char> = s2.chars().collect();

        for i in 0..chars1.len() {
            if chars1[i] != chars2[i] {
                diff += 1;
                if diff > 2 {
                    return false;
                }
            }
        }

        true

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

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.size[root_x] > self.size[root_y] {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            } else {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            self.count -= 1;
        }
    }

    fn count(&self) -> usize {
        self.count

    }
}
