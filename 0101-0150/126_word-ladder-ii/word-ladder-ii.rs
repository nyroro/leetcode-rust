
use std::collections::{HashMap, HashSet, VecDeque};

// Define a struct to represent the Solution



impl Solution {
    // Implement the find_ladders function

    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        // Create a HashMap to store the word and its corresponding steps

        let mut mpp: HashMap<String, i32> = HashMap::new();
        // Create a vector to store the resulting sequences

        let mut ans: Vec<Vec<String>> = Vec::new();
        // Create a variable to store the beginWord

        let b = begin_word.clone();

        // Define a helper function dfs to perform depth-first search

        fn dfs(word: String, seq: &mut Vec<String>, mpp: &HashMap<String, i32>, ans: &mut Vec<Vec<String>>, b: &String) {
            // Check if the word is equal to beginWord

            if word == *b {
                // Reverse the sequence and add it to the result

                seq.reverse();
                ans.push(seq.clone());
                seq.reverse();
                return;
            }
            // Get the steps for the current word

            let steps = mpp.get(&word).unwrap();
            // Iterate through each character of the word

            for (i, _) in word.chars().enumerate() {
                // Replace the character with all lowercase English letters

                for c in 'a'..='z' {
                    let mut new_word = word.clone();
                    new_word.replace_range(i..=i, &c.to_string());
                    // Check if the new_word exists in the HashMap and its steps are one less than the current steps

                    if mpp.contains_key(&new_word) && mpp[&new_word] + 1 == *steps {
                        // Add the new_word to the sequence and recursively call dfs

                        seq.push(new_word.clone());
                        dfs(new_word, seq, mpp, ans, b);
                        seq.pop();
                    }
                }
            }
        }

        // Create a HashSet to store the wordList

        let mut st: HashSet<String> = word_list.iter().cloned().collect();
        // Create a queue to perform breadth-first search

        let mut q: VecDeque<String> = VecDeque::new();
        // Push the beginWord into the queue

        q.push_back(begin_word.clone());
        // Insert the beginWord into the HashMap with steps 1

        mpp.insert(begin_word.clone(), 1);
        // Remove the beginWord from the HashSet

        st.remove(&begin_word);

        // Perform breadth-first search

        while let Some(word) = q.pop_front() {
            // Get the steps for the current word

            let steps = mpp[&word];
            // Check if the word is equal to endWord

            if word == end_word {
                break;
            }
            // Iterate through each character of the word

            for (i, _) in word.chars().enumerate() {
                // Replace the character with all lowercase English letters

                for c in 'a'..='z' {
                    let mut new_word = word.clone();
                    new_word.replace_range(i..=i, &c.to_string());
                    // Check if the new_word exists in the HashSet

                    if st.contains(&new_word) {
                        // Push the new_word into the queue, remove it from the HashSet, and insert it into the HashMap with steps incremented by 1

                        q.push_back(new_word.clone());
                        st.remove(&new_word);
                        mpp.insert(new_word, steps + 1);
                    }
                }
            }
        }

        // Check if endWord exists in the HashMap

        if let Some(seq) = mpp.get(&end_word) {
            // Create a vector to store the sequence

            let mut seq: Vec<String> = Vec::new();
            seq.push(end_word.clone());
            // Call the dfs function to find all the sequences

            dfs(end_word, &mut seq, &mpp, &mut ans, &b);
        }

        // Return the resulting sequences

        ans

    }
}
