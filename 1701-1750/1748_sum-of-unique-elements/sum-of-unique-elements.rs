
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        // 创建一个 HashMap 来存储每个元素的出现次数

        let mut count_map: HashMap<i32, i32> = HashMap::new();

        // 遍历数组，统计每个元素的出现次数

        for num in &nums {
            *count_map.entry(*num).or_insert(0) += 1;
        }

        // 计算唯一元素的和

        let mut sum = 0;
        for (num, count) in count_map {
            if count == 1 {
                sum += num;
            }
        }

        sum

    }
}
