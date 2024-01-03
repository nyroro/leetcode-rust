
impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut repeats = 1;
        let mut temp = a.clone();
        
        while !temp.contains(&b) {
            if temp.len() >= b.len() + a.len() {
                return -1;
            }
            temp += &a;
            repeats += 1;
        }
        
        repeats

    }
}
