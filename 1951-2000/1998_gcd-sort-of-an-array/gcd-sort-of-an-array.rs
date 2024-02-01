
impl Solution {
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dsu = DSU::new(100001); // 创建并初始化并查集

        let mut sorted_nums = nums.clone();
        sorted_nums.sort(); // 对 nums 进行排序


        // 使用筛法计算每个数的最小质因数

        let mut smallest_prime_factors = vec![0; 100001];
        for i in 2..=100000 {
            if smallest_prime_factors[i] == 0 {
                for j in (i..=100000).step_by(i) {
                    if smallest_prime_factors[j] == 0 {
                        smallest_prime_factors[j] = i;
                    }
                }
            }
        }

        // 遍历 nums 中的每个数

        for &num in &nums {
            let mut x = num;
            let mut factors = Vec::new();

            // 计算 num 的所有因子

            while x > 1 {
                factors.push(smallest_prime_factors[x]);
                x /= smallest_prime_factors[x];
            }

            // 将 num 和其因子进行合并

            for &factor in &factors {
                dsu.union(num, factor);
            }
        }

        // 检查 nums 和 sorted_nums 中的每个对应位置的数是否在同一个集合中

        for i in 0..n {
            if dsu.find(nums[i]) != dsu.find(sorted_nums[i]) {
                return false;
            }
        }

        true

    }
}

// 并查集数据结构

struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let mut rank = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        DSU { parent, rank }
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
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_x] = root_y;
                self.rank[root_y] += 1;
            }
        }
    }
}
