
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut new_arr: Vec<i32> = Vec::new();
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                new_arr.push(0);
            }
            new_arr.push(arr[i]);
            i += 1;
        }
        for i in 0..arr.len() {
            arr[i] = new_arr[i];
        }
    }
}
