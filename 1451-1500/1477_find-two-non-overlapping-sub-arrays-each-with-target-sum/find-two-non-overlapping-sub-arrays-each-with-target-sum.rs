
use std::collections::BinaryHeap;

struct SubArray {
    len: i32,
    start: usize,
}

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr1: Vec<SubArray> = Vec::new();
        let mut index: usize = 0;
        let mut arr2: BinaryHeap<i32> = BinaryHeap::new();
        let mut l: usize = 0;
        let mut s: i32 = 0;
        let mut ret: i32 = -1;

        for r in 0..arr.len() {
            s += arr[r];
            if s == target {
                add_result(&mut arr1, &mut index, &mut arr2, l, r);
                s -= arr[l];
                l += 1;
            } else if s > target {
                while s > target && l <= r {
                    s -= arr[l];
                    l += 1;
                }
                if l <= r && s == target {
                    add_result(&mut arr1, &mut index, &mut arr2, l, r);
                    s -= arr[l];
                    l += 1;
                }
            }
        }
        ret

    }

    fn add_result(arr1: &mut Vec<SubArray>, index: &mut usize, arr2: &mut BinaryHeap<i32>, l: usize, r: usize) {
        arr1.push(SubArray { len: (r - l + 1) as i32, start: l });
        while *index < arr1.len() && arr1[*index].start + arr1[*index].len as usize <= l {
            arr2.push(arr1[*index].len);
            *index += 1;
        }
        if let Some(min_len) = arr2.peek() {
            if ret == -1 {
                ret = *min_len + (r - l + 1) as i32;
            } else {
                ret = ret.min(*min_len + (r - l + 1) as i32);
            }
        }
    }
}
