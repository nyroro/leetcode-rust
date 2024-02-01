
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut parent: [usize; 26] = [0; 26];
        for i in 0..26 {
            parent[i] = i;
        }
        
        for equation in &equations {
            let chars: Vec<char> = equation.chars().collect();
            let x = chars[0] as usize - 'a' as usize;
            let y = chars[3] as usize - 'a' as usize;
            let op = chars[1];
            
            if op == '=' {
                Self::union(&mut parent, x, y);
            }
        }
        
        for equation in &equations {
            let chars: Vec<char> = equation.chars().collect();
            let x = chars[0] as usize - 'a' as usize;
            let y = chars[3] as usize - 'a' as usize;
            let op = chars[1];
            
            if op == '!' && Self::find(&mut parent, x) == Self::find(&mut parent, y) {
                return false;
            }
        }
        
        true

    }
    
    fn find(parent: &mut [usize; 26], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }
        parent[x]
    }
    
    fn union(parent: &mut [usize; 26], x: usize, y: usize) {
        let root_x = Self::find(parent, x);
        let root_y = Self::find(parent, y);
        parent[root_x] = root_y;
    }
}
