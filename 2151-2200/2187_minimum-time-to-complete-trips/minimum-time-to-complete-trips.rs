
impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut l: i64 = 0;
        let mut r: i64 = i64::from(time.iter().min().unwrap()) * i64::from(total_trips);
        
        while l < r {
            let mid = l + (r - l) / 2;
            if Self::count_trips(&time, mid) < total_trips as i64 {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        
        l

    }
    
    fn count_trips(time: &Vec<i32>, mid: i64) -> i64 {
        let mut res: i64 = 0;
        for t in time {
            res += mid / i64::from(*t);
        }
        res

    }
}
