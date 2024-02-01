
impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut moves = 0;
        
        for i in 0..seats.len() {
            let distance = (seats[i] - students[i]).abs();
            moves += distance;
        }
        
        moves

    }
}
