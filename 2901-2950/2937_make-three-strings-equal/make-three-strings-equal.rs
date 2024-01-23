
impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let mut ret = -1;
        let mut i = 0;
        for ((c1, c2), c3) in s1.chars().zip(s2.chars()).zip(s3.chars()) {
            if c1 == c2 && c1 == c3 {
                i += 1;
            } else {
                break;
            }
        }
        if i > 0 {
            ret = (s1.len() - i + s2.len() - i + s3.len() - i) as i32;
        }
        ret

    }
}
