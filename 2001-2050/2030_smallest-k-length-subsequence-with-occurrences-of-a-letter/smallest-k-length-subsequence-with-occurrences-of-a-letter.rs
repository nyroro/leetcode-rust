
impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let cnt = s.chars().filter(|&c| c == letter).count();
        let (mut lcnt, mut stack) = (0, Vec::new());
        for (i, t) in s.chars().enumerate() {
            while !stack.is_empty() && stack.len() + s.len() as usize - i - 1 >= k as usize && stack.last() > Some(&t) {
                if stack.last() == Some(&letter) {
                    if lcnt + cnt - 1 < repetition {
                        break;
                    }
                    lcnt -= 1;
                }
                stack.pop();
            }
            if t == letter || stack.len() - lcnt < (k - repetition) as usize {
                stack.push(t);
            }
            if t == letter {
                lcnt += 1;
                cnt -= 1;
            }
        }
        stack.iter().take(k as usize).collect()
    }
}
