
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut left = *weights.iter().max().unwrap() as i32;
        let mut right = weights.iter().sum::<i32>();

        while left < right {
            let mid = left + (right - left) / 2;
            let mut need = 1;
            let mut cur = 0;
            for &weight in &weights {
                if cur + weight > mid {
                    need += 1;
                    cur = 0;
                }
                cur += weight;
            }
            if need <= days {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left

    }
}
