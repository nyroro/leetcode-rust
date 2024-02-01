use std::collections::HashMap;

impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        let mut indexs: HashMap<String, usize> = HashMap::new();
        let mut cnt = 0;
        let mut pre: Vec<usize> = Vec::new();

        fn index(indexs: &mut HashMap<String, usize>, pre: &mut Vec<usize>, cnt: &mut usize, word: &str) -> usize {
            if !indexs.contains_key(word) {
                let current = *cnt;
                *cnt += 1;
                indexs.insert(word.to_string(), current);
                pre.push(current);
                current

            } else {
                *indexs.get(word).unwrap()
            }
        }

        fn root(pre: &mut Vec<usize>, x: usize) -> usize {
            if pre[x] == x {
                x

            } else {
                pre[x] = root(pre, pre[x]);
                pre[x]
            }
        }

        for word in &words {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_unstable();
            let sorted_word: String = chars.into_iter().collect();
            for i in 0..word.len() {
                let mut w = sorted_word.clone();
                w.remove(i);
                let w1 = index(&mut indexs, &mut pre, &mut cnt, &w);
                let w2 = index(&mut indexs, &mut pre, &mut cnt, &sorted_word);
                let r1 = root(&mut pre, w1);
                let r2 = root(&mut pre, w2);
                pre[r1] = r2;
            }
        }

        let mut ans: HashMap<usize, i32> = HashMap::new();
        for word in &words {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_unstable();
            let sorted_word: String = chars.into_iter().collect();
            let r = root(&mut pre, *indexs.get(&sorted_word).unwrap());
            *ans.entry(r).or_insert(0) += 1;
        }

        let max_group_size = *ans.values().max().unwrap_or(&0);
        vec![ans.len() as i32, max_group_size]
    }
}
