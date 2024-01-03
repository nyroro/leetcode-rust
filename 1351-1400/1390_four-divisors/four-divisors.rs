


impl Solution {
    fn count_divisors(num: i32) -> i32 {
        let mut count = 0;
        let mut i = 1;
        while i * i <= num {
            if num % i == 0 {
                count += 2;
                if i * i == num {
                    count -= 1;
                }
            }
            i += 1;
        }
        count

    }

    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for &num in nums.iter() {
            if Solution::count_divisors(num) == 4 {
                sum += (1..=num).filter(|x| num % x == 0).sum::<i32>();
            }
        }
        sum

    }
}
