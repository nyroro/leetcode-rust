
struct RangeModule {
    ranges: Vec<(i32, i32)>,
}

impl RangeModule {
    fn new() -> Self {
        RangeModule {
            ranges: Vec::new(),
        }
    }
    
    fn add_range(&mut self, left: i32, right: i32) {
        let mut merged = Vec::new();
        let mut i = 0;
        while i < self.ranges.len() && self.ranges[i].1 < left {
            merged.push(self.ranges[i]);
            i += 1;
        }
        while i < self.ranges.len() && self.ranges[i].0 <= right {
            left = left.min(self.ranges[i].0);
            right = right.max(self.ranges[i].1);
            i += 1;
        }
        merged.push((left, right));
        while i < self.ranges.len() {
            merged.push(self.ranges[i]);
            i += 1;
        }
        self.ranges = merged;
    }
    
    fn query_range(&self, left: i32, right: i32) -> bool {
        for range in &self.ranges {
            if range.0 <= left && range.1 >= right {
                return true;
            }
        }
        false

    }
    
    fn remove_range(&mut self, left: i32, right: i32) {
        let mut merged = Vec::new();
        for range in &self.ranges {
            if range.1 <= left || range.0 >= right {
                merged.push(*range);
            } else {
                if range.0 < left {
                    merged.push((range.0, left));
                }
                if range.1 > right {
                    merged.push((right, range.1));
                }
            }
        }
        self.ranges = merged;
    }
}
