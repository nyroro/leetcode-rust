
impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        // 创建一个结构体来存储数字和它们的映射值

        struct MappedNum {
            original: i32,
            mapped: i32,
        }
        
        // 将nums中的每个数字映射到它们的映射值，并存储在一个元组中

        let mut mapped_nums: Vec<MappedNum> = nums.iter().map(|&num| {
            let mapped = num.to_string().chars().map(|c| mapping[c.to_digit(10).unwrap() as usize]).fold(0, |acc, x| acc * 10 + x);
            MappedNum { original: num, mapped: mapped }
        }).collect();
        
        // 根据映射值对元组进行排序

        mapped_nums.sort_by_key(|x| x.mapped);
        
        // 提取原始数字并返回排序后的数字数组

        mapped_nums.iter().map(|x| x.original).collect()
    }
}
