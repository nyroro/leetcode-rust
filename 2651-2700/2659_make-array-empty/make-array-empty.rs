


impl Solution {
    pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
        let mut sorted_index: Vec<usize> = (0..nums.len()).collect();
        sorted_index.sort_by_key(|&i| nums[i]);

        let mut s = 0;
        let mut curr = 0;
        let mut curr_loop_subtract = 0;
        let mut looped_subtract = 0;

        for &ind in &sorted_index {
            if curr <= ind {
                s += (ind - curr + 1) as i64;
                curr = ind + 1;
                curr_loop_subtract += 1;
            } else {
                s += (ind + nums.len() - curr + 1) as i64;
                curr = ind + 1;
                s -= looped_subtract;
                looped_subtract += curr_loop_subtract;
                curr_loop_subtract = 1;
            }
        }

        s -= (curr as i64 - curr_loop_subtract);
        s

    }
}
