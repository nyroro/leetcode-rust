
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut sorted_target = target.clone();
        let mut sorted_arr = arr.clone();
        
        sorted_target.sort();
        sorted_arr.sort();
        
        sorted_target == sorted_arr

    }
}
