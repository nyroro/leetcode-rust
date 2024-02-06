
use std::collections::HashMap;



impl Solution {
    pub fn find_valid_split(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return -1;
        }

        let mut mp: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &v) in nums.iter().enumerate() {
            let mut d = 2;
            let mut value = v;

            while d * d <= value {
                if value % d == 0 {
                    if let Some(pos) = mp.get_mut(&d) {
                        if pos.len() == 0 {
                            pos.push(i);
                        } else if pos.len() == 1 {
                            pos.push(i);
                        } else {
                            pos[1] = i;
                        }
                    } else {
                        mp.insert(d, vec![i]);
                    }

                    value /= d;

                    while value % d == 0 {
                        value /= d;
                    }
                }
                d += 1;
            }

            if value > 1 {
                if let Some(pos) = mp.get_mut(&value) {
                    if pos.len() == 0 {
                        pos.push(i);
                    } else if pos.len() == 1 {
                        pos.push(i);
                    } else {
                        pos[1] = i;
                    }
                } else {
                    mp.insert(value, vec![i]);
                }
            }
        }

        let mut intervals: Vec<&Vec<usize>> = mp.values().filter(|v| v.len() == 2).collect();
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        if intervals.is_empty() || intervals[0][0] > 0 {
            return 0;
        }

        let mut mx = intervals[0][1];

        for i in 1..intervals.len() {
            if intervals[i][0] > mx {
                return mx as i32;
            }
            mx = mx.max(intervals[i][1]);
        }

        if mx < n - 1 {
            return mx as i32;
        }

        -1

    }
}
