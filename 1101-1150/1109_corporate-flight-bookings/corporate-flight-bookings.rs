
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut seats = vec![0; n as usize];
        
        for booking in bookings {
            let first = booking[0] as usize;
            let last = booking[1] as usize;
            let seats_reserved = booking[2];
            
            for i in first..=last {
                seats[i - 1] += seats_reserved;
            }
        }
        
        seats

    }
}
