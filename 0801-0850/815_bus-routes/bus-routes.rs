
use std::collections::{HashMap, HashSet, VecDeque};



impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let mut stop_to_routes = HashMap::new();
        for (i, route) in routes.iter().enumerate() {
            for &stop in route {
                stop_to_routes.entry(stop).or_insert_with(Vec::new).push(i);
            }
        }

        let mut visited_routes = vec![false; routes.len()];
        let mut visited_stops = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((source, 0));
        visited_stops.insert(source);

        while let Some((stop, buses)) = queue.pop_front() {
            if stop == target {
                return buses;
            }

            for &route_idx in stop_to_routes.get(&stop).unwrap_or(&vec![]).iter() {
                if visited_routes[route_idx] {
                    continue;
                }
                visited_routes[route_idx] = true;

                for &next_stop in routes[route_idx].iter() {
                    if !visited_stops.contains(&next_stop) {
                        visited_stops.insert(next_stop);
                        queue.push_back((next_stop, buses + 1));
                    }
                }
            }
        }

        -1

    }
}
