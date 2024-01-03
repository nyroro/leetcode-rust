
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        
        for num in arr {
            // 检查是否存在num的两倍或者num的一半

            if set.contains(&(num * 2)) || (num % 2 == 0 && set.contains(&(num / 2))) {
                return true;
            }
            
            // 将num添加到集合中

            set.insert(num);
        }
        
        false

    }
}
