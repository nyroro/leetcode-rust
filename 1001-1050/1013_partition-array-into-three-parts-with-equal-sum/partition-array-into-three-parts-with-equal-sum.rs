
impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 {
            return false;
        }
        
        let part_sum = sum / 3;
        let mut i = 0;
        let mut j = arr.len() - 1;
        let mut left_sum = arr[i];
        let mut right_sum = arr[j];
        
        while i + 1 < j {
            if left_sum == part_sum && right_sum == part_sum {
                return true;
            }
            
            if left_sum != part_sum {
                i += 1;
                left_sum += arr[i];
            }
            
            if right_sum != part_sum {
                j -= 1;
                right_sum += arr[j];
            }
        }
        
        false

    }
}
