
impl Solution {
    pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        use std::collections::HashMap;

        let mut max_sum: i64 = 0;
        let mut sum: i64 = 0;
        let mut distinct_count = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            sum += num as i64;
            *distinct_count.entry(num).or_insert(0) += 1;

            if i as i32 >= k {
                let start_num = nums[i as usize - k as usize];
                sum -= start_num as i64;
                *distinct_count.get_mut(&start_num).unwrap() -= 1;
                if distinct_count[&start_num] == 0 {
                    distinct_count.remove(&start_num);
                }
            }

            if distinct_count.len() as i32 >= m {
                max_sum = max_sum.max(sum);
            }
        }

        max_sum

    }
}
