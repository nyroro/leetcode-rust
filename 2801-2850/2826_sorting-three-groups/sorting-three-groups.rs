


impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut w = vec![0, 0, 0];
        for v in nums {
            w = vec![
                w[0] + if v != 1 { 1 } else { 0 },
                std::cmp::min(w[0], w[1]) + if v != 2 { 1 } else { 0 },
                *w.iter().min().unwrap() + if v != 3 { 1 } else { 0 },
            ];
        }
        *w.iter().min().unwrap()
    }
}
