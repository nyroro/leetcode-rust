
impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let n = flips.len();
        let mut lights = vec![0; n];
        let mut result = 0;
        let mut max_index = 0;

        for i in 0..n {
            let index = flips[i] as usize - 1;
            lights[index] = 1;
            max_index = max_index.max(index);

            if max_index == i {
                result += 1;
            }
        }

        result

    }
}
