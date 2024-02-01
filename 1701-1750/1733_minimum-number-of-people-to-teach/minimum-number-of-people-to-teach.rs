
impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashSet, HashMap};

        let mut incompatible = HashSet::new();
        for f in &friendships {
            let lang1 = &languages[(f[0] - 1) as usize];
            let lang2 = &languages[(f[1] - 1) as usize];
            if lang1.iter().all(|l| !lang2.contains(l)) {
                incompatible.insert(f[0]);
                incompatible.insert(f[1]);
            }
        }

        let mut language_count = HashMap::new();
        for i in incompatible {
            for lang in &languages[(i - 1) as usize] {
                *language_count.entry(lang).or_insert(0) += 1;
            }
        }

        let max_count = language_count.values().max().unwrap_or(&0);
        incompatible.len() as i32 - max_count

    }
}
