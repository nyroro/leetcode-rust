
impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        if k == 0 {
            if num % 10 == 0 {
                return 1;
            } else {
                return -1;
            }
        }
        let mut s = std::collections::HashSet::new();
        let mut x = k;
        let t = num % 10;
        while !s.contains(&(x % 10)) && x % 10 != t {
            s.insert(x % 10);
            x += k;
        }
        if x % 10 == t && x <= num {
            return (s.len() + 1) as i32;
        } else {
            return -1;
        }
    }
}
