
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut arr = vec![first];
        for i in 0..encoded.len() {
            let next = encoded[i] ^ arr[i];
            arr.push(next);
        }
        arr

    }
}
