
impl Solution {
    pub fn check_array(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut b: Vec<i32> = Vec::new();
        let mut p: Vec<i32> = vec![0];

        for (i, &x) in nums.iter().enumerate() {
            let prev_sum_index = i as i32 - k + 1;
            let prev_sum = if prev_sum_index >= 0 {
                p[i - k as usize + 1]
            } else {
                0

            };
            let current_b = x - (p[i] - prev_sum);
            b.push(current_b);
            p.push(p[i] + current_b);
        }

        b.iter().all(|&x| x >= 0) && !(0..k - 1).any(|i| b[n - i as usize - 1] != 0)
    }
}
