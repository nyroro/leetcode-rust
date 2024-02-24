
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct NumWithHalf {
    num: f64,
    half: f64,
}

impl Eq for NumWithHalf {}

impl PartialOrd for NumWithHalf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.half.partial_cmp(&other.half).unwrap())
    }
}

impl Ord for NumWithHalf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.half.partial_cmp(&other.half).unwrap()
    }
}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0f64;
        let mut pq = BinaryHeap::new();

        for &num in nums.iter() {
            let half = num as f64 / 2.0;
            sum += num as f64;
            pq.push(NumWithHalf { num: num as f64, half });
        }

        let mut s = sum;
        let mut ans = 0;

        while let Some(NumWithHalf { num, half }) = pq.pop() {
            s -= half;
            if s <= sum / 2.0 {
                ans += 1;
                break;
            }
            ans += 1;
            pq.push(NumWithHalf { num: half, half: half / 2.0 });
        }

        ans

    }
}
