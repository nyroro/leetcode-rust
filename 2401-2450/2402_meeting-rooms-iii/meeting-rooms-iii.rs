
impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let n = n as i64; // Convert n to i64

        let mut occupied: BinaryHeap<Reverse<(i64, i64)>> = BinaryHeap::new();
        let mut free: BinaryHeap<Reverse<i64>> = (0..n).map(|x| Reverse(x)).collect();
        let mut counter = vec![0; n as usize];

        let mut sorted_meetings = meetings.clone();
        sorted_meetings.sort();

        for meet in sorted_meetings {
            let mut index = 0;
            while let Some(Reverse((start, room))) = occupied.peek() {
                if *start <= meet[0] as i64 {
                    free.push(Reverse(*room));
                    occupied.pop();
                } else {
                    break;
                }
            }
            if let Some(Reverse(room)) = free.pop() {
                index = room;
                occupied.push(Reverse((meet[1] as i64, room)));
            } else {
                if let Some(Reverse((start, room))) = occupied.pop() {
                    index = room;
                    occupied.push(Reverse((start + (meet[1] as i64 - meet[0] as i64), room)));
                }
            }
            counter[index as usize] += 1;
        }

        let mut ans = 0;
        let mut max_value = 0;
        for (i, &count) in counter.iter().enumerate() {
            if count > max_value {
                max_value = count;
                ans = i as i32;
            }
        }
        ans

    }
}
