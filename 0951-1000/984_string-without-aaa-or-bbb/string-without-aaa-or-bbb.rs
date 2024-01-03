
impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let mut result = String::new();
        let mut a_count = a;
        let mut b_count = b;

        while a_count > 0 || b_count > 0 {
            if a_count > b_count {
                if result.ends_with("aa") {
                    result.push('b');
                    b_count -= 1;
                } else {
                    result.push('a');
                    a_count -= 1;
                }
            } else {
                if result.ends_with("bb") {
                    result.push('a');
                    a_count -= 1;
                } else {
                    result.push('b');
                    b_count -= 1;
                }
            }
        }

        result

    }
}
