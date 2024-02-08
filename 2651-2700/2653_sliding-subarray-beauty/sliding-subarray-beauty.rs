
use std::collections::HashMap;
use std::convert::TryInto;

impl Solution {
    pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k_usize: usize = k.try_into().unwrap();
        let x_usize: usize = x.try_into().unwrap();
        let mut arr1 = vec![0; 101];
        let mut arr2 = vec![0; nums.len() - k_usize + 1];
        let (mut index, mut count, mut temp) = (0, 0, 0);

        for i in 0..k_usize - 1 {
            arr1[(nums[i] + 50) as usize] += 1;
        }

        for i in k_usize - 1..nums.len() {
            arr1[(nums[i] + 50) as usize] += 1;
            for j in 0..101 {
                count += arr1[j];
                if x_usize <= count {
                    temp = j as i32 - 50;
                    if temp > 0 {
                        arr2[index] = 0;
                        break;
                    } else {
                        arr2[index] = temp;
                        break;
                    }
                }
            }
            count = 0;
            arr1[(nums[i - k_usize + 1] + 50) as usize] -= 1;
            index += 1;
        }
        arr2

    }
}
