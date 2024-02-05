
impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut res: Vec<f64> = Vec::new();
        let mut temp: Vec<i32> = nums.iter().take(k as usize - 1).cloned().collect();
        temp.sort();

        for j in k as usize - 1..nums.len() {
            let insert_pos = temp.binary_search(&nums[j]).unwrap_or_else(|x| x);
            temp.insert(insert_pos, nums[j]);
            let median = if k % 2 == 0 {
                let mid1 = temp[k as usize / 2 - 1];
                let mid2 = temp[k as usize / 2];
                (mid1 as f64 + mid2 as f64) / 2.0

            } else {
                temp[k as usize / 2] as f64

            };
            res.push(median);
            if let Ok(remove_pos) = temp.binary_search(&nums[j - k as usize + 1]) {
                temp.remove(remove_pos);
            }
        }

        res

    }
}
