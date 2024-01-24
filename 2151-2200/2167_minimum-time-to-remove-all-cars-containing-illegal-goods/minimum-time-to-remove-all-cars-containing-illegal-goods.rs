
impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut left = vec![std::i32::MAX; s.len()];
        let mut right = vec![std::i32::MAX; s.len()];

        // 从左到右填充left数组

        if s[0] == '0' {
            left[0] = 0;
        } else {
            left[0] = 1;
        }
        for i in 1..s.len() {
            if s[i] == '0' {
                left[i] = left[i - 1];
            } else {
                left[i] = std::cmp::min(i as i32 + 1, left[i - 1] + 2);
            }
        }

        // 从右到左填充right数组

        if s[s.len() - 1] == '0' {
            right[s.len() - 1] = 0;
        } else {
            right[s.len() - 1] = 1;
        }
        for i in (0..s.len() - 1).rev() {
            if s[i] == '0' {
                right[i] = right[i + 1];
            } else {
                right[i] = std::cmp::min((s.len() - i) as i32, right[i + 1] + 2);
            }
        }

        // 找到最小的移除时间

        let mut min_time = std::i32::MAX;
        for i in 0..s.len() - 1 {
            min_time = std::cmp::min(min_time, left[i] + right[i + 1]);
        }

        std::cmp::min(min_time, left[s.len() - 1])
    }
}
