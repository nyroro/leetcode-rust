
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let mut count = 0;
        for i in 0..=limit {
            for j in 0..=limit {
                for k in 0..=limit {
                    if i + j + k == n && i <= limit && j <= limit && k <= limit {
                        count += 1;
                    }
                }
            }
        }
        count

    }
}
