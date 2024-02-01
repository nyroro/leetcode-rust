
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut deletions = 0;
        let mut b_count = 0;

        for ch in s.chars() {
            if ch == 'a' {
                if b_count > 0 {
                    b_count -= 1;
                    deletions += 1;
                }
            } else {
                b_count += 1;
            }
        }

        deletions

    }
}
