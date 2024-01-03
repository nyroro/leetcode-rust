
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        
        let n = candy_type.len();
        let max_candies = n / 2;
        
        let unique_candies: HashSet<&i32> = candy_type.iter().collect();
        let num_unique_candies = unique_candies.len();
        
        num_unique_candies.min(max_candies) as i32

    }
}
