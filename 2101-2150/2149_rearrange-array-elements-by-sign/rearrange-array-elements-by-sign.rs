
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        // 创建两个新的数组，一个用于存储正整数，另一个用于存储负整数

        let mut positive_nums: Vec<i32> = Vec::new();
        let mut negative_nums: Vec<i32> = Vec::new();
        
        // 遍历给定的数组，并根据正负性将整数放入相应的数组中

        for &num in nums.iter() {
            if num > 0 {
                positive_nums.push(num);
            } else {
                negative_nums.push(num);
            }
        }
        
        // 交替从这两个数组中取出整数，以确保相邻的整数具有相反的符号

        let mut result: Vec<i32> = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < positive_nums.len() && j < negative_nums.len() {
            result.push(positive_nums[i]);
            result.push(negative_nums[j]);
            i += 1;
            j += 1;
        }
        
        // 将剩余的整数放回原始的 `nums` 数组中

        result.append(&mut positive_nums[i..].to_vec());
        result.append(&mut negative_nums[j..].to_vec());
        
        // 返回修改后的数组

        result

    }
}
