
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let num_str = num.to_string();
        let mut min_sum = i32::MAX;

        for i in 1..num_str.len() {
            let (left, right) = num_str.split_at(i);
            let left_num = left.parse::<i32>().unwrap();
            let right_num = right.parse::<i32>().unwrap();
            min_sum = min_sum.min(left_num + right_num);
        }

        min_sum

    }
}
