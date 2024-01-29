
use std::cmp;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let mut gc = 0;
        for &i in nums_divide.iter() {
            gc = gcd(gc, i);
        }
        let mut sorted_nums = nums;
        sorted_nums.sort();

        for (i, &num) in sorted_nums.iter().enumerate() {
            if gc % num == 0 {
                return i as i32;
            }
        }
        -1

    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a

}
