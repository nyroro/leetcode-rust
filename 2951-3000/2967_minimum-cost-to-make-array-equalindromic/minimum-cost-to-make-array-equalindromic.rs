
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i64 {
        fn is_palindromic(mut x: i32) -> bool {
            let s = x.to_string();
            s == s.chars().rev().collect::<String>()
        }

        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mid = sorted_nums[sorted_nums.len() / 2];
        let mut x = mid;
        let mut y = mid;

        while !is_palindromic(x) {
            x -= 1;
        }
        while !is_palindromic(y) {
            y += 1;
        }

        let mut ans1 = 0;
        let mut ans2 = 0;
        for &t in &sorted_nums {
            ans1 += (t - x).abs() as i64;
            ans2 += (t - y).abs() as i64;
        }

        ans1.min(ans2)
    }
}
