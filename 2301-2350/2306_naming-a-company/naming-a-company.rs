
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut table: HashMap<char, HashSet<&str>> = HashMap::new();

        // Populate the HashMap with the first letter as key and the rest of the string as value

        for t in &ideas {
            let (first, rest) = t.split_at(1);
            table.entry(first.chars().next().unwrap()).or_insert(HashSet::new()).insert(rest);
        }

        let mut ret = 0;

        // Iterate through all possible pairs of distinct letters

        for i in b'a'..=b'z' {
            for j in b'a'..=b'z' {
                if i == j {
                    continue;
                }
                let a = char::from(i);
                let b = char::from(j);

                let s1 = table.get(&a).unwrap_or(&HashSet::new());
                let s2 = table.get(&b).unwrap_or(&HashSet::new());

                let common: HashSet<_> = s1.intersection(s2).cloned().collect();
                let v1: HashSet<_> = s1.difference(&common).cloned().collect();
                let v2: HashSet<_> = s2.difference(&common).cloned().collect();

                ret += (v1.len() * v2.len()) as i64;
            }
        }

        ret

    }
}
