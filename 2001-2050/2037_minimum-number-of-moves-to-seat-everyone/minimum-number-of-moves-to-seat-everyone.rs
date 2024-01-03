
impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut sorted_seats = seats.clone();
        let mut sorted_students = students.clone();
        
        sorted_seats.sort();
        sorted_students.sort();
        
        let mut moves = 0;
        
        for i in 0..seats.len() {
            let distance = (sorted_seats[i] - sorted_students[i]).abs();
            moves += distance;
        }
        
        moves

    }
}
