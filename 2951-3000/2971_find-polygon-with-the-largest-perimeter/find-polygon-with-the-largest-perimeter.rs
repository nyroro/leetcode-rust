


impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut ans = -1;
        let mut cur: i64 = nums[0] as i64 + nums[1] as i64;

        for &num in nums.iter().skip(2) {
            if cur > num as i64 {
                ans = cur + num as i64;
            }
            cur += num as i64;
        }

        ans

    }
}
