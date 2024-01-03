
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut index = 0;
        
        for i in 1..=n {
            if index == target.len() {
                break;
            }
            if target[index] == i {
                result.push("Push".to_string());
                index += 1;
            } else {
                result.push("Push".to_string());
                result.push("Pop".to_string());
            }
        }
        
        result

    }
}
