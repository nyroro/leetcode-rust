
/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation

 * struct MountainArray;
 * impl MountainArray {
 *     fn get(index: i32) -> i32;
 *     fn length() -> i32;
 * }
 */

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let mut left = 0;
        let mut right = mountain_arr.length() - 1;
        
        // Find the peak index

        while left < right {
            let mid = (left + right) / 2;
            if mountain_arr.get(mid) < mountain_arr.get(mid + 1) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        let peak = left;
        
        // Search in the ascending part

        left = 0;
        right = peak;
        while left < right {
            let mid = (left + right) / 2;
            if mountain_arr.get(mid) == target {
                return mid;
            } else if mountain_arr.get(mid) < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        // Search in the descending part

        left = peak;
        right = mountain_arr.length() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if mountain_arr.get(mid) == target {
                return mid;
            } else if mountain_arr.get(mid) > target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        return -1;
    }
}
