
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut freq = HashMap::new();
        let mut append = HashMap::new();
        for &x in nums.iter() {
            *freq.entry(x).or_insert(0) += 1;
        }
        for &x in nums.iter() {
            if *freq.get(&x).unwrap() == 0 {
                continue;
            } else if *append.get(&x).unwrap_or(&0) > 0 {
                *append.entry(x).and_modify(|e| *e -= 1).or_insert(0);
                *append.entry(x + 1).and_modify(|e| *e += 1).or_insert(1);
            } else if *freq.get(&(x + 1)).unwrap_or(&0) > 0 && *freq.get(&(x + 2)).unwrap_or(&0) > 0 {
                *freq.entry(x + 1).and_modify(|e| *e -= 1).or_insert(0);
                *freq.entry(x + 2).and_modify(|e| *e -= 1).or_insert(0);
                *append.entry(x + 3).and_modify(|e| *e += 1).or_insert(1);
            } else {
                return false;
            }
            *freq.entry(x).and_modify(|e| *e -= 1).or_insert(0);
        }
        true

    }
}
