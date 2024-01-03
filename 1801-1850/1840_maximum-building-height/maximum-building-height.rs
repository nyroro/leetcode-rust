
impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut restrictions = restrictions;
        restrictions.push(vec![1, 0]);
        restrictions.sort_unstable();
        
        let m = restrictions.len();
        for i in 1..m {
            restrictions[i][1] = restrictions[i][1].min(restrictions[i - 1][1] + (restrictions[i][0] - restrictions[i - 1][0]));
        }
        
        let mut max_height = 0;
        for i in (0..m - 1).rev() {
            restrictions[i][1] = restrictions[i][1].min(restrictions[i + 1][1] + (restrictions[i + 1][0] - restrictions[i][0]));
            max_height = max_height.max((restrictions[i + 1][1] + restrictions[i][1] + restrictions[i + 1][0] - restrictions[i][0]) / 2);
        }
        
        max_height.max(n - restrictions[m - 1][0] + restrictions[m - 1][1])
    }
}
