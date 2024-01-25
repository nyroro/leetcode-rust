
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        let mut table: HashMap<i32, HashSet<i32>> = HashMap::new();

        // Populate the HashMap with pairs and their ancestors

        for pair in pairs {
            table.entry(pair[0]).or_insert(HashSet::new()).insert(pair[1]);
            table.entry(pair[1]).or_insert(HashSet::new()).insert(pair[0]);
        }

        let mut roots: Vec<i32> = table.keys().cloned().collect();
        roots.sort_by_key(|&k| -(table[&k].len() as i32));

        let mut ans = 1;
        let mut ancestor: HashSet<i32> = HashSet::new();

        for &root in &roots {
            let pre = ancestor.intersection(&table[&root]).min_by_key(|&t| table[t].len()).cloned();
            ancestor.insert(root);

            if let Some(pre_node) = pre {
                let pre_set = table[&pre_node].union(&vec![pre_node].iter().cloned().collect()).cloned().collect();
                if !table[&root].difference(&pre_set).next().is_none() {
                    return 0;
                }
                if table[&root].len() == table[&pre_node].len() {
                    ans = 2;
                }
            } else if table[&root].len() != table.len() - 1 {
                return 0;
            }
        }

        ans

    }
}
