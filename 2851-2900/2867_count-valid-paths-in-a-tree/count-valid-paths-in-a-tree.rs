
struct Tree {
    n: usize,
    edges: Vec<(usize, usize)>,
    adj_list: Vec<Vec<usize>>,
}

impl Tree {
    fn new(n: usize, edges: Vec<Vec<i32>>) -> Tree {
        let edges: Vec<(usize, usize)> = edges.iter().map(|edge| (edge[0] as usize, edge[1] as usize)).collect();
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n + 1];
        for (u, v) in &edges {
            adj_list[*u].push(*v);
            adj_list[*v].push(*u);
        }
        Tree { n, edges, adj_list }
    }

    fn count_paths(&self) -> i64 {
        fn dfs(x: usize, f: usize, con: &Vec<Vec<usize>>, prime: &Vec<bool>, r: &mut i64) -> (i64, i64) {
            let mut v = [1 - prime[x] as i64, prime[x] as i64];
            for &y in &con[x] {
                if y == f {
                    continue;
                }
                let p = dfs(y, x, con, prime, r);
                *r += p.0 * v[1] + p.1 * v[0];
                if prime[x] {
                    v[1] += p.0;
                } else {
                    v[0] += p.0;
                    v[1] += p.1;
                }
            }
            (v[0], v[1])
        }

        let mut prime = vec![true; self.n + 1];
        prime[1] = false;

        let mut all_primes = vec![];
        for i in 2..=self.n {
            if prime[i] {
                all_primes.push(i);
            }
            for &x in &all_primes {
                let temp = i * x;
                if temp > self.n {
                    break;
                }
                prime[temp] = false;
                if i % x == 0 {
                    break;
                }
            }
        }

        let mut con = vec![vec![]; self.n + 1];
        for (u, v) in &self.edges {
            con[*u].push(*v);
            con[*v].push(*u);
        }

        let mut result = 0;
        dfs(1, 0, &con, &prime, &mut result);
        result

    }
}

impl Solution {
    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let tree = Tree::new(n as usize, edges);
        tree.count_paths()
    }
}
