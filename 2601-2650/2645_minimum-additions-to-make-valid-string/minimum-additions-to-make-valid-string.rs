


impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut insertions = 0;
        let chars: Vec<char> = word.chars().collect();

        let len_word = chars.len();
        let mut i = 0;
        while i < len_word - 1 {
            let c = chars[i];
            let cnex = chars[i + 1];
            if c == 'a' {
                if cnex == 'a' {
                    insertions += 2;
                } else if i < len_word - 2 && cnex == 'b' && chars[i + 2] == 'c' {
                    i += 2;
                } else {
                    insertions += 1;
                    i += 1;
                }
            } else if c == 'b' {
                if cnex == 'c' {
                    insertions += 1;
                    i += 1;
                } else {
                    insertions += 2;
                }
            } else {
                insertions += 2;
            }
            i += 1;
        }
        if i < len_word {
            insertions += 2;
        }
        insertions

    }
}
