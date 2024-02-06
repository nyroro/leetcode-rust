
use std::collections::HashMap;



impl Solution {
    pub fn possibly_equals(s1: String, s2: String) -> bool {
        fn parse_int(s: &str, i: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
            (i..s.len()).scan(0, move |n, j| {
                if s[j..].chars().next().unwrap().is_digit(10) {
                    *n = *n * 10 + s[j..].chars().next().unwrap().to_digit(10).unwrap() as usize;
                    Some((*n, j - i + 1))
                } else {
                    None

                }
            })
        }

        fn dfs(i: usize, j: usize, skip: isize, s1: &str, s2: &str, memo: &mut HashMap<(usize, usize, isize), bool>) -> bool {
            if let Some(&result) = memo.get(&(i, j, skip)) {
                return result;
            }

            let result = if i == s1.len() && j == s2.len() {
                skip == 0

            } else if i < s1.len() && s1[i..].chars().next().unwrap().is_digit(10) {
                parse_int(&s1, i).any(|(n, k)| dfs(i + k, j, skip - n as isize, s1, s2, memo))
            } else if j < s2.len() && s2[j..].chars().next().unwrap().is_digit(10) {
                parse_int(&s2, j).any(|(n, k)| dfs(i, j + k, skip + n as isize, s1, s2, memo))
            } else if skip == 0 && i < s1.len() && j < s2.len() {
                s1[i..=i] == s2[j..=j] && dfs(i + 1, j + 1, skip, s1, s2, memo)
            } else if skip > 0 && i < s1.len() {
                dfs(i + 1, j, skip - 1, s1, s2, memo)
            } else if skip < 0 && j < s2.len() {
                dfs(i, j + 1, skip + 1, s1, s2, memo)
            } else {
                false

            };

            memo.insert((i, j, skip), result);
            result

        }

        let mut memo = HashMap::new();
        dfs(0, 0, 0, &s1, &s2, &mut memo)
    }
}
