
impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        let mut tiles = tiles;
        tiles.sort(); // 对tiles数组进行排序


        fn gao(tiles: &Vec<Vec<i32>>, carpet_len: i32) -> i32 {
            let mut ret = 0;
            let mut r = 0;
            let mut now = 0;
            for i in 0..tiles.len() {
                if i > 0 {
                    now -= tiles[i-1][1] - tiles[i-1][0] + 1;
                }
                while r < tiles.len() && tiles[r][1] < tiles[i][0] + carpet_len {
                    now += tiles[r][1] - tiles[r][0] + 1;
                    r += 1;
                }
                if r < tiles.len() {
                    ret = ret.max(now + (tiles[i][0] + carpet_len - tiles[r][0]).max(0));
                } else {
                    ret = ret.max(now);
                }
            }
            ret

        }

        let mut result = gao(&tiles, carpet_len);
        let m = tiles.iter().map(|t| t[1]).max().unwrap();
        tiles = tiles.iter().map(|t| vec![m - t[1] + 1, m - t[0] + 1]).collect();
        tiles.sort();
        result = result.max(gao(&tiles, carpet_len));
        result

    }
}
