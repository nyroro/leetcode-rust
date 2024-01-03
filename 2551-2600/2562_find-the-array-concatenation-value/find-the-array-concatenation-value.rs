
impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut conc_val: i64 = 0;
        let mut nums = nums;

        while !nums.is_empty() {
            if nums.len() > 1 {
                let first = nums.remove(0);
                let last = nums.pop().unwrap();
                let concat_val = format!("{}{}", first, last).parse::<i64>().unwrap();
                conc_val += concat_val;
            } else {
                conc_val += nums[0] as i64;
                nums.clear();
            }
        }

        conc_val

    }
}
