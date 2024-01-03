
/*
 * // This is the custom function interface.
 * // You should not implement it, or speculate about its implementation

 * struct CustomFunction;
 * impl CustomFunction {
 *    pub fn f(x:i32,y:i32)->i32{}
 * }
 */

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut x = 1;
        let mut y = 1000;
        
        while x <= 1000 && y >= 1 {
            let value = customfunction.f(x, y);
            
            if value == z {
                result.push(vec![x, y]);
                x += 1;
                y -= 1;
            } else if value < z {
                x += 1;
            } else {
                y -= 1;
            }
        }
        
        result

    }
}
