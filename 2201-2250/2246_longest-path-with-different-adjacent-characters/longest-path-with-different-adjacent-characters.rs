
use std::collections::HashMap;



impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let n = parent.len();
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 1..n {
            let p = parent[i] as i32;
            if !graph.contains_key(&p) {
                graph.insert(p, vec![]);
            }
            graph.get_mut(&p).unwrap().push(i as i32);
        }

        let mut ans = 0;

        fn dfs(x: i32, s: &Vec<char>, graph: &HashMap<i32, Vec<i32>>, ans: &mut i32) -> i32 {
            let mut max_len = 0;
            if let Some(children) = graph.get(&x) {
                for &y in children {
                    let len = dfs(y, s, graph, ans) + 1;
                    if s[x as usize] != s[y as usize] {
                        *ans = (*ans).max(max_len + len);
                        max_len = max_len.max(len);
                    }
                }
            }
            max_len

        }

        dfs(0, &s.chars().collect(), &graph, &mut ans);
        ans + 1

    }
}
