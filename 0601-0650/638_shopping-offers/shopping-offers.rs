
use std::collections::HashMap;

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut memo: HashMap<Vec<i32>, i32> = HashMap::new();
        Self::helper(&price, &special, &needs, &mut memo)
    }
    
    fn helper(price: &Vec<i32>, special: &Vec<Vec<i32>>, needs: &Vec<i32>, memo: &mut HashMap<Vec<i32>, i32>) -> i32 {
        if let Some(&val) = memo.get(needs) {
            return val;
        }
        
        let mut res = Self::regular_price(&price, &needs);
        
        for s in special {
            let mut clone_needs = needs.clone();
            let mut i = 0;
            while i < needs.len() {
                if clone_needs[i] < s[i] {
                    break;
                }
                clone_needs[i] -= s[i];
                i += 1;
            }
            if i == needs.len() {
                res = res.min(s.last().unwrap() + Self::helper(price, special, &clone_needs, memo));
            }
        }
        
        memo.insert(needs.clone(), res);
        res

    }
    
    fn regular_price(price: &Vec<i32>, needs: &Vec<i32>) -> i32 {
        price.iter().zip(needs).map(|(&p, &n)| p * n).sum()
    }
}
