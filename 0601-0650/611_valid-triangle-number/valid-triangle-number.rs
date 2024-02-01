


impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                for k in (j + 1)..nums.len() {
                    if nums[i] + nums[j] > nums[k] {
                        count += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        count

    }
}

fn main() {
    let nums1 = vec![2, 2, 3, 4];
    let nums2 = vec![4, 2, 3, 4];
    println!("Output for nums1: {}", Solution::triangle_number(nums1));
    println!("Output for nums2: {}", Solution::triangle_number(nums2));
}
