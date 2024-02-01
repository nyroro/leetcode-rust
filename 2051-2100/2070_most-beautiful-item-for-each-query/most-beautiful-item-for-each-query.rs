
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items;
        items.sort_by(|a, b| b[1].cmp(&a[1]).then(b[0].cmp(&a[0])));

        let mut arr: VecDeque<Vec<i32>> = VecDeque::new();
        for t in items {
            if arr.is_empty() {
                arr.push_back(t);
                continue;
            }
            if t[1] == arr.back().unwrap()[1] {
                if t[0] < arr.back().unwrap()[0] {
                    arr.back_mut().unwrap()[0] = t[0];
                }
            } else if t[0] < arr.back().unwrap()[0] {
                arr.push_back(t);
            }
        }

        let mut q: Vec<(usize, i32)> = queries.iter().cloned().enumerate().map(|(i, val)| (i, val)).collect();
        q.sort_by_key(|&(i, val)| val);

        let mut ret: Vec<i32> = vec![0; q.len()];
        for i in (0..q.len()).rev() {
            let (index, t) = q[i];
            while !arr.is_empty() && arr.front().unwrap()[0] > t {
                arr.pop_front();
            }
            if !arr.is_empty() {
                ret[index] = arr.front().unwrap()[1];
            }
        }
        ret

    }
}
