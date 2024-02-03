impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let m = 1000000007_i64;
        let mut nums = nums.iter()
            .zip(s.chars())
            .map(|(&num, ch)| num as i64 + ((ch as i64) - 79) / 3 * d as i64)
            .collect::<Vec<i64>>();
        nums.sort();
        let idxs = (1 - nums.len() as i64..nums.len() as i64).step_by(2).map(|idx| ((2*m + idx) % m) as i64);
        let nums = nums.iter().map(|&num| ((100*m + num as i64) % m) as i64);
        let result = nums.zip(idxs).map(
                |(num, idx)| (num as i64 * idx as i64)%m as i64
            )
            .fold(0_i64, |acc, (num)| (acc as i64 + num)%m as i64);
        
        result as i32
    }
}
