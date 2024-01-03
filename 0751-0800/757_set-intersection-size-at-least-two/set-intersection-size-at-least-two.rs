
impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[1].cmp(&b[1]).then_with(|| b[0].cmp(&a[0])));
        let mut result = 0;
        let mut last = vec![-1, -1];
        
        for interval in intervals {
            if interval[0] > last[1] {
                result += 2;
                last = vec![interval[1] - 1, interval[1]];
            } else if interval[0] > last[0] {
                result += 1;
                last[0] = last[1];
                last[1] = interval[1];
            }
        }
        
        result

    }
}
