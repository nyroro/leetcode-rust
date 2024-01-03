
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;

        for (i, &height) in heights.iter().enumerate() {
            while let Some(&top) = stack.last() {
                if height <= heights[top] {
                    stack.pop();
                    let width = if stack.is_empty() {
                        i as i32

                    } else {
                        (i - stack.last().unwrap() - 1) as i32

                    };
                    let area = heights[top] * width;
                    max_area = max_area.max(area);
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        while let Some(&top) = stack.last() {
            stack.pop();
            let width = if stack.is_empty() {
                heights.len() as i32

            } else {
                (heights.len() - stack.last().unwrap() - 1) as i32

            };
            let area = heights[top] * width;
            max_area = max_area.max(area);
        }

        max_area

    }
}
