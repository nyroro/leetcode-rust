
impl Solution {
    pub fn arrange_words(text: String) -> String {
        // Convert the first letter to lowercase and split the text into words

        let mut words: Vec<&str> = text.split_whitespace().collect();
        let first_word = words[0].to_lowercase();
        words[0] = &first_word;

        // Sort the words by length

        words.sort_by_key(|word| word.len());

        // Capitalize the first word and join the words back into a sentence

        let result = words.join(" ");
        let result = result.chars().enumerate().map(|(i, c)| if i == 0 { c.to_uppercase().next().unwrap() } else { c }).collect();

        result

    }
}
