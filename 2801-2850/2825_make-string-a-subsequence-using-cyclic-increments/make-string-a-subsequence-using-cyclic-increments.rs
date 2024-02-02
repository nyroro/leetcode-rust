
impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let str1 = str1.chars().collect::<Vec<char>>();
        let str2 = str2.chars().collect::<Vec<char>>();
        let ascii: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'a'];

        let mut it = str1.iter().peekable();
        for ch in str2.iter() {
            let mut found = false;
            while let Some(c) = it.next() {
                if *c == *ch || ascii[(*c as u8 - b'a') as usize] == *ch || ascii[(*c as u8 - b'a') as usize + 1] == *ch {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        true

    }
}
