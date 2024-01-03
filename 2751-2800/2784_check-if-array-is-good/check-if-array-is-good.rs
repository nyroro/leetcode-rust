
impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        // 找到数组 nums 中的最大值

        let max_num = *nums.iter().max().unwrap();
        
        // 判断 max_num 是否等于数组 nums 的长度减一

        if max_num != nums.len() as i32 - 1 {
            return false;
        }
        
        // 判断 nums 是否是 base[max_num] 的一个排列

        let mut count = vec![0; max_num as usize + 1];
        for &num in &nums {
            count[num as usize] += 1;
        }
        for i in 1..max_num as usize {
            if count[i] != 1 {
                return false;
            }
        }
        if count[max_num as usize] != 2 {
            return false;
        }
        
        true

    }
}
