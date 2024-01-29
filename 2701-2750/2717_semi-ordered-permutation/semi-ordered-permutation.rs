


impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let i = nums.iter().position(|&x| x == 1).unwrap() as i32;
        let j = nums.iter().position(|&x| x == n).unwrap() as i32;
        let ans = i + n - 1 - j - (i > j) as i32;
        ans

    }
}
