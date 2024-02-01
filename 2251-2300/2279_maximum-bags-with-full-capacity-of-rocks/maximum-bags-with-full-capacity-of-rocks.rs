
impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
        let mut remaining_capacity: Vec<i32> = Vec::new();
        let mut full_capacity_bags = 0;

        for i in 0..capacity.len() {
            remaining_capacity.push(capacity[i] - rocks[i]);
        }

        remaining_capacity.sort();

        for i in 0..capacity.len() {
            if additional_rocks >= remaining_capacity[i] {
                additional_rocks -= remaining_capacity[i];
                full_capacity_bags += 1;
            } else {
                break;
            }
        }

        full_capacity_bags

    }
}
