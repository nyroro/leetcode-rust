
struct RecentCounter {
    requests: Vec<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            requests: Vec::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.requests.push(t);
        let start_time = t - 3000;
        let mut count = 0;
        for &request_time in &self.requests {
            if request_time >= start_time && request_time <= t {
                count += 1;
            }
        }
        count

    }
}
