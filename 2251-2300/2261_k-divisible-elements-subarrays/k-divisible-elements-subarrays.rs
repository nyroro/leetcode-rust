
impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        let mut sub_arrays = std::collections::HashSet::new();
        let n = nums.len();

        for start in 0..n {
            let mut cnt = 0;
            let mut temp = String::new();
            for i in start..n {
                if nums[i] % p == 0 {
                    cnt += 1;
                }
                temp.push_str(&nums[i].to_string());
                temp.push(',');
                if cnt > k {
                    break;
                }
                sub_arrays.insert(temp.clone());
            }
        }

        sub_arrays.len() as i32

    }
}
