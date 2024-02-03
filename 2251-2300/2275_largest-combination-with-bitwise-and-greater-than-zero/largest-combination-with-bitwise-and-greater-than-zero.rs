


impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut max_count = 0;

        for i in 0..24 {
            let mut bit_count = 0;
            for j in 0..candidates.len() {
                if (candidates[j] & (1 << i)) > 0 {
                    bit_count += 1;
                }
            }
            max_count = max_count.max(bit_count);
        }

        max_count

    }
}
