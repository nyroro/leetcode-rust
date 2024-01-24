
use std::collections::HashMap;

struct Encrypter {
    encrypt_map: HashMap<char, String>,
    decrypt_map: HashMap<String, i32>,
}

impl Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let mut encrypt_map = HashMap::new();
        for i in 0..keys.len() {
            encrypt_map.insert(keys[i], values[i].clone());
        }

        let mut decrypt_map = HashMap::new();
        for word in dictionary {
            let encrypted_word = Self::encrypt_word(&encrypt_map, &word);
            *decrypt_map.entry(encrypted_word).or_insert(0) += 1;
        }

        Encrypter {
            encrypt_map,
            decrypt_map,
        }
    }

    fn encrypt_word(encrypt_map: &HashMap<char, String>, word: &str) -> String {
        let mut encrypted_word = String::new();
        for c in word.chars() {
            if let Some(value) = encrypt_map.get(&c) {
                encrypted_word.push_str(value);
            } else {
                return String::new();
            }
        }
        encrypted_word

    }

    fn encrypt(&self, word1: String) -> String {
        Self::encrypt_word(&self.encrypt_map, &word1)
    }

    fn decrypt(&self, word2: String) -> i32 {
        *self.decrypt_map.get(&word2).unwrap_or(&0)
    }
}
