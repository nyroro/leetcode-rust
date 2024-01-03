
impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut distinct_or: HashSet<i32> = HashSet::new();
        let mut cur_or: HashSet<i32> = HashSet::new();

        for num in arr {
            let mut new_cur_or: HashSet<i32> = HashSet::new();
            new_cur_or.insert(num);

            for &val in &cur_or {
                new_cur_or.insert(val | num);
            }

            cur_or = new_cur_or.clone();
            distinct_or.extend(cur_or.iter());
        }

        distinct_or.len() as i32

    }
}
