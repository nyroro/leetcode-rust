
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new(); // Create an empty 2D array

        for &num in nums.iter() {
            let mut found = false;
            for row in result.iter_mut() {
                if !row.contains(&num) {
                    row.push(num);
                    found = true;
                    break;
                }
            }
            if !found {
                result.push(vec![num]);
            }
        }
        result

    }
}
