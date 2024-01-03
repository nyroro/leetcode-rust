
use std::collections::HashMap;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();
        let mut res: Vec<String> = Vec::new();

        for i in 0..key_name.len() {
            let name = &key_name[i];
            let time = Self::get_time(&key_time[i]);
            if !map.contains_key(name) {
                map.insert(name.clone(), Vec::new());
            }
            map.get_mut(name).unwrap().push(time);
        }

        for (name, times) in &map {
            let mut sorted_times = times.clone();
            sorted_times.sort();

            for i in 2..sorted_times.len() {
                if sorted_times[i] - sorted_times[i - 2] <= 60 {
                    res.push(name.clone());
                    break;
                }
            }
        }

        res.sort();
        res.dedup();

        res

    }

    fn get_time(s: &String) -> i32 {
        let parts: Vec<&str> = s.split(":").collect();
        let hours: i32 = parts[0].parse().unwrap();
        let minutes: i32 = parts[1].parse().unwrap();
        hours * 60 + minutes

    }
}
