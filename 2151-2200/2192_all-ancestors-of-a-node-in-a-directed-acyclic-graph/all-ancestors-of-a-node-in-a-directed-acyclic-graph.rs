
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut table: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut degrees: Vec<i32> = vec![0; n as usize];
        
        for e in &edges {
            table.entry(e[0]).or_insert(Vec::new()).push(e[1]);
            degrees[e[1] as usize] += 1;
        }
        
        let mut q: Vec<i32> = Vec::new();
        for i in 0..n {
            if degrees[i as usize] == 0 {
                q.push(i);
            }
        }
        
        let mut qi = 0;
        let mut ret: Vec<HashSet<i32>> = vec![HashSet::new(); n as usize];
        
        while qi < q.len() {
            let now = q[qi];
            qi += 1;
            if let Some(t) = table.get(&now) {
                for &node in t {
                    let ancestors = ret[now as usize].iter().cloned().collect::<HashSet<i32>>();
                    ret[node as usize].insert(now);
                    ret[node as usize].extend(ancestors);
                    degrees[node as usize] -= 1;
                    if degrees[node as usize] <= 0 {
                        q.push(node);
                    }
                }
            }
        }
        
        let mut result: Vec<Vec<i32>> = Vec::new();
        for v in ret {
            let mut sorted_v: Vec<i32> = v.into_iter().collect();
            sorted_v.sort();
            result.push(sorted_v);
        }
        
        result

    }
}
