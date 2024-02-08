
use std::collections::HashMap;



impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let mut vgaps: Vec<i32> = Vec::new();
        let mut hgaps: Vec<i32> = Vec::new();
        
        let mut hf = h_fences.clone();
        let mut vf = v_fences.clone();
        
        hf.push(m);
        hf.push(1); // including boundaries

        vf.push(n);
        vf.push(1);
        
        hf.sort();
        vf.sort();
        
        for i in 0..vf.len()-1 {
            for j in i+1..vf.len() {
                hgaps.push(vf[j] - vf[i]); // finding all possible gaps

            }
        }
        
        for i in 0..hf.len()-1 {
            for j in i+1..hf.len() {
                vgaps.push(hf[j] - hf[i]);
            }
        }
        
        let mut res: i64 = -1;
        
        vgaps.sort();
        let mut hm: HashMap<i32, i32> = HashMap::new();
        
        for gap in hgaps {
            hm.insert(gap, 1); // storing all horizontal gaps in a map

        }
        
        for &gap in vgaps.iter().rev() {
            if let Some(_) = hm.get(&gap) { // checking for common gaps using the map

                let l = gap as i64;
                res = (l * l) % 1000000007;
                break;
            }
        }
        
        res as i32

    }
}
