
impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut s1_odd: Vec<char> = s1.chars().step_by(2).collect();
        let mut s1_even: Vec<char> = s1.chars().skip(1).step_by(2).collect();
        let mut s2_odd: Vec<char> = s2.chars().step_by(2).collect();
        let mut s2_even: Vec<char> = s2.chars().skip(1).step_by(2).collect();

        s1_odd.sort();
        s1_even.sort();
        s2_odd.sort();
        s2_even.sort();

        s1_odd == s2_odd && s1_even == s2_even

    }
}
