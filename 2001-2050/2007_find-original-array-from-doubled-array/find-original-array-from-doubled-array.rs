
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut original = Vec::new();
        let mut count_map = HashMap::new();

        for &num in &changed {
            *count_map.entry(num).or_insert(0) += 1;
        }

        let mut sorted_keys: Vec<i32> = count_map.keys().cloned().collect();
        sorted_keys.sort_unstable();

        for &num in &sorted_keys {
            let count = *count_map.get(&num).unwrap_or(&0);
            if count == 0 {
                continue;
            }

            let double = num * 2;
            let double_count = *count_map.get(&double).unwrap_or(&0);

            if num == 0 {
                if count % 2 != 0 {
                    return vec![];
                } else {
                    original.extend(std::iter::repeat(0).take(count / 2));
                }
            } else if double_count < count {
                return vec![];
            } else {
                original.extend(std::iter::repeat(num).take(count));
                *count_map.get_mut(&double).unwrap() -= count;
            }
        }

        original

    }
}
