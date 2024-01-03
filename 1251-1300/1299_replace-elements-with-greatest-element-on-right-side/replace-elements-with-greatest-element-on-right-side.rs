
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut max_value = -1;
        let mut result = vec![];

        for i in (0..arr.len()).rev() {
            result.insert(0, max_value);
            max_value = max_value.max(arr[i]);
        }

        result

    }
}
