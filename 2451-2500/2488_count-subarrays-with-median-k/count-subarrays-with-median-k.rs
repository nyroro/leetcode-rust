
use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut arr = Vec::new();
        for &n in nums.iter() {
            if n < k {
                arr.push(-1);
            } else if n > k {
                arr.push(1);
            } else {
                arr.push(0);
            }
        }

        let mut book_pre: HashMap<i32, i32> = HashMap::new();
        book_pre.insert(0, 1);
        let mut agg = 0;
        let mut check = false;
        let mut ans = 0;

        for &a in arr.iter() {
            if a == 0 {
                check = true;
            }
            agg += a;
            if check {
                ans += book_pre.get(&agg).unwrap_or(&0) + book_pre.get(&(agg - 1)).unwrap_or(&0);
            } else {
                *book_pre.entry(agg).or_insert(0) += 1;
            }
        }

        ans as i32

    }
}
