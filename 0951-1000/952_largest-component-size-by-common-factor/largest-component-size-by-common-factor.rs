
use std::collections::HashMap;

struct UnionFind {
    parent: HashMap<i32, i32>,
    size: HashMap<i32, i32>,
}

impl UnionFind {
    fn new() -> UnionFind {
        UnionFind {
            parent: HashMap::new(),
            size: HashMap::new(),
        }
    }

    fn find(&mut self, mut x: i32) -> i32 {
        while self.parent.contains_key(&x) {
            let px = self.parent[&x];
            if px == x {
                break;
            }
            x = px;
        }
        x

    }

    fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            let size_x = *self.size.get(&root_x).unwrap_or(&1);
            let size_y = *self.size.get(&root_y).unwrap_or(&1);
            if size_x < size_y {
                self.parent.insert(root_x, root_y);
                *self.size.entry(root_y).or_insert(1) += size_x;
            } else {
                self.parent.insert(root_y, root_x);
                *self.size.entry(root_x).or_insert(1) += size_y;
            }
        }
    }
}

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let mut uf = UnionFind::new();
        for &num in &nums {
            let mut x = num;
            let mut i = 2;
            while i * i <= x {
                if x % i == 0 {
                    uf.union(x, i);
                    uf.union(x, x / i);
                }
                i += 1;
            }
        }

        let mut count_map: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;

        for &num in &nums {
            let root = uf.find(num);
            *count_map.entry(root).or_insert(0) += 1;
            max_count = max_count.max(*count_map.get(&root).unwrap());
        }

        max_count

    }
}
