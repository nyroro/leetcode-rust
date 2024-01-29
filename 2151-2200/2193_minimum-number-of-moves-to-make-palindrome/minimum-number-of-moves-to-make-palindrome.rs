
impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let mut l = 0;
        let mut r = s.len() - 1;
        let mut res = 0;
        let mut st: Vec<char> = s.chars().collect();

        while l < r {
            if st[l] != st[r] {
                let mut i = r;
                while i > l && st[l] != st[i] {
                    i -= 1;
                }
                if i == l {
                    st[i..=i+1].swap(0, 1);
                    res += 1;
                    continue;
                } else {
                    while i < r {
                        st[i..=i+1].swap(0, 1);
                        i += 1;
                        res += 1;
                    }
                }
            }
            l += 1;
            r -= 1;
        }
        res

    }
}
