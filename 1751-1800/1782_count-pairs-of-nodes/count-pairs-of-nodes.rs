


use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut degree = vec![0; n];
        let mut freq: HashMap<(usize, usize), i32> = HashMap::new();

        for edge in &edges {
            let u = (edge[0] - 1) as usize;
            let v = (edge[1] - 1) as usize;
            degree[u] += 1;
            degree[v] += 1;
            let key = (u.min(v), u.max(v));
            *freq.entry(key).or_insert(0) += 1;
        }

        let mut vals = degree.clone();
        vals.sort();

        let mut ans = Vec::new();
        for &query in &queries {
            let mut cnt = 0;
            let (mut lo, mut hi) = (0, n - 1);
            while lo < hi {
                if query < vals[lo] + vals[hi] {
                    cnt += (hi - lo) as i32;
                    hi -= 1;
                } else {
                    lo += 1;
                }
            }
            for (key, &f) in &freq {
                let (u, v) = *key;
                if degree[u] + degree[v] - f <= query && query < degree[u] + degree[v] {
                    cnt -= 1;
                }
            }
            ans.push(cnt);
        }

        ans

    }
}
