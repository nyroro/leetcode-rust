


impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort(); // Sort the events based on start time


        let size = events.len();
        let mut curr_max = 0;
        let mut max_value = vec![0; size]; // Precompute the maximum values


        for i in (0..size).rev() {
            curr_max = curr_max.max(events[i][2]);
            max_value[i] = curr_max;
        }

        let mut max_two_events = 0;
        let mut curr_two_events = 0;
        let mut next_ind;

        for i in 0..size {
            curr_two_events = events[i][2];
            next_ind = Solution::upper_bound(&events, events[i][1], size);
            if next_ind < size {
                curr_two_events += max_value[next_ind];
            }
            max_two_events = max_two_events.max(curr_two_events);
        }

        max_two_events

    }

    fn upper_bound(events: &Vec<Vec<i32>>, key: i32, size: usize) -> usize {
        let mut low = 0;
        let mut high = size - 1;
        let mut mid;

        while low <= high {
            mid = (low + high) / 2;
            if events[mid][0] > key {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        low

    }
}
