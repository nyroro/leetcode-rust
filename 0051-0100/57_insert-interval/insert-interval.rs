
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut start = new_interval[0];
        let mut end = new_interval[1];
        let mut inserted = false;
        
        for interval in intervals {
            if interval[1] < start {
                result.push(interval);
            } else if interval[0] > end {
                if !inserted {
                    result.push(vec![start, end]);
                    inserted = true;
                }
                result.push(interval);
            } else {
                start = start.min(interval[0]);
                end = end.max(interval[1]);
            }
        }
        
        if !inserted {
            result.push(vec![start, end]);
        }
        
        result

    }
}
