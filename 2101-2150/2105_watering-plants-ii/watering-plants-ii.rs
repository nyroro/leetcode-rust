
impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut left_capacity = capacity_a;
        let mut right_capacity = capacity_b;
        let mut left_index = 0;
        let mut right_index = plants.len() - 1;
        let mut refill_count = 0;

        while left_index < right_index {
            left_capacity -= plants[left_index];
            right_capacity -= plants[right_index];

            if left_capacity < 0 {
                left_capacity = capacity_a - plants[left_index];
                refill_count += 1;
            }
            if right_capacity < 0 {
                right_capacity = capacity_b - plants[right_index];
                refill_count += 1;
            }

            left_index += 1;
            right_index -= 1;
        }

        if left_index == right_index {
            let c = plants[left_index];
            let t = std::cmp::max(left_capacity, right_capacity);
            if t < c {
                refill_count += 1;
            }
        }

        refill_count

    }
}
