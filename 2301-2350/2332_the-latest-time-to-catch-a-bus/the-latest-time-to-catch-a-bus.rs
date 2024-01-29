
use std::collections::HashSet;

impl Solution {
    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let mut buses = buses;
        let mut passengers = passengers;
        buses.sort();
        passengers.sort();

        let mut index = 0;
        let mut start = 0;

        let mut passengers_set: HashSet<i32> = passengers.iter().cloned().collect();

        for &bus_start in &buses {
            let mut cap = capacity;
            while index < passengers.len() && bus_start >= passengers[index] && cap > 0 {
                cap -= 1;
                index += 1;
            }

            let latest = if cap > 0 { bus_start } else { passengers[index - 1] };

            let mut latest_time = latest;
            while passengers_set.contains(&latest_time) {
                latest_time -= 1;
            }

            start = latest_time;
        }

        start

    }
}
