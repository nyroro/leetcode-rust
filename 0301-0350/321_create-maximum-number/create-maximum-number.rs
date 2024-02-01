
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let m = nums1.len();
        let n = nums2.len();
        let mut result = vec![0; k as usize];

        for i in 0..=k.min(m as i32) {
            if k - i >= 0 && k - i <= n as i32 {
                let subsequence1 = Self::get_max_subsequence(&nums1, i as usize);
                let subsequence2 = Self::get_max_subsequence(&nums2, (k - i) as usize);
                let merged = Self::merge(subsequence1, subsequence2);
                if Self::compare(&merged, &result) {
                    result = merged;
                }
            }
        }

        result

    }

    fn get_max_subsequence(nums: &[i32], k: usize) -> Vec<i32> {
        let mut stack = Vec::new();
        let to_remove = nums.len() - k;

        for &num in nums {
            while !stack.is_empty() && num > *stack.last().unwrap() && to_remove > 0 {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(num);
        }

        stack.truncate(k);
        stack

    }

    fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut merged = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i..] > nums2[j..] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        }

        merged.extend_from_slice(&nums1[i..]);
        merged.extend_from_slice(&nums2[j..]);

        merged

    }

    fn compare(nums1: &[i32], nums2: &[i32]) -> bool {
        for (num1, num2) in nums1.iter().zip(nums2.iter()) {
            if num1 > num2 {
                return true;
            } else if num1 < num2 {
                return false;
            }
        }

        false

    }
}
