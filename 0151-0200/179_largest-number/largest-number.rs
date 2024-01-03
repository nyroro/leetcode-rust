
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_str: Vec<String> = nums.iter().map(|&num| num.to_string()).collect();
        nums_str.sort_unstable_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));
        let result: String = nums_str.join("");
        if result.starts_with("0") {
            return "0".to_string();
        }
        result

    }
}
