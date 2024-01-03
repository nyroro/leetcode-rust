
impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        for i in 0..=num {
            let reverse = i.to_string().chars().rev().collect::<String>();
            let reverse_num = reverse.parse::<i32>().unwrap_or(0);
            if i + reverse_num == num {
                return true;
            }
        }
        false

    }
}
