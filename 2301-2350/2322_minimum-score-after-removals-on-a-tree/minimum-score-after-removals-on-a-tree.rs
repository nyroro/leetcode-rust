
struct Tree {
    nums: Vec<i32>,
    edges: Vec<Vec<i32>>,
    graph: Vec<Vec<usize>>,
    score: Vec<i32>,
    tin: Vec<usize>,
    tout: Vec<usize>,
}

impl Tree {
    fn new(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Tree {
        let n = nums.len();
        let mut graph = vec![vec![]; n];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        Tree {
            nums,
            edges,
            graph,
            score: vec![0; n],
            tin: vec![0; n],
            tout: vec![0; n],
        }
    }

    fn dfs(&mut self, u: usize, p: i32, t: &mut usize) {
        self.score[u] = self.nums[u];
        self.tin[u] = *t;
        *t += 1;
        for i in 0..self.graph[u].len() {
            let v = self.graph[u][i];
            if v as i32 != p {
                self.dfs(v, u as i32, t);
                self.score[u] ^= self.score[v];
            }
        }
        self.tout[u] = *t;
    }

    fn minimum_score(&mut self) -> i32 {
        let n = self.nums.len();
        let mut ans = std::i32::MAX;
        for u in 1..n {
            for v in (u + 1)..n {
                let (uu, vv, xx) = if self.tin[v] <= self.tin[u] && self.tout[v] >= self.tout[u] {
                    (self.score[u], self.score[v] ^ self.score[u], self.score[0] ^ self.score[v])
                } else if self.tin[v] >= self.tin[u] && self.tout[v] <= self.tout[u] {
                    (self.score[u] ^ self.score[v], self.score[v], self.score[0] ^ self.score[u])
                } else {
                    (self.score[u], self.score[v], self.score[0] ^ self.score[u] ^ self.score[v])
                };
                ans = ans.min(uu.max(vv).max(xx) - uu.min(vv).min(xx));
            }
        }
        ans

    }
}

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut tree = Tree::new(nums, edges);
        let mut t = 0;
        tree.dfs(0, -1, &mut t);
        tree.minimum_score()
    }
}
