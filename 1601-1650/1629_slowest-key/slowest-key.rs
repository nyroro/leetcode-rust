
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut max_duration = release_times[0];
        let mut max_key = keys_pressed.chars().next().unwrap();
        
        for i in 1..release_times.len() {
            let duration = release_times[i] - release_times[i - 1];
            if duration > max_duration || (duration == max_duration && keys_pressed.chars().nth(i).unwrap() > max_key) {
                max_duration = duration;
                max_key = keys_pressed.chars().nth(i).unwrap();
            }
        }
        
        max_key

    }
}
