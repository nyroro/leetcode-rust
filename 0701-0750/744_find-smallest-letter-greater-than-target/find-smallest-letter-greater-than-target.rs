
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut start = 0;
        let mut end = letters.len();
        
        while start < end {
            let mid = start + (end - start) / 2;
            
            if letters[mid] <= target {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        
        if start == letters.len() {
            letters[0]
        } else {
            letters[start]
        }
    }
}
