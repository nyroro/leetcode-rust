
impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut max_additional_rocks = 0;
        let mut full_capacity_bags = 0;

        for i in 0..capacity.len() {
            let remaining_capacity = capacity[i] - rocks[i];
            max_additional_rocks += remaining_capacity;
        }

        for i in 0..capacity.len() {
            let remaining_capacity = capacity[i] - rocks[i];
            if remaining_capacity >= max_additional_rocks {
                full_capacity_bags += 1;
            }
        }

        full_capacity_bags

    }
}
