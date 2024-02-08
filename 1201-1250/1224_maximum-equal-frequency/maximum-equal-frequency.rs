
use std::collections::HashMap;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let mut mp: HashMap<i32, i32> = HashMap::new(); // freq of elements

        let mut mpp: HashMap<i32, i32> = HashMap::new(); // freq of freq


        for i in 0..nums.len() {
            let old = mp.get(&nums[i]).unwrap_or(&0).clone(); // prev freq

            *mp.entry(nums[i]).or_insert(0) += 1;

            if mpp.get(&old).unwrap_or(&0) > &0 {
                *mpp.entry(old).or_insert(0) -= 1;
            }
            if mpp.get(&old).unwrap_or(&0) == &0 {
                mpp.remove(&old);
            }
            *mpp.entry(*mp.get(&nums[i]).unwrap_or(&0)).or_insert(0) += 1;

            let a = *mpp.keys().min().unwrap_or(&0);
            let b = *mpp.keys().max().unwrap_or(&0);

            if mpp.len() == 1 && (a == 1 || mpp[&a] == 1) {
                ans = ans.max(i as i32 + 1);
            } else if mpp.len() == 2 && (b == a + 1 && mpp[&b] == 1 || mpp[&a] == 1 && a == 1) {
                ans = ans.max(i as i32 + 1);
            }
        }
        ans

    }
}
