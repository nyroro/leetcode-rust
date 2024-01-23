
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr: Vec<i32> = Vec::new();
        arr.push(pref[0]);
        let mut t = arr[0];
        for i in 1..pref.len() {
            arr.push(pref[i-1] ^ pref[i]);
        }
        arr

    }
}
