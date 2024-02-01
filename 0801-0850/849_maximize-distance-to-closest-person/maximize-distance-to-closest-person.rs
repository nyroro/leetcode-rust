
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut prev_person = -1;
        let mut max_distance = 0;

        for (i, &seat) in seats.iter().enumerate() {
            if seat == 1 {
                if prev_person == -1 {
                    max_distance = i as i32;
                } else {
                    max_distance = max_distance.max((i - prev_person) as i32 / 2);
                }
                prev_person = i as i32;
            }
        }

        max_distance.max(seats.len() as i32 - 1 - prev_person)
    }
}
