
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; 10001];
        for num in nums {
            count[num as usize] += num;
        }
        
        let mut dp1 = 0;
        let mut dp2 = 0;
        
        for i in 1..=10000 {
            let temp = dp1;
            dp1 = dp1.max(dp2);
            dp2 = temp + count[i];
        }
        
        dp1.max(dp2)
    }
}
