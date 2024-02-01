
impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();

        let remove_count = (arr.len() as f64 * 0.05) as usize;
        sorted_arr.drain(remove_count..arr.len() - remove_count);

        let sum: i32 = sorted_arr.iter().sum();
        let mean = sum as f64 / sorted_arr.len() as f64;

        mean

    }
}
