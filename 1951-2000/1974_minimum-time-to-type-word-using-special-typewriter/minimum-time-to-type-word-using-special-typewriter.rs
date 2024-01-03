
impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut time = 0;
        let mut current = 'a';

        for ch in word.chars() {
            let clockwise_distance = (ch as u8 - current as u8 + 26) as i32 % 26;
            let counterclockwise_distance = (current as u8 - ch as u8 + 26) as i32 % 26;
            let min_distance = clockwise_distance.min(counterclockwise_distance);

            time += min_distance;
            current = ch;
            time += 1;
        }

        time

    }
}
