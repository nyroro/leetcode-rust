
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut deletion_count = 0;
        let mut sorted = vec![false; strs.len() - 1];

        for col in 0..strs[0].len() {
            let mut is_sorted = true;
            for i in 0..strs.len() - 1 {
                if !sorted[i] && strs[i].as_bytes()[col] > strs[i + 1].as_bytes()[col] {
                    is_sorted = false;
                    break;
                }
            }
            if !is_sorted {
                deletion_count += 1;
            } else {
                for i in 0..strs.len() - 1 {
                    if strs[i].as_bytes()[col] < strs[i + 1].as_bytes()[col] {
                        sorted[i] = true;
                    }
                }
            }
        }

        deletion_count

    }
}
