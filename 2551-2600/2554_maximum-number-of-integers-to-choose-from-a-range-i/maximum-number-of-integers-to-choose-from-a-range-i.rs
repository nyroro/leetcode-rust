
impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut max_count = 0;
        let mut sum = 0;
        
        for i in 1..=n {
            if !banned.contains(&i) {
                sum += i;
                if sum <= max_sum {
                    max_count += 1;
                } else {
                    break;
                }
            }
        }
        
        max_count

    }
}
