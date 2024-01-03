
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by_key(|interval| interval[0]); // 按照起始位置排序

        let mut result: Vec<Vec<i32>> = Vec::new();
        
        for interval in intervals {
            if let Some(last) = result.last_mut() {
                if last[1] >= interval[0] {
                    last[1] = last[1].max(interval[1]); // 合并区间

                    continue;
                }
            }
            result.push(interval); // 添加新的区间

        }
        
        result

    }
}
