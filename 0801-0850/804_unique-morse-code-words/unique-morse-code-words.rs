
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse_code = vec![
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        
        let mut transformations = std::collections::HashSet::new();
        
        for word in words {
            let mut transformation = String::new();
            for ch in word.chars() {
                let index = (ch as u8 - b'a') as usize;
                transformation.push_str(&morse_code[index]);
            }
            transformations.insert(transformation);
        }
        
        transformations.len() as i32

    }
}
