


impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut q = 0;
        let mut block = false;
        let mut ret = 0;

        for t in directions.chars() {
            match t {
                'R' => {
                    q += 1;
                    block = true;
                }
                'S' => {
                    ret += q;
                    q = 0;
                    block = true;
                }
                'L' => {
                    if block {
                        ret += q + 1;
                        q = 0;
                    }
                }
                _ => {}
            }
        }
        ret

    }
}

fn main() {
    let result1 = Solution::count_collisions(String::from("RLRSLL"));
    let result2 = Solution::count_collisions(String::from("LLRR"));
    println!("Result 1: {}", result1); // Output: 5

    println!("Result 2: {}", result2); // Output: 0

}
