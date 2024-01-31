


impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i64 {
        let mut max_sum = 0;

        let n = nums.len() as i64; // Change to i64

        for i in 1..=n {
            let mut total = 0;
            for j in 1..=n {
                let next_ele = i * j * j;
                if next_ele > n {
                    break;
                }
                total += nums[(next_ele - 1) as usize] as i64; // Convert to i64

            }
            max_sum = max_sum.max(total);
        }

        max_sum

    }
}
