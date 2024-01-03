
impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut start = start_value;
        let mut end = target;
        let mut result = 0;
        
        while start < end {
            if end % 2 == 0 {
                end /= 2;
            } else {
                end += 1;
            }
            result += 1;
        }
        
        result + start - end

    }
}
