
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        
        let mut index_sum: HashMap<&str, usize> = HashMap::new();
        let mut min_index_sum = usize::MAX;
        
        // 遍历list1，将字符串及其索引和存储到index_sum中

        for (i, s) in list1.iter().enumerate() {
            index_sum.insert(s.as_str(), i);
        }
        
        let mut result: Vec<String> = Vec::new();
        
        // 遍历list2，查找与list1中字符串相同的字符串，并计算索引和

        for (i, s) in list2.iter().enumerate() {
            if let Some(j) = index_sum.get(s.as_str()) {
                let index_sum = i + j;
                
                // 如果当前索引和小于最小索引和，则更新最小索引和和结果数组

                if index_sum < min_index_sum {
                    min_index_sum = index_sum;
                    result.clear();
                    result.push(s.clone());
                } else if index_sum == min_index_sum {
                    // 如果当前索引和等于最小索引和，则将字符串添加到结果数组中

                    result.push(s.clone());
                }
            }
        }
        
        result

    }
}
