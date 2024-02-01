


impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut sums = vec![0; len + 1];
        sums[0] = nums[0];
        for i in 1..nums.len() {
            sums[i] = sums[i - 1] + nums[i];
        }
        Solution::merge(&mut sums, lower, upper, 0, len)
    }

    fn merge(sums: &mut Vec<i32>, lower: i32, upper: i32, low: usize, high: usize) -> i32 {
        if high <= low {
            return 0;
        }
        if low + 1 == high {
            if sums[low] <= upper && sums[low] >= lower {
                return 1;
            } else {
                return 0;
            }
        }
        let mid = (low + high) / 2;
        let mut x = mid;
        let mut y = mid;
        let mut ret = Solution::merge(sums, lower, upper, low, mid) + Solution::merge(sums, lower, upper, mid, high);
        for i in low..mid {
            while x < high && sums[x] - sums[i] < lower {
                x += 1;
            }
            while y < high && sums[y] - sums[i] <= upper {
                y += 1;
            }
            ret += (y - x) as i32;
        }
        sums[low..high].sort();
        ret

    }
}
