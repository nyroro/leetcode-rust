
impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; 100000];
        for &num in &nums {
            count[num as usize] += 1;
        }
        
        let mut ans = 0;
        let mut taken = 0;
        for i in 0..100000 {
            if count[i] >= 2 {
                taken += count[i] - 1;
                ans -= i as i32 * (count[i] - 1) as i32;
            } else if taken > 0 && count[i] == 0 {
                taken -= 1;
                ans += i as i32;
            }
        }
        
        ans

    }
}
