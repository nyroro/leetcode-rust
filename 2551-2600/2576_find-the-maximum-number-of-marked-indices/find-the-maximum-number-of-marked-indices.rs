
impl Solution {
    pub fn max_num_of_marked_indices(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let n = nums.len();
        let mut marked = vec![0; n];
        let mut j = n - 1;

        for i in (0..n/2).rev() {
            if marked[i] == 1 {
                continue;
            }
            let mut found = false;
            while j > i && marked[j] == 1 {
                j -= 1;
            }
            if j == i {
                continue;
            }
            if nums[i] * 2 <= nums[j] {
                marked[i] = 1;
                marked[j] = 1;
                j -= 1;
            }
        }

        marked.iter().sum()
    }
}
