
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut less_than_pivot = Vec::new();
        let mut greater_than_pivot = Vec::new();
        
        for num in nums {
            if num < pivot {
                less_than_pivot.push(num);
            } else if num > pivot {
                greater_than_pivot.push(num);
            }
        }
        
        let mut result = Vec::new();
        result.append(&mut less_than_pivot);
        result.append(&mut vec![pivot; nums.iter().filter(|&&x| x == pivot).count()]);
        result.append(&mut greater_than_pivot);
        
        result

    }
}
