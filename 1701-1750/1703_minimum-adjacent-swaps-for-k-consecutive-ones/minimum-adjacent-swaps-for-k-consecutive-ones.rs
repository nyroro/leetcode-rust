
impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }
        
        let mut ones = 0;
        let half = (k + 1) >> 1;
        let (mut left_sum, mut right_sum) = (0, 0);
        let (mut left, mut right, mut mid) = (0, 0, 0);
        
        for (i, &num) in nums.iter().enumerate() {
            if num == 0 {
                continue;
            }
            ones += 1;
            if ones < half {
                left_sum += i as i32;
                if ones == 1 {
                    left = i as i32;
                }
            } else if ones == half {
                mid = i as i32;
            } else {
                right_sum += i as i32;
                if ones == k {
                    right = i as i32;
                    break;
                }
            }
        }
        
        if k == 2 {
            left = mid;
        }
        
        let (left_half, right_half) = if k & 1 == 1 {
            (half - 1, half - 1)
        } else {
            (half - 1, half)
        };
        
        let left_offset = (left_half * (left_half - 1)) >> 1;
        let right_offset = (right_half * (right_half - 1)) >> 1;
        
        let mut res = (
            left_half * (mid - 1) - left_sum - left_offset +
            right_sum - right_half * (mid + 1) - right_offset

        );
        
        let n = nums.len() as i32;
        
        while right < n {
            right += 1;
            while right < n && nums[right as usize] == 0 {
                right += 1;
            }
            if right == n {
                break;
            }
            right_sum += right;
            left_sum -= left;
            left += 1;
            while nums[left as usize] == 0 {
                left += 1;
            }
            left_sum += mid;
            mid += 1;
            while nums[mid as usize] == 0 {
                mid += 1;
            }
            right_sum -= mid;
            let cur = (
                left_half * (mid - 1) - left_sum - left_offset +
                right_sum - right_half * (mid + 1) - right_offset

            );
            res = res.min(cur);
        }
        
        res

    }
}
