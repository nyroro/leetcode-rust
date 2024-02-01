
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![-1; intervals.len()];
        let mut map = std::collections::BTreeMap::new();
        
        for (i, interval) in intervals.iter().enumerate() {
            map.insert(interval[0], i);
        }
        
        for (i, interval) in intervals.iter().enumerate() {
            let end = interval[1];
            let mut min_start = std::i32::MAX;
            
            for (&start, &idx) in map.range(end..) {
                min_start = min_start.min(start);
                result[i] = idx as i32;
                break;
            }
            
            if min_start == std::i32::MAX {
                result[i] = -1;
            }
        }
        
        result

    }
}
