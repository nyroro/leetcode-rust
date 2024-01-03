
impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();

        let remove_count = (arr.len() as f64 * 0.05) as usize;
        let (_, middle) = sorted_arr.split_at(remove_count);
        let (middle, _) = middle.split_at(arr.len() - 2 * remove_count);

        let sum: i32 = middle.iter().sum();
        let mean = sum as f64 / middle.len() as f64;

        mean

    }
}
