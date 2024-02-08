
impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut ans = 0;
        let mut n = s.len();
        let mut k = String::new();
        let c1 = if x > y { 'a' } else { 'b' };
        let c2 = if c1 == 'a' { 'b' } else { 'a' };

        for c in s.chars() {
            if !k.is_empty() && k.chars().last().unwrap() == c1 && c == c2 {
                ans += std::cmp::max(x, y);
                k.pop();
            } else {
                k.push(c);
            }
        }

        let mut s = k.clone();  // Create a mutable copy of k

        n = s.len();
        k.clear();

        for c in s.chars() {
            if !k.is_empty() && k.chars().last().unwrap() == c2 && c == c1 {
                ans += std::cmp::min(x, y);
                k.pop();
            } else {
                k.push(c);
            }
        }

        ans

    }
}
