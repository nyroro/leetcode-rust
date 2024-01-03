
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut total_water = 0;
        let mut left_max = vec![0; n];
        let mut right_max = vec![0; n];

        let mut max_height = 0;
        for i in 0..n {
            max_height = max_height.max(height[i]);
            left_max[i] = max_height;
        }

        max_height = 0;
        for i in (0..n).rev() {
            max_height = max_height.max(height[i]);
            right_max[i] = max_height;
        }

        for i in 0..n {
            let min_height = left_max[i].min(right_max[i]);
            if min_height > height[i] {
                total_water += min_height - height[i];
            }
        }

        total_water

    }
}
