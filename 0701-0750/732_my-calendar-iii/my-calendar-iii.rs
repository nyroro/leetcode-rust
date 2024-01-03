
use std::collections::BTreeMap;

struct MyCalendarThree {
    bookings: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree {
            bookings: BTreeMap::new(),
        }
    }
    
    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        *self.bookings.entry(start_time).or_insert(0) += 1;
        *self.bookings.entry(end_time).or_insert(0) -= 1;
        
        let mut max_k = 0;
        let mut k = 0;
        
        for (_, count) in &self.bookings {
            k += count;
            max_k = max_k.max(k);
        }
        
        max_k

    }
}
