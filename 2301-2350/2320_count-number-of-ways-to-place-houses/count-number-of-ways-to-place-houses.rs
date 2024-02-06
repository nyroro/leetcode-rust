
impl Solution {
    pub fn count_house_placements(n: i32) -> i64 {
        let (mut prev, mut pprev) = (2i64, 1i64);
        for _ in 1..n {
            let temp = (pprev + prev) % (10i64.pow(9) + 7);
            pprev = prev;
            prev = temp;
        }
        (prev.pow(2) % (10i64.pow(9) + 7)) as i64

    }
}
