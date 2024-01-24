


impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut arr: Vec<(i32, i32)> = Vec::new();
        let mut cnt = 0;
        let mut ret = 0;
        
        for i in (0..nums.len()).rev() {
            cnt = 0;
            while arr.len() > 0 && nums[i] > arr.last().unwrap().0 {
                cnt = std::cmp::max(cnt + 1, arr.last().unwrap().1);
                arr.pop();
            }
            ret = std::cmp::max(ret, cnt);
            arr.push((nums[i], cnt));
        }
        
        ret

    }
}
