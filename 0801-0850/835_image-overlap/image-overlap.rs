
impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        // Create a helper function to find the maximum overlap

        fn find_max_overlap(row: usize, col: usize, img1: &Vec<Vec<i32>>, img2: &Vec<Vec<i32>>) -> i32 {
            // Initialize variables to store overlap in different directions

            let mut overlap_right_down = 0;
            let mut overlap_right_up = 0;
            let mut overlap_left_down = 0;
            let mut overlap_left_up = 0;
            let n = img1.len();  // Assuming img1 and img2 are square matrices of size n x n
            
            for i in 0..n {
                for j in 0..n {
                    // Calculate overlap in the right-down direction

                    if i + row < n && j + col < n {
                        overlap_right_down += img1[i + row][j + col] & img2[i][j];
                    }
                    // Calculate overlap in the left-down direction

                    if i.checked_sub(row).is_some() && j + col < n {
                        overlap_left_down += img1[i - row][j + col] & img2[i][j];
                    }
                    // Calculate overlap in the left-up direction

                    if i.checked_sub(row).is_some() && j.checked_sub(col).is_some() {
                        overlap_left_up += img1[i - row][j - col] & img2[i][j];
                    }
                    // Calculate overlap in the right-up direction

                    if j.checked_sub(col).is_some() && i + row < n {
                        overlap_right_up += img1[i + row][j - col] & img2[i][j];
                    }
                }
            }
            // Return the maximum overlap among the four directions

            overlap_right_down.max(overlap_left_down).max(overlap_right_up).max(overlap_left_up)
        }
        
        let mut max_overlap = 0;
        // Slides over the whole matrix in loops

        for i in 0..img1.len() {
            for j in 0..img1.len() {
                // Calculate and update the maximum overlap in current iteration

                max_overlap = max_overlap.max(find_max_overlap(i, j, &img1, &img2));
            }
        }
        max_overlap

    }
}
