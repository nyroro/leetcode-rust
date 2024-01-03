
impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        // Create a vector of indices from 0 to n-1

        let mut indices: Vec<usize> = (0..reward1.len()).collect();
        
        // Sort the indices based on the difference reward1[i] - reward2[i]
        indices.sort_by(|&i, &j| (reward1[j] - reward2[j]).cmp(&(reward1[i] - reward2[i])));
        
        // Calculate the total points for the first mouse eating the first k cheeses

        let mut total_points = 0;
        for i in 0..k as usize {
            total_points += reward1[indices[i]];
        }
        
        // Calculate the total points for the second mouse eating the remaining cheeses

        for i in k as usize..reward1.len() {
            total_points += reward2[indices[i]];
        }
        
        // Return the total points

        total_points

    }
}
