
impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let mut alice_total = 0;
        let mut bob_total = 0;
        
        for alice_size in &alice_sizes {
            alice_total += alice_size;
        }
        
        for bob_size in &bob_sizes {
            bob_total += bob_size;
        }
        
        let diff = alice_total - bob_total;
        
        for alice_size in &alice_sizes {
            let bob_size = alice_size - diff / 2;
            if bob_sizes.contains(&bob_size) {
                return vec![*alice_size, bob_size];
            }
        }
        
        // 如果没有找到合适的交换方案，返回空数组

        vec![]
    }
}
