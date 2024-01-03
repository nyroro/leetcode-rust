
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img[0].len();
        let mut result = vec![vec![0; n]; m];
        
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0;
                let mut count = 0;
                
                for r in -1..=1 {
                    for c in -1..=1 {
                        let row = i as i32 + r;
                        let col = j as i32 + c;
                        
                        if row >= 0 && row < m as i32 && col >= 0 && col < n as i32 {
                            sum += img[row as usize][col as usize];
                            count += 1;
                        }
                    }
                }
                
                result[i][j] = sum / count;
            }
        }
        
        result

    }
}
