
use std::collections::{HashMap, HashSet};



impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut indeg: HashMap<i32, i32> = HashMap::new();
        let mut outdeg: HashMap<i32, i32> = HashMap::new();
        let mut rpath: Vec<i32> = Vec::new();

        // Build the graph

        for edge in &pairs {
            let start = edge[0];
            let end = edge[1];
            adj.entry(start).or_insert(Vec::new()).push(end);
            *outdeg.entry(start).or_insert(0) += 1;
            *indeg.entry(end).or_insert(0) += 1;
        }

        // Perform DFS

        fn dfs(i: i32, adj: &mut HashMap<i32, Vec<i32>>, rpath: &mut Vec<i32>) {
            while let Some(j) = adj.get_mut(&i).and_then(|v| v.pop()) {
                dfs(j, adj, rpath);
            }
            rpath.push(i);
        }

        // Find the starting vertex

        let mut i0 = *outdeg.keys().next().unwrap();
        for (v, deg) in &outdeg {
            if *deg == indeg.get(v).unwrap_or(&0) + 1 {
                i0 = *v;
                break;
            }
        }

        dfs(i0, &mut adj, &mut rpath);

        // Generate the valid arrangement

        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(pairs.len());
        for i in (0..rpath.len() - 1).rev() {
            ans.push(vec![rpath[i + 1], rpath[i]]);
        }

        ans

    }
}
