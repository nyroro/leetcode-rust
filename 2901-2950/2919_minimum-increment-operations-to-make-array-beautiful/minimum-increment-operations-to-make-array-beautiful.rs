


impl Solution {
    pub fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
        let mut f: i64 = 0;
        let mut g: i64 = 0;
        let mut h: i64 = 0;

        for x in nums {
            let new_h = std::cmp::min(std::cmp::min(f, g), h) + std::cmp::max(k as i64 - x as i64, 0);
            f = g;
            g = h;
            h = new_h;
        }

        std::cmp::min(std::cmp::min(f, g), h)
    }
}
