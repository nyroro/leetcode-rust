
impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut result = std::i32::MIN;
        let mut deque = VecDeque::new();
        
        for point in points {
            let x = point[0];
            let y = point[1];
            
            while !deque.is_empty() && x - deque[0][0] > k {
                deque.pop_front();
            }
            
            if !deque.is_empty() {
                let prev = &deque[0];
                result = result.max(prev[1] + y + (x - prev[0]));
            }
            
            while !deque.is_empty() && y - x >= deque[deque.len() - 1][1] - deque[deque.len() - 1][0] {
                deque.pop_back();
            }
            
            deque.push_back(vec![x, y]);
        }
        
        result

    }
}
