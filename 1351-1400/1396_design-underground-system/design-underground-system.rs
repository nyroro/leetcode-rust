
use std::collections::HashMap;

struct UndergroundSystem {
    check_ins: HashMap<i32, (String, i32)>,
    travel_times: HashMap<(String, String), (f64, usize)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            check_ins: HashMap::new(),
            travel_times: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_ins.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((start_station, start_time)) = self.check_ins.remove(&id) {
            let travel_time = (t - start_time) as f64;
            let entry = self.travel_times.entry((start_station, station_name)).or_insert((0.0, 0));
            entry.0 += travel_time;
            entry.1 += 1;
        }
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some((total_time, total_trips)) = self.travel_times.get(&(start_station, end_station)) {
            *total_time / *total_trips as f64

        } else {
            0.0

        }
    }
}
