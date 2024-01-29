
use std::collections::HashSet;



impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let nums_set: HashSet<i32> = nums.into_iter().collect();
        let mut ans = 0;
        let m = *nums_set.iter().max().unwrap_or(&0);
        
        for x in 1..=m {
            let mut g = 0;
            for xx in (x..=m).step_by(x as usize) {
                if nums_set.contains(&xx) {
                    g = gcd(g, xx);
                }
            }
            if g == x {
                ans += 1;
            }
        }
        
        ans

    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a

}
