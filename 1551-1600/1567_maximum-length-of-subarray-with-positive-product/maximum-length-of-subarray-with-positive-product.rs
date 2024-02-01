
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut pos_len = 0;
        let mut neg_len = 0;
        let mut sign = 1;

        for &num in nums.iter() {
            if num > 0 {
                pos_len += 1;
                if neg_len > 0 {
                    neg_len += 1;
                }
            } else if num < 0 {
                let temp = pos_len;
                pos_len = neg_len + 1;
                neg_len = temp + 1;
            } else {
                pos_len = 0;
                neg_len = 0;
                sign = 1;
            }

            if pos_len > 0 {
                max_len = max_len.max(pos_len);
            } else if neg_len > 0 {
                max_len = max_len.max(neg_len);
            }
        }

        max_len

    }
}
