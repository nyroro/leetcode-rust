
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut ans: Vec<i64> = Vec::new();
        let mut pq: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        let mut sum: i64 = 0;

        for i in 0..n {
            if nums[i] > 0 {
                sum += nums[i] as i64;
            }
            nums[i] = nums[i].abs();
        }

        nums.sort();

        pq.push((sum - nums[0] as i64, 0));
        ans.push(sum);

        while ans.len() < k as usize {
            if let Some((val, index)) = pq.pop() {
                ans.push(val);
                if index + 1 < n {
                    pq.push((val + nums[index] as i64 - nums[index + 1] as i64, index + 1));
                    pq.push((val - nums[index + 1] as i64, index + 1));
                }
            }
        }

        ans[(k - 1) as usize]
    }
}
