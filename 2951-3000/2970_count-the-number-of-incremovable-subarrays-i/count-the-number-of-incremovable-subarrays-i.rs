
impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for left in 0..nums.len() {
            for right in left..nums.len() {
                let mut valid = true;
                let mut a = Vec::new();
                a.extend_from_slice(&nums[0..left]);
                a.extend_from_slice(&nums[right + 1..]);
                if a.is_empty() {
                    count += 1;
                } else {
                    for k in 0..a.len() - 1 {
                        if a[k] >= a[k + 1] {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        count += 1;
                    }
                }
            }
        }
        count

    }
}
