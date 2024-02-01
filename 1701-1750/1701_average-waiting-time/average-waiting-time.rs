
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut waiting_time = 0;
        let mut current_time = 0;

        for customer in customers {
            let arrival_time = customer[0];
            let preparation_time = customer[1];

            if current_time < arrival_time {
                current_time = arrival_time + preparation_time;
                waiting_time += preparation_time;
            } else {
                waiting_time += current_time + preparation_time - arrival_time;
                current_time += preparation_time;
            }
        }

        waiting_time as f64 / customers.len() as f64

    }
}
