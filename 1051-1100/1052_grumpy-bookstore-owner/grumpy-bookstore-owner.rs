
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let n = customers.len();
        let mut total_satisfied = 0;
        let mut extra_satisfied = 0;
        let mut max_satisfied = 0;
        
        for i in 0..n {
            total_satisfied += customers[i] * (1 - grumpy[i]);
            extra_satisfied += customers[i] * grumpy[i];
            
            if i as i32 >= minutes {
                extra_satisfied -= customers[i - minutes as usize] * grumpy[i - minutes as usize];
            }
            
            max_satisfied = max_satisfied.max(extra_satisfied);
        }
        
        total_satisfied + max_satisfied

    }
}
