
impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let mut count = 0;
        let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        let chars: Vec<char> = word.chars().collect();

        for i in 0..chars.len() {
            let mut found_vowels = [false; 5];
            for j in i..chars.len() {
                if vowels.contains(&chars[j]) {
                    match chars[j] {
                        'a' => found_vowels[0] = true,
                        'e' => found_vowels[1] = true,
                        'i' => found_vowels[2] = true,
                        'o' => found_vowels[3] = true,
                        'u' => found_vowels[4] = true,
                        _ => {}
                    }
                    if found_vowels.iter().all(|&x| x) {
                        count += 1;
                    }
                } else {
                    break;
                }
            }
        }

        count

    }
}
