
// Step 1: Create a helper function to calculate the greatest common divisor (GCD)
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a

}

// Step 2: Create a UnionFind struct to implement the Union Find data structure

struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind { parent: (0..size as i32).collect() }
    }

    fn find(&mut self, mut x: i32) -> i32 {
        while x != self.parent[x as usize] {
            self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
            x = self.parent[x as usize];
        }
        x

    }

    fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x as usize] = root_y;
        }
    }
}

// Step 3: Implement the are_connected function in the Solution block

impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut uf = UnionFind::new((n + 1) as usize);
        let mut result = vec![];

        for i in (threshold + 1)..=n {
            let mut j = i * 2;
            while j <= n {
                uf.union(i, j);
                j += i;
            }
        }

        for query in queries {
            let x = query[0];
            let y = query[1];
            result.push(uf.find(x) == uf.find(y));
        }

        result

    }
}
