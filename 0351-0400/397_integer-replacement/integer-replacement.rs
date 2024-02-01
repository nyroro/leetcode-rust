


impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        Solution::helper(n, 0)
    }
    
    fn helper(n: i32, count: i32) -> i32 {
        if n == 1 {
            return count;
        }
        
        if n % 2 == 0 {
            return Solution::helper(n / 2, count + 1);
        } else {
            return std::cmp::min(Solution::helper(n - 1, count + 1), Solution::helper(n + 1, count + 1));
        }
    }
}

fn main() {
    println!("{}", Solution::integer_replacement(8)); // Output: 3

    println!("{}", Solution::integer_replacement(7)); // Output: 4

    println!("{}", Solution::integer_replacement(4)); // Output: 2

}
