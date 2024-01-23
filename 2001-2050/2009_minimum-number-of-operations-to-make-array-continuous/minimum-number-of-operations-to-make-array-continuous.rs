


impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut arr = nums.clone();
        arr.sort();
        arr.dedup();
        let n = nums.len() as i32;
        let mut ret = n - 1;
        for i in 0..arr.len() {
            let x = arr[i] + n;
            let j = match arr.binary_search(&x) {
                Ok(pos) => pos,
                Err(pos) => pos,
            };
            ret = ret.min(n - j as i32 + i as i32);
        }
        ret

    }
}
