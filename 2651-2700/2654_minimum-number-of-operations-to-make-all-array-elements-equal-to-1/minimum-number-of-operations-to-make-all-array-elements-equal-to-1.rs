
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ct = 0;
        let n = nums.len() as i32;

        for &d in &nums {
            if d == 1 {
                ct += 1;
            }
        }

        if ct > 0 {
            return n - ct;
        }

        let mut steps = 0;
        let mut nums = nums;

        while steps < n {
            for i in 0..(n - steps - 1) as usize {
                nums[i] = gcd(nums[i], nums[i + 1]);
                if nums[i] == 1 {
                    return (steps + n) as i32;
                }
            }
            steps += 1;
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
