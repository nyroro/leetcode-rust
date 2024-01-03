
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_line = Vec::new();
        let mut current_length = 0;
        
        for word in words {
            let word_length = word.len();
            
            if current_length + current_line.len() + word_length <= max_width as usize {
                current_line.push(word);
                current_length += word_length;
            } else {
                let num_words = current_line.len();
                let num_spaces = max_width as usize - current_length;
                
                let mut line = String::new();
                
                if num_words == 1 {
                    line.push_str(&current_line[0]);
                    line.push_str(&" ".repeat(num_spaces));
                } else {
                    let num_gaps = num_words - 1;
                    let num_spaces_per_gap = num_spaces / num_gaps;
                    let num_extra_spaces = num_spaces % num_gaps;
                    
                    for i in 0..num_words {
                        line.push_str(&current_line[i]);
                        
                        if i < num_gaps {
                            line.push_str(&" ".repeat(num_spaces_per_gap));
                            
                            if i < num_extra_spaces {
                                line.push_str(" ");
                            }
                        }
                    }
                }
                
                result.push(line);
                
                current_line.clear();
                current_line.push(word);
                current_length = word_length;
            }
        }
        
        let last_line = current_line.join(" ");
        let num_spaces = max_width as usize - last_line.len();
        let mut last_line_padded = last_line + &" ".repeat(num_spaces);
        
        result.push(last_line_padded);
        
        result

    }
}
