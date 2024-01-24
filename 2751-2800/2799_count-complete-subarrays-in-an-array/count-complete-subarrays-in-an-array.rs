
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut ret = 0;
        let s: HashSet<i32> = nums.iter().cloned().collect();
        let m = s.len();
        let n = nums.len();

        for i in 0..n {
            let mut ss = HashSet::new();
            for j in i..n {
                ss.insert(nums[j]);
                if ss.len() == m {
                    ret += (n - j) as i32;
                    break;
                }
            }
        }
        ret

    }
}
