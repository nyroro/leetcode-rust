
impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut change = vec![1; n];

        for i in 0..n {
            let idx = (i as i32 - nums[i] + 1 + n as i32) as usize % n;
            change[idx] -= 1;
        }

        let mut acc_change = vec![0; n];
        let mut acc = 0;
        for (i, &val) in change.iter().enumerate() {
            acc += val;
            acc_change[i] = acc;
        }

        let max_val = *acc_change.iter().max().unwrap();
        let result = acc_change.iter().position(|&x| x == max_val).unwrap() as i32;

        result

    }
}
