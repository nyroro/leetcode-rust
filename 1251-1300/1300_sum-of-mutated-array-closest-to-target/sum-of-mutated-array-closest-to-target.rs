
impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = *arr.iter().max().unwrap();
        
        while left < right {
            let mid = (left + right) / 2;
            let sum: i32 = arr.iter().map(|&x| if x > mid { mid } else { x }).sum();
            
            if sum < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        if (arr.iter().map(|&x| if x > left { left } else { x }).sum::<i32>() - target).abs() < (arr.iter().map(|&x| if x > left - 1 { left - 1 } else { x }).sum::<i32>() - target).abs() {
            left

        } else {
            left - 1

        }
    }
}
