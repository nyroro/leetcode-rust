
impl Solution {
    pub fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
        let mut new_nums: Vec<Vec<i32>> = Vec::new();

        // Step 2 and 3: Sort the rows in descending order and store them in new_nums

        for row in &nums {
            let mut sorted_row = row.clone();
            sorted_row.sort_by(|a, b| b.cmp(a));
            new_nums.push(sorted_row);
        }

        // Step 4: Transpose the new_nums matrix

        let transposed: Vec<Vec<i32>> = (0..new_nums[0].len())
            .map(|i| new_nums.iter().map(|row| row[i]).collect())
            .collect();

        // Step 5 and 6: Calculate the final score

        let mut score = 0;
        for col in transposed {
            score += col.iter().max().unwrap();
        }

        // Step 7: Return the final score

        score

    }
}
