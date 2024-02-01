
use std::collections::HashMap;

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut table = HashMap::new();
        for (i, &t) in nums1.iter().enumerate() {
            table.insert(t, i);
        }

        let mut nums2_mapped = Vec::new();
        for &t in &nums2 {
            nums2_mapped.push(*table.get(&t).unwrap());
        }

        let mut arr = vec![0; nums1.len() + 1];
        let mut s = vec![0; nums1.len()];

        fn cal(arr: &Vec<i32>, x: usize) -> i64 {
            let mut ret = 0;
            let mut x = x as i32;
            while x > 0 {
                ret += arr[x as usize];
                x -= x & -x;
            }
            ret as i64

        }

        fn update(arr: &mut Vec<i32>, mut x: usize, val: i32) {
            let mut x = x as i32;
            while x <= arr.len() as i32 {
                arr[x as usize] += val;
                x += x & -x;
            }
        }

        for (i, &t) in nums2_mapped.iter().enumerate() {
            s[i] = cal(&arr, t + 1);
            update(&mut arr, t + 1, 1);
        }

        arr = vec![0; nums1.len() + 1];
        let mut b = vec![0; nums1.len()];

        for i in (0..nums2_mapped.len()).rev() {
            let t = nums2_mapped[i];
            b[i] = cal(&arr, nums2.len()) - cal(&arr, t);
            update(&mut arr, t + 1, 1);
        }

        let mut ret = 0;
        for (x, y) in s.iter().zip(b.iter()) {
            ret += x * y;
        }
        ret

    }
}
