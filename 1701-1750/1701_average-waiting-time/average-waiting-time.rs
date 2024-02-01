
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut waiting_time = 0.0; // Initialize waiting_time as f64

        let mut current_time = 0;

        for customer in &customers {
            let arrival_time = customer[0];
            let preparation_time = customer[1];

            if current_time < arrival_time {
                current_time = arrival_time;
            }

            waiting_time += (current_time + preparation_time - arrival_time) as f64; // Convert the result to f64

            current_time += preparation_time;
        }

        waiting_time / customers.len() as f64 // Ensure the division result is of type f64

    }
}
