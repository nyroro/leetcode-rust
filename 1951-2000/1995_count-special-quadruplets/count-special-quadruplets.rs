
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;

        for a in 0..n-3 {
            for b in a+1..n-2 {
                for c in b+1..n-1 {
                    let sum = nums[a] + nums[b] + nums[c];
                    for d in c+1..n {
                        if nums[d] == sum {
                            count += 1;
                        }
                    }
                }
            }
        }

        count

    }
}
