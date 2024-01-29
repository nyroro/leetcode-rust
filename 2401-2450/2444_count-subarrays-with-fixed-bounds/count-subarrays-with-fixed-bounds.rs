
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut count = 0; // total amount

        let mut last_min_k: Option<usize> = None; // at initialization we don't have such elements

        let mut last_max_k: Option<usize> = None; // at initialization we don't have such elements

        let mut left_most = 0; // we have no elements before which violates constraints, llb value


        for (inx, &n) in nums.iter().enumerate() {
            if n < min_k || n > max_k { // this element violates constraints

                // last_min_k and last_max_k can't be included in any window after inx

                left_most = inx + 1;
                last_min_k = None;
                last_max_k = None;
            } else {
                if n == min_k {
                    last_min_k = Some(inx);
                }
                if n == max_k {
                    last_max_k = Some(inx);
                }
                // we have both max_k and min_k so let's calculate the number of subarrays ended at inx

                if let (Some(last_min), Some(last_max)) = (last_min_k, last_max_k) {
                    let right_most_left_border = last_min.min(last_max);
                    count += (right_most_left_border - left_most + 1) as i64;
                }
            }
        }

        count

    }
}
