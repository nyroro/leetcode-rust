
use std::collections::HashMap;



impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = edges.len() + 1;
        let mut adj = vec![vec![]; n];
        let mut sub = vec![0; n];
        let mut mp = HashMap::new();

        for edge in &edges {
            adj[edge[0] as usize].push(edge[1] as usize);
            adj[edge[1] as usize].push(edge[0] as usize);
        }

        for guess in &guesses {
            mp.insert((guess[0], guess[1]), true);
        }

        let mut count = 0;
        Solution::dfs(0, -1, &mut mp, &mut sub, &adj);
        Solution::sfd(0, -1, &mut sub, &mp, &adj, k, &mut count);
        count

    }

    fn dfs(root: usize, par: i32, mp: &mut HashMap<(i32, i32), bool>, sub: &mut Vec<i32>, adj: &Vec<Vec<usize>>) {
        for &i in &adj[root] {
            if i as i32 == par {
                continue;
            }
            if mp.contains_key(&(root as i32, i as i32)) {
                sub[root] += 1;
            }
            Solution::dfs(i, root as i32, mp, sub, adj);
            sub[root] += sub[i];
        }
    }

    fn sfd(root: usize, par: i32, sub: &mut Vec<i32>, mp: &HashMap<(i32, i32), bool>, adj: &Vec<Vec<usize>>, k: i32, count: &mut i32) {
        let num = if par == -1 { sub[root] } else { sub[par as usize] - if mp.contains_key(&(par, root as i32)) { 1 } else { 0 } + if mp.contains_key(&(root as i32, par)) { 1 } else { 0 } };
        if num >= k {
            *count += 1;
        }
        sub[root] = num;
        for &i in &adj[root] {
            if i as i32 == par {
                continue;
            }
            Solution::sfd(i, root as i32, sub, mp, adj, k, count);
        }
    }
}
