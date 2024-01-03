
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut record: Vec<i32> = Vec::new();
        let mut sum = 0;
        
        for op in operations {
            match op.as_str() {
                "C" => {
                    if let Some(last) = record.pop() {
                        sum -= last;
                    }
                },
                "D" => {
                    if let Some(last) = record.last() {
                        let double = 2 * last;
                        sum += double;
                        record.push(double);
                    }
                },
                "+" => {
                    if let Some(last) = record.windows(2).last() {
                        let new_score = last.iter().sum();
                        sum += new_score;
                        record.push(new_score);
                    }
                },
                _ => {
                    let score = op.parse::<i32>().unwrap();
                    sum += score;
                    record.push(score);
                }
            }
        }
        
        sum

    }
}
