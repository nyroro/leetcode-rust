
impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut map = std::collections::HashMap::new();
        for (i, &num) in arr.iter().enumerate() {
            map.insert(num, i);
        }
        
        let mut i = 0;
        while i < pieces.len() {
            let piece = &pieces[i];
            if let Some(&start) = map.get(&piece[0]) {
                if arr[start..start+piece.len()] == piece[..] {
                    i += 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        arr.len() == pieces.iter().map(|piece| piece.len()).sum()
    }
}
