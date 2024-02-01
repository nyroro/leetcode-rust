
struct TrieNode {
    is_word: bool,
    children: std::collections::HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_word: false,
            children: std::collections::HashMap::new(),
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::new());
        }
        node.is_word = true;
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        
        let mut result = Vec::new();
        let rows = board.len();
        let cols = board[0].len();
        
        for i in 0..rows {
            for j in 0..cols {
                let mut visited = vec![vec![false; cols]; rows];
                let mut word = String::new();
                Solution::search(&board, i, j, &trie.root, &mut visited, &mut word, &mut result);
            }
        }
        
        result

    }
    
    fn search(
        board: &Vec<Vec<char>>,
        row: usize,
        col: usize,
        node: &TrieNode,
        visited: &mut Vec<Vec<bool>>,
        word: &mut String,
        result: &mut Vec<String>,
    ) {
        if row >= board.len() || col >= board[0].len() || visited[row][col] {
            return;
        }
        
        let ch = board[row][col];
        if !node.children.contains_key(&ch) {
            return;
        }
        
        visited[row][col] = true;
        word.push(ch);
        
        let next_node = &node.children[&ch];
        if next_node.is_word {
            result.push(word.clone());
            next_node.is_word = false; // Avoid duplicate results

        }
        
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;
            if new_row >= 0 && new_row < board.len() as i32 && new_col >= 0 && new_col < board[0].len() as i32 {
                Solution::search(
                    board,
                    new_row as usize,
                    new_col as usize,
                    next_node,
                    visited,
                    word,
                    result,
                );
            }
        }
        
        visited[row][col] = false;
        word.pop();
    }
}
