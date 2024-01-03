
use std::collections::HashMap;



impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let mut seen = HashMap::new();
        let mut password = String::new();
        for _ in 0..n {
            password.push('0' as u8 as char);
        }
        seen.insert(password.clone(), false);
        Solution::dfs(&mut password, n, k, &mut seen);
        password

    }

    fn dfs(password: &mut String, n: i32, k: i32, seen: &mut HashMap<String, bool>) {
        let target = (k.pow(n as u32)) as usize;
        if seen.len() == target {
            return;
        }
        let prefix = password.len() - (n - 1) as usize;
        for i in 0..k {
            let next = format!("{}{}", &password[prefix..], i);
            if !seen.contains_key(&next) {
                seen.insert(next.clone(), false);
                password.push((i as u8 + b'0') as char);
                Solution::dfs(password, n, k, seen);
                if seen.len() == target {
                    return;
                }
                seen.remove(&next);
                password.pop();
            }
        }
    }
}
