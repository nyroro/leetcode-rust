
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut cnt = vec![0; 26];
        for c in s.chars() {
            cnt[c as usize - 'a' as usize] += 1;
        }
        
        let mut ans = String::new();
        let mut stk = Vec::new();
        let mut mi = 'a';
        
        for c in s.chars() {
            cnt[c as usize - 'a' as usize] -= 1;
            while mi < 'z' && cnt[mi as usize - 'a' as usize] == 0 {
                mi = (mi as u8 + 1) as char;
            }
            stk.push(c);
            while !stk.is_empty() && stk[stk.len() - 1] <= mi {
                ans.push(stk.pop().unwrap());
            }
        }
        
        ans

    }
}
