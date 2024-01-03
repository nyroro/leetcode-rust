
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_by_key(|e| e[0]); // Sort events by start day

        let mut max_day = 0;
        for event in &events {
            max_day = max_day.max(event[1] as usize); // Find the maximum day

        }
        let mut attended = 0;
        let mut day = 1;
        let mut event_index = 0;
        let mut pq = BinaryHeap::new(); // Create a max heap to store the end days of events

        while day <= max_day {
            while event_index < events.len() && events[event_index][0] == day as i32 {
                pq.push(Reverse(events[event_index][1])); // Add the end day of the event to the heap

                event_index += 1;
            }
            while let Some(Reverse(end)) = pq.pop() {
                if end >= day as i32 {
                    attended += 1;
                    break;
                }
            }
            day += 1;
        }
        attended

    }
}
