
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut c = c;
        for i in 2..=c {
            let mut count = 0;
            while c % i == 0 {
                count += 1;
                c /= i;
            }
            if i % 4 == 3 && count % 2 != 0 {
                return false;
            }
        }
        return c % 4 != 3;
    }
}
