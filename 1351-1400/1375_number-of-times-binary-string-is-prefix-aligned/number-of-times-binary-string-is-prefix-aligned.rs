
impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let n = flips.len();
        let mut lights = vec![0; n];
        let mut result = 0;
        let mut prefix_sum = 0;

        for i in 0..n {
            let index = flips[i] as usize - 1;
            lights[index] = 1;
            prefix_sum += 1;

            if prefix_sum == i as i32 + 1 {
                result += 1;
            }
        }

        result

    }
}
