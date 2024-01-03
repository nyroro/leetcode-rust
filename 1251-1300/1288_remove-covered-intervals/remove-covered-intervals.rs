
impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else {
                b[1].cmp(&a[1])
            }
        });

        let mut count = 1;
        let mut left = intervals[0][0];
        let mut right = intervals[0][1];

        for i in 1..intervals.len() {
            let interval = &intervals[i];
            if interval[0] > left && interval[1] > right {
                left = interval[0];
                right = interval[1];
                count += 1;
            } else if interval[0] <= left && interval[1] > right {
                right = interval[1];
            }
        }

        count

    }
}
