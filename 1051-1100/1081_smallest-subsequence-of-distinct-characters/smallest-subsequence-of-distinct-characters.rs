
impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut count: [usize; 26] = [0; 26];
        let mut visited: [bool; 26] = [false; 26];

        for ch in s.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }

        for ch in s.chars() {
            count[ch as usize - 'a' as usize] -= 1;

            if visited[ch as usize - 'a' as usize] {
                continue;
            }

            while let Some(&top) = stack.last() {
                if top > ch && count[top as usize - 'a' as usize] > 0 {
                    visited[top as usize - 'a' as usize] = false;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(ch);
            visited[ch as usize - 'a' as usize] = true;
        }

        stack.iter().collect()
    }
}
