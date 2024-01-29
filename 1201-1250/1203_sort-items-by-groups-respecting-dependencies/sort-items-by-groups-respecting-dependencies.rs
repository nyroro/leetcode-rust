
use std::collections::{HashMap, HashSet};



impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let mut group = group;
        for i in 0..n as usize {
            if group[i] == -1 {
                group[i] = i as i32 + m;
            }
        }

        let mut graph0: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut seen0: Vec<i32> = vec![0; (m + n) as usize];
        let mut graph1: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut seen1: Vec<i32> = vec![0; n as usize];

        for (i, x) in before_items.iter().enumerate() {
            for &j in x {
                if group[j as usize] != group[i] {
                    graph0.entry(group[j as usize]).or_insert(Vec::new()).push(group[i]);
                    seen0[group[i] as usize] += 1;
                }
                graph1.entry(j).or_insert(Vec::new()).push(i as i32);
                seen1[i] += 1;
            }
        }

        fn topological_sort(graph: &HashMap<i32, Vec<i32>>, seen: &mut Vec<i32>) -> Vec<i32> {
            let mut ans: Vec<i32> = Vec::new();
            let mut stack: Vec<i32> = (0..seen.len() as i32).filter(|&k| seen[k as usize] == 0).collect();
            while let Some(n) = stack.pop() {
                ans.push(n);
                if let Some(adj) = graph.get(&n) {
                    for &s in adj {
                        seen[s as usize] -= 1;
                        if seen[s as usize] == 0 {
                            stack.push(s);
                        }
                    }
                }
            }
            ans

        }

        let top0 = topological_sort(&graph0, &mut seen0);
        if top0.len() != (m + n) as usize {
            return Vec::new();
        }

        let top1 = topological_sort(&graph1, &mut seen1);
        if top1.len() != n as usize {
            return Vec::new();
        }

        let map0: HashMap<i32, usize> = top0.iter().enumerate().map(|(i, &x)| (x, i)).collect();
        let map1: HashMap<i32, usize> = top1.iter().enumerate().map(|(i, &x)| (x, i)).collect();

        let mut items: Vec<i32> = (0..n).map(|x| x as i32).collect();
        items.sort_by_key(|&x| (map0[&group[x as usize]], map1[&x]));
        items

    }
}
