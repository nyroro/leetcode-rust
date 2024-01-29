
impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let n = derived.len();
        let mut A = vec![0; n];

        // Calculate the values of A

        for i in 0..n - 1 {
            A[i + 1] = A[i] ^ derived[i];
        }

        // Check the condition for each element

        for i in 0..n {
            let ni = (i + 1) % n;
            if A[i] ^ A[ni] != derived[i] {
                return false;
            }
        }

        true

    }
}
