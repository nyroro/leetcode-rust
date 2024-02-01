
impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let mut left = vec![std::i32::MAX; s.len()];
        let mut right = vec![std::i32::MAX; s.len()];

        // 从左到右填充left数组

        let mut count = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '0' {
                left[i] = count;
            } else {
                left[i] = count + 2;
            }
            count = left[i];
        }

        // 从右到左填充right数组

        let mut count = 0;
        for (i, c) in s.chars().rev().enumerate() {
            if c == '0' {
                right[i] = count;
            } else {
                right[i] = count + 2;
            }
            count = right[i];
        }

        // 找到最小的移除时间

        let mut min_time = std::i32::MAX;
        for i in 0..s.len()-1 {
            min_time = min_time.min(left[i] + right[s.len()-i-2]);
        }

        min_time

    }
}
