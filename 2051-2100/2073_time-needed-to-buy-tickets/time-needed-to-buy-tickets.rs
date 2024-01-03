
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut tickets_left = tickets.clone();
        let mut time = 0;
        let mut index = 0;
        
        loop {
            for i in 0..tickets_left.len() {
                if tickets_left[i] > 0 {
                    tickets_left[i] -= 1;
                    time += 1;
                    if i == k as usize && tickets_left[i] == 0 {
                        return time;
                    }
                }
            }
        }
    }
}
