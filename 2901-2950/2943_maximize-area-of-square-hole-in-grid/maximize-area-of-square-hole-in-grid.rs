


impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let mut h_bars = h_bars;
        let mut v_bars = v_bars;
        h_bars.sort();
        v_bars.sort();

        let max_consecutive_h = Self::max_consecutive(&h_bars);
        let max_consecutive_v = Self::max_consecutive(&v_bars);

        let side_length = max_consecutive_h.min(max_consecutive_v) + 1;
        let max_area = side_length * side_length;

        max_area

    }

    fn max_consecutive(bars: &Vec<i32>) -> i32 {
        let mut max_count = 1;
        let mut current_count = 1;

        for i in 1..bars.len() {
            if bars[i] == bars[i - 1] + 1 {
                current_count += 1;
                max_count = max_count.max(current_count);
            } else {
                current_count = 1;
            }
        }

        max_count

    }
}
