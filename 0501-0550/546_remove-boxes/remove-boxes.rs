
impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let n = boxes.len();
        let mut memo = vec![vec![vec![0; n]; n]; n];
        Solution::calculate_points(&boxes, &mut memo, 0, n - 1, 0) as i32

    }

    fn calculate_points(boxes: &Vec<i32>, memo: &mut Vec<Vec<Vec<i32>>>, l: usize, r: usize, k: usize) -> usize {
        if l > r {
            return 0;
        }
        if memo[l][r][k] > 0 {
            return memo[l][r][k] as usize;
        }
        let mut l_new = l;
        let mut k_new = k;
        while l_new + 1 <= r && boxes[l_new] == boxes[l_new + 1] {
            l_new += 1;
            k_new += 1;
        }
        let mut res = (k_new + 1) * (k_new + 1) + Solution::calculate_points(boxes, memo, l_new + 1, r, 0);
        for m in (l_new + 1)..=r {
            if boxes[l_new] == boxes[m] {
                res = res.max(Solution::calculate_points(boxes, memo, l_new + 1, m - 1, 0) + Solution::calculate_points(boxes, memo, m, r, k_new + 1));
            }
        }
        memo[l][r][k] = res as i32;
        res

    }
}
