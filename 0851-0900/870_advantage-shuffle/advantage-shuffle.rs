
struct IndexedNum {
    index: usize,
    value: i32,
}

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1.iter().enumerate().map(|(i, &v)| IndexedNum { index: i, value: v }).collect::<Vec<IndexedNum>>();
        let nums2 = nums2.iter().enumerate().map(|(i, &v)| IndexedNum { index: i, value: v }).collect::<Vec<IndexedNum>>();

        nums1.sort_by_key(|x| x.value);
        let mut nums2 = nums2;
        nums2.sort_by_key(|x| x.value);

        let mut result = vec![-1; nums1.len()];
        let mut remaining = Vec::new();
        let mut j = 0;

        for i in 0..nums1.len() {
            if nums1[i].value > nums2[j].value {
                result[nums2[j].index] = nums1[i].value;
                j += 1;
            } else {
                remaining.push(nums1[i].value);
            }
        }

        for i in 0..result.len() {
            if result[i] == -1 {
                result[i] = remaining.pop().unwrap();
            }
        }

        result

    }
}
