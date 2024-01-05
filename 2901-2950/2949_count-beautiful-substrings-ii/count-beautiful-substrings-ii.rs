use std::collections::HashMap;

impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i64 {
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        
        // If the string is not long enough to construct a beautiful substring, return 0
        if n < 2 {
            return 0;
        }

        let mut beautiful_count = 0;

        // Find the minimum length, t, such that t * t % (4 * k) == 0
        let mut t = 0;
        for i in 1..=n {
            if i * i % (4 * k as usize) == 0 {
                t = i;
                break;
            }
        }

        // If t is 0, return 0 directly
        if t == 0 {
            return 0;
        }

        // Define the function f(ss) = vowels(ss) - consonants(ss)
        let mut f_map: HashMap<(usize, i32), i32> = HashMap::new();
        let mut vowels = 0;
        let mut consonants = 0;
        f_map.insert((0, 0), 1);
        for i in 0..n {
            if Self::is_vowel(s_chars[i]) {
                vowels += 1;
            } else {
                consonants += 1;
            }
            let v = vowels - consonants;
            let x = (i + 1) % t; // Calculate the index x based on the length of the substring
            beautiful_count += *f_map.get(&(x, v)).unwrap_or(&0) as i64;
            *f_map.entry((x, v)).or_insert(0) += 1;
        }

        beautiful_count
    }

    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }
}