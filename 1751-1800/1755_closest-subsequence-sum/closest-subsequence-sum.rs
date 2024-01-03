
impl Solution {
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let m = n / 2;
        let mut left = Vec::new();
        let mut right = Vec::new();

        // 计算左半部分所有可能的和

        for i in 0..(1 << m) {
            let mut sum = 0;
            for j in 0..m {
                if (i >> j) & 1 == 1 {
                    sum += nums[j];
                }
            }
            left.push(sum);
        }

        // 计算右半部分所有可能的和

        for i in 0..(1 << (n - m)) {
            let mut sum = 0;
            for j in 0..(n - m) {
                if (i >> j) & 1 == 1 {
                    sum += nums[m + j];
                }
            }
            right.push(sum);
        }

        // 对右半部分的和进行排序

        right.sort();

        let mut result = std::i32::MAX;

        // 遍历左半部分的和，寻找最接近目标值的和

        for &x in &left {
            let y = goal - x;
            let idx = right.binary_search(&y).unwrap_or_else(|e| e);
            if idx < right.len() {
                result = result.min((right[idx] - y).abs());
            }
            if idx > 0 {
                result = result.min((right[idx - 1] - y).abs());
            }
        }

        result

    }
}
