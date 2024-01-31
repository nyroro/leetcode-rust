
impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut ii = *nums.iter().min().unwrap() as i64;
        let mut j = *nums.iter().max().unwrap() as i64;
        
        while ii < j {
            let mid = (ii + j) / 2;
            let mut total1 = 0;
            let mut total2 = 0;
            
            for i in 0..nums.len() {
                total1 += (mid - nums[i] as i64).abs() * cost[i] as i64;
                total2 += (mid + 1 - nums[i] as i64).abs() * cost[i] as i64;
            }
            
            if total1 < total2 {
                j = mid;
            } else {
                ii = mid + 1;
            }
        }
        
        let mut ans = 0;
        
        for i in 0..nums.len() {
            ans += (ii - nums[i] as i64).abs() * cost[i] as i64;
        }
        
        ans

    }
}
