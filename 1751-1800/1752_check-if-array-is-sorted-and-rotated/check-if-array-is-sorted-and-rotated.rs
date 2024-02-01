
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        for i in 0..n {
            let next = (i + 1) % n;
            if nums[i] > nums[next] {
                let mut is_sorted = true;
                for j in 0..n {
                    let next = (i + j) % n;
                    let next_next = (i + j + 1) % n;
                    if nums[next] > nums[next_next] {
                        is_sorted = false;
                        break;
                    }
                }
                return is_sorted;
            }
        }
        true

    }
}
