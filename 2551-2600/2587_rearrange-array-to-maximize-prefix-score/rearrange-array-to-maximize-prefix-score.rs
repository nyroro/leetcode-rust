
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let mut negative_nums = BinaryHeap::new();
        let mut score = 0;
        let mut psum = 0;
        let mut zeros = 0;

        for num in nums {
            if num > 0 {
                score += 1;
                psum += num as i64;
            } else if num == 0 {
                zeros += 1;
            } else {
                negative_nums.push(Reverse(num.abs() as i64));
            }
        }

        if psum > 0 {
            score += zeros;
        }

        while let Some(Reverse(n)) = negative_nums.pop() {
            psum -= n;
            if psum > 0 {
                score += 1;
            } else {
                break;
            }
        }

        score

    }
}
