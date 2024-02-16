
use std::collections::BinaryHeap;



impl Solution {
    pub fn min_operations(nums: Vec<i32>, mut target: i32) -> i32 {
        let mut sum: i64 = 0;
        for &num in &nums {
            sum += num as i64;
        }

        if sum < target as i64 {
            return -1;
        }

        let mut pq = BinaryHeap::from(nums);
        let mut ops = 0;

        while target > 0 {
            if let Some(curr) = pq.pop() {
                sum -= curr as i64;
                if curr <= target {
                    target -= curr;
                } else if curr > target && sum < target as i64 {
                    ops += 1;
                    sum += curr as i64;
                    pq.push((curr / 2) as i32);
                    pq.push((curr / 2) as i32);
                }
                if pq.is_empty() && target != 0 {
                    if curr > target && curr != 1 {
                        ops += 1;
                        sum += curr as i64;
                        pq.push((curr / 2) as i32);
                        pq.push((curr / 2) as i32);
                    } else {
                        return -1;
                    }
                }
            }
        }

        ops

    }
}
