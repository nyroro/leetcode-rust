
use std::collections::HashMap;



impl Solution {
    pub fn count_palindrome_paths(parent: Vec<i32>, s: String) -> i64 {
        let mut g: Vec<Vec<(usize, u8)>> = vec![vec![]; parent.len()];
        let mut frq: HashMap<i32, i32> = HashMap::new();

        for j in 1..parent.len() {
            g[parent[j] as usize].push((j, s.as_bytes()[j]));
        }

        fn dfs(node: usize, path: i32, g: &Vec<Vec<(usize, u8)>>, frq: &mut HashMap<i32, i32>) {
            *frq.entry(path).or_insert(0) += 1;

            for &(child, ch) in &g[node] {
                let new_path = path ^ (1 << (ch - b'a'));
                dfs(child, new_path, g, frq);
            }
        }

        dfs(0, 0, &g, &mut frq);

        let mut result: i64 = 0;
        for (&val, &cnt) in &frq {
            for j in 0..26 {
                let other = val ^ (1 << j);
                if let Some(&other_cnt) = frq.get(&other) {
                    result += cnt as i64 * other_cnt as i64;
                }
            }
            result += cnt as i64 * (cnt as i64 - 1);
        }

        result / 2

    }
}
