
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr1: Vec<(i32, usize, usize)> = Vec::new();
        let mut index: usize = 0;
        let mut arr2: BinaryHeap<i32> = BinaryHeap::new();
        let mut l: usize = 0;
        let mut s: i32 = 0;
        let mut ret: i32 = -1;

        for r in 0..arr.len() {
            s += arr[r];
            if s == target {
                Self::add_result(&mut arr1, &mut index, &mut arr2, l, r, &mut ret);
                s -= arr[l];
                l += 1;
            } else if s > target {
                while s > target && l <= r {
                    s -= arr[l];
                    l += 1;
                }
                if l <= r && s == target {
                    Self::add_result(&mut arr1, &mut index, &mut arr2, l, r, &mut ret);
                    s -= arr[l];
                    l += 1;
                }
            }
        }
        ret

    }

    fn add_result(arr1: &mut Vec<(i32, usize, usize)>, index: &mut usize, arr2: &mut BinaryHeap<i32>, l: usize, r: usize, ret: &mut i32) {
        arr1.push(((r - l + 1) as i32, l, r));
        while *index < arr1.len() && arr1[*index].2 < l {
            arr2.push(-arr1[*index].0);
            *index += 1;
        }
        if let Some(top_len) = arr2.peek() {
            if *ret == -1 {
                *ret = -*top_len + (r - l + 1) as i32;
            } else {
                *ret = (*ret).min(-*top_len + (r - l + 1) as i32);
            }
        }
    }
}
