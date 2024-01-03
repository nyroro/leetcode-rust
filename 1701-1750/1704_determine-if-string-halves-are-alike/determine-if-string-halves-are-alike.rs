
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mid = s.len() / 2;
        let (a, b) = s.split_at(mid);
        let count_a = a.chars().filter(|c| vowels.contains(c)).count();
        let count_b = b.chars().filter(|c| vowels.contains(c)).count();
        count_a == count_b

    }
}
