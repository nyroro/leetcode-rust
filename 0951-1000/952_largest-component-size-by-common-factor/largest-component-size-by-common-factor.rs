
struct UnionFind {
    parent: Vec<i32>,
    size: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parent = vec![-1; n];
        let size = vec![1; n];
        UnionFind { parent, size }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != -1 {
            x = self.parent[x] as usize;
        }
        x

    }

    fn union(&mut self, mut x: usize, mut y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent[root_y] = root_x as i32;
            self.size[root_x] += self.size[root_y];
        }
    }
}

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap() as usize;
        let mut uf = UnionFind::new(max_num + 1);

        for &num in &nums {
            let mut x = num as usize;
            let mut i = 2;
            while i * i <= x {
                if x % i == 0 {
                    uf.union(x, i as usize);
                    uf.union(x, x / i);
                }
                i += 1;
            }
        }

        let mut count_map: std::collections::HashMap<usize, i32> = std::collections::HashMap::new();
        let mut max_count = 0;

        for &num in &nums {
            let root = uf.find(num as usize);
            *count_map.entry(root).or_insert(0) += 1;
            max_count = max_count.max(*count_map.get(&root).unwrap());
        }

        max_count

    }
}
