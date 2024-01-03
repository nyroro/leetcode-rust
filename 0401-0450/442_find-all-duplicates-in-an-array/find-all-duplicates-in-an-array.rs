
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut set = std::collections::HashSet::new();
        let mut result = Vec::new();
        
        for num in nums {
            if set.contains(&num) {
                result.push(num);
            } else {
                set.insert(num);
            }
        }
        
        result

    }
}
