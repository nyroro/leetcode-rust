


impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let mut res = 0;
        let mut tab = vec![1];

        for i in 1..s.len() {
            if s.chars().nth(i) == s.chars().nth(i - 1) {
                tab.push(1);
            } else {
                let last = tab.len() - 1;
                tab[last] += 1;
            }
        }

        for j in 1..tab.len() {
            res = res.max(tab[j] + tab[j - 1]);
        }

        res.max(tab[0]) as i32

    }
}
