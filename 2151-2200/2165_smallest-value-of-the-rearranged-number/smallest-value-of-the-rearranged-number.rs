
impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        let mut num_str = num.to_string();
        let mut t = std::collections::BTreeMap::new();
        
        for c in num_str.chars() {
            *t.entry(c).or_insert(0) += 1;
        }
        
        if num < 0 {
            let mut ans = 0;
            for i in (0..=9).rev() {
                let c = (i as u8 + b'0') as char;
                let x = *t.get(&c).unwrap_or(&0);
                for _ in 0..x {
                    ans = ans * 10 + i;
                }
            }
            return -ans;
        } else {
            let mut ans = 0;
            for i in 1..=9 {
                let c = (i as u8 + b'0') as char;
                let x = *t.get(&c).unwrap_or(&0);
                if x > 0 {
                    ans = ans * 10 + i;
                    *t.get_mut(&c).unwrap() -= 1;
                    break;
                }
            }
            for i in 0..=9 {
                let c = (i as u8 + b'0') as char;
                let x = *t.get(&c).unwrap_or(&0);
                for _ in 0..x {
                    ans = ans * 10 + i;
                }
            }
            return ans;
        }
    }
}
