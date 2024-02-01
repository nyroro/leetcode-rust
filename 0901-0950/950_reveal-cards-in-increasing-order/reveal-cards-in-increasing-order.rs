
use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck;
        deck.sort(); // 将卡牌按照从小到大的顺序排序

        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        for i in (0..deck.len()).rev() {
            if let Some(last) = queue.pop_back() {
                queue.push_front(last);
            }
            queue.push_front(deck[i]);
        }
        
        while let Some(card) = queue.pop_front() {
            result.push(card);
        }
        
        result

    }
}
