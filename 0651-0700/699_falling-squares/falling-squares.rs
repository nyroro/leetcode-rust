
impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut heights = Vec::new();
        let mut result = Vec::new();
        let mut max_height = 0;

        for position in positions {
            let left = position[0];
            let side_length = position[1];
            let right = left + side_length;
            let mut height = 0;

            for i in 0..heights.len() {
                let (start, end, h) = heights[i];
                if start < right && left < end {
                    height = height.max(h);
                }
            }

            height += side_length;
            max_height = max_height.max(height);

            heights.push((left, right, height));
            result.push(max_height);
        }

        result

    }
}
