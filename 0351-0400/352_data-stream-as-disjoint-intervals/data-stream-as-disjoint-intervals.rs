
struct SummaryRanges {
    intervals: Vec<Vec<i32>>,
}

impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            intervals: Vec::new(),
        }
    }
    
    fn add_num(&mut self, value: i32) {
        let mut merged = false;
        for interval in &mut self.intervals {
            let start = interval[0];
            let end = interval[1];
            if value >= start && value <= end {
                // Value is already in an existing interval

                merged = true;
                break;
            } else if value == end + 1 {
                // Value extends the end of an existing interval

                interval[1] = value;
                merged = true;
                break;
            } else if value == start - 1 {
                // Value extends the start of an existing interval

                interval[0] = value;
                merged = true;
                break;
            }
        }
        
        if !merged {
            // Value is a new interval

            self.intervals.push(vec![value, value]);
        }
        
        // Merge overlapping intervals

        self.intervals.sort();
        let mut i = 0;
        while i < self.intervals.len() - 1 {
            let current_end = self.intervals[i][1];
            let next_start = self.intervals[i + 1][0];
            if current_end >= next_start - 1 {
                self.intervals[i][1] = self.intervals[i + 1][1];
                self.intervals.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
    
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.clone()
    }
}
