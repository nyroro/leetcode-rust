
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut remainder = vec![0; 60];
        
        for t in time {
            let r = t % 60;
            let complement = (60 - r) % 60;
            count += remainder[complement as usize];
            remainder[r as usize] += 1;
        }
        
        count

    }
}
